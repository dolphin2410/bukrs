mod net;
mod varint;
mod core;
mod api;
mod macros;

use std::{net::SocketAddr, sync::{Arc, Mutex}, collections::HashMap};
use futures::{StreamExt, stream::{SplitSink, SplitStream}, SinkExt};
use net::{Codec, BukrsPacketData, BukrsFuture, Packet, BukrsReqAPI, cast_packet, BukrsResAPI};
use rand::Rng;
use tokio::net::TcpStream;
use tokio_util::codec::{Decoder, Framed};

type DefaultTx = SplitSink<Framed<TcpStream, Codec>, BukrsPacketData>;
type DefaultRx = SplitStream<Framed<TcpStream, Codec>>;
type BukrsListener = fn (Box<dyn Packet>) -> ();
type ArcMutex<T> = Arc<Mutex<T>>;

pub struct API {
    pub(crate) tx: DefaultTx,
    pub(crate) listeners: Arc<Mutex<Vec<BukrsListener>>>,
    pub(crate) payload_listeners: Arc<Mutex<HashMap<u32, Arc<BukrsFuture>>>>,
}

async fn send_packet_tx(tx: &mut DefaultTx, event: impl Packet, payload_id: Option<u32>) -> anyhow::Result<()> {
    tx.send(BukrsPacketData { payload_id, event: Box::new(event) }).await.unwrap();
    tx.flush().await.unwrap();
    Ok(())
}

impl API {
    async fn init_listener(map: ArcMutex<HashMap<u32, Arc<BukrsFuture>>>, listeners: ArcMutex<Vec<BukrsListener>>, mut rx: DefaultRx) {
        while let Some(Ok(msg)) = rx.next().await {
            if let Some(payload_id) = msg.payload_id {
                if let Some(future) = map.lock().unwrap().get(&payload_id) {
                    let future = future.clone();
                    future.set_data(msg.event.clone_box());
                    future.wake();
                }
            }
    
            for listener in listeners.lock().unwrap().iter() {
                listener(msg.event.clone_box());
            }
        }
    }

    /// Request for API
    pub async fn request(server: SocketAddr) -> anyhow::Result<API> {
        let codec = Codec;
        let client = TcpStream::connect(server).await?;
        let (tx, rx) = codec.framed(client).split();
        let mut api = API { tx, listeners: arc_mutex!(vec![]), payload_listeners: arc_mutex!(HashMap::new()) };  // Initiate api
        tokio::spawn(Self::init_listener(api.payload_listeners.clone(), api.listeners.clone(), rx));   // Initiate listeners
        let _response = api.send_packet_await::<BukrsResAPI>(BukrsReqAPI {  }).await?;

        Ok(api)
    }

    pub async fn send_packet_await<T: Packet + Clone>(&mut self, packet: impl Packet) -> anyhow::Result<T> {
        let payload_id = rand::thread_rng().gen_range(1..u32::MAX);  // TODO a better way for this // maybe uuid?
        self.send_packet(packet, Some(payload_id)).await?;  // send packet with payload id
        let future = Arc::new(BukrsFuture::new(payload_id, self.payload_listeners.clone()));
        self.payload_listeners.lock().unwrap().insert(payload_id, future.clone());  // Add future to payload handlers
        let response_packet = future.as_ref().await;
        Ok(cast_packet::<T>(&response_packet).unwrap())
    }

    pub fn add_listener(&self, listener: fn(Box<dyn Packet>) -> ()) {    // For ServerData
        self.listeners.lock().unwrap().push(listener);
    }

    pub async fn send_packet(&mut self, packet: impl Packet, payload_id: Option<u32>) -> anyhow::Result<()> {
        send_packet_tx(&mut self.tx, packet, payload_id).await?;    // Sends packet to server
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;

    use futures::StreamExt;
    use tokio::net::TcpListener;
    use tokio_util::codec::Decoder;

    use crate::{net::{Codec, BukrsReqCreateInventory, BukrsResCreateInventory, BukrsResOnlinePlayers, BukrsReqOnlinePlayers, BukrsReqPlayerInvOpen, BukrsResPlayerInvOpen, BukrsReqAPI, BukrsResAPI, cast_packet}, API, send_packet_tx, core::{player::PlayerId, invfx::{InventorySize, InvfxId}}};

    #[tokio::test]
    async fn server() -> anyhow::Result<()> {
        println!("Server Initialized");
        let server = TcpListener::bind::<SocketAddr>("127.0.0.1:25565".parse()?).await?;
        loop {
            let (socket, _) = server.accept().await?;
            tokio::spawn(async move {
                let codec = Codec;
                let (mut tx, mut rx) = codec.framed(socket).split();
                loop {
                    if let Some(Ok(msg)) = rx.next().await {
                        // println!("Income: {:?}", msg.event);
                        if let Some(BukrsReqAPI { .. }) = cast_packet(&msg.event) {
                            println!("BukrsReq");
                            send_packet_tx(&mut tx, BukrsResAPI { api_id: 1024 }, msg.payload_id).await.unwrap();
                        }
                        if let Some(BukrsReqOnlinePlayers { .. }) = cast_packet(&msg.event) {
                            println!("ReqOnlinePlayers");
                            send_packet_tx(&mut tx, BukrsResOnlinePlayers { players: vec![PlayerId(1024)]}, msg.payload_id).await.unwrap();
                        }
                        if let Some(BukrsReqCreateInventory { .. }) = cast_packet(&msg.event) {
                            println!("ReqCreateInventory");
                            send_packet_tx(&mut tx, BukrsResCreateInventory { inv_id: InvfxId(1024) }, msg.payload_id).await.unwrap();
                        }
                        if let Some(BukrsReqPlayerInvOpen { .. }) = cast_packet(&msg.event) {
                            println!("InvOpen");
                            send_packet_tx(&mut tx, BukrsResPlayerInvOpen {  }, msg.payload_id).await.unwrap();
                        }
                        println!("Waiting...");
                    }
                }
            });
        }
    }

    #[tokio::test]
    async fn client() -> anyhow::Result<()> {
        println!("Client Initialized");
        let mut api = API::request("127.0.0.1:25565".parse()?).await?;
        println!("DONE");
        let BukrsResCreateInventory { inv_id } = api.send_packet_await(BukrsReqCreateInventory { name: "BukkitRs".to_string(), size: InventorySize::Inv27 }).await?;
        
        let BukrsResOnlinePlayers { players } = api.send_packet_await(BukrsReqOnlinePlayers {  }).await?;
        println!("Players: {:?}", players);
            for player in players.iter() {
                let BukrsResPlayerInvOpen {  } = api.send_packet_await(BukrsReqPlayerInvOpen { inv_id: inv_id.clone(), player_id: player.clone() }).await?;
            }
        Ok(())
    }
}

// 1114991474 1114991474
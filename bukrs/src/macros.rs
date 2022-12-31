#[macro_export]
macro_rules! arc_mutex {
    ($internal:expr) => {
        std::sync::Arc::new(std::sync::Mutex::new($internal))
    };
}

/// https://github.com/feather-rs/feather/blob/main/feather/protocol/src/packets.rs
#[macro_export]
macro_rules! register_packet {
    (
        $(  
            $packet:ident {
                $(
                    $field:ident $typ:ident $(<$generics:ident>)?
                );* $(;)?
            } $(,)?
        )*
    ) => {
        $(
            #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, bukrs_derive::BukrsPacket)]
            pub struct $packet {
                $(
                    pub $field: $typ $(::<$generics>)?,
                )*
            }

            #[typetag::serde]
            impl crate::net::Packet for $packet {
                fn clone_box(&self) -> Box<dyn crate::net::Packet> {
                    Box::new(self.clone())
                }

                fn get_any(&self) -> Box<dyn std::any::Any> {
                    Box::new(self.clone())
                }
            }

            impl std::fmt::Display for $packet {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "{:?}", self)
                }
            }

            #[ctor::ctor]
            #[allow(non_snake_case)]
            fn $packet() {
                crate::net::CONSTRUCTORS.lock().unwrap().insert(stringify!($packet).to_string(), |buf: &mut bytes::BytesMut| {
                    Box::new($packet::decode(buf))
                });
            }
        )*
    };
}
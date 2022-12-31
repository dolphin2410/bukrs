package me.dolphin2410.bukrs

import io.netty.channel.ChannelHandlerContext
import io.netty.util.AttributeKey
import org.bukkit.Bukkit
import org.bukkit.plugin.java.JavaPlugin
import java.util.Random

class BukrsMain: JavaPlugin() {
    companion object {
        @JvmStatic
        val BukrsClientIdKey = AttributeKey.valueOf<Int>("BukrsClientIdKey")!!
    }

    val clients = ArrayList<ChannelHandlerContext>()

    override fun onEnable() {
        defaultCodecs()

        BukrsEvents.addListener(object: BukrsListener {
            @BukrsEventHandler
            fun apiReq(ctx: ChannelHandlerContext, payloadId: Int, packet: DefaultPackets.BukrsReqAPI) {
                val random = Random()
                val id = random.nextInt(Int.MAX_VALUE)
                ctx.pipeline().writeAndFlush(payloadId to DefaultPackets.BukrsResAPI(id)).sync()
                clients.add(ctx)
                ctx.channel().attr(BukrsClientIdKey).set(id)
            }

            @BukrsEventHandler
            fun bukrsPlayers(ctx: ChannelHandlerContext, payloadId: Int, packet: DefaultPackets.BukrsReqOnlinePlayers) {
                ctx.pipeline().writeAndFlush(payloadId to DefaultPackets.BukrsResOnlinePlayers(Bukkit.getOnlinePlayers().map { it.entityId }))
            }

            @BukrsEventHandler
            fun bukrsPlayerById(ctx: ChannelHandlerContext, payloadId: Int, packet: DefaultPackets.BukrsReqPlayerById) {
                // TODO error handling
                val player = Bukkit.getOnlinePlayers().find { it.entityId == packet.player }!!
                ctx.pipeline().writeAndFlush(payloadId to DefaultPackets.BukrsResPlayerData(PlayerData(player.entityId, player.name, player.uniqueId)))
            }
        })
        NettyServer().run()
    }
}
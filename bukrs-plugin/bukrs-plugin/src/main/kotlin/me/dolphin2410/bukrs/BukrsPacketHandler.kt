package me.dolphin2410.bukrs

import io.netty.channel.ChannelHandlerContext
import io.netty.channel.SimpleChannelInboundHandler

class BukrsPacketHandler: SimpleChannelInboundHandler<Pair<Int, PacketType>>() {
    override fun channelRead0(ctx: ChannelHandlerContext, msg: Pair<Int, PacketType>) {
        BukrsEvents.dispatch(ctx, msg)
    }
}
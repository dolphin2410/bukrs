package me.dolphin2410.bukrs

import io.netty.channel.ChannelHandlerContext

annotation class BukrsEventHandler

interface BukrsListener

object BukrsEvents {
    private val targets = ArrayList<BukrsListener>()

    fun addListener(l: BukrsListener) {
        targets.add(l)
    }

    fun dispatch(ctx: ChannelHandlerContext, p: Pair<Int, PacketType>) {
        for (target in targets) {
            for (method in target::class.java.declaredMethods) {
                if (method.isAnnotationPresent(BukrsEventHandler::class.java)) {
                    if (method.parameterTypes[0].isInstance(ctx) && method.parameterTypes[1].kotlin.isInstance(p.first) && method.parameterTypes[2].isInstance(p.second)) {
                        method.invoke(target, ctx, p.first, p.second)
                    }
                }
            }
        }
    }
}
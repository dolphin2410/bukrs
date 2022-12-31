package me.dolphin2410.bukrs

import io.netty.buffer.ByteBuf
import io.netty.buffer.Unpooled
import io.netty.channel.ChannelHandlerContext
import io.netty.handler.codec.MessageToByteEncoder
import java.lang.RuntimeException

class BukrsEncoder: MessageToByteEncoder<Pair<Int, PacketType>>() {
    fun encodeHeader(size: Int, payloadId: Int, buf: ByteBuf) {
        writeVarInt(size, buf)
        buf.writeInt(payloadId)
    }

    fun encodePacket(packet: PacketType, buf: ByteBuf): Result<Unit> {
        val clazz = packet::class.java
        clazz.declaredFields.forEach {
            if (!hasCodec(it.type)) {
                return Result.failure(RuntimeException("Codec Unregistered for Type: ${it.type.name}"))
            }
            it.isAccessible = true
            val value = it.get(packet)

            @Suppress("Unchecked_Cast")
            encodeTypeUnsafe(it.type as Class<Any>, value, buf)
        }
        return Result.success(Unit)
    }

    override fun encode(ctx: ChannelHandlerContext, msg: Pair<Int, PacketType>, out: ByteBuf) {
        val payloadId = msg.first
        val packet = msg.second
        val buf = Unpooled.buffer(1024)
        encodeType(String::class.java, packet::class.java.simpleName, buf)
        encodePacket(packet, buf)
        encodeHeader(buf.readableBytes(), payloadId, out)
        out.writeBytes(buf)
    }
}
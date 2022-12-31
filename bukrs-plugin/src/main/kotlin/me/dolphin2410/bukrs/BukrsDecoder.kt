package me.dolphin2410.bukrs

import io.netty.buffer.ByteBuf
import io.netty.channel.ChannelHandlerContext
import io.netty.handler.codec.ByteToMessageDecoder
import java.lang.RuntimeException

class BukrsDecoder: ByteToMessageDecoder() {
    /**
     * Pair of PACKET_SIZE and PAYLOAD_ID
     */
    fun decodeHeader(buf: ByteBuf): Pair<Int, Int> {
        val size = readVarInt(buf).getOrThrow()
        val payloadId = buf.readInt()

        return size to payloadId
    }

    fun decodePacket(name: String, buf: ByteBuf): Result<PacketType> {
        val clazz = DefaultPackets::class.java.declaredClasses.find { it.simpleName == name } ?: return Result.failure(RuntimeException("Undefined Packet: $name"))
        val values = clazz.declaredFields.map {
            if (!hasCodec(it.type)) {
                return Result.failure(RuntimeException("Codec Unregistered for Type: ${it.type.name}"))
            }
            decodeType(it.type, buf)
        }
        val constructor = clazz.getDeclaredConstructor(*values.map { it::class.java }.toTypedArray())
        return Result.success(constructor.newInstance(*values.toTypedArray()) as PacketType)
    }

    override fun decode(ctx: ChannelHandlerContext, src: ByteBuf, out: MutableList<Any>) {
        val cloned = src.copy()
        if (cloned.readableBytes() < 6) return
        val header = decodeHeader(cloned)
        if (src.readableBytes() < cloned.readerIndex() + header.first) {
            return
        }

        src.readerIndex(cloned.readerIndex())

        val clazz = decodeType(String::class.java, cloned)

        val packet = decodePacket(clazz, cloned).getOrThrow()
        out.add(header.second to packet)

        src.readerIndex(src.readerIndex() + header.first)

    }
}
package me.dolphin2410.bukrs

import io.netty.buffer.ByteBuf

const val SEGMENT_BITS = 0x7F; // 2^7 - 1
const val CONTINUE_BIT = 0x80;

fun writeVarInt(value: Int, buffer: ByteBuf) {
    var latestValue = value   // This value is updated every loop

    while(true) {
        if (latestValue and SEGMENT_BITS.inv() == 0) {   // if value < 128
            buffer.writeByte(latestValue); // value can be coerced to an u8 type
            break; // Done.
        }

        buffer.writeByte((latestValue and SEGMENT_BITS) or CONTINUE_BIT)   // value can be coerced to a u8 type

        latestValue = (value shr 7); // 'unsigned shift right' ( >>> )
    }
}

fun readVarInt(src: ByteBuf): Result<Int> {
    var value = 0
    var position = 0
    var currentByte: Int

    while(true) {
        currentByte = src.readByte().toInt();
        value = value or (currentByte and SEGMENT_BITS) shl position;

        if ((currentByte and CONTINUE_BIT) == 0) { break; }

        position += 7;

        if (position >= 32) {
            return Result.failure(RuntimeException("Varint too big"));
        }
    }

    return Result.success(value)
}
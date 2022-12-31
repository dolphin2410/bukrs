package me.dolphin2410.bukrs

import io.netty.buffer.ByteBuf

internal val codecs = HashMap<Class<out Any>, TypeCodec<out Any>>()

interface TypeCodec<T> {
    fun decode(src: ByteBuf): T

    fun encode(src: T, target: ByteBuf)
}

fun <T: Any> pushCodec(clazz: Class<T>, codec: TypeCodec<T>) {
    codecs[clazz] = codec
}

fun hasCodec(clazz: Class<*>): Boolean {
    return codecs.containsKey(clazz)
}

@Suppress("Unchecked_Cast")
fun <T: Any> decodeType(clazz: Class<T>, buf: ByteBuf): T {
    return codecs[clazz]!!.decode(buf) as T
}

@Suppress("Unchecked_Cast")
fun <T: Any> encodeType(clazz: Class<T>, type: T, target: ByteBuf) {
    return (codecs[clazz]!! as TypeCodec<T>).encode(type, target)
}

@Suppress("Unchecked_Cast")
fun <T: Any> encodeTypeUnsafe(clazz: Class<Any>, type: T, target: ByteBuf) {
    return (codecs[clazz]!! as TypeCodec<T>).encode(type, target)
}

fun defaultCodecs() {
    pushCodec(Byte::class.java, object: TypeCodec<Byte> {
        override fun decode(src: ByteBuf): Byte {
            return src.readByte()
        }

        override fun encode(src: Byte, target: ByteBuf) {
            target.writeByte(src.toInt())
        }
    })

    pushCodec(Short::class.java, object: TypeCodec<Short> {
        override fun decode(src: ByteBuf): Short {
            return src.readShort()
        }

        override fun encode(src: Short, target: ByteBuf) {
            target.writeShort(src.toInt())
        }
    })

    pushCodec(Int::class.java, object: TypeCodec<Int> {
        override fun decode(src: ByteBuf): Int {
            return src.readInt()
        }

        override fun encode(src: Int, target: ByteBuf) {
            target.writeInt(src)
        }
    })

    pushCodec(Long::class.java, object: TypeCodec<Long> {
        override fun decode(src: ByteBuf): Long {
            return src.readLong()
        }

        override fun encode(src: Long, target: ByteBuf) {
            target.writeLong(src)
        }
    })

    pushCodec(UByte::class.java, object: TypeCodec<UByte> {
        override fun decode(src: ByteBuf): UByte {
            return src.readByte().toUByte()
        }

        override fun encode(src: UByte, target: ByteBuf) {
            target.writeByte(src.toInt())
        }
    })

    pushCodec(UShort::class.java, object: TypeCodec<UShort> {
        override fun decode(src: ByteBuf): UShort {
            return src.readShort().toUShort()
        }

        override fun encode(src: UShort, target: ByteBuf) {
            target.writeShort(src.toInt())
        }
    })

    pushCodec(UInt::class.java, object: TypeCodec<UInt> {
        override fun decode(src: ByteBuf): UInt {
            return src.readInt().toUInt()
        }

        override fun encode(src: UInt, target: ByteBuf) {
            target.writeInt(src.toInt())
        }
    })

    pushCodec(ULong::class.java, object: TypeCodec<ULong> {
        override fun decode(src: ByteBuf): ULong {
            return src.readLong().toULong()
        }

        override fun encode(src: ULong, target: ByteBuf) {
            target.writeLong(src.toLong())
        }
    })

    pushCodec(Float::class.java, object: TypeCodec<Float> {
        override fun decode(src: ByteBuf): Float {
            return src.readFloat()
        }

        override fun encode(src: Float, target: ByteBuf) {
            target.writeFloat(src)
        }
    })

    pushCodec(Double::class.java, object: TypeCodec<Double> {
        override fun decode(src: ByteBuf): Double {
            return src.readDouble()
        }

        override fun encode(src: Double, target: ByteBuf) {
            target.writeDouble(src)
        }
    })

    pushCodec(String::class.java, object: TypeCodec<String> {
        override fun decode(src: ByteBuf): String {
            val size = src.readInt()
            val buf = ByteArray(size)
            src.readBytes(buf)
            return String(buf)
        }

        override fun encode(src: String, target: ByteBuf) {
            val array = src.toByteArray()
            target.writeInt(array.size)
            target.writeBytes(array)
        }
    })
}
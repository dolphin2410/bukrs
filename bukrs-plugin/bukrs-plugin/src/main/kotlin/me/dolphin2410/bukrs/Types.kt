package me.dolphin2410.bukrs

import io.netty.buffer.ByteBuf
import java.util.UUID

data class PlayerData(val id: Int, val name: String, val uniqueId: UUID)

data class PlayerId(val id: Int)

data class InvfxId(val id: Int)

data class ItemStackWrapper(val name: String, val material: String)

data class InvSlotWrapper(val slot: Byte, val item: ItemStackWrapper)

data class InvListWrapper(val id: InvfxId, val internal: ArrayList<InvSlotWrapper>)

enum class InventorySize {
    Inv9,
    Inv18,
    Inv27,
    Inv36,
    Inv45,
    Inv54;

    init {
        pushCodec(InventorySize::class.java, object: TypeCodec<InventorySize> {
            override fun encode(src: InventorySize, target: ByteBuf) {
                val value = when (src) {
                    Inv9 -> 9
                    Inv18 -> 18
                    Inv27 -> 27
                    Inv36 -> 36
                    Inv45 -> 45
                    Inv54 -> 54
                }

                target.writeByte(value)
            }

            override fun decode(src: ByteBuf): InventorySize {
                return when (src.readByte().toInt()) {
                    9 -> Inv9
                    18 -> Inv18
                    27 -> Inv27
                    36 -> Inv36
                    45 -> Inv45
                    54 -> Inv54
                    else -> throw RuntimeException("Invalid InventorySize")
                }
            }
        })
    }
}
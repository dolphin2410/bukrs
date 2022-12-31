package me.dolphin2410.bukrs

@Target(AnnotationTarget.CLASS)
@Retention(AnnotationRetention.RUNTIME)
annotation class Packet {

}

interface PacketType

interface PacketGroup

class DefaultPackets: PacketGroup {
    @Packet
    class BukrsReqAPI: PacketType

    @Packet
    data class BukrsResAPI(val apiId: Int): PacketType

    @Packet
    class BukrsReqOnlinePlayers: PacketType

    @Packet
    data class BukrsResOnlinePlayers(val players: List<Int>): PacketType

    @Packet
    data class BukrsReqPlayerById(val player: Int): PacketType

    @Packet
    data class BukrsReqPlayerByName(val player: String): PacketType

    @Packet
    data class BukrsResPlayerData(val data: PlayerData): PacketType

    @Packet
    data class BukrsReqCreateInventory(val name: String, val size: InventorySize) // Request creation invfx

    @Packet
    data class BukrsResCreateInventory(val invId: InvfxId)    // Verify Invfx creation

    @Packet
    data class BukrsSDInvClick(val slot: Byte, val playerId: PlayerId)   // SD: Server Data

    @Packet
    data class BukrsSDInvOpen(val playerId: PlayerId)

    @Packet
    data class BukrsSDInvClose(val playerId: PlayerId)

    @Packet
    data class BukrsReqPlayerInvOpen(val inv_id: InvfxId, val playerId: PlayerId)

    @Packet
    class BukrsResPlayerInvOpen

    @Packet
    data class BukrsReqCreateInvList(val inv_id: InvfxId, val list: InvListWrapper)

    @Packet
    class BukrsResCreateInvList

    @Packet
    data class BukrsReqModifyInvList(val invId: InvfxId, val list: InvListWrapper)

    @Packet
    class BukrsResModifyInvList
}
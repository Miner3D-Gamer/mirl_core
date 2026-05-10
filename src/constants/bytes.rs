/// A singular byte (8 bits)
pub const BYTE: u8 = 1;
/// A Kilobyte, 1024 bytes
pub const KB: u16 = BYTE as u16 * 1024;
/// A Megabyte, 1024 kilobytes
pub const MB: u32 = KB as u32 * 1024;
/// A Gigabyte, 1024 megabytes
pub const GB: u32 = MB * 1024;
/// A Terabyte, 1024 gigabytes
pub const TB: u64 = GB as u64 * 1024;
/// A Petabyte, 1024 terabytes
pub const PB: u64 = TB * 1024;
/// An Exabyte, 1024 petabytes
pub const EB: u64 = PB * 1024;
/// A Zettabyte, 1024 exabytes
pub const ZB: u128 = EB as u128 * 1024;
/// A Yottabyte, 1024 zettabytes
pub const YB: u128 = ZB * 1024;
/// A Ronnabyte, 1024 yottabytes
pub const RB: u128 = YB * 1024;
/// A Quettabyte, 1024 ronnabytes
pub const QB: u128 = RB * 1024;
/// A Brontobyte, 1024 quettabytes
pub const BB: u128 = QB * 1024;
/// A Geopbyte, 1024 brontobytes
pub const GPB: u128 = BB * 1024; // Tripple letters!
// /// A Xenottabyte, 1024 geopbytes
// pub const XENOTTABYTE: u128 = GEOPBYTE * 1024;

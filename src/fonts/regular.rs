// DO NOT MAKE EDITS HERE because this file is automatically generated.
// To make changes, see blitstr2/codegen/main.go
//
// This code includes encoded bitmaps of glyphs from the Geneva typeface which
// was designed by Susan Kare and released by Apple in 1984. Geneva is a
// registered trademark of Apple Inc. The PNG sprite sheets of Geneva glyphs came
// from screenshots taken on Macintosh System 7, with additional Latin Extended
// characters added by Sam Blenny.
//
//! Regular Font
#![allow(dead_code)]

/// Maximum height of glyph patterns in this bitmap typeface.
pub const MAX_HEIGHT: u8 = 15;

/// Unicode character codepoints corresponding to glyph sprites in GLYPHS array.
/// Indended use:
///  1. Do binary search on CODEPOINTS to find index of the codepoint corresponding
///     to the glyph you want to locate
///  2. Multiply resulting CODEPOINTS index by 8 (<<3) to get index into GLYPHS for
///     the corresponding glyph sprite (because 16*16px sprite size is 8*u32)
pub const CODEPOINTS: [u32; 206] = [
0x00020,
0x00021,
0x00022,
0x00023,
0x00024,
0x00025,
0x00026,
0x00027,
0x00028,
0x00029,
0x0002A,
0x0002B,
0x0002C,
0x0002D,
0x0002E,
0x0002F,
0x00030,
0x00031,
0x00032,
0x00033,
0x00034,
0x00035,
0x00036,
0x00037,
0x00038,
0x00039,
0x0003A,
0x0003B,
0x0003C,
0x0003D,
0x0003E,
0x0003F,
0x00040,
0x00041,
0x00042,
0x00043,
0x00044,
0x00045,
0x00046,
0x00047,
0x00048,
0x00049,
0x0004A,
0x0004B,
0x0004C,
0x0004D,
0x0004E,
0x0004F,
0x00050,
0x00051,
0x00052,
0x00053,
0x00054,
0x00055,
0x00056,
0x00057,
0x00058,
0x00059,
0x0005A,
0x0005B,
0x0005C,
0x0005D,
0x0005E,
0x0005F,
0x00060,
0x00061,
0x00062,
0x00063,
0x00064,
0x00065,
0x00066,
0x00067,
0x00068,
0x00069,
0x0006A,
0x0006B,
0x0006C,
0x0006D,
0x0006E,
0x0006F,
0x00070,
0x00071,
0x00072,
0x00073,
0x00074,
0x00075,
0x00076,
0x00077,
0x00078,
0x00079,
0x0007A,
0x0007B,
0x0007C,
0x0007D,
0x0007E,
0x000A0,
0x000A1,
0x000A2,
0x000A3,
0x000A4,
0x000A5,
0x000A6,
0x000A7,
0x000A8,
0x000A9,
0x000AA,
0x000AB,
0x000AC,
0x000AD,
0x000AE,
0x000AF,
0x000B0,
0x000B1,
0x000B2,
0x000B3,
0x000B4,
0x000B5,
0x000B6,
0x000B7,
0x000B8,
0x000B9,
0x000BA,
0x000BB,
0x000BC,
0x000BD,
0x000BE,
0x000BF,
0x000C0,
0x000C1,
0x000C2,
0x000C3,
0x000C4,
0x000C5,
0x000C6,
0x000C7,
0x000C8,
0x000C9,
0x000CA,
0x000CB,
0x000CC,
0x000CD,
0x000CE,
0x000CF,
0x000D0,
0x000D1,
0x000D2,
0x000D3,
0x000D4,
0x000D5,
0x000D6,
0x000D7,
0x000D8,
0x000D9,
0x000DA,
0x000DB,
0x000DC,
0x000DD,
0x000DE,
0x000DF,
0x000E0,
0x000E1,
0x000E2,
0x000E3,
0x000E4,
0x000E5,
0x000E6,
0x000E7,
0x000E8,
0x000E9,
0x000EA,
0x000EB,
0x000EC,
0x000ED,
0x000EE,
0x000EF,
0x000F0,
0x000F1,
0x000F2,
0x000F3,
0x000F4,
0x000F5,
0x000F6,
0x000F7,
0x000F8,
0x000F9,
0x000FA,
0x000FB,
0x000FC,
0x000FD,
0x000FE,
0x000FF,
0x00152,
0x00153,
0x02018,
0x02019,
0x0201A,
0x0201B,
0x0201C,
0x0201D,
0x0201E,
0x0201F,
0x02020,
0x02021,
0x02022,
0x020AC,
0x0FFFD,
];

/// Packed 16px * 16px glyph pattern data.
/// Pixels are packed in row-major order with LSB of first pixel word
/// containing the top left pixel. Bit of 0 means clear, 1 means set
pub const GLYPHS: [u32; 1648] = [
0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00010000, 0x00010001, 0x00010001, 0x00010001, 0x00010000, 0x00000000, 0x00000000,
0x00000000, 0x00050000, 0x00000005, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00280000, 0x0014007e, 0x000a003f, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x000e0004, 0x00050015, 0x000c0006, 0x00140014, 0x000e0015, 0x00000004, 0x00000000,
0x00000000, 0x007e0000, 0x00290049, 0x00080016, 0x004a0034, 0x00310049, 0x00000000, 0x00000000,
0x00000000, 0x000c0000, 0x000a0012, 0x000a0004, 0x00210051, 0x008e0051, 0x00000000, 0x00000000,
0x00000000, 0x00010000, 0x00000001, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00020004, 0x00010002, 0x00010001, 0x00010001, 0x00020002, 0x00000004, 0x00000000,
0x00000000, 0x00020001, 0x00040002, 0x00040004, 0x00040004, 0x00020002, 0x00000001, 0x00000000,
0x00000000, 0x00000000, 0x00150004, 0x000a000e, 0x00000011, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00040000, 0x001f0004, 0x00040004, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00020000, 0x00010002, 0x00000000,
0x00000000, 0x00000000, 0x00000000, 0x001f0000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00010000, 0x00000000, 0x00000000,
0x00000000, 0x00100010, 0x00080008, 0x00040004, 0x00020002, 0x00010001, 0x00000000, 0x00000000,
0x00000000, 0x000c0000, 0x00210012, 0x00210021, 0x00210021, 0x000c0012, 0x00000000, 0x00000000,
0x00000000, 0x00020000, 0x00020003, 0x00020002, 0x00020002, 0x00020002, 0x00000000, 0x00000000,
0x00000000, 0x001c0000, 0x00210022, 0x00100020, 0x00040008, 0x003f0002, 0x00000000, 0x00000000,
0x00000000, 0x003f0000, 0x00080010, 0x0010000c, 0x00200020, 0x000e0011, 0x00000000, 0x00000000,
0x00000000, 0x00200000, 0x00280030, 0x00220024, 0x007f0021, 0x00200020, 0x00000000, 0x00000000,
0x00000000, 0x003f0000, 0x00010001, 0x0020001f, 0x00200020, 0x001e0021, 0x00000000, 0x00000000,
0x00000000, 0x001c0000, 0x00010002, 0x0021001f, 0x00210021, 0x001e0021, 0x00000000, 0x00000000,
0x00000000, 0x003f0000, 0x00100020, 0x00080010, 0x00040008, 0x00040004, 0x00000000, 0x00000000,
0x00000000, 0x001e0000, 0x00210021, 0x0021001e, 0x00210021, 0x001e0021, 0x00000000, 0x00000000,
0x00000000, 0x001e0000, 0x00210021, 0x00210021, 0x0020003e, 0x000e0010, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00000000, 0x00000001, 0x00000000, 0x00010000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00000000, 0x00000002, 0x00000000, 0x00020000, 0x00010002, 0x00000000,
0x00000000, 0x00000000, 0x00080000, 0x00020004, 0x00020001, 0x00080004, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00000000, 0x001f0000, 0x00000000, 0x0000001f, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00010000, 0x00040002, 0x00040008, 0x00010002, 0x00000000, 0x00000000,
0x00000000, 0x001e0000, 0x00210021, 0x00080010, 0x00040004, 0x00040000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x0042003c, 0x00a500b9, 0x007900a5, 0x001c0002, 0x00000000, 0x00000000,
0x00000000, 0x00080000, 0x00140008, 0x00220014, 0x007f0022, 0x00410041, 0x00000000, 0x00000000,
0x00000000, 0x001f0000, 0x00210021, 0x0021001f, 0x00210021, 0x001f0021, 0x00000000, 0x00000000,
0x00000000, 0x001e0000, 0x00010021, 0x00010001, 0x00010001, 0x001e0021, 0x00000000, 0x00000000,
0x00000000, 0x000f0000, 0x00210011, 0x00210021, 0x00210021, 0x000f0011, 0x00000000, 0x00000000,
0x00000000, 0x001f0000, 0x00010001, 0x000f0001, 0x00010001, 0x001f0001, 0x00000000, 0x00000000,
0x00000000, 0x001f0000, 0x00010001, 0x000f0001, 0x00010001, 0x00010001, 0x00000000, 0x00000000,
0x00000000, 0x001e0000, 0x00010021, 0x00390001, 0x00210021, 0x001e0021, 0x00000000, 0x00000000,
0x00000000, 0x00210000, 0x00210021, 0x003f0021, 0x00210021, 0x00210021, 0x00000000, 0x00000000,
0x00000000, 0x00010000, 0x00010001, 0x00010001, 0x00010001, 0x00010001, 0x00000000, 0x00000000,
0x00000000, 0x00200000, 0x00200020, 0x00200020, 0x00210020, 0x001e0021, 0x00000000, 0x00000000,
0x00000000, 0x00210000, 0x00090011, 0x00030005, 0x00090005, 0x00210011, 0x00000000, 0x00000000,
0x00000000, 0x00010000, 0x00010001, 0x00010001, 0x00010001, 0x001f0001, 0x00000000, 0x00000000,
0x00000000, 0x00410000, 0x00550063, 0x00410049, 0x00410041, 0x00410041, 0x00000000, 0x00000000,
0x00000000, 0x00230000, 0x00250023, 0x00290025, 0x00310029, 0x00210031, 0x00000000, 0x00000000,
0x00000000, 0x001e0000, 0x00210021, 0x00210021, 0x00210021, 0x001e0021, 0x00000000, 0x00000000,
0x00000000, 0x001f0000, 0x00210021, 0x001f0021, 0x00010001, 0x00010001, 0x00000000, 0x00000000,
0x00000000, 0x001e0000, 0x00210021, 0x00210021, 0x00210021, 0x001e0021, 0x00300008, 0x00000000,
0x00000000, 0x001f0000, 0x00210021, 0x001f0021, 0x00110009, 0x00210021, 0x00000000, 0x00000000,
0x00000000, 0x001e0000, 0x00010021, 0x001e0001, 0x00200020, 0x001e0021, 0x00000000, 0x00000000,
0x00000000, 0x007f0000, 0x00080008, 0x00080008, 0x00080008, 0x00080008, 0x00000000, 0x00000000,
0x00000000, 0x00210000, 0x00210021, 0x00210021, 0x00210021, 0x001e0021, 0x00000000, 0x00000000,
0x00000000, 0x00410000, 0x00410041, 0x00220022, 0x00140014, 0x00080008, 0x00000000, 0x00000000,
0x00000000, 0x01010000, 0x01110101, 0x00aa0092, 0x004400aa, 0x00440044, 0x00000000, 0x00000000,
0x00000000, 0x00110000, 0x000a0011, 0x0004000a, 0x000a000a, 0x00110011, 0x00000000, 0x00000000,
0x00000000, 0x00110000, 0x00110011, 0x000a000a, 0x00040004, 0x00040004, 0x00000000, 0x00000000,
0x00000000, 0x001f0000, 0x00080010, 0x00040008, 0x00020002, 0x001f0001, 0x00000000, 0x00000000,
0x00000000, 0x00010003, 0x00010001, 0x00010001, 0x00010001, 0x00010001, 0x00000003, 0x00000000,
0x00000000, 0x00010001, 0x00020002, 0x00040004, 0x00080008, 0x00100010, 0x00000000, 0x00000000,
0x00000000, 0x00020003, 0x00020002, 0x00020002, 0x00020002, 0x00020002, 0x00000003, 0x00000000,
0x00000000, 0x00020000, 0x00050002, 0x00000005, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00ff0000, 0x00000000, 0x00000000,
0x00000000, 0x00010000, 0x00000002, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x000e0000, 0x00100011, 0x0011001e, 0x001e0011, 0x00000000, 0x00000000,
0x00000000, 0x00010000, 0x000f0001, 0x00110011, 0x00110011, 0x000f0011, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x000e0000, 0x00010011, 0x00010001, 0x000e0011, 0x00000000, 0x00000000,
0x00000000, 0x00100000, 0x001e0010, 0x00110011, 0x00110011, 0x001e0011, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x000e0000, 0x00110011, 0x0001001f, 0x000e0011, 0x00000000, 0x00000000,
0x00000000, 0x000c0000, 0x00070002, 0x00020002, 0x00020002, 0x00020002, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x001e0000, 0x00110011, 0x00110011, 0x001e0011, 0x00110010, 0x0000000e,
0x00000000, 0x00010000, 0x000d0001, 0x00110013, 0x00110011, 0x00110011, 0x00000000, 0x00000000,
0x00000000, 0x00010000, 0x00010000, 0x00010001, 0x00010001, 0x00010001, 0x00000000, 0x00000000,
0x00000000, 0x00040000, 0x00040000, 0x00040004, 0x00040004, 0x00040004, 0x00040004, 0x00000003,
0x00000000, 0x00010000, 0x00110001, 0x00050009, 0x00050003, 0x00110009, 0x00000000, 0x00000000,
0x00000000, 0x00010000, 0x00010001, 0x00010001, 0x00010001, 0x00010001, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00cd0000, 0x01110133, 0x01110111, 0x01110111, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x000d0000, 0x00110013, 0x00110011, 0x00110011, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x000e0000, 0x00110011, 0x00110011, 0x000e0011, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x000f0000, 0x00110011, 0x00110011, 0x000f0011, 0x00010001, 0x00000000,
0x00000000, 0x00000000, 0x001e0000, 0x00110011, 0x00110011, 0x001e0011, 0x00100010, 0x00000000,
0x00000000, 0x00000000, 0x001d0000, 0x00010003, 0x00010001, 0x00010001, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x000e0000, 0x00010011, 0x0010000e, 0x000e0011, 0x00000000, 0x00000000,
0x00000000, 0x00020000, 0x00070002, 0x00020002, 0x00020002, 0x000c0002, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00110000, 0x00110011, 0x00110011, 0x00160019, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00110000, 0x00110011, 0x000a000a, 0x00040004, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x01110000, 0x00aa0111, 0x004400aa, 0x00440044, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00110000, 0x000a0011, 0x000a0004, 0x00110011, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00220000, 0x00220022, 0x00140014, 0x00080008, 0x00040004, 0x00000003,
0x00000000, 0x00000000, 0x001f0000, 0x00080010, 0x00020004, 0x001f0001, 0x00000000, 0x00000000,
0x00000000, 0x00020004, 0x00020002, 0x00010002, 0x00020002, 0x00020002, 0x00000004, 0x00000000,
0x00000000, 0x00010001, 0x00010001, 0x00010001, 0x00010001, 0x00010001, 0x00000001, 0x00000000,
0x00000000, 0x00020001, 0x00020002, 0x00040002, 0x00020002, 0x00020002, 0x00000001, 0x00000000,
0x00000000, 0x00260000, 0x00000019, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00010000, 0x00010000, 0x00010001, 0x00010001, 0x00010001, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x000e0004, 0x00050015, 0x00050005, 0x000e0015, 0x00000004, 0x00000000,
0x00000000, 0x00180000, 0x00040004, 0x000f0004, 0x00020002, 0x003f0002, 0x00000000, 0x00000000,
0x00000000, 0x00120000, 0x0012000c, 0x00210021, 0x000c0012, 0x00000012, 0x00000000, 0x00000000,
0x00000000, 0x00110000, 0x001f000a, 0x001f0004, 0x00040004, 0x00040004, 0x00000000, 0x00000000,
0x00000000, 0x00010001, 0x00010001, 0x00000001, 0x00010001, 0x00010001, 0x00000001, 0x00000000,
0x00000000, 0x001e0000, 0x00010021, 0x00190006, 0x00260021, 0x00200018, 0x001e0021, 0x00000000,
0x00000000, 0x00050000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x0042003c, 0x00850099, 0x00990085, 0x003c0042, 0x00000000, 0x00000000,
0x00000000, 0x00060000, 0x000e0008, 0x000e0009, 0x000f0000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00000000, 0x00240000, 0x00090012, 0x00240012, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00000000, 0x001f0000, 0x00100010, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00000000, 0x001f0000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x0042003c, 0x00a5009d, 0x00a5009d, 0x003c0042, 0x00000000, 0x00000000,
0x00000000, 0x00070000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00060000, 0x00090009, 0x00000006, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00040000, 0x001f0004, 0x00040004, 0x001f0000, 0x00000000, 0x00000000,
0x00070000, 0x00070004, 0x00070001, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00070000, 0x00070004, 0x00070004, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00020000, 0x00000001, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00880000, 0x00440088, 0x00440044, 0x005e0022, 0x00010001, 0x00000000,
0x00000000, 0x007e0000, 0x002f002f, 0x002e002f, 0x00280028, 0x00280028, 0x00280028, 0x00000000,
0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000001, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00020001, 0x00000003,
0x00020000, 0x00020003, 0x00070002, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00060000, 0x00090009, 0x00060009, 0x000f0000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00000000, 0x00090000, 0x00240012, 0x00090012, 0x00000000, 0x00000000,
0x00420000, 0x00220043, 0x00170022, 0x01480150, 0x010401c8, 0x00000104, 0x00000000, 0x00000000,
0x00420000, 0x00220043, 0x00170022, 0x010801d0, 0x004401c8, 0x000001c4, 0x00000000, 0x00000000,
0x00470000, 0x00270044, 0x00170024, 0x01480150, 0x010401c8, 0x00000104, 0x00000000, 0x00000000,
0x00000000, 0x00040000, 0x00040000, 0x00020004, 0x00110001, 0x000e0011, 0x00000000, 0x00000000,
0x00080004, 0x00080000, 0x00140008, 0x00220014, 0x007f0022, 0x00410041, 0x00000000, 0x00000000,
0x00080010, 0x00080000, 0x00140008, 0x00220014, 0x007f0022, 0x00410041, 0x00000000, 0x00000000,
0x00140008, 0x00080000, 0x00140008, 0x00220014, 0x007f0022, 0x00410041, 0x00000000, 0x00000000,
0x0032004c, 0x00080000, 0x00140008, 0x00220014, 0x007f0022, 0x00410041, 0x00000000, 0x00000000,
0x00140000, 0x00080000, 0x00140008, 0x00220014, 0x007f0022, 0x00410041, 0x00000000, 0x00000000,
0x00080000, 0x00080014, 0x00140008, 0x00220014, 0x007f0022, 0x00410041, 0x00000000, 0x00000000,
0x00000000, 0x03f00000, 0x00280028, 0x01fc0024, 0x00220022, 0x03e10021, 0x00000000, 0x00000000,
0x00000000, 0x001e0000, 0x00010021, 0x00010001, 0x00010001, 0x001e0021, 0x00100008, 0x00000008,
0x00040002, 0x001f0000, 0x00010001, 0x000f0001, 0x00010001, 0x001f0001, 0x00000000, 0x00000000,
0x00040008, 0x001f0000, 0x00010001, 0x000f0001, 0x00010001, 0x001f0001, 0x00000000, 0x00000000,
0x000a0004, 0x001f0000, 0x00010001, 0x000f0001, 0x00010001, 0x001f0001, 0x00000000, 0x00000000,
0x000a0000, 0x001f0000, 0x00010001, 0x000f0001, 0x00010001, 0x001f0001, 0x00000000, 0x00000000,
0x00020001, 0x00020000, 0x00020002, 0x00020002, 0x00020002, 0x00020002, 0x00000000, 0x00000000,
0x00010002, 0x00010000, 0x00010001, 0x00010001, 0x00010001, 0x00010001, 0x00000000, 0x00000000,
0x00050002, 0x00020000, 0x00020002, 0x00020002, 0x00020002, 0x00020002, 0x00000000, 0x00000000,
0x00050000, 0x00020000, 0x00020002, 0x00020002, 0x00020002, 0x00020002, 0x00000000, 0x00000000,
0x00000000, 0x001e0000, 0x00420022, 0x00470042, 0x00420042, 0x001e0022, 0x00000000, 0x00000000,
0x00190026, 0x00230000, 0x00250023, 0x00290025, 0x00310029, 0x00210031, 0x00000000, 0x00000000,
0x00080004, 0x001e0000, 0x00210021, 0x00210021, 0x00210021, 0x001e0021, 0x00000000, 0x00000000,
0x00040008, 0x001e0000, 0x00210021, 0x00210021, 0x00210021, 0x001e0021, 0x00000000, 0x00000000,
0x000a0004, 0x001e0000, 0x00210021, 0x00210021, 0x00210021, 0x001e0021, 0x00000000, 0x00000000,
0x00190026, 0x001e0000, 0x00210021, 0x00210021, 0x00210021, 0x001e0021, 0x00000000, 0x00000000,
0x00120000, 0x001e0000, 0x00210021, 0x00210021, 0x00210021, 0x001e0021, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00000000, 0x00110000, 0x0004000a, 0x0011000a, 0x00000000, 0x00000000,
0x00000000, 0x00bc0000, 0x00620042, 0x004a0052, 0x00420046, 0x003c0043, 0x00000000, 0x00000000,
0x00080004, 0x00210000, 0x00210021, 0x00210021, 0x00210021, 0x001e0021, 0x00000000, 0x00000000,
0x00040008, 0x00210000, 0x00210021, 0x00210021, 0x00210021, 0x001e0021, 0x00000000, 0x00000000,
0x000a0004, 0x00210000, 0x00210021, 0x00210021, 0x00210021, 0x001e0021, 0x00000000, 0x00000000,
0x00120000, 0x00210000, 0x00210021, 0x00210021, 0x00210021, 0x001e0021, 0x00000000, 0x00000000,
0x00040008, 0x00110000, 0x00110011, 0x000a000a, 0x00040004, 0x00040004, 0x00000000, 0x00000000,
0x00000000, 0x00010000, 0x000f0001, 0x00110011, 0x000f0011, 0x00010001, 0x00000000, 0x00000000,
0x00000000, 0x000e0000, 0x00090011, 0x00050005, 0x00210019, 0x001d0021, 0x00000000, 0x00000000,
0x00000000, 0x00080004, 0x000e0000, 0x001e0011, 0x00110011, 0x001e0011, 0x00000000, 0x00000000,
0x00000000, 0x00020004, 0x000e0000, 0x001e0011, 0x00110011, 0x001e0011, 0x00000000, 0x00000000,
0x00000000, 0x000a0004, 0x000e0000, 0x001e0011, 0x00110011, 0x001e0011, 0x00000000, 0x00000000,
0x00000000, 0x000d0016, 0x000e0000, 0x001e0011, 0x00110011, 0x001e0011, 0x00000000, 0x00000000,
0x00000000, 0x000a0000, 0x000e0000, 0x001e0011, 0x00110011, 0x001e0011, 0x00000000, 0x00000000,
0x00090006, 0x00060009, 0x000e0000, 0x001e0011, 0x00110011, 0x001e0011, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00ee0000, 0x01100111, 0x001101fe, 0x00ee0111, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x000e0000, 0x00010011, 0x00010001, 0x000e0011, 0x00020004, 0x00000000,
0x00000000, 0x00040002, 0x000e0000, 0x00110011, 0x0001001f, 0x000e0011, 0x00000000, 0x00000000,
0x00000000, 0x00020004, 0x000e0000, 0x00110011, 0x0001001f, 0x000e0011, 0x00000000, 0x00000000,
0x00000000, 0x000a0004, 0x000e0000, 0x00110011, 0x0001001f, 0x000e0011, 0x00000000, 0x00000000,
0x00000000, 0x000a0000, 0x000e0000, 0x00110011, 0x0001001f, 0x000e0011, 0x00000000, 0x00000000,
0x00000000, 0x00020001, 0x00020000, 0x00020002, 0x00020002, 0x00020002, 0x00000000, 0x00000000,
0x00000000, 0x00010002, 0x00010000, 0x00010001, 0x00010001, 0x00010001, 0x00000000, 0x00000000,
0x00000000, 0x00050002, 0x00020000, 0x00020002, 0x00020002, 0x00020002, 0x00000000, 0x00000000,
0x00000000, 0x00050000, 0x00020000, 0x00020002, 0x00020002, 0x00020002, 0x00000000, 0x00000000,
0x00000000, 0x000a0001, 0x000c0007, 0x00110012, 0x00110011, 0x00060009, 0x00000000, 0x00000000,
0x00000000, 0x000d0016, 0x000d0000, 0x00110013, 0x00110011, 0x00110011, 0x00000000, 0x00000000,
0x00000000, 0x00040002, 0x000e0000, 0x00110011, 0x00110011, 0x000e0011, 0x00000000, 0x00000000,
0x00000000, 0x00040008, 0x000e0000, 0x00110011, 0x00110011, 0x000e0011, 0x00000000, 0x00000000,
0x00000000, 0x000a0004, 0x000e0000, 0x00110011, 0x00110011, 0x000e0011, 0x00000000, 0x00000000,
0x00000000, 0x000d0016, 0x000e0000, 0x00110011, 0x00110011, 0x000e0011, 0x00000000, 0x00000000,
0x00000000, 0x000a0000, 0x000e0000, 0x00110011, 0x00110011, 0x000e0011, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00040000, 0x001f0000, 0x00040000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x005c0000, 0x00320022, 0x0026002a, 0x001d0022, 0x00000000, 0x00000000,
0x00000000, 0x00040002, 0x00110000, 0x00110011, 0x00110011, 0x00160019, 0x00000000, 0x00000000,
0x00000000, 0x00040008, 0x00110000, 0x00110011, 0x00110011, 0x00160019, 0x00000000, 0x00000000,
0x00000000, 0x000a0004, 0x00110000, 0x00110011, 0x00110011, 0x00160019, 0x00000000, 0x00000000,
0x00000000, 0x000a0000, 0x00110000, 0x00110011, 0x00110011, 0x00160019, 0x00000000, 0x00000000,
0x00000000, 0x00040008, 0x00220000, 0x00220022, 0x00140014, 0x00080008, 0x00040004, 0x00000003,
0x00000000, 0x00010000, 0x000f0001, 0x00110011, 0x00110011, 0x000f0011, 0x00010001, 0x00000000,
0x00000000, 0x00140000, 0x00220000, 0x00220022, 0x00140014, 0x00080008, 0x00040004, 0x00000003,
0x00000000, 0x03de0000, 0x00210021, 0x01e10021, 0x00210021, 0x03de0021, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00ee0000, 0x01110111, 0x001101f1, 0x00ee0111, 0x00000000, 0x00000000,
0x00000000, 0x00010002, 0x00000001, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00020002, 0x00000001, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00020000, 0x00010002, 0x00000000,
0x00000000, 0x00010001, 0x00000002, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00090012, 0x00000009, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00120012, 0x00000009, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00120000, 0x00090012, 0x00000000,
0x00000000, 0x00090009, 0x00000012, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00040000, 0x001f0004, 0x00040004, 0x00040004, 0x00040004, 0x00000000, 0x00000000,
0x00000000, 0x00040000, 0x001f0004, 0x00040004, 0x00040004, 0x001f0004, 0x00040004, 0x00000000,
0x00000000, 0x00000000, 0x001e0000, 0x003f003f, 0x003f003f, 0x0000001e, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00840078, 0x003f0002, 0x003f0002, 0x00780084, 0x00000000, 0x00000000,
0x00000000, 0x00380010, 0x00d6006c, 0x01ef01df, 0x006c00fe, 0x00100038, 0x00000000, 0x00000000,
];

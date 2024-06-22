#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _0_cs: _0Cs,
    _0_conblk_ad: _0ConblkAd,
    _0_ti: _0Ti,
    _0_source_ad: _0SourceAd,
    _0_dest_ad: _0DestAd,
    _0_txfr_len: _0TxfrLen,
    _0_stride: _0Stride,
    _0_nextconbk: _0Nextconbk,
    _0_debug: _0Debug,
    _reserved9: [u8; 0xdc],
    _1_cs: _1Cs,
    _1_conblk_ad: _1ConblkAd,
    _1_ti: _1Ti,
    _1_source_ad: _1SourceAd,
    _1_dest_ad: _1DestAd,
    _1_txfr_len: _1TxfrLen,
    _1_stride: _1Stride,
    _1_nextconbk: _1Nextconbk,
    _1_debug: _1Debug,
    _reserved18: [u8; 0xdc],
    _2_cs: _2Cs,
    _2_conblk_ad: _2ConblkAd,
    _2_ti: _2Ti,
    _2_source_ad: _2SourceAd,
    _2_dest_ad: _2DestAd,
    _2_txfr_len: _2TxfrLen,
    _2_stride: _2Stride,
    _2_nextconbk: _2Nextconbk,
    _2_debug: _2Debug,
    _reserved27: [u8; 0xdc],
    _3_cs: _3Cs,
    _3_conblk_ad: _3ConblkAd,
    _3_ti: _3Ti,
    _3_source_ad: _3SourceAd,
    _3_dest_ad: _3DestAd,
    _3_txfr_len: _3TxfrLen,
    _3_stride: _3Stride,
    _3_nextconbk: _3Nextconbk,
    _3_debug: _3Debug,
    _reserved36: [u8; 0xdc],
    _4_cs: _4Cs,
    _4_conblk_ad: _4ConblkAd,
    _4_ti: _4Ti,
    _4_source_ad: _4SourceAd,
    _4_dest_ad: _4DestAd,
    _4_txfr_len: _4TxfrLen,
    _4_stride: _4Stride,
    _4_nextconbk: _4Nextconbk,
    _4_debug: _4Debug,
    _reserved45: [u8; 0xdc],
    _5_cs: _5Cs,
    _5_conblk_ad: _5ConblkAd,
    _5_ti: _5Ti,
    _5_source_ad: _5SourceAd,
    _5_dest_ad: _5DestAd,
    _5_txfr_len: _5TxfrLen,
    _5_stride: _5Stride,
    _5_nextconbk: _5Nextconbk,
    _5_debug: _5Debug,
    _reserved54: [u8; 0xdc],
    _6_cs: _6Cs,
    _6_conblk_ad: _6ConblkAd,
    _6_ti: _6Ti,
    _6_source_ad: _6SourceAd,
    _6_dest_ad: _6DestAd,
    _6_txfr_len: _6TxfrLen,
    _6_stride: _6Stride,
    _6_nextconbk: _6Nextconbk,
    _6_debug: _6Debug,
    _reserved63: [u8; 0xdc],
    _7_cs: _7Cs,
    _7_conblk_ad: _7ConblkAd,
    _7_ti: _7Ti,
    _7_source_ad: _7SourceAd,
    _7_dest_ad: _7DestAd,
    _7_txfr_len: _7TxfrLen,
    _7_stride: _7Stride,
    _7_nextconbk: _7Nextconbk,
    _7_debug: _7Debug,
    _reserved72: [u8; 0xdc],
    _8_cs: _8Cs,
    _8_conblk_ad: _8ConblkAd,
    _8_ti: _8Ti,
    _8_source_ad: _8SourceAd,
    _8_dest_ad: _8DestAd,
    _8_txfr_len: _8TxfrLen,
    _8_stride: _8Stride,
    _8_nextconbk: _8Nextconbk,
    _8_debug: _8Debug,
    _reserved81: [u8; 0xdc],
    _9_cs: _9Cs,
    _9_conblk_ad: _9ConblkAd,
    _9_ti: _9Ti,
    _9_source_ad: _9SourceAd,
    _9_dest_ad: _9DestAd,
    _9_txfr_len: _9TxfrLen,
    _9_stride: _9Stride,
    _9_nextconbk: _9Nextconbk,
    _9_debug: _9Debug,
    _reserved90: [u8; 0xdc],
    _10_cs: _10Cs,
    _10_conblk_ad: _10ConblkAd,
    _10_ti: _10Ti,
    _10_source_ad: _10SourceAd,
    _10_dest_ad: _10DestAd,
    _10_txfr_len: _10TxfrLen,
    _10_stride: _10Stride,
    _10_nextconbk: _10Nextconbk,
    _10_debug: _10Debug,
    _reserved99: [u8; 0xdc],
    _11_cs: _11Cs,
    _11_conblk_ad: _11ConblkAd,
    _11_ti: _11Ti,
    _11_source_ad: _11SourceAd,
    _11_dest_ad: _11DestAd,
    _11_txfr_len: _11TxfrLen,
    _11_stride: _11Stride,
    _11_nextconbk: _11Nextconbk,
    _11_debug: _11Debug,
    _reserved108: [u8; 0xdc],
    _12_cs: _12Cs,
    _12_conblk_ad: _12ConblkAd,
    _12_ti: _12Ti,
    _12_source_ad: _12SourceAd,
    _12_dest_ad: _12DestAd,
    _12_txfr_len: _12TxfrLen,
    _12_stride: _12Stride,
    _12_nextconbk: _12Nextconbk,
    _12_debug: _12Debug,
    _reserved117: [u8; 0xdc],
    _13_cs: _13Cs,
    _13_conblk_ad: _13ConblkAd,
    _13_ti: _13Ti,
    _13_source_ad: _13SourceAd,
    _13_dest_ad: _13DestAd,
    _13_txfr_len: _13TxfrLen,
    _13_stride: _13Stride,
    _13_nextconbk: _13Nextconbk,
    _13_debug: _13Debug,
    _reserved126: [u8; 0xdc],
    _14_cs: _14Cs,
    _14_conblk_ad: _14ConblkAd,
    _14_ti: _14Ti,
    _14_source_ad: _14SourceAd,
    _14_dest_ad: _14DestAd,
    _14_txfr_len: _14TxfrLen,
    _14_stride: _14Stride,
    _14_nextconbk: _14Nextconbk,
    _14_debug: _14Debug,
    _reserved135: [u8; 0x01bc],
    int_status: IntStatus,
    _reserved136: [u8; 0x0c],
    enable: Enable,
}
impl RegisterBlock {
    #[doc = "0x00 - DMA Channel 0 Control and Status"]
    #[inline(always)]
    pub const fn _0_cs(&self) -> &_0Cs {
        &self._0_cs
    }
    #[doc = "0x04 - DMA Channel 0 Control Block Address"]
    #[inline(always)]
    pub const fn _0_conblk_ad(&self) -> &_0ConblkAd {
        &self._0_conblk_ad
    }
    #[doc = "0x08 - DMA Channel 0 CB Word 0 (Transfer Information)"]
    #[inline(always)]
    pub const fn _0_ti(&self) -> &_0Ti {
        &self._0_ti
    }
    #[doc = "0x0c - DMA Channel 0 CB Word 1 (Source Address)"]
    #[inline(always)]
    pub const fn _0_source_ad(&self) -> &_0SourceAd {
        &self._0_source_ad
    }
    #[doc = "0x10 - DMA Channel 0 CB Word 2 (Destination Address)"]
    #[inline(always)]
    pub const fn _0_dest_ad(&self) -> &_0DestAd {
        &self._0_dest_ad
    }
    #[doc = "0x14 - DMA Channel 0 CB Word 3 (Transfer Length)"]
    #[inline(always)]
    pub const fn _0_txfr_len(&self) -> &_0TxfrLen {
        &self._0_txfr_len
    }
    #[doc = "0x18 - DMA Channel 0 CB Word 4 (2D Stride)"]
    #[inline(always)]
    pub const fn _0_stride(&self) -> &_0Stride {
        &self._0_stride
    }
    #[doc = "0x1c - DMA Channel 0 CB Word 5 (Next CB Address)"]
    #[inline(always)]
    pub const fn _0_nextconbk(&self) -> &_0Nextconbk {
        &self._0_nextconbk
    }
    #[doc = "0x20 - DMA Channel 0 Debug"]
    #[inline(always)]
    pub const fn _0_debug(&self) -> &_0Debug {
        &self._0_debug
    }
    #[doc = "0x100 - DMA Channel 1 Control and Status"]
    #[inline(always)]
    pub const fn _1_cs(&self) -> &_1Cs {
        &self._1_cs
    }
    #[doc = "0x104 - DMA Channel 1 Control Block Address"]
    #[inline(always)]
    pub const fn _1_conblk_ad(&self) -> &_1ConblkAd {
        &self._1_conblk_ad
    }
    #[doc = "0x108 - DMA Channel 1 CB Word 0 (Transfer Information)"]
    #[inline(always)]
    pub const fn _1_ti(&self) -> &_1Ti {
        &self._1_ti
    }
    #[doc = "0x10c - DMA Channel 1 CB Word 1 (Source Address)"]
    #[inline(always)]
    pub const fn _1_source_ad(&self) -> &_1SourceAd {
        &self._1_source_ad
    }
    #[doc = "0x110 - DMA Channel 1 CB Word 2 (Destination Address)"]
    #[inline(always)]
    pub const fn _1_dest_ad(&self) -> &_1DestAd {
        &self._1_dest_ad
    }
    #[doc = "0x114 - DMA Channel 1 CB Word 3 (Transfer Length)"]
    #[inline(always)]
    pub const fn _1_txfr_len(&self) -> &_1TxfrLen {
        &self._1_txfr_len
    }
    #[doc = "0x118 - DMA Channel 1 CB Word 4 (2D Stride)"]
    #[inline(always)]
    pub const fn _1_stride(&self) -> &_1Stride {
        &self._1_stride
    }
    #[doc = "0x11c - DMA Channel 1 CB Word 5 (Next CB Address)"]
    #[inline(always)]
    pub const fn _1_nextconbk(&self) -> &_1Nextconbk {
        &self._1_nextconbk
    }
    #[doc = "0x120 - DMA Channel 1 Debug"]
    #[inline(always)]
    pub const fn _1_debug(&self) -> &_1Debug {
        &self._1_debug
    }
    #[doc = "0x200 - DMA Channel 2 Control and Status"]
    #[inline(always)]
    pub const fn _2_cs(&self) -> &_2Cs {
        &self._2_cs
    }
    #[doc = "0x204 - DMA Channel 2 Control Block Address"]
    #[inline(always)]
    pub const fn _2_conblk_ad(&self) -> &_2ConblkAd {
        &self._2_conblk_ad
    }
    #[doc = "0x208 - DMA Channel 2 CB Word 0 (Transfer Information)"]
    #[inline(always)]
    pub const fn _2_ti(&self) -> &_2Ti {
        &self._2_ti
    }
    #[doc = "0x20c - DMA Channel 2 CB Word 1 (Source Address)"]
    #[inline(always)]
    pub const fn _2_source_ad(&self) -> &_2SourceAd {
        &self._2_source_ad
    }
    #[doc = "0x210 - DMA Channel 2 CB Word 2 (Destination Address)"]
    #[inline(always)]
    pub const fn _2_dest_ad(&self) -> &_2DestAd {
        &self._2_dest_ad
    }
    #[doc = "0x214 - DMA Channel 2 CB Word 3 (Transfer Length)"]
    #[inline(always)]
    pub const fn _2_txfr_len(&self) -> &_2TxfrLen {
        &self._2_txfr_len
    }
    #[doc = "0x218 - DMA Channel 2 CB Word 4 (2D Stride)"]
    #[inline(always)]
    pub const fn _2_stride(&self) -> &_2Stride {
        &self._2_stride
    }
    #[doc = "0x21c - DMA Channel 2 CB Word 5 (Next CB Address)"]
    #[inline(always)]
    pub const fn _2_nextconbk(&self) -> &_2Nextconbk {
        &self._2_nextconbk
    }
    #[doc = "0x220 - DMA Channel 2 Debug"]
    #[inline(always)]
    pub const fn _2_debug(&self) -> &_2Debug {
        &self._2_debug
    }
    #[doc = "0x300 - DMA Channel 3 Control and Status"]
    #[inline(always)]
    pub const fn _3_cs(&self) -> &_3Cs {
        &self._3_cs
    }
    #[doc = "0x304 - DMA Channel 3 Control Block Address"]
    #[inline(always)]
    pub const fn _3_conblk_ad(&self) -> &_3ConblkAd {
        &self._3_conblk_ad
    }
    #[doc = "0x308 - DMA Channel 3 CB Word 0 (Transfer Information)"]
    #[inline(always)]
    pub const fn _3_ti(&self) -> &_3Ti {
        &self._3_ti
    }
    #[doc = "0x30c - DMA Channel 3 CB Word 1 (Source Address)"]
    #[inline(always)]
    pub const fn _3_source_ad(&self) -> &_3SourceAd {
        &self._3_source_ad
    }
    #[doc = "0x310 - DMA Channel 3 CB Word 2 (Destination Address)"]
    #[inline(always)]
    pub const fn _3_dest_ad(&self) -> &_3DestAd {
        &self._3_dest_ad
    }
    #[doc = "0x314 - DMA Channel 3 CB Word 3 (Transfer Length)"]
    #[inline(always)]
    pub const fn _3_txfr_len(&self) -> &_3TxfrLen {
        &self._3_txfr_len
    }
    #[doc = "0x318 - DMA Channel 3 CB Word 4 (2D Stride)"]
    #[inline(always)]
    pub const fn _3_stride(&self) -> &_3Stride {
        &self._3_stride
    }
    #[doc = "0x31c - DMA Channel 3 CB Word 5 (Next CB Address)"]
    #[inline(always)]
    pub const fn _3_nextconbk(&self) -> &_3Nextconbk {
        &self._3_nextconbk
    }
    #[doc = "0x320 - DMA Channel 3 Debug"]
    #[inline(always)]
    pub const fn _3_debug(&self) -> &_3Debug {
        &self._3_debug
    }
    #[doc = "0x400 - DMA Channel 4 Control and Status"]
    #[inline(always)]
    pub const fn _4_cs(&self) -> &_4Cs {
        &self._4_cs
    }
    #[doc = "0x404 - DMA Channel 4 Control Block Address"]
    #[inline(always)]
    pub const fn _4_conblk_ad(&self) -> &_4ConblkAd {
        &self._4_conblk_ad
    }
    #[doc = "0x408 - DMA Channel 4 CB Word 0 (Transfer Information)"]
    #[inline(always)]
    pub const fn _4_ti(&self) -> &_4Ti {
        &self._4_ti
    }
    #[doc = "0x40c - DMA Channel 4 CB Word 1 (Source Address)"]
    #[inline(always)]
    pub const fn _4_source_ad(&self) -> &_4SourceAd {
        &self._4_source_ad
    }
    #[doc = "0x410 - DMA Channel 4 CB Word 2 (Destination Address)"]
    #[inline(always)]
    pub const fn _4_dest_ad(&self) -> &_4DestAd {
        &self._4_dest_ad
    }
    #[doc = "0x414 - DMA Channel 4 CB Word 3 (Transfer Length)"]
    #[inline(always)]
    pub const fn _4_txfr_len(&self) -> &_4TxfrLen {
        &self._4_txfr_len
    }
    #[doc = "0x418 - DMA Channel 4 CB Word 4 (2D Stride)"]
    #[inline(always)]
    pub const fn _4_stride(&self) -> &_4Stride {
        &self._4_stride
    }
    #[doc = "0x41c - DMA Channel 4 CB Word 5 (Next CB Address)"]
    #[inline(always)]
    pub const fn _4_nextconbk(&self) -> &_4Nextconbk {
        &self._4_nextconbk
    }
    #[doc = "0x420 - DMA Channel 4 Debug"]
    #[inline(always)]
    pub const fn _4_debug(&self) -> &_4Debug {
        &self._4_debug
    }
    #[doc = "0x500 - DMA Channel 5 Control and Status"]
    #[inline(always)]
    pub const fn _5_cs(&self) -> &_5Cs {
        &self._5_cs
    }
    #[doc = "0x504 - DMA Channel 5 Control Block Address"]
    #[inline(always)]
    pub const fn _5_conblk_ad(&self) -> &_5ConblkAd {
        &self._5_conblk_ad
    }
    #[doc = "0x508 - DMA Channel 5 CB Word 0 (Transfer Information)"]
    #[inline(always)]
    pub const fn _5_ti(&self) -> &_5Ti {
        &self._5_ti
    }
    #[doc = "0x50c - DMA Channel 5 CB Word 1 (Source Address)"]
    #[inline(always)]
    pub const fn _5_source_ad(&self) -> &_5SourceAd {
        &self._5_source_ad
    }
    #[doc = "0x510 - DMA Channel 5 CB Word 2 (Destination Address)"]
    #[inline(always)]
    pub const fn _5_dest_ad(&self) -> &_5DestAd {
        &self._5_dest_ad
    }
    #[doc = "0x514 - DMA Channel 5 CB Word 3 (Transfer Length)"]
    #[inline(always)]
    pub const fn _5_txfr_len(&self) -> &_5TxfrLen {
        &self._5_txfr_len
    }
    #[doc = "0x518 - DMA Channel 5 CB Word 4 (2D Stride)"]
    #[inline(always)]
    pub const fn _5_stride(&self) -> &_5Stride {
        &self._5_stride
    }
    #[doc = "0x51c - DMA Channel 5 CB Word 5 (Next CB Address)"]
    #[inline(always)]
    pub const fn _5_nextconbk(&self) -> &_5Nextconbk {
        &self._5_nextconbk
    }
    #[doc = "0x520 - DMA Channel 5 Debug"]
    #[inline(always)]
    pub const fn _5_debug(&self) -> &_5Debug {
        &self._5_debug
    }
    #[doc = "0x600 - DMA Channel 6 Control and Status"]
    #[inline(always)]
    pub const fn _6_cs(&self) -> &_6Cs {
        &self._6_cs
    }
    #[doc = "0x604 - DMA Channel 6 Control Block Address"]
    #[inline(always)]
    pub const fn _6_conblk_ad(&self) -> &_6ConblkAd {
        &self._6_conblk_ad
    }
    #[doc = "0x608 - DMA Channel 6 CB Word 0 (Transfer Information)"]
    #[inline(always)]
    pub const fn _6_ti(&self) -> &_6Ti {
        &self._6_ti
    }
    #[doc = "0x60c - DMA Channel 6 CB Word 1 (Source Address)"]
    #[inline(always)]
    pub const fn _6_source_ad(&self) -> &_6SourceAd {
        &self._6_source_ad
    }
    #[doc = "0x610 - DMA Channel 6 CB Word 2 (Destination Address)"]
    #[inline(always)]
    pub const fn _6_dest_ad(&self) -> &_6DestAd {
        &self._6_dest_ad
    }
    #[doc = "0x614 - DMA Channel 6 CB Word 3 (Transfer Length)"]
    #[inline(always)]
    pub const fn _6_txfr_len(&self) -> &_6TxfrLen {
        &self._6_txfr_len
    }
    #[doc = "0x618 - DMA Channel 6 CB Word 4 (2D Stride)"]
    #[inline(always)]
    pub const fn _6_stride(&self) -> &_6Stride {
        &self._6_stride
    }
    #[doc = "0x61c - DMA Channel 6 CB Word 5 (Next CB Address)"]
    #[inline(always)]
    pub const fn _6_nextconbk(&self) -> &_6Nextconbk {
        &self._6_nextconbk
    }
    #[doc = "0x620 - DMA Channel 6 Debug"]
    #[inline(always)]
    pub const fn _6_debug(&self) -> &_6Debug {
        &self._6_debug
    }
    #[doc = "0x700 - DMA Channel 7 Control and Status"]
    #[inline(always)]
    pub const fn _7_cs(&self) -> &_7Cs {
        &self._7_cs
    }
    #[doc = "0x704 - DMA Channel 7 Control Block Address"]
    #[inline(always)]
    pub const fn _7_conblk_ad(&self) -> &_7ConblkAd {
        &self._7_conblk_ad
    }
    #[doc = "0x708 - DMA Channel 7 CB Word 0 (Transfer Information)"]
    #[inline(always)]
    pub const fn _7_ti(&self) -> &_7Ti {
        &self._7_ti
    }
    #[doc = "0x70c - DMA Channel 7 CB Word 1 (Source Address)"]
    #[inline(always)]
    pub const fn _7_source_ad(&self) -> &_7SourceAd {
        &self._7_source_ad
    }
    #[doc = "0x710 - DMA Channel 7 CB Word 2 (Destination Address)"]
    #[inline(always)]
    pub const fn _7_dest_ad(&self) -> &_7DestAd {
        &self._7_dest_ad
    }
    #[doc = "0x714 - DMA Channel 7 CB Word 3 (Transfer Length)"]
    #[inline(always)]
    pub const fn _7_txfr_len(&self) -> &_7TxfrLen {
        &self._7_txfr_len
    }
    #[doc = "0x718 - DMA Channel 7 CB Word 4 (2D Stride)"]
    #[inline(always)]
    pub const fn _7_stride(&self) -> &_7Stride {
        &self._7_stride
    }
    #[doc = "0x71c - DMA Channel 7 CB Word 5 (Next CB Address)"]
    #[inline(always)]
    pub const fn _7_nextconbk(&self) -> &_7Nextconbk {
        &self._7_nextconbk
    }
    #[doc = "0x720 - DMA Channel 7 Debug"]
    #[inline(always)]
    pub const fn _7_debug(&self) -> &_7Debug {
        &self._7_debug
    }
    #[doc = "0x800 - DMA Channel 8 Control and Status"]
    #[inline(always)]
    pub const fn _8_cs(&self) -> &_8Cs {
        &self._8_cs
    }
    #[doc = "0x804 - DMA Channel 8 Control Block Address"]
    #[inline(always)]
    pub const fn _8_conblk_ad(&self) -> &_8ConblkAd {
        &self._8_conblk_ad
    }
    #[doc = "0x808 - DMA Channel 8 CB Word 0 (Transfer Information)"]
    #[inline(always)]
    pub const fn _8_ti(&self) -> &_8Ti {
        &self._8_ti
    }
    #[doc = "0x80c - DMA Channel 8 CB Word 1 (Source Address)"]
    #[inline(always)]
    pub const fn _8_source_ad(&self) -> &_8SourceAd {
        &self._8_source_ad
    }
    #[doc = "0x810 - DMA Channel 8 CB Word 2 (Destination Address)"]
    #[inline(always)]
    pub const fn _8_dest_ad(&self) -> &_8DestAd {
        &self._8_dest_ad
    }
    #[doc = "0x814 - DMA Channel 8 CB Word 3 (Transfer Length)"]
    #[inline(always)]
    pub const fn _8_txfr_len(&self) -> &_8TxfrLen {
        &self._8_txfr_len
    }
    #[doc = "0x818 - DMA Channel 8 CB Word 4 (2D Stride)"]
    #[inline(always)]
    pub const fn _8_stride(&self) -> &_8Stride {
        &self._8_stride
    }
    #[doc = "0x81c - DMA Channel 8 CB Word 5 (Next CB Address)"]
    #[inline(always)]
    pub const fn _8_nextconbk(&self) -> &_8Nextconbk {
        &self._8_nextconbk
    }
    #[doc = "0x820 - DMA Channel 8 Debug"]
    #[inline(always)]
    pub const fn _8_debug(&self) -> &_8Debug {
        &self._8_debug
    }
    #[doc = "0x900 - DMA Channel 9 Control and Status"]
    #[inline(always)]
    pub const fn _9_cs(&self) -> &_9Cs {
        &self._9_cs
    }
    #[doc = "0x904 - DMA Channel 9 Control Block Address"]
    #[inline(always)]
    pub const fn _9_conblk_ad(&self) -> &_9ConblkAd {
        &self._9_conblk_ad
    }
    #[doc = "0x908 - DMA Channel 9 CB Word 0 (Transfer Information)"]
    #[inline(always)]
    pub const fn _9_ti(&self) -> &_9Ti {
        &self._9_ti
    }
    #[doc = "0x90c - DMA Channel 9 CB Word 1 (Source Address)"]
    #[inline(always)]
    pub const fn _9_source_ad(&self) -> &_9SourceAd {
        &self._9_source_ad
    }
    #[doc = "0x910 - DMA Channel 9 CB Word 2 (Destination Address)"]
    #[inline(always)]
    pub const fn _9_dest_ad(&self) -> &_9DestAd {
        &self._9_dest_ad
    }
    #[doc = "0x914 - DMA Channel 9 CB Word 3 (Transfer Length)"]
    #[inline(always)]
    pub const fn _9_txfr_len(&self) -> &_9TxfrLen {
        &self._9_txfr_len
    }
    #[doc = "0x918 - DMA Channel 9 CB Word 4 (2D Stride)"]
    #[inline(always)]
    pub const fn _9_stride(&self) -> &_9Stride {
        &self._9_stride
    }
    #[doc = "0x91c - DMA Channel 9 CB Word 5 (Next CB Address)"]
    #[inline(always)]
    pub const fn _9_nextconbk(&self) -> &_9Nextconbk {
        &self._9_nextconbk
    }
    #[doc = "0x920 - DMA Channel 9 Debug"]
    #[inline(always)]
    pub const fn _9_debug(&self) -> &_9Debug {
        &self._9_debug
    }
    #[doc = "0xa00 - DMA Channel 10 Control and Status"]
    #[inline(always)]
    pub const fn _10_cs(&self) -> &_10Cs {
        &self._10_cs
    }
    #[doc = "0xa04 - DMA Channel 10 Control Block Address"]
    #[inline(always)]
    pub const fn _10_conblk_ad(&self) -> &_10ConblkAd {
        &self._10_conblk_ad
    }
    #[doc = "0xa08 - DMA Channel 10 CB Word 0 (Transfer Information)"]
    #[inline(always)]
    pub const fn _10_ti(&self) -> &_10Ti {
        &self._10_ti
    }
    #[doc = "0xa0c - DMA Channel 10 CB Word 1 (Source Address)"]
    #[inline(always)]
    pub const fn _10_source_ad(&self) -> &_10SourceAd {
        &self._10_source_ad
    }
    #[doc = "0xa10 - DMA Channel 10 CB Word 2 (Destination Address)"]
    #[inline(always)]
    pub const fn _10_dest_ad(&self) -> &_10DestAd {
        &self._10_dest_ad
    }
    #[doc = "0xa14 - DMA Channel 10 CB Word 3 (Transfer Length)"]
    #[inline(always)]
    pub const fn _10_txfr_len(&self) -> &_10TxfrLen {
        &self._10_txfr_len
    }
    #[doc = "0xa18 - DMA Channel 10 CB Word 4 (2D Stride)"]
    #[inline(always)]
    pub const fn _10_stride(&self) -> &_10Stride {
        &self._10_stride
    }
    #[doc = "0xa1c - DMA Channel 10 CB Word 5 (Next CB Address)"]
    #[inline(always)]
    pub const fn _10_nextconbk(&self) -> &_10Nextconbk {
        &self._10_nextconbk
    }
    #[doc = "0xa20 - DMA Channel 10 Debug"]
    #[inline(always)]
    pub const fn _10_debug(&self) -> &_10Debug {
        &self._10_debug
    }
    #[doc = "0xb00 - DMA Channel 11 Control and Status"]
    #[inline(always)]
    pub const fn _11_cs(&self) -> &_11Cs {
        &self._11_cs
    }
    #[doc = "0xb04 - DMA Channel 11 Control Block Address"]
    #[inline(always)]
    pub const fn _11_conblk_ad(&self) -> &_11ConblkAd {
        &self._11_conblk_ad
    }
    #[doc = "0xb08 - DMA Channel 11 CB Word 0 (Transfer Information)"]
    #[inline(always)]
    pub const fn _11_ti(&self) -> &_11Ti {
        &self._11_ti
    }
    #[doc = "0xb0c - DMA Channel 11 CB Word 1 (Source Address)"]
    #[inline(always)]
    pub const fn _11_source_ad(&self) -> &_11SourceAd {
        &self._11_source_ad
    }
    #[doc = "0xb10 - DMA Channel 11 CB Word 2 (Destination Address)"]
    #[inline(always)]
    pub const fn _11_dest_ad(&self) -> &_11DestAd {
        &self._11_dest_ad
    }
    #[doc = "0xb14 - DMA Channel 11 CB Word 3 (Transfer Length)"]
    #[inline(always)]
    pub const fn _11_txfr_len(&self) -> &_11TxfrLen {
        &self._11_txfr_len
    }
    #[doc = "0xb18 - DMA Channel 11 CB Word 4 (2D Stride)"]
    #[inline(always)]
    pub const fn _11_stride(&self) -> &_11Stride {
        &self._11_stride
    }
    #[doc = "0xb1c - DMA Channel 11 CB Word 5 (Next CB Address)"]
    #[inline(always)]
    pub const fn _11_nextconbk(&self) -> &_11Nextconbk {
        &self._11_nextconbk
    }
    #[doc = "0xb20 - DMA Channel 11 Debug"]
    #[inline(always)]
    pub const fn _11_debug(&self) -> &_11Debug {
        &self._11_debug
    }
    #[doc = "0xc00 - DMA Channel 12 Control and Status"]
    #[inline(always)]
    pub const fn _12_cs(&self) -> &_12Cs {
        &self._12_cs
    }
    #[doc = "0xc04 - DMA Channel 12 Control Block Address"]
    #[inline(always)]
    pub const fn _12_conblk_ad(&self) -> &_12ConblkAd {
        &self._12_conblk_ad
    }
    #[doc = "0xc08 - DMA Channel 12 CB Word 0 (Transfer Information)"]
    #[inline(always)]
    pub const fn _12_ti(&self) -> &_12Ti {
        &self._12_ti
    }
    #[doc = "0xc0c - DMA Channel 12 CB Word 1 (Source Address)"]
    #[inline(always)]
    pub const fn _12_source_ad(&self) -> &_12SourceAd {
        &self._12_source_ad
    }
    #[doc = "0xc10 - DMA Channel 12 CB Word 2 (Destination Address)"]
    #[inline(always)]
    pub const fn _12_dest_ad(&self) -> &_12DestAd {
        &self._12_dest_ad
    }
    #[doc = "0xc14 - DMA Channel 12 CB Word 3 (Transfer Length)"]
    #[inline(always)]
    pub const fn _12_txfr_len(&self) -> &_12TxfrLen {
        &self._12_txfr_len
    }
    #[doc = "0xc18 - DMA Channel 12 CB Word 4 (2D Stride)"]
    #[inline(always)]
    pub const fn _12_stride(&self) -> &_12Stride {
        &self._12_stride
    }
    #[doc = "0xc1c - DMA Channel 12 CB Word 5 (Next CB Address)"]
    #[inline(always)]
    pub const fn _12_nextconbk(&self) -> &_12Nextconbk {
        &self._12_nextconbk
    }
    #[doc = "0xc20 - DMA Channel 12 Debug"]
    #[inline(always)]
    pub const fn _12_debug(&self) -> &_12Debug {
        &self._12_debug
    }
    #[doc = "0xd00 - DMA Channel 13 Control and Status"]
    #[inline(always)]
    pub const fn _13_cs(&self) -> &_13Cs {
        &self._13_cs
    }
    #[doc = "0xd04 - DMA Channel 13 Control Block Address"]
    #[inline(always)]
    pub const fn _13_conblk_ad(&self) -> &_13ConblkAd {
        &self._13_conblk_ad
    }
    #[doc = "0xd08 - DMA Channel 13 CB Word 0 (Transfer Information)"]
    #[inline(always)]
    pub const fn _13_ti(&self) -> &_13Ti {
        &self._13_ti
    }
    #[doc = "0xd0c - DMA Channel 13 CB Word 1 (Source Address)"]
    #[inline(always)]
    pub const fn _13_source_ad(&self) -> &_13SourceAd {
        &self._13_source_ad
    }
    #[doc = "0xd10 - DMA Channel 13 CB Word 2 (Destination Address)"]
    #[inline(always)]
    pub const fn _13_dest_ad(&self) -> &_13DestAd {
        &self._13_dest_ad
    }
    #[doc = "0xd14 - DMA Channel 13 CB Word 3 (Transfer Length)"]
    #[inline(always)]
    pub const fn _13_txfr_len(&self) -> &_13TxfrLen {
        &self._13_txfr_len
    }
    #[doc = "0xd18 - DMA Channel 13 CB Word 4 (2D Stride)"]
    #[inline(always)]
    pub const fn _13_stride(&self) -> &_13Stride {
        &self._13_stride
    }
    #[doc = "0xd1c - DMA Channel 13 CB Word 5 (Next CB Address)"]
    #[inline(always)]
    pub const fn _13_nextconbk(&self) -> &_13Nextconbk {
        &self._13_nextconbk
    }
    #[doc = "0xd20 - DMA Channel 13 Debug"]
    #[inline(always)]
    pub const fn _13_debug(&self) -> &_13Debug {
        &self._13_debug
    }
    #[doc = "0xe00 - DMA Channel 14 Control and Status"]
    #[inline(always)]
    pub const fn _14_cs(&self) -> &_14Cs {
        &self._14_cs
    }
    #[doc = "0xe04 - DMA Channel 14 Control Block Address"]
    #[inline(always)]
    pub const fn _14_conblk_ad(&self) -> &_14ConblkAd {
        &self._14_conblk_ad
    }
    #[doc = "0xe08 - DMA Channel 14 CB Word 0 (Transfer Information)"]
    #[inline(always)]
    pub const fn _14_ti(&self) -> &_14Ti {
        &self._14_ti
    }
    #[doc = "0xe0c - DMA Channel 14 CB Word 1 (Source Address)"]
    #[inline(always)]
    pub const fn _14_source_ad(&self) -> &_14SourceAd {
        &self._14_source_ad
    }
    #[doc = "0xe10 - DMA Channel 14 CB Word 2 (Destination Address)"]
    #[inline(always)]
    pub const fn _14_dest_ad(&self) -> &_14DestAd {
        &self._14_dest_ad
    }
    #[doc = "0xe14 - DMA Channel 14 CB Word 3 (Transfer Length)"]
    #[inline(always)]
    pub const fn _14_txfr_len(&self) -> &_14TxfrLen {
        &self._14_txfr_len
    }
    #[doc = "0xe18 - DMA Channel 14 CB Word 4 (2D Stride)"]
    #[inline(always)]
    pub const fn _14_stride(&self) -> &_14Stride {
        &self._14_stride
    }
    #[doc = "0xe1c - DMA Channel 14 CB Word 5 (Next CB Address)"]
    #[inline(always)]
    pub const fn _14_nextconbk(&self) -> &_14Nextconbk {
        &self._14_nextconbk
    }
    #[doc = "0xe20 - DMA Channel 14 Debug"]
    #[inline(always)]
    pub const fn _14_debug(&self) -> &_14Debug {
        &self._14_debug
    }
    #[doc = "0xfe0 - Interrupt status of each DMA channel"]
    #[inline(always)]
    pub const fn int_status(&self) -> &IntStatus {
        &self.int_status
    }
    #[doc = "0xff0 - Global enable bits for each DMA channel"]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
}
#[doc = "0_CS (rw) register accessor: DMA Channel 0 Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`_0_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_0_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_cs`]
module"]
#[doc(alias = "0_CS")]
pub type _0Cs = crate::Reg<_0_cs::_0CsSpec>;
#[doc = "DMA Channel 0 Control and Status"]
pub mod _0_cs;
#[doc = "0_CONBLK_AD (rw) register accessor: DMA Channel 0 Control Block Address\n\nYou can [`read`](crate::Reg::read) this register and get [`_0_conblk_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_0_conblk_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_conblk_ad`]
module"]
#[doc(alias = "0_CONBLK_AD")]
pub type _0ConblkAd = crate::Reg<_0_conblk_ad::_0ConblkAdSpec>;
#[doc = "DMA Channel 0 Control Block Address"]
pub mod _0_conblk_ad;
#[doc = "0_TI (rw) register accessor: DMA Channel 0 CB Word 0 (Transfer Information)\n\nYou can [`read`](crate::Reg::read) this register and get [`_0_ti::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_0_ti::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_ti`]
module"]
#[doc(alias = "0_TI")]
pub type _0Ti = crate::Reg<_0_ti::_0TiSpec>;
#[doc = "DMA Channel 0 CB Word 0 (Transfer Information)"]
pub mod _0_ti;
#[doc = "0_SOURCE_AD (rw) register accessor: DMA Channel 0 CB Word 1 (Source Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_0_source_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_0_source_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_source_ad`]
module"]
#[doc(alias = "0_SOURCE_AD")]
pub type _0SourceAd = crate::Reg<_0_source_ad::_0SourceAdSpec>;
#[doc = "DMA Channel 0 CB Word 1 (Source Address)"]
pub mod _0_source_ad;
#[doc = "0_DEST_AD (rw) register accessor: DMA Channel 0 CB Word 2 (Destination Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_0_dest_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_0_dest_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_dest_ad`]
module"]
#[doc(alias = "0_DEST_AD")]
pub type _0DestAd = crate::Reg<_0_dest_ad::_0DestAdSpec>;
#[doc = "DMA Channel 0 CB Word 2 (Destination Address)"]
pub mod _0_dest_ad;
#[doc = "0_TXFR_LEN (rw) register accessor: DMA Channel 0 CB Word 3 (Transfer Length)\n\nYou can [`read`](crate::Reg::read) this register and get [`_0_txfr_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_0_txfr_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_txfr_len`]
module"]
#[doc(alias = "0_TXFR_LEN")]
pub type _0TxfrLen = crate::Reg<_0_txfr_len::_0TxfrLenSpec>;
#[doc = "DMA Channel 0 CB Word 3 (Transfer Length)"]
pub mod _0_txfr_len;
#[doc = "0_STRIDE (rw) register accessor: DMA Channel 0 CB Word 4 (2D Stride)\n\nYou can [`read`](crate::Reg::read) this register and get [`_0_stride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_0_stride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_stride`]
module"]
#[doc(alias = "0_STRIDE")]
pub type _0Stride = crate::Reg<_0_stride::_0StrideSpec>;
#[doc = "DMA Channel 0 CB Word 4 (2D Stride)"]
pub mod _0_stride;
#[doc = "0_NEXTCONBK (rw) register accessor: DMA Channel 0 CB Word 5 (Next CB Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_0_nextconbk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_0_nextconbk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_nextconbk`]
module"]
#[doc(alias = "0_NEXTCONBK")]
pub type _0Nextconbk = crate::Reg<_0_nextconbk::_0NextconbkSpec>;
#[doc = "DMA Channel 0 CB Word 5 (Next CB Address)"]
pub mod _0_nextconbk;
#[doc = "0_DEBUG (rw) register accessor: DMA Channel 0 Debug\n\nYou can [`read`](crate::Reg::read) this register and get [`_0_debug::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_0_debug::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_0_debug`]
module"]
#[doc(alias = "0_DEBUG")]
pub type _0Debug = crate::Reg<_0_debug::_0DebugSpec>;
#[doc = "DMA Channel 0 Debug"]
pub mod _0_debug;
#[doc = "1_CS (rw) register accessor: DMA Channel 1 Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`_1_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_1_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_cs`]
module"]
#[doc(alias = "1_CS")]
pub type _1Cs = crate::Reg<_1_cs::_1CsSpec>;
#[doc = "DMA Channel 1 Control and Status"]
pub mod _1_cs;
#[doc = "1_CONBLK_AD (rw) register accessor: DMA Channel 1 Control Block Address\n\nYou can [`read`](crate::Reg::read) this register and get [`_1_conblk_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_1_conblk_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_conblk_ad`]
module"]
#[doc(alias = "1_CONBLK_AD")]
pub type _1ConblkAd = crate::Reg<_1_conblk_ad::_1ConblkAdSpec>;
#[doc = "DMA Channel 1 Control Block Address"]
pub mod _1_conblk_ad;
#[doc = "1_TI (rw) register accessor: DMA Channel 1 CB Word 0 (Transfer Information)\n\nYou can [`read`](crate::Reg::read) this register and get [`_1_ti::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_1_ti::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_ti`]
module"]
#[doc(alias = "1_TI")]
pub type _1Ti = crate::Reg<_1_ti::_1TiSpec>;
#[doc = "DMA Channel 1 CB Word 0 (Transfer Information)"]
pub mod _1_ti;
#[doc = "1_SOURCE_AD (rw) register accessor: DMA Channel 1 CB Word 1 (Source Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_1_source_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_1_source_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_source_ad`]
module"]
#[doc(alias = "1_SOURCE_AD")]
pub type _1SourceAd = crate::Reg<_1_source_ad::_1SourceAdSpec>;
#[doc = "DMA Channel 1 CB Word 1 (Source Address)"]
pub mod _1_source_ad;
#[doc = "1_DEST_AD (rw) register accessor: DMA Channel 1 CB Word 2 (Destination Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_1_dest_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_1_dest_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_dest_ad`]
module"]
#[doc(alias = "1_DEST_AD")]
pub type _1DestAd = crate::Reg<_1_dest_ad::_1DestAdSpec>;
#[doc = "DMA Channel 1 CB Word 2 (Destination Address)"]
pub mod _1_dest_ad;
#[doc = "1_TXFR_LEN (rw) register accessor: DMA Channel 1 CB Word 3 (Transfer Length)\n\nYou can [`read`](crate::Reg::read) this register and get [`_1_txfr_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_1_txfr_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_txfr_len`]
module"]
#[doc(alias = "1_TXFR_LEN")]
pub type _1TxfrLen = crate::Reg<_1_txfr_len::_1TxfrLenSpec>;
#[doc = "DMA Channel 1 CB Word 3 (Transfer Length)"]
pub mod _1_txfr_len;
#[doc = "1_STRIDE (rw) register accessor: DMA Channel 1 CB Word 4 (2D Stride)\n\nYou can [`read`](crate::Reg::read) this register and get [`_1_stride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_1_stride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_stride`]
module"]
#[doc(alias = "1_STRIDE")]
pub type _1Stride = crate::Reg<_1_stride::_1StrideSpec>;
#[doc = "DMA Channel 1 CB Word 4 (2D Stride)"]
pub mod _1_stride;
#[doc = "1_NEXTCONBK (rw) register accessor: DMA Channel 1 CB Word 5 (Next CB Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_1_nextconbk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_1_nextconbk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_nextconbk`]
module"]
#[doc(alias = "1_NEXTCONBK")]
pub type _1Nextconbk = crate::Reg<_1_nextconbk::_1NextconbkSpec>;
#[doc = "DMA Channel 1 CB Word 5 (Next CB Address)"]
pub mod _1_nextconbk;
#[doc = "1_DEBUG (rw) register accessor: DMA Channel 1 Debug\n\nYou can [`read`](crate::Reg::read) this register and get [`_1_debug::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_1_debug::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1_debug`]
module"]
#[doc(alias = "1_DEBUG")]
pub type _1Debug = crate::Reg<_1_debug::_1DebugSpec>;
#[doc = "DMA Channel 1 Debug"]
pub mod _1_debug;
#[doc = "2_CS (rw) register accessor: DMA Channel 2 Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`_2_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_2_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2_cs`]
module"]
#[doc(alias = "2_CS")]
pub type _2Cs = crate::Reg<_2_cs::_2CsSpec>;
#[doc = "DMA Channel 2 Control and Status"]
pub mod _2_cs;
#[doc = "2_CONBLK_AD (rw) register accessor: DMA Channel 2 Control Block Address\n\nYou can [`read`](crate::Reg::read) this register and get [`_2_conblk_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_2_conblk_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2_conblk_ad`]
module"]
#[doc(alias = "2_CONBLK_AD")]
pub type _2ConblkAd = crate::Reg<_2_conblk_ad::_2ConblkAdSpec>;
#[doc = "DMA Channel 2 Control Block Address"]
pub mod _2_conblk_ad;
#[doc = "2_TI (rw) register accessor: DMA Channel 2 CB Word 0 (Transfer Information)\n\nYou can [`read`](crate::Reg::read) this register and get [`_2_ti::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_2_ti::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2_ti`]
module"]
#[doc(alias = "2_TI")]
pub type _2Ti = crate::Reg<_2_ti::_2TiSpec>;
#[doc = "DMA Channel 2 CB Word 0 (Transfer Information)"]
pub mod _2_ti;
#[doc = "2_SOURCE_AD (rw) register accessor: DMA Channel 2 CB Word 1 (Source Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_2_source_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_2_source_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2_source_ad`]
module"]
#[doc(alias = "2_SOURCE_AD")]
pub type _2SourceAd = crate::Reg<_2_source_ad::_2SourceAdSpec>;
#[doc = "DMA Channel 2 CB Word 1 (Source Address)"]
pub mod _2_source_ad;
#[doc = "2_DEST_AD (rw) register accessor: DMA Channel 2 CB Word 2 (Destination Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_2_dest_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_2_dest_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2_dest_ad`]
module"]
#[doc(alias = "2_DEST_AD")]
pub type _2DestAd = crate::Reg<_2_dest_ad::_2DestAdSpec>;
#[doc = "DMA Channel 2 CB Word 2 (Destination Address)"]
pub mod _2_dest_ad;
#[doc = "2_TXFR_LEN (rw) register accessor: DMA Channel 2 CB Word 3 (Transfer Length)\n\nYou can [`read`](crate::Reg::read) this register and get [`_2_txfr_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_2_txfr_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2_txfr_len`]
module"]
#[doc(alias = "2_TXFR_LEN")]
pub type _2TxfrLen = crate::Reg<_2_txfr_len::_2TxfrLenSpec>;
#[doc = "DMA Channel 2 CB Word 3 (Transfer Length)"]
pub mod _2_txfr_len;
#[doc = "2_STRIDE (rw) register accessor: DMA Channel 2 CB Word 4 (2D Stride)\n\nYou can [`read`](crate::Reg::read) this register and get [`_2_stride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_2_stride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2_stride`]
module"]
#[doc(alias = "2_STRIDE")]
pub type _2Stride = crate::Reg<_2_stride::_2StrideSpec>;
#[doc = "DMA Channel 2 CB Word 4 (2D Stride)"]
pub mod _2_stride;
#[doc = "2_NEXTCONBK (rw) register accessor: DMA Channel 2 CB Word 5 (Next CB Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_2_nextconbk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_2_nextconbk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2_nextconbk`]
module"]
#[doc(alias = "2_NEXTCONBK")]
pub type _2Nextconbk = crate::Reg<_2_nextconbk::_2NextconbkSpec>;
#[doc = "DMA Channel 2 CB Word 5 (Next CB Address)"]
pub mod _2_nextconbk;
#[doc = "2_DEBUG (rw) register accessor: DMA Channel 2 Debug\n\nYou can [`read`](crate::Reg::read) this register and get [`_2_debug::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_2_debug::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2_debug`]
module"]
#[doc(alias = "2_DEBUG")]
pub type _2Debug = crate::Reg<_2_debug::_2DebugSpec>;
#[doc = "DMA Channel 2 Debug"]
pub mod _2_debug;
#[doc = "3_CS (rw) register accessor: DMA Channel 3 Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`_3_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_cs`]
module"]
#[doc(alias = "3_CS")]
pub type _3Cs = crate::Reg<_3_cs::_3CsSpec>;
#[doc = "DMA Channel 3 Control and Status"]
pub mod _3_cs;
#[doc = "3_CONBLK_AD (rw) register accessor: DMA Channel 3 Control Block Address\n\nYou can [`read`](crate::Reg::read) this register and get [`_3_conblk_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3_conblk_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_conblk_ad`]
module"]
#[doc(alias = "3_CONBLK_AD")]
pub type _3ConblkAd = crate::Reg<_3_conblk_ad::_3ConblkAdSpec>;
#[doc = "DMA Channel 3 Control Block Address"]
pub mod _3_conblk_ad;
#[doc = "3_TI (rw) register accessor: DMA Channel 3 CB Word 0 (Transfer Information)\n\nYou can [`read`](crate::Reg::read) this register and get [`_3_ti::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3_ti::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_ti`]
module"]
#[doc(alias = "3_TI")]
pub type _3Ti = crate::Reg<_3_ti::_3TiSpec>;
#[doc = "DMA Channel 3 CB Word 0 (Transfer Information)"]
pub mod _3_ti;
#[doc = "3_SOURCE_AD (rw) register accessor: DMA Channel 3 CB Word 1 (Source Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_3_source_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3_source_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_source_ad`]
module"]
#[doc(alias = "3_SOURCE_AD")]
pub type _3SourceAd = crate::Reg<_3_source_ad::_3SourceAdSpec>;
#[doc = "DMA Channel 3 CB Word 1 (Source Address)"]
pub mod _3_source_ad;
#[doc = "3_DEST_AD (rw) register accessor: DMA Channel 3 CB Word 2 (Destination Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_3_dest_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3_dest_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_dest_ad`]
module"]
#[doc(alias = "3_DEST_AD")]
pub type _3DestAd = crate::Reg<_3_dest_ad::_3DestAdSpec>;
#[doc = "DMA Channel 3 CB Word 2 (Destination Address)"]
pub mod _3_dest_ad;
#[doc = "3_TXFR_LEN (rw) register accessor: DMA Channel 3 CB Word 3 (Transfer Length)\n\nYou can [`read`](crate::Reg::read) this register and get [`_3_txfr_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3_txfr_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_txfr_len`]
module"]
#[doc(alias = "3_TXFR_LEN")]
pub type _3TxfrLen = crate::Reg<_3_txfr_len::_3TxfrLenSpec>;
#[doc = "DMA Channel 3 CB Word 3 (Transfer Length)"]
pub mod _3_txfr_len;
#[doc = "3_STRIDE (rw) register accessor: DMA Channel 3 CB Word 4 (2D Stride)\n\nYou can [`read`](crate::Reg::read) this register and get [`_3_stride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3_stride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_stride`]
module"]
#[doc(alias = "3_STRIDE")]
pub type _3Stride = crate::Reg<_3_stride::_3StrideSpec>;
#[doc = "DMA Channel 3 CB Word 4 (2D Stride)"]
pub mod _3_stride;
#[doc = "3_NEXTCONBK (rw) register accessor: DMA Channel 3 CB Word 5 (Next CB Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_3_nextconbk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3_nextconbk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_nextconbk`]
module"]
#[doc(alias = "3_NEXTCONBK")]
pub type _3Nextconbk = crate::Reg<_3_nextconbk::_3NextconbkSpec>;
#[doc = "DMA Channel 3 CB Word 5 (Next CB Address)"]
pub mod _3_nextconbk;
#[doc = "3_DEBUG (rw) register accessor: DMA Channel 3 Debug\n\nYou can [`read`](crate::Reg::read) this register and get [`_3_debug::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3_debug::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_debug`]
module"]
#[doc(alias = "3_DEBUG")]
pub type _3Debug = crate::Reg<_3_debug::_3DebugSpec>;
#[doc = "DMA Channel 3 Debug"]
pub mod _3_debug;
#[doc = "4_CS (rw) register accessor: DMA Channel 4 Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`_4_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_4_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_4_cs`]
module"]
#[doc(alias = "4_CS")]
pub type _4Cs = crate::Reg<_4_cs::_4CsSpec>;
#[doc = "DMA Channel 4 Control and Status"]
pub mod _4_cs;
#[doc = "4_CONBLK_AD (rw) register accessor: DMA Channel 4 Control Block Address\n\nYou can [`read`](crate::Reg::read) this register and get [`_4_conblk_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_4_conblk_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_4_conblk_ad`]
module"]
#[doc(alias = "4_CONBLK_AD")]
pub type _4ConblkAd = crate::Reg<_4_conblk_ad::_4ConblkAdSpec>;
#[doc = "DMA Channel 4 Control Block Address"]
pub mod _4_conblk_ad;
#[doc = "4_TI (rw) register accessor: DMA Channel 4 CB Word 0 (Transfer Information)\n\nYou can [`read`](crate::Reg::read) this register and get [`_4_ti::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_4_ti::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_4_ti`]
module"]
#[doc(alias = "4_TI")]
pub type _4Ti = crate::Reg<_4_ti::_4TiSpec>;
#[doc = "DMA Channel 4 CB Word 0 (Transfer Information)"]
pub mod _4_ti;
#[doc = "4_SOURCE_AD (rw) register accessor: DMA Channel 4 CB Word 1 (Source Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_4_source_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_4_source_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_4_source_ad`]
module"]
#[doc(alias = "4_SOURCE_AD")]
pub type _4SourceAd = crate::Reg<_4_source_ad::_4SourceAdSpec>;
#[doc = "DMA Channel 4 CB Word 1 (Source Address)"]
pub mod _4_source_ad;
#[doc = "4_DEST_AD (rw) register accessor: DMA Channel 4 CB Word 2 (Destination Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_4_dest_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_4_dest_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_4_dest_ad`]
module"]
#[doc(alias = "4_DEST_AD")]
pub type _4DestAd = crate::Reg<_4_dest_ad::_4DestAdSpec>;
#[doc = "DMA Channel 4 CB Word 2 (Destination Address)"]
pub mod _4_dest_ad;
#[doc = "4_TXFR_LEN (rw) register accessor: DMA Channel 4 CB Word 3 (Transfer Length)\n\nYou can [`read`](crate::Reg::read) this register and get [`_4_txfr_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_4_txfr_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_4_txfr_len`]
module"]
#[doc(alias = "4_TXFR_LEN")]
pub type _4TxfrLen = crate::Reg<_4_txfr_len::_4TxfrLenSpec>;
#[doc = "DMA Channel 4 CB Word 3 (Transfer Length)"]
pub mod _4_txfr_len;
#[doc = "4_STRIDE (rw) register accessor: DMA Channel 4 CB Word 4 (2D Stride)\n\nYou can [`read`](crate::Reg::read) this register and get [`_4_stride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_4_stride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_4_stride`]
module"]
#[doc(alias = "4_STRIDE")]
pub type _4Stride = crate::Reg<_4_stride::_4StrideSpec>;
#[doc = "DMA Channel 4 CB Word 4 (2D Stride)"]
pub mod _4_stride;
#[doc = "4_NEXTCONBK (rw) register accessor: DMA Channel 4 CB Word 5 (Next CB Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_4_nextconbk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_4_nextconbk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_4_nextconbk`]
module"]
#[doc(alias = "4_NEXTCONBK")]
pub type _4Nextconbk = crate::Reg<_4_nextconbk::_4NextconbkSpec>;
#[doc = "DMA Channel 4 CB Word 5 (Next CB Address)"]
pub mod _4_nextconbk;
#[doc = "4_DEBUG (rw) register accessor: DMA Channel 4 Debug\n\nYou can [`read`](crate::Reg::read) this register and get [`_4_debug::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_4_debug::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_4_debug`]
module"]
#[doc(alias = "4_DEBUG")]
pub type _4Debug = crate::Reg<_4_debug::_4DebugSpec>;
#[doc = "DMA Channel 4 Debug"]
pub mod _4_debug;
#[doc = "5_CS (rw) register accessor: DMA Channel 5 Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`_5_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_5_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_5_cs`]
module"]
#[doc(alias = "5_CS")]
pub type _5Cs = crate::Reg<_5_cs::_5CsSpec>;
#[doc = "DMA Channel 5 Control and Status"]
pub mod _5_cs;
#[doc = "5_CONBLK_AD (rw) register accessor: DMA Channel 5 Control Block Address\n\nYou can [`read`](crate::Reg::read) this register and get [`_5_conblk_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_5_conblk_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_5_conblk_ad`]
module"]
#[doc(alias = "5_CONBLK_AD")]
pub type _5ConblkAd = crate::Reg<_5_conblk_ad::_5ConblkAdSpec>;
#[doc = "DMA Channel 5 Control Block Address"]
pub mod _5_conblk_ad;
#[doc = "5_TI (rw) register accessor: DMA Channel 5 CB Word 0 (Transfer Information)\n\nYou can [`read`](crate::Reg::read) this register and get [`_5_ti::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_5_ti::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_5_ti`]
module"]
#[doc(alias = "5_TI")]
pub type _5Ti = crate::Reg<_5_ti::_5TiSpec>;
#[doc = "DMA Channel 5 CB Word 0 (Transfer Information)"]
pub mod _5_ti;
#[doc = "5_SOURCE_AD (rw) register accessor: DMA Channel 5 CB Word 1 (Source Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_5_source_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_5_source_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_5_source_ad`]
module"]
#[doc(alias = "5_SOURCE_AD")]
pub type _5SourceAd = crate::Reg<_5_source_ad::_5SourceAdSpec>;
#[doc = "DMA Channel 5 CB Word 1 (Source Address)"]
pub mod _5_source_ad;
#[doc = "5_DEST_AD (rw) register accessor: DMA Channel 5 CB Word 2 (Destination Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_5_dest_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_5_dest_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_5_dest_ad`]
module"]
#[doc(alias = "5_DEST_AD")]
pub type _5DestAd = crate::Reg<_5_dest_ad::_5DestAdSpec>;
#[doc = "DMA Channel 5 CB Word 2 (Destination Address)"]
pub mod _5_dest_ad;
#[doc = "5_TXFR_LEN (rw) register accessor: DMA Channel 5 CB Word 3 (Transfer Length)\n\nYou can [`read`](crate::Reg::read) this register and get [`_5_txfr_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_5_txfr_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_5_txfr_len`]
module"]
#[doc(alias = "5_TXFR_LEN")]
pub type _5TxfrLen = crate::Reg<_5_txfr_len::_5TxfrLenSpec>;
#[doc = "DMA Channel 5 CB Word 3 (Transfer Length)"]
pub mod _5_txfr_len;
#[doc = "5_STRIDE (rw) register accessor: DMA Channel 5 CB Word 4 (2D Stride)\n\nYou can [`read`](crate::Reg::read) this register and get [`_5_stride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_5_stride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_5_stride`]
module"]
#[doc(alias = "5_STRIDE")]
pub type _5Stride = crate::Reg<_5_stride::_5StrideSpec>;
#[doc = "DMA Channel 5 CB Word 4 (2D Stride)"]
pub mod _5_stride;
#[doc = "5_NEXTCONBK (rw) register accessor: DMA Channel 5 CB Word 5 (Next CB Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_5_nextconbk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_5_nextconbk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_5_nextconbk`]
module"]
#[doc(alias = "5_NEXTCONBK")]
pub type _5Nextconbk = crate::Reg<_5_nextconbk::_5NextconbkSpec>;
#[doc = "DMA Channel 5 CB Word 5 (Next CB Address)"]
pub mod _5_nextconbk;
#[doc = "5_DEBUG (rw) register accessor: DMA Channel 5 Debug\n\nYou can [`read`](crate::Reg::read) this register and get [`_5_debug::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_5_debug::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_5_debug`]
module"]
#[doc(alias = "5_DEBUG")]
pub type _5Debug = crate::Reg<_5_debug::_5DebugSpec>;
#[doc = "DMA Channel 5 Debug"]
pub mod _5_debug;
#[doc = "6_CS (rw) register accessor: DMA Channel 6 Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`_6_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_6_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_6_cs`]
module"]
#[doc(alias = "6_CS")]
pub type _6Cs = crate::Reg<_6_cs::_6CsSpec>;
#[doc = "DMA Channel 6 Control and Status"]
pub mod _6_cs;
#[doc = "6_CONBLK_AD (rw) register accessor: DMA Channel 6 Control Block Address\n\nYou can [`read`](crate::Reg::read) this register and get [`_6_conblk_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_6_conblk_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_6_conblk_ad`]
module"]
#[doc(alias = "6_CONBLK_AD")]
pub type _6ConblkAd = crate::Reg<_6_conblk_ad::_6ConblkAdSpec>;
#[doc = "DMA Channel 6 Control Block Address"]
pub mod _6_conblk_ad;
#[doc = "6_TI (rw) register accessor: DMA Channel 6 CB Word 0 (Transfer Information)\n\nYou can [`read`](crate::Reg::read) this register and get [`_6_ti::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_6_ti::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_6_ti`]
module"]
#[doc(alias = "6_TI")]
pub type _6Ti = crate::Reg<_6_ti::_6TiSpec>;
#[doc = "DMA Channel 6 CB Word 0 (Transfer Information)"]
pub mod _6_ti;
#[doc = "6_SOURCE_AD (rw) register accessor: DMA Channel 6 CB Word 1 (Source Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_6_source_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_6_source_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_6_source_ad`]
module"]
#[doc(alias = "6_SOURCE_AD")]
pub type _6SourceAd = crate::Reg<_6_source_ad::_6SourceAdSpec>;
#[doc = "DMA Channel 6 CB Word 1 (Source Address)"]
pub mod _6_source_ad;
#[doc = "6_DEST_AD (rw) register accessor: DMA Channel 6 CB Word 2 (Destination Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_6_dest_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_6_dest_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_6_dest_ad`]
module"]
#[doc(alias = "6_DEST_AD")]
pub type _6DestAd = crate::Reg<_6_dest_ad::_6DestAdSpec>;
#[doc = "DMA Channel 6 CB Word 2 (Destination Address)"]
pub mod _6_dest_ad;
#[doc = "6_TXFR_LEN (rw) register accessor: DMA Channel 6 CB Word 3 (Transfer Length)\n\nYou can [`read`](crate::Reg::read) this register and get [`_6_txfr_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_6_txfr_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_6_txfr_len`]
module"]
#[doc(alias = "6_TXFR_LEN")]
pub type _6TxfrLen = crate::Reg<_6_txfr_len::_6TxfrLenSpec>;
#[doc = "DMA Channel 6 CB Word 3 (Transfer Length)"]
pub mod _6_txfr_len;
#[doc = "6_STRIDE (rw) register accessor: DMA Channel 6 CB Word 4 (2D Stride)\n\nYou can [`read`](crate::Reg::read) this register and get [`_6_stride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_6_stride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_6_stride`]
module"]
#[doc(alias = "6_STRIDE")]
pub type _6Stride = crate::Reg<_6_stride::_6StrideSpec>;
#[doc = "DMA Channel 6 CB Word 4 (2D Stride)"]
pub mod _6_stride;
#[doc = "6_NEXTCONBK (rw) register accessor: DMA Channel 6 CB Word 5 (Next CB Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_6_nextconbk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_6_nextconbk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_6_nextconbk`]
module"]
#[doc(alias = "6_NEXTCONBK")]
pub type _6Nextconbk = crate::Reg<_6_nextconbk::_6NextconbkSpec>;
#[doc = "DMA Channel 6 CB Word 5 (Next CB Address)"]
pub mod _6_nextconbk;
#[doc = "6_DEBUG (rw) register accessor: DMA Channel 6 Debug\n\nYou can [`read`](crate::Reg::read) this register and get [`_6_debug::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_6_debug::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_6_debug`]
module"]
#[doc(alias = "6_DEBUG")]
pub type _6Debug = crate::Reg<_6_debug::_6DebugSpec>;
#[doc = "DMA Channel 6 Debug"]
pub mod _6_debug;
#[doc = "7_CS (rw) register accessor: DMA Channel 7 Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`_7_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_7_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_7_cs`]
module"]
#[doc(alias = "7_CS")]
pub type _7Cs = crate::Reg<_7_cs::_7CsSpec>;
#[doc = "DMA Channel 7 Control and Status"]
pub mod _7_cs;
#[doc = "7_CONBLK_AD (rw) register accessor: DMA Channel 7 Control Block Address\n\nYou can [`read`](crate::Reg::read) this register and get [`_7_conblk_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_7_conblk_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_7_conblk_ad`]
module"]
#[doc(alias = "7_CONBLK_AD")]
pub type _7ConblkAd = crate::Reg<_7_conblk_ad::_7ConblkAdSpec>;
#[doc = "DMA Channel 7 Control Block Address"]
pub mod _7_conblk_ad;
#[doc = "7_TI (rw) register accessor: DMA Channel 7 CB Word 0 (Transfer Information)\n\nYou can [`read`](crate::Reg::read) this register and get [`_7_ti::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_7_ti::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_7_ti`]
module"]
#[doc(alias = "7_TI")]
pub type _7Ti = crate::Reg<_7_ti::_7TiSpec>;
#[doc = "DMA Channel 7 CB Word 0 (Transfer Information)"]
pub mod _7_ti;
#[doc = "7_SOURCE_AD (rw) register accessor: DMA Channel 7 CB Word 1 (Source Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_7_source_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_7_source_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_7_source_ad`]
module"]
#[doc(alias = "7_SOURCE_AD")]
pub type _7SourceAd = crate::Reg<_7_source_ad::_7SourceAdSpec>;
#[doc = "DMA Channel 7 CB Word 1 (Source Address)"]
pub mod _7_source_ad;
#[doc = "7_DEST_AD (rw) register accessor: DMA Channel 7 CB Word 2 (Destination Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_7_dest_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_7_dest_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_7_dest_ad`]
module"]
#[doc(alias = "7_DEST_AD")]
pub type _7DestAd = crate::Reg<_7_dest_ad::_7DestAdSpec>;
#[doc = "DMA Channel 7 CB Word 2 (Destination Address)"]
pub mod _7_dest_ad;
#[doc = "7_TXFR_LEN (rw) register accessor: DMA Channel 7 CB Word 3 (Transfer Length)\n\nYou can [`read`](crate::Reg::read) this register and get [`_7_txfr_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_7_txfr_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_7_txfr_len`]
module"]
#[doc(alias = "7_TXFR_LEN")]
pub type _7TxfrLen = crate::Reg<_7_txfr_len::_7TxfrLenSpec>;
#[doc = "DMA Channel 7 CB Word 3 (Transfer Length)"]
pub mod _7_txfr_len;
#[doc = "7_STRIDE (rw) register accessor: DMA Channel 7 CB Word 4 (2D Stride)\n\nYou can [`read`](crate::Reg::read) this register and get [`_7_stride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_7_stride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_7_stride`]
module"]
#[doc(alias = "7_STRIDE")]
pub type _7Stride = crate::Reg<_7_stride::_7StrideSpec>;
#[doc = "DMA Channel 7 CB Word 4 (2D Stride)"]
pub mod _7_stride;
#[doc = "7_NEXTCONBK (rw) register accessor: DMA Channel 7 CB Word 5 (Next CB Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_7_nextconbk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_7_nextconbk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_7_nextconbk`]
module"]
#[doc(alias = "7_NEXTCONBK")]
pub type _7Nextconbk = crate::Reg<_7_nextconbk::_7NextconbkSpec>;
#[doc = "DMA Channel 7 CB Word 5 (Next CB Address)"]
pub mod _7_nextconbk;
#[doc = "7_DEBUG (rw) register accessor: DMA Channel 7 Debug\n\nYou can [`read`](crate::Reg::read) this register and get [`_7_debug::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_7_debug::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_7_debug`]
module"]
#[doc(alias = "7_DEBUG")]
pub type _7Debug = crate::Reg<_7_debug::_7DebugSpec>;
#[doc = "DMA Channel 7 Debug"]
pub mod _7_debug;
#[doc = "8_CS (rw) register accessor: DMA Channel 8 Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`_8_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_8_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_8_cs`]
module"]
#[doc(alias = "8_CS")]
pub type _8Cs = crate::Reg<_8_cs::_8CsSpec>;
#[doc = "DMA Channel 8 Control and Status"]
pub mod _8_cs;
#[doc = "8_CONBLK_AD (rw) register accessor: DMA Channel 8 Control Block Address\n\nYou can [`read`](crate::Reg::read) this register and get [`_8_conblk_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_8_conblk_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_8_conblk_ad`]
module"]
#[doc(alias = "8_CONBLK_AD")]
pub type _8ConblkAd = crate::Reg<_8_conblk_ad::_8ConblkAdSpec>;
#[doc = "DMA Channel 8 Control Block Address"]
pub mod _8_conblk_ad;
#[doc = "8_TI (rw) register accessor: DMA Channel 8 CB Word 0 (Transfer Information)\n\nYou can [`read`](crate::Reg::read) this register and get [`_8_ti::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_8_ti::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_8_ti`]
module"]
#[doc(alias = "8_TI")]
pub type _8Ti = crate::Reg<_8_ti::_8TiSpec>;
#[doc = "DMA Channel 8 CB Word 0 (Transfer Information)"]
pub mod _8_ti;
#[doc = "8_SOURCE_AD (rw) register accessor: DMA Channel 8 CB Word 1 (Source Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_8_source_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_8_source_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_8_source_ad`]
module"]
#[doc(alias = "8_SOURCE_AD")]
pub type _8SourceAd = crate::Reg<_8_source_ad::_8SourceAdSpec>;
#[doc = "DMA Channel 8 CB Word 1 (Source Address)"]
pub mod _8_source_ad;
#[doc = "8_DEST_AD (rw) register accessor: DMA Channel 8 CB Word 2 (Destination Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_8_dest_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_8_dest_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_8_dest_ad`]
module"]
#[doc(alias = "8_DEST_AD")]
pub type _8DestAd = crate::Reg<_8_dest_ad::_8DestAdSpec>;
#[doc = "DMA Channel 8 CB Word 2 (Destination Address)"]
pub mod _8_dest_ad;
#[doc = "8_TXFR_LEN (rw) register accessor: DMA Channel 8 CB Word 3 (Transfer Length)\n\nYou can [`read`](crate::Reg::read) this register and get [`_8_txfr_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_8_txfr_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_8_txfr_len`]
module"]
#[doc(alias = "8_TXFR_LEN")]
pub type _8TxfrLen = crate::Reg<_8_txfr_len::_8TxfrLenSpec>;
#[doc = "DMA Channel 8 CB Word 3 (Transfer Length)"]
pub mod _8_txfr_len;
#[doc = "8_STRIDE (rw) register accessor: DMA Channel 8 CB Word 4 (2D Stride)\n\nYou can [`read`](crate::Reg::read) this register and get [`_8_stride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_8_stride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_8_stride`]
module"]
#[doc(alias = "8_STRIDE")]
pub type _8Stride = crate::Reg<_8_stride::_8StrideSpec>;
#[doc = "DMA Channel 8 CB Word 4 (2D Stride)"]
pub mod _8_stride;
#[doc = "8_NEXTCONBK (rw) register accessor: DMA Channel 8 CB Word 5 (Next CB Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_8_nextconbk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_8_nextconbk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_8_nextconbk`]
module"]
#[doc(alias = "8_NEXTCONBK")]
pub type _8Nextconbk = crate::Reg<_8_nextconbk::_8NextconbkSpec>;
#[doc = "DMA Channel 8 CB Word 5 (Next CB Address)"]
pub mod _8_nextconbk;
#[doc = "8_DEBUG (rw) register accessor: DMA Channel 8 Debug\n\nYou can [`read`](crate::Reg::read) this register and get [`_8_debug::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_8_debug::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_8_debug`]
module"]
#[doc(alias = "8_DEBUG")]
pub type _8Debug = crate::Reg<_8_debug::_8DebugSpec>;
#[doc = "DMA Channel 8 Debug"]
pub mod _8_debug;
#[doc = "9_CS (rw) register accessor: DMA Channel 9 Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`_9_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_9_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_9_cs`]
module"]
#[doc(alias = "9_CS")]
pub type _9Cs = crate::Reg<_9_cs::_9CsSpec>;
#[doc = "DMA Channel 9 Control and Status"]
pub mod _9_cs;
#[doc = "9_CONBLK_AD (rw) register accessor: DMA Channel 9 Control Block Address\n\nYou can [`read`](crate::Reg::read) this register and get [`_9_conblk_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_9_conblk_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_9_conblk_ad`]
module"]
#[doc(alias = "9_CONBLK_AD")]
pub type _9ConblkAd = crate::Reg<_9_conblk_ad::_9ConblkAdSpec>;
#[doc = "DMA Channel 9 Control Block Address"]
pub mod _9_conblk_ad;
#[doc = "9_TI (rw) register accessor: DMA Channel 9 CB Word 0 (Transfer Information)\n\nYou can [`read`](crate::Reg::read) this register and get [`_9_ti::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_9_ti::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_9_ti`]
module"]
#[doc(alias = "9_TI")]
pub type _9Ti = crate::Reg<_9_ti::_9TiSpec>;
#[doc = "DMA Channel 9 CB Word 0 (Transfer Information)"]
pub mod _9_ti;
#[doc = "9_SOURCE_AD (rw) register accessor: DMA Channel 9 CB Word 1 (Source Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_9_source_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_9_source_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_9_source_ad`]
module"]
#[doc(alias = "9_SOURCE_AD")]
pub type _9SourceAd = crate::Reg<_9_source_ad::_9SourceAdSpec>;
#[doc = "DMA Channel 9 CB Word 1 (Source Address)"]
pub mod _9_source_ad;
#[doc = "9_DEST_AD (rw) register accessor: DMA Channel 9 CB Word 2 (Destination Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_9_dest_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_9_dest_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_9_dest_ad`]
module"]
#[doc(alias = "9_DEST_AD")]
pub type _9DestAd = crate::Reg<_9_dest_ad::_9DestAdSpec>;
#[doc = "DMA Channel 9 CB Word 2 (Destination Address)"]
pub mod _9_dest_ad;
#[doc = "9_TXFR_LEN (rw) register accessor: DMA Channel 9 CB Word 3 (Transfer Length)\n\nYou can [`read`](crate::Reg::read) this register and get [`_9_txfr_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_9_txfr_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_9_txfr_len`]
module"]
#[doc(alias = "9_TXFR_LEN")]
pub type _9TxfrLen = crate::Reg<_9_txfr_len::_9TxfrLenSpec>;
#[doc = "DMA Channel 9 CB Word 3 (Transfer Length)"]
pub mod _9_txfr_len;
#[doc = "9_STRIDE (rw) register accessor: DMA Channel 9 CB Word 4 (2D Stride)\n\nYou can [`read`](crate::Reg::read) this register and get [`_9_stride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_9_stride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_9_stride`]
module"]
#[doc(alias = "9_STRIDE")]
pub type _9Stride = crate::Reg<_9_stride::_9StrideSpec>;
#[doc = "DMA Channel 9 CB Word 4 (2D Stride)"]
pub mod _9_stride;
#[doc = "9_NEXTCONBK (rw) register accessor: DMA Channel 9 CB Word 5 (Next CB Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_9_nextconbk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_9_nextconbk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_9_nextconbk`]
module"]
#[doc(alias = "9_NEXTCONBK")]
pub type _9Nextconbk = crate::Reg<_9_nextconbk::_9NextconbkSpec>;
#[doc = "DMA Channel 9 CB Word 5 (Next CB Address)"]
pub mod _9_nextconbk;
#[doc = "9_DEBUG (rw) register accessor: DMA Channel 9 Debug\n\nYou can [`read`](crate::Reg::read) this register and get [`_9_debug::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_9_debug::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_9_debug`]
module"]
#[doc(alias = "9_DEBUG")]
pub type _9Debug = crate::Reg<_9_debug::_9DebugSpec>;
#[doc = "DMA Channel 9 Debug"]
pub mod _9_debug;
#[doc = "10_CS (rw) register accessor: DMA Channel 10 Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`_10_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_10_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_10_cs`]
module"]
#[doc(alias = "10_CS")]
pub type _10Cs = crate::Reg<_10_cs::_10CsSpec>;
#[doc = "DMA Channel 10 Control and Status"]
pub mod _10_cs;
#[doc = "10_CONBLK_AD (rw) register accessor: DMA Channel 10 Control Block Address\n\nYou can [`read`](crate::Reg::read) this register and get [`_10_conblk_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_10_conblk_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_10_conblk_ad`]
module"]
#[doc(alias = "10_CONBLK_AD")]
pub type _10ConblkAd = crate::Reg<_10_conblk_ad::_10ConblkAdSpec>;
#[doc = "DMA Channel 10 Control Block Address"]
pub mod _10_conblk_ad;
#[doc = "10_TI (rw) register accessor: DMA Channel 10 CB Word 0 (Transfer Information)\n\nYou can [`read`](crate::Reg::read) this register and get [`_10_ti::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_10_ti::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_10_ti`]
module"]
#[doc(alias = "10_TI")]
pub type _10Ti = crate::Reg<_10_ti::_10TiSpec>;
#[doc = "DMA Channel 10 CB Word 0 (Transfer Information)"]
pub mod _10_ti;
#[doc = "10_SOURCE_AD (rw) register accessor: DMA Channel 10 CB Word 1 (Source Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_10_source_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_10_source_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_10_source_ad`]
module"]
#[doc(alias = "10_SOURCE_AD")]
pub type _10SourceAd = crate::Reg<_10_source_ad::_10SourceAdSpec>;
#[doc = "DMA Channel 10 CB Word 1 (Source Address)"]
pub mod _10_source_ad;
#[doc = "10_DEST_AD (rw) register accessor: DMA Channel 10 CB Word 2 (Destination Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_10_dest_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_10_dest_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_10_dest_ad`]
module"]
#[doc(alias = "10_DEST_AD")]
pub type _10DestAd = crate::Reg<_10_dest_ad::_10DestAdSpec>;
#[doc = "DMA Channel 10 CB Word 2 (Destination Address)"]
pub mod _10_dest_ad;
#[doc = "10_TXFR_LEN (rw) register accessor: DMA Channel 10 CB Word 3 (Transfer Length)\n\nYou can [`read`](crate::Reg::read) this register and get [`_10_txfr_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_10_txfr_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_10_txfr_len`]
module"]
#[doc(alias = "10_TXFR_LEN")]
pub type _10TxfrLen = crate::Reg<_10_txfr_len::_10TxfrLenSpec>;
#[doc = "DMA Channel 10 CB Word 3 (Transfer Length)"]
pub mod _10_txfr_len;
#[doc = "10_STRIDE (rw) register accessor: DMA Channel 10 CB Word 4 (2D Stride)\n\nYou can [`read`](crate::Reg::read) this register and get [`_10_stride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_10_stride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_10_stride`]
module"]
#[doc(alias = "10_STRIDE")]
pub type _10Stride = crate::Reg<_10_stride::_10StrideSpec>;
#[doc = "DMA Channel 10 CB Word 4 (2D Stride)"]
pub mod _10_stride;
#[doc = "10_NEXTCONBK (rw) register accessor: DMA Channel 10 CB Word 5 (Next CB Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_10_nextconbk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_10_nextconbk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_10_nextconbk`]
module"]
#[doc(alias = "10_NEXTCONBK")]
pub type _10Nextconbk = crate::Reg<_10_nextconbk::_10NextconbkSpec>;
#[doc = "DMA Channel 10 CB Word 5 (Next CB Address)"]
pub mod _10_nextconbk;
#[doc = "10_DEBUG (rw) register accessor: DMA Channel 10 Debug\n\nYou can [`read`](crate::Reg::read) this register and get [`_10_debug::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_10_debug::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_10_debug`]
module"]
#[doc(alias = "10_DEBUG")]
pub type _10Debug = crate::Reg<_10_debug::_10DebugSpec>;
#[doc = "DMA Channel 10 Debug"]
pub mod _10_debug;
#[doc = "11_CS (rw) register accessor: DMA Channel 11 Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`_11_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_11_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_11_cs`]
module"]
#[doc(alias = "11_CS")]
pub type _11Cs = crate::Reg<_11_cs::_11CsSpec>;
#[doc = "DMA Channel 11 Control and Status"]
pub mod _11_cs;
#[doc = "11_CONBLK_AD (rw) register accessor: DMA Channel 11 Control Block Address\n\nYou can [`read`](crate::Reg::read) this register and get [`_11_conblk_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_11_conblk_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_11_conblk_ad`]
module"]
#[doc(alias = "11_CONBLK_AD")]
pub type _11ConblkAd = crate::Reg<_11_conblk_ad::_11ConblkAdSpec>;
#[doc = "DMA Channel 11 Control Block Address"]
pub mod _11_conblk_ad;
#[doc = "11_TI (rw) register accessor: DMA Channel 11 CB Word 0 (Transfer Information)\n\nYou can [`read`](crate::Reg::read) this register and get [`_11_ti::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_11_ti::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_11_ti`]
module"]
#[doc(alias = "11_TI")]
pub type _11Ti = crate::Reg<_11_ti::_11TiSpec>;
#[doc = "DMA Channel 11 CB Word 0 (Transfer Information)"]
pub mod _11_ti;
#[doc = "11_SOURCE_AD (rw) register accessor: DMA Channel 11 CB Word 1 (Source Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_11_source_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_11_source_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_11_source_ad`]
module"]
#[doc(alias = "11_SOURCE_AD")]
pub type _11SourceAd = crate::Reg<_11_source_ad::_11SourceAdSpec>;
#[doc = "DMA Channel 11 CB Word 1 (Source Address)"]
pub mod _11_source_ad;
#[doc = "11_DEST_AD (rw) register accessor: DMA Channel 11 CB Word 2 (Destination Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_11_dest_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_11_dest_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_11_dest_ad`]
module"]
#[doc(alias = "11_DEST_AD")]
pub type _11DestAd = crate::Reg<_11_dest_ad::_11DestAdSpec>;
#[doc = "DMA Channel 11 CB Word 2 (Destination Address)"]
pub mod _11_dest_ad;
#[doc = "11_TXFR_LEN (rw) register accessor: DMA Channel 11 CB Word 3 (Transfer Length)\n\nYou can [`read`](crate::Reg::read) this register and get [`_11_txfr_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_11_txfr_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_11_txfr_len`]
module"]
#[doc(alias = "11_TXFR_LEN")]
pub type _11TxfrLen = crate::Reg<_11_txfr_len::_11TxfrLenSpec>;
#[doc = "DMA Channel 11 CB Word 3 (Transfer Length)"]
pub mod _11_txfr_len;
#[doc = "11_STRIDE (rw) register accessor: DMA Channel 11 CB Word 4 (2D Stride)\n\nYou can [`read`](crate::Reg::read) this register and get [`_11_stride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_11_stride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_11_stride`]
module"]
#[doc(alias = "11_STRIDE")]
pub type _11Stride = crate::Reg<_11_stride::_11StrideSpec>;
#[doc = "DMA Channel 11 CB Word 4 (2D Stride)"]
pub mod _11_stride;
#[doc = "11_NEXTCONBK (rw) register accessor: DMA Channel 11 CB Word 5 (Next CB Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_11_nextconbk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_11_nextconbk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_11_nextconbk`]
module"]
#[doc(alias = "11_NEXTCONBK")]
pub type _11Nextconbk = crate::Reg<_11_nextconbk::_11NextconbkSpec>;
#[doc = "DMA Channel 11 CB Word 5 (Next CB Address)"]
pub mod _11_nextconbk;
#[doc = "11_DEBUG (rw) register accessor: DMA Channel 11 Debug\n\nYou can [`read`](crate::Reg::read) this register and get [`_11_debug::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_11_debug::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_11_debug`]
module"]
#[doc(alias = "11_DEBUG")]
pub type _11Debug = crate::Reg<_11_debug::_11DebugSpec>;
#[doc = "DMA Channel 11 Debug"]
pub mod _11_debug;
#[doc = "12_CS (rw) register accessor: DMA Channel 12 Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`_12_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_12_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_12_cs`]
module"]
#[doc(alias = "12_CS")]
pub type _12Cs = crate::Reg<_12_cs::_12CsSpec>;
#[doc = "DMA Channel 12 Control and Status"]
pub mod _12_cs;
#[doc = "12_CONBLK_AD (rw) register accessor: DMA Channel 12 Control Block Address\n\nYou can [`read`](crate::Reg::read) this register and get [`_12_conblk_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_12_conblk_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_12_conblk_ad`]
module"]
#[doc(alias = "12_CONBLK_AD")]
pub type _12ConblkAd = crate::Reg<_12_conblk_ad::_12ConblkAdSpec>;
#[doc = "DMA Channel 12 Control Block Address"]
pub mod _12_conblk_ad;
#[doc = "12_TI (rw) register accessor: DMA Channel 12 CB Word 0 (Transfer Information)\n\nYou can [`read`](crate::Reg::read) this register and get [`_12_ti::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_12_ti::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_12_ti`]
module"]
#[doc(alias = "12_TI")]
pub type _12Ti = crate::Reg<_12_ti::_12TiSpec>;
#[doc = "DMA Channel 12 CB Word 0 (Transfer Information)"]
pub mod _12_ti;
#[doc = "12_SOURCE_AD (rw) register accessor: DMA Channel 12 CB Word 1 (Source Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_12_source_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_12_source_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_12_source_ad`]
module"]
#[doc(alias = "12_SOURCE_AD")]
pub type _12SourceAd = crate::Reg<_12_source_ad::_12SourceAdSpec>;
#[doc = "DMA Channel 12 CB Word 1 (Source Address)"]
pub mod _12_source_ad;
#[doc = "12_DEST_AD (rw) register accessor: DMA Channel 12 CB Word 2 (Destination Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_12_dest_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_12_dest_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_12_dest_ad`]
module"]
#[doc(alias = "12_DEST_AD")]
pub type _12DestAd = crate::Reg<_12_dest_ad::_12DestAdSpec>;
#[doc = "DMA Channel 12 CB Word 2 (Destination Address)"]
pub mod _12_dest_ad;
#[doc = "12_TXFR_LEN (rw) register accessor: DMA Channel 12 CB Word 3 (Transfer Length)\n\nYou can [`read`](crate::Reg::read) this register and get [`_12_txfr_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_12_txfr_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_12_txfr_len`]
module"]
#[doc(alias = "12_TXFR_LEN")]
pub type _12TxfrLen = crate::Reg<_12_txfr_len::_12TxfrLenSpec>;
#[doc = "DMA Channel 12 CB Word 3 (Transfer Length)"]
pub mod _12_txfr_len;
#[doc = "12_STRIDE (rw) register accessor: DMA Channel 12 CB Word 4 (2D Stride)\n\nYou can [`read`](crate::Reg::read) this register and get [`_12_stride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_12_stride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_12_stride`]
module"]
#[doc(alias = "12_STRIDE")]
pub type _12Stride = crate::Reg<_12_stride::_12StrideSpec>;
#[doc = "DMA Channel 12 CB Word 4 (2D Stride)"]
pub mod _12_stride;
#[doc = "12_NEXTCONBK (rw) register accessor: DMA Channel 12 CB Word 5 (Next CB Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_12_nextconbk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_12_nextconbk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_12_nextconbk`]
module"]
#[doc(alias = "12_NEXTCONBK")]
pub type _12Nextconbk = crate::Reg<_12_nextconbk::_12NextconbkSpec>;
#[doc = "DMA Channel 12 CB Word 5 (Next CB Address)"]
pub mod _12_nextconbk;
#[doc = "12_DEBUG (rw) register accessor: DMA Channel 12 Debug\n\nYou can [`read`](crate::Reg::read) this register and get [`_12_debug::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_12_debug::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_12_debug`]
module"]
#[doc(alias = "12_DEBUG")]
pub type _12Debug = crate::Reg<_12_debug::_12DebugSpec>;
#[doc = "DMA Channel 12 Debug"]
pub mod _12_debug;
#[doc = "13_CS (rw) register accessor: DMA Channel 13 Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`_13_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_13_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_13_cs`]
module"]
#[doc(alias = "13_CS")]
pub type _13Cs = crate::Reg<_13_cs::_13CsSpec>;
#[doc = "DMA Channel 13 Control and Status"]
pub mod _13_cs;
#[doc = "13_CONBLK_AD (rw) register accessor: DMA Channel 13 Control Block Address\n\nYou can [`read`](crate::Reg::read) this register and get [`_13_conblk_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_13_conblk_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_13_conblk_ad`]
module"]
#[doc(alias = "13_CONBLK_AD")]
pub type _13ConblkAd = crate::Reg<_13_conblk_ad::_13ConblkAdSpec>;
#[doc = "DMA Channel 13 Control Block Address"]
pub mod _13_conblk_ad;
#[doc = "13_TI (rw) register accessor: DMA Channel 13 CB Word 0 (Transfer Information)\n\nYou can [`read`](crate::Reg::read) this register and get [`_13_ti::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_13_ti::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_13_ti`]
module"]
#[doc(alias = "13_TI")]
pub type _13Ti = crate::Reg<_13_ti::_13TiSpec>;
#[doc = "DMA Channel 13 CB Word 0 (Transfer Information)"]
pub mod _13_ti;
#[doc = "13_SOURCE_AD (rw) register accessor: DMA Channel 13 CB Word 1 (Source Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_13_source_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_13_source_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_13_source_ad`]
module"]
#[doc(alias = "13_SOURCE_AD")]
pub type _13SourceAd = crate::Reg<_13_source_ad::_13SourceAdSpec>;
#[doc = "DMA Channel 13 CB Word 1 (Source Address)"]
pub mod _13_source_ad;
#[doc = "13_DEST_AD (rw) register accessor: DMA Channel 13 CB Word 2 (Destination Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_13_dest_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_13_dest_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_13_dest_ad`]
module"]
#[doc(alias = "13_DEST_AD")]
pub type _13DestAd = crate::Reg<_13_dest_ad::_13DestAdSpec>;
#[doc = "DMA Channel 13 CB Word 2 (Destination Address)"]
pub mod _13_dest_ad;
#[doc = "13_TXFR_LEN (rw) register accessor: DMA Channel 13 CB Word 3 (Transfer Length)\n\nYou can [`read`](crate::Reg::read) this register and get [`_13_txfr_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_13_txfr_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_13_txfr_len`]
module"]
#[doc(alias = "13_TXFR_LEN")]
pub type _13TxfrLen = crate::Reg<_13_txfr_len::_13TxfrLenSpec>;
#[doc = "DMA Channel 13 CB Word 3 (Transfer Length)"]
pub mod _13_txfr_len;
#[doc = "13_STRIDE (rw) register accessor: DMA Channel 13 CB Word 4 (2D Stride)\n\nYou can [`read`](crate::Reg::read) this register and get [`_13_stride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_13_stride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_13_stride`]
module"]
#[doc(alias = "13_STRIDE")]
pub type _13Stride = crate::Reg<_13_stride::_13StrideSpec>;
#[doc = "DMA Channel 13 CB Word 4 (2D Stride)"]
pub mod _13_stride;
#[doc = "13_NEXTCONBK (rw) register accessor: DMA Channel 13 CB Word 5 (Next CB Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_13_nextconbk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_13_nextconbk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_13_nextconbk`]
module"]
#[doc(alias = "13_NEXTCONBK")]
pub type _13Nextconbk = crate::Reg<_13_nextconbk::_13NextconbkSpec>;
#[doc = "DMA Channel 13 CB Word 5 (Next CB Address)"]
pub mod _13_nextconbk;
#[doc = "13_DEBUG (rw) register accessor: DMA Channel 13 Debug\n\nYou can [`read`](crate::Reg::read) this register and get [`_13_debug::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_13_debug::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_13_debug`]
module"]
#[doc(alias = "13_DEBUG")]
pub type _13Debug = crate::Reg<_13_debug::_13DebugSpec>;
#[doc = "DMA Channel 13 Debug"]
pub mod _13_debug;
#[doc = "14_CS (rw) register accessor: DMA Channel 14 Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`_14_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_14_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_14_cs`]
module"]
#[doc(alias = "14_CS")]
pub type _14Cs = crate::Reg<_14_cs::_14CsSpec>;
#[doc = "DMA Channel 14 Control and Status"]
pub mod _14_cs;
#[doc = "14_CONBLK_AD (rw) register accessor: DMA Channel 14 Control Block Address\n\nYou can [`read`](crate::Reg::read) this register and get [`_14_conblk_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_14_conblk_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_14_conblk_ad`]
module"]
#[doc(alias = "14_CONBLK_AD")]
pub type _14ConblkAd = crate::Reg<_14_conblk_ad::_14ConblkAdSpec>;
#[doc = "DMA Channel 14 Control Block Address"]
pub mod _14_conblk_ad;
#[doc = "14_TI (rw) register accessor: DMA Channel 14 CB Word 0 (Transfer Information)\n\nYou can [`read`](crate::Reg::read) this register and get [`_14_ti::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_14_ti::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_14_ti`]
module"]
#[doc(alias = "14_TI")]
pub type _14Ti = crate::Reg<_14_ti::_14TiSpec>;
#[doc = "DMA Channel 14 CB Word 0 (Transfer Information)"]
pub mod _14_ti;
#[doc = "14_SOURCE_AD (rw) register accessor: DMA Channel 14 CB Word 1 (Source Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_14_source_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_14_source_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_14_source_ad`]
module"]
#[doc(alias = "14_SOURCE_AD")]
pub type _14SourceAd = crate::Reg<_14_source_ad::_14SourceAdSpec>;
#[doc = "DMA Channel 14 CB Word 1 (Source Address)"]
pub mod _14_source_ad;
#[doc = "14_DEST_AD (rw) register accessor: DMA Channel 14 CB Word 2 (Destination Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_14_dest_ad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_14_dest_ad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_14_dest_ad`]
module"]
#[doc(alias = "14_DEST_AD")]
pub type _14DestAd = crate::Reg<_14_dest_ad::_14DestAdSpec>;
#[doc = "DMA Channel 14 CB Word 2 (Destination Address)"]
pub mod _14_dest_ad;
#[doc = "14_TXFR_LEN (rw) register accessor: DMA Channel 14 CB Word 3 (Transfer Length)\n\nYou can [`read`](crate::Reg::read) this register and get [`_14_txfr_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_14_txfr_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_14_txfr_len`]
module"]
#[doc(alias = "14_TXFR_LEN")]
pub type _14TxfrLen = crate::Reg<_14_txfr_len::_14TxfrLenSpec>;
#[doc = "DMA Channel 14 CB Word 3 (Transfer Length)"]
pub mod _14_txfr_len;
#[doc = "14_STRIDE (rw) register accessor: DMA Channel 14 CB Word 4 (2D Stride)\n\nYou can [`read`](crate::Reg::read) this register and get [`_14_stride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_14_stride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_14_stride`]
module"]
#[doc(alias = "14_STRIDE")]
pub type _14Stride = crate::Reg<_14_stride::_14StrideSpec>;
#[doc = "DMA Channel 14 CB Word 4 (2D Stride)"]
pub mod _14_stride;
#[doc = "14_NEXTCONBK (rw) register accessor: DMA Channel 14 CB Word 5 (Next CB Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_14_nextconbk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_14_nextconbk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_14_nextconbk`]
module"]
#[doc(alias = "14_NEXTCONBK")]
pub type _14Nextconbk = crate::Reg<_14_nextconbk::_14NextconbkSpec>;
#[doc = "DMA Channel 14 CB Word 5 (Next CB Address)"]
pub mod _14_nextconbk;
#[doc = "14_DEBUG (rw) register accessor: DMA Channel 14 Debug\n\nYou can [`read`](crate::Reg::read) this register and get [`_14_debug::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_14_debug::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_14_debug`]
module"]
#[doc(alias = "14_DEBUG")]
pub type _14Debug = crate::Reg<_14_debug::_14DebugSpec>;
#[doc = "DMA Channel 14 Debug"]
pub mod _14_debug;
#[doc = "INT_STATUS (rw) register accessor: Interrupt status of each DMA channel\n\nYou can [`read`](crate::Reg::read) this register and get [`int_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_status`]
module"]
#[doc(alias = "INT_STATUS")]
pub type IntStatus = crate::Reg<int_status::IntStatusSpec>;
#[doc = "Interrupt status of each DMA channel"]
pub mod int_status;
#[doc = "ENABLE (rw) register accessor: Global enable bits for each DMA channel\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`]
module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Global enable bits for each DMA channel"]
pub mod enable;

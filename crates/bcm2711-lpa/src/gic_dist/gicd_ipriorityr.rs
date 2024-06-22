#[repr(C)]
#[derive(Debug)]
#[doc = "Interrupt Priority"]
#[doc(alias = "GICD_IPRIORITYR")]
pub struct GicdIpriorityr {
    gicd_ipriorityr0: GicdIpriorityr0,
    gicd_ipriorityr1: GicdIpriorityr1,
    gicd_ipriorityr2: GicdIpriorityr2,
    gicd_ipriorityr3: GicdIpriorityr3,
    gicd_ipriorityr4: GicdIpriorityr4,
    gicd_ipriorityr5: GicdIpriorityr5,
    gicd_ipriorityr6: GicdIpriorityr6,
    gicd_ipriorityr7: GicdIpriorityr7,
    gicd_ipriorityr8: GicdIpriorityr8,
    gicd_ipriorityr9: GicdIpriorityr9,
    gicd_ipriorityr10: GicdIpriorityr10,
    gicd_ipriorityr11: GicdIpriorityr11,
    gicd_ipriorityr12: GicdIpriorityr12,
    gicd_ipriorityr13: GicdIpriorityr13,
    gicd_ipriorityr14: GicdIpriorityr14,
    gicd_ipriorityr15: GicdIpriorityr15,
    gicd_ipriorityr16: GicdIpriorityr16,
    gicd_ipriorityr17: GicdIpriorityr17,
    gicd_ipriorityr18: GicdIpriorityr18,
    gicd_ipriorityr19: GicdIpriorityr19,
    gicd_ipriorityr20: GicdIpriorityr20,
    gicd_ipriorityr21: GicdIpriorityr21,
    gicd_ipriorityr22: GicdIpriorityr22,
    gicd_ipriorityr23: GicdIpriorityr23,
    gicd_ipriorityr24: GicdIpriorityr24,
    gicd_ipriorityr25: GicdIpriorityr25,
    gicd_ipriorityr26: GicdIpriorityr26,
    gicd_ipriorityr27: GicdIpriorityr27,
    gicd_ipriorityr28: GicdIpriorityr28,
    gicd_ipriorityr29: GicdIpriorityr29,
    gicd_ipriorityr30: GicdIpriorityr30,
    gicd_ipriorityr31: GicdIpriorityr31,
    gicd_ipriorityr32: GicdIpriorityr32,
    gicd_ipriorityr33: GicdIpriorityr33,
    gicd_ipriorityr34: GicdIpriorityr34,
    gicd_ipriorityr35: GicdIpriorityr35,
    gicd_ipriorityr36: GicdIpriorityr36,
    gicd_ipriorityr37: GicdIpriorityr37,
    gicd_ipriorityr38: GicdIpriorityr38,
    gicd_ipriorityr39: GicdIpriorityr39,
    gicd_ipriorityr40: GicdIpriorityr40,
    gicd_ipriorityr41: GicdIpriorityr41,
    gicd_ipriorityr42: GicdIpriorityr42,
    gicd_ipriorityr43: GicdIpriorityr43,
    gicd_ipriorityr44: GicdIpriorityr44,
    gicd_ipriorityr45: GicdIpriorityr45,
    gicd_ipriorityr46: GicdIpriorityr46,
    gicd_ipriorityr47: GicdIpriorityr47,
    gicd_ipriorityr48: GicdIpriorityr48,
    gicd_ipriorityr49: GicdIpriorityr49,
    gicd_ipriorityr50: GicdIpriorityr50,
    gicd_ipriorityr51: GicdIpriorityr51,
    gicd_ipriorityr52: GicdIpriorityr52,
    gicd_ipriorityr53: GicdIpriorityr53,
    gicd_ipriorityr54: GicdIpriorityr54,
    gicd_ipriorityr55: GicdIpriorityr55,
}
impl GicdIpriorityr {
    #[doc = "0x00 - Interrupt Priority 0 - 3 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr0(&self) -> &GicdIpriorityr0 {
        &self.gicd_ipriorityr0
    }
    #[doc = "0x04 - Interrupt Priority 4 - 7 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr1(&self) -> &GicdIpriorityr1 {
        &self.gicd_ipriorityr1
    }
    #[doc = "0x08 - Interrupt Priority 8 - 11 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr2(&self) -> &GicdIpriorityr2 {
        &self.gicd_ipriorityr2
    }
    #[doc = "0x0c - Interrupt Priority 12 - 15 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr3(&self) -> &GicdIpriorityr3 {
        &self.gicd_ipriorityr3
    }
    #[doc = "0x10 - Interrupt Priority 16 - 19 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr4(&self) -> &GicdIpriorityr4 {
        &self.gicd_ipriorityr4
    }
    #[doc = "0x14 - Interrupt Priority 20 - 23 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr5(&self) -> &GicdIpriorityr5 {
        &self.gicd_ipriorityr5
    }
    #[doc = "0x18 - Interrupt Priority 24 - 27 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr6(&self) -> &GicdIpriorityr6 {
        &self.gicd_ipriorityr6
    }
    #[doc = "0x1c - Interrupt Priority 28 - 31 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr7(&self) -> &GicdIpriorityr7 {
        &self.gicd_ipriorityr7
    }
    #[doc = "0x20 - Interrupt Priority 32 - 35 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr8(&self) -> &GicdIpriorityr8 {
        &self.gicd_ipriorityr8
    }
    #[doc = "0x24 - Interrupt Priority 36 - 39 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr9(&self) -> &GicdIpriorityr9 {
        &self.gicd_ipriorityr9
    }
    #[doc = "0x28 - Interrupt Priority 40 - 43 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr10(&self) -> &GicdIpriorityr10 {
        &self.gicd_ipriorityr10
    }
    #[doc = "0x2c - Interrupt Priority 44 - 47 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr11(&self) -> &GicdIpriorityr11 {
        &self.gicd_ipriorityr11
    }
    #[doc = "0x30 - Interrupt Priority 48 - 51 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr12(&self) -> &GicdIpriorityr12 {
        &self.gicd_ipriorityr12
    }
    #[doc = "0x34 - Interrupt Priority 52 - 55 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr13(&self) -> &GicdIpriorityr13 {
        &self.gicd_ipriorityr13
    }
    #[doc = "0x38 - Interrupt Priority 56 - 59 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr14(&self) -> &GicdIpriorityr14 {
        &self.gicd_ipriorityr14
    }
    #[doc = "0x3c - Interrupt Priority 60 - 63 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr15(&self) -> &GicdIpriorityr15 {
        &self.gicd_ipriorityr15
    }
    #[doc = "0x40 - Interrupt Priority 64 - 67 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr16(&self) -> &GicdIpriorityr16 {
        &self.gicd_ipriorityr16
    }
    #[doc = "0x44 - Interrupt Priority 68 - 71 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr17(&self) -> &GicdIpriorityr17 {
        &self.gicd_ipriorityr17
    }
    #[doc = "0x48 - Interrupt Priority 72 - 75 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr18(&self) -> &GicdIpriorityr18 {
        &self.gicd_ipriorityr18
    }
    #[doc = "0x4c - Interrupt Priority 76 - 79 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr19(&self) -> &GicdIpriorityr19 {
        &self.gicd_ipriorityr19
    }
    #[doc = "0x50 - Interrupt Priority 80 - 83 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr20(&self) -> &GicdIpriorityr20 {
        &self.gicd_ipriorityr20
    }
    #[doc = "0x54 - Interrupt Priority 84 - 87 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr21(&self) -> &GicdIpriorityr21 {
        &self.gicd_ipriorityr21
    }
    #[doc = "0x58 - Interrupt Priority 88 - 91 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr22(&self) -> &GicdIpriorityr22 {
        &self.gicd_ipriorityr22
    }
    #[doc = "0x5c - Interrupt Priority 92 - 95 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr23(&self) -> &GicdIpriorityr23 {
        &self.gicd_ipriorityr23
    }
    #[doc = "0x60 - Interrupt Priority 96 - 99 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr24(&self) -> &GicdIpriorityr24 {
        &self.gicd_ipriorityr24
    }
    #[doc = "0x64 - Interrupt Priority 100 - 103 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr25(&self) -> &GicdIpriorityr25 {
        &self.gicd_ipriorityr25
    }
    #[doc = "0x68 - Interrupt Priority 104 - 107 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr26(&self) -> &GicdIpriorityr26 {
        &self.gicd_ipriorityr26
    }
    #[doc = "0x6c - Interrupt Priority 108 - 111 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr27(&self) -> &GicdIpriorityr27 {
        &self.gicd_ipriorityr27
    }
    #[doc = "0x70 - Interrupt Priority 112 - 115 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr28(&self) -> &GicdIpriorityr28 {
        &self.gicd_ipriorityr28
    }
    #[doc = "0x74 - Interrupt Priority 116 - 119 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr29(&self) -> &GicdIpriorityr29 {
        &self.gicd_ipriorityr29
    }
    #[doc = "0x78 - Interrupt Priority 120 - 123 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr30(&self) -> &GicdIpriorityr30 {
        &self.gicd_ipriorityr30
    }
    #[doc = "0x7c - Interrupt Priority 124 - 127 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr31(&self) -> &GicdIpriorityr31 {
        &self.gicd_ipriorityr31
    }
    #[doc = "0x80 - Interrupt Priority 128 - 131 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr32(&self) -> &GicdIpriorityr32 {
        &self.gicd_ipriorityr32
    }
    #[doc = "0x84 - Interrupt Priority 132 - 135 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr33(&self) -> &GicdIpriorityr33 {
        &self.gicd_ipriorityr33
    }
    #[doc = "0x88 - Interrupt Priority 136 - 139 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr34(&self) -> &GicdIpriorityr34 {
        &self.gicd_ipriorityr34
    }
    #[doc = "0x8c - Interrupt Priority 140 - 143 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr35(&self) -> &GicdIpriorityr35 {
        &self.gicd_ipriorityr35
    }
    #[doc = "0x90 - Interrupt Priority 144 - 147 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr36(&self) -> &GicdIpriorityr36 {
        &self.gicd_ipriorityr36
    }
    #[doc = "0x94 - Interrupt Priority 148 - 151 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr37(&self) -> &GicdIpriorityr37 {
        &self.gicd_ipriorityr37
    }
    #[doc = "0x98 - Interrupt Priority 152 - 155 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr38(&self) -> &GicdIpriorityr38 {
        &self.gicd_ipriorityr38
    }
    #[doc = "0x9c - Interrupt Priority 156 - 159 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr39(&self) -> &GicdIpriorityr39 {
        &self.gicd_ipriorityr39
    }
    #[doc = "0xa0 - Interrupt Priority 160 - 163 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr40(&self) -> &GicdIpriorityr40 {
        &self.gicd_ipriorityr40
    }
    #[doc = "0xa4 - Interrupt Priority 164 - 167 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr41(&self) -> &GicdIpriorityr41 {
        &self.gicd_ipriorityr41
    }
    #[doc = "0xa8 - Interrupt Priority 168 - 171 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr42(&self) -> &GicdIpriorityr42 {
        &self.gicd_ipriorityr42
    }
    #[doc = "0xac - Interrupt Priority 172 - 175 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr43(&self) -> &GicdIpriorityr43 {
        &self.gicd_ipriorityr43
    }
    #[doc = "0xb0 - Interrupt Priority 176 - 179 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr44(&self) -> &GicdIpriorityr44 {
        &self.gicd_ipriorityr44
    }
    #[doc = "0xb4 - Interrupt Priority 180 - 183 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr45(&self) -> &GicdIpriorityr45 {
        &self.gicd_ipriorityr45
    }
    #[doc = "0xb8 - Interrupt Priority 184 - 187 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr46(&self) -> &GicdIpriorityr46 {
        &self.gicd_ipriorityr46
    }
    #[doc = "0xbc - Interrupt Priority 188 - 191 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr47(&self) -> &GicdIpriorityr47 {
        &self.gicd_ipriorityr47
    }
    #[doc = "0xc0 - Interrupt Priority 192 - 195 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr48(&self) -> &GicdIpriorityr48 {
        &self.gicd_ipriorityr48
    }
    #[doc = "0xc4 - Interrupt Priority 196 - 199 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr49(&self) -> &GicdIpriorityr49 {
        &self.gicd_ipriorityr49
    }
    #[doc = "0xc8 - Interrupt Priority 200 - 203 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr50(&self) -> &GicdIpriorityr50 {
        &self.gicd_ipriorityr50
    }
    #[doc = "0xcc - Interrupt Priority 204 - 207 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr51(&self) -> &GicdIpriorityr51 {
        &self.gicd_ipriorityr51
    }
    #[doc = "0xd0 - Interrupt Priority 208 - 211 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr52(&self) -> &GicdIpriorityr52 {
        &self.gicd_ipriorityr52
    }
    #[doc = "0xd4 - Interrupt Priority 212 - 215 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr53(&self) -> &GicdIpriorityr53 {
        &self.gicd_ipriorityr53
    }
    #[doc = "0xd8 - Interrupt Priority 216 - 219 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr54(&self) -> &GicdIpriorityr54 {
        &self.gicd_ipriorityr54
    }
    #[doc = "0xdc - Interrupt Priority 220 - 223 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr55(&self) -> &GicdIpriorityr55 {
        &self.gicd_ipriorityr55
    }
}
#[doc = "GICD_IPRIORITYR0 (rw) register accessor: Interrupt Priority 0 - 3 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr0`]
module"]
#[doc(alias = "GICD_IPRIORITYR0")]
pub type GicdIpriorityr0 = crate::Reg<gicd_ipriorityr0::GicdIpriorityr0Spec>;
#[doc = "Interrupt Priority 0 - 3 (Lower is first)"]
pub mod gicd_ipriorityr0;
#[doc = "GICD_IPRIORITYR1 (rw) register accessor: Interrupt Priority 4 - 7 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr1`]
module"]
#[doc(alias = "GICD_IPRIORITYR1")]
pub type GicdIpriorityr1 = crate::Reg<gicd_ipriorityr1::GicdIpriorityr1Spec>;
#[doc = "Interrupt Priority 4 - 7 (Lower is first)"]
pub mod gicd_ipriorityr1;
#[doc = "GICD_IPRIORITYR2 (rw) register accessor: Interrupt Priority 8 - 11 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr2`]
module"]
#[doc(alias = "GICD_IPRIORITYR2")]
pub type GicdIpriorityr2 = crate::Reg<gicd_ipriorityr2::GicdIpriorityr2Spec>;
#[doc = "Interrupt Priority 8 - 11 (Lower is first)"]
pub mod gicd_ipriorityr2;
#[doc = "GICD_IPRIORITYR3 (rw) register accessor: Interrupt Priority 12 - 15 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr3`]
module"]
#[doc(alias = "GICD_IPRIORITYR3")]
pub type GicdIpriorityr3 = crate::Reg<gicd_ipriorityr3::GicdIpriorityr3Spec>;
#[doc = "Interrupt Priority 12 - 15 (Lower is first)"]
pub mod gicd_ipriorityr3;
#[doc = "GICD_IPRIORITYR4 (rw) register accessor: Interrupt Priority 16 - 19 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr4`]
module"]
#[doc(alias = "GICD_IPRIORITYR4")]
pub type GicdIpriorityr4 = crate::Reg<gicd_ipriorityr4::GicdIpriorityr4Spec>;
#[doc = "Interrupt Priority 16 - 19 (Lower is first)"]
pub mod gicd_ipriorityr4;
#[doc = "GICD_IPRIORITYR5 (rw) register accessor: Interrupt Priority 20 - 23 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr5`]
module"]
#[doc(alias = "GICD_IPRIORITYR5")]
pub type GicdIpriorityr5 = crate::Reg<gicd_ipriorityr5::GicdIpriorityr5Spec>;
#[doc = "Interrupt Priority 20 - 23 (Lower is first)"]
pub mod gicd_ipriorityr5;
#[doc = "GICD_IPRIORITYR6 (rw) register accessor: Interrupt Priority 24 - 27 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr6`]
module"]
#[doc(alias = "GICD_IPRIORITYR6")]
pub type GicdIpriorityr6 = crate::Reg<gicd_ipriorityr6::GicdIpriorityr6Spec>;
#[doc = "Interrupt Priority 24 - 27 (Lower is first)"]
pub mod gicd_ipriorityr6;
#[doc = "GICD_IPRIORITYR7 (rw) register accessor: Interrupt Priority 28 - 31 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr7`]
module"]
#[doc(alias = "GICD_IPRIORITYR7")]
pub type GicdIpriorityr7 = crate::Reg<gicd_ipriorityr7::GicdIpriorityr7Spec>;
#[doc = "Interrupt Priority 28 - 31 (Lower is first)"]
pub mod gicd_ipriorityr7;
#[doc = "GICD_IPRIORITYR8 (rw) register accessor: Interrupt Priority 32 - 35 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr8`]
module"]
#[doc(alias = "GICD_IPRIORITYR8")]
pub type GicdIpriorityr8 = crate::Reg<gicd_ipriorityr8::GicdIpriorityr8Spec>;
#[doc = "Interrupt Priority 32 - 35 (Lower is first)"]
pub mod gicd_ipriorityr8;
#[doc = "GICD_IPRIORITYR9 (rw) register accessor: Interrupt Priority 36 - 39 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr9`]
module"]
#[doc(alias = "GICD_IPRIORITYR9")]
pub type GicdIpriorityr9 = crate::Reg<gicd_ipriorityr9::GicdIpriorityr9Spec>;
#[doc = "Interrupt Priority 36 - 39 (Lower is first)"]
pub mod gicd_ipriorityr9;
#[doc = "GICD_IPRIORITYR10 (rw) register accessor: Interrupt Priority 40 - 43 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr10`]
module"]
#[doc(alias = "GICD_IPRIORITYR10")]
pub type GicdIpriorityr10 = crate::Reg<gicd_ipriorityr10::GicdIpriorityr10Spec>;
#[doc = "Interrupt Priority 40 - 43 (Lower is first)"]
pub mod gicd_ipriorityr10;
#[doc = "GICD_IPRIORITYR11 (rw) register accessor: Interrupt Priority 44 - 47 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr11`]
module"]
#[doc(alias = "GICD_IPRIORITYR11")]
pub type GicdIpriorityr11 = crate::Reg<gicd_ipriorityr11::GicdIpriorityr11Spec>;
#[doc = "Interrupt Priority 44 - 47 (Lower is first)"]
pub mod gicd_ipriorityr11;
#[doc = "GICD_IPRIORITYR12 (rw) register accessor: Interrupt Priority 48 - 51 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr12`]
module"]
#[doc(alias = "GICD_IPRIORITYR12")]
pub type GicdIpriorityr12 = crate::Reg<gicd_ipriorityr12::GicdIpriorityr12Spec>;
#[doc = "Interrupt Priority 48 - 51 (Lower is first)"]
pub mod gicd_ipriorityr12;
#[doc = "GICD_IPRIORITYR13 (rw) register accessor: Interrupt Priority 52 - 55 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr13`]
module"]
#[doc(alias = "GICD_IPRIORITYR13")]
pub type GicdIpriorityr13 = crate::Reg<gicd_ipriorityr13::GicdIpriorityr13Spec>;
#[doc = "Interrupt Priority 52 - 55 (Lower is first)"]
pub mod gicd_ipriorityr13;
#[doc = "GICD_IPRIORITYR14 (rw) register accessor: Interrupt Priority 56 - 59 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr14`]
module"]
#[doc(alias = "GICD_IPRIORITYR14")]
pub type GicdIpriorityr14 = crate::Reg<gicd_ipriorityr14::GicdIpriorityr14Spec>;
#[doc = "Interrupt Priority 56 - 59 (Lower is first)"]
pub mod gicd_ipriorityr14;
#[doc = "GICD_IPRIORITYR15 (rw) register accessor: Interrupt Priority 60 - 63 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr15`]
module"]
#[doc(alias = "GICD_IPRIORITYR15")]
pub type GicdIpriorityr15 = crate::Reg<gicd_ipriorityr15::GicdIpriorityr15Spec>;
#[doc = "Interrupt Priority 60 - 63 (Lower is first)"]
pub mod gicd_ipriorityr15;
#[doc = "GICD_IPRIORITYR16 (rw) register accessor: Interrupt Priority 64 - 67 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr16`]
module"]
#[doc(alias = "GICD_IPRIORITYR16")]
pub type GicdIpriorityr16 = crate::Reg<gicd_ipriorityr16::GicdIpriorityr16Spec>;
#[doc = "Interrupt Priority 64 - 67 (Lower is first)"]
pub mod gicd_ipriorityr16;
#[doc = "GICD_IPRIORITYR17 (rw) register accessor: Interrupt Priority 68 - 71 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr17`]
module"]
#[doc(alias = "GICD_IPRIORITYR17")]
pub type GicdIpriorityr17 = crate::Reg<gicd_ipriorityr17::GicdIpriorityr17Spec>;
#[doc = "Interrupt Priority 68 - 71 (Lower is first)"]
pub mod gicd_ipriorityr17;
#[doc = "GICD_IPRIORITYR18 (rw) register accessor: Interrupt Priority 72 - 75 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr18`]
module"]
#[doc(alias = "GICD_IPRIORITYR18")]
pub type GicdIpriorityr18 = crate::Reg<gicd_ipriorityr18::GicdIpriorityr18Spec>;
#[doc = "Interrupt Priority 72 - 75 (Lower is first)"]
pub mod gicd_ipriorityr18;
#[doc = "GICD_IPRIORITYR19 (rw) register accessor: Interrupt Priority 76 - 79 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr19`]
module"]
#[doc(alias = "GICD_IPRIORITYR19")]
pub type GicdIpriorityr19 = crate::Reg<gicd_ipriorityr19::GicdIpriorityr19Spec>;
#[doc = "Interrupt Priority 76 - 79 (Lower is first)"]
pub mod gicd_ipriorityr19;
#[doc = "GICD_IPRIORITYR20 (rw) register accessor: Interrupt Priority 80 - 83 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr20`]
module"]
#[doc(alias = "GICD_IPRIORITYR20")]
pub type GicdIpriorityr20 = crate::Reg<gicd_ipriorityr20::GicdIpriorityr20Spec>;
#[doc = "Interrupt Priority 80 - 83 (Lower is first)"]
pub mod gicd_ipriorityr20;
#[doc = "GICD_IPRIORITYR21 (rw) register accessor: Interrupt Priority 84 - 87 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr21`]
module"]
#[doc(alias = "GICD_IPRIORITYR21")]
pub type GicdIpriorityr21 = crate::Reg<gicd_ipriorityr21::GicdIpriorityr21Spec>;
#[doc = "Interrupt Priority 84 - 87 (Lower is first)"]
pub mod gicd_ipriorityr21;
#[doc = "GICD_IPRIORITYR22 (rw) register accessor: Interrupt Priority 88 - 91 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr22`]
module"]
#[doc(alias = "GICD_IPRIORITYR22")]
pub type GicdIpriorityr22 = crate::Reg<gicd_ipriorityr22::GicdIpriorityr22Spec>;
#[doc = "Interrupt Priority 88 - 91 (Lower is first)"]
pub mod gicd_ipriorityr22;
#[doc = "GICD_IPRIORITYR23 (rw) register accessor: Interrupt Priority 92 - 95 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr23`]
module"]
#[doc(alias = "GICD_IPRIORITYR23")]
pub type GicdIpriorityr23 = crate::Reg<gicd_ipriorityr23::GicdIpriorityr23Spec>;
#[doc = "Interrupt Priority 92 - 95 (Lower is first)"]
pub mod gicd_ipriorityr23;
#[doc = "GICD_IPRIORITYR24 (rw) register accessor: Interrupt Priority 96 - 99 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr24`]
module"]
#[doc(alias = "GICD_IPRIORITYR24")]
pub type GicdIpriorityr24 = crate::Reg<gicd_ipriorityr24::GicdIpriorityr24Spec>;
#[doc = "Interrupt Priority 96 - 99 (Lower is first)"]
pub mod gicd_ipriorityr24;
#[doc = "GICD_IPRIORITYR25 (rw) register accessor: Interrupt Priority 100 - 103 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr25`]
module"]
#[doc(alias = "GICD_IPRIORITYR25")]
pub type GicdIpriorityr25 = crate::Reg<gicd_ipriorityr25::GicdIpriorityr25Spec>;
#[doc = "Interrupt Priority 100 - 103 (Lower is first)"]
pub mod gicd_ipriorityr25;
#[doc = "GICD_IPRIORITYR26 (rw) register accessor: Interrupt Priority 104 - 107 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr26`]
module"]
#[doc(alias = "GICD_IPRIORITYR26")]
pub type GicdIpriorityr26 = crate::Reg<gicd_ipriorityr26::GicdIpriorityr26Spec>;
#[doc = "Interrupt Priority 104 - 107 (Lower is first)"]
pub mod gicd_ipriorityr26;
#[doc = "GICD_IPRIORITYR27 (rw) register accessor: Interrupt Priority 108 - 111 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr27`]
module"]
#[doc(alias = "GICD_IPRIORITYR27")]
pub type GicdIpriorityr27 = crate::Reg<gicd_ipriorityr27::GicdIpriorityr27Spec>;
#[doc = "Interrupt Priority 108 - 111 (Lower is first)"]
pub mod gicd_ipriorityr27;
#[doc = "GICD_IPRIORITYR28 (rw) register accessor: Interrupt Priority 112 - 115 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr28`]
module"]
#[doc(alias = "GICD_IPRIORITYR28")]
pub type GicdIpriorityr28 = crate::Reg<gicd_ipriorityr28::GicdIpriorityr28Spec>;
#[doc = "Interrupt Priority 112 - 115 (Lower is first)"]
pub mod gicd_ipriorityr28;
#[doc = "GICD_IPRIORITYR29 (rw) register accessor: Interrupt Priority 116 - 119 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr29`]
module"]
#[doc(alias = "GICD_IPRIORITYR29")]
pub type GicdIpriorityr29 = crate::Reg<gicd_ipriorityr29::GicdIpriorityr29Spec>;
#[doc = "Interrupt Priority 116 - 119 (Lower is first)"]
pub mod gicd_ipriorityr29;
#[doc = "GICD_IPRIORITYR30 (rw) register accessor: Interrupt Priority 120 - 123 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr30`]
module"]
#[doc(alias = "GICD_IPRIORITYR30")]
pub type GicdIpriorityr30 = crate::Reg<gicd_ipriorityr30::GicdIpriorityr30Spec>;
#[doc = "Interrupt Priority 120 - 123 (Lower is first)"]
pub mod gicd_ipriorityr30;
#[doc = "GICD_IPRIORITYR31 (rw) register accessor: Interrupt Priority 124 - 127 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr31`]
module"]
#[doc(alias = "GICD_IPRIORITYR31")]
pub type GicdIpriorityr31 = crate::Reg<gicd_ipriorityr31::GicdIpriorityr31Spec>;
#[doc = "Interrupt Priority 124 - 127 (Lower is first)"]
pub mod gicd_ipriorityr31;
#[doc = "GICD_IPRIORITYR32 (rw) register accessor: Interrupt Priority 128 - 131 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr32`]
module"]
#[doc(alias = "GICD_IPRIORITYR32")]
pub type GicdIpriorityr32 = crate::Reg<gicd_ipriorityr32::GicdIpriorityr32Spec>;
#[doc = "Interrupt Priority 128 - 131 (Lower is first)"]
pub mod gicd_ipriorityr32;
#[doc = "GICD_IPRIORITYR33 (rw) register accessor: Interrupt Priority 132 - 135 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr33`]
module"]
#[doc(alias = "GICD_IPRIORITYR33")]
pub type GicdIpriorityr33 = crate::Reg<gicd_ipriorityr33::GicdIpriorityr33Spec>;
#[doc = "Interrupt Priority 132 - 135 (Lower is first)"]
pub mod gicd_ipriorityr33;
#[doc = "GICD_IPRIORITYR34 (rw) register accessor: Interrupt Priority 136 - 139 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr34`]
module"]
#[doc(alias = "GICD_IPRIORITYR34")]
pub type GicdIpriorityr34 = crate::Reg<gicd_ipriorityr34::GicdIpriorityr34Spec>;
#[doc = "Interrupt Priority 136 - 139 (Lower is first)"]
pub mod gicd_ipriorityr34;
#[doc = "GICD_IPRIORITYR35 (rw) register accessor: Interrupt Priority 140 - 143 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr35::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr35::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr35`]
module"]
#[doc(alias = "GICD_IPRIORITYR35")]
pub type GicdIpriorityr35 = crate::Reg<gicd_ipriorityr35::GicdIpriorityr35Spec>;
#[doc = "Interrupt Priority 140 - 143 (Lower is first)"]
pub mod gicd_ipriorityr35;
#[doc = "GICD_IPRIORITYR36 (rw) register accessor: Interrupt Priority 144 - 147 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr36::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr36`]
module"]
#[doc(alias = "GICD_IPRIORITYR36")]
pub type GicdIpriorityr36 = crate::Reg<gicd_ipriorityr36::GicdIpriorityr36Spec>;
#[doc = "Interrupt Priority 144 - 147 (Lower is first)"]
pub mod gicd_ipriorityr36;
#[doc = "GICD_IPRIORITYR37 (rw) register accessor: Interrupt Priority 148 - 151 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr37::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr37::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr37`]
module"]
#[doc(alias = "GICD_IPRIORITYR37")]
pub type GicdIpriorityr37 = crate::Reg<gicd_ipriorityr37::GicdIpriorityr37Spec>;
#[doc = "Interrupt Priority 148 - 151 (Lower is first)"]
pub mod gicd_ipriorityr37;
#[doc = "GICD_IPRIORITYR38 (rw) register accessor: Interrupt Priority 152 - 155 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr38`]
module"]
#[doc(alias = "GICD_IPRIORITYR38")]
pub type GicdIpriorityr38 = crate::Reg<gicd_ipriorityr38::GicdIpriorityr38Spec>;
#[doc = "Interrupt Priority 152 - 155 (Lower is first)"]
pub mod gicd_ipriorityr38;
#[doc = "GICD_IPRIORITYR39 (rw) register accessor: Interrupt Priority 156 - 159 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr39::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr39::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr39`]
module"]
#[doc(alias = "GICD_IPRIORITYR39")]
pub type GicdIpriorityr39 = crate::Reg<gicd_ipriorityr39::GicdIpriorityr39Spec>;
#[doc = "Interrupt Priority 156 - 159 (Lower is first)"]
pub mod gicd_ipriorityr39;
#[doc = "GICD_IPRIORITYR40 (rw) register accessor: Interrupt Priority 160 - 163 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr40`]
module"]
#[doc(alias = "GICD_IPRIORITYR40")]
pub type GicdIpriorityr40 = crate::Reg<gicd_ipriorityr40::GicdIpriorityr40Spec>;
#[doc = "Interrupt Priority 160 - 163 (Lower is first)"]
pub mod gicd_ipriorityr40;
#[doc = "GICD_IPRIORITYR41 (rw) register accessor: Interrupt Priority 164 - 167 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr41::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr41::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr41`]
module"]
#[doc(alias = "GICD_IPRIORITYR41")]
pub type GicdIpriorityr41 = crate::Reg<gicd_ipriorityr41::GicdIpriorityr41Spec>;
#[doc = "Interrupt Priority 164 - 167 (Lower is first)"]
pub mod gicd_ipriorityr41;
#[doc = "GICD_IPRIORITYR42 (rw) register accessor: Interrupt Priority 168 - 171 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr42::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr42::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr42`]
module"]
#[doc(alias = "GICD_IPRIORITYR42")]
pub type GicdIpriorityr42 = crate::Reg<gicd_ipriorityr42::GicdIpriorityr42Spec>;
#[doc = "Interrupt Priority 168 - 171 (Lower is first)"]
pub mod gicd_ipriorityr42;
#[doc = "GICD_IPRIORITYR43 (rw) register accessor: Interrupt Priority 172 - 175 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr43::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr43::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr43`]
module"]
#[doc(alias = "GICD_IPRIORITYR43")]
pub type GicdIpriorityr43 = crate::Reg<gicd_ipriorityr43::GicdIpriorityr43Spec>;
#[doc = "Interrupt Priority 172 - 175 (Lower is first)"]
pub mod gicd_ipriorityr43;
#[doc = "GICD_IPRIORITYR44 (rw) register accessor: Interrupt Priority 176 - 179 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr44`]
module"]
#[doc(alias = "GICD_IPRIORITYR44")]
pub type GicdIpriorityr44 = crate::Reg<gicd_ipriorityr44::GicdIpriorityr44Spec>;
#[doc = "Interrupt Priority 176 - 179 (Lower is first)"]
pub mod gicd_ipriorityr44;
#[doc = "GICD_IPRIORITYR45 (rw) register accessor: Interrupt Priority 180 - 183 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr45`]
module"]
#[doc(alias = "GICD_IPRIORITYR45")]
pub type GicdIpriorityr45 = crate::Reg<gicd_ipriorityr45::GicdIpriorityr45Spec>;
#[doc = "Interrupt Priority 180 - 183 (Lower is first)"]
pub mod gicd_ipriorityr45;
#[doc = "GICD_IPRIORITYR46 (rw) register accessor: Interrupt Priority 184 - 187 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr46::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr46::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr46`]
module"]
#[doc(alias = "GICD_IPRIORITYR46")]
pub type GicdIpriorityr46 = crate::Reg<gicd_ipriorityr46::GicdIpriorityr46Spec>;
#[doc = "Interrupt Priority 184 - 187 (Lower is first)"]
pub mod gicd_ipriorityr46;
#[doc = "GICD_IPRIORITYR47 (rw) register accessor: Interrupt Priority 188 - 191 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr47::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr47::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr47`]
module"]
#[doc(alias = "GICD_IPRIORITYR47")]
pub type GicdIpriorityr47 = crate::Reg<gicd_ipriorityr47::GicdIpriorityr47Spec>;
#[doc = "Interrupt Priority 188 - 191 (Lower is first)"]
pub mod gicd_ipriorityr47;
#[doc = "GICD_IPRIORITYR48 (rw) register accessor: Interrupt Priority 192 - 195 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr48`]
module"]
#[doc(alias = "GICD_IPRIORITYR48")]
pub type GicdIpriorityr48 = crate::Reg<gicd_ipriorityr48::GicdIpriorityr48Spec>;
#[doc = "Interrupt Priority 192 - 195 (Lower is first)"]
pub mod gicd_ipriorityr48;
#[doc = "GICD_IPRIORITYR49 (rw) register accessor: Interrupt Priority 196 - 199 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr49::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr49::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr49`]
module"]
#[doc(alias = "GICD_IPRIORITYR49")]
pub type GicdIpriorityr49 = crate::Reg<gicd_ipriorityr49::GicdIpriorityr49Spec>;
#[doc = "Interrupt Priority 196 - 199 (Lower is first)"]
pub mod gicd_ipriorityr49;
#[doc = "GICD_IPRIORITYR50 (rw) register accessor: Interrupt Priority 200 - 203 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr50`]
module"]
#[doc(alias = "GICD_IPRIORITYR50")]
pub type GicdIpriorityr50 = crate::Reg<gicd_ipriorityr50::GicdIpriorityr50Spec>;
#[doc = "Interrupt Priority 200 - 203 (Lower is first)"]
pub mod gicd_ipriorityr50;
#[doc = "GICD_IPRIORITYR51 (rw) register accessor: Interrupt Priority 204 - 207 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr51::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr51::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr51`]
module"]
#[doc(alias = "GICD_IPRIORITYR51")]
pub type GicdIpriorityr51 = crate::Reg<gicd_ipriorityr51::GicdIpriorityr51Spec>;
#[doc = "Interrupt Priority 204 - 207 (Lower is first)"]
pub mod gicd_ipriorityr51;
#[doc = "GICD_IPRIORITYR52 (rw) register accessor: Interrupt Priority 208 - 211 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr52::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr52::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr52`]
module"]
#[doc(alias = "GICD_IPRIORITYR52")]
pub type GicdIpriorityr52 = crate::Reg<gicd_ipriorityr52::GicdIpriorityr52Spec>;
#[doc = "Interrupt Priority 208 - 211 (Lower is first)"]
pub mod gicd_ipriorityr52;
#[doc = "GICD_IPRIORITYR53 (rw) register accessor: Interrupt Priority 212 - 215 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr53::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr53::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr53`]
module"]
#[doc(alias = "GICD_IPRIORITYR53")]
pub type GicdIpriorityr53 = crate::Reg<gicd_ipriorityr53::GicdIpriorityr53Spec>;
#[doc = "Interrupt Priority 212 - 215 (Lower is first)"]
pub mod gicd_ipriorityr53;
#[doc = "GICD_IPRIORITYR54 (rw) register accessor: Interrupt Priority 216 - 219 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr54`]
module"]
#[doc(alias = "GICD_IPRIORITYR54")]
pub type GicdIpriorityr54 = crate::Reg<gicd_ipriorityr54::GicdIpriorityr54Spec>;
#[doc = "Interrupt Priority 216 - 219 (Lower is first)"]
pub mod gicd_ipriorityr54;
#[doc = "GICD_IPRIORITYR55 (rw) register accessor: Interrupt Priority 220 - 223 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr55::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr55::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr55`]
module"]
#[doc(alias = "GICD_IPRIORITYR55")]
pub type GicdIpriorityr55 = crate::Reg<gicd_ipriorityr55::GicdIpriorityr55Spec>;
#[doc = "Interrupt Priority 220 - 223 (Lower is first)"]
pub mod gicd_ipriorityr55;

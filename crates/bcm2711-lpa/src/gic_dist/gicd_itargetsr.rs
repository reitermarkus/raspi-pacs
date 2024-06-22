#[repr(C)]
#[derive(Debug)]
#[doc = "Interrupt Processor Targets"]
#[doc(alias = "GICD_ITARGETSR")]
pub struct GicdItargetsr {
    gicd_itargetsr0: GicdItargetsr0,
    gicd_itargetsr1: GicdItargetsr1,
    gicd_itargetsr2: GicdItargetsr2,
    gicd_itargetsr3: GicdItargetsr3,
    gicd_itargetsr4: GicdItargetsr4,
    gicd_itargetsr5: GicdItargetsr5,
    gicd_itargetsr6: GicdItargetsr6,
    gicd_itargetsr7: GicdItargetsr7,
    gicd_itargetsr8: GicdItargetsr8,
    gicd_itargetsr9: GicdItargetsr9,
    gicd_itargetsr10: GicdItargetsr10,
    gicd_itargetsr11: GicdItargetsr11,
    gicd_itargetsr12: GicdItargetsr12,
    gicd_itargetsr13: GicdItargetsr13,
    gicd_itargetsr14: GicdItargetsr14,
    gicd_itargetsr15: GicdItargetsr15,
    gicd_itargetsr16: GicdItargetsr16,
    gicd_itargetsr17: GicdItargetsr17,
    gicd_itargetsr18: GicdItargetsr18,
    gicd_itargetsr19: GicdItargetsr19,
    gicd_itargetsr20: GicdItargetsr20,
    gicd_itargetsr21: GicdItargetsr21,
    gicd_itargetsr22: GicdItargetsr22,
    gicd_itargetsr23: GicdItargetsr23,
    gicd_itargetsr24: GicdItargetsr24,
    gicd_itargetsr25: GicdItargetsr25,
    gicd_itargetsr26: GicdItargetsr26,
    gicd_itargetsr27: GicdItargetsr27,
    gicd_itargetsr28: GicdItargetsr28,
    gicd_itargetsr29: GicdItargetsr29,
    gicd_itargetsr30: GicdItargetsr30,
    gicd_itargetsr31: GicdItargetsr31,
    gicd_itargetsr32: GicdItargetsr32,
    gicd_itargetsr33: GicdItargetsr33,
    gicd_itargetsr34: GicdItargetsr34,
    gicd_itargetsr35: GicdItargetsr35,
    gicd_itargetsr36: GicdItargetsr36,
    gicd_itargetsr37: GicdItargetsr37,
    gicd_itargetsr38: GicdItargetsr38,
    gicd_itargetsr39: GicdItargetsr39,
    gicd_itargetsr40: GicdItargetsr40,
    gicd_itargetsr41: GicdItargetsr41,
    gicd_itargetsr42: GicdItargetsr42,
    gicd_itargetsr43: GicdItargetsr43,
    gicd_itargetsr44: GicdItargetsr44,
    gicd_itargetsr45: GicdItargetsr45,
    gicd_itargetsr46: GicdItargetsr46,
    gicd_itargetsr47: GicdItargetsr47,
    gicd_itargetsr48: GicdItargetsr48,
    gicd_itargetsr49: GicdItargetsr49,
    gicd_itargetsr50: GicdItargetsr50,
    gicd_itargetsr51: GicdItargetsr51,
    gicd_itargetsr52: GicdItargetsr52,
    gicd_itargetsr53: GicdItargetsr53,
    gicd_itargetsr54: GicdItargetsr54,
    gicd_itargetsr55: GicdItargetsr55,
}
impl GicdItargetsr {
    #[doc = "0x00 - Interrupt Processor Target 0 - 3"]
    #[inline(always)]
    pub const fn gicd_itargetsr0(&self) -> &GicdItargetsr0 {
        &self.gicd_itargetsr0
    }
    #[doc = "0x04 - Interrupt Processor Target 4 - 7"]
    #[inline(always)]
    pub const fn gicd_itargetsr1(&self) -> &GicdItargetsr1 {
        &self.gicd_itargetsr1
    }
    #[doc = "0x08 - Interrupt Processor Target 8 - 11"]
    #[inline(always)]
    pub const fn gicd_itargetsr2(&self) -> &GicdItargetsr2 {
        &self.gicd_itargetsr2
    }
    #[doc = "0x0c - Interrupt Processor Target 12 - 15"]
    #[inline(always)]
    pub const fn gicd_itargetsr3(&self) -> &GicdItargetsr3 {
        &self.gicd_itargetsr3
    }
    #[doc = "0x10 - Interrupt Processor Target 16 - 19"]
    #[inline(always)]
    pub const fn gicd_itargetsr4(&self) -> &GicdItargetsr4 {
        &self.gicd_itargetsr4
    }
    #[doc = "0x14 - Interrupt Processor Target 20 - 23"]
    #[inline(always)]
    pub const fn gicd_itargetsr5(&self) -> &GicdItargetsr5 {
        &self.gicd_itargetsr5
    }
    #[doc = "0x18 - Interrupt Processor Target 24 - 27"]
    #[inline(always)]
    pub const fn gicd_itargetsr6(&self) -> &GicdItargetsr6 {
        &self.gicd_itargetsr6
    }
    #[doc = "0x1c - Interrupt Processor Target 28 - 31"]
    #[inline(always)]
    pub const fn gicd_itargetsr7(&self) -> &GicdItargetsr7 {
        &self.gicd_itargetsr7
    }
    #[doc = "0x20 - Interrupt Processor Target 32 - 35"]
    #[inline(always)]
    pub const fn gicd_itargetsr8(&self) -> &GicdItargetsr8 {
        &self.gicd_itargetsr8
    }
    #[doc = "0x24 - Interrupt Processor Target 36 - 39"]
    #[inline(always)]
    pub const fn gicd_itargetsr9(&self) -> &GicdItargetsr9 {
        &self.gicd_itargetsr9
    }
    #[doc = "0x28 - Interrupt Processor Target 40 - 43"]
    #[inline(always)]
    pub const fn gicd_itargetsr10(&self) -> &GicdItargetsr10 {
        &self.gicd_itargetsr10
    }
    #[doc = "0x2c - Interrupt Processor Target 44 - 47"]
    #[inline(always)]
    pub const fn gicd_itargetsr11(&self) -> &GicdItargetsr11 {
        &self.gicd_itargetsr11
    }
    #[doc = "0x30 - Interrupt Processor Target 48 - 51"]
    #[inline(always)]
    pub const fn gicd_itargetsr12(&self) -> &GicdItargetsr12 {
        &self.gicd_itargetsr12
    }
    #[doc = "0x34 - Interrupt Processor Target 52 - 55"]
    #[inline(always)]
    pub const fn gicd_itargetsr13(&self) -> &GicdItargetsr13 {
        &self.gicd_itargetsr13
    }
    #[doc = "0x38 - Interrupt Processor Target 56 - 59"]
    #[inline(always)]
    pub const fn gicd_itargetsr14(&self) -> &GicdItargetsr14 {
        &self.gicd_itargetsr14
    }
    #[doc = "0x3c - Interrupt Processor Target 60 - 63"]
    #[inline(always)]
    pub const fn gicd_itargetsr15(&self) -> &GicdItargetsr15 {
        &self.gicd_itargetsr15
    }
    #[doc = "0x40 - Interrupt Processor Target 64 - 67"]
    #[inline(always)]
    pub const fn gicd_itargetsr16(&self) -> &GicdItargetsr16 {
        &self.gicd_itargetsr16
    }
    #[doc = "0x44 - Interrupt Processor Target 68 - 71"]
    #[inline(always)]
    pub const fn gicd_itargetsr17(&self) -> &GicdItargetsr17 {
        &self.gicd_itargetsr17
    }
    #[doc = "0x48 - Interrupt Processor Target 72 - 75"]
    #[inline(always)]
    pub const fn gicd_itargetsr18(&self) -> &GicdItargetsr18 {
        &self.gicd_itargetsr18
    }
    #[doc = "0x4c - Interrupt Processor Target 76 - 79"]
    #[inline(always)]
    pub const fn gicd_itargetsr19(&self) -> &GicdItargetsr19 {
        &self.gicd_itargetsr19
    }
    #[doc = "0x50 - Interrupt Processor Target 80 - 83"]
    #[inline(always)]
    pub const fn gicd_itargetsr20(&self) -> &GicdItargetsr20 {
        &self.gicd_itargetsr20
    }
    #[doc = "0x54 - Interrupt Processor Target 84 - 87"]
    #[inline(always)]
    pub const fn gicd_itargetsr21(&self) -> &GicdItargetsr21 {
        &self.gicd_itargetsr21
    }
    #[doc = "0x58 - Interrupt Processor Target 88 - 91"]
    #[inline(always)]
    pub const fn gicd_itargetsr22(&self) -> &GicdItargetsr22 {
        &self.gicd_itargetsr22
    }
    #[doc = "0x5c - Interrupt Processor Target 92 - 95"]
    #[inline(always)]
    pub const fn gicd_itargetsr23(&self) -> &GicdItargetsr23 {
        &self.gicd_itargetsr23
    }
    #[doc = "0x60 - Interrupt Processor Target 96 - 99"]
    #[inline(always)]
    pub const fn gicd_itargetsr24(&self) -> &GicdItargetsr24 {
        &self.gicd_itargetsr24
    }
    #[doc = "0x64 - Interrupt Processor Target 100 - 103"]
    #[inline(always)]
    pub const fn gicd_itargetsr25(&self) -> &GicdItargetsr25 {
        &self.gicd_itargetsr25
    }
    #[doc = "0x68 - Interrupt Processor Target 104 - 107"]
    #[inline(always)]
    pub const fn gicd_itargetsr26(&self) -> &GicdItargetsr26 {
        &self.gicd_itargetsr26
    }
    #[doc = "0x6c - Interrupt Processor Target 108 - 111"]
    #[inline(always)]
    pub const fn gicd_itargetsr27(&self) -> &GicdItargetsr27 {
        &self.gicd_itargetsr27
    }
    #[doc = "0x70 - Interrupt Processor Target 112 - 115"]
    #[inline(always)]
    pub const fn gicd_itargetsr28(&self) -> &GicdItargetsr28 {
        &self.gicd_itargetsr28
    }
    #[doc = "0x74 - Interrupt Processor Target 116 - 119"]
    #[inline(always)]
    pub const fn gicd_itargetsr29(&self) -> &GicdItargetsr29 {
        &self.gicd_itargetsr29
    }
    #[doc = "0x78 - Interrupt Processor Target 120 - 123"]
    #[inline(always)]
    pub const fn gicd_itargetsr30(&self) -> &GicdItargetsr30 {
        &self.gicd_itargetsr30
    }
    #[doc = "0x7c - Interrupt Processor Target 124 - 127"]
    #[inline(always)]
    pub const fn gicd_itargetsr31(&self) -> &GicdItargetsr31 {
        &self.gicd_itargetsr31
    }
    #[doc = "0x80 - Interrupt Processor Target 128 - 131"]
    #[inline(always)]
    pub const fn gicd_itargetsr32(&self) -> &GicdItargetsr32 {
        &self.gicd_itargetsr32
    }
    #[doc = "0x84 - Interrupt Processor Target 132 - 135"]
    #[inline(always)]
    pub const fn gicd_itargetsr33(&self) -> &GicdItargetsr33 {
        &self.gicd_itargetsr33
    }
    #[doc = "0x88 - Interrupt Processor Target 136 - 139"]
    #[inline(always)]
    pub const fn gicd_itargetsr34(&self) -> &GicdItargetsr34 {
        &self.gicd_itargetsr34
    }
    #[doc = "0x8c - Interrupt Processor Target 140 - 143"]
    #[inline(always)]
    pub const fn gicd_itargetsr35(&self) -> &GicdItargetsr35 {
        &self.gicd_itargetsr35
    }
    #[doc = "0x90 - Interrupt Processor Target 144 - 147"]
    #[inline(always)]
    pub const fn gicd_itargetsr36(&self) -> &GicdItargetsr36 {
        &self.gicd_itargetsr36
    }
    #[doc = "0x94 - Interrupt Processor Target 148 - 151"]
    #[inline(always)]
    pub const fn gicd_itargetsr37(&self) -> &GicdItargetsr37 {
        &self.gicd_itargetsr37
    }
    #[doc = "0x98 - Interrupt Processor Target 152 - 155"]
    #[inline(always)]
    pub const fn gicd_itargetsr38(&self) -> &GicdItargetsr38 {
        &self.gicd_itargetsr38
    }
    #[doc = "0x9c - Interrupt Processor Target 156 - 159"]
    #[inline(always)]
    pub const fn gicd_itargetsr39(&self) -> &GicdItargetsr39 {
        &self.gicd_itargetsr39
    }
    #[doc = "0xa0 - Interrupt Processor Target 160 - 163"]
    #[inline(always)]
    pub const fn gicd_itargetsr40(&self) -> &GicdItargetsr40 {
        &self.gicd_itargetsr40
    }
    #[doc = "0xa4 - Interrupt Processor Target 164 - 167"]
    #[inline(always)]
    pub const fn gicd_itargetsr41(&self) -> &GicdItargetsr41 {
        &self.gicd_itargetsr41
    }
    #[doc = "0xa8 - Interrupt Processor Target 168 - 171"]
    #[inline(always)]
    pub const fn gicd_itargetsr42(&self) -> &GicdItargetsr42 {
        &self.gicd_itargetsr42
    }
    #[doc = "0xac - Interrupt Processor Target 172 - 175"]
    #[inline(always)]
    pub const fn gicd_itargetsr43(&self) -> &GicdItargetsr43 {
        &self.gicd_itargetsr43
    }
    #[doc = "0xb0 - Interrupt Processor Target 176 - 179"]
    #[inline(always)]
    pub const fn gicd_itargetsr44(&self) -> &GicdItargetsr44 {
        &self.gicd_itargetsr44
    }
    #[doc = "0xb4 - Interrupt Processor Target 180 - 183"]
    #[inline(always)]
    pub const fn gicd_itargetsr45(&self) -> &GicdItargetsr45 {
        &self.gicd_itargetsr45
    }
    #[doc = "0xb8 - Interrupt Processor Target 184 - 187"]
    #[inline(always)]
    pub const fn gicd_itargetsr46(&self) -> &GicdItargetsr46 {
        &self.gicd_itargetsr46
    }
    #[doc = "0xbc - Interrupt Processor Target 188 - 191"]
    #[inline(always)]
    pub const fn gicd_itargetsr47(&self) -> &GicdItargetsr47 {
        &self.gicd_itargetsr47
    }
    #[doc = "0xc0 - Interrupt Processor Target 192 - 195"]
    #[inline(always)]
    pub const fn gicd_itargetsr48(&self) -> &GicdItargetsr48 {
        &self.gicd_itargetsr48
    }
    #[doc = "0xc4 - Interrupt Processor Target 196 - 199"]
    #[inline(always)]
    pub const fn gicd_itargetsr49(&self) -> &GicdItargetsr49 {
        &self.gicd_itargetsr49
    }
    #[doc = "0xc8 - Interrupt Processor Target 200 - 203"]
    #[inline(always)]
    pub const fn gicd_itargetsr50(&self) -> &GicdItargetsr50 {
        &self.gicd_itargetsr50
    }
    #[doc = "0xcc - Interrupt Processor Target 204 - 207"]
    #[inline(always)]
    pub const fn gicd_itargetsr51(&self) -> &GicdItargetsr51 {
        &self.gicd_itargetsr51
    }
    #[doc = "0xd0 - Interrupt Processor Target 208 - 211"]
    #[inline(always)]
    pub const fn gicd_itargetsr52(&self) -> &GicdItargetsr52 {
        &self.gicd_itargetsr52
    }
    #[doc = "0xd4 - Interrupt Processor Target 212 - 215"]
    #[inline(always)]
    pub const fn gicd_itargetsr53(&self) -> &GicdItargetsr53 {
        &self.gicd_itargetsr53
    }
    #[doc = "0xd8 - Interrupt Processor Target 216 - 219"]
    #[inline(always)]
    pub const fn gicd_itargetsr54(&self) -> &GicdItargetsr54 {
        &self.gicd_itargetsr54
    }
    #[doc = "0xdc - Interrupt Processor Target 220 - 223"]
    #[inline(always)]
    pub const fn gicd_itargetsr55(&self) -> &GicdItargetsr55 {
        &self.gicd_itargetsr55
    }
}
#[doc = "GICD_ITARGETSR0 (rw) register accessor: Interrupt Processor Target 0 - 3\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr0`]
module"]
#[doc(alias = "GICD_ITARGETSR0")]
pub type GicdItargetsr0 = crate::Reg<gicd_itargetsr0::GicdItargetsr0Spec>;
#[doc = "Interrupt Processor Target 0 - 3"]
pub mod gicd_itargetsr0;
#[doc = "GICD_ITARGETSR1 (rw) register accessor: Interrupt Processor Target 4 - 7\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr1`]
module"]
#[doc(alias = "GICD_ITARGETSR1")]
pub type GicdItargetsr1 = crate::Reg<gicd_itargetsr1::GicdItargetsr1Spec>;
#[doc = "Interrupt Processor Target 4 - 7"]
pub mod gicd_itargetsr1;
#[doc = "GICD_ITARGETSR2 (rw) register accessor: Interrupt Processor Target 8 - 11\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr2`]
module"]
#[doc(alias = "GICD_ITARGETSR2")]
pub type GicdItargetsr2 = crate::Reg<gicd_itargetsr2::GicdItargetsr2Spec>;
#[doc = "Interrupt Processor Target 8 - 11"]
pub mod gicd_itargetsr2;
#[doc = "GICD_ITARGETSR3 (rw) register accessor: Interrupt Processor Target 12 - 15\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr3`]
module"]
#[doc(alias = "GICD_ITARGETSR3")]
pub type GicdItargetsr3 = crate::Reg<gicd_itargetsr3::GicdItargetsr3Spec>;
#[doc = "Interrupt Processor Target 12 - 15"]
pub mod gicd_itargetsr3;
#[doc = "GICD_ITARGETSR4 (rw) register accessor: Interrupt Processor Target 16 - 19\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr4`]
module"]
#[doc(alias = "GICD_ITARGETSR4")]
pub type GicdItargetsr4 = crate::Reg<gicd_itargetsr4::GicdItargetsr4Spec>;
#[doc = "Interrupt Processor Target 16 - 19"]
pub mod gicd_itargetsr4;
#[doc = "GICD_ITARGETSR5 (rw) register accessor: Interrupt Processor Target 20 - 23\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr5`]
module"]
#[doc(alias = "GICD_ITARGETSR5")]
pub type GicdItargetsr5 = crate::Reg<gicd_itargetsr5::GicdItargetsr5Spec>;
#[doc = "Interrupt Processor Target 20 - 23"]
pub mod gicd_itargetsr5;
#[doc = "GICD_ITARGETSR6 (rw) register accessor: Interrupt Processor Target 24 - 27\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr6`]
module"]
#[doc(alias = "GICD_ITARGETSR6")]
pub type GicdItargetsr6 = crate::Reg<gicd_itargetsr6::GicdItargetsr6Spec>;
#[doc = "Interrupt Processor Target 24 - 27"]
pub mod gicd_itargetsr6;
#[doc = "GICD_ITARGETSR7 (rw) register accessor: Interrupt Processor Target 28 - 31\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr7`]
module"]
#[doc(alias = "GICD_ITARGETSR7")]
pub type GicdItargetsr7 = crate::Reg<gicd_itargetsr7::GicdItargetsr7Spec>;
#[doc = "Interrupt Processor Target 28 - 31"]
pub mod gicd_itargetsr7;
#[doc = "GICD_ITARGETSR8 (rw) register accessor: Interrupt Processor Target 32 - 35\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr8`]
module"]
#[doc(alias = "GICD_ITARGETSR8")]
pub type GicdItargetsr8 = crate::Reg<gicd_itargetsr8::GicdItargetsr8Spec>;
#[doc = "Interrupt Processor Target 32 - 35"]
pub mod gicd_itargetsr8;
#[doc = "GICD_ITARGETSR9 (rw) register accessor: Interrupt Processor Target 36 - 39\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr9`]
module"]
#[doc(alias = "GICD_ITARGETSR9")]
pub type GicdItargetsr9 = crate::Reg<gicd_itargetsr9::GicdItargetsr9Spec>;
#[doc = "Interrupt Processor Target 36 - 39"]
pub mod gicd_itargetsr9;
#[doc = "GICD_ITARGETSR10 (rw) register accessor: Interrupt Processor Target 40 - 43\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr10`]
module"]
#[doc(alias = "GICD_ITARGETSR10")]
pub type GicdItargetsr10 = crate::Reg<gicd_itargetsr10::GicdItargetsr10Spec>;
#[doc = "Interrupt Processor Target 40 - 43"]
pub mod gicd_itargetsr10;
#[doc = "GICD_ITARGETSR11 (rw) register accessor: Interrupt Processor Target 44 - 47\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr11`]
module"]
#[doc(alias = "GICD_ITARGETSR11")]
pub type GicdItargetsr11 = crate::Reg<gicd_itargetsr11::GicdItargetsr11Spec>;
#[doc = "Interrupt Processor Target 44 - 47"]
pub mod gicd_itargetsr11;
#[doc = "GICD_ITARGETSR12 (rw) register accessor: Interrupt Processor Target 48 - 51\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr12`]
module"]
#[doc(alias = "GICD_ITARGETSR12")]
pub type GicdItargetsr12 = crate::Reg<gicd_itargetsr12::GicdItargetsr12Spec>;
#[doc = "Interrupt Processor Target 48 - 51"]
pub mod gicd_itargetsr12;
#[doc = "GICD_ITARGETSR13 (rw) register accessor: Interrupt Processor Target 52 - 55\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr13`]
module"]
#[doc(alias = "GICD_ITARGETSR13")]
pub type GicdItargetsr13 = crate::Reg<gicd_itargetsr13::GicdItargetsr13Spec>;
#[doc = "Interrupt Processor Target 52 - 55"]
pub mod gicd_itargetsr13;
#[doc = "GICD_ITARGETSR14 (rw) register accessor: Interrupt Processor Target 56 - 59\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr14`]
module"]
#[doc(alias = "GICD_ITARGETSR14")]
pub type GicdItargetsr14 = crate::Reg<gicd_itargetsr14::GicdItargetsr14Spec>;
#[doc = "Interrupt Processor Target 56 - 59"]
pub mod gicd_itargetsr14;
#[doc = "GICD_ITARGETSR15 (rw) register accessor: Interrupt Processor Target 60 - 63\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr15`]
module"]
#[doc(alias = "GICD_ITARGETSR15")]
pub type GicdItargetsr15 = crate::Reg<gicd_itargetsr15::GicdItargetsr15Spec>;
#[doc = "Interrupt Processor Target 60 - 63"]
pub mod gicd_itargetsr15;
#[doc = "GICD_ITARGETSR16 (rw) register accessor: Interrupt Processor Target 64 - 67\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr16`]
module"]
#[doc(alias = "GICD_ITARGETSR16")]
pub type GicdItargetsr16 = crate::Reg<gicd_itargetsr16::GicdItargetsr16Spec>;
#[doc = "Interrupt Processor Target 64 - 67"]
pub mod gicd_itargetsr16;
#[doc = "GICD_ITARGETSR17 (rw) register accessor: Interrupt Processor Target 68 - 71\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr17`]
module"]
#[doc(alias = "GICD_ITARGETSR17")]
pub type GicdItargetsr17 = crate::Reg<gicd_itargetsr17::GicdItargetsr17Spec>;
#[doc = "Interrupt Processor Target 68 - 71"]
pub mod gicd_itargetsr17;
#[doc = "GICD_ITARGETSR18 (rw) register accessor: Interrupt Processor Target 72 - 75\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr18`]
module"]
#[doc(alias = "GICD_ITARGETSR18")]
pub type GicdItargetsr18 = crate::Reg<gicd_itargetsr18::GicdItargetsr18Spec>;
#[doc = "Interrupt Processor Target 72 - 75"]
pub mod gicd_itargetsr18;
#[doc = "GICD_ITARGETSR19 (rw) register accessor: Interrupt Processor Target 76 - 79\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr19`]
module"]
#[doc(alias = "GICD_ITARGETSR19")]
pub type GicdItargetsr19 = crate::Reg<gicd_itargetsr19::GicdItargetsr19Spec>;
#[doc = "Interrupt Processor Target 76 - 79"]
pub mod gicd_itargetsr19;
#[doc = "GICD_ITARGETSR20 (rw) register accessor: Interrupt Processor Target 80 - 83\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr20`]
module"]
#[doc(alias = "GICD_ITARGETSR20")]
pub type GicdItargetsr20 = crate::Reg<gicd_itargetsr20::GicdItargetsr20Spec>;
#[doc = "Interrupt Processor Target 80 - 83"]
pub mod gicd_itargetsr20;
#[doc = "GICD_ITARGETSR21 (rw) register accessor: Interrupt Processor Target 84 - 87\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr21`]
module"]
#[doc(alias = "GICD_ITARGETSR21")]
pub type GicdItargetsr21 = crate::Reg<gicd_itargetsr21::GicdItargetsr21Spec>;
#[doc = "Interrupt Processor Target 84 - 87"]
pub mod gicd_itargetsr21;
#[doc = "GICD_ITARGETSR22 (rw) register accessor: Interrupt Processor Target 88 - 91\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr22`]
module"]
#[doc(alias = "GICD_ITARGETSR22")]
pub type GicdItargetsr22 = crate::Reg<gicd_itargetsr22::GicdItargetsr22Spec>;
#[doc = "Interrupt Processor Target 88 - 91"]
pub mod gicd_itargetsr22;
#[doc = "GICD_ITARGETSR23 (rw) register accessor: Interrupt Processor Target 92 - 95\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr23`]
module"]
#[doc(alias = "GICD_ITARGETSR23")]
pub type GicdItargetsr23 = crate::Reg<gicd_itargetsr23::GicdItargetsr23Spec>;
#[doc = "Interrupt Processor Target 92 - 95"]
pub mod gicd_itargetsr23;
#[doc = "GICD_ITARGETSR24 (rw) register accessor: Interrupt Processor Target 96 - 99\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr24`]
module"]
#[doc(alias = "GICD_ITARGETSR24")]
pub type GicdItargetsr24 = crate::Reg<gicd_itargetsr24::GicdItargetsr24Spec>;
#[doc = "Interrupt Processor Target 96 - 99"]
pub mod gicd_itargetsr24;
#[doc = "GICD_ITARGETSR25 (rw) register accessor: Interrupt Processor Target 100 - 103\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr25`]
module"]
#[doc(alias = "GICD_ITARGETSR25")]
pub type GicdItargetsr25 = crate::Reg<gicd_itargetsr25::GicdItargetsr25Spec>;
#[doc = "Interrupt Processor Target 100 - 103"]
pub mod gicd_itargetsr25;
#[doc = "GICD_ITARGETSR26 (rw) register accessor: Interrupt Processor Target 104 - 107\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr26`]
module"]
#[doc(alias = "GICD_ITARGETSR26")]
pub type GicdItargetsr26 = crate::Reg<gicd_itargetsr26::GicdItargetsr26Spec>;
#[doc = "Interrupt Processor Target 104 - 107"]
pub mod gicd_itargetsr26;
#[doc = "GICD_ITARGETSR27 (rw) register accessor: Interrupt Processor Target 108 - 111\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr27`]
module"]
#[doc(alias = "GICD_ITARGETSR27")]
pub type GicdItargetsr27 = crate::Reg<gicd_itargetsr27::GicdItargetsr27Spec>;
#[doc = "Interrupt Processor Target 108 - 111"]
pub mod gicd_itargetsr27;
#[doc = "GICD_ITARGETSR28 (rw) register accessor: Interrupt Processor Target 112 - 115\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr28`]
module"]
#[doc(alias = "GICD_ITARGETSR28")]
pub type GicdItargetsr28 = crate::Reg<gicd_itargetsr28::GicdItargetsr28Spec>;
#[doc = "Interrupt Processor Target 112 - 115"]
pub mod gicd_itargetsr28;
#[doc = "GICD_ITARGETSR29 (rw) register accessor: Interrupt Processor Target 116 - 119\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr29`]
module"]
#[doc(alias = "GICD_ITARGETSR29")]
pub type GicdItargetsr29 = crate::Reg<gicd_itargetsr29::GicdItargetsr29Spec>;
#[doc = "Interrupt Processor Target 116 - 119"]
pub mod gicd_itargetsr29;
#[doc = "GICD_ITARGETSR30 (rw) register accessor: Interrupt Processor Target 120 - 123\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr30`]
module"]
#[doc(alias = "GICD_ITARGETSR30")]
pub type GicdItargetsr30 = crate::Reg<gicd_itargetsr30::GicdItargetsr30Spec>;
#[doc = "Interrupt Processor Target 120 - 123"]
pub mod gicd_itargetsr30;
#[doc = "GICD_ITARGETSR31 (rw) register accessor: Interrupt Processor Target 124 - 127\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr31`]
module"]
#[doc(alias = "GICD_ITARGETSR31")]
pub type GicdItargetsr31 = crate::Reg<gicd_itargetsr31::GicdItargetsr31Spec>;
#[doc = "Interrupt Processor Target 124 - 127"]
pub mod gicd_itargetsr31;
#[doc = "GICD_ITARGETSR32 (rw) register accessor: Interrupt Processor Target 128 - 131\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr32`]
module"]
#[doc(alias = "GICD_ITARGETSR32")]
pub type GicdItargetsr32 = crate::Reg<gicd_itargetsr32::GicdItargetsr32Spec>;
#[doc = "Interrupt Processor Target 128 - 131"]
pub mod gicd_itargetsr32;
#[doc = "GICD_ITARGETSR33 (rw) register accessor: Interrupt Processor Target 132 - 135\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr33`]
module"]
#[doc(alias = "GICD_ITARGETSR33")]
pub type GicdItargetsr33 = crate::Reg<gicd_itargetsr33::GicdItargetsr33Spec>;
#[doc = "Interrupt Processor Target 132 - 135"]
pub mod gicd_itargetsr33;
#[doc = "GICD_ITARGETSR34 (rw) register accessor: Interrupt Processor Target 136 - 139\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr34`]
module"]
#[doc(alias = "GICD_ITARGETSR34")]
pub type GicdItargetsr34 = crate::Reg<gicd_itargetsr34::GicdItargetsr34Spec>;
#[doc = "Interrupt Processor Target 136 - 139"]
pub mod gicd_itargetsr34;
#[doc = "GICD_ITARGETSR35 (rw) register accessor: Interrupt Processor Target 140 - 143\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr35::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr35::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr35`]
module"]
#[doc(alias = "GICD_ITARGETSR35")]
pub type GicdItargetsr35 = crate::Reg<gicd_itargetsr35::GicdItargetsr35Spec>;
#[doc = "Interrupt Processor Target 140 - 143"]
pub mod gicd_itargetsr35;
#[doc = "GICD_ITARGETSR36 (rw) register accessor: Interrupt Processor Target 144 - 147\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr36::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr36`]
module"]
#[doc(alias = "GICD_ITARGETSR36")]
pub type GicdItargetsr36 = crate::Reg<gicd_itargetsr36::GicdItargetsr36Spec>;
#[doc = "Interrupt Processor Target 144 - 147"]
pub mod gicd_itargetsr36;
#[doc = "GICD_ITARGETSR37 (rw) register accessor: Interrupt Processor Target 148 - 151\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr37::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr37::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr37`]
module"]
#[doc(alias = "GICD_ITARGETSR37")]
pub type GicdItargetsr37 = crate::Reg<gicd_itargetsr37::GicdItargetsr37Spec>;
#[doc = "Interrupt Processor Target 148 - 151"]
pub mod gicd_itargetsr37;
#[doc = "GICD_ITARGETSR38 (rw) register accessor: Interrupt Processor Target 152 - 155\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr38`]
module"]
#[doc(alias = "GICD_ITARGETSR38")]
pub type GicdItargetsr38 = crate::Reg<gicd_itargetsr38::GicdItargetsr38Spec>;
#[doc = "Interrupt Processor Target 152 - 155"]
pub mod gicd_itargetsr38;
#[doc = "GICD_ITARGETSR39 (rw) register accessor: Interrupt Processor Target 156 - 159\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr39::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr39::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr39`]
module"]
#[doc(alias = "GICD_ITARGETSR39")]
pub type GicdItargetsr39 = crate::Reg<gicd_itargetsr39::GicdItargetsr39Spec>;
#[doc = "Interrupt Processor Target 156 - 159"]
pub mod gicd_itargetsr39;
#[doc = "GICD_ITARGETSR40 (rw) register accessor: Interrupt Processor Target 160 - 163\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr40`]
module"]
#[doc(alias = "GICD_ITARGETSR40")]
pub type GicdItargetsr40 = crate::Reg<gicd_itargetsr40::GicdItargetsr40Spec>;
#[doc = "Interrupt Processor Target 160 - 163"]
pub mod gicd_itargetsr40;
#[doc = "GICD_ITARGETSR41 (rw) register accessor: Interrupt Processor Target 164 - 167\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr41::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr41::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr41`]
module"]
#[doc(alias = "GICD_ITARGETSR41")]
pub type GicdItargetsr41 = crate::Reg<gicd_itargetsr41::GicdItargetsr41Spec>;
#[doc = "Interrupt Processor Target 164 - 167"]
pub mod gicd_itargetsr41;
#[doc = "GICD_ITARGETSR42 (rw) register accessor: Interrupt Processor Target 168 - 171\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr42::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr42::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr42`]
module"]
#[doc(alias = "GICD_ITARGETSR42")]
pub type GicdItargetsr42 = crate::Reg<gicd_itargetsr42::GicdItargetsr42Spec>;
#[doc = "Interrupt Processor Target 168 - 171"]
pub mod gicd_itargetsr42;
#[doc = "GICD_ITARGETSR43 (rw) register accessor: Interrupt Processor Target 172 - 175\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr43::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr43::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr43`]
module"]
#[doc(alias = "GICD_ITARGETSR43")]
pub type GicdItargetsr43 = crate::Reg<gicd_itargetsr43::GicdItargetsr43Spec>;
#[doc = "Interrupt Processor Target 172 - 175"]
pub mod gicd_itargetsr43;
#[doc = "GICD_ITARGETSR44 (rw) register accessor: Interrupt Processor Target 176 - 179\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr44`]
module"]
#[doc(alias = "GICD_ITARGETSR44")]
pub type GicdItargetsr44 = crate::Reg<gicd_itargetsr44::GicdItargetsr44Spec>;
#[doc = "Interrupt Processor Target 176 - 179"]
pub mod gicd_itargetsr44;
#[doc = "GICD_ITARGETSR45 (rw) register accessor: Interrupt Processor Target 180 - 183\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr45`]
module"]
#[doc(alias = "GICD_ITARGETSR45")]
pub type GicdItargetsr45 = crate::Reg<gicd_itargetsr45::GicdItargetsr45Spec>;
#[doc = "Interrupt Processor Target 180 - 183"]
pub mod gicd_itargetsr45;
#[doc = "GICD_ITARGETSR46 (rw) register accessor: Interrupt Processor Target 184 - 187\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr46::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr46::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr46`]
module"]
#[doc(alias = "GICD_ITARGETSR46")]
pub type GicdItargetsr46 = crate::Reg<gicd_itargetsr46::GicdItargetsr46Spec>;
#[doc = "Interrupt Processor Target 184 - 187"]
pub mod gicd_itargetsr46;
#[doc = "GICD_ITARGETSR47 (rw) register accessor: Interrupt Processor Target 188 - 191\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr47::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr47::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr47`]
module"]
#[doc(alias = "GICD_ITARGETSR47")]
pub type GicdItargetsr47 = crate::Reg<gicd_itargetsr47::GicdItargetsr47Spec>;
#[doc = "Interrupt Processor Target 188 - 191"]
pub mod gicd_itargetsr47;
#[doc = "GICD_ITARGETSR48 (rw) register accessor: Interrupt Processor Target 192 - 195\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr48`]
module"]
#[doc(alias = "GICD_ITARGETSR48")]
pub type GicdItargetsr48 = crate::Reg<gicd_itargetsr48::GicdItargetsr48Spec>;
#[doc = "Interrupt Processor Target 192 - 195"]
pub mod gicd_itargetsr48;
#[doc = "GICD_ITARGETSR49 (rw) register accessor: Interrupt Processor Target 196 - 199\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr49::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr49::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr49`]
module"]
#[doc(alias = "GICD_ITARGETSR49")]
pub type GicdItargetsr49 = crate::Reg<gicd_itargetsr49::GicdItargetsr49Spec>;
#[doc = "Interrupt Processor Target 196 - 199"]
pub mod gicd_itargetsr49;
#[doc = "GICD_ITARGETSR50 (rw) register accessor: Interrupt Processor Target 200 - 203\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr50`]
module"]
#[doc(alias = "GICD_ITARGETSR50")]
pub type GicdItargetsr50 = crate::Reg<gicd_itargetsr50::GicdItargetsr50Spec>;
#[doc = "Interrupt Processor Target 200 - 203"]
pub mod gicd_itargetsr50;
#[doc = "GICD_ITARGETSR51 (rw) register accessor: Interrupt Processor Target 204 - 207\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr51::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr51::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr51`]
module"]
#[doc(alias = "GICD_ITARGETSR51")]
pub type GicdItargetsr51 = crate::Reg<gicd_itargetsr51::GicdItargetsr51Spec>;
#[doc = "Interrupt Processor Target 204 - 207"]
pub mod gicd_itargetsr51;
#[doc = "GICD_ITARGETSR52 (rw) register accessor: Interrupt Processor Target 208 - 211\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr52::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr52::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr52`]
module"]
#[doc(alias = "GICD_ITARGETSR52")]
pub type GicdItargetsr52 = crate::Reg<gicd_itargetsr52::GicdItargetsr52Spec>;
#[doc = "Interrupt Processor Target 208 - 211"]
pub mod gicd_itargetsr52;
#[doc = "GICD_ITARGETSR53 (rw) register accessor: Interrupt Processor Target 212 - 215\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr53::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr53::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr53`]
module"]
#[doc(alias = "GICD_ITARGETSR53")]
pub type GicdItargetsr53 = crate::Reg<gicd_itargetsr53::GicdItargetsr53Spec>;
#[doc = "Interrupt Processor Target 212 - 215"]
pub mod gicd_itargetsr53;
#[doc = "GICD_ITARGETSR54 (rw) register accessor: Interrupt Processor Target 216 - 219\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr54`]
module"]
#[doc(alias = "GICD_ITARGETSR54")]
pub type GicdItargetsr54 = crate::Reg<gicd_itargetsr54::GicdItargetsr54Spec>;
#[doc = "Interrupt Processor Target 216 - 219"]
pub mod gicd_itargetsr54;
#[doc = "GICD_ITARGETSR55 (rw) register accessor: Interrupt Processor Target 220 - 223\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr55::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr55::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr55`]
module"]
#[doc(alias = "GICD_ITARGETSR55")]
pub type GicdItargetsr55 = crate::Reg<gicd_itargetsr55::GicdItargetsr55Spec>;
#[doc = "Interrupt Processor Target 220 - 223"]
pub mod gicd_itargetsr55;

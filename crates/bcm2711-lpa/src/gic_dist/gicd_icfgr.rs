#[repr(C)]
#[derive(Debug)]
#[doc = "Interrupt Configuration"]
#[doc(alias = "GICD_ICFGR")]
pub struct GicdIcfgr {
    gicd_icfgr0: GicdIcfgr0,
    gicd_icfgr4: GicdIcfgr4,
    gicd_icfgr8: GicdIcfgr8,
    gicd_icfgr12: GicdIcfgr12,
    gicd_icfgr16: GicdIcfgr16,
    gicd_icfgr20: GicdIcfgr20,
    gicd_icfgr24: GicdIcfgr24,
    gicd_icfgr28: GicdIcfgr28,
    gicd_icfgr32: GicdIcfgr32,
    gicd_icfgr36: GicdIcfgr36,
    gicd_icfgr40: GicdIcfgr40,
    gicd_icfgr44: GicdIcfgr44,
    gicd_icfgr48: GicdIcfgr48,
    gicd_icfgr52: GicdIcfgr52,
}
impl GicdIcfgr {
    #[doc = "0x00 - Interrupt Configuration 0 - 15"]
    #[inline(always)]
    pub const fn gicd_icfgr0(&self) -> &GicdIcfgr0 {
        &self.gicd_icfgr0
    }
    #[doc = "0x04 - Interrupt Configuration 16 - 31"]
    #[inline(always)]
    pub const fn gicd_icfgr4(&self) -> &GicdIcfgr4 {
        &self.gicd_icfgr4
    }
    #[doc = "0x08 - Interrupt Configuration 32 - 47"]
    #[inline(always)]
    pub const fn gicd_icfgr8(&self) -> &GicdIcfgr8 {
        &self.gicd_icfgr8
    }
    #[doc = "0x0c - Interrupt Configuration 48 - 63"]
    #[inline(always)]
    pub const fn gicd_icfgr12(&self) -> &GicdIcfgr12 {
        &self.gicd_icfgr12
    }
    #[doc = "0x10 - Interrupt Configuration 64 - 79"]
    #[inline(always)]
    pub const fn gicd_icfgr16(&self) -> &GicdIcfgr16 {
        &self.gicd_icfgr16
    }
    #[doc = "0x14 - Interrupt Configuration 80 - 95"]
    #[inline(always)]
    pub const fn gicd_icfgr20(&self) -> &GicdIcfgr20 {
        &self.gicd_icfgr20
    }
    #[doc = "0x18 - Interrupt Configuration 96 - 111"]
    #[inline(always)]
    pub const fn gicd_icfgr24(&self) -> &GicdIcfgr24 {
        &self.gicd_icfgr24
    }
    #[doc = "0x1c - Interrupt Configuration 112 - 127"]
    #[inline(always)]
    pub const fn gicd_icfgr28(&self) -> &GicdIcfgr28 {
        &self.gicd_icfgr28
    }
    #[doc = "0x20 - Interrupt Configuration 128 - 143"]
    #[inline(always)]
    pub const fn gicd_icfgr32(&self) -> &GicdIcfgr32 {
        &self.gicd_icfgr32
    }
    #[doc = "0x24 - Interrupt Configuration 144 - 159"]
    #[inline(always)]
    pub const fn gicd_icfgr36(&self) -> &GicdIcfgr36 {
        &self.gicd_icfgr36
    }
    #[doc = "0x28 - Interrupt Configuration 160 - 175"]
    #[inline(always)]
    pub const fn gicd_icfgr40(&self) -> &GicdIcfgr40 {
        &self.gicd_icfgr40
    }
    #[doc = "0x2c - Interrupt Configuration 176 - 191"]
    #[inline(always)]
    pub const fn gicd_icfgr44(&self) -> &GicdIcfgr44 {
        &self.gicd_icfgr44
    }
    #[doc = "0x30 - Interrupt Configuration 192 - 207"]
    #[inline(always)]
    pub const fn gicd_icfgr48(&self) -> &GicdIcfgr48 {
        &self.gicd_icfgr48
    }
    #[doc = "0x34 - Interrupt Configuration 208 - 223"]
    #[inline(always)]
    pub const fn gicd_icfgr52(&self) -> &GicdIcfgr52 {
        &self.gicd_icfgr52
    }
}
#[doc = "GICD_ICFGR0 (rw) register accessor: Interrupt Configuration 0 - 15\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icfgr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icfgr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icfgr0`]
module"]
#[doc(alias = "GICD_ICFGR0")]
pub type GicdIcfgr0 = crate::Reg<gicd_icfgr0::GicdIcfgr0Spec>;
#[doc = "Interrupt Configuration 0 - 15"]
pub mod gicd_icfgr0;
#[doc = "GICD_ICFGR4 (rw) register accessor: Interrupt Configuration 16 - 31\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icfgr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icfgr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icfgr4`]
module"]
#[doc(alias = "GICD_ICFGR4")]
pub type GicdIcfgr4 = crate::Reg<gicd_icfgr4::GicdIcfgr4Spec>;
#[doc = "Interrupt Configuration 16 - 31"]
pub mod gicd_icfgr4;
#[doc = "GICD_ICFGR8 (rw) register accessor: Interrupt Configuration 32 - 47\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icfgr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icfgr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icfgr8`]
module"]
#[doc(alias = "GICD_ICFGR8")]
pub type GicdIcfgr8 = crate::Reg<gicd_icfgr8::GicdIcfgr8Spec>;
#[doc = "Interrupt Configuration 32 - 47"]
pub mod gicd_icfgr8;
#[doc = "GICD_ICFGR12 (rw) register accessor: Interrupt Configuration 48 - 63\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icfgr12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icfgr12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icfgr12`]
module"]
#[doc(alias = "GICD_ICFGR12")]
pub type GicdIcfgr12 = crate::Reg<gicd_icfgr12::GicdIcfgr12Spec>;
#[doc = "Interrupt Configuration 48 - 63"]
pub mod gicd_icfgr12;
#[doc = "GICD_ICFGR16 (rw) register accessor: Interrupt Configuration 64 - 79\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icfgr16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icfgr16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icfgr16`]
module"]
#[doc(alias = "GICD_ICFGR16")]
pub type GicdIcfgr16 = crate::Reg<gicd_icfgr16::GicdIcfgr16Spec>;
#[doc = "Interrupt Configuration 64 - 79"]
pub mod gicd_icfgr16;
#[doc = "GICD_ICFGR20 (rw) register accessor: Interrupt Configuration 80 - 95\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icfgr20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icfgr20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icfgr20`]
module"]
#[doc(alias = "GICD_ICFGR20")]
pub type GicdIcfgr20 = crate::Reg<gicd_icfgr20::GicdIcfgr20Spec>;
#[doc = "Interrupt Configuration 80 - 95"]
pub mod gicd_icfgr20;
#[doc = "GICD_ICFGR24 (rw) register accessor: Interrupt Configuration 96 - 111\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icfgr24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icfgr24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icfgr24`]
module"]
#[doc(alias = "GICD_ICFGR24")]
pub type GicdIcfgr24 = crate::Reg<gicd_icfgr24::GicdIcfgr24Spec>;
#[doc = "Interrupt Configuration 96 - 111"]
pub mod gicd_icfgr24;
#[doc = "GICD_ICFGR28 (rw) register accessor: Interrupt Configuration 112 - 127\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icfgr28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icfgr28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icfgr28`]
module"]
#[doc(alias = "GICD_ICFGR28")]
pub type GicdIcfgr28 = crate::Reg<gicd_icfgr28::GicdIcfgr28Spec>;
#[doc = "Interrupt Configuration 112 - 127"]
pub mod gicd_icfgr28;
#[doc = "GICD_ICFGR32 (rw) register accessor: Interrupt Configuration 128 - 143\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icfgr32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icfgr32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icfgr32`]
module"]
#[doc(alias = "GICD_ICFGR32")]
pub type GicdIcfgr32 = crate::Reg<gicd_icfgr32::GicdIcfgr32Spec>;
#[doc = "Interrupt Configuration 128 - 143"]
pub mod gicd_icfgr32;
#[doc = "GICD_ICFGR36 (rw) register accessor: Interrupt Configuration 144 - 159\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icfgr36::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icfgr36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icfgr36`]
module"]
#[doc(alias = "GICD_ICFGR36")]
pub type GicdIcfgr36 = crate::Reg<gicd_icfgr36::GicdIcfgr36Spec>;
#[doc = "Interrupt Configuration 144 - 159"]
pub mod gicd_icfgr36;
#[doc = "GICD_ICFGR40 (rw) register accessor: Interrupt Configuration 160 - 175\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icfgr40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icfgr40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icfgr40`]
module"]
#[doc(alias = "GICD_ICFGR40")]
pub type GicdIcfgr40 = crate::Reg<gicd_icfgr40::GicdIcfgr40Spec>;
#[doc = "Interrupt Configuration 160 - 175"]
pub mod gicd_icfgr40;
#[doc = "GICD_ICFGR44 (rw) register accessor: Interrupt Configuration 176 - 191\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icfgr44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icfgr44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icfgr44`]
module"]
#[doc(alias = "GICD_ICFGR44")]
pub type GicdIcfgr44 = crate::Reg<gicd_icfgr44::GicdIcfgr44Spec>;
#[doc = "Interrupt Configuration 176 - 191"]
pub mod gicd_icfgr44;
#[doc = "GICD_ICFGR48 (rw) register accessor: Interrupt Configuration 192 - 207\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icfgr48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icfgr48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icfgr48`]
module"]
#[doc(alias = "GICD_ICFGR48")]
pub type GicdIcfgr48 = crate::Reg<gicd_icfgr48::GicdIcfgr48Spec>;
#[doc = "Interrupt Configuration 192 - 207"]
pub mod gicd_icfgr48;
#[doc = "GICD_ICFGR52 (rw) register accessor: Interrupt Configuration 208 - 223\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icfgr52::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icfgr52::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icfgr52`]
module"]
#[doc(alias = "GICD_ICFGR52")]
pub type GicdIcfgr52 = crate::Reg<gicd_icfgr52::GicdIcfgr52Spec>;
#[doc = "Interrupt Configuration 208 - 223"]
pub mod gicd_icfgr52;

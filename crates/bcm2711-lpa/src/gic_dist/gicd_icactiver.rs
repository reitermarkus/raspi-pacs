#[repr(C)]
#[derive(Debug)]
#[doc = "Interrupt Clear-Active Registers"]
#[doc(alias = "GICD_ICACTIVER")]
pub struct GicdIcactiver {
    gicd_icactiver0: GicdIcactiver0,
    gicd_icactiver1: GicdIcactiver1,
    gicd_icactiver2: GicdIcactiver2,
    gicd_icactiver3: GicdIcactiver3,
    gicd_icactiver4: GicdIcactiver4,
    gicd_icactiver5: GicdIcactiver5,
    gicd_icactiver6: GicdIcactiver6,
}
impl GicdIcactiver {
    #[doc = "0x00 - Interrupt Clear-Active"]
    #[inline(always)]
    pub const fn gicd_icactiver0(&self) -> &GicdIcactiver0 {
        &self.gicd_icactiver0
    }
    #[doc = "0x04 - Interrupt Clear-Active"]
    #[inline(always)]
    pub const fn gicd_icactiver1(&self) -> &GicdIcactiver1 {
        &self.gicd_icactiver1
    }
    #[doc = "0x08 - Interrupt Clear-Active"]
    #[inline(always)]
    pub const fn gicd_icactiver2(&self) -> &GicdIcactiver2 {
        &self.gicd_icactiver2
    }
    #[doc = "0x0c - Interrupt Clear-Active"]
    #[inline(always)]
    pub const fn gicd_icactiver3(&self) -> &GicdIcactiver3 {
        &self.gicd_icactiver3
    }
    #[doc = "0x10 - Interrupt Clear-Active"]
    #[inline(always)]
    pub const fn gicd_icactiver4(&self) -> &GicdIcactiver4 {
        &self.gicd_icactiver4
    }
    #[doc = "0x14 - Interrupt Clear-Active"]
    #[inline(always)]
    pub const fn gicd_icactiver5(&self) -> &GicdIcactiver5 {
        &self.gicd_icactiver5
    }
    #[doc = "0x18 - Interrupt Clear-Active"]
    #[inline(always)]
    pub const fn gicd_icactiver6(&self) -> &GicdIcactiver6 {
        &self.gicd_icactiver6
    }
}
#[doc = "GICD_ICACTIVER0 (rw) register accessor: Interrupt Clear-Active\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icactiver0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icactiver0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icactiver0`]
module"]
#[doc(alias = "GICD_ICACTIVER0")]
pub type GicdIcactiver0 = crate::Reg<gicd_icactiver0::GicdIcactiver0Spec>;
#[doc = "Interrupt Clear-Active"]
pub mod gicd_icactiver0;
#[doc = "GICD_ICACTIVER1 (rw) register accessor: Interrupt Clear-Active\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icactiver1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icactiver1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icactiver1`]
module"]
#[doc(alias = "GICD_ICACTIVER1")]
pub type GicdIcactiver1 = crate::Reg<gicd_icactiver1::GicdIcactiver1Spec>;
#[doc = "Interrupt Clear-Active"]
pub mod gicd_icactiver1;
#[doc = "GICD_ICACTIVER2 (rw) register accessor: Interrupt Clear-Active\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icactiver2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icactiver2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icactiver2`]
module"]
#[doc(alias = "GICD_ICACTIVER2")]
pub type GicdIcactiver2 = crate::Reg<gicd_icactiver2::GicdIcactiver2Spec>;
#[doc = "Interrupt Clear-Active"]
pub mod gicd_icactiver2;
#[doc = "GICD_ICACTIVER3 (rw) register accessor: Interrupt Clear-Active\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icactiver3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icactiver3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icactiver3`]
module"]
#[doc(alias = "GICD_ICACTIVER3")]
pub type GicdIcactiver3 = crate::Reg<gicd_icactiver3::GicdIcactiver3Spec>;
#[doc = "Interrupt Clear-Active"]
pub mod gicd_icactiver3;
#[doc = "GICD_ICACTIVER4 (rw) register accessor: Interrupt Clear-Active\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icactiver4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icactiver4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icactiver4`]
module"]
#[doc(alias = "GICD_ICACTIVER4")]
pub type GicdIcactiver4 = crate::Reg<gicd_icactiver4::GicdIcactiver4Spec>;
#[doc = "Interrupt Clear-Active"]
pub mod gicd_icactiver4;
#[doc = "GICD_ICACTIVER5 (rw) register accessor: Interrupt Clear-Active\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icactiver5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icactiver5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icactiver5`]
module"]
#[doc(alias = "GICD_ICACTIVER5")]
pub type GicdIcactiver5 = crate::Reg<gicd_icactiver5::GicdIcactiver5Spec>;
#[doc = "Interrupt Clear-Active"]
pub mod gicd_icactiver5;
#[doc = "GICD_ICACTIVER6 (rw) register accessor: Interrupt Clear-Active\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icactiver6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icactiver6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icactiver6`]
module"]
#[doc(alias = "GICD_ICACTIVER6")]
pub type GicdIcactiver6 = crate::Reg<gicd_icactiver6::GicdIcactiver6Spec>;
#[doc = "Interrupt Clear-Active"]
pub mod gicd_icactiver6;

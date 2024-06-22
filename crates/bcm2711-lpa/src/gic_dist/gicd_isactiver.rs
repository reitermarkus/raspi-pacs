#[repr(C)]
#[derive(Debug)]
#[doc = "Interrupt Set-Active Registers"]
#[doc(alias = "GICD_ISACTIVER")]
pub struct GicdIsactiver {
    gicd_isactiver0: GicdIsactiver0,
    gicd_isactiver1: GicdIsactiver1,
    gicd_isactiver2: GicdIsactiver2,
    gicd_isactiver3: GicdIsactiver3,
    gicd_isactiver4: GicdIsactiver4,
    gicd_isactiver5: GicdIsactiver5,
    gicd_isactiver6: GicdIsactiver6,
}
impl GicdIsactiver {
    #[doc = "0x00 - Interrupt Set-Active"]
    #[inline(always)]
    pub const fn gicd_isactiver0(&self) -> &GicdIsactiver0 {
        &self.gicd_isactiver0
    }
    #[doc = "0x04 - Interrupt Set-Active"]
    #[inline(always)]
    pub const fn gicd_isactiver1(&self) -> &GicdIsactiver1 {
        &self.gicd_isactiver1
    }
    #[doc = "0x08 - Interrupt Set-Active"]
    #[inline(always)]
    pub const fn gicd_isactiver2(&self) -> &GicdIsactiver2 {
        &self.gicd_isactiver2
    }
    #[doc = "0x0c - Interrupt Set-Active"]
    #[inline(always)]
    pub const fn gicd_isactiver3(&self) -> &GicdIsactiver3 {
        &self.gicd_isactiver3
    }
    #[doc = "0x10 - Interrupt Set-Active"]
    #[inline(always)]
    pub const fn gicd_isactiver4(&self) -> &GicdIsactiver4 {
        &self.gicd_isactiver4
    }
    #[doc = "0x14 - Interrupt Set-Active"]
    #[inline(always)]
    pub const fn gicd_isactiver5(&self) -> &GicdIsactiver5 {
        &self.gicd_isactiver5
    }
    #[doc = "0x18 - Interrupt Set-Active"]
    #[inline(always)]
    pub const fn gicd_isactiver6(&self) -> &GicdIsactiver6 {
        &self.gicd_isactiver6
    }
}
#[doc = "GICD_ISACTIVER0 (rw) register accessor: Interrupt Set-Active\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_isactiver0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_isactiver0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isactiver0`]
module"]
#[doc(alias = "GICD_ISACTIVER0")]
pub type GicdIsactiver0 = crate::Reg<gicd_isactiver0::GicdIsactiver0Spec>;
#[doc = "Interrupt Set-Active"]
pub mod gicd_isactiver0;
#[doc = "GICD_ISACTIVER1 (rw) register accessor: Interrupt Set-Active\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_isactiver1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_isactiver1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isactiver1`]
module"]
#[doc(alias = "GICD_ISACTIVER1")]
pub type GicdIsactiver1 = crate::Reg<gicd_isactiver1::GicdIsactiver1Spec>;
#[doc = "Interrupt Set-Active"]
pub mod gicd_isactiver1;
#[doc = "GICD_ISACTIVER2 (rw) register accessor: Interrupt Set-Active\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_isactiver2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_isactiver2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isactiver2`]
module"]
#[doc(alias = "GICD_ISACTIVER2")]
pub type GicdIsactiver2 = crate::Reg<gicd_isactiver2::GicdIsactiver2Spec>;
#[doc = "Interrupt Set-Active"]
pub mod gicd_isactiver2;
#[doc = "GICD_ISACTIVER3 (rw) register accessor: Interrupt Set-Active\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_isactiver3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_isactiver3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isactiver3`]
module"]
#[doc(alias = "GICD_ISACTIVER3")]
pub type GicdIsactiver3 = crate::Reg<gicd_isactiver3::GicdIsactiver3Spec>;
#[doc = "Interrupt Set-Active"]
pub mod gicd_isactiver3;
#[doc = "GICD_ISACTIVER4 (rw) register accessor: Interrupt Set-Active\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_isactiver4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_isactiver4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isactiver4`]
module"]
#[doc(alias = "GICD_ISACTIVER4")]
pub type GicdIsactiver4 = crate::Reg<gicd_isactiver4::GicdIsactiver4Spec>;
#[doc = "Interrupt Set-Active"]
pub mod gicd_isactiver4;
#[doc = "GICD_ISACTIVER5 (rw) register accessor: Interrupt Set-Active\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_isactiver5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_isactiver5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isactiver5`]
module"]
#[doc(alias = "GICD_ISACTIVER5")]
pub type GicdIsactiver5 = crate::Reg<gicd_isactiver5::GicdIsactiver5Spec>;
#[doc = "Interrupt Set-Active"]
pub mod gicd_isactiver5;
#[doc = "GICD_ISACTIVER6 (rw) register accessor: Interrupt Set-Active\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_isactiver6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_isactiver6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isactiver6`]
module"]
#[doc(alias = "GICD_ISACTIVER6")]
pub type GicdIsactiver6 = crate::Reg<gicd_isactiver6::GicdIsactiver6Spec>;
#[doc = "Interrupt Set-Active"]
pub mod gicd_isactiver6;

#[repr(C)]
#[derive(Debug)]
#[doc = "Interrupt Clear-Enable Registers"]
#[doc(alias = "GICD_ICENABLER")]
pub struct GicdIcenabler {
    gicd_icenabler0: GicdIcenabler0,
    gicd_icenabler1: GicdIcenabler1,
    gicd_icenabler2: GicdIcenabler2,
    gicd_icenabler3: GicdIcenabler3,
    gicd_icenabler4: GicdIcenabler4,
    gicd_icenabler5: GicdIcenabler5,
    gicd_icenabler6: GicdIcenabler6,
}
impl GicdIcenabler {
    #[doc = "0x00 - Interrupt Clear-Enable"]
    #[inline(always)]
    pub const fn gicd_icenabler0(&self) -> &GicdIcenabler0 {
        &self.gicd_icenabler0
    }
    #[doc = "0x04 - Interrupt Clear-Enable"]
    #[inline(always)]
    pub const fn gicd_icenabler1(&self) -> &GicdIcenabler1 {
        &self.gicd_icenabler1
    }
    #[doc = "0x08 - Interrupt Clear-Enable"]
    #[inline(always)]
    pub const fn gicd_icenabler2(&self) -> &GicdIcenabler2 {
        &self.gicd_icenabler2
    }
    #[doc = "0x0c - Interrupt Clear-Enable"]
    #[inline(always)]
    pub const fn gicd_icenabler3(&self) -> &GicdIcenabler3 {
        &self.gicd_icenabler3
    }
    #[doc = "0x10 - Interrupt Clear-Enable"]
    #[inline(always)]
    pub const fn gicd_icenabler4(&self) -> &GicdIcenabler4 {
        &self.gicd_icenabler4
    }
    #[doc = "0x14 - Interrupt Clear-Enable"]
    #[inline(always)]
    pub const fn gicd_icenabler5(&self) -> &GicdIcenabler5 {
        &self.gicd_icenabler5
    }
    #[doc = "0x18 - Interrupt Clear-Enable"]
    #[inline(always)]
    pub const fn gicd_icenabler6(&self) -> &GicdIcenabler6 {
        &self.gicd_icenabler6
    }
}
#[doc = "GICD_ICENABLER0 (rw) register accessor: Interrupt Clear-Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icenabler0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icenabler0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icenabler0`]
module"]
#[doc(alias = "GICD_ICENABLER0")]
pub type GicdIcenabler0 = crate::Reg<gicd_icenabler0::GicdIcenabler0Spec>;
#[doc = "Interrupt Clear-Enable"]
pub mod gicd_icenabler0;
#[doc = "GICD_ICENABLER1 (rw) register accessor: Interrupt Clear-Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icenabler1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icenabler1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icenabler1`]
module"]
#[doc(alias = "GICD_ICENABLER1")]
pub type GicdIcenabler1 = crate::Reg<gicd_icenabler1::GicdIcenabler1Spec>;
#[doc = "Interrupt Clear-Enable"]
pub mod gicd_icenabler1;
#[doc = "GICD_ICENABLER2 (rw) register accessor: Interrupt Clear-Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icenabler2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icenabler2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icenabler2`]
module"]
#[doc(alias = "GICD_ICENABLER2")]
pub type GicdIcenabler2 = crate::Reg<gicd_icenabler2::GicdIcenabler2Spec>;
#[doc = "Interrupt Clear-Enable"]
pub mod gicd_icenabler2;
#[doc = "GICD_ICENABLER3 (rw) register accessor: Interrupt Clear-Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icenabler3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icenabler3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icenabler3`]
module"]
#[doc(alias = "GICD_ICENABLER3")]
pub type GicdIcenabler3 = crate::Reg<gicd_icenabler3::GicdIcenabler3Spec>;
#[doc = "Interrupt Clear-Enable"]
pub mod gicd_icenabler3;
#[doc = "GICD_ICENABLER4 (rw) register accessor: Interrupt Clear-Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icenabler4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icenabler4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icenabler4`]
module"]
#[doc(alias = "GICD_ICENABLER4")]
pub type GicdIcenabler4 = crate::Reg<gicd_icenabler4::GicdIcenabler4Spec>;
#[doc = "Interrupt Clear-Enable"]
pub mod gicd_icenabler4;
#[doc = "GICD_ICENABLER5 (rw) register accessor: Interrupt Clear-Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icenabler5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icenabler5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icenabler5`]
module"]
#[doc(alias = "GICD_ICENABLER5")]
pub type GicdIcenabler5 = crate::Reg<gicd_icenabler5::GicdIcenabler5Spec>;
#[doc = "Interrupt Clear-Enable"]
pub mod gicd_icenabler5;
#[doc = "GICD_ICENABLER6 (rw) register accessor: Interrupt Clear-Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icenabler6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icenabler6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icenabler6`]
module"]
#[doc(alias = "GICD_ICENABLER6")]
pub type GicdIcenabler6 = crate::Reg<gicd_icenabler6::GicdIcenabler6Spec>;
#[doc = "Interrupt Clear-Enable"]
pub mod gicd_icenabler6;

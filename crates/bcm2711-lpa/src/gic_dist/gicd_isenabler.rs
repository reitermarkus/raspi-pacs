#[repr(C)]
#[derive(Debug)]
#[doc = "Interrupt Set-Enable Registers"]
#[doc(alias = "GICD_ISENABLER")]
pub struct GicdIsenabler {
    gicd_isenabler0: GicdIsenabler0,
    gicd_isenabler1: GicdIsenabler1,
    gicd_isenabler2: GicdIsenabler2,
    gicd_isenabler3: GicdIsenabler3,
    gicd_isenabler4: GicdIsenabler4,
    gicd_isenabler5: GicdIsenabler5,
    gicd_isenabler6: GicdIsenabler6,
}
impl GicdIsenabler {
    #[doc = "0x00 - Interrupt Set-Enable"]
    #[inline(always)]
    pub const fn gicd_isenabler0(&self) -> &GicdIsenabler0 {
        &self.gicd_isenabler0
    }
    #[doc = "0x04 - Interrupt Set-Enable"]
    #[inline(always)]
    pub const fn gicd_isenabler1(&self) -> &GicdIsenabler1 {
        &self.gicd_isenabler1
    }
    #[doc = "0x08 - Interrupt Set-Enable"]
    #[inline(always)]
    pub const fn gicd_isenabler2(&self) -> &GicdIsenabler2 {
        &self.gicd_isenabler2
    }
    #[doc = "0x0c - Interrupt Set-Enable"]
    #[inline(always)]
    pub const fn gicd_isenabler3(&self) -> &GicdIsenabler3 {
        &self.gicd_isenabler3
    }
    #[doc = "0x10 - Interrupt Set-Enable"]
    #[inline(always)]
    pub const fn gicd_isenabler4(&self) -> &GicdIsenabler4 {
        &self.gicd_isenabler4
    }
    #[doc = "0x14 - Interrupt Set-Enable"]
    #[inline(always)]
    pub const fn gicd_isenabler5(&self) -> &GicdIsenabler5 {
        &self.gicd_isenabler5
    }
    #[doc = "0x18 - Interrupt Set-Enable"]
    #[inline(always)]
    pub const fn gicd_isenabler6(&self) -> &GicdIsenabler6 {
        &self.gicd_isenabler6
    }
}
#[doc = "GICD_ISENABLER0 (rw) register accessor: Interrupt Set-Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_isenabler0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_isenabler0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isenabler0`]
module"]
#[doc(alias = "GICD_ISENABLER0")]
pub type GicdIsenabler0 = crate::Reg<gicd_isenabler0::GicdIsenabler0Spec>;
#[doc = "Interrupt Set-Enable"]
pub mod gicd_isenabler0;
#[doc = "GICD_ISENABLER1 (rw) register accessor: Interrupt Set-Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_isenabler1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_isenabler1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isenabler1`]
module"]
#[doc(alias = "GICD_ISENABLER1")]
pub type GicdIsenabler1 = crate::Reg<gicd_isenabler1::GicdIsenabler1Spec>;
#[doc = "Interrupt Set-Enable"]
pub mod gicd_isenabler1;
#[doc = "GICD_ISENABLER2 (rw) register accessor: Interrupt Set-Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_isenabler2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_isenabler2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isenabler2`]
module"]
#[doc(alias = "GICD_ISENABLER2")]
pub type GicdIsenabler2 = crate::Reg<gicd_isenabler2::GicdIsenabler2Spec>;
#[doc = "Interrupt Set-Enable"]
pub mod gicd_isenabler2;
#[doc = "GICD_ISENABLER3 (rw) register accessor: Interrupt Set-Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_isenabler3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_isenabler3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isenabler3`]
module"]
#[doc(alias = "GICD_ISENABLER3")]
pub type GicdIsenabler3 = crate::Reg<gicd_isenabler3::GicdIsenabler3Spec>;
#[doc = "Interrupt Set-Enable"]
pub mod gicd_isenabler3;
#[doc = "GICD_ISENABLER4 (rw) register accessor: Interrupt Set-Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_isenabler4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_isenabler4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isenabler4`]
module"]
#[doc(alias = "GICD_ISENABLER4")]
pub type GicdIsenabler4 = crate::Reg<gicd_isenabler4::GicdIsenabler4Spec>;
#[doc = "Interrupt Set-Enable"]
pub mod gicd_isenabler4;
#[doc = "GICD_ISENABLER5 (rw) register accessor: Interrupt Set-Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_isenabler5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_isenabler5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isenabler5`]
module"]
#[doc(alias = "GICD_ISENABLER5")]
pub type GicdIsenabler5 = crate::Reg<gicd_isenabler5::GicdIsenabler5Spec>;
#[doc = "Interrupt Set-Enable"]
pub mod gicd_isenabler5;
#[doc = "GICD_ISENABLER6 (rw) register accessor: Interrupt Set-Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_isenabler6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_isenabler6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isenabler6`]
module"]
#[doc(alias = "GICD_ISENABLER6")]
pub type GicdIsenabler6 = crate::Reg<gicd_isenabler6::GicdIsenabler6Spec>;
#[doc = "Interrupt Set-Enable"]
pub mod gicd_isenabler6;

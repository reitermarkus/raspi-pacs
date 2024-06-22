#[repr(C)]
#[derive(Debug)]
#[doc = "Interrupt Set-Pending Registers"]
#[doc(alias = "GICD_ISPENDR")]
pub struct GicdIspendr {
    gicd_ispendr0: GicdIspendr0,
    gicd_ispendr1: GicdIspendr1,
    gicd_ispendr2: GicdIspendr2,
    gicd_ispendr3: GicdIspendr3,
    gicd_ispendr4: GicdIspendr4,
    gicd_ispendr5: GicdIspendr5,
    gicd_ispendr6: GicdIspendr6,
}
impl GicdIspendr {
    #[doc = "0x00 - Interrupt Set-Pending"]
    #[inline(always)]
    pub const fn gicd_ispendr0(&self) -> &GicdIspendr0 {
        &self.gicd_ispendr0
    }
    #[doc = "0x04 - Interrupt Set-Pending"]
    #[inline(always)]
    pub const fn gicd_ispendr1(&self) -> &GicdIspendr1 {
        &self.gicd_ispendr1
    }
    #[doc = "0x08 - Interrupt Set-Pending"]
    #[inline(always)]
    pub const fn gicd_ispendr2(&self) -> &GicdIspendr2 {
        &self.gicd_ispendr2
    }
    #[doc = "0x0c - Interrupt Set-Pending"]
    #[inline(always)]
    pub const fn gicd_ispendr3(&self) -> &GicdIspendr3 {
        &self.gicd_ispendr3
    }
    #[doc = "0x10 - Interrupt Set-Pending"]
    #[inline(always)]
    pub const fn gicd_ispendr4(&self) -> &GicdIspendr4 {
        &self.gicd_ispendr4
    }
    #[doc = "0x14 - Interrupt Set-Pending"]
    #[inline(always)]
    pub const fn gicd_ispendr5(&self) -> &GicdIspendr5 {
        &self.gicd_ispendr5
    }
    #[doc = "0x18 - Interrupt Set-Pending"]
    #[inline(always)]
    pub const fn gicd_ispendr6(&self) -> &GicdIspendr6 {
        &self.gicd_ispendr6
    }
}
#[doc = "GICD_ISPENDR0 (rw) register accessor: Interrupt Set-Pending\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ispendr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ispendr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ispendr0`]
module"]
#[doc(alias = "GICD_ISPENDR0")]
pub type GicdIspendr0 = crate::Reg<gicd_ispendr0::GicdIspendr0Spec>;
#[doc = "Interrupt Set-Pending"]
pub mod gicd_ispendr0;
#[doc = "GICD_ISPENDR1 (rw) register accessor: Interrupt Set-Pending\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ispendr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ispendr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ispendr1`]
module"]
#[doc(alias = "GICD_ISPENDR1")]
pub type GicdIspendr1 = crate::Reg<gicd_ispendr1::GicdIspendr1Spec>;
#[doc = "Interrupt Set-Pending"]
pub mod gicd_ispendr1;
#[doc = "GICD_ISPENDR2 (rw) register accessor: Interrupt Set-Pending\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ispendr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ispendr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ispendr2`]
module"]
#[doc(alias = "GICD_ISPENDR2")]
pub type GicdIspendr2 = crate::Reg<gicd_ispendr2::GicdIspendr2Spec>;
#[doc = "Interrupt Set-Pending"]
pub mod gicd_ispendr2;
#[doc = "GICD_ISPENDR3 (rw) register accessor: Interrupt Set-Pending\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ispendr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ispendr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ispendr3`]
module"]
#[doc(alias = "GICD_ISPENDR3")]
pub type GicdIspendr3 = crate::Reg<gicd_ispendr3::GicdIspendr3Spec>;
#[doc = "Interrupt Set-Pending"]
pub mod gicd_ispendr3;
#[doc = "GICD_ISPENDR4 (rw) register accessor: Interrupt Set-Pending\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ispendr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ispendr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ispendr4`]
module"]
#[doc(alias = "GICD_ISPENDR4")]
pub type GicdIspendr4 = crate::Reg<gicd_ispendr4::GicdIspendr4Spec>;
#[doc = "Interrupt Set-Pending"]
pub mod gicd_ispendr4;
#[doc = "GICD_ISPENDR5 (rw) register accessor: Interrupt Set-Pending\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ispendr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ispendr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ispendr5`]
module"]
#[doc(alias = "GICD_ISPENDR5")]
pub type GicdIspendr5 = crate::Reg<gicd_ispendr5::GicdIspendr5Spec>;
#[doc = "Interrupt Set-Pending"]
pub mod gicd_ispendr5;
#[doc = "GICD_ISPENDR6 (rw) register accessor: Interrupt Set-Pending\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ispendr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ispendr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ispendr6`]
module"]
#[doc(alias = "GICD_ISPENDR6")]
pub type GicdIspendr6 = crate::Reg<gicd_ispendr6::GicdIspendr6Spec>;
#[doc = "Interrupt Set-Pending"]
pub mod gicd_ispendr6;

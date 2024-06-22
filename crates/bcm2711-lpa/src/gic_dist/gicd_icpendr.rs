#[repr(C)]
#[derive(Debug)]
#[doc = "Interrupt Clear-Pending Registers"]
#[doc(alias = "GICD_ICPENDR")]
pub struct GicdIcpendr {
    gicd_icpendr0: GicdIcpendr0,
    gicd_icpendr1: GicdIcpendr1,
    gicd_icpendr2: GicdIcpendr2,
    gicd_icpendr3: GicdIcpendr3,
    gicd_icpendr4: GicdIcpendr4,
    gicd_icpendr5: GicdIcpendr5,
    gicd_icpendr6: GicdIcpendr6,
}
impl GicdIcpendr {
    #[doc = "0x00 - Interrupt Clear-Pending"]
    #[inline(always)]
    pub const fn gicd_icpendr0(&self) -> &GicdIcpendr0 {
        &self.gicd_icpendr0
    }
    #[doc = "0x04 - Interrupt Clear-Pending"]
    #[inline(always)]
    pub const fn gicd_icpendr1(&self) -> &GicdIcpendr1 {
        &self.gicd_icpendr1
    }
    #[doc = "0x08 - Interrupt Clear-Pending"]
    #[inline(always)]
    pub const fn gicd_icpendr2(&self) -> &GicdIcpendr2 {
        &self.gicd_icpendr2
    }
    #[doc = "0x0c - Interrupt Clear-Pending"]
    #[inline(always)]
    pub const fn gicd_icpendr3(&self) -> &GicdIcpendr3 {
        &self.gicd_icpendr3
    }
    #[doc = "0x10 - Interrupt Clear-Pending"]
    #[inline(always)]
    pub const fn gicd_icpendr4(&self) -> &GicdIcpendr4 {
        &self.gicd_icpendr4
    }
    #[doc = "0x14 - Interrupt Clear-Pending"]
    #[inline(always)]
    pub const fn gicd_icpendr5(&self) -> &GicdIcpendr5 {
        &self.gicd_icpendr5
    }
    #[doc = "0x18 - Interrupt Clear-Pending"]
    #[inline(always)]
    pub const fn gicd_icpendr6(&self) -> &GicdIcpendr6 {
        &self.gicd_icpendr6
    }
}
#[doc = "GICD_ICPENDR0 (rw) register accessor: Interrupt Clear-Pending\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icpendr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icpendr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icpendr0`]
module"]
#[doc(alias = "GICD_ICPENDR0")]
pub type GicdIcpendr0 = crate::Reg<gicd_icpendr0::GicdIcpendr0Spec>;
#[doc = "Interrupt Clear-Pending"]
pub mod gicd_icpendr0;
#[doc = "GICD_ICPENDR1 (rw) register accessor: Interrupt Clear-Pending\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icpendr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icpendr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icpendr1`]
module"]
#[doc(alias = "GICD_ICPENDR1")]
pub type GicdIcpendr1 = crate::Reg<gicd_icpendr1::GicdIcpendr1Spec>;
#[doc = "Interrupt Clear-Pending"]
pub mod gicd_icpendr1;
#[doc = "GICD_ICPENDR2 (rw) register accessor: Interrupt Clear-Pending\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icpendr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icpendr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icpendr2`]
module"]
#[doc(alias = "GICD_ICPENDR2")]
pub type GicdIcpendr2 = crate::Reg<gicd_icpendr2::GicdIcpendr2Spec>;
#[doc = "Interrupt Clear-Pending"]
pub mod gicd_icpendr2;
#[doc = "GICD_ICPENDR3 (rw) register accessor: Interrupt Clear-Pending\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icpendr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icpendr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icpendr3`]
module"]
#[doc(alias = "GICD_ICPENDR3")]
pub type GicdIcpendr3 = crate::Reg<gicd_icpendr3::GicdIcpendr3Spec>;
#[doc = "Interrupt Clear-Pending"]
pub mod gicd_icpendr3;
#[doc = "GICD_ICPENDR4 (rw) register accessor: Interrupt Clear-Pending\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icpendr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icpendr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icpendr4`]
module"]
#[doc(alias = "GICD_ICPENDR4")]
pub type GicdIcpendr4 = crate::Reg<gicd_icpendr4::GicdIcpendr4Spec>;
#[doc = "Interrupt Clear-Pending"]
pub mod gicd_icpendr4;
#[doc = "GICD_ICPENDR5 (rw) register accessor: Interrupt Clear-Pending\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icpendr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icpendr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icpendr5`]
module"]
#[doc(alias = "GICD_ICPENDR5")]
pub type GicdIcpendr5 = crate::Reg<gicd_icpendr5::GicdIcpendr5Spec>;
#[doc = "Interrupt Clear-Pending"]
pub mod gicd_icpendr5;
#[doc = "GICD_ICPENDR6 (rw) register accessor: Interrupt Clear-Pending\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icpendr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icpendr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icpendr6`]
module"]
#[doc(alias = "GICD_ICPENDR6")]
pub type GicdIcpendr6 = crate::Reg<gicd_icpendr6::GicdIcpendr6Spec>;
#[doc = "Interrupt Clear-Pending"]
pub mod gicd_icpendr6;

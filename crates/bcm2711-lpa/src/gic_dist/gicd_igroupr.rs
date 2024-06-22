#[repr(C)]
#[derive(Debug)]
#[doc = "Interrupt Group Registers"]
#[doc(alias = "GICD_IGROUPR")]
pub struct GicdIgroupr {
    gicd_igroupr0: GicdIgroupr0,
    gicd_igroupr1: GicdIgroupr1,
    gicd_igroupr2: GicdIgroupr2,
    gicd_igroupr3: GicdIgroupr3,
    gicd_igroupr4: GicdIgroupr4,
    gicd_igroupr5: GicdIgroupr5,
    gicd_igroupr6: GicdIgroupr6,
}
impl GicdIgroupr {
    #[doc = "0x00 - Interrupt Group"]
    #[inline(always)]
    pub const fn gicd_igroupr0(&self) -> &GicdIgroupr0 {
        &self.gicd_igroupr0
    }
    #[doc = "0x04 - Interrupt Group"]
    #[inline(always)]
    pub const fn gicd_igroupr1(&self) -> &GicdIgroupr1 {
        &self.gicd_igroupr1
    }
    #[doc = "0x08 - Interrupt Group"]
    #[inline(always)]
    pub const fn gicd_igroupr2(&self) -> &GicdIgroupr2 {
        &self.gicd_igroupr2
    }
    #[doc = "0x0c - Interrupt Group"]
    #[inline(always)]
    pub const fn gicd_igroupr3(&self) -> &GicdIgroupr3 {
        &self.gicd_igroupr3
    }
    #[doc = "0x10 - Interrupt Group"]
    #[inline(always)]
    pub const fn gicd_igroupr4(&self) -> &GicdIgroupr4 {
        &self.gicd_igroupr4
    }
    #[doc = "0x14 - Interrupt Group"]
    #[inline(always)]
    pub const fn gicd_igroupr5(&self) -> &GicdIgroupr5 {
        &self.gicd_igroupr5
    }
    #[doc = "0x18 - Interrupt Group"]
    #[inline(always)]
    pub const fn gicd_igroupr6(&self) -> &GicdIgroupr6 {
        &self.gicd_igroupr6
    }
}
#[doc = "GICD_IGROUPR0 (rw) register accessor: Interrupt Group\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_igroupr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_igroupr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_igroupr0`]
module"]
#[doc(alias = "GICD_IGROUPR0")]
pub type GicdIgroupr0 = crate::Reg<gicd_igroupr0::GicdIgroupr0Spec>;
#[doc = "Interrupt Group"]
pub mod gicd_igroupr0;
#[doc = "GICD_IGROUPR1 (rw) register accessor: Interrupt Group\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_igroupr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_igroupr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_igroupr1`]
module"]
#[doc(alias = "GICD_IGROUPR1")]
pub type GicdIgroupr1 = crate::Reg<gicd_igroupr1::GicdIgroupr1Spec>;
#[doc = "Interrupt Group"]
pub mod gicd_igroupr1;
#[doc = "GICD_IGROUPR2 (rw) register accessor: Interrupt Group\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_igroupr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_igroupr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_igroupr2`]
module"]
#[doc(alias = "GICD_IGROUPR2")]
pub type GicdIgroupr2 = crate::Reg<gicd_igroupr2::GicdIgroupr2Spec>;
#[doc = "Interrupt Group"]
pub mod gicd_igroupr2;
#[doc = "GICD_IGROUPR3 (rw) register accessor: Interrupt Group\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_igroupr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_igroupr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_igroupr3`]
module"]
#[doc(alias = "GICD_IGROUPR3")]
pub type GicdIgroupr3 = crate::Reg<gicd_igroupr3::GicdIgroupr3Spec>;
#[doc = "Interrupt Group"]
pub mod gicd_igroupr3;
#[doc = "GICD_IGROUPR4 (rw) register accessor: Interrupt Group\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_igroupr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_igroupr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_igroupr4`]
module"]
#[doc(alias = "GICD_IGROUPR4")]
pub type GicdIgroupr4 = crate::Reg<gicd_igroupr4::GicdIgroupr4Spec>;
#[doc = "Interrupt Group"]
pub mod gicd_igroupr4;
#[doc = "GICD_IGROUPR5 (rw) register accessor: Interrupt Group\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_igroupr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_igroupr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_igroupr5`]
module"]
#[doc(alias = "GICD_IGROUPR5")]
pub type GicdIgroupr5 = crate::Reg<gicd_igroupr5::GicdIgroupr5Spec>;
#[doc = "Interrupt Group"]
pub mod gicd_igroupr5;
#[doc = "GICD_IGROUPR6 (rw) register accessor: Interrupt Group\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_igroupr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_igroupr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_igroupr6`]
module"]
#[doc(alias = "GICD_IGROUPR6")]
pub type GicdIgroupr6 = crate::Reg<gicd_igroupr6::GicdIgroupr6Spec>;
#[doc = "Interrupt Group"]
pub mod gicd_igroupr6;

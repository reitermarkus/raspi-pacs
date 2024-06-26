#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cs: Cs,
    clo: Clo,
    chi: Chi,
    c0: C0,
    c1: C1,
    c2: C2,
    c3: C3,
}
impl RegisterBlock {
    #[doc = "0x00 - Control / Status"]
    #[inline(always)]
    pub const fn cs(&self) -> &Cs {
        &self.cs
    }
    #[doc = "0x04 - Lower 32 bits for the free running counter"]
    #[inline(always)]
    pub const fn clo(&self) -> &Clo {
        &self.clo
    }
    #[doc = "0x08 - Higher 32 bits for the free running counter"]
    #[inline(always)]
    pub const fn chi(&self) -> &Chi {
        &self.chi
    }
    #[doc = "0x0c - Compare channel 0"]
    #[inline(always)]
    pub const fn c0(&self) -> &C0 {
        &self.c0
    }
    #[doc = "0x10 - Compare channel 1"]
    #[inline(always)]
    pub const fn c1(&self) -> &C1 {
        &self.c1
    }
    #[doc = "0x14 - Compare channel 2"]
    #[inline(always)]
    pub const fn c2(&self) -> &C2 {
        &self.c2
    }
    #[doc = "0x18 - Compare channel 3"]
    #[inline(always)]
    pub const fn c3(&self) -> &C3 {
        &self.c3
    }
}
#[doc = "CS (rw) register accessor: Control / Status\n\nYou can [`read`](crate::Reg::read) this register and get [`cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs`]
module"]
#[doc(alias = "CS")]
pub type Cs = crate::Reg<cs::CsSpec>;
#[doc = "Control / Status"]
pub mod cs;
#[doc = "CLO (r) register accessor: Lower 32 bits for the free running counter\n\nYou can [`read`](crate::Reg::read) this register and get [`clo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clo`]
module"]
#[doc(alias = "CLO")]
pub type Clo = crate::Reg<clo::CloSpec>;
#[doc = "Lower 32 bits for the free running counter"]
pub mod clo;
#[doc = "CHI (r) register accessor: Higher 32 bits for the free running counter\n\nYou can [`read`](crate::Reg::read) this register and get [`chi::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chi`]
module"]
#[doc(alias = "CHI")]
pub type Chi = crate::Reg<chi::ChiSpec>;
#[doc = "Higher 32 bits for the free running counter"]
pub mod chi;
#[doc = "C0 (rw) register accessor: Compare channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c0`]
module"]
pub type C0 = crate::Reg<c0::C0Spec>;
#[doc = "Compare channel 0"]
pub mod c0;
#[doc = "C1 (rw) register accessor: Compare channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`c1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1`]
module"]
pub type C1 = crate::Reg<c1::C1Spec>;
#[doc = "Compare channel 1"]
pub mod c1;
#[doc = "C2 (rw) register accessor: Compare channel 2\n\nYou can [`read`](crate::Reg::read) this register and get [`c2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2`]
module"]
pub type C2 = crate::Reg<c2::C2Spec>;
#[doc = "Compare channel 2"]
pub mod c2;
#[doc = "C3 (rw) register accessor: Compare channel 3\n\nYou can [`read`](crate::Reg::read) this register and get [`c3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c3`]
module"]
pub type C3 = crate::Reg<c3::C3Spec>;
#[doc = "Compare channel 3"]
pub mod c3;

#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0200],
    basic_pending: BasicPending,
    pending_1: Pending1,
    pending_2: Pending2,
    fiq_control: FiqControl,
    enable_1: Enable1,
    enable_2: Enable2,
    enable_basic: EnableBasic,
    disable_1: Disable1,
    disable_2: Disable2,
    disable_basic: DisableBasic,
}
impl RegisterBlock {
    #[doc = "0x200 - Basic pending info"]
    #[inline(always)]
    pub const fn basic_pending(&self) -> &BasicPending {
        &self.basic_pending
    }
    #[doc = "0x204 - Pending state for interrupts 1 - 31"]
    #[inline(always)]
    pub const fn pending_1(&self) -> &Pending1 {
        &self.pending_1
    }
    #[doc = "0x208 - Pending state for interrupts 32 - 63"]
    #[inline(always)]
    pub const fn pending_2(&self) -> &Pending2 {
        &self.pending_2
    }
    #[doc = "0x20c - FIQ control"]
    #[inline(always)]
    pub const fn fiq_control(&self) -> &FiqControl {
        &self.fiq_control
    }
    #[doc = "0x210 - Enable interrupts 1 - 31"]
    #[inline(always)]
    pub const fn enable_1(&self) -> &Enable1 {
        &self.enable_1
    }
    #[doc = "0x214 - Enable interrupts 32 - 63"]
    #[inline(always)]
    pub const fn enable_2(&self) -> &Enable2 {
        &self.enable_2
    }
    #[doc = "0x218 - Enable basic interrupts"]
    #[inline(always)]
    pub const fn enable_basic(&self) -> &EnableBasic {
        &self.enable_basic
    }
    #[doc = "0x21c - Disable interrupts 1 - 31"]
    #[inline(always)]
    pub const fn disable_1(&self) -> &Disable1 {
        &self.disable_1
    }
    #[doc = "0x220 - Disable interrupts 32 - 63"]
    #[inline(always)]
    pub const fn disable_2(&self) -> &Disable2 {
        &self.disable_2
    }
    #[doc = "0x224 - Disable basic interrupts"]
    #[inline(always)]
    pub const fn disable_basic(&self) -> &DisableBasic {
        &self.disable_basic
    }
}
#[doc = "BASIC_PENDING (r) register accessor: Basic pending info\n\nYou can [`read`](crate::Reg::read) this register and get [`basic_pending::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@basic_pending`]
module"]
#[doc(alias = "BASIC_PENDING")]
pub type BasicPending = crate::Reg<basic_pending::BasicPendingSpec>;
#[doc = "Basic pending info"]
pub mod basic_pending;
#[doc = "PENDING_1 (r) register accessor: Pending state for interrupts 1 - 31\n\nYou can [`read`](crate::Reg::read) this register and get [`pending_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pending_1`]
module"]
#[doc(alias = "PENDING_1")]
pub type Pending1 = crate::Reg<pending_1::Pending1Spec>;
#[doc = "Pending state for interrupts 1 - 31"]
pub mod pending_1;
#[doc = "PENDING_2 (r) register accessor: Pending state for interrupts 32 - 63\n\nYou can [`read`](crate::Reg::read) this register and get [`pending_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pending_2`]
module"]
#[doc(alias = "PENDING_2")]
pub type Pending2 = crate::Reg<pending_2::Pending2Spec>;
#[doc = "Pending state for interrupts 32 - 63"]
pub mod pending_2;
#[doc = "FIQ_CONTROL (rw) register accessor: FIQ control\n\nYou can [`read`](crate::Reg::read) this register and get [`fiq_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiq_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fiq_control`]
module"]
#[doc(alias = "FIQ_CONTROL")]
pub type FiqControl = crate::Reg<fiq_control::FiqControlSpec>;
#[doc = "FIQ control"]
pub mod fiq_control;
#[doc = "ENABLE_1 (rw) register accessor: Enable interrupts 1 - 31\n\nYou can [`read`](crate::Reg::read) this register and get [`enable_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable_1`]
module"]
#[doc(alias = "ENABLE_1")]
pub type Enable1 = crate::Reg<enable_1::Enable1Spec>;
#[doc = "Enable interrupts 1 - 31"]
pub mod enable_1;
#[doc = "ENABLE_2 (rw) register accessor: Enable interrupts 32 - 63\n\nYou can [`read`](crate::Reg::read) this register and get [`enable_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable_2`]
module"]
#[doc(alias = "ENABLE_2")]
pub type Enable2 = crate::Reg<enable_2::Enable2Spec>;
#[doc = "Enable interrupts 32 - 63"]
pub mod enable_2;
#[doc = "ENABLE_BASIC (rw) register accessor: Enable basic interrupts\n\nYou can [`read`](crate::Reg::read) this register and get [`enable_basic::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable_basic::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable_basic`]
module"]
#[doc(alias = "ENABLE_BASIC")]
pub type EnableBasic = crate::Reg<enable_basic::EnableBasicSpec>;
#[doc = "Enable basic interrupts"]
pub mod enable_basic;
#[doc = "DISABLE_1 (rw) register accessor: Disable interrupts 1 - 31\n\nYou can [`read`](crate::Reg::read) this register and get [`disable_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`disable_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@disable_1`]
module"]
#[doc(alias = "DISABLE_1")]
pub type Disable1 = crate::Reg<disable_1::Disable1Spec>;
#[doc = "Disable interrupts 1 - 31"]
pub mod disable_1;
#[doc = "DISABLE_2 (rw) register accessor: Disable interrupts 32 - 63\n\nYou can [`read`](crate::Reg::read) this register and get [`disable_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`disable_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@disable_2`]
module"]
#[doc(alias = "DISABLE_2")]
pub type Disable2 = crate::Reg<disable_2::Disable2Spec>;
#[doc = "Disable interrupts 32 - 63"]
pub mod disable_2;
#[doc = "DISABLE_BASIC (rw) register accessor: Disable basic interrupts\n\nYou can [`read`](crate::Reg::read) this register and get [`disable_basic::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`disable_basic::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@disable_basic`]
module"]
#[doc(alias = "DISABLE_BASIC")]
pub type DisableBasic = crate::Reg<disable_basic::DisableBasicSpec>;
#[doc = "Disable basic interrupts"]
pub mod disable_basic;

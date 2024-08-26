#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl: Ctl,
    div: Div,
}
impl RegisterBlock {
    #[doc = "0x00 - Control"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x04 - Clock divisor"]
    #[inline(always)]
    pub const fn div(&self) -> &Div {
        &self.div
    }
}
#[doc = "CTL (rw) register accessor: Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Control"]
pub mod ctl;
#[doc = "DIV (rw) register accessor: Clock divisor\n\nYou can [`read`](crate::Reg::read) this register and get [`div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@div`]
module"]
#[doc(alias = "DIV")]
pub type Div = crate::Reg<div::DivSpec>;
#[doc = "Clock divisor"]
pub mod div;

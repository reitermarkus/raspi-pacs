#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cs: Cs,
    div: Div,
}
impl RegisterBlock {
    #[doc = "0x00 - Control / Status"]
    #[inline(always)]
    pub const fn cs(&self) -> &Cs {
        &self.cs
    }
    #[doc = "0x04 - Clock divisor"]
    #[inline(always)]
    pub const fn div(&self) -> &Div {
        &self.div
    }
}
#[doc = "CS (rw) register accessor: Control / Status\n\nYou can [`read`](crate::Reg::read) this register and get [`cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs`]
module"]
#[doc(alias = "CS")]
pub type Cs = crate::Reg<cs::CsSpec>;
#[doc = "Control / Status"]
pub mod cs;
#[doc = "DIV (rw) register accessor: Clock divisor\n\nYou can [`read`](crate::Reg::read) this register and get [`div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@div`]
module"]
#[doc(alias = "DIV")]
pub type Div = crate::Reg<div::DivSpec>;
#[doc = "Clock divisor"]
pub mod div;

#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cs: Cs,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt status"]
    #[inline(always)]
    pub const fn cs(&self) -> &Cs {
        &self.cs
    }
}
#[doc = "CS (rw) register accessor: Interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs`]
module"]
#[doc(alias = "CS")]
pub type Cs = crate::Reg<cs::CsSpec>;
#[doc = "Interrupt status"]
pub mod cs;

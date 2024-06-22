#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    irq: Irq,
    enables: Enables,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt status"]
    #[inline(always)]
    pub const fn irq(&self) -> &Irq {
        &self.irq
    }
    #[doc = "0x04 - Enable sub-peripherals"]
    #[inline(always)]
    pub const fn enables(&self) -> &Enables {
        &self.enables
    }
}
#[doc = "IRQ (rw) register accessor: Interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`irq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq`]
module"]
#[doc(alias = "IRQ")]
pub type Irq = crate::Reg<irq::IrqSpec>;
#[doc = "Interrupt status"]
#[path = "aux_/irq.rs"]
pub mod irq;
#[doc = "ENABLES (rw) register accessor: Enable sub-peripherals\n\nYou can [`read`](crate::Reg::read) this register and get [`enables::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enables::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enables`]
module"]
#[doc(alias = "ENABLES")]
pub type Enables = crate::Reg<enables::EnablesSpec>;
#[doc = "Enable sub-peripherals"]
#[path = "aux_/enables.rs"]
pub mod enables;

#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1c],
    rstc: Rstc,
    _reserved1: [u8; 0x04],
    wdog: Wdog,
}
impl RegisterBlock {
    #[doc = "0x1c - Reset Control"]
    #[inline(always)]
    pub const fn rstc(&self) -> &Rstc {
        &self.rstc
    }
    #[doc = "0x24 - Watchdog control"]
    #[inline(always)]
    pub const fn wdog(&self) -> &Wdog {
        &self.wdog
    }
}
#[doc = "RSTC (rw) register accessor: Reset Control\n\nYou can [`read`](crate::Reg::read) this register and get [`rstc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstc`]
module"]
#[doc(alias = "RSTC")]
pub type Rstc = crate::Reg<rstc::RstcSpec>;
#[doc = "Reset Control"]
pub mod rstc;
#[doc = "WDOG (rw) register accessor: Watchdog control\n\nYou can [`read`](crate::Reg::read) this register and get [`wdog::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdog::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdog`]
module"]
#[doc(alias = "WDOG")]
pub type Wdog = crate::Reg<wdog::WdogSpec>;
#[doc = "Watchdog control"]
pub mod wdog;

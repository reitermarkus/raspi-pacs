#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cs: Cs,
    fifo: Fifo,
    clk: Clk,
    dlen: Dlen,
    ltoh: Ltoh,
    dc: Dc,
}
impl RegisterBlock {
    #[doc = "0x00 - Control and Status"]
    #[inline(always)]
    pub const fn cs(&self) -> &Cs {
        &self.cs
    }
    #[doc = "0x04 - FIFO access"]
    #[inline(always)]
    pub const fn fifo(&self) -> &Fifo {
        &self.fifo
    }
    #[doc = "0x08 - Clock divider"]
    #[inline(always)]
    pub const fn clk(&self) -> &Clk {
        &self.clk
    }
    #[doc = "0x0c - Data length"]
    #[inline(always)]
    pub const fn dlen(&self) -> &Dlen {
        &self.dlen
    }
    #[doc = "0x10 - LoSSI output hold delay"]
    #[inline(always)]
    pub const fn ltoh(&self) -> &Ltoh {
        &self.ltoh
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn dc(&self) -> &Dc {
        &self.dc
    }
}
#[doc = "CS (rw) register accessor: Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs`]
module"]
#[doc(alias = "CS")]
pub type Cs = crate::Reg<cs::CsSpec>;
#[doc = "Control and Status"]
pub mod cs;
#[doc = "FIFO (rw) register accessor: FIFO access\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo`]
module"]
#[doc(alias = "FIFO")]
pub type Fifo = crate::Reg<fifo::FifoSpec>;
#[doc = "FIFO access"]
pub mod fifo;
#[doc = "CLK (rw) register accessor: Clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk`]
module"]
#[doc(alias = "CLK")]
pub type Clk = crate::Reg<clk::ClkSpec>;
#[doc = "Clock divider"]
pub mod clk;
#[doc = "DLEN (rw) register accessor: Data length\n\nYou can [`read`](crate::Reg::read) this register and get [`dlen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dlen`]
module"]
#[doc(alias = "DLEN")]
pub type Dlen = crate::Reg<dlen::DlenSpec>;
#[doc = "Data length"]
pub mod dlen;
#[doc = "LTOH (rw) register accessor: LoSSI output hold delay\n\nYou can [`read`](crate::Reg::read) this register and get [`ltoh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltoh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ltoh`]
module"]
#[doc(alias = "LTOH")]
pub type Ltoh = crate::Reg<ltoh::LtohSpec>;
#[doc = "LoSSI output hold delay"]
pub mod ltoh;
#[doc = "DC (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc`]
module"]
#[doc(alias = "DC")]
pub type Dc = crate::Reg<dc::DcSpec>;
#[doc = ""]
pub mod dc;

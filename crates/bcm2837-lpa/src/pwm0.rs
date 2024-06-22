#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl: Ctl,
    sta: Sta,
    dmac: Dmac,
    _reserved3: [u8; 0x04],
    rng1: Rng1,
    dat1: Dat1,
    fif1: Fif1,
    _reserved6: [u8; 0x04],
    rng2: Rng2,
    dat2: Dat2,
}
impl RegisterBlock {
    #[doc = "0x00 - Control"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x04 - Status"]
    #[inline(always)]
    pub const fn sta(&self) -> &Sta {
        &self.sta
    }
    #[doc = "0x08 - DMA control"]
    #[inline(always)]
    pub const fn dmac(&self) -> &Dmac {
        &self.dmac
    }
    #[doc = "0x10 - Range for channel 1"]
    #[inline(always)]
    pub const fn rng1(&self) -> &Rng1 {
        &self.rng1
    }
    #[doc = "0x14 - Channel 1 data"]
    #[inline(always)]
    pub const fn dat1(&self) -> &Dat1 {
        &self.dat1
    }
    #[doc = "0x18 - FIFO input"]
    #[inline(always)]
    pub const fn fif1(&self) -> &Fif1 {
        &self.fif1
    }
    #[doc = "0x20 - Range for channel 2"]
    #[inline(always)]
    pub const fn rng2(&self) -> &Rng2 {
        &self.rng2
    }
    #[doc = "0x24 - Channel 2 data"]
    #[inline(always)]
    pub const fn dat2(&self) -> &Dat2 {
        &self.dat2
    }
}
#[doc = "CTL (rw) register accessor: Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Control"]
pub mod ctl;
#[doc = "STA (rw) register accessor: Status\n\nYou can [`read`](crate::Reg::read) this register and get [`sta::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sta::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sta`]
module"]
#[doc(alias = "STA")]
pub type Sta = crate::Reg<sta::StaSpec>;
#[doc = "Status"]
pub mod sta;
#[doc = "DMAC (rw) register accessor: DMA control\n\nYou can [`read`](crate::Reg::read) this register and get [`dmac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac`]
module"]
#[doc(alias = "DMAC")]
pub type Dmac = crate::Reg<dmac::DmacSpec>;
#[doc = "DMA control"]
pub mod dmac;
#[doc = "RNG1 (rw) register accessor: Range for channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rng1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng1`]
module"]
#[doc(alias = "RNG1")]
pub type Rng1 = crate::Reg<rng1::Rng1Spec>;
#[doc = "Range for channel 1"]
pub mod rng1;
#[doc = "DAT1 (rw) register accessor: Channel 1 data\n\nYou can [`read`](crate::Reg::read) this register and get [`dat1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dat1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat1`]
module"]
#[doc(alias = "DAT1")]
pub type Dat1 = crate::Reg<dat1::Dat1Spec>;
#[doc = "Channel 1 data"]
pub mod dat1;
#[doc = "FIF1 (w) register accessor: FIFO input\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fif1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fif1`]
module"]
#[doc(alias = "FIF1")]
pub type Fif1 = crate::Reg<fif1::Fif1Spec>;
#[doc = "FIFO input"]
pub mod fif1;
#[doc = "RNG2 (rw) register accessor: Range for channel 2\n\nYou can [`read`](crate::Reg::read) this register and get [`rng2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng2`]
module"]
#[doc(alias = "RNG2")]
pub type Rng2 = crate::Reg<rng2::Rng2Spec>;
#[doc = "Range for channel 2"]
pub mod rng2;
#[doc = "DAT2 (rw) register accessor: Channel 2 data\n\nYou can [`read`](crate::Reg::read) this register and get [`dat2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dat2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat2`]
module"]
#[doc(alias = "DAT2")]
pub type Dat2 = crate::Reg<dat2::Dat2Spec>;
#[doc = "Channel 2 data"]
pub mod dat2;

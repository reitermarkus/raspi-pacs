#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    hcfg: Hcfg,
    hfir: Hfir,
    hfnum: Hfnum,
    _reserved3: [u8; 0x04],
    hptxsts: Hptxsts,
    haint: Haint,
    haintmsk: Haintmsk,
    _reserved6: [u8; 0x24],
    hprt: Hprt,
    _reserved7: [u8; 0xbc],
    host_channel: (),
}
impl RegisterBlock {
    #[doc = "0x00 - OTG_HS host configuration register"]
    #[inline(always)]
    pub const fn hcfg(&self) -> &Hcfg {
        &self.hcfg
    }
    #[doc = "0x04 - OTG_HS Host frame interval register"]
    #[inline(always)]
    pub const fn hfir(&self) -> &Hfir {
        &self.hfir
    }
    #[doc = "0x08 - OTG_HS host frame number/frame time remaining register"]
    #[inline(always)]
    pub const fn hfnum(&self) -> &Hfnum {
        &self.hfnum
    }
    #[doc = "0x10 - Host periodic transmit FIFO/queue status register"]
    #[inline(always)]
    pub const fn hptxsts(&self) -> &Hptxsts {
        &self.hptxsts
    }
    #[doc = "0x14 - OTG_HS Host all channels interrupt register"]
    #[inline(always)]
    pub const fn haint(&self) -> &Haint {
        &self.haint
    }
    #[doc = "0x18 - OTG_HS host all channels interrupt mask register"]
    #[inline(always)]
    pub const fn haintmsk(&self) -> &Haintmsk {
        &self.haintmsk
    }
    #[doc = "0x40 - OTG_HS host port control and status register"]
    #[inline(always)]
    pub const fn hprt(&self) -> &Hprt {
        &self.hprt
    }
    #[doc = "0x100..0x220 - Host channel %s"]
    #[inline(always)]
    pub const fn host_channel(&self, n: usize) -> &HostChannel {
        #[allow(clippy::no_effect)]
        [(); 12][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(256)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x220 - Host channel %s"]
    #[inline(always)]
    pub fn host_channel_iter(&self) -> impl Iterator<Item = &HostChannel> {
        (0..12).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(256)
                .add(32 * n)
                .cast()
        })
    }
}
#[doc = "HCFG (rw) register accessor: OTG_HS host configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcfg`]
module"]
#[doc(alias = "HCFG")]
pub type Hcfg = crate::Reg<hcfg::HcfgSpec>;
#[doc = "OTG_HS host configuration register"]
pub mod hcfg;
#[doc = "HFIR (rw) register accessor: OTG_HS Host frame interval register\n\nYou can [`read`](crate::Reg::read) this register and get [`hfir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfir`]
module"]
#[doc(alias = "HFIR")]
pub type Hfir = crate::Reg<hfir::HfirSpec>;
#[doc = "OTG_HS Host frame interval register"]
pub mod hfir;
#[doc = "HFNUM (r) register accessor: OTG_HS host frame number/frame time remaining register\n\nYou can [`read`](crate::Reg::read) this register and get [`hfnum::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfnum`]
module"]
#[doc(alias = "HFNUM")]
pub type Hfnum = crate::Reg<hfnum::HfnumSpec>;
#[doc = "OTG_HS host frame number/frame time remaining register"]
pub mod hfnum;
#[doc = "HPTXSTS (rw) register accessor: Host periodic transmit FIFO/queue status register\n\nYou can [`read`](crate::Reg::read) this register and get [`hptxsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hptxsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hptxsts`]
module"]
#[doc(alias = "HPTXSTS")]
pub type Hptxsts = crate::Reg<hptxsts::HptxstsSpec>;
#[doc = "Host periodic transmit FIFO/queue status register"]
pub mod hptxsts;
#[doc = "HAINT (r) register accessor: OTG_HS Host all channels interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`haint::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@haint`]
module"]
#[doc(alias = "HAINT")]
pub type Haint = crate::Reg<haint::HaintSpec>;
#[doc = "OTG_HS Host all channels interrupt register"]
pub mod haint;
#[doc = "HAINTMSK (rw) register accessor: OTG_HS host all channels interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`haintmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`haintmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@haintmsk`]
module"]
#[doc(alias = "HAINTMSK")]
pub type Haintmsk = crate::Reg<haintmsk::HaintmskSpec>;
#[doc = "OTG_HS host all channels interrupt mask register"]
pub mod haintmsk;
#[doc = "HPRT (rw) register accessor: OTG_HS host port control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`hprt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hprt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hprt`]
module"]
#[doc(alias = "HPRT")]
pub type Hprt = crate::Reg<hprt::HprtSpec>;
#[doc = "OTG_HS host port control and status register"]
pub mod hprt;
#[doc = "Host channel %s"]
pub use self::host_channel::HostChannel;
#[doc = r"Cluster"]
#[doc = "Host channel %s"]
pub mod host_channel;

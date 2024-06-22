#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dcfg: Dcfg,
    dctl: Dctl,
    dsts: Dsts,
    _reserved3: [u8; 0x04],
    diepmsk: Diepmsk,
    doepmsk: Doepmsk,
    daint: Daint,
    daintmsk: Daintmsk,
    _reserved7: [u8; 0x08],
    dvbusdis: Dvbusdis,
    dvbuspulse: Dvbuspulse,
    dthrctl: Dthrctl,
    diepempmsk: Diepempmsk,
    deachint: Deachint,
    deachintmsk: Deachintmsk,
    diepeachmsk1: Diepeachmsk1,
    _reserved14: [u8; 0x3c],
    doepeachmsk1: Doepeachmsk1,
    _reserved15: [u8; 0x7c],
    in_endpoint: (),
    _reserved16: [u8; 0x0200],
    out_endpoint: (),
}
impl RegisterBlock {
    #[doc = "0x00 - OTG_HS device configuration register"]
    #[inline(always)]
    pub const fn dcfg(&self) -> &Dcfg {
        &self.dcfg
    }
    #[doc = "0x04 - OTG_HS device control register"]
    #[inline(always)]
    pub const fn dctl(&self) -> &Dctl {
        &self.dctl
    }
    #[doc = "0x08 - OTG_HS device status register"]
    #[inline(always)]
    pub const fn dsts(&self) -> &Dsts {
        &self.dsts
    }
    #[doc = "0x10 - OTG_HS device IN endpoint common interrupt mask register"]
    #[inline(always)]
    pub const fn diepmsk(&self) -> &Diepmsk {
        &self.diepmsk
    }
    #[doc = "0x14 - OTG_HS device OUT endpoint common interrupt mask register"]
    #[inline(always)]
    pub const fn doepmsk(&self) -> &Doepmsk {
        &self.doepmsk
    }
    #[doc = "0x18 - OTG_HS device all endpoints interrupt register"]
    #[inline(always)]
    pub const fn daint(&self) -> &Daint {
        &self.daint
    }
    #[doc = "0x1c - OTG_HS all endpoints interrupt mask register"]
    #[inline(always)]
    pub const fn daintmsk(&self) -> &Daintmsk {
        &self.daintmsk
    }
    #[doc = "0x28 - OTG_HS device VBUS discharge time register"]
    #[inline(always)]
    pub const fn dvbusdis(&self) -> &Dvbusdis {
        &self.dvbusdis
    }
    #[doc = "0x2c - OTG_HS device VBUS pulsing time register"]
    #[inline(always)]
    pub const fn dvbuspulse(&self) -> &Dvbuspulse {
        &self.dvbuspulse
    }
    #[doc = "0x30 - OTG_HS Device threshold control register"]
    #[inline(always)]
    pub const fn dthrctl(&self) -> &Dthrctl {
        &self.dthrctl
    }
    #[doc = "0x34 - OTG_HS device IN endpoint FIFO empty interrupt mask register"]
    #[inline(always)]
    pub const fn diepempmsk(&self) -> &Diepempmsk {
        &self.diepempmsk
    }
    #[doc = "0x38 - OTG_HS device each endpoint interrupt register"]
    #[inline(always)]
    pub const fn deachint(&self) -> &Deachint {
        &self.deachint
    }
    #[doc = "0x3c - OTG_HS device each endpoint interrupt register mask"]
    #[inline(always)]
    pub const fn deachintmsk(&self) -> &Deachintmsk {
        &self.deachintmsk
    }
    #[doc = "0x40 - OTG_HS device each in endpoint-1 interrupt register"]
    #[inline(always)]
    pub const fn diepeachmsk1(&self) -> &Diepeachmsk1 {
        &self.diepeachmsk1
    }
    #[doc = "0x80 - OTG_HS device each OUT endpoint-1 interrupt register"]
    #[inline(always)]
    pub const fn doepeachmsk1(&self) -> &Doepeachmsk1 {
        &self.doepeachmsk1
    }
    #[doc = "0x100..0x250 - IN Endpoint %s"]
    #[inline(always)]
    pub const fn in_endpoint(&self, n: usize) -> &InEndpoint {
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
    #[doc = "0x100..0x250 - IN Endpoint %s"]
    #[inline(always)]
    pub fn in_endpoint_iter(&self) -> impl Iterator<Item = &InEndpoint> {
        (0..12).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(256)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x300..0x420 - OUT Endpoint %s"]
    #[inline(always)]
    pub const fn out_endpoint(&self, n: usize) -> &OutEndpoint {
        #[allow(clippy::no_effect)]
        [(); 12][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(768)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x300..0x420 - OUT Endpoint %s"]
    #[inline(always)]
    pub fn out_endpoint_iter(&self) -> impl Iterator<Item = &OutEndpoint> {
        (0..12).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(768)
                .add(32 * n)
                .cast()
        })
    }
}
#[doc = "DCFG (rw) register accessor: OTG_HS device configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcfg`]
module"]
#[doc(alias = "DCFG")]
pub type Dcfg = crate::Reg<dcfg::DcfgSpec>;
#[doc = "OTG_HS device configuration register"]
pub mod dcfg;
#[doc = "DCTL (rw) register accessor: OTG_HS device control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dctl`]
module"]
#[doc(alias = "DCTL")]
pub type Dctl = crate::Reg<dctl::DctlSpec>;
#[doc = "OTG_HS device control register"]
pub mod dctl;
#[doc = "DSTS (r) register accessor: OTG_HS device status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsts`]
module"]
#[doc(alias = "DSTS")]
pub type Dsts = crate::Reg<dsts::DstsSpec>;
#[doc = "OTG_HS device status register"]
pub mod dsts;
#[doc = "DIEPMSK (rw) register accessor: OTG_HS device IN endpoint common interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`diepmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepmsk`]
module"]
#[doc(alias = "DIEPMSK")]
pub type Diepmsk = crate::Reg<diepmsk::DiepmskSpec>;
#[doc = "OTG_HS device IN endpoint common interrupt mask register"]
pub mod diepmsk;
#[doc = "DOEPMSK (rw) register accessor: OTG_HS device OUT endpoint common interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`doepmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepmsk`]
module"]
#[doc(alias = "DOEPMSK")]
pub type Doepmsk = crate::Reg<doepmsk::DoepmskSpec>;
#[doc = "OTG_HS device OUT endpoint common interrupt mask register"]
pub mod doepmsk;
#[doc = "DAINT (r) register accessor: OTG_HS device all endpoints interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`daint::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daint`]
module"]
#[doc(alias = "DAINT")]
pub type Daint = crate::Reg<daint::DaintSpec>;
#[doc = "OTG_HS device all endpoints interrupt register"]
pub mod daint;
#[doc = "DAINTMSK (rw) register accessor: OTG_HS all endpoints interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`daintmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daintmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daintmsk`]
module"]
#[doc(alias = "DAINTMSK")]
pub type Daintmsk = crate::Reg<daintmsk::DaintmskSpec>;
#[doc = "OTG_HS all endpoints interrupt mask register"]
pub mod daintmsk;
#[doc = "DVBUSDIS (rw) register accessor: OTG_HS device VBUS discharge time register\n\nYou can [`read`](crate::Reg::read) this register and get [`dvbusdis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvbusdis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvbusdis`]
module"]
#[doc(alias = "DVBUSDIS")]
pub type Dvbusdis = crate::Reg<dvbusdis::DvbusdisSpec>;
#[doc = "OTG_HS device VBUS discharge time register"]
pub mod dvbusdis;
#[doc = "DVBUSPULSE (rw) register accessor: OTG_HS device VBUS pulsing time register\n\nYou can [`read`](crate::Reg::read) this register and get [`dvbuspulse::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvbuspulse::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvbuspulse`]
module"]
#[doc(alias = "DVBUSPULSE")]
pub type Dvbuspulse = crate::Reg<dvbuspulse::DvbuspulseSpec>;
#[doc = "OTG_HS device VBUS pulsing time register"]
pub mod dvbuspulse;
#[doc = "DTHRCTL (rw) register accessor: OTG_HS Device threshold control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dthrctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dthrctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dthrctl`]
module"]
#[doc(alias = "DTHRCTL")]
pub type Dthrctl = crate::Reg<dthrctl::DthrctlSpec>;
#[doc = "OTG_HS Device threshold control register"]
pub mod dthrctl;
#[doc = "DIEPEMPMSK (rw) register accessor: OTG_HS device IN endpoint FIFO empty interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`diepempmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepempmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepempmsk`]
module"]
#[doc(alias = "DIEPEMPMSK")]
pub type Diepempmsk = crate::Reg<diepempmsk::DiepempmskSpec>;
#[doc = "OTG_HS device IN endpoint FIFO empty interrupt mask register"]
pub mod diepempmsk;
#[doc = "DEACHINT (rw) register accessor: OTG_HS device each endpoint interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`deachint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deachint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deachint`]
module"]
#[doc(alias = "DEACHINT")]
pub type Deachint = crate::Reg<deachint::DeachintSpec>;
#[doc = "OTG_HS device each endpoint interrupt register"]
pub mod deachint;
#[doc = "DEACHINTMSK (rw) register accessor: OTG_HS device each endpoint interrupt register mask\n\nYou can [`read`](crate::Reg::read) this register and get [`deachintmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deachintmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deachintmsk`]
module"]
#[doc(alias = "DEACHINTMSK")]
pub type Deachintmsk = crate::Reg<deachintmsk::DeachintmskSpec>;
#[doc = "OTG_HS device each endpoint interrupt register mask"]
pub mod deachintmsk;
#[doc = "DIEPEACHMSK1 (rw) register accessor: OTG_HS device each in endpoint-1 interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`diepeachmsk1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepeachmsk1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepeachmsk1`]
module"]
#[doc(alias = "DIEPEACHMSK1")]
pub type Diepeachmsk1 = crate::Reg<diepeachmsk1::Diepeachmsk1Spec>;
#[doc = "OTG_HS device each in endpoint-1 interrupt register"]
pub mod diepeachmsk1;
#[doc = "DOEPEACHMSK1 (rw) register accessor: OTG_HS device each OUT endpoint-1 interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`doepeachmsk1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepeachmsk1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepeachmsk1`]
module"]
#[doc(alias = "DOEPEACHMSK1")]
pub type Doepeachmsk1 = crate::Reg<doepeachmsk1::Doepeachmsk1Spec>;
#[doc = "OTG_HS device each OUT endpoint-1 interrupt register"]
pub mod doepeachmsk1;
#[doc = "IN Endpoint %s"]
pub use self::in_endpoint::InEndpoint;
#[doc = r"Cluster"]
#[doc = "IN Endpoint %s"]
pub mod in_endpoint;
#[doc = "OUT Endpoint %s"]
pub use self::out_endpoint::OutEndpoint;
#[doc = r"Cluster"]
#[doc = "OUT Endpoint %s"]
pub mod out_endpoint;

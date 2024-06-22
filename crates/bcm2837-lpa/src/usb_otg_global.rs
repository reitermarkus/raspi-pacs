#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gotgctl: Gotgctl,
    gotgint: Gotgint,
    gahbcfg: Gahbcfg,
    gusbcfg: Gusbcfg,
    grstctl: Grstctl,
    gintsts: Gintsts,
    gintmsk: Gintmsk,
    _reserved_7_grxstsr: [u8; 0x04],
    _reserved_8_grxstsp: [u8; 0x04],
    grxfsiz: Grxfsiz,
    _reserved_10_gnptxfsiz_host: [u8; 0x04],
    gnptxsts: Gnptxsts,
    _reserved12: [u8; 0x08],
    gccfg: Gccfg,
    cid: Cid,
    vid: Vid,
    hw_direction: HwDirection,
    hw_config0: HwConfig0,
    _reserved17: [u8; 0xb4],
    hptxfsiz: Hptxfsiz,
    dieptxf1: Dieptxf1,
    dieptxf2: Dieptxf2,
    _reserved20: [u8; 0x10],
    dieptxf3: Dieptxf3,
    dieptxf4: Dieptxf4,
    dieptxf5: Dieptxf5,
    dieptxf6: Dieptxf6,
    dieptxf7: Dieptxf7,
}
impl RegisterBlock {
    #[doc = "0x00 - OTG_HS control and status register"]
    #[inline(always)]
    pub const fn gotgctl(&self) -> &Gotgctl {
        &self.gotgctl
    }
    #[doc = "0x04 - OTG_HS interrupt register"]
    #[inline(always)]
    pub const fn gotgint(&self) -> &Gotgint {
        &self.gotgint
    }
    #[doc = "0x08 - OTG_HS AHB configuration register"]
    #[inline(always)]
    pub const fn gahbcfg(&self) -> &Gahbcfg {
        &self.gahbcfg
    }
    #[doc = "0x0c - OTG_HS USB configuration register"]
    #[inline(always)]
    pub const fn gusbcfg(&self) -> &Gusbcfg {
        &self.gusbcfg
    }
    #[doc = "0x10 - OTG_HS reset register"]
    #[inline(always)]
    pub const fn grstctl(&self) -> &Grstctl {
        &self.grstctl
    }
    #[doc = "0x14 - OTG_HS core interrupt register"]
    #[inline(always)]
    pub const fn gintsts(&self) -> &Gintsts {
        &self.gintsts
    }
    #[doc = "0x18 - OTG_HS interrupt mask register"]
    #[inline(always)]
    pub const fn gintmsk(&self) -> &Gintmsk {
        &self.gintmsk
    }
    #[doc = "0x1c - OTG_HS Receive status debug read register (peripheral mode mode)"]
    #[inline(always)]
    pub const fn grxstsr_peripheral(&self) -> &GrxstsrPeripheral {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - OTG_HS Receive status debug read register (host mode)"]
    #[inline(always)]
    pub const fn grxstsr_host(&self) -> &GrxstsrHost {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x20 - OTG_HS status read and pop register (peripheral mode)"]
    #[inline(always)]
    pub const fn grxstsp_peripheral(&self) -> &GrxstspPeripheral {
        unsafe { &*(self as *const Self).cast::<u8>().add(32).cast() }
    }
    #[doc = "0x20 - OTG_HS status read and pop register (host mode)"]
    #[inline(always)]
    pub const fn grxstsp_host(&self) -> &GrxstspHost {
        unsafe { &*(self as *const Self).cast::<u8>().add(32).cast() }
    }
    #[doc = "0x24 - OTG_HS Receive FIFO size register"]
    #[inline(always)]
    pub const fn grxfsiz(&self) -> &Grxfsiz {
        &self.grxfsiz
    }
    #[doc = "0x28 - Endpoint 0 transmit FIFO size (peripheral mode)"]
    #[inline(always)]
    pub const fn tx0fsiz_peripheral(&self) -> &Tx0fsizPeripheral {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x28 - OTG_HS nonperiodic transmit FIFO size register (host mode)"]
    #[inline(always)]
    pub const fn gnptxfsiz_host(&self) -> &GnptxfsizHost {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x2c - OTG_HS nonperiodic transmit FIFO/queue status register"]
    #[inline(always)]
    pub const fn gnptxsts(&self) -> &Gnptxsts {
        &self.gnptxsts
    }
    #[doc = "0x38 - OTG_HS general core configuration register"]
    #[inline(always)]
    pub const fn gccfg(&self) -> &Gccfg {
        &self.gccfg
    }
    #[doc = "0x3c - OTG_HS core ID register"]
    #[inline(always)]
    pub const fn cid(&self) -> &Cid {
        &self.cid
    }
    #[doc = "0x40 - OTG_HS vendor ID register"]
    #[inline(always)]
    pub const fn vid(&self) -> &Vid {
        &self.vid
    }
    #[doc = "0x44 - Direction"]
    #[inline(always)]
    pub const fn hw_direction(&self) -> &HwDirection {
        &self.hw_direction
    }
    #[doc = "0x48 - Hardware Config 0"]
    #[inline(always)]
    pub const fn hw_config0(&self) -> &HwConfig0 {
        &self.hw_config0
    }
    #[doc = "0x100 - OTG_HS Host periodic transmit FIFO size register"]
    #[inline(always)]
    pub const fn hptxfsiz(&self) -> &Hptxfsiz {
        &self.hptxfsiz
    }
    #[doc = "0x104 - OTG_HS device IN endpoint transmit FIFO size register"]
    #[inline(always)]
    pub const fn dieptxf1(&self) -> &Dieptxf1 {
        &self.dieptxf1
    }
    #[doc = "0x108 - OTG_HS device IN endpoint transmit FIFO size register"]
    #[inline(always)]
    pub const fn dieptxf2(&self) -> &Dieptxf2 {
        &self.dieptxf2
    }
    #[doc = "0x11c - OTG_HS device IN endpoint transmit FIFO size register"]
    #[inline(always)]
    pub const fn dieptxf3(&self) -> &Dieptxf3 {
        &self.dieptxf3
    }
    #[doc = "0x120 - OTG_HS device IN endpoint transmit FIFO size register"]
    #[inline(always)]
    pub const fn dieptxf4(&self) -> &Dieptxf4 {
        &self.dieptxf4
    }
    #[doc = "0x124 - OTG_HS device IN endpoint transmit FIFO size register"]
    #[inline(always)]
    pub const fn dieptxf5(&self) -> &Dieptxf5 {
        &self.dieptxf5
    }
    #[doc = "0x128 - OTG_HS device IN endpoint transmit FIFO size register"]
    #[inline(always)]
    pub const fn dieptxf6(&self) -> &Dieptxf6 {
        &self.dieptxf6
    }
    #[doc = "0x12c - OTG_HS device IN endpoint transmit FIFO size register"]
    #[inline(always)]
    pub const fn dieptxf7(&self) -> &Dieptxf7 {
        &self.dieptxf7
    }
}
#[doc = "GOTGCTL (rw) register accessor: OTG_HS control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`gotgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gotgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gotgctl`]
module"]
#[doc(alias = "GOTGCTL")]
pub type Gotgctl = crate::Reg<gotgctl::GotgctlSpec>;
#[doc = "OTG_HS control and status register"]
pub mod gotgctl;
#[doc = "GOTGINT (rw) register accessor: OTG_HS interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`gotgint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gotgint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gotgint`]
module"]
#[doc(alias = "GOTGINT")]
pub type Gotgint = crate::Reg<gotgint::GotgintSpec>;
#[doc = "OTG_HS interrupt register"]
pub mod gotgint;
#[doc = "GAHBCFG (rw) register accessor: OTG_HS AHB configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`gahbcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gahbcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gahbcfg`]
module"]
#[doc(alias = "GAHBCFG")]
pub type Gahbcfg = crate::Reg<gahbcfg::GahbcfgSpec>;
#[doc = "OTG_HS AHB configuration register"]
pub mod gahbcfg;
#[doc = "GUSBCFG (rw) register accessor: OTG_HS USB configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`gusbcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gusbcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gusbcfg`]
module"]
#[doc(alias = "GUSBCFG")]
pub type Gusbcfg = crate::Reg<gusbcfg::GusbcfgSpec>;
#[doc = "OTG_HS USB configuration register"]
pub mod gusbcfg;
#[doc = "GRSTCTL (rw) register accessor: OTG_HS reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`grstctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grstctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grstctl`]
module"]
#[doc(alias = "GRSTCTL")]
pub type Grstctl = crate::Reg<grstctl::GrstctlSpec>;
#[doc = "OTG_HS reset register"]
pub mod grstctl;
#[doc = "GINTSTS (rw) register accessor: OTG_HS core interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`gintsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintsts`]
module"]
#[doc(alias = "GINTSTS")]
pub type Gintsts = crate::Reg<gintsts::GintstsSpec>;
#[doc = "OTG_HS core interrupt register"]
pub mod gintsts;
#[doc = "GINTMSK (rw) register accessor: OTG_HS interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`gintmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintmsk`]
module"]
#[doc(alias = "GINTMSK")]
pub type Gintmsk = crate::Reg<gintmsk::GintmskSpec>;
#[doc = "OTG_HS interrupt mask register"]
pub mod gintmsk;
#[doc = "GRXSTSR_Host (r) register accessor: OTG_HS Receive status debug read register (host mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`grxstsr_host::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsr_host`]
module"]
#[doc(alias = "GRXSTSR_Host")]
pub type GrxstsrHost = crate::Reg<grxstsr_host::GrxstsrHostSpec>;
#[doc = "OTG_HS Receive status debug read register (host mode)"]
pub mod grxstsr_host;
#[doc = "GRXSTSP_Host (r) register accessor: OTG_HS status read and pop register (host mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`grxstsp_host::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsp_host`]
module"]
#[doc(alias = "GRXSTSP_Host")]
pub type GrxstspHost = crate::Reg<grxstsp_host::GrxstspHostSpec>;
#[doc = "OTG_HS status read and pop register (host mode)"]
pub mod grxstsp_host;
#[doc = "GRXFSIZ (rw) register accessor: OTG_HS Receive FIFO size register\n\nYou can [`read`](crate::Reg::read) this register and get [`grxfsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grxfsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxfsiz`]
module"]
#[doc(alias = "GRXFSIZ")]
pub type Grxfsiz = crate::Reg<grxfsiz::GrxfsizSpec>;
#[doc = "OTG_HS Receive FIFO size register"]
pub mod grxfsiz;
#[doc = "GNPTXFSIZ_Host (rw) register accessor: OTG_HS nonperiodic transmit FIFO size register (host mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`gnptxfsiz_host::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gnptxfsiz_host::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gnptxfsiz_host`]
module"]
#[doc(alias = "GNPTXFSIZ_Host")]
pub type GnptxfsizHost = crate::Reg<gnptxfsiz_host::GnptxfsizHostSpec>;
#[doc = "OTG_HS nonperiodic transmit FIFO size register (host mode)"]
pub mod gnptxfsiz_host;
#[doc = "TX0FSIZ_Peripheral (rw) register accessor: Endpoint 0 transmit FIFO size (peripheral mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`tx0fsiz_peripheral::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx0fsiz_peripheral::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx0fsiz_peripheral`]
module"]
#[doc(alias = "TX0FSIZ_Peripheral")]
pub type Tx0fsizPeripheral = crate::Reg<tx0fsiz_peripheral::Tx0fsizPeripheralSpec>;
#[doc = "Endpoint 0 transmit FIFO size (peripheral mode)"]
pub mod tx0fsiz_peripheral;
#[doc = "GNPTXSTS (r) register accessor: OTG_HS nonperiodic transmit FIFO/queue status register\n\nYou can [`read`](crate::Reg::read) this register and get [`gnptxsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gnptxsts`]
module"]
#[doc(alias = "GNPTXSTS")]
pub type Gnptxsts = crate::Reg<gnptxsts::GnptxstsSpec>;
#[doc = "OTG_HS nonperiodic transmit FIFO/queue status register"]
pub mod gnptxsts;
#[doc = "GCCFG (rw) register accessor: OTG_HS general core configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`gccfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gccfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gccfg`]
module"]
#[doc(alias = "GCCFG")]
pub type Gccfg = crate::Reg<gccfg::GccfgSpec>;
#[doc = "OTG_HS general core configuration register"]
pub mod gccfg;
#[doc = "CID (rw) register accessor: OTG_HS core ID register\n\nYou can [`read`](crate::Reg::read) this register and get [`cid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cid`]
module"]
#[doc(alias = "CID")]
pub type Cid = crate::Reg<cid::CidSpec>;
#[doc = "OTG_HS core ID register"]
pub mod cid;
#[doc = "VID (r) register accessor: OTG_HS vendor ID register\n\nYou can [`read`](crate::Reg::read) this register and get [`vid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vid`]
module"]
#[doc(alias = "VID")]
pub type Vid = crate::Reg<vid::VidSpec>;
#[doc = "OTG_HS vendor ID register"]
pub mod vid;
#[doc = "HW_DIRECTION (r) register accessor: Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_direction::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_direction`]
module"]
#[doc(alias = "HW_DIRECTION")]
pub type HwDirection = crate::Reg<hw_direction::HwDirectionSpec>;
#[doc = "Direction"]
pub mod hw_direction;
#[doc = "HW_CONFIG0 (r) register accessor: Hardware Config 0\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_config0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_config0`]
module"]
#[doc(alias = "HW_CONFIG0")]
pub type HwConfig0 = crate::Reg<hw_config0::HwConfig0Spec>;
#[doc = "Hardware Config 0"]
pub mod hw_config0;
#[doc = "HPTXFSIZ (rw) register accessor: OTG_HS Host periodic transmit FIFO size register\n\nYou can [`read`](crate::Reg::read) this register and get [`hptxfsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hptxfsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hptxfsiz`]
module"]
#[doc(alias = "HPTXFSIZ")]
pub type Hptxfsiz = crate::Reg<hptxfsiz::HptxfsizSpec>;
#[doc = "OTG_HS Host periodic transmit FIFO size register"]
pub mod hptxfsiz;
#[doc = "DIEPTXF1 (rw) register accessor: OTG_HS device IN endpoint transmit FIFO size register\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf1`]
module"]
#[doc(alias = "DIEPTXF1")]
pub type Dieptxf1 = crate::Reg<dieptxf1::Dieptxf1Spec>;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod dieptxf1;
#[doc = "DIEPTXF2 (rw) register accessor: OTG_HS device IN endpoint transmit FIFO size register\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf2`]
module"]
#[doc(alias = "DIEPTXF2")]
pub type Dieptxf2 = crate::Reg<dieptxf2::Dieptxf2Spec>;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod dieptxf2;
#[doc = "DIEPTXF3 (rw) register accessor: OTG_HS device IN endpoint transmit FIFO size register\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf3`]
module"]
#[doc(alias = "DIEPTXF3")]
pub type Dieptxf3 = crate::Reg<dieptxf3::Dieptxf3Spec>;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod dieptxf3;
#[doc = "DIEPTXF4 (rw) register accessor: OTG_HS device IN endpoint transmit FIFO size register\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf4`]
module"]
#[doc(alias = "DIEPTXF4")]
pub type Dieptxf4 = crate::Reg<dieptxf4::Dieptxf4Spec>;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod dieptxf4;
#[doc = "DIEPTXF5 (rw) register accessor: OTG_HS device IN endpoint transmit FIFO size register\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf5`]
module"]
#[doc(alias = "DIEPTXF5")]
pub type Dieptxf5 = crate::Reg<dieptxf5::Dieptxf5Spec>;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod dieptxf5;
#[doc = "DIEPTXF6 (rw) register accessor: OTG_HS device IN endpoint transmit FIFO size register\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf6`]
module"]
#[doc(alias = "DIEPTXF6")]
pub type Dieptxf6 = crate::Reg<dieptxf6::Dieptxf6Spec>;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod dieptxf6;
#[doc = "DIEPTXF7 (rw) register accessor: OTG_HS device IN endpoint transmit FIFO size register\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf7`]
module"]
#[doc(alias = "DIEPTXF7")]
pub type Dieptxf7 = crate::Reg<dieptxf7::Dieptxf7Spec>;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod dieptxf7;
#[doc = "GRXSTSR_Peripheral (r) register accessor: OTG_HS Receive status debug read register (peripheral mode mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`grxstsr_peripheral::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsr_peripheral`]
module"]
#[doc(alias = "GRXSTSR_Peripheral")]
pub type GrxstsrPeripheral = crate::Reg<grxstsr_peripheral::GrxstsrPeripheralSpec>;
#[doc = "OTG_HS Receive status debug read register (peripheral mode mode)"]
pub mod grxstsr_peripheral;
#[doc = "GRXSTSP_Peripheral (r) register accessor: OTG_HS status read and pop register (peripheral mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`grxstsp_peripheral::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsp_peripheral`]
module"]
#[doc(alias = "GRXSTSP_Peripheral")]
pub type GrxstspPeripheral = crate::Reg<grxstsp_peripheral::GrxstspPeripheralSpec>;
#[doc = "OTG_HS status read and pop register (peripheral mode)"]
pub mod grxstsp_peripheral;

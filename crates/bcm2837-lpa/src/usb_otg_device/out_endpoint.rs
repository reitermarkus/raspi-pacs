#[repr(C)]
#[derive(Debug)]
#[doc = "OUT Endpoint %s"]
#[doc(alias = "OUT_ENDPOINT")]
pub struct OutEndpoint {
    doepctl: Doepctl,
    _reserved1: [u8; 0x04],
    doepint: Doepint,
    _reserved2: [u8; 0x04],
    doeptsiz: Doeptsiz,
    doepdma: Doepdma,
}
impl OutEndpoint {
    #[doc = "0x00 - Control"]
    #[inline(always)]
    pub const fn doepctl(&self) -> &Doepctl {
        &self.doepctl
    }
    #[doc = "0x08 - Interrupt"]
    #[inline(always)]
    pub const fn doepint(&self) -> &Doepint {
        &self.doepint
    }
    #[doc = "0x10 - Transfer size"]
    #[inline(always)]
    pub const fn doeptsiz(&self) -> &Doeptsiz {
        &self.doeptsiz
    }
    #[doc = "0x14 - DMA address"]
    #[inline(always)]
    pub const fn doepdma(&self) -> &Doepdma {
        &self.doepdma
    }
}
#[doc = "DOEPCTL (rw) register accessor: Control\n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl`]
module"]
#[doc(alias = "DOEPCTL")]
pub type Doepctl = crate::Reg<doepctl::DoepctlSpec>;
#[doc = "Control"]
pub mod doepctl;
#[doc = "DOEPINT (rw) register accessor: Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`doepint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint`]
module"]
#[doc(alias = "DOEPINT")]
pub type Doepint = crate::Reg<doepint::DoepintSpec>;
#[doc = "Interrupt"]
pub mod doepint;
#[doc = "DOEPTSIZ (rw) register accessor: Transfer size\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz`]
module"]
#[doc(alias = "DOEPTSIZ")]
pub type Doeptsiz = crate::Reg<doeptsiz::DoeptsizSpec>;
#[doc = "Transfer size"]
pub mod doeptsiz;
#[doc = "DOEPDMA (rw) register accessor: DMA address\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepdma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma`]
module"]
#[doc(alias = "DOEPDMA")]
pub type Doepdma = crate::Reg<doepdma::DoepdmaSpec>;
#[doc = "DMA address"]
pub mod doepdma;

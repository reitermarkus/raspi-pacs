#[repr(C)]
#[derive(Debug)]
#[doc = "IN Endpoint %s"]
#[doc(alias = "IN_ENDPOINT")]
pub struct InEndpoint {
    diepctl0: Diepctl0,
    _reserved1: [u8; 0x04],
    diepint: Diepint,
    _reserved2: [u8; 0x04],
    dieptsiz: Dieptsiz,
    diepdma: Diepdma,
    dtxfsts: Dtxfsts,
}
impl InEndpoint {
    #[doc = "0x00 - Control"]
    #[inline(always)]
    pub const fn diepctl0(&self) -> &Diepctl0 {
        &self.diepctl0
    }
    #[doc = "0x08 - Interrupt"]
    #[inline(always)]
    pub const fn diepint(&self) -> &Diepint {
        &self.diepint
    }
    #[doc = "0x10 - Transfer size"]
    #[inline(always)]
    pub const fn dieptsiz(&self) -> &Dieptsiz {
        &self.dieptsiz
    }
    #[doc = "0x14 - DMA address"]
    #[inline(always)]
    pub const fn diepdma(&self) -> &Diepdma {
        &self.diepdma
    }
    #[doc = "0x18 - Transmit FIFO status"]
    #[inline(always)]
    pub const fn dtxfsts(&self) -> &Dtxfsts {
        &self.dtxfsts
    }
}
#[doc = "DIEPCTL0 (rw) register accessor: Control\n\nYou can [`read`](crate::Reg::read) this register and get [`diepctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl0`]
module"]
#[doc(alias = "DIEPCTL0")]
pub type Diepctl0 = crate::Reg<diepctl0::Diepctl0Spec>;
#[doc = "Control"]
pub mod diepctl0;
#[doc = "DIEPINT (rw) register accessor: Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint`]
module"]
#[doc(alias = "DIEPINT")]
pub type Diepint = crate::Reg<diepint::DiepintSpec>;
#[doc = "Interrupt"]
pub mod diepint;
#[doc = "DIEPTSIZ (rw) register accessor: Transfer size\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz`]
module"]
#[doc(alias = "DIEPTSIZ")]
pub type Dieptsiz = crate::Reg<dieptsiz::DieptsizSpec>;
#[doc = "Transfer size"]
pub mod dieptsiz;
#[doc = "DIEPDMA (rw) register accessor: DMA address\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepdma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma`]
module"]
#[doc(alias = "DIEPDMA")]
pub type Diepdma = crate::Reg<diepdma::DiepdmaSpec>;
#[doc = "DMA address"]
pub mod diepdma;
#[doc = "DTXFSTS (r) register accessor: Transmit FIFO status\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts`]
module"]
#[doc(alias = "DTXFSTS")]
pub type Dtxfsts = crate::Reg<dtxfsts::DtxfstsSpec>;
#[doc = "Transmit FIFO status"]
pub mod dtxfsts;

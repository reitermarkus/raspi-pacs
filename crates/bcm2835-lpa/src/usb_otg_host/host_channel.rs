#[repr(C)]
#[derive(Debug)]
#[doc = "Host channel %s"]
#[doc(alias = "HOST_CHANNEL")]
pub struct HostChannel {
    hcchar: Hcchar,
    hcsplt: Hcsplt,
    hcint: Hcint,
    hcintmsk: Hcintmsk,
    hctsiz: Hctsiz,
    hcdma: Hcdma,
}
impl HostChannel {
    #[doc = "0x00 - Characteristics register"]
    #[inline(always)]
    pub const fn hcchar(&self) -> &Hcchar {
        &self.hcchar
    }
    #[doc = "0x04 - Split control register"]
    #[inline(always)]
    pub const fn hcsplt(&self) -> &Hcsplt {
        &self.hcsplt
    }
    #[doc = "0x08 - Interrupt register"]
    #[inline(always)]
    pub const fn hcint(&self) -> &Hcint {
        &self.hcint
    }
    #[doc = "0x0c - Interrupt mask"]
    #[inline(always)]
    pub const fn hcintmsk(&self) -> &Hcintmsk {
        &self.hcintmsk
    }
    #[doc = "0x10 - Transfer size"]
    #[inline(always)]
    pub const fn hctsiz(&self) -> &Hctsiz {
        &self.hctsiz
    }
    #[doc = "0x14 - DMA address"]
    #[inline(always)]
    pub const fn hcdma(&self) -> &Hcdma {
        &self.hcdma
    }
}
#[doc = "HCCHAR (rw) register accessor: Characteristics register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar`]
module"]
#[doc(alias = "HCCHAR")]
pub type Hcchar = crate::Reg<hcchar::HccharSpec>;
#[doc = "Characteristics register"]
pub mod hcchar;
#[doc = "HCSPLT (rw) register accessor: Split control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsplt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcsplt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt`]
module"]
#[doc(alias = "HCSPLT")]
pub type Hcsplt = crate::Reg<hcsplt::HcspltSpec>;
#[doc = "Split control register"]
pub mod hcsplt;
#[doc = "HCINT (rw) register accessor: Interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint`]
module"]
#[doc(alias = "HCINT")]
pub type Hcint = crate::Reg<hcint::HcintSpec>;
#[doc = "Interrupt register"]
pub mod hcint;
#[doc = "HCINTMSK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk`]
module"]
#[doc(alias = "HCINTMSK")]
pub type Hcintmsk = crate::Reg<hcintmsk::HcintmskSpec>;
#[doc = "Interrupt mask"]
pub mod hcintmsk;
#[doc = "HCTSIZ (rw) register accessor: Transfer size\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz`]
module"]
#[doc(alias = "HCTSIZ")]
pub type Hctsiz = crate::Reg<hctsiz::HctsizSpec>;
#[doc = "Transfer size"]
pub mod hctsiz;
#[doc = "HCDMA (rw) register accessor: DMA address\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma`]
module"]
#[doc(alias = "HCDMA")]
pub type Hcdma = crate::Reg<hcdma::HcdmaSpec>;
#[doc = "DMA address"]
pub mod hcdma;

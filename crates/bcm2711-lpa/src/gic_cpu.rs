#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gicc_ctlr: GiccCtlr,
    gicc_pmr: GiccPmr,
    gicc_bpr: GiccBpr,
    gicc_iar: GiccIar,
    gicc_eoir: GiccEoir,
    gicc_rpr: GiccRpr,
    gicc_hppir: GiccHppir,
    gicc_abpr: GiccAbpr,
    gicc_aiar: GiccAiar,
    gicc_aeoir: GiccAeoir,
    gicc_ahppir: GiccAhppir,
    _reserved11: [u8; 0xa4],
    gicc_apr0: GiccApr0,
    _reserved12: [u8; 0x0c],
    gicc_nsapr0: GiccNsapr0,
    _reserved13: [u8; 0x18],
    gicc_iidr: GiccIidr,
    _reserved14: [u8; 0x0f00],
    gicc_dir: GiccDir,
}
impl RegisterBlock {
    #[doc = "0x00 - CPU Interface Control"]
    #[inline(always)]
    pub const fn gicc_ctlr(&self) -> &GiccCtlr {
        &self.gicc_ctlr
    }
    #[doc = "0x04 - Interrupt Priority Mask"]
    #[inline(always)]
    pub const fn gicc_pmr(&self) -> &GiccPmr {
        &self.gicc_pmr
    }
    #[doc = "0x08 - Binary Point"]
    #[inline(always)]
    pub const fn gicc_bpr(&self) -> &GiccBpr {
        &self.gicc_bpr
    }
    #[doc = "0x0c - Interrupt Acknowledge"]
    #[inline(always)]
    pub const fn gicc_iar(&self) -> &GiccIar {
        &self.gicc_iar
    }
    #[doc = "0x10 - End of Interrupt"]
    #[inline(always)]
    pub const fn gicc_eoir(&self) -> &GiccEoir {
        &self.gicc_eoir
    }
    #[doc = "0x14 - Running Priority"]
    #[inline(always)]
    pub const fn gicc_rpr(&self) -> &GiccRpr {
        &self.gicc_rpr
    }
    #[doc = "0x18 - Highest Priority Pending Interrupt"]
    #[inline(always)]
    pub const fn gicc_hppir(&self) -> &GiccHppir {
        &self.gicc_hppir
    }
    #[doc = "0x1c - Aliased Binary Point"]
    #[inline(always)]
    pub const fn gicc_abpr(&self) -> &GiccAbpr {
        &self.gicc_abpr
    }
    #[doc = "0x20 - Aliased Interrupt Acknowledge"]
    #[inline(always)]
    pub const fn gicc_aiar(&self) -> &GiccAiar {
        &self.gicc_aiar
    }
    #[doc = "0x24 - Aliased End of Interrupt"]
    #[inline(always)]
    pub const fn gicc_aeoir(&self) -> &GiccAeoir {
        &self.gicc_aeoir
    }
    #[doc = "0x28 - Aliased Highest Priority Pending Interrupt"]
    #[inline(always)]
    pub const fn gicc_ahppir(&self) -> &GiccAhppir {
        &self.gicc_ahppir
    }
    #[doc = "0xd0 - Active Priority"]
    #[inline(always)]
    pub const fn gicc_apr0(&self) -> &GiccApr0 {
        &self.gicc_apr0
    }
    #[doc = "0xe0 - Non-Secure Active Priority"]
    #[inline(always)]
    pub const fn gicc_nsapr0(&self) -> &GiccNsapr0 {
        &self.gicc_nsapr0
    }
    #[doc = "0xfc - CPU Interface Identification Register"]
    #[inline(always)]
    pub const fn gicc_iidr(&self) -> &GiccIidr {
        &self.gicc_iidr
    }
    #[doc = "0x1000 - Deactivate Interrupt"]
    #[inline(always)]
    pub const fn gicc_dir(&self) -> &GiccDir {
        &self.gicc_dir
    }
}
#[doc = "GICC_CTLR (rw) register accessor: CPU Interface Control\n\nYou can [`read`](crate::Reg::read) this register and get [`gicc_ctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicc_ctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicc_ctlr`]
module"]
#[doc(alias = "GICC_CTLR")]
pub type GiccCtlr = crate::Reg<gicc_ctlr::GiccCtlrSpec>;
#[doc = "CPU Interface Control"]
pub mod gicc_ctlr;
#[doc = "GICC_PMR (rw) register accessor: Interrupt Priority Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`gicc_pmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicc_pmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicc_pmr`]
module"]
#[doc(alias = "GICC_PMR")]
pub type GiccPmr = crate::Reg<gicc_pmr::GiccPmrSpec>;
#[doc = "Interrupt Priority Mask"]
pub mod gicc_pmr;
#[doc = "GICC_BPR (rw) register accessor: Binary Point\n\nYou can [`read`](crate::Reg::read) this register and get [`gicc_bpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicc_bpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicc_bpr`]
module"]
#[doc(alias = "GICC_BPR")]
pub type GiccBpr = crate::Reg<gicc_bpr::GiccBprSpec>;
#[doc = "Binary Point"]
pub mod gicc_bpr;
#[doc = "GICC_IAR (r) register accessor: Interrupt Acknowledge\n\nYou can [`read`](crate::Reg::read) this register and get [`gicc_iar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicc_iar`]
module"]
#[doc(alias = "GICC_IAR")]
pub type GiccIar = crate::Reg<gicc_iar::GiccIarSpec>;
#[doc = "Interrupt Acknowledge"]
pub mod gicc_iar;
#[doc = "GICC_EOIR (w) register accessor: End of Interrupt\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicc_eoir::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicc_eoir`]
module"]
#[doc(alias = "GICC_EOIR")]
pub type GiccEoir = crate::Reg<gicc_eoir::GiccEoirSpec>;
#[doc = "End of Interrupt"]
pub mod gicc_eoir;
#[doc = "GICC_RPR (r) register accessor: Running Priority\n\nYou can [`read`](crate::Reg::read) this register and get [`gicc_rpr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicc_rpr`]
module"]
#[doc(alias = "GICC_RPR")]
pub type GiccRpr = crate::Reg<gicc_rpr::GiccRprSpec>;
#[doc = "Running Priority"]
pub mod gicc_rpr;
#[doc = "GICC_HPPIR (rw) register accessor: Highest Priority Pending Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`gicc_hppir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicc_hppir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicc_hppir`]
module"]
#[doc(alias = "GICC_HPPIR")]
pub type GiccHppir = crate::Reg<gicc_hppir::GiccHppirSpec>;
#[doc = "Highest Priority Pending Interrupt"]
pub mod gicc_hppir;
#[doc = "GICC_ABPR (rw) register accessor: Aliased Binary Point\n\nYou can [`read`](crate::Reg::read) this register and get [`gicc_abpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicc_abpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicc_abpr`]
module"]
#[doc(alias = "GICC_ABPR")]
pub type GiccAbpr = crate::Reg<gicc_abpr::GiccAbprSpec>;
#[doc = "Aliased Binary Point"]
pub mod gicc_abpr;
#[doc = "GICC_AIAR (r) register accessor: Aliased Interrupt Acknowledge\n\nYou can [`read`](crate::Reg::read) this register and get [`gicc_aiar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicc_aiar`]
module"]
#[doc(alias = "GICC_AIAR")]
pub type GiccAiar = crate::Reg<gicc_aiar::GiccAiarSpec>;
#[doc = "Aliased Interrupt Acknowledge"]
pub mod gicc_aiar;
#[doc = "GICC_AEOIR (w) register accessor: Aliased End of Interrupt\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicc_aeoir::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicc_aeoir`]
module"]
#[doc(alias = "GICC_AEOIR")]
pub type GiccAeoir = crate::Reg<gicc_aeoir::GiccAeoirSpec>;
#[doc = "Aliased End of Interrupt"]
pub mod gicc_aeoir;
#[doc = "GICC_AHPPIR (r) register accessor: Aliased Highest Priority Pending Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`gicc_ahppir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicc_ahppir`]
module"]
#[doc(alias = "GICC_AHPPIR")]
pub type GiccAhppir = crate::Reg<gicc_ahppir::GiccAhppirSpec>;
#[doc = "Aliased Highest Priority Pending Interrupt"]
pub mod gicc_ahppir;
#[doc = "GICC_APR0 (rw) register accessor: Active Priority\n\nYou can [`read`](crate::Reg::read) this register and get [`gicc_apr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicc_apr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicc_apr0`]
module"]
#[doc(alias = "GICC_APR0")]
pub type GiccApr0 = crate::Reg<gicc_apr0::GiccApr0Spec>;
#[doc = "Active Priority"]
pub mod gicc_apr0;
#[doc = "GICC_NSAPR0 (rw) register accessor: Non-Secure Active Priority\n\nYou can [`read`](crate::Reg::read) this register and get [`gicc_nsapr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicc_nsapr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicc_nsapr0`]
module"]
#[doc(alias = "GICC_NSAPR0")]
pub type GiccNsapr0 = crate::Reg<gicc_nsapr0::GiccNsapr0Spec>;
#[doc = "Non-Secure Active Priority"]
pub mod gicc_nsapr0;
#[doc = "GICC_IIDR (rw) register accessor: CPU Interface Identification Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gicc_iidr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicc_iidr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicc_iidr`]
module"]
#[doc(alias = "GICC_IIDR")]
pub type GiccIidr = crate::Reg<gicc_iidr::GiccIidrSpec>;
#[doc = "CPU Interface Identification Register"]
pub mod gicc_iidr;
#[doc = "GICC_DIR (w) register accessor: Deactivate Interrupt\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicc_dir::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicc_dir`]
module"]
#[doc(alias = "GICC_DIR")]
pub type GiccDir = crate::Reg<gicc_dir::GiccDirSpec>;
#[doc = "Deactivate Interrupt"]
pub mod gicc_dir;

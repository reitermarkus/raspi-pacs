#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gicd_ctlr: GicdCtlr,
    gicd_typer: GicdTyper,
    gicd_iidr: GicdIidr,
    _reserved3: [u8; 0x74],
    gicd_igroupr: GicdIgroupr,
    _reserved4: [u8; 0x64],
    gicd_isenabler: GicdIsenabler,
    _reserved5: [u8; 0x64],
    gicd_icenabler: GicdIcenabler,
    _reserved6: [u8; 0x64],
    gicd_ispendr: GicdIspendr,
    _reserved7: [u8; 0x64],
    gicd_icpendr: GicdIcpendr,
    _reserved8: [u8; 0x64],
    gicd_isactiver: GicdIsactiver,
    _reserved9: [u8; 0x64],
    gicd_icactiver: GicdIcactiver,
    _reserved10: [u8; 0x64],
    gicd_ipriorityr: GicdIpriorityr,
    _reserved11: [u8; 0x0320],
    gicd_itargetsr: GicdItargetsr,
    _reserved12: [u8; 0x0320],
    gicd_icfgr: GicdIcfgr,
    _reserved13: [u8; 0xc8],
    gicd_ppisr: GicdPpisr,
    gicd_spisr0: GicdSpisr0,
    gicd_spisr1: GicdSpisr1,
    gicd_spisr2: GicdSpisr2,
    gicd_spisr3: GicdSpisr3,
    gicd_spisr4: GicdSpisr4,
    gicd_spisr5: GicdSpisr5,
    _reserved20: [u8; 0x01e4],
    gicd_sgir: GicdSgir,
    _reserved21: [u8; 0x0c],
    gicd_cpendsgirn: GicdCpendsgirn,
    _reserved22: [u8; 0x0c],
    gicd_spendsgirn: GicdSpendsgirn,
    _reserved23: [u8; 0xac],
    gicd_pidr4: GicdPidr4,
    gicd_pidr5: GicdPidr5,
    gicd_pidr6: GicdPidr6,
    gicd_pidr7: GicdPidr7,
    gicd_pidr0: GicdPidr0,
    gicd_pidr1: GicdPidr1,
    gicd_pidr2: GicdPidr2,
    gicd_pidr3: GicdPidr3,
    gicd_cidr0: GicdCidr0,
    gicd_cidr1: GicdCidr1,
    gicd_cidr2: GicdCidr2,
    gicd_cidr3: GicdCidr3,
}
impl RegisterBlock {
    #[doc = "0x00 - Distributor Control Register"]
    #[inline(always)]
    pub const fn gicd_ctlr(&self) -> &GicdCtlr {
        &self.gicd_ctlr
    }
    #[doc = "0x04 - Interrupt Controller Type Register"]
    #[inline(always)]
    pub const fn gicd_typer(&self) -> &GicdTyper {
        &self.gicd_typer
    }
    #[doc = "0x08 - Distributor Implementer Identification Register"]
    #[inline(always)]
    pub const fn gicd_iidr(&self) -> &GicdIidr {
        &self.gicd_iidr
    }
    #[doc = "0x80..0x9c - Interrupt Group Registers"]
    #[inline(always)]
    pub const fn gicd_igroupr(&self) -> &GicdIgroupr {
        &self.gicd_igroupr
    }
    #[doc = "0x100..0x11c - Interrupt Set-Enable Registers"]
    #[inline(always)]
    pub const fn gicd_isenabler(&self) -> &GicdIsenabler {
        &self.gicd_isenabler
    }
    #[doc = "0x180..0x19c - Interrupt Clear-Enable Registers"]
    #[inline(always)]
    pub const fn gicd_icenabler(&self) -> &GicdIcenabler {
        &self.gicd_icenabler
    }
    #[doc = "0x200..0x21c - Interrupt Set-Pending Registers"]
    #[inline(always)]
    pub const fn gicd_ispendr(&self) -> &GicdIspendr {
        &self.gicd_ispendr
    }
    #[doc = "0x280..0x29c - Interrupt Clear-Pending Registers"]
    #[inline(always)]
    pub const fn gicd_icpendr(&self) -> &GicdIcpendr {
        &self.gicd_icpendr
    }
    #[doc = "0x300..0x31c - Interrupt Set-Active Registers"]
    #[inline(always)]
    pub const fn gicd_isactiver(&self) -> &GicdIsactiver {
        &self.gicd_isactiver
    }
    #[doc = "0x380..0x39c - Interrupt Clear-Active Registers"]
    #[inline(always)]
    pub const fn gicd_icactiver(&self) -> &GicdIcactiver {
        &self.gicd_icactiver
    }
    #[doc = "0x400..0x4e0 - Interrupt Priority"]
    #[inline(always)]
    pub const fn gicd_ipriorityr(&self) -> &GicdIpriorityr {
        &self.gicd_ipriorityr
    }
    #[doc = "0x800..0x8e0 - Interrupt Processor Targets"]
    #[inline(always)]
    pub const fn gicd_itargetsr(&self) -> &GicdItargetsr {
        &self.gicd_itargetsr
    }
    #[doc = "0xc00..0xc38 - Interrupt Configuration"]
    #[inline(always)]
    pub const fn gicd_icfgr(&self) -> &GicdIcfgr {
        &self.gicd_icfgr
    }
    #[doc = "0xd00 - Private Peripheral Interrupt Status Register"]
    #[inline(always)]
    pub const fn gicd_ppisr(&self) -> &GicdPpisr {
        &self.gicd_ppisr
    }
    #[doc = "0xd04 - Shared Peripheral Interrupt Status Registers"]
    #[inline(always)]
    pub const fn gicd_spisr0(&self) -> &GicdSpisr0 {
        &self.gicd_spisr0
    }
    #[doc = "0xd08 - Shared Peripheral Interrupt Status Registers"]
    #[inline(always)]
    pub const fn gicd_spisr1(&self) -> &GicdSpisr1 {
        &self.gicd_spisr1
    }
    #[doc = "0xd0c - Shared Peripheral Interrupt Status Registers"]
    #[inline(always)]
    pub const fn gicd_spisr2(&self) -> &GicdSpisr2 {
        &self.gicd_spisr2
    }
    #[doc = "0xd10 - Shared Peripheral Interrupt Status Registers"]
    #[inline(always)]
    pub const fn gicd_spisr3(&self) -> &GicdSpisr3 {
        &self.gicd_spisr3
    }
    #[doc = "0xd14 - Shared Peripheral Interrupt Status Registers"]
    #[inline(always)]
    pub const fn gicd_spisr4(&self) -> &GicdSpisr4 {
        &self.gicd_spisr4
    }
    #[doc = "0xd18 - Shared Peripheral Interrupt Status Registers"]
    #[inline(always)]
    pub const fn gicd_spisr5(&self) -> &GicdSpisr5 {
        &self.gicd_spisr5
    }
    #[doc = "0xf00 - Software Generated Interrupt Register"]
    #[inline(always)]
    pub const fn gicd_sgir(&self) -> &GicdSgir {
        &self.gicd_sgir
    }
    #[doc = "0xf10 - SGI Clear-Pending Registers"]
    #[inline(always)]
    pub const fn gicd_cpendsgirn(&self) -> &GicdCpendsgirn {
        &self.gicd_cpendsgirn
    }
    #[doc = "0xf20 - SGI Set-Pending Registers"]
    #[inline(always)]
    pub const fn gicd_spendsgirn(&self) -> &GicdSpendsgirn {
        &self.gicd_spendsgirn
    }
    #[doc = "0xfd0 - Peripheral ID 4"]
    #[inline(always)]
    pub const fn gicd_pidr4(&self) -> &GicdPidr4 {
        &self.gicd_pidr4
    }
    #[doc = "0xfd4 - Peripheral ID 5"]
    #[inline(always)]
    pub const fn gicd_pidr5(&self) -> &GicdPidr5 {
        &self.gicd_pidr5
    }
    #[doc = "0xfd8 - Peripheral ID 6"]
    #[inline(always)]
    pub const fn gicd_pidr6(&self) -> &GicdPidr6 {
        &self.gicd_pidr6
    }
    #[doc = "0xfdc - Peripheral ID 7"]
    #[inline(always)]
    pub const fn gicd_pidr7(&self) -> &GicdPidr7 {
        &self.gicd_pidr7
    }
    #[doc = "0xfe0 - Peripheral ID 0"]
    #[inline(always)]
    pub const fn gicd_pidr0(&self) -> &GicdPidr0 {
        &self.gicd_pidr0
    }
    #[doc = "0xfe4 - Peripheral ID 1"]
    #[inline(always)]
    pub const fn gicd_pidr1(&self) -> &GicdPidr1 {
        &self.gicd_pidr1
    }
    #[doc = "0xfe8 - Peripheral ID 2"]
    #[inline(always)]
    pub const fn gicd_pidr2(&self) -> &GicdPidr2 {
        &self.gicd_pidr2
    }
    #[doc = "0xfec - Peripheral ID 3"]
    #[inline(always)]
    pub const fn gicd_pidr3(&self) -> &GicdPidr3 {
        &self.gicd_pidr3
    }
    #[doc = "0xff0 - Component ID 0"]
    #[inline(always)]
    pub const fn gicd_cidr0(&self) -> &GicdCidr0 {
        &self.gicd_cidr0
    }
    #[doc = "0xff4 - Component ID 1"]
    #[inline(always)]
    pub const fn gicd_cidr1(&self) -> &GicdCidr1 {
        &self.gicd_cidr1
    }
    #[doc = "0xff8 - Component ID 2"]
    #[inline(always)]
    pub const fn gicd_cidr2(&self) -> &GicdCidr2 {
        &self.gicd_cidr2
    }
    #[doc = "0xffc - Component ID 3"]
    #[inline(always)]
    pub const fn gicd_cidr3(&self) -> &GicdCidr3 {
        &self.gicd_cidr3
    }
}
#[doc = "GICD_CTLR (rw) register accessor: Distributor Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ctlr`]
module"]
#[doc(alias = "GICD_CTLR")]
pub type GicdCtlr = crate::Reg<gicd_ctlr::GicdCtlrSpec>;
#[doc = "Distributor Control Register"]
pub mod gicd_ctlr;
#[doc = "GICD_TYPER (r) register accessor: Interrupt Controller Type Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_typer::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_typer`]
module"]
#[doc(alias = "GICD_TYPER")]
pub type GicdTyper = crate::Reg<gicd_typer::GicdTyperSpec>;
#[doc = "Interrupt Controller Type Register"]
pub mod gicd_typer;
#[doc = "GICD_IIDR (r) register accessor: Distributor Implementer Identification Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_iidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_iidr`]
module"]
#[doc(alias = "GICD_IIDR")]
pub type GicdIidr = crate::Reg<gicd_iidr::GicdIidrSpec>;
#[doc = "Distributor Implementer Identification Register"]
pub mod gicd_iidr;
#[doc = "Interrupt Group Registers"]
pub use self::gicd_igroupr::GicdIgroupr;
#[doc = r"Cluster"]
#[doc = "Interrupt Group Registers"]
pub mod gicd_igroupr;
#[doc = "Interrupt Set-Enable Registers"]
pub use self::gicd_isenabler::GicdIsenabler;
#[doc = r"Cluster"]
#[doc = "Interrupt Set-Enable Registers"]
pub mod gicd_isenabler;
#[doc = "Interrupt Clear-Enable Registers"]
pub use self::gicd_icenabler::GicdIcenabler;
#[doc = r"Cluster"]
#[doc = "Interrupt Clear-Enable Registers"]
pub mod gicd_icenabler;
#[doc = "Interrupt Set-Pending Registers"]
pub use self::gicd_ispendr::GicdIspendr;
#[doc = r"Cluster"]
#[doc = "Interrupt Set-Pending Registers"]
pub mod gicd_ispendr;
#[doc = "Interrupt Clear-Pending Registers"]
pub use self::gicd_icpendr::GicdIcpendr;
#[doc = r"Cluster"]
#[doc = "Interrupt Clear-Pending Registers"]
pub mod gicd_icpendr;
#[doc = "Interrupt Set-Active Registers"]
pub use self::gicd_isactiver::GicdIsactiver;
#[doc = r"Cluster"]
#[doc = "Interrupt Set-Active Registers"]
pub mod gicd_isactiver;
#[doc = "Interrupt Clear-Active Registers"]
pub use self::gicd_icactiver::GicdIcactiver;
#[doc = r"Cluster"]
#[doc = "Interrupt Clear-Active Registers"]
pub mod gicd_icactiver;
#[doc = "Interrupt Priority"]
pub use self::gicd_ipriorityr::GicdIpriorityr;
#[doc = r"Cluster"]
#[doc = "Interrupt Priority"]
pub mod gicd_ipriorityr;
#[doc = "Interrupt Processor Targets"]
pub use self::gicd_itargetsr::GicdItargetsr;
#[doc = r"Cluster"]
#[doc = "Interrupt Processor Targets"]
pub mod gicd_itargetsr;
#[doc = "Interrupt Configuration"]
pub use self::gicd_icfgr::GicdIcfgr;
#[doc = r"Cluster"]
#[doc = "Interrupt Configuration"]
pub mod gicd_icfgr;
#[doc = "GICD_PPISR (rw) register accessor: Private Peripheral Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ppisr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ppisr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ppisr`]
module"]
#[doc(alias = "GICD_PPISR")]
pub type GicdPpisr = crate::Reg<gicd_ppisr::GicdPpisrSpec>;
#[doc = "Private Peripheral Interrupt Status Register"]
pub mod gicd_ppisr;
#[doc = "GICD_SPISR0 (rw) register accessor: Shared Peripheral Interrupt Status Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_spisr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_spisr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_spisr0`]
module"]
#[doc(alias = "GICD_SPISR0")]
pub type GicdSpisr0 = crate::Reg<gicd_spisr0::GicdSpisr0Spec>;
#[doc = "Shared Peripheral Interrupt Status Registers"]
pub mod gicd_spisr0;
#[doc = "GICD_SPISR1 (rw) register accessor: Shared Peripheral Interrupt Status Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_spisr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_spisr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_spisr1`]
module"]
#[doc(alias = "GICD_SPISR1")]
pub type GicdSpisr1 = crate::Reg<gicd_spisr1::GicdSpisr1Spec>;
#[doc = "Shared Peripheral Interrupt Status Registers"]
pub mod gicd_spisr1;
#[doc = "GICD_SPISR2 (rw) register accessor: Shared Peripheral Interrupt Status Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_spisr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_spisr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_spisr2`]
module"]
#[doc(alias = "GICD_SPISR2")]
pub type GicdSpisr2 = crate::Reg<gicd_spisr2::GicdSpisr2Spec>;
#[doc = "Shared Peripheral Interrupt Status Registers"]
pub mod gicd_spisr2;
#[doc = "GICD_SPISR3 (rw) register accessor: Shared Peripheral Interrupt Status Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_spisr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_spisr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_spisr3`]
module"]
#[doc(alias = "GICD_SPISR3")]
pub type GicdSpisr3 = crate::Reg<gicd_spisr3::GicdSpisr3Spec>;
#[doc = "Shared Peripheral Interrupt Status Registers"]
pub mod gicd_spisr3;
#[doc = "GICD_SPISR4 (rw) register accessor: Shared Peripheral Interrupt Status Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_spisr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_spisr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_spisr4`]
module"]
#[doc(alias = "GICD_SPISR4")]
pub type GicdSpisr4 = crate::Reg<gicd_spisr4::GicdSpisr4Spec>;
#[doc = "Shared Peripheral Interrupt Status Registers"]
pub mod gicd_spisr4;
#[doc = "GICD_SPISR5 (rw) register accessor: Shared Peripheral Interrupt Status Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_spisr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_spisr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_spisr5`]
module"]
#[doc(alias = "GICD_SPISR5")]
pub type GicdSpisr5 = crate::Reg<gicd_spisr5::GicdSpisr5Spec>;
#[doc = "Shared Peripheral Interrupt Status Registers"]
pub mod gicd_spisr5;
#[doc = "GICD_SGIR (w) register accessor: Software Generated Interrupt Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_sgir::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_sgir`]
module"]
#[doc(alias = "GICD_SGIR")]
pub type GicdSgir = crate::Reg<gicd_sgir::GicdSgirSpec>;
#[doc = "Software Generated Interrupt Register"]
pub mod gicd_sgir;
#[doc = "GICD_CPENDSGIRn (rw) register accessor: SGI Clear-Pending Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_cpendsgirn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_cpendsgirn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_cpendsgirn`]
module"]
#[doc(alias = "GICD_CPENDSGIRn")]
pub type GicdCpendsgirn = crate::Reg<gicd_cpendsgirn::GicdCpendsgirnSpec>;
#[doc = "SGI Clear-Pending Registers"]
pub mod gicd_cpendsgirn;
#[doc = "GICD_SPENDSGIRn (rw) register accessor: SGI Set-Pending Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_spendsgirn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_spendsgirn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_spendsgirn`]
module"]
#[doc(alias = "GICD_SPENDSGIRn")]
pub type GicdSpendsgirn = crate::Reg<gicd_spendsgirn::GicdSpendsgirnSpec>;
#[doc = "SGI Set-Pending Registers"]
pub mod gicd_spendsgirn;
#[doc = "GICD_PIDR4 (r) register accessor: Peripheral ID 4\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_pidr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_pidr4`]
module"]
#[doc(alias = "GICD_PIDR4")]
pub type GicdPidr4 = crate::Reg<gicd_pidr4::GicdPidr4Spec>;
#[doc = "Peripheral ID 4"]
pub mod gicd_pidr4;
#[doc = "GICD_PIDR5 (r) register accessor: Peripheral ID 5\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_pidr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_pidr5`]
module"]
#[doc(alias = "GICD_PIDR5")]
pub type GicdPidr5 = crate::Reg<gicd_pidr5::GicdPidr5Spec>;
#[doc = "Peripheral ID 5"]
pub mod gicd_pidr5;
#[doc = "GICD_PIDR6 (r) register accessor: Peripheral ID 6\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_pidr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_pidr6`]
module"]
#[doc(alias = "GICD_PIDR6")]
pub type GicdPidr6 = crate::Reg<gicd_pidr6::GicdPidr6Spec>;
#[doc = "Peripheral ID 6"]
pub mod gicd_pidr6;
#[doc = "GICD_PIDR7 (r) register accessor: Peripheral ID 7\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_pidr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_pidr7`]
module"]
#[doc(alias = "GICD_PIDR7")]
pub type GicdPidr7 = crate::Reg<gicd_pidr7::GicdPidr7Spec>;
#[doc = "Peripheral ID 7"]
pub mod gicd_pidr7;
#[doc = "GICD_PIDR0 (r) register accessor: Peripheral ID 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_pidr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_pidr0`]
module"]
#[doc(alias = "GICD_PIDR0")]
pub type GicdPidr0 = crate::Reg<gicd_pidr0::GicdPidr0Spec>;
#[doc = "Peripheral ID 0"]
pub mod gicd_pidr0;
#[doc = "GICD_PIDR1 (r) register accessor: Peripheral ID 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_pidr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_pidr1`]
module"]
#[doc(alias = "GICD_PIDR1")]
pub type GicdPidr1 = crate::Reg<gicd_pidr1::GicdPidr1Spec>;
#[doc = "Peripheral ID 1"]
pub mod gicd_pidr1;
#[doc = "GICD_PIDR2 (r) register accessor: Peripheral ID 2\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_pidr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_pidr2`]
module"]
#[doc(alias = "GICD_PIDR2")]
pub type GicdPidr2 = crate::Reg<gicd_pidr2::GicdPidr2Spec>;
#[doc = "Peripheral ID 2"]
pub mod gicd_pidr2;
#[doc = "GICD_PIDR3 (r) register accessor: Peripheral ID 3\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_pidr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_pidr3`]
module"]
#[doc(alias = "GICD_PIDR3")]
pub type GicdPidr3 = crate::Reg<gicd_pidr3::GicdPidr3Spec>;
#[doc = "Peripheral ID 3"]
pub mod gicd_pidr3;
#[doc = "GICD_CIDR0 (r) register accessor: Component ID 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_cidr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_cidr0`]
module"]
#[doc(alias = "GICD_CIDR0")]
pub type GicdCidr0 = crate::Reg<gicd_cidr0::GicdCidr0Spec>;
#[doc = "Component ID 0"]
pub mod gicd_cidr0;
#[doc = "GICD_CIDR1 (r) register accessor: Component ID 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_cidr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_cidr1`]
module"]
#[doc(alias = "GICD_CIDR1")]
pub type GicdCidr1 = crate::Reg<gicd_cidr1::GicdCidr1Spec>;
#[doc = "Component ID 1"]
pub mod gicd_cidr1;
#[doc = "GICD_CIDR2 (r) register accessor: Component ID 2\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_cidr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_cidr2`]
module"]
#[doc(alias = "GICD_CIDR2")]
pub type GicdCidr2 = crate::Reg<gicd_cidr2::GicdCidr2Spec>;
#[doc = "Component ID 2"]
pub mod gicd_cidr2;
#[doc = "GICD_CIDR3 (r) register accessor: Component ID 3\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_cidr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_cidr3`]
module"]
#[doc(alias = "GICD_CIDR3")]
pub type GicdCidr3 = crate::Reg<gicd_cidr3::GicdCidr3Spec>;
#[doc = "Component ID 3"]
pub mod gicd_cidr3;

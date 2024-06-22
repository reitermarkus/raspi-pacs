#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    arg2: Arg2,
    blksizecnt: Blksizecnt,
    arg1: Arg1,
    cmdtm: Cmdtm,
    resp0: Resp0,
    resp1: Resp1,
    resp2: Resp2,
    resp3: Resp3,
    data: Data,
    status: Status,
    control0: Control0,
    control1: Control1,
    interrupt: Interrupt,
    irpt_mask: IrptMask,
    irpt_en: IrptEn,
    control2: Control2,
    _reserved16: [u8; 0x10],
    force_irpt: ForceIrpt,
    _reserved17: [u8; 0x1c],
    boot_timeout: BootTimeout,
    dbg_sel: DbgSel,
    _reserved19: [u8; 0x08],
    exrdfifo_cfg: ExrdfifoCfg,
    exrdfifo_en: ExrdfifoEn,
    tune_step: TuneStep,
    tune_steps_std: TuneStepsStd,
    tune_steps_ddr: TuneStepsDdr,
    _reserved24: [u8; 0x5c],
    spi_int_spt: SpiIntSpt,
    _reserved25: [u8; 0x08],
    slotisr_ver: SlotisrVer,
}
impl RegisterBlock {
    #[doc = "0x00 - Argument for ACMD23 command"]
    #[inline(always)]
    pub const fn arg2(&self) -> &Arg2 {
        &self.arg2
    }
    #[doc = "0x04 - Numer and size in bytes for data block to be transferred"]
    #[inline(always)]
    pub const fn blksizecnt(&self) -> &Blksizecnt {
        &self.blksizecnt
    }
    #[doc = "0x08 - Argument for everything but ACMD23"]
    #[inline(always)]
    pub const fn arg1(&self) -> &Arg1 {
        &self.arg1
    }
    #[doc = "0x0c - Issue commands to the card"]
    #[inline(always)]
    pub const fn cmdtm(&self) -> &Cmdtm {
        &self.cmdtm
    }
    #[doc = "0x10 - Status bits of the response"]
    #[inline(always)]
    pub const fn resp0(&self) -> &Resp0 {
        &self.resp0
    }
    #[doc = "0x14 - Bits 63:32 of CMD2 and CMD10 responses"]
    #[inline(always)]
    pub const fn resp1(&self) -> &Resp1 {
        &self.resp1
    }
    #[doc = "0x18 - Bits 95:64 of CMD2 and CMD10 responses"]
    #[inline(always)]
    pub const fn resp2(&self) -> &Resp2 {
        &self.resp2
    }
    #[doc = "0x1c - Bits 127:96 of CMD2 and CMD10 responses"]
    #[inline(always)]
    pub const fn resp3(&self) -> &Resp3 {
        &self.resp3
    }
    #[doc = "0x20 - Data to/from the card"]
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
    #[doc = "0x24 - Status info for debugging"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x28 - Control"]
    #[inline(always)]
    pub const fn control0(&self) -> &Control0 {
        &self.control0
    }
    #[doc = "0x2c - Configure"]
    #[inline(always)]
    pub const fn control1(&self) -> &Control1 {
        &self.control1
    }
    #[doc = "0x30 - Interrupt flags"]
    #[inline(always)]
    pub const fn interrupt(&self) -> &Interrupt {
        &self.interrupt
    }
    #[doc = "0x34 - Mask interrupts that change in INTERRUPT"]
    #[inline(always)]
    pub const fn irpt_mask(&self) -> &IrptMask {
        &self.irpt_mask
    }
    #[doc = "0x38 - Enable interrupt to core"]
    #[inline(always)]
    pub const fn irpt_en(&self) -> &IrptEn {
        &self.irpt_en
    }
    #[doc = "0x3c - Control 2"]
    #[inline(always)]
    pub const fn control2(&self) -> &Control2 {
        &self.control2
    }
    #[doc = "0x50 - Force an interrupt"]
    #[inline(always)]
    pub const fn force_irpt(&self) -> &ForceIrpt {
        &self.force_irpt
    }
    #[doc = "0x70 - Number of SD clock cycles to wait for boot"]
    #[inline(always)]
    pub const fn boot_timeout(&self) -> &BootTimeout {
        &self.boot_timeout
    }
    #[doc = "0x74 - What submodules are accessed by the debug bus"]
    #[inline(always)]
    pub const fn dbg_sel(&self) -> &DbgSel {
        &self.dbg_sel
    }
    #[doc = "0x80 - Fine tune DMA request generation"]
    #[inline(always)]
    pub const fn exrdfifo_cfg(&self) -> &ExrdfifoCfg {
        &self.exrdfifo_cfg
    }
    #[doc = "0x84 - Enable the extension data register"]
    #[inline(always)]
    pub const fn exrdfifo_en(&self) -> &ExrdfifoEn {
        &self.exrdfifo_en
    }
    #[doc = "0x88 - Sample clock delay step duration"]
    #[inline(always)]
    pub const fn tune_step(&self) -> &TuneStep {
        &self.tune_step
    }
    #[doc = "0x8c - Sample clock delay step count for SDR"]
    #[inline(always)]
    pub const fn tune_steps_std(&self) -> &TuneStepsStd {
        &self.tune_steps_std
    }
    #[doc = "0x90 - Sample clock delay step count for DDR"]
    #[inline(always)]
    pub const fn tune_steps_ddr(&self) -> &TuneStepsDdr {
        &self.tune_steps_ddr
    }
    #[doc = "0xf0 - Interrupts in SPI mode depend on CS"]
    #[inline(always)]
    pub const fn spi_int_spt(&self) -> &SpiIntSpt {
        &self.spi_int_spt
    }
    #[doc = "0xfc - Version information and slot interrupt status"]
    #[inline(always)]
    pub const fn slotisr_ver(&self) -> &SlotisrVer {
        &self.slotisr_ver
    }
}
#[doc = "ARG2 (rw) register accessor: Argument for ACMD23 command\n\nYou can [`read`](crate::Reg::read) this register and get [`arg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arg2`]
module"]
#[doc(alias = "ARG2")]
pub type Arg2 = crate::Reg<arg2::Arg2Spec>;
#[doc = "Argument for ACMD23 command"]
pub mod arg2;
#[doc = "BLKSIZECNT (rw) register accessor: Numer and size in bytes for data block to be transferred\n\nYou can [`read`](crate::Reg::read) this register and get [`blksizecnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blksizecnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blksizecnt`]
module"]
#[doc(alias = "BLKSIZECNT")]
pub type Blksizecnt = crate::Reg<blksizecnt::BlksizecntSpec>;
#[doc = "Numer and size in bytes for data block to be transferred"]
pub mod blksizecnt;
#[doc = "ARG1 (rw) register accessor: Argument for everything but ACMD23\n\nYou can [`read`](crate::Reg::read) this register and get [`arg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arg1`]
module"]
#[doc(alias = "ARG1")]
pub type Arg1 = crate::Reg<arg1::Arg1Spec>;
#[doc = "Argument for everything but ACMD23"]
pub mod arg1;
#[doc = "CMDTM (rw) register accessor: Issue commands to the card\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdtm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdtm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdtm`]
module"]
#[doc(alias = "CMDTM")]
pub type Cmdtm = crate::Reg<cmdtm::CmdtmSpec>;
#[doc = "Issue commands to the card"]
pub mod cmdtm;
#[doc = "RESP0 (rw) register accessor: Status bits of the response\n\nYou can [`read`](crate::Reg::read) this register and get [`resp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp0`]
module"]
#[doc(alias = "RESP0")]
pub type Resp0 = crate::Reg<resp0::Resp0Spec>;
#[doc = "Status bits of the response"]
pub mod resp0;
#[doc = "RESP1 (rw) register accessor: Bits 63:32 of CMD2 and CMD10 responses\n\nYou can [`read`](crate::Reg::read) this register and get [`resp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp1`]
module"]
#[doc(alias = "RESP1")]
pub type Resp1 = crate::Reg<resp1::Resp1Spec>;
#[doc = "Bits 63:32 of CMD2 and CMD10 responses"]
pub mod resp1;
#[doc = "RESP2 (rw) register accessor: Bits 95:64 of CMD2 and CMD10 responses\n\nYou can [`read`](crate::Reg::read) this register and get [`resp2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resp2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp2`]
module"]
#[doc(alias = "RESP2")]
pub type Resp2 = crate::Reg<resp2::Resp2Spec>;
#[doc = "Bits 95:64 of CMD2 and CMD10 responses"]
pub mod resp2;
#[doc = "RESP3 (rw) register accessor: Bits 127:96 of CMD2 and CMD10 responses\n\nYou can [`read`](crate::Reg::read) this register and get [`resp3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resp3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp3`]
module"]
#[doc(alias = "RESP3")]
pub type Resp3 = crate::Reg<resp3::Resp3Spec>;
#[doc = "Bits 127:96 of CMD2 and CMD10 responses"]
pub mod resp3;
#[doc = "DATA (rw) register accessor: Data to/from the card\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "Data to/from the card"]
pub mod data;
#[doc = "STATUS (rw) register accessor: Status info for debugging\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status info for debugging"]
pub mod status;
#[doc = "CONTROL0 (rw) register accessor: Control\n\nYou can [`read`](crate::Reg::read) this register and get [`control0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control0`]
module"]
#[doc(alias = "CONTROL0")]
pub type Control0 = crate::Reg<control0::Control0Spec>;
#[doc = "Control"]
pub mod control0;
#[doc = "CONTROL1 (rw) register accessor: Configure\n\nYou can [`read`](crate::Reg::read) this register and get [`control1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control1`]
module"]
#[doc(alias = "CONTROL1")]
pub type Control1 = crate::Reg<control1::Control1Spec>;
#[doc = "Configure"]
pub mod control1;
#[doc = "INTERRUPT (rw) register accessor: Interrupt flags\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt`]
module"]
#[doc(alias = "INTERRUPT")]
pub type Interrupt = crate::Reg<interrupt::InterruptSpec>;
#[doc = "Interrupt flags"]
pub mod interrupt;
#[doc = "IRPT_MASK (rw) register accessor: Mask interrupts that change in INTERRUPT\n\nYou can [`read`](crate::Reg::read) this register and get [`irpt_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irpt_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irpt_mask`]
module"]
#[doc(alias = "IRPT_MASK")]
pub type IrptMask = crate::Reg<irpt_mask::IrptMaskSpec>;
#[doc = "Mask interrupts that change in INTERRUPT"]
pub mod irpt_mask;
#[doc = "IRPT_EN (rw) register accessor: Enable interrupt to core\n\nYou can [`read`](crate::Reg::read) this register and get [`irpt_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irpt_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irpt_en`]
module"]
#[doc(alias = "IRPT_EN")]
pub type IrptEn = crate::Reg<irpt_en::IrptEnSpec>;
#[doc = "Enable interrupt to core"]
pub mod irpt_en;
#[doc = "CONTROL2 (rw) register accessor: Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`control2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control2`]
module"]
#[doc(alias = "CONTROL2")]
pub type Control2 = crate::Reg<control2::Control2Spec>;
#[doc = "Control 2"]
pub mod control2;
#[doc = "FORCE_IRPT (rw) register accessor: Force an interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`force_irpt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`force_irpt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@force_irpt`]
module"]
#[doc(alias = "FORCE_IRPT")]
pub type ForceIrpt = crate::Reg<force_irpt::ForceIrptSpec>;
#[doc = "Force an interrupt"]
pub mod force_irpt;
#[doc = "BOOT_TIMEOUT (rw) register accessor: Number of SD clock cycles to wait for boot\n\nYou can [`read`](crate::Reg::read) this register and get [`boot_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boot_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@boot_timeout`]
module"]
#[doc(alias = "BOOT_TIMEOUT")]
pub type BootTimeout = crate::Reg<boot_timeout::BootTimeoutSpec>;
#[doc = "Number of SD clock cycles to wait for boot"]
pub mod boot_timeout;
#[doc = "DBG_SEL (rw) register accessor: What submodules are accessed by the debug bus\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_sel`]
module"]
#[doc(alias = "DBG_SEL")]
pub type DbgSel = crate::Reg<dbg_sel::DbgSelSpec>;
#[doc = "What submodules are accessed by the debug bus"]
pub mod dbg_sel;
#[doc = "EXRDFIFO_CFG (rw) register accessor: Fine tune DMA request generation\n\nYou can [`read`](crate::Reg::read) this register and get [`exrdfifo_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exrdfifo_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exrdfifo_cfg`]
module"]
#[doc(alias = "EXRDFIFO_CFG")]
pub type ExrdfifoCfg = crate::Reg<exrdfifo_cfg::ExrdfifoCfgSpec>;
#[doc = "Fine tune DMA request generation"]
pub mod exrdfifo_cfg;
#[doc = "EXRDFIFO_EN (rw) register accessor: Enable the extension data register\n\nYou can [`read`](crate::Reg::read) this register and get [`exrdfifo_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exrdfifo_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exrdfifo_en`]
module"]
#[doc(alias = "EXRDFIFO_EN")]
pub type ExrdfifoEn = crate::Reg<exrdfifo_en::ExrdfifoEnSpec>;
#[doc = "Enable the extension data register"]
pub mod exrdfifo_en;
#[doc = "TUNE_STEP (rw) register accessor: Sample clock delay step duration\n\nYou can [`read`](crate::Reg::read) this register and get [`tune_step::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tune_step::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tune_step`]
module"]
#[doc(alias = "TUNE_STEP")]
pub type TuneStep = crate::Reg<tune_step::TuneStepSpec>;
#[doc = "Sample clock delay step duration"]
pub mod tune_step;
#[doc = "TUNE_STEPS_STD (rw) register accessor: Sample clock delay step count for SDR\n\nYou can [`read`](crate::Reg::read) this register and get [`tune_steps_std::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tune_steps_std::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tune_steps_std`]
module"]
#[doc(alias = "TUNE_STEPS_STD")]
pub type TuneStepsStd = crate::Reg<tune_steps_std::TuneStepsStdSpec>;
#[doc = "Sample clock delay step count for SDR"]
pub mod tune_steps_std;
#[doc = "TUNE_STEPS_DDR (rw) register accessor: Sample clock delay step count for DDR\n\nYou can [`read`](crate::Reg::read) this register and get [`tune_steps_ddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tune_steps_ddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tune_steps_ddr`]
module"]
#[doc(alias = "TUNE_STEPS_DDR")]
pub type TuneStepsDdr = crate::Reg<tune_steps_ddr::TuneStepsDdrSpec>;
#[doc = "Sample clock delay step count for DDR"]
pub mod tune_steps_ddr;
#[doc = "SPI_INT_SPT (rw) register accessor: Interrupts in SPI mode depend on CS\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_int_spt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_int_spt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_int_spt`]
module"]
#[doc(alias = "SPI_INT_SPT")]
pub type SpiIntSpt = crate::Reg<spi_int_spt::SpiIntSptSpec>;
#[doc = "Interrupts in SPI mode depend on CS"]
pub mod spi_int_spt;
#[doc = "SLOTISR_VER (rw) register accessor: Version information and slot interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`slotisr_ver::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slotisr_ver::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slotisr_ver`]
module"]
#[doc(alias = "SLOTISR_VER")]
pub type SlotisrVer = crate::Reg<slotisr_ver::SlotisrVerSpec>;
#[doc = "Version information and slot interrupt status"]
pub mod slotisr_ver;

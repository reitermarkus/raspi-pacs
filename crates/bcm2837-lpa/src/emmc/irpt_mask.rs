#[doc = "Register `IRPT_MASK` reader"]
pub type R = crate::R<IrptMaskSpec>;
#[doc = "Register `IRPT_MASK` writer"]
pub type W = crate::W<IrptMaskSpec>;
#[doc = "Field `CMD_DONE` reader - Command has finished"]
pub type CmdDoneR = crate::BitReader;
#[doc = "Field `CMD_DONE` writer - Command has finished"]
pub type CmdDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_DONE` reader - Data transfer has finished"]
pub type DataDoneR = crate::BitReader;
#[doc = "Field `DATA_DONE` writer - Data transfer has finished"]
pub type DataDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLOCK_GAP` reader - Data transfer has stopped at block gap"]
pub type BlockGapR = crate::BitReader;
#[doc = "Field `BLOCK_GAP` writer - Data transfer has stopped at block gap"]
pub type BlockGapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_RDY` reader - DATA can be written to"]
pub type WriteRdyR = crate::BitReader;
#[doc = "Field `WRITE_RDY` writer - DATA can be written to"]
pub type WriteRdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_RDY` reader - DATA contains data to be read"]
pub type ReadRdyR = crate::BitReader;
#[doc = "Field `READ_RDY` writer - DATA contains data to be read"]
pub type ReadRdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD` reader - Card made interrupt request"]
pub type CardR = crate::BitReader;
#[doc = "Field `CARD` writer - Card made interrupt request"]
pub type CardW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETUNE` reader - Clock retune request"]
pub type RetuneR = crate::BitReader;
#[doc = "Field `RETUNE` writer - Clock retune request"]
pub type RetuneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOTACK` reader - Boot has been acknowledged"]
pub type BootackR = crate::BitReader;
#[doc = "Field `BOOTACK` writer - Boot has been acknowledged"]
pub type BootackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDBOOT` reader - Boot operation has terminated"]
pub type EndbootR = crate::BitReader;
#[doc = "Field `ENDBOOT` writer - Boot operation has terminated"]
pub type EndbootW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTO_ERR` reader - Command timeout"]
pub type CtoErrR = crate::BitReader;
#[doc = "Field `CTO_ERR` writer - Command timeout"]
pub type CtoErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCRC_ERR` reader - Command CRC error"]
pub type CcrcErrR = crate::BitReader;
#[doc = "Field `CCRC_ERR` writer - Command CRC error"]
pub type CcrcErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEND_ERR` reader - Command end bit error (not 1)"]
pub type CendErrR = crate::BitReader;
#[doc = "Field `CEND_ERR` writer - Command end bit error (not 1)"]
pub type CendErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBAD_ERR` reader - Incorrect response command index"]
pub type CbadErrR = crate::BitReader;
#[doc = "Field `CBAD_ERR` writer - Incorrect response command index"]
pub type CbadErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTO_ERR` reader - Data timeout"]
pub type DtoErrR = crate::BitReader;
#[doc = "Field `DTO_ERR` writer - Data timeout"]
pub type DtoErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCRC_ERR` reader - Data CRC error"]
pub type DcrcErrR = crate::BitReader;
#[doc = "Field `DCRC_ERR` writer - Data CRC error"]
pub type DcrcErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEND_ERR` reader - Data end bit error (not 1)"]
pub type DendErrR = crate::BitReader;
#[doc = "Field `DEND_ERR` writer - Data end bit error (not 1)"]
pub type DendErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMD_ERR` reader - Auto command error"]
pub type AcmdErrR = crate::BitReader;
#[doc = "Field `ACMD_ERR` writer - Auto command error"]
pub type AcmdErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Command has finished"]
    #[inline(always)]
    pub fn cmd_done(&self) -> CmdDoneR {
        CmdDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data transfer has finished"]
    #[inline(always)]
    pub fn data_done(&self) -> DataDoneR {
        DataDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data transfer has stopped at block gap"]
    #[inline(always)]
    pub fn block_gap(&self) -> BlockGapR {
        BlockGapR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - DATA can be written to"]
    #[inline(always)]
    pub fn write_rdy(&self) -> WriteRdyR {
        WriteRdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DATA contains data to be read"]
    #[inline(always)]
    pub fn read_rdy(&self) -> ReadRdyR {
        ReadRdyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Card made interrupt request"]
    #[inline(always)]
    pub fn card(&self) -> CardR {
        CardR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Clock retune request"]
    #[inline(always)]
    pub fn retune(&self) -> RetuneR {
        RetuneR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Boot has been acknowledged"]
    #[inline(always)]
    pub fn bootack(&self) -> BootackR {
        BootackR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Boot operation has terminated"]
    #[inline(always)]
    pub fn endboot(&self) -> EndbootR {
        EndbootR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Command timeout"]
    #[inline(always)]
    pub fn cto_err(&self) -> CtoErrR {
        CtoErrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Command CRC error"]
    #[inline(always)]
    pub fn ccrc_err(&self) -> CcrcErrR {
        CcrcErrR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Command end bit error (not 1)"]
    #[inline(always)]
    pub fn cend_err(&self) -> CendErrR {
        CendErrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Incorrect response command index"]
    #[inline(always)]
    pub fn cbad_err(&self) -> CbadErrR {
        CbadErrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Data timeout"]
    #[inline(always)]
    pub fn dto_err(&self) -> DtoErrR {
        DtoErrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Data CRC error"]
    #[inline(always)]
    pub fn dcrc_err(&self) -> DcrcErrR {
        DcrcErrR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Data end bit error (not 1)"]
    #[inline(always)]
    pub fn dend_err(&self) -> DendErrR {
        DendErrR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Auto command error"]
    #[inline(always)]
    pub fn acmd_err(&self) -> AcmdErrR {
        AcmdErrR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRPT_MASK")
            .field("acmd_err", &self.acmd_err())
            .field("dend_err", &self.dend_err())
            .field("dcrc_err", &self.dcrc_err())
            .field("dto_err", &self.dto_err())
            .field("cbad_err", &self.cbad_err())
            .field("cend_err", &self.cend_err())
            .field("ccrc_err", &self.ccrc_err())
            .field("cto_err", &self.cto_err())
            .field("endboot", &self.endboot())
            .field("bootack", &self.bootack())
            .field("retune", &self.retune())
            .field("card", &self.card())
            .field("read_rdy", &self.read_rdy())
            .field("write_rdy", &self.write_rdy())
            .field("block_gap", &self.block_gap())
            .field("data_done", &self.data_done())
            .field("cmd_done", &self.cmd_done())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Command has finished"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_done(&mut self) -> CmdDoneW<IrptMaskSpec> {
        CmdDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Data transfer has finished"]
    #[inline(always)]
    #[must_use]
    pub fn data_done(&mut self) -> DataDoneW<IrptMaskSpec> {
        DataDoneW::new(self, 1)
    }
    #[doc = "Bit 2 - Data transfer has stopped at block gap"]
    #[inline(always)]
    #[must_use]
    pub fn block_gap(&mut self) -> BlockGapW<IrptMaskSpec> {
        BlockGapW::new(self, 2)
    }
    #[doc = "Bit 4 - DATA can be written to"]
    #[inline(always)]
    #[must_use]
    pub fn write_rdy(&mut self) -> WriteRdyW<IrptMaskSpec> {
        WriteRdyW::new(self, 4)
    }
    #[doc = "Bit 5 - DATA contains data to be read"]
    #[inline(always)]
    #[must_use]
    pub fn read_rdy(&mut self) -> ReadRdyW<IrptMaskSpec> {
        ReadRdyW::new(self, 5)
    }
    #[doc = "Bit 8 - Card made interrupt request"]
    #[inline(always)]
    #[must_use]
    pub fn card(&mut self) -> CardW<IrptMaskSpec> {
        CardW::new(self, 8)
    }
    #[doc = "Bit 12 - Clock retune request"]
    #[inline(always)]
    #[must_use]
    pub fn retune(&mut self) -> RetuneW<IrptMaskSpec> {
        RetuneW::new(self, 12)
    }
    #[doc = "Bit 13 - Boot has been acknowledged"]
    #[inline(always)]
    #[must_use]
    pub fn bootack(&mut self) -> BootackW<IrptMaskSpec> {
        BootackW::new(self, 13)
    }
    #[doc = "Bit 14 - Boot operation has terminated"]
    #[inline(always)]
    #[must_use]
    pub fn endboot(&mut self) -> EndbootW<IrptMaskSpec> {
        EndbootW::new(self, 14)
    }
    #[doc = "Bit 16 - Command timeout"]
    #[inline(always)]
    #[must_use]
    pub fn cto_err(&mut self) -> CtoErrW<IrptMaskSpec> {
        CtoErrW::new(self, 16)
    }
    #[doc = "Bit 17 - Command CRC error"]
    #[inline(always)]
    #[must_use]
    pub fn ccrc_err(&mut self) -> CcrcErrW<IrptMaskSpec> {
        CcrcErrW::new(self, 17)
    }
    #[doc = "Bit 18 - Command end bit error (not 1)"]
    #[inline(always)]
    #[must_use]
    pub fn cend_err(&mut self) -> CendErrW<IrptMaskSpec> {
        CendErrW::new(self, 18)
    }
    #[doc = "Bit 19 - Incorrect response command index"]
    #[inline(always)]
    #[must_use]
    pub fn cbad_err(&mut self) -> CbadErrW<IrptMaskSpec> {
        CbadErrW::new(self, 19)
    }
    #[doc = "Bit 20 - Data timeout"]
    #[inline(always)]
    #[must_use]
    pub fn dto_err(&mut self) -> DtoErrW<IrptMaskSpec> {
        DtoErrW::new(self, 20)
    }
    #[doc = "Bit 21 - Data CRC error"]
    #[inline(always)]
    #[must_use]
    pub fn dcrc_err(&mut self) -> DcrcErrW<IrptMaskSpec> {
        DcrcErrW::new(self, 21)
    }
    #[doc = "Bit 22 - Data end bit error (not 1)"]
    #[inline(always)]
    #[must_use]
    pub fn dend_err(&mut self) -> DendErrW<IrptMaskSpec> {
        DendErrW::new(self, 22)
    }
    #[doc = "Bit 24 - Auto command error"]
    #[inline(always)]
    #[must_use]
    pub fn acmd_err(&mut self) -> AcmdErrW<IrptMaskSpec> {
        AcmdErrW::new(self, 24)
    }
}
#[doc = "Mask interrupts that change in INTERRUPT\n\nYou can [`read`](crate::Reg::read) this register and get [`irpt_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irpt_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrptMaskSpec;
impl crate::RegisterSpec for IrptMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irpt_mask::R`](R) reader structure"]
impl crate::Readable for IrptMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`irpt_mask::W`](W) writer structure"]
impl crate::Writable for IrptMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRPT_MASK to value 0"]
impl crate::Resettable for IrptMaskSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `CMD_INHIBIT` reader - Command line still in use"]
pub type CmdInhibitR = crate::BitReader;
#[doc = "Field `CMD_INHIBIT` writer - Command line still in use"]
pub type CmdInhibitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAT_INHIBIT` reader - Data lines still in use"]
pub type DatInhibitR = crate::BitReader;
#[doc = "Field `DAT_INHIBIT` writer - Data lines still in use"]
pub type DatInhibitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAT_ACTIVE` reader - At least one data line is active"]
pub type DatActiveR = crate::BitReader;
#[doc = "Field `DAT_ACTIVE` writer - At least one data line is active"]
pub type DatActiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_TRANSFER` reader - Write transfer is active"]
pub type WriteTransferR = crate::BitReader;
#[doc = "Field `WRITE_TRANSFER` writer - Write transfer is active"]
pub type WriteTransferW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_TRANSFER` reader - Read transfer is active"]
pub type ReadTransferR = crate::BitReader;
#[doc = "Field `READ_TRANSFER` writer - Read transfer is active"]
pub type ReadTransferW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFFER_WRITE_ENABLE` reader - The buffer has space for new data"]
pub type BufferWriteEnableR = crate::BitReader;
#[doc = "Field `BUFFER_WRITE_ENABLE` writer - The buffer has space for new data"]
pub type BufferWriteEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFFER_READ_ENABLE` reader - New data is available to read"]
pub type BufferReadEnableR = crate::BitReader;
#[doc = "Field `BUFFER_READ_ENABLE` writer - New data is available to read"]
pub type BufferReadEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAT_LEVEL0` reader - Value of DAT\\[3:0\\]"]
pub type DatLevel0R = crate::FieldReader;
#[doc = "Field `DAT_LEVEL0` writer - Value of DAT\\[3:0\\]"]
pub type DatLevel0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CMD_LEVEL` reader - Value of CMD"]
pub type CmdLevelR = crate::BitReader;
#[doc = "Field `CMD_LEVEL` writer - Value of CMD"]
pub type CmdLevelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAT_LEVEL1` reader - Value of DAT\\[7:4\\]"]
pub type DatLevel1R = crate::FieldReader;
#[doc = "Field `DAT_LEVEL1` writer - Value of DAT\\[7:4\\]"]
pub type DatLevel1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Command line still in use"]
    #[inline(always)]
    pub fn cmd_inhibit(&self) -> CmdInhibitR {
        CmdInhibitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data lines still in use"]
    #[inline(always)]
    pub fn dat_inhibit(&self) -> DatInhibitR {
        DatInhibitR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - At least one data line is active"]
    #[inline(always)]
    pub fn dat_active(&self) -> DatActiveR {
        DatActiveR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Write transfer is active"]
    #[inline(always)]
    pub fn write_transfer(&self) -> WriteTransferR {
        WriteTransferR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Read transfer is active"]
    #[inline(always)]
    pub fn read_transfer(&self) -> ReadTransferR {
        ReadTransferR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The buffer has space for new data"]
    #[inline(always)]
    pub fn buffer_write_enable(&self) -> BufferWriteEnableR {
        BufferWriteEnableR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - New data is available to read"]
    #[inline(always)]
    pub fn buffer_read_enable(&self) -> BufferReadEnableR {
        BufferReadEnableR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Value of DAT\\[3:0\\]"]
    #[inline(always)]
    pub fn dat_level0(&self) -> DatLevel0R {
        DatLevel0R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Value of CMD"]
    #[inline(always)]
    pub fn cmd_level(&self) -> CmdLevelR {
        CmdLevelR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:28 - Value of DAT\\[7:4\\]"]
    #[inline(always)]
    pub fn dat_level1(&self) -> DatLevel1R {
        DatLevel1R::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("dat_level1", &self.dat_level1())
            .field("cmd_level", &self.cmd_level())
            .field("dat_level0", &self.dat_level0())
            .field("buffer_read_enable", &self.buffer_read_enable())
            .field("buffer_write_enable", &self.buffer_write_enable())
            .field("read_transfer", &self.read_transfer())
            .field("write_transfer", &self.write_transfer())
            .field("dat_active", &self.dat_active())
            .field("dat_inhibit", &self.dat_inhibit())
            .field("cmd_inhibit", &self.cmd_inhibit())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Command line still in use"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_inhibit(&mut self) -> CmdInhibitW<StatusSpec> {
        CmdInhibitW::new(self, 0)
    }
    #[doc = "Bit 1 - Data lines still in use"]
    #[inline(always)]
    #[must_use]
    pub fn dat_inhibit(&mut self) -> DatInhibitW<StatusSpec> {
        DatInhibitW::new(self, 1)
    }
    #[doc = "Bit 2 - At least one data line is active"]
    #[inline(always)]
    #[must_use]
    pub fn dat_active(&mut self) -> DatActiveW<StatusSpec> {
        DatActiveW::new(self, 2)
    }
    #[doc = "Bit 8 - Write transfer is active"]
    #[inline(always)]
    #[must_use]
    pub fn write_transfer(&mut self) -> WriteTransferW<StatusSpec> {
        WriteTransferW::new(self, 8)
    }
    #[doc = "Bit 9 - Read transfer is active"]
    #[inline(always)]
    #[must_use]
    pub fn read_transfer(&mut self) -> ReadTransferW<StatusSpec> {
        ReadTransferW::new(self, 9)
    }
    #[doc = "Bit 10 - The buffer has space for new data"]
    #[inline(always)]
    #[must_use]
    pub fn buffer_write_enable(&mut self) -> BufferWriteEnableW<StatusSpec> {
        BufferWriteEnableW::new(self, 10)
    }
    #[doc = "Bit 11 - New data is available to read"]
    #[inline(always)]
    #[must_use]
    pub fn buffer_read_enable(&mut self) -> BufferReadEnableW<StatusSpec> {
        BufferReadEnableW::new(self, 11)
    }
    #[doc = "Bits 20:23 - Value of DAT\\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dat_level0(&mut self) -> DatLevel0W<StatusSpec> {
        DatLevel0W::new(self, 20)
    }
    #[doc = "Bit 24 - Value of CMD"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_level(&mut self) -> CmdLevelW<StatusSpec> {
        CmdLevelW::new(self, 24)
    }
    #[doc = "Bits 25:28 - Value of DAT\\[7:4\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dat_level1(&mut self) -> DatLevel1W<StatusSpec> {
        DatLevel1W::new(self, 25)
    }
}
#[doc = "Status info for debugging\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}

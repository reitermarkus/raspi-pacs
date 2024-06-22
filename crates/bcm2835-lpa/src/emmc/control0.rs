#[doc = "Register `CONTROL0` reader"]
pub type R = crate::R<Control0Spec>;
#[doc = "Register `CONTROL0` writer"]
pub type W = crate::W<Control0Spec>;
#[doc = "Field `HCTL_DWIDTH` reader - Use 4 data lines"]
pub type HctlDwidthR = crate::BitReader;
#[doc = "Field `HCTL_DWIDTH` writer - Use 4 data lines"]
pub type HctlDwidthW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCTL_HS_EN` reader - Enable high speed mode"]
pub type HctlHsEnR = crate::BitReader;
#[doc = "Field `HCTL_HS_EN` writer - Enable high speed mode"]
pub type HctlHsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCTL_8BIT` reader - Use 8 data lines"]
pub type Hctl8bitR = crate::BitReader;
#[doc = "Field `HCTL_8BIT` writer - Use 8 data lines"]
pub type Hctl8bitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAP_STOP` reader - Stop the current transaction at the next block gap"]
pub type GapStopR = crate::BitReader;
#[doc = "Field `GAP_STOP` writer - Stop the current transaction at the next block gap"]
pub type GapStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAP_RESTART` reader - Restart a transaction stopped by GAP_STOP"]
pub type GapRestartR = crate::BitReader;
#[doc = "Field `GAP_RESTART` writer - Restart a transaction stopped by GAP_STOP"]
pub type GapRestartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READWAIT_EN` reader - Use DAT2 read/wait protocol"]
pub type ReadwaitEnR = crate::BitReader;
#[doc = "Field `READWAIT_EN` writer - Use DAT2 read/wait protocol"]
pub type ReadwaitEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAP_IEN` reader - Enable interrupt on block gap"]
pub type GapIenR = crate::BitReader;
#[doc = "Field `GAP_IEN` writer - Enable interrupt on block gap"]
pub type GapIenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MODE` reader - Enable SPI mode"]
pub type SpiModeR = crate::BitReader;
#[doc = "Field `SPI_MODE` writer - Enable SPI mode"]
pub type SpiModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT_EN` reader - Boot mode enabled"]
pub type BootEnR = crate::BitReader;
#[doc = "Field `BOOT_EN` writer - Boot mode enabled"]
pub type BootEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALT_BOOT_EN` reader - Enable alternate boot mode"]
pub type AltBootEnR = crate::BitReader;
#[doc = "Field `ALT_BOOT_EN` writer - Enable alternate boot mode"]
pub type AltBootEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Use 4 data lines"]
    #[inline(always)]
    pub fn hctl_dwidth(&self) -> HctlDwidthR {
        HctlDwidthR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable high speed mode"]
    #[inline(always)]
    pub fn hctl_hs_en(&self) -> HctlHsEnR {
        HctlHsEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Use 8 data lines"]
    #[inline(always)]
    pub fn hctl_8bit(&self) -> Hctl8bitR {
        Hctl8bitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - Stop the current transaction at the next block gap"]
    #[inline(always)]
    pub fn gap_stop(&self) -> GapStopR {
        GapStopR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Restart a transaction stopped by GAP_STOP"]
    #[inline(always)]
    pub fn gap_restart(&self) -> GapRestartR {
        GapRestartR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Use DAT2 read/wait protocol"]
    #[inline(always)]
    pub fn readwait_en(&self) -> ReadwaitEnR {
        ReadwaitEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable interrupt on block gap"]
    #[inline(always)]
    pub fn gap_ien(&self) -> GapIenR {
        GapIenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable SPI mode"]
    #[inline(always)]
    pub fn spi_mode(&self) -> SpiModeR {
        SpiModeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Boot mode enabled"]
    #[inline(always)]
    pub fn boot_en(&self) -> BootEnR {
        BootEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable alternate boot mode"]
    #[inline(always)]
    pub fn alt_boot_en(&self) -> AltBootEnR {
        AltBootEnR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONTROL0")
            .field("alt_boot_en", &self.alt_boot_en())
            .field("boot_en", &self.boot_en())
            .field("spi_mode", &self.spi_mode())
            .field("gap_ien", &self.gap_ien())
            .field("readwait_en", &self.readwait_en())
            .field("gap_restart", &self.gap_restart())
            .field("gap_stop", &self.gap_stop())
            .field("hctl_8bit", &self.hctl_8bit())
            .field("hctl_hs_en", &self.hctl_hs_en())
            .field("hctl_dwidth", &self.hctl_dwidth())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Use 4 data lines"]
    #[inline(always)]
    #[must_use]
    pub fn hctl_dwidth(&mut self) -> HctlDwidthW<Control0Spec> {
        HctlDwidthW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable high speed mode"]
    #[inline(always)]
    #[must_use]
    pub fn hctl_hs_en(&mut self) -> HctlHsEnW<Control0Spec> {
        HctlHsEnW::new(self, 2)
    }
    #[doc = "Bit 5 - Use 8 data lines"]
    #[inline(always)]
    #[must_use]
    pub fn hctl_8bit(&mut self) -> Hctl8bitW<Control0Spec> {
        Hctl8bitW::new(self, 5)
    }
    #[doc = "Bit 16 - Stop the current transaction at the next block gap"]
    #[inline(always)]
    #[must_use]
    pub fn gap_stop(&mut self) -> GapStopW<Control0Spec> {
        GapStopW::new(self, 16)
    }
    #[doc = "Bit 17 - Restart a transaction stopped by GAP_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn gap_restart(&mut self) -> GapRestartW<Control0Spec> {
        GapRestartW::new(self, 17)
    }
    #[doc = "Bit 18 - Use DAT2 read/wait protocol"]
    #[inline(always)]
    #[must_use]
    pub fn readwait_en(&mut self) -> ReadwaitEnW<Control0Spec> {
        ReadwaitEnW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable interrupt on block gap"]
    #[inline(always)]
    #[must_use]
    pub fn gap_ien(&mut self) -> GapIenW<Control0Spec> {
        GapIenW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable SPI mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mode(&mut self) -> SpiModeW<Control0Spec> {
        SpiModeW::new(self, 20)
    }
    #[doc = "Bit 21 - Boot mode enabled"]
    #[inline(always)]
    #[must_use]
    pub fn boot_en(&mut self) -> BootEnW<Control0Spec> {
        BootEnW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable alternate boot mode"]
    #[inline(always)]
    #[must_use]
    pub fn alt_boot_en(&mut self) -> AltBootEnW<Control0Spec> {
        AltBootEnW::new(self, 22)
    }
}
#[doc = "Control\n\nYou can [`read`](crate::Reg::read) this register and get [`control0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Control0Spec;
impl crate::RegisterSpec for Control0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control0::R`](R) reader structure"]
impl crate::Readable for Control0Spec {}
#[doc = "`write(|w| ..)` method takes [`control0::W`](W) writer structure"]
impl crate::Writable for Control0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONTROL0 to value 0"]
impl crate::Resettable for Control0Spec {
    const RESET_VALUE: u32 = 0;
}

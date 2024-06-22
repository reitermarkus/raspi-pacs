#[doc = "Register `IMSC` reader"]
pub type R = crate::R<ImscSpec>;
#[doc = "Register `IMSC` writer"]
pub type W = crate::W<ImscSpec>;
#[doc = "Field `RIMIM` reader - RIMIM"]
pub type RimimR = crate::BitReader;
#[doc = "Field `RIMIM` writer - RIMIM"]
pub type RimimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSMIM` reader - CTSMIM"]
pub type CtsmimR = crate::BitReader;
#[doc = "Field `CTSMIM` writer - CTSMIM"]
pub type CtsmimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDMIM` reader - DCDMIM"]
pub type DcdmimR = crate::BitReader;
#[doc = "Field `DCDMIM` writer - DCDMIM"]
pub type DcdmimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSRMIM` reader - DSRMIM"]
pub type DsrmimR = crate::BitReader;
#[doc = "Field `DSRMIM` writer - DSRMIM"]
pub type DsrmimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIM` reader - RXIM"]
pub type RximR = crate::BitReader;
#[doc = "Field `RXIM` writer - RXIM"]
pub type RximW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIM` reader - TXIM"]
pub type TximR = crate::BitReader;
#[doc = "Field `TXIM` writer - TXIM"]
pub type TximW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTIM` reader - RTIM"]
pub type RtimR = crate::BitReader;
#[doc = "Field `RTIM` writer - RTIM"]
pub type RtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEIM` reader - FEIM"]
pub type FeimR = crate::BitReader;
#[doc = "Field `FEIM` writer - FEIM"]
pub type FeimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEIM` reader - PEIM"]
pub type PeimR = crate::BitReader;
#[doc = "Field `PEIM` writer - PEIM"]
pub type PeimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEIM` reader - BEIM"]
pub type BeimR = crate::BitReader;
#[doc = "Field `BEIM` writer - BEIM"]
pub type BeimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEIM` reader - OEIM"]
pub type OeimR = crate::BitReader;
#[doc = "Field `OEIM` writer - OEIM"]
pub type OeimW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RIMIM"]
    #[inline(always)]
    pub fn rimim(&self) -> RimimR {
        RimimR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CTSMIM"]
    #[inline(always)]
    pub fn ctsmim(&self) -> CtsmimR {
        CtsmimR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DCDMIM"]
    #[inline(always)]
    pub fn dcdmim(&self) -> DcdmimR {
        DcdmimR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DSRMIM"]
    #[inline(always)]
    pub fn dsrmim(&self) -> DsrmimR {
        DsrmimR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RXIM"]
    #[inline(always)]
    pub fn rxim(&self) -> RximR {
        RximR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TXIM"]
    #[inline(always)]
    pub fn txim(&self) -> TximR {
        TximR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RTIM"]
    #[inline(always)]
    pub fn rtim(&self) -> RtimR {
        RtimR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FEIM"]
    #[inline(always)]
    pub fn feim(&self) -> FeimR {
        FeimR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PEIM"]
    #[inline(always)]
    pub fn peim(&self) -> PeimR {
        PeimR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BEIM"]
    #[inline(always)]
    pub fn beim(&self) -> BeimR {
        BeimR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - OEIM"]
    #[inline(always)]
    pub fn oeim(&self) -> OeimR {
        OeimR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMSC")
            .field("rimim", &self.rimim())
            .field("ctsmim", &self.ctsmim())
            .field("dcdmim", &self.dcdmim())
            .field("dsrmim", &self.dsrmim())
            .field("rxim", &self.rxim())
            .field("txim", &self.txim())
            .field("rtim", &self.rtim())
            .field("feim", &self.feim())
            .field("peim", &self.peim())
            .field("beim", &self.beim())
            .field("oeim", &self.oeim())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - RIMIM"]
    #[inline(always)]
    #[must_use]
    pub fn rimim(&mut self) -> RimimW<ImscSpec> {
        RimimW::new(self, 0)
    }
    #[doc = "Bit 1 - CTSMIM"]
    #[inline(always)]
    #[must_use]
    pub fn ctsmim(&mut self) -> CtsmimW<ImscSpec> {
        CtsmimW::new(self, 1)
    }
    #[doc = "Bit 2 - DCDMIM"]
    #[inline(always)]
    #[must_use]
    pub fn dcdmim(&mut self) -> DcdmimW<ImscSpec> {
        DcdmimW::new(self, 2)
    }
    #[doc = "Bit 3 - DSRMIM"]
    #[inline(always)]
    #[must_use]
    pub fn dsrmim(&mut self) -> DsrmimW<ImscSpec> {
        DsrmimW::new(self, 3)
    }
    #[doc = "Bit 4 - RXIM"]
    #[inline(always)]
    #[must_use]
    pub fn rxim(&mut self) -> RximW<ImscSpec> {
        RximW::new(self, 4)
    }
    #[doc = "Bit 5 - TXIM"]
    #[inline(always)]
    #[must_use]
    pub fn txim(&mut self) -> TximW<ImscSpec> {
        TximW::new(self, 5)
    }
    #[doc = "Bit 6 - RTIM"]
    #[inline(always)]
    #[must_use]
    pub fn rtim(&mut self) -> RtimW<ImscSpec> {
        RtimW::new(self, 6)
    }
    #[doc = "Bit 7 - FEIM"]
    #[inline(always)]
    #[must_use]
    pub fn feim(&mut self) -> FeimW<ImscSpec> {
        FeimW::new(self, 7)
    }
    #[doc = "Bit 8 - PEIM"]
    #[inline(always)]
    #[must_use]
    pub fn peim(&mut self) -> PeimW<ImscSpec> {
        PeimW::new(self, 8)
    }
    #[doc = "Bit 9 - BEIM"]
    #[inline(always)]
    #[must_use]
    pub fn beim(&mut self) -> BeimW<ImscSpec> {
        BeimW::new(self, 9)
    }
    #[doc = "Bit 10 - OEIM"]
    #[inline(always)]
    #[must_use]
    pub fn oeim(&mut self) -> OeimW<ImscSpec> {
        OeimW::new(self, 10)
    }
}
#[doc = "Interrupt Mask set_Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imsc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imsc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImscSpec;
impl crate::RegisterSpec for ImscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imsc::R`](R) reader structure"]
impl crate::Readable for ImscSpec {}
#[doc = "`write(|w| ..)` method takes [`imsc::W`](W) writer structure"]
impl crate::Writable for ImscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMSC to value 0"]
impl crate::Resettable for ImscSpec {
    const RESET_VALUE: u32 = 0;
}

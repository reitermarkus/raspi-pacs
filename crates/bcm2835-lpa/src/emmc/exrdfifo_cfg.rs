#[doc = "Register `EXRDFIFO_CFG` reader"]
pub type R = crate::R<ExrdfifoCfgSpec>;
#[doc = "Register `EXRDFIFO_CFG` writer"]
pub type W = crate::W<ExrdfifoCfgSpec>;
#[doc = "Field `RD_THRSH` reader - Read threshold in 32 bit words"]
pub type RdThrshR = crate::FieldReader;
#[doc = "Field `RD_THRSH` writer - Read threshold in 32 bit words"]
pub type RdThrshW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Read threshold in 32 bit words"]
    #[inline(always)]
    pub fn rd_thrsh(&self) -> RdThrshR {
        RdThrshR::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXRDFIFO_CFG")
            .field("rd_thrsh", &self.rd_thrsh())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Read threshold in 32 bit words"]
    #[inline(always)]
    #[must_use]
    pub fn rd_thrsh(&mut self) -> RdThrshW<ExrdfifoCfgSpec> {
        RdThrshW::new(self, 0)
    }
}
#[doc = "Fine tune DMA request generation\n\nYou can [`read`](crate::Reg::read) this register and get [`exrdfifo_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exrdfifo_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExrdfifoCfgSpec;
impl crate::RegisterSpec for ExrdfifoCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exrdfifo_cfg::R`](R) reader structure"]
impl crate::Readable for ExrdfifoCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`exrdfifo_cfg::W`](W) writer structure"]
impl crate::Writable for ExrdfifoCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXRDFIFO_CFG to value 0"]
impl crate::Resettable for ExrdfifoCfgSpec {
    const RESET_VALUE: u32 = 0;
}

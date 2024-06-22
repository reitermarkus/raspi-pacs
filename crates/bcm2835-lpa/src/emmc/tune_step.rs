#[doc = "Register `TUNE_STEP` reader"]
pub type R = crate::R<TuneStepSpec>;
#[doc = "Register `TUNE_STEP` writer"]
pub type W = crate::W<TuneStepSpec>;
#[doc = "Field `DELAY` reader - "]
pub type DelayR = crate::FieldReader;
#[doc = "Field `DELAY` writer - "]
pub type DelayW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn delay(&self) -> DelayR {
        DelayR::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TUNE_STEP")
            .field("delay", &self.delay())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn delay(&mut self) -> DelayW<TuneStepSpec> {
        DelayW::new(self, 0)
    }
}
#[doc = "Sample clock delay step duration\n\nYou can [`read`](crate::Reg::read) this register and get [`tune_step::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tune_step::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TuneStepSpec;
impl crate::RegisterSpec for TuneStepSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tune_step::R`](R) reader structure"]
impl crate::Readable for TuneStepSpec {}
#[doc = "`write(|w| ..)` method takes [`tune_step::W`](W) writer structure"]
impl crate::Writable for TuneStepSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TUNE_STEP to value 0"]
impl crate::Resettable for TuneStepSpec {
    const RESET_VALUE: u32 = 0;
}

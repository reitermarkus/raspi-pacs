#[doc = "Register `TUNE_STEPS_STD` reader"]
pub type R = crate::R<TuneStepsStdSpec>;
#[doc = "Register `TUNE_STEPS_STD` writer"]
pub type W = crate::W<TuneStepsStdSpec>;
#[doc = "Field `STEPS` reader - "]
pub type StepsR = crate::FieldReader;
#[doc = "Field `STEPS` writer - "]
pub type StepsW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn steps(&self) -> StepsR {
        StepsR::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TUNE_STEPS_STD")
            .field("steps", &self.steps())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn steps(&mut self) -> StepsW<TuneStepsStdSpec> {
        StepsW::new(self, 0)
    }
}
#[doc = "Sample clock delay step count for SDR\n\nYou can [`read`](crate::Reg::read) this register and get [`tune_steps_std::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tune_steps_std::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TuneStepsStdSpec;
impl crate::RegisterSpec for TuneStepsStdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tune_steps_std::R`](R) reader structure"]
impl crate::Readable for TuneStepsStdSpec {}
#[doc = "`write(|w| ..)` method takes [`tune_steps_std::W`](W) writer structure"]
impl crate::Writable for TuneStepsStdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TUNE_STEPS_STD to value 0"]
impl crate::Resettable for TuneStepsStdSpec {
    const RESET_VALUE: u32 = 0;
}

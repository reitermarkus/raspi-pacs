#[doc = "Register `CLO` reader"]
pub type R = crate::R<CloSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Lower 32 bits for the free running counter\n\nYou can [`read`](crate::Reg::read) this register and get [`clo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CloSpec;
impl crate::RegisterSpec for CloSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clo::R`](R) reader structure"]
impl crate::Readable for CloSpec {}
#[doc = "`reset()` method sets CLO to value 0"]
impl crate::Resettable for CloSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `SCRATCH` reader"]
pub type R = crate::R<ScratchSpec>;
#[doc = "Register `SCRATCH` writer"]
pub type W = crate::W<ScratchSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Scratch\n\nYou can [`read`](crate::Reg::read) this register and get [`scratch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scratch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScratchSpec;
impl crate::RegisterSpec for ScratchSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`scratch::R`](R) reader structure"]
impl crate::Readable for ScratchSpec {}
#[doc = "`write(|w| ..)` method takes [`scratch::W`](W) writer structure"]
impl crate::Writable for ScratchSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SCRATCH to value 0"]
impl crate::Resettable for ScratchSpec {
    const RESET_VALUE: u8 = 0;
}

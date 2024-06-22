#[doc = "Register `RNG2` reader"]
pub type R = crate::R<Rng2Spec>;
#[doc = "Register `RNG2` writer"]
pub type W = crate::W<Rng2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Range for channel 2\n\nYou can [`read`](crate::Reg::read) this register and get [`rng2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rng2Spec;
impl crate::RegisterSpec for Rng2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng2::R`](R) reader structure"]
impl crate::Readable for Rng2Spec {}
#[doc = "`write(|w| ..)` method takes [`rng2::W`](W) writer structure"]
impl crate::Writable for Rng2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNG2 to value 0x20"]
impl crate::Resettable for Rng2Spec {
    const RESET_VALUE: u32 = 0x20;
}

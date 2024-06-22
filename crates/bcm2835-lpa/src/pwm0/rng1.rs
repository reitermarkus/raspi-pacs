#[doc = "Register `RNG1` reader"]
pub type R = crate::R<Rng1Spec>;
#[doc = "Register `RNG1` writer"]
pub type W = crate::W<Rng1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Range for channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rng1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rng1Spec;
impl crate::RegisterSpec for Rng1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng1::R`](R) reader structure"]
impl crate::Readable for Rng1Spec {}
#[doc = "`write(|w| ..)` method takes [`rng1::W`](W) writer structure"]
impl crate::Writable for Rng1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNG1 to value 0x20"]
impl crate::Resettable for Rng1Spec {
    const RESET_VALUE: u32 = 0x20;
}

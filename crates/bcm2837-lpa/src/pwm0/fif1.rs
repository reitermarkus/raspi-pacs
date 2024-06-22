#[doc = "Register `FIF1` writer"]
pub type W = crate::W<Fif1Spec>;
impl core::fmt::Debug for crate::generic::Reg<Fif1Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "FIFO input\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fif1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fif1Spec;
impl crate::RegisterSpec for Fif1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fif1::W`](W) writer structure"]
impl crate::Writable for Fif1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIF1 to value 0"]
impl crate::Resettable for Fif1Spec {
    const RESET_VALUE: u32 = 0;
}

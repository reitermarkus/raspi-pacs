#[doc = "Register `C0` reader"]
pub type R = crate::R<C0Spec>;
#[doc = "Register `C0` writer"]
pub type W = crate::W<C0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Compare channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`c0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C0Spec;
impl crate::RegisterSpec for C0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c0::R`](R) reader structure"]
impl crate::Readable for C0Spec {}
#[doc = "`write(|w| ..)` method takes [`c0::W`](W) writer structure"]
impl crate::Writable for C0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C0 to value 0"]
impl crate::Resettable for C0Spec {
    const RESET_VALUE: u32 = 0;
}

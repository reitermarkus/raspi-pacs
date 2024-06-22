#[doc = "Register `ARG2` reader"]
pub type R = crate::R<Arg2Spec>;
#[doc = "Register `ARG2` writer"]
pub type W = crate::W<Arg2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Argument for ACMD23 command\n\nYou can [`read`](crate::Reg::read) this register and get [`arg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Arg2Spec;
impl crate::RegisterSpec for Arg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arg2::R`](R) reader structure"]
impl crate::Readable for Arg2Spec {}
#[doc = "`write(|w| ..)` method takes [`arg2::W`](W) writer structure"]
impl crate::Writable for Arg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARG2 to value 0"]
impl crate::Resettable for Arg2Spec {
    const RESET_VALUE: u32 = 0;
}

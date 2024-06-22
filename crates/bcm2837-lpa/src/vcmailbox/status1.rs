#[doc = "Register `STATUS1` reader"]
pub type R = crate::R<Status1Spec>;
#[doc = "Register `STATUS1` writer"]
pub type W = crate::W<Status1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`status1::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Status1Spec;
impl crate::RegisterSpec for Status1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status1::R`](R) reader structure"]
impl crate::Readable for Status1Spec {}
#[doc = "`write(|w| ..)` method takes [`status1::W`](W) writer structure"]
impl crate::Writable for Status1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}

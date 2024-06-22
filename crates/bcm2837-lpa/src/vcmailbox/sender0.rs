#[doc = "Register `SENDER0` reader"]
pub type R = crate::R<Sender0Spec>;
#[doc = "Register `SENDER0` writer"]
pub type W = crate::W<Sender0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sender0::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sender0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sender0Spec;
impl crate::RegisterSpec for Sender0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sender0::R`](R) reader structure"]
impl crate::Readable for Sender0Spec {}
#[doc = "`write(|w| ..)` method takes [`sender0::W`](W) writer structure"]
impl crate::Writable for Sender0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}

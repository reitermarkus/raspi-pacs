#[doc = "Register `SENDER1` reader"]
pub type R = crate::R<Sender1Spec>;
#[doc = "Register `SENDER1` writer"]
pub type W = crate::W<Sender1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sender1::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sender1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sender1Spec;
impl crate::RegisterSpec for Sender1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sender1::R`](R) reader structure"]
impl crate::Readable for Sender1Spec {}
#[doc = "`write(|w| ..)` method takes [`sender1::W`](W) writer structure"]
impl crate::Writable for Sender1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}

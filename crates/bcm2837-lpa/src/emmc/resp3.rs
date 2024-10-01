#[doc = "Register `RESP3` reader"]
pub type R = crate::R<Resp3Spec>;
#[doc = "Register `RESP3` writer"]
pub type W = crate::W<Resp3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Bits 127:96 of CMD2 and CMD10 responses\n\nYou can [`read`](crate::Reg::read) this register and get [`resp3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resp3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Resp3Spec;
impl crate::RegisterSpec for Resp3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp3::R`](R) reader structure"]
impl crate::Readable for Resp3Spec {}
#[doc = "`write(|w| ..)` method takes [`resp3::W`](W) writer structure"]
impl crate::Writable for Resp3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESP3 to value 0"]
impl crate::Resettable for Resp3Spec {
    const RESET_VALUE: u32 = 0;
}

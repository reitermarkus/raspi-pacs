#[doc = "Register `RESP1` reader"]
pub type R = crate::R<Resp1Spec>;
#[doc = "Register `RESP1` writer"]
pub type W = crate::W<Resp1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Bits 63:32 of CMD2 and CMD10 responses\n\nYou can [`read`](crate::Reg::read) this register and get [`resp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Resp1Spec;
impl crate::RegisterSpec for Resp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp1::R`](R) reader structure"]
impl crate::Readable for Resp1Spec {}
#[doc = "`write(|w| ..)` method takes [`resp1::W`](W) writer structure"]
impl crate::Writable for Resp1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESP1 to value 0"]
impl crate::Resettable for Resp1Spec {
    const RESET_VALUE: u32 = 0;
}

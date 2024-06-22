#[doc = "Register `PEEK0` reader"]
pub type R = crate::R<Peek0Spec>;
#[doc = "Register `PEEK0` writer"]
pub type W = crate::W<Peek0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`peek0::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peek0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Peek0Spec;
impl crate::RegisterSpec for Peek0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peek0::R`](R) reader structure"]
impl crate::Readable for Peek0Spec {}
#[doc = "`write(|w| ..)` method takes [`peek0::W`](W) writer structure"]
impl crate::Writable for Peek0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}

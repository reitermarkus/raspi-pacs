#[doc = "Register `DAT1` reader"]
pub type R = crate::R<Dat1Spec>;
#[doc = "Register `DAT1` writer"]
pub type W = crate::W<Dat1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Channel 1 data\n\nYou can [`read`](crate::Reg::read) this register and get [`dat1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dat1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat1Spec;
impl crate::RegisterSpec for Dat1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dat1::R`](R) reader structure"]
impl crate::Readable for Dat1Spec {}
#[doc = "`write(|w| ..)` method takes [`dat1::W`](W) writer structure"]
impl crate::Writable for Dat1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAT1 to value 0"]
impl crate::Resettable for Dat1Spec {
    const RESET_VALUE: u32 = 0;
}

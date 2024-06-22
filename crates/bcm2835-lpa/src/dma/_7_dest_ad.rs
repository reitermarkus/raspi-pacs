#[doc = "Register `7_DEST_AD` reader"]
pub type R = crate::R<_7DestAdSpec>;
#[doc = "Register `7_DEST_AD` writer"]
pub type W = crate::W<_7DestAdSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Channel 7 CB Word 2 (Destination Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_7_dest_ad::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_7_dest_ad::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _7DestAdSpec;
impl crate::RegisterSpec for _7DestAdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_7_dest_ad::R`](R) reader structure"]
impl crate::Readable for _7DestAdSpec {}
#[doc = "`write(|w| ..)` method takes [`_7_dest_ad::W`](W) writer structure"]
impl crate::Writable for _7DestAdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets 7_DEST_AD to value 0"]
impl crate::Resettable for _7DestAdSpec {
    const RESET_VALUE: u32 = 0;
}

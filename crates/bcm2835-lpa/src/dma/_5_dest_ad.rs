#[doc = "Register `5_DEST_AD` reader"]
pub type R = crate::R<_5DestAdSpec>;
#[doc = "Register `5_DEST_AD` writer"]
pub type W = crate::W<_5DestAdSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Channel 5 CB Word 2 (Destination Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_5_dest_ad::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_5_dest_ad::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _5DestAdSpec;
impl crate::RegisterSpec for _5DestAdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_5_dest_ad::R`](R) reader structure"]
impl crate::Readable for _5DestAdSpec {}
#[doc = "`write(|w| ..)` method takes [`_5_dest_ad::W`](W) writer structure"]
impl crate::Writable for _5DestAdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets 5_DEST_AD to value 0"]
impl crate::Resettable for _5DestAdSpec {
    const RESET_VALUE: u32 = 0;
}

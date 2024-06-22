#[doc = "Register `6_DEST_AD` reader"]
pub type R = crate::R<_6DestAdSpec>;
#[doc = "Register `6_DEST_AD` writer"]
pub type W = crate::W<_6DestAdSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Channel 6 CB Word 2 (Destination Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_6_dest_ad::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_6_dest_ad::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _6DestAdSpec;
impl crate::RegisterSpec for _6DestAdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_6_dest_ad::R`](R) reader structure"]
impl crate::Readable for _6DestAdSpec {}
#[doc = "`write(|w| ..)` method takes [`_6_dest_ad::W`](W) writer structure"]
impl crate::Writable for _6DestAdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets 6_DEST_AD to value 0"]
impl crate::Resettable for _6DestAdSpec {
    const RESET_VALUE: u32 = 0;
}

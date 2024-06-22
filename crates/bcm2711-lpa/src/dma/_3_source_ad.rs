#[doc = "Register `3_SOURCE_AD` reader"]
pub type R = crate::R<_3SourceAdSpec>;
#[doc = "Register `3_SOURCE_AD` writer"]
pub type W = crate::W<_3SourceAdSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Channel 3 CB Word 1 (Source Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_3_source_ad::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3_source_ad::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _3SourceAdSpec;
impl crate::RegisterSpec for _3SourceAdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_3_source_ad::R`](R) reader structure"]
impl crate::Readable for _3SourceAdSpec {}
#[doc = "`write(|w| ..)` method takes [`_3_source_ad::W`](W) writer structure"]
impl crate::Writable for _3SourceAdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets 3_SOURCE_AD to value 0"]
impl crate::Resettable for _3SourceAdSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `12_SOURCE_AD` reader"]
pub type R = crate::R<_12SourceAdSpec>;
#[doc = "Register `12_SOURCE_AD` writer"]
pub type W = crate::W<_12SourceAdSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Channel 12 CB Word 1 (Source Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_12_source_ad::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_12_source_ad::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _12SourceAdSpec;
impl crate::RegisterSpec for _12SourceAdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_12_source_ad::R`](R) reader structure"]
impl crate::Readable for _12SourceAdSpec {}
#[doc = "`write(|w| ..)` method takes [`_12_source_ad::W`](W) writer structure"]
impl crate::Writable for _12SourceAdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets 12_SOURCE_AD to value 0"]
impl crate::Resettable for _12SourceAdSpec {
    const RESET_VALUE: u32 = 0;
}

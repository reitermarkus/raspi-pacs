#[doc = "Register `10_STRIDE` reader"]
pub type R = crate::R<_10StrideSpec>;
#[doc = "Register `10_STRIDE` writer"]
pub type W = crate::W<_10StrideSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Channel 10 CB Word 4 (2D Stride)\n\nYou can [`read`](crate::Reg::read) this register and get [`_10_stride::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_10_stride::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _10StrideSpec;
impl crate::RegisterSpec for _10StrideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_10_stride::R`](R) reader structure"]
impl crate::Readable for _10StrideSpec {}
#[doc = "`write(|w| ..)` method takes [`_10_stride::W`](W) writer structure"]
impl crate::Writable for _10StrideSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets 10_STRIDE to value 0"]
impl crate::Resettable for _10StrideSpec {
    const RESET_VALUE: u32 = 0;
}

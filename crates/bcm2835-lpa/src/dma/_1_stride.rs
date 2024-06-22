#[doc = "Register `1_STRIDE` reader"]
pub type R = crate::R<_1StrideSpec>;
#[doc = "Register `1_STRIDE` writer"]
pub type W = crate::W<_1StrideSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Channel 1 CB Word 4 (2D Stride)\n\nYou can [`read`](crate::Reg::read) this register and get [`_1_stride::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_1_stride::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _1StrideSpec;
impl crate::RegisterSpec for _1StrideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_1_stride::R`](R) reader structure"]
impl crate::Readable for _1StrideSpec {}
#[doc = "`write(|w| ..)` method takes [`_1_stride::W`](W) writer structure"]
impl crate::Writable for _1StrideSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets 1_STRIDE to value 0"]
impl crate::Resettable for _1StrideSpec {
    const RESET_VALUE: u32 = 0;
}

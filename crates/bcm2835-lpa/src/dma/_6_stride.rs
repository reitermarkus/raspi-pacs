#[doc = "Register `6_STRIDE` reader"]
pub type R = crate::R<_6StrideSpec>;
#[doc = "Register `6_STRIDE` writer"]
pub type W = crate::W<_6StrideSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Channel 6 CB Word 4 (2D Stride)\n\nYou can [`read`](crate::Reg::read) this register and get [`_6_stride::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_6_stride::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _6StrideSpec;
impl crate::RegisterSpec for _6StrideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_6_stride::R`](R) reader structure"]
impl crate::Readable for _6StrideSpec {}
#[doc = "`write(|w| ..)` method takes [`_6_stride::W`](W) writer structure"]
impl crate::Writable for _6StrideSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets 6_STRIDE to value 0"]
impl crate::Resettable for _6StrideSpec {
    const RESET_VALUE: u32 = 0;
}

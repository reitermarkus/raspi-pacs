#[doc = "Register `11_STRIDE` reader"]
pub type R = crate::R<_11StrideSpec>;
#[doc = "Register `11_STRIDE` writer"]
pub type W = crate::W<_11StrideSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Channel 11 CB Word 4 (2D Stride)\n\nYou can [`read`](crate::Reg::read) this register and get [`_11_stride::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_11_stride::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _11StrideSpec;
impl crate::RegisterSpec for _11StrideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_11_stride::R`](R) reader structure"]
impl crate::Readable for _11StrideSpec {}
#[doc = "`write(|w| ..)` method takes [`_11_stride::W`](W) writer structure"]
impl crate::Writable for _11StrideSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets 11_STRIDE to value 0"]
impl crate::Resettable for _11StrideSpec {
    const RESET_VALUE: u32 = 0;
}

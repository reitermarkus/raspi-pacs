#[doc = "Register `3_TI` reader"]
pub type R = crate::R<_3TiSpec>;
#[doc = "Register `3_TI` writer"]
pub type W = crate::W<_3TiSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Channel 3 CB Word 0 (Transfer Information)\n\nYou can [`read`](crate::Reg::read) this register and get [`_3_ti::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3_ti::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _3TiSpec;
impl crate::RegisterSpec for _3TiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_3_ti::R`](R) reader structure"]
impl crate::Readable for _3TiSpec {}
#[doc = "`write(|w| ..)` method takes [`_3_ti::W`](W) writer structure"]
impl crate::Writable for _3TiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets 3_TI to value 0"]
impl crate::Resettable for _3TiSpec {
    const RESET_VALUE: u32 = 0;
}

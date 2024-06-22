#[doc = "Register `14_NEXTCONBK` reader"]
pub type R = crate::R<_14NextconbkSpec>;
#[doc = "Register `14_NEXTCONBK` writer"]
pub type W = crate::W<_14NextconbkSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Channel 14 CB Word 5 (Next CB Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_14_nextconbk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_14_nextconbk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _14NextconbkSpec;
impl crate::RegisterSpec for _14NextconbkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_14_nextconbk::R`](R) reader structure"]
impl crate::Readable for _14NextconbkSpec {}
#[doc = "`write(|w| ..)` method takes [`_14_nextconbk::W`](W) writer structure"]
impl crate::Writable for _14NextconbkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets 14_NEXTCONBK to value 0"]
impl crate::Resettable for _14NextconbkSpec {
    const RESET_VALUE: u32 = 0;
}

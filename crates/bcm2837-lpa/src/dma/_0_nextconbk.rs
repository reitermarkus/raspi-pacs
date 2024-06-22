#[doc = "Register `0_NEXTCONBK` reader"]
pub type R = crate::R<_0NextconbkSpec>;
#[doc = "Register `0_NEXTCONBK` writer"]
pub type W = crate::W<_0NextconbkSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Channel 0 CB Word 5 (Next CB Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`_0_nextconbk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_0_nextconbk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _0NextconbkSpec;
impl crate::RegisterSpec for _0NextconbkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_0_nextconbk::R`](R) reader structure"]
impl crate::Readable for _0NextconbkSpec {}
#[doc = "`write(|w| ..)` method takes [`_0_nextconbk::W`](W) writer structure"]
impl crate::Writable for _0NextconbkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets 0_NEXTCONBK to value 0"]
impl crate::Resettable for _0NextconbkSpec {
    const RESET_VALUE: u32 = 0;
}

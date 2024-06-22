#[doc = "Register `0_TXFR_LEN` reader"]
pub type R = crate::R<_0TxfrLenSpec>;
#[doc = "Register `0_TXFR_LEN` writer"]
pub type W = crate::W<_0TxfrLenSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA Channel 0 CB Word 3 (Transfer Length)\n\nYou can [`read`](crate::Reg::read) this register and get [`_0_txfr_len::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_0_txfr_len::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _0TxfrLenSpec;
impl crate::RegisterSpec for _0TxfrLenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_0_txfr_len::R`](R) reader structure"]
impl crate::Readable for _0TxfrLenSpec {}
#[doc = "`write(|w| ..)` method takes [`_0_txfr_len::W`](W) writer structure"]
impl crate::Writable for _0TxfrLenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets 0_TXFR_LEN to value 0"]
impl crate::Resettable for _0TxfrLenSpec {
    const RESET_VALUE: u32 = 0;
}

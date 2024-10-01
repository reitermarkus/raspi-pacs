#[doc = "Register `GICC_APR0` reader"]
pub type R = crate::R<GiccApr0Spec>;
#[doc = "Register `GICC_APR0` writer"]
pub type W = crate::W<GiccApr0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Active Priority\n\nYou can [`read`](crate::Reg::read) this register and get [`gicc_apr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicc_apr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiccApr0Spec;
impl crate::RegisterSpec for GiccApr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicc_apr0::R`](R) reader structure"]
impl crate::Readable for GiccApr0Spec {}
#[doc = "`write(|w| ..)` method takes [`gicc_apr0::W`](W) writer structure"]
impl crate::Writable for GiccApr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICC_APR0 to value 0"]
impl crate::Resettable for GiccApr0Spec {
    const RESET_VALUE: u32 = 0;
}

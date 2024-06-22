#[doc = "Register `GICD_CPENDSGIRn` reader"]
pub type R = crate::R<GicdCpendsgirnSpec>;
#[doc = "Register `GICD_CPENDSGIRn` writer"]
pub type W = crate::W<GicdCpendsgirnSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "SGI Clear-Pending Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_cpendsgirn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_cpendsgirn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdCpendsgirnSpec;
impl crate::RegisterSpec for GicdCpendsgirnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_cpendsgirn::R`](R) reader structure"]
impl crate::Readable for GicdCpendsgirnSpec {}
#[doc = "`write(|w| ..)` method takes [`gicd_cpendsgirn::W`](W) writer structure"]
impl crate::Writable for GicdCpendsgirnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_CPENDSGIRn to value 0"]
impl crate::Resettable for GicdCpendsgirnSpec {
    const RESET_VALUE: u32 = 0;
}

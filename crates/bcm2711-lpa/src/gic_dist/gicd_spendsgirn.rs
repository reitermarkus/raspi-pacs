#[doc = "Register `GICD_SPENDSGIRn` reader"]
pub type R = crate::R<GicdSpendsgirnSpec>;
#[doc = "Register `GICD_SPENDSGIRn` writer"]
pub type W = crate::W<GicdSpendsgirnSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "SGI Set-Pending Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_spendsgirn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_spendsgirn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdSpendsgirnSpec;
impl crate::RegisterSpec for GicdSpendsgirnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_spendsgirn::R`](R) reader structure"]
impl crate::Readable for GicdSpendsgirnSpec {}
#[doc = "`write(|w| ..)` method takes [`gicd_spendsgirn::W`](W) writer structure"]
impl crate::Writable for GicdSpendsgirnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_SPENDSGIRn to value 0"]
impl crate::Resettable for GicdSpendsgirnSpec {
    const RESET_VALUE: u32 = 0;
}

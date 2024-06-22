#[doc = "Register `GICD_CTLR` reader"]
pub type R = crate::R<GicdCtlrSpec>;
#[doc = "Register `GICD_CTLR` writer"]
pub type W = crate::W<GicdCtlrSpec>;
#[doc = "Field `ENABLE_GROUP0` reader - Enable group 0 interrupts"]
pub type EnableGroup0R = crate::BitReader;
#[doc = "Field `ENABLE_GROUP0` writer - Enable group 0 interrupts"]
pub type EnableGroup0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_GROUP1` reader - Enable group 1 interrupts"]
pub type EnableGroup1R = crate::BitReader;
#[doc = "Field `ENABLE_GROUP1` writer - Enable group 1 interrupts"]
pub type EnableGroup1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable group 0 interrupts"]
    #[inline(always)]
    pub fn enable_group0(&self) -> EnableGroup0R {
        EnableGroup0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable group 1 interrupts"]
    #[inline(always)]
    pub fn enable_group1(&self) -> EnableGroup1R {
        EnableGroup1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_CTLR")
            .field("enable_group0", &self.enable_group0())
            .field("enable_group1", &self.enable_group1())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enable group 0 interrupts"]
    #[inline(always)]
    #[must_use]
    pub fn enable_group0(&mut self) -> EnableGroup0W<GicdCtlrSpec> {
        EnableGroup0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable group 1 interrupts"]
    #[inline(always)]
    #[must_use]
    pub fn enable_group1(&mut self) -> EnableGroup1W<GicdCtlrSpec> {
        EnableGroup1W::new(self, 1)
    }
}
#[doc = "Distributor Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ctlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ctlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdCtlrSpec;
impl crate::RegisterSpec for GicdCtlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ctlr::R`](R) reader structure"]
impl crate::Readable for GicdCtlrSpec {}
#[doc = "`write(|w| ..)` method takes [`gicd_ctlr::W`](W) writer structure"]
impl crate::Writable for GicdCtlrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_CTLR to value 0"]
impl crate::Resettable for GicdCtlrSpec {
    const RESET_VALUE: u32 = 0;
}

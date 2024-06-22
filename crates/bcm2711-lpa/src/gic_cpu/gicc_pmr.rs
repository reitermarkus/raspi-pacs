#[doc = "Register `GICC_PMR` reader"]
pub type R = crate::R<GiccPmrSpec>;
#[doc = "Register `GICC_PMR` writer"]
pub type W = crate::W<GiccPmrSpec>;
#[doc = "Field `PRIORITY` reader - Interrupts with a higher number are not signaled"]
pub type PriorityR = crate::FieldReader;
#[doc = "Field `PRIORITY` writer - Interrupts with a higher number are not signaled"]
pub type PriorityW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupts with a higher number are not signaled"]
    #[inline(always)]
    pub fn priority(&self) -> PriorityR {
        PriorityR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICC_PMR")
            .field("priority", &self.priority())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupts with a higher number are not signaled"]
    #[inline(always)]
    #[must_use]
    pub fn priority(&mut self) -> PriorityW<GiccPmrSpec> {
        PriorityW::new(self, 0)
    }
}
#[doc = "Interrupt Priority Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`gicc_pmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicc_pmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiccPmrSpec;
impl crate::RegisterSpec for GiccPmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicc_pmr::R`](R) reader structure"]
impl crate::Readable for GiccPmrSpec {}
#[doc = "`write(|w| ..)` method takes [`gicc_pmr::W`](W) writer structure"]
impl crate::Writable for GiccPmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICC_PMR to value 0"]
impl crate::Resettable for GiccPmrSpec {
    const RESET_VALUE: u32 = 0;
}

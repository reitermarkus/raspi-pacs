#[doc = "Register `GICC_AEOIR` writer"]
pub type W = crate::W<GiccAeoirSpec>;
#[doc = "Field `INTERRUPT_ID` writer - Interrupt ID"]
pub type InterruptIdW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `CPUID` writer - CPUID that requested a software interrupt, 0 otherwise"]
pub type CpuidW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl core::fmt::Debug for crate::generic::Reg<GiccAeoirSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:9 - Interrupt ID"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_id(&mut self) -> InterruptIdW<GiccAeoirSpec> {
        InterruptIdW::new(self, 0)
    }
    #[doc = "Bits 10:12 - CPUID that requested a software interrupt, 0 otherwise"]
    #[inline(always)]
    #[must_use]
    pub fn cpuid(&mut self) -> CpuidW<GiccAeoirSpec> {
        CpuidW::new(self, 10)
    }
}
#[doc = "Aliased End of Interrupt\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicc_aeoir::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiccAeoirSpec;
impl crate::RegisterSpec for GiccAeoirSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gicc_aeoir::W`](W) writer structure"]
impl crate::Writable for GiccAeoirSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICC_AEOIR to value 0"]
impl crate::Resettable for GiccAeoirSpec {
    const RESET_VALUE: u32 = 0;
}

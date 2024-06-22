#[doc = "Register `GICC_HPPIR` reader"]
pub type R = crate::R<GiccHppirSpec>;
#[doc = "Register `GICC_HPPIR` writer"]
pub type W = crate::W<GiccHppirSpec>;
#[doc = "Field `INTERRUPT_ID` reader - Pending Interrupt ID"]
pub type InterruptIdR = crate::FieldReader<u16>;
#[doc = "Field `INTERRUPT_ID` writer - Pending Interrupt ID"]
pub type InterruptIdW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `CPUID` reader - CPUID that requested a software interrupt, 0 otherwise"]
pub type CpuidR = crate::FieldReader;
#[doc = "Field `CPUID` writer - CPUID that requested a software interrupt, 0 otherwise"]
pub type CpuidW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:9 - Pending Interrupt ID"]
    #[inline(always)]
    pub fn interrupt_id(&self) -> InterruptIdR {
        InterruptIdR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:12 - CPUID that requested a software interrupt, 0 otherwise"]
    #[inline(always)]
    pub fn cpuid(&self) -> CpuidR {
        CpuidR::new(((self.bits >> 10) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICC_HPPIR")
            .field("cpuid", &self.cpuid())
            .field("interrupt_id", &self.interrupt_id())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - Pending Interrupt ID"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_id(&mut self) -> InterruptIdW<GiccHppirSpec> {
        InterruptIdW::new(self, 0)
    }
    #[doc = "Bits 10:12 - CPUID that requested a software interrupt, 0 otherwise"]
    #[inline(always)]
    #[must_use]
    pub fn cpuid(&mut self) -> CpuidW<GiccHppirSpec> {
        CpuidW::new(self, 10)
    }
}
#[doc = "Highest Priority Pending Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`gicc_hppir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicc_hppir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiccHppirSpec;
impl crate::RegisterSpec for GiccHppirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicc_hppir::R`](R) reader structure"]
impl crate::Readable for GiccHppirSpec {}
#[doc = "`write(|w| ..)` method takes [`gicc_hppir::W`](W) writer structure"]
impl crate::Writable for GiccHppirSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICC_HPPIR to value 0"]
impl crate::Resettable for GiccHppirSpec {
    const RESET_VALUE: u32 = 0;
}

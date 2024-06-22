#[doc = "Register `GICC_IAR` reader"]
pub type R = crate::R<GiccIarSpec>;
#[doc = "Field `INTERRUPT_ID` reader - Interrupt ID"]
pub type InterruptIdR = crate::FieldReader<u16>;
#[doc = "Field `CPUID` reader - CPUID that requested a software interrupt, 0 otherwise"]
pub type CpuidR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:9 - Interrupt ID"]
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
        f.debug_struct("GICC_IAR")
            .field("cpuid", &self.cpuid())
            .field("interrupt_id", &self.interrupt_id())
            .finish()
    }
}
#[doc = "Interrupt Acknowledge\n\nYou can [`read`](crate::Reg::read) this register and get [`gicc_iar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiccIarSpec;
impl crate::RegisterSpec for GiccIarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicc_iar::R`](R) reader structure"]
impl crate::Readable for GiccIarSpec {}
#[doc = "`reset()` method sets GICC_IAR to value 0"]
impl crate::Resettable for GiccIarSpec {
    const RESET_VALUE: u32 = 0;
}

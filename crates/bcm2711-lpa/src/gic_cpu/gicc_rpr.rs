#[doc = "Register `GICC_RPR` reader"]
pub type R = crate::R<GiccRprSpec>;
#[doc = "Field `PRIORITY` reader - Current running priority"]
pub type PriorityR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Current running priority"]
    #[inline(always)]
    pub fn priority(&self) -> PriorityR {
        PriorityR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICC_RPR")
            .field("priority", &self.priority())
            .finish()
    }
}
#[doc = "Running Priority\n\nYou can [`read`](crate::Reg::read) this register and get [`gicc_rpr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiccRprSpec;
impl crate::RegisterSpec for GiccRprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicc_rpr::R`](R) reader structure"]
impl crate::Readable for GiccRprSpec {}
#[doc = "`reset()` method sets GICC_RPR to value 0"]
impl crate::Resettable for GiccRprSpec {
    const RESET_VALUE: u32 = 0;
}

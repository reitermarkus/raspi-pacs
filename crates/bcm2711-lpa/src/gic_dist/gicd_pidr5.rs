#[doc = "Register `GICD_PIDR5` reader"]
pub type R = crate::R<GicdPidr5Spec>;
#[doc = "Peripheral ID 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GicdPidr5 {
    #[doc = "0: Valid"]
    Valid = 0,
    #[doc = "1: Invalid"]
    Invalid = 1,
}
impl From<GicdPidr5> for u32 {
    #[inline(always)]
    fn from(variant: GicdPidr5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GicdPidr5 {
    type Ux = u32;
}
impl crate::IsEnum for GicdPidr5 {}
#[doc = "Field `GICD_PIDR5` reader - Peripheral ID 5"]
pub type GicdPidr5R = crate::FieldReader<GicdPidr5>;
impl GicdPidr5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GicdPidr5 {
        match self.bits {
            0 => GicdPidr5::Valid,
            _ => GicdPidr5::Invalid,
        }
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == GicdPidr5::Valid
    }
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        matches!(self.variant(), GicdPidr5::Invalid)
    }
}
impl R {
    #[doc = "Bits 0:31 - Peripheral ID 5"]
    #[inline(always)]
    pub fn gicd_pidr5(&self) -> GicdPidr5R {
        GicdPidr5R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_PIDR5")
            .field("gicd_pidr5", &self.gicd_pidr5())
            .finish()
    }
}
#[doc = "Peripheral ID 5\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_pidr5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdPidr5Spec;
impl crate::RegisterSpec for GicdPidr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_pidr5::R`](R) reader structure"]
impl crate::Readable for GicdPidr5Spec {}
#[doc = "`reset()` method sets GICD_PIDR5 to value 0"]
impl crate::Resettable for GicdPidr5Spec {
    const RESET_VALUE: u32 = 0;
}

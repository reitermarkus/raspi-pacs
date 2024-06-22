#[doc = "Register `GICD_PIDR6` reader"]
pub type R = crate::R<GicdPidr6Spec>;
#[doc = "Peripheral ID 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GicdPidr6 {
    #[doc = "0: Valid"]
    Valid = 0,
    #[doc = "1: Invalid"]
    Invalid = 1,
}
impl From<GicdPidr6> for u32 {
    #[inline(always)]
    fn from(variant: GicdPidr6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GicdPidr6 {
    type Ux = u32;
}
impl crate::IsEnum for GicdPidr6 {}
#[doc = "Field `GICD_PIDR6` reader - Peripheral ID 6"]
pub type GicdPidr6R = crate::FieldReader<GicdPidr6>;
impl GicdPidr6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GicdPidr6 {
        match self.bits {
            0 => GicdPidr6::Valid,
            _ => GicdPidr6::Invalid,
        }
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == GicdPidr6::Valid
    }
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        matches!(self.variant(), GicdPidr6::Invalid)
    }
}
impl R {
    #[doc = "Bits 0:31 - Peripheral ID 6"]
    #[inline(always)]
    pub fn gicd_pidr6(&self) -> GicdPidr6R {
        GicdPidr6R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_PIDR6")
            .field("gicd_pidr6", &self.gicd_pidr6())
            .finish()
    }
}
#[doc = "Peripheral ID 6\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_pidr6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdPidr6Spec;
impl crate::RegisterSpec for GicdPidr6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_pidr6::R`](R) reader structure"]
impl crate::Readable for GicdPidr6Spec {}
#[doc = "`reset()` method sets GICD_PIDR6 to value 0"]
impl crate::Resettable for GicdPidr6Spec {
    const RESET_VALUE: u32 = 0;
}

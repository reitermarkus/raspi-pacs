#[doc = "Register `GICD_PIDR7` reader"]
pub type R = crate::R<GicdPidr7Spec>;
#[doc = "Peripheral ID 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GicdPidr7 {
    #[doc = "0: Valid"]
    Valid = 0,
    #[doc = "1: Invalid"]
    Invalid = 1,
}
impl From<GicdPidr7> for u32 {
    #[inline(always)]
    fn from(variant: GicdPidr7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GicdPidr7 {
    type Ux = u32;
}
impl crate::IsEnum for GicdPidr7 {}
#[doc = "Field `GICD_PIDR7` reader - Peripheral ID 7"]
pub type GicdPidr7R = crate::FieldReader<GicdPidr7>;
impl GicdPidr7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GicdPidr7 {
        match self.bits {
            0 => GicdPidr7::Valid,
            _ => GicdPidr7::Invalid,
        }
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == GicdPidr7::Valid
    }
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        matches!(self.variant(), GicdPidr7::Invalid)
    }
}
impl R {
    #[doc = "Bits 0:31 - Peripheral ID 7"]
    #[inline(always)]
    pub fn gicd_pidr7(&self) -> GicdPidr7R {
        GicdPidr7R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_PIDR7")
            .field("gicd_pidr7", &self.gicd_pidr7())
            .finish()
    }
}
#[doc = "Peripheral ID 7\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_pidr7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdPidr7Spec;
impl crate::RegisterSpec for GicdPidr7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_pidr7::R`](R) reader structure"]
impl crate::Readable for GicdPidr7Spec {}
#[doc = "`reset()` method sets GICD_PIDR7 to value 0"]
impl crate::Resettable for GicdPidr7Spec {
    const RESET_VALUE: u32 = 0;
}

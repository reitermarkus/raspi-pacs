#[doc = "Register `GICD_PIDR3` reader"]
pub type R = crate::R<GicdPidr3Spec>;
#[doc = "Peripheral ID 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GicdPidr3 {
    #[doc = "0: Valid"]
    Valid = 0,
    #[doc = "1: Invalid"]
    Invalid = 1,
}
impl From<GicdPidr3> for u32 {
    #[inline(always)]
    fn from(variant: GicdPidr3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GicdPidr3 {
    type Ux = u32;
}
impl crate::IsEnum for GicdPidr3 {}
#[doc = "Field `GICD_PIDR3` reader - Peripheral ID 3"]
pub type GicdPidr3R = crate::FieldReader<GicdPidr3>;
impl GicdPidr3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GicdPidr3 {
        match self.bits {
            0 => GicdPidr3::Valid,
            _ => GicdPidr3::Invalid,
        }
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == GicdPidr3::Valid
    }
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        matches!(self.variant(), GicdPidr3::Invalid)
    }
}
impl R {
    #[doc = "Bits 0:31 - Peripheral ID 3"]
    #[inline(always)]
    pub fn gicd_pidr3(&self) -> GicdPidr3R {
        GicdPidr3R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_PIDR3")
            .field("gicd_pidr3", &self.gicd_pidr3())
            .finish()
    }
}
#[doc = "Peripheral ID 3\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_pidr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdPidr3Spec;
impl crate::RegisterSpec for GicdPidr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_pidr3::R`](R) reader structure"]
impl crate::Readable for GicdPidr3Spec {}
#[doc = "`reset()` method sets GICD_PIDR3 to value 0"]
impl crate::Resettable for GicdPidr3Spec {
    const RESET_VALUE: u32 = 0;
}

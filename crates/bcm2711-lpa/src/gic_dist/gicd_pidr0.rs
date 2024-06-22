#[doc = "Register `GICD_PIDR0` reader"]
pub type R = crate::R<GicdPidr0Spec>;
#[doc = "Peripheral ID 0\n\nValue on reset: 144"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GicdPidr0 {
    #[doc = "144: Valid"]
    Valid = 144,
    #[doc = "0: Invalid"]
    Invalid = 0,
}
impl From<GicdPidr0> for u32 {
    #[inline(always)]
    fn from(variant: GicdPidr0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GicdPidr0 {
    type Ux = u32;
}
impl crate::IsEnum for GicdPidr0 {}
#[doc = "Field `GICD_PIDR0` reader - Peripheral ID 0"]
pub type GicdPidr0R = crate::FieldReader<GicdPidr0>;
impl GicdPidr0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GicdPidr0 {
        match self.bits {
            144 => GicdPidr0::Valid,
            _ => GicdPidr0::Invalid,
        }
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == GicdPidr0::Valid
    }
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        matches!(self.variant(), GicdPidr0::Invalid)
    }
}
impl R {
    #[doc = "Bits 0:31 - Peripheral ID 0"]
    #[inline(always)]
    pub fn gicd_pidr0(&self) -> GicdPidr0R {
        GicdPidr0R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_PIDR0")
            .field("gicd_pidr0", &self.gicd_pidr0())
            .finish()
    }
}
#[doc = "Peripheral ID 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_pidr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdPidr0Spec;
impl crate::RegisterSpec for GicdPidr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_pidr0::R`](R) reader structure"]
impl crate::Readable for GicdPidr0Spec {}
#[doc = "`reset()` method sets GICD_PIDR0 to value 0x90"]
impl crate::Resettable for GicdPidr0Spec {
    const RESET_VALUE: u32 = 0x90;
}

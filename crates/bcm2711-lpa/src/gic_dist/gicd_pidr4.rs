#[doc = "Register `GICD_PIDR4` reader"]
pub type R = crate::R<GicdPidr4Spec>;
#[doc = "Peripheral ID 4\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GicdPidr4 {
    #[doc = "4: Valid"]
    Valid = 4,
    #[doc = "0: Invalid"]
    Invalid = 0,
}
impl From<GicdPidr4> for u32 {
    #[inline(always)]
    fn from(variant: GicdPidr4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GicdPidr4 {
    type Ux = u32;
}
impl crate::IsEnum for GicdPidr4 {}
#[doc = "Field `GICD_PIDR4` reader - Peripheral ID 4"]
pub type GicdPidr4R = crate::FieldReader<GicdPidr4>;
impl GicdPidr4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GicdPidr4 {
        match self.bits {
            4 => GicdPidr4::Valid,
            _ => GicdPidr4::Invalid,
        }
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == GicdPidr4::Valid
    }
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        matches!(self.variant(), GicdPidr4::Invalid)
    }
}
impl R {
    #[doc = "Bits 0:31 - Peripheral ID 4"]
    #[inline(always)]
    pub fn gicd_pidr4(&self) -> GicdPidr4R {
        GicdPidr4R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_PIDR4")
            .field("gicd_pidr4", &self.gicd_pidr4())
            .finish()
    }
}
#[doc = "Peripheral ID 4\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_pidr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdPidr4Spec;
impl crate::RegisterSpec for GicdPidr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_pidr4::R`](R) reader structure"]
impl crate::Readable for GicdPidr4Spec {}
#[doc = "`reset()` method sets GICD_PIDR4 to value 0x04"]
impl crate::Resettable for GicdPidr4Spec {
    const RESET_VALUE: u32 = 0x04;
}

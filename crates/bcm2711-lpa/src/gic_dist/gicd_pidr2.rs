#[doc = "Register `GICD_PIDR2` reader"]
pub type R = crate::R<GicdPidr2Spec>;
#[doc = "Peripheral ID 2\n\nValue on reset: 43"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GicdPidr2 {
    #[doc = "43: Valid"]
    Valid = 43,
    #[doc = "0: Invalid"]
    Invalid = 0,
}
impl From<GicdPidr2> for u32 {
    #[inline(always)]
    fn from(variant: GicdPidr2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GicdPidr2 {
    type Ux = u32;
}
impl crate::IsEnum for GicdPidr2 {}
#[doc = "Field `GICD_PIDR2` reader - Peripheral ID 2"]
pub type GicdPidr2R = crate::FieldReader<GicdPidr2>;
impl GicdPidr2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GicdPidr2 {
        match self.bits {
            43 => GicdPidr2::Valid,
            _ => GicdPidr2::Invalid,
        }
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == GicdPidr2::Valid
    }
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        matches!(self.variant(), GicdPidr2::Invalid)
    }
}
impl R {
    #[doc = "Bits 0:31 - Peripheral ID 2"]
    #[inline(always)]
    pub fn gicd_pidr2(&self) -> GicdPidr2R {
        GicdPidr2R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_PIDR2")
            .field("gicd_pidr2", &self.gicd_pidr2())
            .finish()
    }
}
#[doc = "Peripheral ID 2\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_pidr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdPidr2Spec;
impl crate::RegisterSpec for GicdPidr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_pidr2::R`](R) reader structure"]
impl crate::Readable for GicdPidr2Spec {}
#[doc = "`reset()` method sets GICD_PIDR2 to value 0x2b"]
impl crate::Resettable for GicdPidr2Spec {
    const RESET_VALUE: u32 = 0x2b;
}

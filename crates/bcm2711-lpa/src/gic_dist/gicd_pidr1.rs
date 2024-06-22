#[doc = "Register `GICD_PIDR1` reader"]
pub type R = crate::R<GicdPidr1Spec>;
#[doc = "Peripheral ID 1\n\nValue on reset: 180"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GicdPidr1 {
    #[doc = "180: Valid"]
    Valid = 180,
    #[doc = "0: Invalid"]
    Invalid = 0,
}
impl From<GicdPidr1> for u32 {
    #[inline(always)]
    fn from(variant: GicdPidr1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GicdPidr1 {
    type Ux = u32;
}
impl crate::IsEnum for GicdPidr1 {}
#[doc = "Field `GICD_PIDR1` reader - Peripheral ID 1"]
pub type GicdPidr1R = crate::FieldReader<GicdPidr1>;
impl GicdPidr1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GicdPidr1 {
        match self.bits {
            180 => GicdPidr1::Valid,
            _ => GicdPidr1::Invalid,
        }
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == GicdPidr1::Valid
    }
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        matches!(self.variant(), GicdPidr1::Invalid)
    }
}
impl R {
    #[doc = "Bits 0:31 - Peripheral ID 1"]
    #[inline(always)]
    pub fn gicd_pidr1(&self) -> GicdPidr1R {
        GicdPidr1R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_PIDR1")
            .field("gicd_pidr1", &self.gicd_pidr1())
            .finish()
    }
}
#[doc = "Peripheral ID 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_pidr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdPidr1Spec;
impl crate::RegisterSpec for GicdPidr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_pidr1::R`](R) reader structure"]
impl crate::Readable for GicdPidr1Spec {}
#[doc = "`reset()` method sets GICD_PIDR1 to value 0xb4"]
impl crate::Resettable for GicdPidr1Spec {
    const RESET_VALUE: u32 = 0xb4;
}

#[doc = "Register `GICD_CIDR0` reader"]
pub type R = crate::R<GicdCidr0Spec>;
#[doc = "Component ID 0\n\nValue on reset: 13"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GicdCidr0 {
    #[doc = "13: Valid"]
    Valid = 13,
    #[doc = "0: Invalid ID"]
    Invalid = 0,
}
impl From<GicdCidr0> for u32 {
    #[inline(always)]
    fn from(variant: GicdCidr0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GicdCidr0 {
    type Ux = u32;
}
impl crate::IsEnum for GicdCidr0 {}
#[doc = "Field `GICD_CIDR0` reader - Component ID 0"]
pub type GicdCidr0R = crate::FieldReader<GicdCidr0>;
impl GicdCidr0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GicdCidr0 {
        match self.bits {
            13 => GicdCidr0::Valid,
            _ => GicdCidr0::Invalid,
        }
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == GicdCidr0::Valid
    }
    #[doc = "Invalid ID"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        matches!(self.variant(), GicdCidr0::Invalid)
    }
}
impl R {
    #[doc = "Bits 0:31 - Component ID 0"]
    #[inline(always)]
    pub fn gicd_cidr0(&self) -> GicdCidr0R {
        GicdCidr0R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_CIDR0")
            .field("gicd_cidr0", &self.gicd_cidr0())
            .finish()
    }
}
#[doc = "Component ID 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_cidr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdCidr0Spec;
impl crate::RegisterSpec for GicdCidr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_cidr0::R`](R) reader structure"]
impl crate::Readable for GicdCidr0Spec {}
#[doc = "`reset()` method sets GICD_CIDR0 to value 0x0d"]
impl crate::Resettable for GicdCidr0Spec {
    const RESET_VALUE: u32 = 0x0d;
}

#[doc = "Register `GICD_CIDR2` reader"]
pub type R = crate::R<GicdCidr2Spec>;
#[doc = "Component ID 2\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GicdCidr2 {
    #[doc = "5: Valid"]
    Valid = 5,
    #[doc = "0: Invalid ID"]
    Invalid = 0,
}
impl From<GicdCidr2> for u32 {
    #[inline(always)]
    fn from(variant: GicdCidr2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GicdCidr2 {
    type Ux = u32;
}
impl crate::IsEnum for GicdCidr2 {}
#[doc = "Field `GICD_CIDR2` reader - Component ID 2"]
pub type GicdCidr2R = crate::FieldReader<GicdCidr2>;
impl GicdCidr2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GicdCidr2 {
        match self.bits {
            5 => GicdCidr2::Valid,
            _ => GicdCidr2::Invalid,
        }
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == GicdCidr2::Valid
    }
    #[doc = "Invalid ID"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        matches!(self.variant(), GicdCidr2::Invalid)
    }
}
impl R {
    #[doc = "Bits 0:31 - Component ID 2"]
    #[inline(always)]
    pub fn gicd_cidr2(&self) -> GicdCidr2R {
        GicdCidr2R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_CIDR2")
            .field("gicd_cidr2", &self.gicd_cidr2())
            .finish()
    }
}
#[doc = "Component ID 2\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_cidr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdCidr2Spec;
impl crate::RegisterSpec for GicdCidr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_cidr2::R`](R) reader structure"]
impl crate::Readable for GicdCidr2Spec {}
#[doc = "`reset()` method sets GICD_CIDR2 to value 0x05"]
impl crate::Resettable for GicdCidr2Spec {
    const RESET_VALUE: u32 = 0x05;
}

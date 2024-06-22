#[doc = "Register `GICD_CIDR3` reader"]
pub type R = crate::R<GicdCidr3Spec>;
#[doc = "Component ID 3\n\nValue on reset: 177"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GicdCidr3 {
    #[doc = "177: Valid"]
    Valid = 177,
    #[doc = "0: Invalid ID"]
    Invalid = 0,
}
impl From<GicdCidr3> for u32 {
    #[inline(always)]
    fn from(variant: GicdCidr3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GicdCidr3 {
    type Ux = u32;
}
impl crate::IsEnum for GicdCidr3 {}
#[doc = "Field `GICD_CIDR3` reader - Component ID 3"]
pub type GicdCidr3R = crate::FieldReader<GicdCidr3>;
impl GicdCidr3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GicdCidr3 {
        match self.bits {
            177 => GicdCidr3::Valid,
            _ => GicdCidr3::Invalid,
        }
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == GicdCidr3::Valid
    }
    #[doc = "Invalid ID"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        matches!(self.variant(), GicdCidr3::Invalid)
    }
}
impl R {
    #[doc = "Bits 0:31 - Component ID 3"]
    #[inline(always)]
    pub fn gicd_cidr3(&self) -> GicdCidr3R {
        GicdCidr3R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_CIDR3")
            .field("gicd_cidr3", &self.gicd_cidr3())
            .finish()
    }
}
#[doc = "Component ID 3\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_cidr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdCidr3Spec;
impl crate::RegisterSpec for GicdCidr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_cidr3::R`](R) reader structure"]
impl crate::Readable for GicdCidr3Spec {}
#[doc = "`reset()` method sets GICD_CIDR3 to value 0xb1"]
impl crate::Resettable for GicdCidr3Spec {
    const RESET_VALUE: u32 = 0xb1;
}

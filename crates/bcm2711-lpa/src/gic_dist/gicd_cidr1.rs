#[doc = "Register `GICD_CIDR1` reader"]
pub type R = crate::R<GicdCidr1Spec>;
#[doc = "Component ID 1\n\nValue on reset: 240"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GicdCidr1 {
    #[doc = "240: Valid"]
    Valid = 240,
    #[doc = "0: Invalid ID"]
    Invalid = 0,
}
impl From<GicdCidr1> for u32 {
    #[inline(always)]
    fn from(variant: GicdCidr1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GicdCidr1 {
    type Ux = u32;
}
impl crate::IsEnum for GicdCidr1 {}
#[doc = "Field `GICD_CIDR1` reader - Component ID 1"]
pub type GicdCidr1R = crate::FieldReader<GicdCidr1>;
impl GicdCidr1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GicdCidr1 {
        match self.bits {
            240 => GicdCidr1::Valid,
            _ => GicdCidr1::Invalid,
        }
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == GicdCidr1::Valid
    }
    #[doc = "Invalid ID"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        matches!(self.variant(), GicdCidr1::Invalid)
    }
}
impl R {
    #[doc = "Bits 0:31 - Component ID 1"]
    #[inline(always)]
    pub fn gicd_cidr1(&self) -> GicdCidr1R {
        GicdCidr1R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_CIDR1")
            .field("gicd_cidr1", &self.gicd_cidr1())
            .finish()
    }
}
#[doc = "Component ID 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_cidr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdCidr1Spec;
impl crate::RegisterSpec for GicdCidr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_cidr1::R`](R) reader structure"]
impl crate::Readable for GicdCidr1Spec {}
#[doc = "`reset()` method sets GICD_CIDR1 to value 0xf0"]
impl crate::Resettable for GicdCidr1Spec {
    const RESET_VALUE: u32 = 0xf0;
}

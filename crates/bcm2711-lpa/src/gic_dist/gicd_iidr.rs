#[doc = "Register `GICD_IIDR` reader"]
pub type R = crate::R<GicdIidrSpec>;
#[doc = "Field `IMPLEMENTER` reader - Implementer"]
pub type ImplementerR = crate::FieldReader<u16>;
#[doc = "Field `REVISION` reader - Revision"]
pub type RevisionR = crate::FieldReader;
#[doc = "Field `VARIANT` reader - Variant"]
pub type VariantR = crate::FieldReader;
#[doc = "Field `PRODUCT_ID` reader - Product ID"]
pub type ProductIdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:11 - Implementer"]
    #[inline(always)]
    pub fn implementer(&self) -> ImplementerR {
        ImplementerR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - Revision"]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Variant"]
    #[inline(always)]
    pub fn variant(&self) -> VariantR {
        VariantR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Product ID"]
    #[inline(always)]
    pub fn product_id(&self) -> ProductIdR {
        ProductIdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IIDR")
            .field("product_id", &self.product_id())
            .field("variant", &self.variant())
            .field("revision", &self.revision())
            .field("implementer", &self.implementer())
            .finish()
    }
}
#[doc = "Distributor Implementer Identification Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_iidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIidrSpec;
impl crate::RegisterSpec for GicdIidrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_iidr::R`](R) reader structure"]
impl crate::Readable for GicdIidrSpec {}
#[doc = "`reset()` method sets GICD_IIDR to value 0x0200_143b"]
impl crate::Resettable for GicdIidrSpec {
    const RESET_VALUE: u32 = 0x0200_143b;
}

#[doc = "Register `CID` reader"]
pub type R = crate::R<CidSpec>;
#[doc = "Register `CID` writer"]
pub type W = crate::W<CidSpec>;
#[doc = "Field `PRODUCT_ID` reader - Product ID field"]
pub type ProductIdR = crate::FieldReader<u32>;
#[doc = "Field `PRODUCT_ID` writer - Product ID field"]
pub type ProductIdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Product ID field"]
    #[inline(always)]
    pub fn product_id(&self) -> ProductIdR {
        ProductIdR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CID")
            .field("product_id", &self.product_id())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Product ID field"]
    #[inline(always)]
    #[must_use]
    pub fn product_id(&mut self) -> ProductIdW<CidSpec> {
        ProductIdW::new(self, 0)
    }
}
#[doc = "OTG_HS core ID register\n\nYou can [`read`](crate::Reg::read) this register and get [`cid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CidSpec;
impl crate::RegisterSpec for CidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cid::R`](R) reader structure"]
impl crate::Readable for CidSpec {}
#[doc = "`write(|w| ..)` method takes [`cid::W`](W) writer structure"]
impl crate::Writable for CidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CID to value 0x1200"]
impl crate::Resettable for CidSpec {
    const RESET_VALUE: u32 = 0x1200;
}

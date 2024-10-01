#[doc = "Register `DAINT` reader"]
pub type R = crate::R<DaintSpec>;
#[doc = "Field `IEPINT` reader - IN endpoint interrupt bits"]
pub type IepintR = crate::FieldReader<u16>;
#[doc = "Field `OEPINT` reader - OUT endpoint interrupt bits"]
pub type OepintR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint interrupt bits"]
    #[inline(always)]
    pub fn iepint(&self) -> IepintR {
        IepintR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - OUT endpoint interrupt bits"]
    #[inline(always)]
    pub fn oepint(&self) -> OepintR {
        OepintR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAINT")
            .field("iepint", &self.iepint())
            .field("oepint", &self.oepint())
            .finish()
    }
}
#[doc = "OTG_HS device all endpoints interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`daint::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DaintSpec;
impl crate::RegisterSpec for DaintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`daint::R`](R) reader structure"]
impl crate::Readable for DaintSpec {}
#[doc = "`reset()` method sets DAINT to value 0"]
impl crate::Resettable for DaintSpec {
    const RESET_VALUE: u32 = 0;
}

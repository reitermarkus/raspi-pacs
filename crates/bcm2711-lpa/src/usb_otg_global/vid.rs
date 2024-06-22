#[doc = "Register `VID` reader"]
pub type R = crate::R<VidSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "OTG_HS vendor ID register\n\nYou can [`read`](crate::Reg::read) this register and get [`vid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VidSpec;
impl crate::RegisterSpec for VidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid::R`](R) reader structure"]
impl crate::Readable for VidSpec {}

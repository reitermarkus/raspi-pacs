#[doc = "Register `CHI` reader"]
pub type R = crate::R<ChiSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Higher 32 bits for the free running counter\n\nYou can [`read`](crate::Reg::read) this register and get [`chi::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChiSpec;
impl crate::RegisterSpec for ChiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chi::R`](R) reader structure"]
impl crate::Readable for ChiSpec {}
#[doc = "`reset()` method sets CHI to value 0"]
impl crate::Resettable for ChiSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `READ` reader"]
pub type R = crate::R<ReadSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Read messages from the VideoCore\n\nYou can [`read`](crate::Reg::read) this register and get [`read::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReadSpec;
impl crate::RegisterSpec for ReadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`read::R`](R) reader structure"]
impl crate::Readable for ReadSpec {}

#[doc = "Register `PEEK` reader"]
pub type R = crate::R<PeekSpec>;
#[doc = "Field `DATA` reader - FIFO data access"]
pub type DataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - FIFO data access"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PEEK").field("data", &self.data()).finish()
    }
}
#[doc = "Read the RXFIFO without removing an entry\n\nYou can [`read`](crate::Reg::read) this register and get [`peek::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeekSpec;
impl crate::RegisterSpec for PeekSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peek::R`](R) reader structure"]
impl crate::Readable for PeekSpec {}
#[doc = "`reset()` method sets PEEK to value 0"]
impl crate::Resettable for PeekSpec {
    const RESET_VALUE: u32 = 0;
}

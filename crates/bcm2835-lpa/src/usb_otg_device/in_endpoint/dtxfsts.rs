#[doc = "Register `DTXFSTS` reader"]
pub type R = crate::R<DtxfstsSpec>;
#[doc = "Field `INEPTFSAV` reader - IN endpoint TxFIFO space avail"]
pub type IneptfsavR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint TxFIFO space avail"]
    #[inline(always)]
    pub fn ineptfsav(&self) -> IneptfsavR {
        IneptfsavR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTXFSTS")
            .field("ineptfsav", &self.ineptfsav())
            .finish()
    }
}
#[doc = "Transmit FIFO status\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtxfstsSpec;
impl crate::RegisterSpec for DtxfstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtxfsts::R`](R) reader structure"]
impl crate::Readable for DtxfstsSpec {}
#[doc = "`reset()` method sets DTXFSTS to value 0"]
impl crate::Resettable for DtxfstsSpec {
    const RESET_VALUE: u32 = 0;
}

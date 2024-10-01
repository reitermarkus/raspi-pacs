#[doc = "Register `HPTXFSIZ` reader"]
pub type R = crate::R<HptxfsizSpec>;
#[doc = "Register `HPTXFSIZ` writer"]
pub type W = crate::W<HptxfsizSpec>;
#[doc = "Field `PTXSA` reader - Host periodic TxFIFO start address"]
pub type PtxsaR = crate::FieldReader<u16>;
#[doc = "Field `PTXSA` writer - Host periodic TxFIFO start address"]
pub type PtxsaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PTXFD` reader - Host periodic TxFIFO depth"]
pub type PtxfdR = crate::FieldReader<u16>;
#[doc = "Field `PTXFD` writer - Host periodic TxFIFO depth"]
pub type PtxfdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Host periodic TxFIFO start address"]
    #[inline(always)]
    pub fn ptxsa(&self) -> PtxsaR {
        PtxsaR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Host periodic TxFIFO depth"]
    #[inline(always)]
    pub fn ptxfd(&self) -> PtxfdR {
        PtxfdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPTXFSIZ")
            .field("ptxsa", &self.ptxsa())
            .field("ptxfd", &self.ptxfd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Host periodic TxFIFO start address"]
    #[inline(always)]
    #[must_use]
    pub fn ptxsa(&mut self) -> PtxsaW<HptxfsizSpec> {
        PtxsaW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Host periodic TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn ptxfd(&mut self) -> PtxfdW<HptxfsizSpec> {
        PtxfdW::new(self, 16)
    }
}
#[doc = "OTG_HS Host periodic transmit FIFO size register\n\nYou can [`read`](crate::Reg::read) this register and get [`hptxfsiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hptxfsiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HptxfsizSpec;
impl crate::RegisterSpec for HptxfsizSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hptxfsiz::R`](R) reader structure"]
impl crate::Readable for HptxfsizSpec {}
#[doc = "`write(|w| ..)` method takes [`hptxfsiz::W`](W) writer structure"]
impl crate::Writable for HptxfsizSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HPTXFSIZ to value 0x0200_0600"]
impl crate::Resettable for HptxfsizSpec {
    const RESET_VALUE: u32 = 0x0200_0600;
}

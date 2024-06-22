#[doc = "Register `GNPTXFSIZ_Host` reader"]
pub type R = crate::R<GnptxfsizHostSpec>;
#[doc = "Register `GNPTXFSIZ_Host` writer"]
pub type W = crate::W<GnptxfsizHostSpec>;
#[doc = "Field `NPTXFSA` reader - Nonperiodic transmit RAM start address"]
pub type NptxfsaR = crate::FieldReader<u16>;
#[doc = "Field `NPTXFSA` writer - Nonperiodic transmit RAM start address"]
pub type NptxfsaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `NPTXFD` reader - Nonperiodic TxFIFO depth"]
pub type NptxfdR = crate::FieldReader<u16>;
#[doc = "Field `NPTXFD` writer - Nonperiodic TxFIFO depth"]
pub type NptxfdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Nonperiodic transmit RAM start address"]
    #[inline(always)]
    pub fn nptxfsa(&self) -> NptxfsaR {
        NptxfsaR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Nonperiodic TxFIFO depth"]
    #[inline(always)]
    pub fn nptxfd(&self) -> NptxfdR {
        NptxfdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GNPTXFSIZ_Host")
            .field("nptxfsa", &self.nptxfsa())
            .field("nptxfd", &self.nptxfd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Nonperiodic transmit RAM start address"]
    #[inline(always)]
    #[must_use]
    pub fn nptxfsa(&mut self) -> NptxfsaW<GnptxfsizHostSpec> {
        NptxfsaW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Nonperiodic TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn nptxfd(&mut self) -> NptxfdW<GnptxfsizHostSpec> {
        NptxfdW::new(self, 16)
    }
}
#[doc = "OTG_HS nonperiodic transmit FIFO size register (host mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`gnptxfsiz_host::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gnptxfsiz_host::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GnptxfsizHostSpec;
impl crate::RegisterSpec for GnptxfsizHostSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gnptxfsiz_host::R`](R) reader structure"]
impl crate::Readable for GnptxfsizHostSpec {}
#[doc = "`write(|w| ..)` method takes [`gnptxfsiz_host::W`](W) writer structure"]
impl crate::Writable for GnptxfsizHostSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GNPTXFSIZ_Host to value 0x0200"]
impl crate::Resettable for GnptxfsizHostSpec {
    const RESET_VALUE: u32 = 0x0200;
}

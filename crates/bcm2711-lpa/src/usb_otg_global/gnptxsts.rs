#[doc = "Register `GNPTXSTS` reader"]
pub type R = crate::R<GnptxstsSpec>;
#[doc = "Field `NPTXFSAV` reader - Nonperiodic TxFIFO space available"]
pub type NptxfsavR = crate::FieldReader<u16>;
#[doc = "Field `NPTQXSAV` reader - Nonperiodic transmit request queue space available"]
pub type NptqxsavR = crate::FieldReader;
#[doc = "Field `NPTXQTOP` reader - Top of the nonperiodic transmit request queue"]
pub type NptxqtopR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Nonperiodic TxFIFO space available"]
    #[inline(always)]
    pub fn nptxfsav(&self) -> NptxfsavR {
        NptxfsavR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Nonperiodic transmit request queue space available"]
    #[inline(always)]
    pub fn nptqxsav(&self) -> NptqxsavR {
        NptqxsavR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:30 - Top of the nonperiodic transmit request queue"]
    #[inline(always)]
    pub fn nptxqtop(&self) -> NptxqtopR {
        NptxqtopR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GNPTXSTS")
            .field("nptxfsav", &self.nptxfsav())
            .field("nptqxsav", &self.nptqxsav())
            .field("nptxqtop", &self.nptxqtop())
            .finish()
    }
}
#[doc = "OTG_HS nonperiodic transmit FIFO/queue status register\n\nYou can [`read`](crate::Reg::read) this register and get [`gnptxsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GnptxstsSpec;
impl crate::RegisterSpec for GnptxstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gnptxsts::R`](R) reader structure"]
impl crate::Readable for GnptxstsSpec {}
#[doc = "`reset()` method sets GNPTXSTS to value 0x0008_0200"]
impl crate::Resettable for GnptxstsSpec {
    const RESET_VALUE: u32 = 0x0008_0200;
}

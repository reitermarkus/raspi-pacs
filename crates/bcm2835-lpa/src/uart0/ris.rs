#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Field `RIRMIS` reader - RIRMIS"]
pub type RirmisR = crate::BitReader;
#[doc = "Field `CTSRMIS` reader - CTSRMIS"]
pub type CtsrmisR = crate::BitReader;
#[doc = "Field `DCDRMIS` reader - DCDRMIS"]
pub type DcdrmisR = crate::BitReader;
#[doc = "Field `DSRRMIS` reader - DSRRMIS"]
pub type DsrrmisR = crate::BitReader;
#[doc = "Field `RXRIS` reader - RXRIS"]
pub type RxrisR = crate::BitReader;
#[doc = "Field `TXRIS` reader - TXRIS"]
pub type TxrisR = crate::BitReader;
#[doc = "Field `RTRIS` reader - RTRIS"]
pub type RtrisR = crate::BitReader;
#[doc = "Field `FERIS` reader - FERIS"]
pub type FerisR = crate::BitReader;
#[doc = "Field `PERIS` reader - PERIS"]
pub type PerisR = crate::BitReader;
#[doc = "Field `BERIS` reader - BERIS"]
pub type BerisR = crate::BitReader;
#[doc = "Field `OERIS` reader - OERIS"]
pub type OerisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RIRMIS"]
    #[inline(always)]
    pub fn rirmis(&self) -> RirmisR {
        RirmisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CTSRMIS"]
    #[inline(always)]
    pub fn ctsrmis(&self) -> CtsrmisR {
        CtsrmisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DCDRMIS"]
    #[inline(always)]
    pub fn dcdrmis(&self) -> DcdrmisR {
        DcdrmisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DSRRMIS"]
    #[inline(always)]
    pub fn dsrrmis(&self) -> DsrrmisR {
        DsrrmisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RXRIS"]
    #[inline(always)]
    pub fn rxris(&self) -> RxrisR {
        RxrisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TXRIS"]
    #[inline(always)]
    pub fn txris(&self) -> TxrisR {
        TxrisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RTRIS"]
    #[inline(always)]
    pub fn rtris(&self) -> RtrisR {
        RtrisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FERIS"]
    #[inline(always)]
    pub fn feris(&self) -> FerisR {
        FerisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PERIS"]
    #[inline(always)]
    pub fn peris(&self) -> PerisR {
        PerisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BERIS"]
    #[inline(always)]
    pub fn beris(&self) -> BerisR {
        BerisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - OERIS"]
    #[inline(always)]
    pub fn oeris(&self) -> OerisR {
        OerisR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RIS")
            .field("rirmis", &self.rirmis())
            .field("ctsrmis", &self.ctsrmis())
            .field("dcdrmis", &self.dcdrmis())
            .field("dsrrmis", &self.dsrrmis())
            .field("rxris", &self.rxris())
            .field("txris", &self.txris())
            .field("rtris", &self.rtris())
            .field("feris", &self.feris())
            .field("peris", &self.peris())
            .field("beris", &self.beris())
            .field("oeris", &self.oeris())
            .finish()
    }
}
#[doc = "Raw Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisSpec;
impl crate::RegisterSpec for RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RisSpec {}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RisSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Field `RIMMIS` reader - RIMMIS"]
pub type RimmisR = crate::BitReader;
#[doc = "Field `CTSMMIS` reader - CTSMMIS"]
pub type CtsmmisR = crate::BitReader;
#[doc = "Field `DCDMMIS` reader - DCDMMIS"]
pub type DcdmmisR = crate::BitReader;
#[doc = "Field `DSRMMIS` reader - DSRMMIS"]
pub type DsrmmisR = crate::BitReader;
#[doc = "Field `RXMIS` reader - RXMIS"]
pub type RxmisR = crate::BitReader;
#[doc = "Field `TXMIS` reader - TXMIS"]
pub type TxmisR = crate::BitReader;
#[doc = "Field `RTMIS` reader - RTMIS"]
pub type RtmisR = crate::BitReader;
#[doc = "Field `FEMIS` reader - FEMIS"]
pub type FemisR = crate::BitReader;
#[doc = "Field `PEMIS` reader - PEMIS"]
pub type PemisR = crate::BitReader;
#[doc = "Field `BEMIS` reader - BEMIS"]
pub type BemisR = crate::BitReader;
#[doc = "Field `OEMIS` reader - OEMIS"]
pub type OemisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RIMMIS"]
    #[inline(always)]
    pub fn rimmis(&self) -> RimmisR {
        RimmisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CTSMMIS"]
    #[inline(always)]
    pub fn ctsmmis(&self) -> CtsmmisR {
        CtsmmisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DCDMMIS"]
    #[inline(always)]
    pub fn dcdmmis(&self) -> DcdmmisR {
        DcdmmisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DSRMMIS"]
    #[inline(always)]
    pub fn dsrmmis(&self) -> DsrmmisR {
        DsrmmisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RXMIS"]
    #[inline(always)]
    pub fn rxmis(&self) -> RxmisR {
        RxmisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TXMIS"]
    #[inline(always)]
    pub fn txmis(&self) -> TxmisR {
        TxmisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RTMIS"]
    #[inline(always)]
    pub fn rtmis(&self) -> RtmisR {
        RtmisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FEMIS"]
    #[inline(always)]
    pub fn femis(&self) -> FemisR {
        FemisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PEMIS"]
    #[inline(always)]
    pub fn pemis(&self) -> PemisR {
        PemisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BEMIS"]
    #[inline(always)]
    pub fn bemis(&self) -> BemisR {
        BemisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - OEMIS"]
    #[inline(always)]
    pub fn oemis(&self) -> OemisR {
        OemisR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MIS")
            .field("rimmis", &self.rimmis())
            .field("ctsmmis", &self.ctsmmis())
            .field("dcdmmis", &self.dcdmmis())
            .field("dsrmmis", &self.dsrmmis())
            .field("rxmis", &self.rxmis())
            .field("txmis", &self.txmis())
            .field("rtmis", &self.rtmis())
            .field("femis", &self.femis())
            .field("pemis", &self.pemis())
            .field("bemis", &self.bemis())
            .field("oemis", &self.oemis())
            .finish()
    }
}
#[doc = "Masked Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisSpec;
impl crate::RegisterSpec for MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MisSpec {}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MisSpec {
    const RESET_VALUE: u32 = 0;
}

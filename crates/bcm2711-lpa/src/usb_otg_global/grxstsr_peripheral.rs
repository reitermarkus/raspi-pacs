#[doc = "Register `GRXSTSR_Peripheral` reader"]
pub type R = crate::R<GrxstsrPeripheralSpec>;
#[doc = "Field `EPNUM` reader - Endpoint number"]
pub type EpnumR = crate::FieldReader;
#[doc = "Field `BCNT` reader - Byte count"]
pub type BcntR = crate::FieldReader<u16>;
#[doc = "Field `DPID` reader - Data PID"]
pub type DpidR = crate::FieldReader;
#[doc = "Field `PKTSTS` reader - Packet status"]
pub type PktstsR = crate::FieldReader;
#[doc = "Field `FRMNUM` reader - Frame number"]
pub type FrmnumR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Endpoint number"]
    #[inline(always)]
    pub fn epnum(&self) -> EpnumR {
        EpnumR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:14 - Byte count"]
    #[inline(always)]
    pub fn bcnt(&self) -> BcntR {
        BcntR::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    #[doc = "Bits 15:16 - Data PID"]
    #[inline(always)]
    pub fn dpid(&self) -> DpidR {
        DpidR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:20 - Packet status"]
    #[inline(always)]
    pub fn pktsts(&self) -> PktstsR {
        PktstsR::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 21:24 - Frame number"]
    #[inline(always)]
    pub fn frmnum(&self) -> FrmnumR {
        FrmnumR::new(((self.bits >> 21) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GRXSTSR_Peripheral")
            .field("epnum", &self.epnum())
            .field("bcnt", &self.bcnt())
            .field("dpid", &self.dpid())
            .field("pktsts", &self.pktsts())
            .field("frmnum", &self.frmnum())
            .finish()
    }
}
#[doc = "OTG_HS Receive status debug read register (peripheral mode mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`grxstsr_peripheral::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrxstsrPeripheralSpec;
impl crate::RegisterSpec for GrxstsrPeripheralSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grxstsr_peripheral::R`](R) reader structure"]
impl crate::Readable for GrxstsrPeripheralSpec {}
#[doc = "`reset()` method sets GRXSTSR_Peripheral to value 0"]
impl crate::Resettable for GrxstsrPeripheralSpec {
    const RESET_VALUE: u32 = 0;
}

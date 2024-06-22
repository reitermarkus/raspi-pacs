#[doc = "Register `RSR` reader"]
pub type R = crate::R<RsrSpec>;
#[doc = "Field `FE` reader - FE"]
pub type FeR = crate::BitReader;
#[doc = "Field `PE` reader - PE"]
pub type PeR = crate::BitReader;
#[doc = "Field `BE` reader - BE"]
pub type BeR = crate::BitReader;
#[doc = "Field `OE` reader - OE"]
pub type OeR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - FE"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PE"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BE"]
    #[inline(always)]
    pub fn be(&self) -> BeR {
        BeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OE"]
    #[inline(always)]
    pub fn oe(&self) -> OeR {
        OeR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSR")
            .field("fe", &self.fe())
            .field("pe", &self.pe())
            .field("be", &self.be())
            .field("oe", &self.oe())
            .finish()
    }
}
#[doc = "Receive Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RsrSpec;
impl crate::RegisterSpec for RsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsr::R`](R) reader structure"]
impl crate::Readable for RsrSpec {}
#[doc = "`reset()` method sets RSR to value 0"]
impl crate::Resettable for RsrSpec {
    const RESET_VALUE: u32 = 0;
}

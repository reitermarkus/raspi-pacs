#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `UARTEN` reader - UARTEN"]
pub type UartenR = crate::BitReader;
#[doc = "Field `UARTEN` writer - UARTEN"]
pub type UartenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIREN` reader - SIREN"]
pub type SirenR = crate::BitReader;
#[doc = "Field `SIREN` writer - SIREN"]
pub type SirenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIRLP` reader - SIRLP"]
pub type SirlpR = crate::BitReader;
#[doc = "Field `SIRLP` writer - SIRLP"]
pub type SirlpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXE` reader - TXE"]
pub type TxeR = crate::BitReader;
#[doc = "Field `TXE` writer - TXE"]
pub type TxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXE` reader - RXE"]
pub type RxeR = crate::BitReader;
#[doc = "Field `RXE` writer - RXE"]
pub type RxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTR` reader - DTR"]
pub type DtrR = crate::BitReader;
#[doc = "Field `DTR` writer - DTR"]
pub type DtrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTS` reader - RTS"]
pub type RtsR = crate::BitReader;
#[doc = "Field `RTS` writer - RTS"]
pub type RtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSEN` reader - RTSEN"]
pub type RtsenR = crate::BitReader;
#[doc = "Field `RTSEN` writer - RTSEN"]
pub type RtsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSEN` reader - CTSEN"]
pub type CtsenR = crate::BitReader;
#[doc = "Field `CTSEN` writer - CTSEN"]
pub type CtsenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - UARTEN"]
    #[inline(always)]
    pub fn uarten(&self) -> UartenR {
        UartenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SIREN"]
    #[inline(always)]
    pub fn siren(&self) -> SirenR {
        SirenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SIRLP"]
    #[inline(always)]
    pub fn sirlp(&self) -> SirlpR {
        SirlpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - TXE"]
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RXE"]
    #[inline(always)]
    pub fn rxe(&self) -> RxeR {
        RxeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DTR"]
    #[inline(always)]
    pub fn dtr(&self) -> DtrR {
        DtrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RTS"]
    #[inline(always)]
    pub fn rts(&self) -> RtsR {
        RtsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - RTSEN"]
    #[inline(always)]
    pub fn rtsen(&self) -> RtsenR {
        RtsenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CTSEN"]
    #[inline(always)]
    pub fn ctsen(&self) -> CtsenR {
        CtsenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("uarten", &self.uarten())
            .field("siren", &self.siren())
            .field("sirlp", &self.sirlp())
            .field("txe", &self.txe())
            .field("rxe", &self.rxe())
            .field("dtr", &self.dtr())
            .field("rts", &self.rts())
            .field("rtsen", &self.rtsen())
            .field("ctsen", &self.ctsen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - UARTEN"]
    #[inline(always)]
    #[must_use]
    pub fn uarten(&mut self) -> UartenW<CrSpec> {
        UartenW::new(self, 0)
    }
    #[doc = "Bit 1 - SIREN"]
    #[inline(always)]
    #[must_use]
    pub fn siren(&mut self) -> SirenW<CrSpec> {
        SirenW::new(self, 1)
    }
    #[doc = "Bit 2 - SIRLP"]
    #[inline(always)]
    #[must_use]
    pub fn sirlp(&mut self) -> SirlpW<CrSpec> {
        SirlpW::new(self, 2)
    }
    #[doc = "Bit 8 - TXE"]
    #[inline(always)]
    #[must_use]
    pub fn txe(&mut self) -> TxeW<CrSpec> {
        TxeW::new(self, 8)
    }
    #[doc = "Bit 9 - RXE"]
    #[inline(always)]
    #[must_use]
    pub fn rxe(&mut self) -> RxeW<CrSpec> {
        RxeW::new(self, 9)
    }
    #[doc = "Bit 10 - DTR"]
    #[inline(always)]
    #[must_use]
    pub fn dtr(&mut self) -> DtrW<CrSpec> {
        DtrW::new(self, 10)
    }
    #[doc = "Bit 11 - RTS"]
    #[inline(always)]
    #[must_use]
    pub fn rts(&mut self) -> RtsW<CrSpec> {
        RtsW::new(self, 11)
    }
    #[doc = "Bit 14 - RTSEN"]
    #[inline(always)]
    #[must_use]
    pub fn rtsen(&mut self) -> RtsenW<CrSpec> {
        RtsenW::new(self, 14)
    }
    #[doc = "Bit 15 - CTSEN"]
    #[inline(always)]
    #[must_use]
    pub fn ctsen(&mut self) -> CtsenW<CrSpec> {
        CtsenW::new(self, 15)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0;
}

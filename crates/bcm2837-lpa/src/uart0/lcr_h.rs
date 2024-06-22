#[doc = "Register `LCR_H` reader"]
pub type R = crate::R<LcrHSpec>;
#[doc = "Register `LCR_H` writer"]
pub type W = crate::W<LcrHSpec>;
#[doc = "Field `BRK` reader - BRK"]
pub type BrkR = crate::BitReader;
#[doc = "Field `BRK` writer - BRK"]
pub type BrkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN` reader - PEN"]
pub type PenR = crate::BitReader;
#[doc = "Field `PEN` writer - PEN"]
pub type PenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPS` reader - EPS"]
pub type EpsR = crate::BitReader;
#[doc = "Field `EPS` writer - EPS"]
pub type EpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STP2` reader - STP2"]
pub type Stp2R = crate::BitReader;
#[doc = "Field `STP2` writer - STP2"]
pub type Stp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEN` reader - FEN"]
pub type FenR = crate::BitReader;
#[doc = "Field `FEN` writer - FEN"]
pub type FenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WLEN` reader - WLEN"]
pub type WlenR = crate::FieldReader;
#[doc = "Field `WLEN` writer - WLEN"]
pub type WlenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPS` reader - SPS"]
pub type SpsR = crate::BitReader;
#[doc = "Field `SPS` writer - SPS"]
pub type SpsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - BRK"]
    #[inline(always)]
    pub fn brk(&self) -> BrkR {
        BrkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PEN"]
    #[inline(always)]
    pub fn pen(&self) -> PenR {
        PenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EPS"]
    #[inline(always)]
    pub fn eps(&self) -> EpsR {
        EpsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STP2"]
    #[inline(always)]
    pub fn stp2(&self) -> Stp2R {
        Stp2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FEN"]
    #[inline(always)]
    pub fn fen(&self) -> FenR {
        FenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - WLEN"]
    #[inline(always)]
    pub fn wlen(&self) -> WlenR {
        WlenR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - SPS"]
    #[inline(always)]
    pub fn sps(&self) -> SpsR {
        SpsR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCR_H")
            .field("brk", &self.brk())
            .field("pen", &self.pen())
            .field("eps", &self.eps())
            .field("stp2", &self.stp2())
            .field("fen", &self.fen())
            .field("wlen", &self.wlen())
            .field("sps", &self.sps())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - BRK"]
    #[inline(always)]
    #[must_use]
    pub fn brk(&mut self) -> BrkW<LcrHSpec> {
        BrkW::new(self, 0)
    }
    #[doc = "Bit 1 - PEN"]
    #[inline(always)]
    #[must_use]
    pub fn pen(&mut self) -> PenW<LcrHSpec> {
        PenW::new(self, 1)
    }
    #[doc = "Bit 2 - EPS"]
    #[inline(always)]
    #[must_use]
    pub fn eps(&mut self) -> EpsW<LcrHSpec> {
        EpsW::new(self, 2)
    }
    #[doc = "Bit 3 - STP2"]
    #[inline(always)]
    #[must_use]
    pub fn stp2(&mut self) -> Stp2W<LcrHSpec> {
        Stp2W::new(self, 3)
    }
    #[doc = "Bit 4 - FEN"]
    #[inline(always)]
    #[must_use]
    pub fn fen(&mut self) -> FenW<LcrHSpec> {
        FenW::new(self, 4)
    }
    #[doc = "Bits 5:6 - WLEN"]
    #[inline(always)]
    #[must_use]
    pub fn wlen(&mut self) -> WlenW<LcrHSpec> {
        WlenW::new(self, 5)
    }
    #[doc = "Bit 7 - SPS"]
    #[inline(always)]
    #[must_use]
    pub fn sps(&mut self) -> SpsW<LcrHSpec> {
        SpsW::new(self, 7)
    }
}
#[doc = "Line Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcr_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcr_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcrHSpec;
impl crate::RegisterSpec for LcrHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcr_h::R`](R) reader structure"]
impl crate::Readable for LcrHSpec {}
#[doc = "`write(|w| ..)` method takes [`lcr_h::W`](W) writer structure"]
impl crate::Writable for LcrHSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LCR_H to value 0"]
impl crate::Resettable for LcrHSpec {
    const RESET_VALUE: u32 = 0;
}

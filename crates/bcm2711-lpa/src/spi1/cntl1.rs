#[doc = "Register `CNTL1` reader"]
pub type R = crate::R<Cntl1Spec>;
#[doc = "Register `CNTL1` writer"]
pub type W = crate::W<Cntl1Spec>;
#[doc = "Field `KEEP_INPUT` reader - Don't clear the RX shift register before a new transaction"]
pub type KeepInputR = crate::BitReader;
#[doc = "Field `KEEP_INPUT` writer - Don't clear the RX shift register before a new transaction"]
pub type KeepInputW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSB_FIRST` reader - Shift the most significant bit first (MSB)"]
pub type MsbFirstR = crate::BitReader;
#[doc = "Field `MSB_FIRST` writer - Shift the most significant bit first (MSB)"]
pub type MsbFirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE_ENABLE` reader - Enable DONE interrupt"]
pub type DoneEnableR = crate::BitReader;
#[doc = "Field `DONE_ENABLE` writer - Enable DONE interrupt"]
pub type DoneEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXE_ENABLE` reader - Enable TX empty interrupt"]
pub type TxeEnableR = crate::BitReader;
#[doc = "Field `TXE_ENABLE` writer - Enable TX empty interrupt"]
pub type TxeEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_HIGH_TIME` reader - Additional SPI clock cycles where CS is high"]
pub type CsHighTimeR = crate::FieldReader;
#[doc = "Field `CS_HIGH_TIME` writer - Additional SPI clock cycles where CS is high"]
pub type CsHighTimeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Don't clear the RX shift register before a new transaction"]
    #[inline(always)]
    pub fn keep_input(&self) -> KeepInputR {
        KeepInputR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Shift the most significant bit first (MSB)"]
    #[inline(always)]
    pub fn msb_first(&self) -> MsbFirstR {
        MsbFirstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable DONE interrupt"]
    #[inline(always)]
    pub fn done_enable(&self) -> DoneEnableR {
        DoneEnableR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable TX empty interrupt"]
    #[inline(always)]
    pub fn txe_enable(&self) -> TxeEnableR {
        TxeEnableR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Additional SPI clock cycles where CS is high"]
    #[inline(always)]
    pub fn cs_high_time(&self) -> CsHighTimeR {
        CsHighTimeR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNTL1")
            .field("cs_high_time", &self.cs_high_time())
            .field("txe_enable", &self.txe_enable())
            .field("done_enable", &self.done_enable())
            .field("msb_first", &self.msb_first())
            .field("keep_input", &self.keep_input())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Don't clear the RX shift register before a new transaction"]
    #[inline(always)]
    #[must_use]
    pub fn keep_input(&mut self) -> KeepInputW<Cntl1Spec> {
        KeepInputW::new(self, 0)
    }
    #[doc = "Bit 1 - Shift the most significant bit first (MSB)"]
    #[inline(always)]
    #[must_use]
    pub fn msb_first(&mut self) -> MsbFirstW<Cntl1Spec> {
        MsbFirstW::new(self, 1)
    }
    #[doc = "Bit 6 - Enable DONE interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn done_enable(&mut self) -> DoneEnableW<Cntl1Spec> {
        DoneEnableW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable TX empty interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txe_enable(&mut self) -> TxeEnableW<Cntl1Spec> {
        TxeEnableW::new(self, 7)
    }
    #[doc = "Bits 8:10 - Additional SPI clock cycles where CS is high"]
    #[inline(always)]
    #[must_use]
    pub fn cs_high_time(&mut self) -> CsHighTimeW<Cntl1Spec> {
        CsHighTimeW::new(self, 8)
    }
}
#[doc = "Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cntl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cntl1Spec;
impl crate::RegisterSpec for Cntl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntl1::R`](R) reader structure"]
impl crate::Readable for Cntl1Spec {}
#[doc = "`write(|w| ..)` method takes [`cntl1::W`](W) writer structure"]
impl crate::Writable for Cntl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNTL1 to value 0"]
impl crate::Resettable for Cntl1Spec {
    const RESET_VALUE: u32 = 0;
}

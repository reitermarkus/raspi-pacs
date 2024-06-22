#[doc = "Register `CNTL` reader"]
pub type R = crate::R<CntlSpec>;
#[doc = "Register `CNTL` writer"]
pub type W = crate::W<CntlSpec>;
#[doc = "Field `RX_ENABLE` reader - Enable receive"]
pub type RxEnableR = crate::BitReader;
#[doc = "Field `RX_ENABLE` writer - Enable receive"]
pub type RxEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_ENABLE` reader - Enable transmit"]
pub type TxEnableR = crate::BitReader;
#[doc = "Field `TX_ENABLE` writer - Enable transmit"]
pub type TxEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTS_ENABLE` reader - Enable auto receive flow control with RTS"]
pub type RtsEnableR = crate::BitReader;
#[doc = "Field `RTS_ENABLE` writer - Enable auto receive flow control with RTS"]
pub type RtsEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTS_ENABLE` reader - Enable auto transmit flow control with CTS"]
pub type CtsEnableR = crate::BitReader;
#[doc = "Field `CTS_ENABLE` writer - Enable auto transmit flow control with CTS"]
pub type CtsEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "FIFO level to de-assert RTS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FifoLevel {
    #[doc = "0: 3 empty spaces"]
    _3empty = 0,
    #[doc = "1: 2 empty spaces"]
    _2empty = 1,
    #[doc = "2: 1 empty spaces"]
    _1empty = 2,
    #[doc = "3: 4 empty spaces"]
    _4empty = 3,
}
impl From<FifoLevel> for u8 {
    #[inline(always)]
    fn from(variant: FifoLevel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FifoLevel {
    type Ux = u8;
}
impl crate::IsEnum for FifoLevel {}
#[doc = "Field `RTS_FIFO_LEVEL` reader - FIFO level to de-assert RTS"]
pub type RtsFifoLevelR = crate::FieldReader<FifoLevel>;
impl RtsFifoLevelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FifoLevel {
        match self.bits {
            0 => FifoLevel::_3empty,
            1 => FifoLevel::_2empty,
            2 => FifoLevel::_1empty,
            3 => FifoLevel::_4empty,
            _ => unreachable!(),
        }
    }
    #[doc = "3 empty spaces"]
    #[inline(always)]
    pub fn is_3empty(&self) -> bool {
        *self == FifoLevel::_3empty
    }
    #[doc = "2 empty spaces"]
    #[inline(always)]
    pub fn is_2empty(&self) -> bool {
        *self == FifoLevel::_2empty
    }
    #[doc = "1 empty spaces"]
    #[inline(always)]
    pub fn is_1empty(&self) -> bool {
        *self == FifoLevel::_1empty
    }
    #[doc = "4 empty spaces"]
    #[inline(always)]
    pub fn is_4empty(&self) -> bool {
        *self == FifoLevel::_4empty
    }
}
#[doc = "Field `RTS_FIFO_LEVEL` writer - FIFO level to de-assert RTS"]
pub type RtsFifoLevelW<'a, REG> = crate::FieldWriter<'a, REG, 2, FifoLevel, crate::Safe>;
impl<'a, REG> RtsFifoLevelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "3 empty spaces"]
    #[inline(always)]
    pub fn _3empty(self) -> &'a mut crate::W<REG> {
        self.variant(FifoLevel::_3empty)
    }
    #[doc = "2 empty spaces"]
    #[inline(always)]
    pub fn _2empty(self) -> &'a mut crate::W<REG> {
        self.variant(FifoLevel::_2empty)
    }
    #[doc = "1 empty spaces"]
    #[inline(always)]
    pub fn _1empty(self) -> &'a mut crate::W<REG> {
        self.variant(FifoLevel::_1empty)
    }
    #[doc = "4 empty spaces"]
    #[inline(always)]
    pub fn _4empty(self) -> &'a mut crate::W<REG> {
        self.variant(FifoLevel::_4empty)
    }
}
#[doc = "RTS assert level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AssertLevel {
    #[doc = "0: Assert high"]
    High = 0,
    #[doc = "1: Assert low"]
    Low = 1,
}
impl From<AssertLevel> for bool {
    #[inline(always)]
    fn from(variant: AssertLevel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTS_ASSERT` reader - RTS assert level"]
pub type RtsAssertR = crate::BitReader<AssertLevel>;
impl RtsAssertR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AssertLevel {
        match self.bits {
            false => AssertLevel::High,
            true => AssertLevel::Low,
        }
    }
    #[doc = "Assert high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AssertLevel::High
    }
    #[doc = "Assert low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AssertLevel::Low
    }
}
#[doc = "Field `RTS_ASSERT` writer - RTS assert level"]
pub type RtsAssertW<'a, REG> = crate::BitWriter<'a, REG, AssertLevel>;
impl<'a, REG> RtsAssertW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Assert high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(AssertLevel::High)
    }
    #[doc = "Assert low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(AssertLevel::Low)
    }
}
#[doc = "Field `CTS_ASSERT` reader - CTS assert level"]
pub use RtsAssertR as CtsAssertR;
#[doc = "Field `CTS_ASSERT` writer - CTS assert level"]
pub use RtsAssertW as CtsAssertW;
impl R {
    #[doc = "Bit 0 - Enable receive"]
    #[inline(always)]
    pub fn rx_enable(&self) -> RxEnableR {
        RxEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable transmit"]
    #[inline(always)]
    pub fn tx_enable(&self) -> TxEnableR {
        TxEnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable auto receive flow control with RTS"]
    #[inline(always)]
    pub fn rts_enable(&self) -> RtsEnableR {
        RtsEnableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable auto transmit flow control with CTS"]
    #[inline(always)]
    pub fn cts_enable(&self) -> CtsEnableR {
        CtsEnableR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - FIFO level to de-assert RTS"]
    #[inline(always)]
    pub fn rts_fifo_level(&self) -> RtsFifoLevelR {
        RtsFifoLevelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - RTS assert level"]
    #[inline(always)]
    pub fn rts_assert(&self) -> RtsAssertR {
        RtsAssertR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CTS assert level"]
    #[inline(always)]
    pub fn cts_assert(&self) -> CtsAssertR {
        CtsAssertR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNTL")
            .field("cts_assert", &self.cts_assert())
            .field("rts_assert", &self.rts_assert())
            .field("rts_fifo_level", &self.rts_fifo_level())
            .field("cts_enable", &self.cts_enable())
            .field("rts_enable", &self.rts_enable())
            .field("tx_enable", &self.tx_enable())
            .field("rx_enable", &self.rx_enable())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enable receive"]
    #[inline(always)]
    #[must_use]
    pub fn rx_enable(&mut self) -> RxEnableW<CntlSpec> {
        RxEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable transmit"]
    #[inline(always)]
    #[must_use]
    pub fn tx_enable(&mut self) -> TxEnableW<CntlSpec> {
        TxEnableW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable auto receive flow control with RTS"]
    #[inline(always)]
    #[must_use]
    pub fn rts_enable(&mut self) -> RtsEnableW<CntlSpec> {
        RtsEnableW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable auto transmit flow control with CTS"]
    #[inline(always)]
    #[must_use]
    pub fn cts_enable(&mut self) -> CtsEnableW<CntlSpec> {
        CtsEnableW::new(self, 3)
    }
    #[doc = "Bits 4:5 - FIFO level to de-assert RTS"]
    #[inline(always)]
    #[must_use]
    pub fn rts_fifo_level(&mut self) -> RtsFifoLevelW<CntlSpec> {
        RtsFifoLevelW::new(self, 4)
    }
    #[doc = "Bit 6 - RTS assert level"]
    #[inline(always)]
    #[must_use]
    pub fn rts_assert(&mut self) -> RtsAssertW<CntlSpec> {
        RtsAssertW::new(self, 6)
    }
    #[doc = "Bit 7 - CTS assert level"]
    #[inline(always)]
    #[must_use]
    pub fn cts_assert(&mut self) -> CtsAssertW<CntlSpec> {
        CtsAssertW::new(self, 7)
    }
}
#[doc = "Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CntlSpec;
impl crate::RegisterSpec for CntlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntl::R`](R) reader structure"]
impl crate::Readable for CntlSpec {}
#[doc = "`write(|w| ..)` method takes [`cntl::W`](W) writer structure"]
impl crate::Writable for CntlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNTL to value 0"]
impl crate::Resettable for CntlSpec {
    const RESET_VALUE: u32 = 0;
}

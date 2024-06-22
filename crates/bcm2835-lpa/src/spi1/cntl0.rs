#[doc = "Register `CNTL0` reader"]
pub type R = crate::R<Cntl0Spec>;
#[doc = "Register `CNTL0` writer"]
pub type W = crate::W<Cntl0Spec>;
#[doc = "Field `SHIFT_LENGTH` reader - Number of bits to shift"]
pub type ShiftLengthR = crate::FieldReader;
#[doc = "Field `SHIFT_LENGTH` writer - Number of bits to shift"]
pub type ShiftLengthW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MSB_FIRST` reader - Shift out the most significant bit (MSB) first"]
pub type MsbFirstR = crate::BitReader;
#[doc = "Field `MSB_FIRST` writer - Shift out the most significant bit (MSB) first"]
pub type MsbFirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVERT_CLK` reader - Idle clock high"]
pub type InvertClkR = crate::BitReader;
#[doc = "Field `INVERT_CLK` writer - Idle clock high"]
pub type InvertClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_RISING` reader - Data is clocked out on rising edge of CLK"]
pub type OutRisingR = crate::BitReader;
#[doc = "Field `OUT_RISING` writer - Data is clocked out on rising edge of CLK"]
pub type OutRisingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_FIFOS` reader - Clear FIFOs"]
pub type ClearFifosR = crate::BitReader;
#[doc = "Field `CLEAR_FIFOS` writer - Clear FIFOs"]
pub type ClearFifosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_RISING` reader - Data is clocked in on rising edge of CLK"]
pub type InRisingR = crate::BitReader;
#[doc = "Field `IN_RISING` writer - Data is clocked in on rising edge of CLK"]
pub type InRisingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE` reader - Enable the interface"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable the interface"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Controls extra DOUT hold time in system clock cycles\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DoutHoldTime {
    #[doc = "0: `0`"]
    _0 = 0,
    #[doc = "1: `1`"]
    _1 = 1,
    #[doc = "2: `10`"]
    _4 = 2,
    #[doc = "3: `11`"]
    _7 = 3,
}
impl From<DoutHoldTime> for u8 {
    #[inline(always)]
    fn from(variant: DoutHoldTime) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DoutHoldTime {
    type Ux = u8;
}
impl crate::IsEnum for DoutHoldTime {}
#[doc = "Field `DOUT_HOLD_TIME` reader - Controls extra DOUT hold time in system clock cycles"]
pub type DoutHoldTimeR = crate::FieldReader<DoutHoldTime>;
impl DoutHoldTimeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DoutHoldTime {
        match self.bits {
            0 => DoutHoldTime::_0,
            1 => DoutHoldTime::_1,
            2 => DoutHoldTime::_4,
            3 => DoutHoldTime::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DoutHoldTime::_0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DoutHoldTime::_1
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == DoutHoldTime::_4
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        *self == DoutHoldTime::_7
    }
}
#[doc = "Field `DOUT_HOLD_TIME` writer - Controls extra DOUT hold time in system clock cycles"]
pub type DoutHoldTimeW<'a, REG> = crate::FieldWriter<'a, REG, 2, DoutHoldTime, crate::Safe>;
impl<'a, REG> DoutHoldTimeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DoutHoldTime::_0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DoutHoldTime::_1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(DoutHoldTime::_4)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut crate::W<REG> {
        self.variant(DoutHoldTime::_7)
    }
}
#[doc = "Field `VARIABLE_WIDTH` reader - Take shift length and data from FIFO"]
pub type VariableWidthR = crate::BitReader;
#[doc = "Field `VARIABLE_WIDTH` writer - Take shift length and data from FIFO"]
pub type VariableWidthW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VARIABLE_CS` reader - Take CS pattern and data from TX FIFO (along with VARIABLE_WIDTH)"]
pub type VariableCsR = crate::BitReader;
#[doc = "Field `VARIABLE_CS` writer - Take CS pattern and data from TX FIFO (along with VARIABLE_WIDTH)"]
pub type VariableCsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POST_INPUT` reader - Post input mode"]
pub type PostInputR = crate::BitReader;
#[doc = "Field `POST_INPUT` writer - Post input mode"]
pub type PostInputW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHIP_SELECTS` reader - The CS pattern when active"]
pub type ChipSelectsR = crate::FieldReader;
#[doc = "Field `CHIP_SELECTS` writer - The CS pattern when active"]
pub type ChipSelectsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPEED` reader - SPI clock speed. clk = sys / 2 * (SPEED + 1)"]
pub type SpeedR = crate::FieldReader<u16>;
#[doc = "Field `SPEED` writer - SPI clock speed. clk = sys / 2 * (SPEED + 1)"]
pub type SpeedW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:5 - Number of bits to shift"]
    #[inline(always)]
    pub fn shift_length(&self) -> ShiftLengthR {
        ShiftLengthR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Shift out the most significant bit (MSB) first"]
    #[inline(always)]
    pub fn msb_first(&self) -> MsbFirstR {
        MsbFirstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Idle clock high"]
    #[inline(always)]
    pub fn invert_clk(&self) -> InvertClkR {
        InvertClkR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Data is clocked out on rising edge of CLK"]
    #[inline(always)]
    pub fn out_rising(&self) -> OutRisingR {
        OutRisingR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clear FIFOs"]
    #[inline(always)]
    pub fn clear_fifos(&self) -> ClearFifosR {
        ClearFifosR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data is clocked in on rising edge of CLK"]
    #[inline(always)]
    pub fn in_rising(&self) -> InRisingR {
        InRisingR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable the interface"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Controls extra DOUT hold time in system clock cycles"]
    #[inline(always)]
    pub fn dout_hold_time(&self) -> DoutHoldTimeR {
        DoutHoldTimeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Take shift length and data from FIFO"]
    #[inline(always)]
    pub fn variable_width(&self) -> VariableWidthR {
        VariableWidthR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Take CS pattern and data from TX FIFO (along with VARIABLE_WIDTH)"]
    #[inline(always)]
    pub fn variable_cs(&self) -> VariableCsR {
        VariableCsR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Post input mode"]
    #[inline(always)]
    pub fn post_input(&self) -> PostInputR {
        PostInputR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - The CS pattern when active"]
    #[inline(always)]
    pub fn chip_selects(&self) -> ChipSelectsR {
        ChipSelectsR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:31 - SPI clock speed. clk = sys / 2 * (SPEED + 1)"]
    #[inline(always)]
    pub fn speed(&self) -> SpeedR {
        SpeedR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNTL0")
            .field("speed", &self.speed())
            .field("chip_selects", &self.chip_selects())
            .field("post_input", &self.post_input())
            .field("variable_cs", &self.variable_cs())
            .field("variable_width", &self.variable_width())
            .field("dout_hold_time", &self.dout_hold_time())
            .field("enable", &self.enable())
            .field("in_rising", &self.in_rising())
            .field("clear_fifos", &self.clear_fifos())
            .field("out_rising", &self.out_rising())
            .field("invert_clk", &self.invert_clk())
            .field("msb_first", &self.msb_first())
            .field("shift_length", &self.shift_length())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Number of bits to shift"]
    #[inline(always)]
    #[must_use]
    pub fn shift_length(&mut self) -> ShiftLengthW<Cntl0Spec> {
        ShiftLengthW::new(self, 0)
    }
    #[doc = "Bit 6 - Shift out the most significant bit (MSB) first"]
    #[inline(always)]
    #[must_use]
    pub fn msb_first(&mut self) -> MsbFirstW<Cntl0Spec> {
        MsbFirstW::new(self, 6)
    }
    #[doc = "Bit 7 - Idle clock high"]
    #[inline(always)]
    #[must_use]
    pub fn invert_clk(&mut self) -> InvertClkW<Cntl0Spec> {
        InvertClkW::new(self, 7)
    }
    #[doc = "Bit 8 - Data is clocked out on rising edge of CLK"]
    #[inline(always)]
    #[must_use]
    pub fn out_rising(&mut self) -> OutRisingW<Cntl0Spec> {
        OutRisingW::new(self, 8)
    }
    #[doc = "Bit 9 - Clear FIFOs"]
    #[inline(always)]
    #[must_use]
    pub fn clear_fifos(&mut self) -> ClearFifosW<Cntl0Spec> {
        ClearFifosW::new(self, 9)
    }
    #[doc = "Bit 10 - Data is clocked in on rising edge of CLK"]
    #[inline(always)]
    #[must_use]
    pub fn in_rising(&mut self) -> InRisingW<Cntl0Spec> {
        InRisingW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable the interface"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<Cntl0Spec> {
        EnableW::new(self, 11)
    }
    #[doc = "Bits 12:13 - Controls extra DOUT hold time in system clock cycles"]
    #[inline(always)]
    #[must_use]
    pub fn dout_hold_time(&mut self) -> DoutHoldTimeW<Cntl0Spec> {
        DoutHoldTimeW::new(self, 12)
    }
    #[doc = "Bit 14 - Take shift length and data from FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn variable_width(&mut self) -> VariableWidthW<Cntl0Spec> {
        VariableWidthW::new(self, 14)
    }
    #[doc = "Bit 15 - Take CS pattern and data from TX FIFO (along with VARIABLE_WIDTH)"]
    #[inline(always)]
    #[must_use]
    pub fn variable_cs(&mut self) -> VariableCsW<Cntl0Spec> {
        VariableCsW::new(self, 15)
    }
    #[doc = "Bit 16 - Post input mode"]
    #[inline(always)]
    #[must_use]
    pub fn post_input(&mut self) -> PostInputW<Cntl0Spec> {
        PostInputW::new(self, 16)
    }
    #[doc = "Bits 17:19 - The CS pattern when active"]
    #[inline(always)]
    #[must_use]
    pub fn chip_selects(&mut self) -> ChipSelectsW<Cntl0Spec> {
        ChipSelectsW::new(self, 17)
    }
    #[doc = "Bits 20:31 - SPI clock speed. clk = sys / 2 * (SPEED + 1)"]
    #[inline(always)]
    #[must_use]
    pub fn speed(&mut self) -> SpeedW<Cntl0Spec> {
        SpeedW::new(self, 20)
    }
}
#[doc = "Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cntl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cntl0Spec;
impl crate::RegisterSpec for Cntl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntl0::R`](R) reader structure"]
impl crate::Readable for Cntl0Spec {}
#[doc = "`write(|w| ..)` method takes [`cntl0::W`](W) writer structure"]
impl crate::Writable for Cntl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNTL0 to value 0x000e_0000"]
impl crate::Resettable for Cntl0Spec {
    const RESET_VALUE: u32 = 0x000e_0000;
}

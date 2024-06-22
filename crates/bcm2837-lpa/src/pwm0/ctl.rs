#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `PWEN1` reader - Enable channel 1"]
pub type Pwen1R = crate::BitReader;
#[doc = "Field `PWEN1` writer - Enable channel 1"]
pub type Pwen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Channel 1 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode1 {
    #[doc = "0: `0`"]
    Pwm = 0,
    #[doc = "1: `1`"]
    Serial = 1,
}
impl From<Mode1> for bool {
    #[inline(always)]
    fn from(variant: Mode1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE1` reader - Channel 1 mode"]
pub type Mode1R = crate::BitReader<Mode1>;
impl Mode1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode1 {
        match self.bits {
            false => Mode1::Pwm,
            true => Mode1::Serial,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == Mode1::Pwm
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_serial(&self) -> bool {
        *self == Mode1::Serial
    }
}
#[doc = "Field `MODE1` writer - Channel 1 mode"]
pub type Mode1W<'a, REG> = crate::BitWriter<'a, REG, Mode1>;
impl<'a, REG> Mode1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::Pwm)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn serial(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::Serial)
    }
}
#[doc = "Field `RPTL1` reader - Repeat last value from FIFO for channel 1"]
pub type Rptl1R = crate::BitReader;
#[doc = "Field `RPTL1` writer - Repeat last value from FIFO for channel 1"]
pub type Rptl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBIT1` reader - State when not transmitting on channel 1"]
pub type Sbit1R = crate::BitReader;
#[doc = "Field `SBIT1` writer - State when not transmitting on channel 1"]
pub type Sbit1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLA1` reader - Channel 1 polarity inverted"]
pub type Pola1R = crate::BitReader;
#[doc = "Field `POLA1` writer - Channel 1 polarity inverted"]
pub type Pola1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USEF1` reader - Use FIFO for channel 1"]
pub type Usef1R = crate::BitReader;
#[doc = "Field `USEF1` writer - Use FIFO for channel 1"]
pub type Usef1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRF1` reader - Clear FIFO"]
pub type Clrf1R = crate::BitReader;
#[doc = "Field `CLRF1` writer - Clear FIFO"]
pub type Clrf1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSEN1` reader - M/S mode for channel 1"]
pub type Msen1R = crate::BitReader;
#[doc = "Field `MSEN1` writer - M/S mode for channel 1"]
pub type Msen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWEN2` reader - Enable channel 2"]
pub type Pwen2R = crate::BitReader;
#[doc = "Field `PWEN2` writer - Enable channel 2"]
pub type Pwen2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Channel 2 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode2 {
    #[doc = "0: `0`"]
    Pwm = 0,
    #[doc = "1: `1`"]
    Serial = 1,
}
impl From<Mode2> for bool {
    #[inline(always)]
    fn from(variant: Mode2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE2` reader - Channel 2 mode"]
pub type Mode2R = crate::BitReader<Mode2>;
impl Mode2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode2 {
        match self.bits {
            false => Mode2::Pwm,
            true => Mode2::Serial,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == Mode2::Pwm
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_serial(&self) -> bool {
        *self == Mode2::Serial
    }
}
#[doc = "Field `MODE2` writer - Channel 2 mode"]
pub type Mode2W<'a, REG> = crate::BitWriter<'a, REG, Mode2>;
impl<'a, REG> Mode2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut crate::W<REG> {
        self.variant(Mode2::Pwm)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn serial(self) -> &'a mut crate::W<REG> {
        self.variant(Mode2::Serial)
    }
}
#[doc = "Field `RPTL2` reader - Repeat last value from FIFO for channel 2"]
pub type Rptl2R = crate::BitReader;
#[doc = "Field `RPTL2` writer - Repeat last value from FIFO for channel 2"]
pub type Rptl2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBIT2` reader - State when not transmitting on channel 2"]
pub type Sbit2R = crate::BitReader;
#[doc = "Field `SBIT2` writer - State when not transmitting on channel 2"]
pub type Sbit2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLA2` reader - Channel 2 polarity inverted"]
pub type Pola2R = crate::BitReader;
#[doc = "Field `POLA2` writer - Channel 2 polarity inverted"]
pub type Pola2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USEF2` reader - Use FIFO for channel 2"]
pub type Usef2R = crate::BitReader;
#[doc = "Field `USEF2` writer - Use FIFO for channel 2"]
pub type Usef2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSEN2` reader - M/S mode for channel 2"]
pub type Msen2R = crate::BitReader;
#[doc = "Field `MSEN2` writer - M/S mode for channel 2"]
pub type Msen2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable channel 1"]
    #[inline(always)]
    pub fn pwen1(&self) -> Pwen1R {
        Pwen1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 mode"]
    #[inline(always)]
    pub fn mode1(&self) -> Mode1R {
        Mode1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Repeat last value from FIFO for channel 1"]
    #[inline(always)]
    pub fn rptl1(&self) -> Rptl1R {
        Rptl1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - State when not transmitting on channel 1"]
    #[inline(always)]
    pub fn sbit1(&self) -> Sbit1R {
        Sbit1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 1 polarity inverted"]
    #[inline(always)]
    pub fn pola1(&self) -> Pola1R {
        Pola1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Use FIFO for channel 1"]
    #[inline(always)]
    pub fn usef1(&self) -> Usef1R {
        Usef1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Clear FIFO"]
    #[inline(always)]
    pub fn clrf1(&self) -> Clrf1R {
        Clrf1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - M/S mode for channel 1"]
    #[inline(always)]
    pub fn msen1(&self) -> Msen1R {
        Msen1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable channel 2"]
    #[inline(always)]
    pub fn pwen2(&self) -> Pwen2R {
        Pwen2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 2 mode"]
    #[inline(always)]
    pub fn mode2(&self) -> Mode2R {
        Mode2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Repeat last value from FIFO for channel 2"]
    #[inline(always)]
    pub fn rptl2(&self) -> Rptl2R {
        Rptl2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - State when not transmitting on channel 2"]
    #[inline(always)]
    pub fn sbit2(&self) -> Sbit2R {
        Sbit2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 2 polarity inverted"]
    #[inline(always)]
    pub fn pola2(&self) -> Pola2R {
        Pola2R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Use FIFO for channel 2"]
    #[inline(always)]
    pub fn usef2(&self) -> Usef2R {
        Usef2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - M/S mode for channel 2"]
    #[inline(always)]
    pub fn msen2(&self) -> Msen2R {
        Msen2R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTL")
            .field("msen2", &self.msen2())
            .field("usef2", &self.usef2())
            .field("pola2", &self.pola2())
            .field("sbit2", &self.sbit2())
            .field("rptl2", &self.rptl2())
            .field("mode2", &self.mode2())
            .field("pwen2", &self.pwen2())
            .field("msen1", &self.msen1())
            .field("clrf1", &self.clrf1())
            .field("usef1", &self.usef1())
            .field("pola1", &self.pola1())
            .field("sbit1", &self.sbit1())
            .field("rptl1", &self.rptl1())
            .field("mode1", &self.mode1())
            .field("pwen1", &self.pwen1())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enable channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn pwen1(&mut self) -> Pwen1W<CtlSpec> {
        Pwen1W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode1(&mut self) -> Mode1W<CtlSpec> {
        Mode1W::new(self, 1)
    }
    #[doc = "Bit 2 - Repeat last value from FIFO for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn rptl1(&mut self) -> Rptl1W<CtlSpec> {
        Rptl1W::new(self, 2)
    }
    #[doc = "Bit 3 - State when not transmitting on channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn sbit1(&mut self) -> Sbit1W<CtlSpec> {
        Sbit1W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 1 polarity inverted"]
    #[inline(always)]
    #[must_use]
    pub fn pola1(&mut self) -> Pola1W<CtlSpec> {
        Pola1W::new(self, 4)
    }
    #[doc = "Bit 5 - Use FIFO for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn usef1(&mut self) -> Usef1W<CtlSpec> {
        Usef1W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn clrf1(&mut self) -> Clrf1W<CtlSpec> {
        Clrf1W::new(self, 6)
    }
    #[doc = "Bit 7 - M/S mode for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn msen1(&mut self) -> Msen1W<CtlSpec> {
        Msen1W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn pwen2(&mut self) -> Pwen2W<CtlSpec> {
        Pwen2W::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 2 mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode2(&mut self) -> Mode2W<CtlSpec> {
        Mode2W::new(self, 9)
    }
    #[doc = "Bit 10 - Repeat last value from FIFO for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn rptl2(&mut self) -> Rptl2W<CtlSpec> {
        Rptl2W::new(self, 10)
    }
    #[doc = "Bit 11 - State when not transmitting on channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn sbit2(&mut self) -> Sbit2W<CtlSpec> {
        Sbit2W::new(self, 11)
    }
    #[doc = "Bit 12 - Channel 2 polarity inverted"]
    #[inline(always)]
    #[must_use]
    pub fn pola2(&mut self) -> Pola2W<CtlSpec> {
        Pola2W::new(self, 12)
    }
    #[doc = "Bit 13 - Use FIFO for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn usef2(&mut self) -> Usef2W<CtlSpec> {
        Usef2W::new(self, 13)
    }
    #[doc = "Bit 15 - M/S mode for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn msen2(&mut self) -> Msen2W<CtlSpec> {
        Msen2W::new(self, 15)
    }
}
#[doc = "Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0;
}

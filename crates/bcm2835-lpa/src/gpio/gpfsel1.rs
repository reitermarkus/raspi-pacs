#[doc = "Register `GPFSEL1` reader"]
pub type R = crate::R<Gpfsel1Spec>;
#[doc = "Register `GPFSEL1` writer"]
pub type W = crate::W<Gpfsel1Spec>;
#[doc = "Function Select 10"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel10 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Pin is connected to SPI0_MOSI"]
    Spi0Mosi = 4,
    #[doc = "5: Pin is connected to SD2"]
    Sd2 = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    Reserved3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    Reserved4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    Reserved5 = 2,
}
impl From<Fsel10> for u8 {
    #[inline(always)]
    fn from(variant: Fsel10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel10 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel10 {}
#[doc = "Field `FSEL10` reader - Function Select 10"]
pub type Fsel10R = crate::FieldReader<Fsel10>;
impl Fsel10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel10 {
        match self.bits {
            0 => Fsel10::Input,
            1 => Fsel10::Output,
            4 => Fsel10::Spi0Mosi,
            5 => Fsel10::Sd2,
            6 => Fsel10::Reserved2,
            7 => Fsel10::Reserved3,
            3 => Fsel10::Reserved4,
            2 => Fsel10::Reserved5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel10::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel10::Output
    }
    #[doc = "Pin is connected to SPI0_MOSI"]
    #[inline(always)]
    pub fn is_spi0_mosi(&self) -> bool {
        *self == Fsel10::Spi0Mosi
    }
    #[doc = "Pin is connected to SD2"]
    #[inline(always)]
    pub fn is_sd2(&self) -> bool {
        *self == Fsel10::Sd2
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel10::Reserved2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == Fsel10::Reserved3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fsel10::Reserved4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fsel10::Reserved5
    }
}
#[doc = "Field `FSEL10` writer - Function Select 10"]
pub type Fsel10W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel10, crate::Safe>;
impl<'a, REG> Fsel10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel10::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel10::Output)
    }
    #[doc = "Pin is connected to SPI0_MOSI"]
    #[inline(always)]
    pub fn spi0_mosi(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel10::Spi0Mosi)
    }
    #[doc = "Pin is connected to SD2"]
    #[inline(always)]
    pub fn sd2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel10::Sd2)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel10::Reserved2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel10::Reserved3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel10::Reserved4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel10::Reserved5)
    }
}
#[doc = "Function Select 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel11 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Pin is connected to SPI0_SCLK"]
    Spi0Sclk = 4,
    #[doc = "5: Pin is connected to SD3"]
    Sd3 = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    Reserved3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    Reserved4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    Reserved5 = 2,
}
impl From<Fsel11> for u8 {
    #[inline(always)]
    fn from(variant: Fsel11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel11 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel11 {}
#[doc = "Field `FSEL11` reader - Function Select 11"]
pub type Fsel11R = crate::FieldReader<Fsel11>;
impl Fsel11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel11 {
        match self.bits {
            0 => Fsel11::Input,
            1 => Fsel11::Output,
            4 => Fsel11::Spi0Sclk,
            5 => Fsel11::Sd3,
            6 => Fsel11::Reserved2,
            7 => Fsel11::Reserved3,
            3 => Fsel11::Reserved4,
            2 => Fsel11::Reserved5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel11::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel11::Output
    }
    #[doc = "Pin is connected to SPI0_SCLK"]
    #[inline(always)]
    pub fn is_spi0_sclk(&self) -> bool {
        *self == Fsel11::Spi0Sclk
    }
    #[doc = "Pin is connected to SD3"]
    #[inline(always)]
    pub fn is_sd3(&self) -> bool {
        *self == Fsel11::Sd3
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel11::Reserved2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == Fsel11::Reserved3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fsel11::Reserved4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fsel11::Reserved5
    }
}
#[doc = "Field `FSEL11` writer - Function Select 11"]
pub type Fsel11W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel11, crate::Safe>;
impl<'a, REG> Fsel11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel11::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel11::Output)
    }
    #[doc = "Pin is connected to SPI0_SCLK"]
    #[inline(always)]
    pub fn spi0_sclk(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel11::Spi0Sclk)
    }
    #[doc = "Pin is connected to SD3"]
    #[inline(always)]
    pub fn sd3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel11::Sd3)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel11::Reserved2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel11::Reserved3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel11::Reserved4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel11::Reserved5)
    }
}
#[doc = "Function Select 12"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel12 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Pin is connected to PWM0_0"]
    Pwm0_0 = 4,
    #[doc = "5: Pin is connected to SD4"]
    Sd4 = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    Reserved3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    Reserved4 = 3,
    #[doc = "2: Pin is connected to ARM_TMS"]
    ArmTms = 2,
}
impl From<Fsel12> for u8 {
    #[inline(always)]
    fn from(variant: Fsel12) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel12 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel12 {}
#[doc = "Field `FSEL12` reader - Function Select 12"]
pub type Fsel12R = crate::FieldReader<Fsel12>;
impl Fsel12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel12 {
        match self.bits {
            0 => Fsel12::Input,
            1 => Fsel12::Output,
            4 => Fsel12::Pwm0_0,
            5 => Fsel12::Sd4,
            6 => Fsel12::Reserved2,
            7 => Fsel12::Reserved3,
            3 => Fsel12::Reserved4,
            2 => Fsel12::ArmTms,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel12::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel12::Output
    }
    #[doc = "Pin is connected to PWM0_0"]
    #[inline(always)]
    pub fn is_pwm0_0(&self) -> bool {
        *self == Fsel12::Pwm0_0
    }
    #[doc = "Pin is connected to SD4"]
    #[inline(always)]
    pub fn is_sd4(&self) -> bool {
        *self == Fsel12::Sd4
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel12::Reserved2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == Fsel12::Reserved3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fsel12::Reserved4
    }
    #[doc = "Pin is connected to ARM_TMS"]
    #[inline(always)]
    pub fn is_arm_tms(&self) -> bool {
        *self == Fsel12::ArmTms
    }
}
#[doc = "Field `FSEL12` writer - Function Select 12"]
pub type Fsel12W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel12, crate::Safe>;
impl<'a, REG> Fsel12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel12::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel12::Output)
    }
    #[doc = "Pin is connected to PWM0_0"]
    #[inline(always)]
    pub fn pwm0_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel12::Pwm0_0)
    }
    #[doc = "Pin is connected to SD4"]
    #[inline(always)]
    pub fn sd4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel12::Sd4)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel12::Reserved2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel12::Reserved3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel12::Reserved4)
    }
    #[doc = "Pin is connected to ARM_TMS"]
    #[inline(always)]
    pub fn arm_tms(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel12::ArmTms)
    }
}
#[doc = "Function Select 13"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel13 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Pin is connected to PWM0_1"]
    Pwm0_1 = 4,
    #[doc = "5: Pin is connected to SD5"]
    Sd5 = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    Reserved3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    Reserved4 = 3,
    #[doc = "2: Pin is connected to ARM_TCK"]
    ArmTck = 2,
}
impl From<Fsel13> for u8 {
    #[inline(always)]
    fn from(variant: Fsel13) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel13 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel13 {}
#[doc = "Field `FSEL13` reader - Function Select 13"]
pub type Fsel13R = crate::FieldReader<Fsel13>;
impl Fsel13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel13 {
        match self.bits {
            0 => Fsel13::Input,
            1 => Fsel13::Output,
            4 => Fsel13::Pwm0_1,
            5 => Fsel13::Sd5,
            6 => Fsel13::Reserved2,
            7 => Fsel13::Reserved3,
            3 => Fsel13::Reserved4,
            2 => Fsel13::ArmTck,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel13::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel13::Output
    }
    #[doc = "Pin is connected to PWM0_1"]
    #[inline(always)]
    pub fn is_pwm0_1(&self) -> bool {
        *self == Fsel13::Pwm0_1
    }
    #[doc = "Pin is connected to SD5"]
    #[inline(always)]
    pub fn is_sd5(&self) -> bool {
        *self == Fsel13::Sd5
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel13::Reserved2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == Fsel13::Reserved3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fsel13::Reserved4
    }
    #[doc = "Pin is connected to ARM_TCK"]
    #[inline(always)]
    pub fn is_arm_tck(&self) -> bool {
        *self == Fsel13::ArmTck
    }
}
#[doc = "Field `FSEL13` writer - Function Select 13"]
pub type Fsel13W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel13, crate::Safe>;
impl<'a, REG> Fsel13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel13::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel13::Output)
    }
    #[doc = "Pin is connected to PWM0_1"]
    #[inline(always)]
    pub fn pwm0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel13::Pwm0_1)
    }
    #[doc = "Pin is connected to SD5"]
    #[inline(always)]
    pub fn sd5(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel13::Sd5)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel13::Reserved2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel13::Reserved3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel13::Reserved4)
    }
    #[doc = "Pin is connected to ARM_TCK"]
    #[inline(always)]
    pub fn arm_tck(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel13::ArmTck)
    }
}
#[doc = "Function Select 14"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel14 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Pin is connected to TXD0"]
    Txd0 = 4,
    #[doc = "5: Pin is connected to SD6"]
    Sd6 = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    Reserved3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    Reserved4 = 3,
    #[doc = "2: Pin is connected to TXD1"]
    Txd1 = 2,
}
impl From<Fsel14> for u8 {
    #[inline(always)]
    fn from(variant: Fsel14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel14 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel14 {}
#[doc = "Field `FSEL14` reader - Function Select 14"]
pub type Fsel14R = crate::FieldReader<Fsel14>;
impl Fsel14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel14 {
        match self.bits {
            0 => Fsel14::Input,
            1 => Fsel14::Output,
            4 => Fsel14::Txd0,
            5 => Fsel14::Sd6,
            6 => Fsel14::Reserved2,
            7 => Fsel14::Reserved3,
            3 => Fsel14::Reserved4,
            2 => Fsel14::Txd1,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel14::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel14::Output
    }
    #[doc = "Pin is connected to TXD0"]
    #[inline(always)]
    pub fn is_txd0(&self) -> bool {
        *self == Fsel14::Txd0
    }
    #[doc = "Pin is connected to SD6"]
    #[inline(always)]
    pub fn is_sd6(&self) -> bool {
        *self == Fsel14::Sd6
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel14::Reserved2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == Fsel14::Reserved3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fsel14::Reserved4
    }
    #[doc = "Pin is connected to TXD1"]
    #[inline(always)]
    pub fn is_txd1(&self) -> bool {
        *self == Fsel14::Txd1
    }
}
#[doc = "Field `FSEL14` writer - Function Select 14"]
pub type Fsel14W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel14, crate::Safe>;
impl<'a, REG> Fsel14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel14::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel14::Output)
    }
    #[doc = "Pin is connected to TXD0"]
    #[inline(always)]
    pub fn txd0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel14::Txd0)
    }
    #[doc = "Pin is connected to SD6"]
    #[inline(always)]
    pub fn sd6(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel14::Sd6)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel14::Reserved2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel14::Reserved3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel14::Reserved4)
    }
    #[doc = "Pin is connected to TXD1"]
    #[inline(always)]
    pub fn txd1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel14::Txd1)
    }
}
#[doc = "Function Select 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel15 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Pin is connected to RXD0"]
    Rxd0 = 4,
    #[doc = "5: Pin is connected to SD7"]
    Sd7 = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    Reserved3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    Reserved4 = 3,
    #[doc = "2: Pin is connected to RXD1"]
    Rxd1 = 2,
}
impl From<Fsel15> for u8 {
    #[inline(always)]
    fn from(variant: Fsel15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel15 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel15 {}
#[doc = "Field `FSEL15` reader - Function Select 15"]
pub type Fsel15R = crate::FieldReader<Fsel15>;
impl Fsel15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel15 {
        match self.bits {
            0 => Fsel15::Input,
            1 => Fsel15::Output,
            4 => Fsel15::Rxd0,
            5 => Fsel15::Sd7,
            6 => Fsel15::Reserved2,
            7 => Fsel15::Reserved3,
            3 => Fsel15::Reserved4,
            2 => Fsel15::Rxd1,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel15::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel15::Output
    }
    #[doc = "Pin is connected to RXD0"]
    #[inline(always)]
    pub fn is_rxd0(&self) -> bool {
        *self == Fsel15::Rxd0
    }
    #[doc = "Pin is connected to SD7"]
    #[inline(always)]
    pub fn is_sd7(&self) -> bool {
        *self == Fsel15::Sd7
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel15::Reserved2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == Fsel15::Reserved3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fsel15::Reserved4
    }
    #[doc = "Pin is connected to RXD1"]
    #[inline(always)]
    pub fn is_rxd1(&self) -> bool {
        *self == Fsel15::Rxd1
    }
}
#[doc = "Field `FSEL15` writer - Function Select 15"]
pub type Fsel15W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel15, crate::Safe>;
impl<'a, REG> Fsel15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel15::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel15::Output)
    }
    #[doc = "Pin is connected to RXD0"]
    #[inline(always)]
    pub fn rxd0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel15::Rxd0)
    }
    #[doc = "Pin is connected to SD7"]
    #[inline(always)]
    pub fn sd7(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel15::Sd7)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel15::Reserved2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel15::Reserved3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel15::Reserved4)
    }
    #[doc = "Pin is connected to RXD1"]
    #[inline(always)]
    pub fn rxd1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel15::Rxd1)
    }
}
#[doc = "Function Select 16"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel16 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Alt function 0 reserved"]
    Reserved0 = 4,
    #[doc = "5: Pin is connected to SD8"]
    Sd8 = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Pin is connected to CTS0"]
    Cts0 = 7,
    #[doc = "3: Pin is connected to SPI1_CE2_N"]
    Spi1Ce2N = 3,
    #[doc = "2: Pin is connected to CTS1"]
    Cts1 = 2,
}
impl From<Fsel16> for u8 {
    #[inline(always)]
    fn from(variant: Fsel16) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel16 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel16 {}
#[doc = "Field `FSEL16` reader - Function Select 16"]
pub type Fsel16R = crate::FieldReader<Fsel16>;
impl Fsel16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel16 {
        match self.bits {
            0 => Fsel16::Input,
            1 => Fsel16::Output,
            4 => Fsel16::Reserved0,
            5 => Fsel16::Sd8,
            6 => Fsel16::Reserved2,
            7 => Fsel16::Cts0,
            3 => Fsel16::Spi1Ce2N,
            2 => Fsel16::Cts1,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel16::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel16::Output
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fsel16::Reserved0
    }
    #[doc = "Pin is connected to SD8"]
    #[inline(always)]
    pub fn is_sd8(&self) -> bool {
        *self == Fsel16::Sd8
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel16::Reserved2
    }
    #[doc = "Pin is connected to CTS0"]
    #[inline(always)]
    pub fn is_cts0(&self) -> bool {
        *self == Fsel16::Cts0
    }
    #[doc = "Pin is connected to SPI1_CE2_N"]
    #[inline(always)]
    pub fn is_spi1_ce2_n(&self) -> bool {
        *self == Fsel16::Spi1Ce2N
    }
    #[doc = "Pin is connected to CTS1"]
    #[inline(always)]
    pub fn is_cts1(&self) -> bool {
        *self == Fsel16::Cts1
    }
}
#[doc = "Field `FSEL16` writer - Function Select 16"]
pub type Fsel16W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel16, crate::Safe>;
impl<'a, REG> Fsel16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel16::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel16::Output)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel16::Reserved0)
    }
    #[doc = "Pin is connected to SD8"]
    #[inline(always)]
    pub fn sd8(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel16::Sd8)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel16::Reserved2)
    }
    #[doc = "Pin is connected to CTS0"]
    #[inline(always)]
    pub fn cts0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel16::Cts0)
    }
    #[doc = "Pin is connected to SPI1_CE2_N"]
    #[inline(always)]
    pub fn spi1_ce2_n(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel16::Spi1Ce2N)
    }
    #[doc = "Pin is connected to CTS1"]
    #[inline(always)]
    pub fn cts1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel16::Cts1)
    }
}
#[doc = "Function Select 17"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel17 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Alt function 0 reserved"]
    Reserved0 = 4,
    #[doc = "5: Pin is connected to SD9"]
    Sd9 = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Pin is connected to RTS0"]
    Rts0 = 7,
    #[doc = "3: Pin is connected to SPI1_CE1_N"]
    Spi1Ce1N = 3,
    #[doc = "2: Pin is connected to RTS1"]
    Rts1 = 2,
}
impl From<Fsel17> for u8 {
    #[inline(always)]
    fn from(variant: Fsel17) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel17 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel17 {}
#[doc = "Field `FSEL17` reader - Function Select 17"]
pub type Fsel17R = crate::FieldReader<Fsel17>;
impl Fsel17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel17 {
        match self.bits {
            0 => Fsel17::Input,
            1 => Fsel17::Output,
            4 => Fsel17::Reserved0,
            5 => Fsel17::Sd9,
            6 => Fsel17::Reserved2,
            7 => Fsel17::Rts0,
            3 => Fsel17::Spi1Ce1N,
            2 => Fsel17::Rts1,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel17::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel17::Output
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fsel17::Reserved0
    }
    #[doc = "Pin is connected to SD9"]
    #[inline(always)]
    pub fn is_sd9(&self) -> bool {
        *self == Fsel17::Sd9
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel17::Reserved2
    }
    #[doc = "Pin is connected to RTS0"]
    #[inline(always)]
    pub fn is_rts0(&self) -> bool {
        *self == Fsel17::Rts0
    }
    #[doc = "Pin is connected to SPI1_CE1_N"]
    #[inline(always)]
    pub fn is_spi1_ce1_n(&self) -> bool {
        *self == Fsel17::Spi1Ce1N
    }
    #[doc = "Pin is connected to RTS1"]
    #[inline(always)]
    pub fn is_rts1(&self) -> bool {
        *self == Fsel17::Rts1
    }
}
#[doc = "Field `FSEL17` writer - Function Select 17"]
pub type Fsel17W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel17, crate::Safe>;
impl<'a, REG> Fsel17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel17::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel17::Output)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel17::Reserved0)
    }
    #[doc = "Pin is connected to SD9"]
    #[inline(always)]
    pub fn sd9(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel17::Sd9)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel17::Reserved2)
    }
    #[doc = "Pin is connected to RTS0"]
    #[inline(always)]
    pub fn rts0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel17::Rts0)
    }
    #[doc = "Pin is connected to SPI1_CE1_N"]
    #[inline(always)]
    pub fn spi1_ce1_n(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel17::Spi1Ce1N)
    }
    #[doc = "Pin is connected to RTS1"]
    #[inline(always)]
    pub fn rts1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel17::Rts1)
    }
}
#[doc = "Function Select 18"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel18 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Pin is connected to PCM_CLK"]
    PcmClk = 4,
    #[doc = "5: Pin is connected to SD10"]
    Sd10 = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    Reserved3 = 7,
    #[doc = "3: Pin is connected to SPI1_CE0_N"]
    Spi1Ce0N = 3,
    #[doc = "2: Pin is connected to PWM0_0"]
    Pwm0_0 = 2,
}
impl From<Fsel18> for u8 {
    #[inline(always)]
    fn from(variant: Fsel18) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel18 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel18 {}
#[doc = "Field `FSEL18` reader - Function Select 18"]
pub type Fsel18R = crate::FieldReader<Fsel18>;
impl Fsel18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel18 {
        match self.bits {
            0 => Fsel18::Input,
            1 => Fsel18::Output,
            4 => Fsel18::PcmClk,
            5 => Fsel18::Sd10,
            6 => Fsel18::Reserved2,
            7 => Fsel18::Reserved3,
            3 => Fsel18::Spi1Ce0N,
            2 => Fsel18::Pwm0_0,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel18::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel18::Output
    }
    #[doc = "Pin is connected to PCM_CLK"]
    #[inline(always)]
    pub fn is_pcm_clk(&self) -> bool {
        *self == Fsel18::PcmClk
    }
    #[doc = "Pin is connected to SD10"]
    #[inline(always)]
    pub fn is_sd10(&self) -> bool {
        *self == Fsel18::Sd10
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel18::Reserved2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == Fsel18::Reserved3
    }
    #[doc = "Pin is connected to SPI1_CE0_N"]
    #[inline(always)]
    pub fn is_spi1_ce0_n(&self) -> bool {
        *self == Fsel18::Spi1Ce0N
    }
    #[doc = "Pin is connected to PWM0_0"]
    #[inline(always)]
    pub fn is_pwm0_0(&self) -> bool {
        *self == Fsel18::Pwm0_0
    }
}
#[doc = "Field `FSEL18` writer - Function Select 18"]
pub type Fsel18W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel18, crate::Safe>;
impl<'a, REG> Fsel18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel18::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel18::Output)
    }
    #[doc = "Pin is connected to PCM_CLK"]
    #[inline(always)]
    pub fn pcm_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel18::PcmClk)
    }
    #[doc = "Pin is connected to SD10"]
    #[inline(always)]
    pub fn sd10(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel18::Sd10)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel18::Reserved2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel18::Reserved3)
    }
    #[doc = "Pin is connected to SPI1_CE0_N"]
    #[inline(always)]
    pub fn spi1_ce0_n(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel18::Spi1Ce0N)
    }
    #[doc = "Pin is connected to PWM0_0"]
    #[inline(always)]
    pub fn pwm0_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel18::Pwm0_0)
    }
}
#[doc = "Function Select 19"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel19 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Pin is connected to PCM_FS"]
    PcmFs = 4,
    #[doc = "5: Pin is connected to SD11"]
    Sd11 = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    Reserved3 = 7,
    #[doc = "3: Pin is connected to SPI1_MISO"]
    Spi1Miso = 3,
    #[doc = "2: Pin is connected to PWM0_1"]
    Pwm0_1 = 2,
}
impl From<Fsel19> for u8 {
    #[inline(always)]
    fn from(variant: Fsel19) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel19 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel19 {}
#[doc = "Field `FSEL19` reader - Function Select 19"]
pub type Fsel19R = crate::FieldReader<Fsel19>;
impl Fsel19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel19 {
        match self.bits {
            0 => Fsel19::Input,
            1 => Fsel19::Output,
            4 => Fsel19::PcmFs,
            5 => Fsel19::Sd11,
            6 => Fsel19::Reserved2,
            7 => Fsel19::Reserved3,
            3 => Fsel19::Spi1Miso,
            2 => Fsel19::Pwm0_1,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel19::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel19::Output
    }
    #[doc = "Pin is connected to PCM_FS"]
    #[inline(always)]
    pub fn is_pcm_fs(&self) -> bool {
        *self == Fsel19::PcmFs
    }
    #[doc = "Pin is connected to SD11"]
    #[inline(always)]
    pub fn is_sd11(&self) -> bool {
        *self == Fsel19::Sd11
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel19::Reserved2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == Fsel19::Reserved3
    }
    #[doc = "Pin is connected to SPI1_MISO"]
    #[inline(always)]
    pub fn is_spi1_miso(&self) -> bool {
        *self == Fsel19::Spi1Miso
    }
    #[doc = "Pin is connected to PWM0_1"]
    #[inline(always)]
    pub fn is_pwm0_1(&self) -> bool {
        *self == Fsel19::Pwm0_1
    }
}
#[doc = "Field `FSEL19` writer - Function Select 19"]
pub type Fsel19W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel19, crate::Safe>;
impl<'a, REG> Fsel19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel19::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel19::Output)
    }
    #[doc = "Pin is connected to PCM_FS"]
    #[inline(always)]
    pub fn pcm_fs(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel19::PcmFs)
    }
    #[doc = "Pin is connected to SD11"]
    #[inline(always)]
    pub fn sd11(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel19::Sd11)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel19::Reserved2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel19::Reserved3)
    }
    #[doc = "Pin is connected to SPI1_MISO"]
    #[inline(always)]
    pub fn spi1_miso(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel19::Spi1Miso)
    }
    #[doc = "Pin is connected to PWM0_1"]
    #[inline(always)]
    pub fn pwm0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel19::Pwm0_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Function Select 10"]
    #[inline(always)]
    pub fn fsel10(&self) -> Fsel10R {
        Fsel10R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Function Select 11"]
    #[inline(always)]
    pub fn fsel11(&self) -> Fsel11R {
        Fsel11R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Function Select 12"]
    #[inline(always)]
    pub fn fsel12(&self) -> Fsel12R {
        Fsel12R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Function Select 13"]
    #[inline(always)]
    pub fn fsel13(&self) -> Fsel13R {
        Fsel13R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Function Select 14"]
    #[inline(always)]
    pub fn fsel14(&self) -> Fsel14R {
        Fsel14R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Function Select 15"]
    #[inline(always)]
    pub fn fsel15(&self) -> Fsel15R {
        Fsel15R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Function Select 16"]
    #[inline(always)]
    pub fn fsel16(&self) -> Fsel16R {
        Fsel16R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Function Select 17"]
    #[inline(always)]
    pub fn fsel17(&self) -> Fsel17R {
        Fsel17R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Function Select 18"]
    #[inline(always)]
    pub fn fsel18(&self) -> Fsel18R {
        Fsel18R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - Function Select 19"]
    #[inline(always)]
    pub fn fsel19(&self) -> Fsel19R {
        Fsel19R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPFSEL1")
            .field("fsel10", &self.fsel10())
            .field("fsel11", &self.fsel11())
            .field("fsel12", &self.fsel12())
            .field("fsel13", &self.fsel13())
            .field("fsel14", &self.fsel14())
            .field("fsel15", &self.fsel15())
            .field("fsel16", &self.fsel16())
            .field("fsel17", &self.fsel17())
            .field("fsel18", &self.fsel18())
            .field("fsel19", &self.fsel19())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Function Select 10"]
    #[inline(always)]
    #[must_use]
    pub fn fsel10(&mut self) -> Fsel10W<Gpfsel1Spec> {
        Fsel10W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Function Select 11"]
    #[inline(always)]
    #[must_use]
    pub fn fsel11(&mut self) -> Fsel11W<Gpfsel1Spec> {
        Fsel11W::new(self, 3)
    }
    #[doc = "Bits 6:8 - Function Select 12"]
    #[inline(always)]
    #[must_use]
    pub fn fsel12(&mut self) -> Fsel12W<Gpfsel1Spec> {
        Fsel12W::new(self, 6)
    }
    #[doc = "Bits 9:11 - Function Select 13"]
    #[inline(always)]
    #[must_use]
    pub fn fsel13(&mut self) -> Fsel13W<Gpfsel1Spec> {
        Fsel13W::new(self, 9)
    }
    #[doc = "Bits 12:14 - Function Select 14"]
    #[inline(always)]
    #[must_use]
    pub fn fsel14(&mut self) -> Fsel14W<Gpfsel1Spec> {
        Fsel14W::new(self, 12)
    }
    #[doc = "Bits 15:17 - Function Select 15"]
    #[inline(always)]
    #[must_use]
    pub fn fsel15(&mut self) -> Fsel15W<Gpfsel1Spec> {
        Fsel15W::new(self, 15)
    }
    #[doc = "Bits 18:20 - Function Select 16"]
    #[inline(always)]
    #[must_use]
    pub fn fsel16(&mut self) -> Fsel16W<Gpfsel1Spec> {
        Fsel16W::new(self, 18)
    }
    #[doc = "Bits 21:23 - Function Select 17"]
    #[inline(always)]
    #[must_use]
    pub fn fsel17(&mut self) -> Fsel17W<Gpfsel1Spec> {
        Fsel17W::new(self, 21)
    }
    #[doc = "Bits 24:26 - Function Select 18"]
    #[inline(always)]
    #[must_use]
    pub fn fsel18(&mut self) -> Fsel18W<Gpfsel1Spec> {
        Fsel18W::new(self, 24)
    }
    #[doc = "Bits 27:29 - Function Select 19"]
    #[inline(always)]
    #[must_use]
    pub fn fsel19(&mut self) -> Fsel19W<Gpfsel1Spec> {
        Fsel19W::new(self, 27)
    }
}
#[doc = "GPIO Function Select 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpfsel1::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpfsel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpfsel1Spec;
impl crate::RegisterSpec for Gpfsel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpfsel1::R`](R) reader structure"]
impl crate::Readable for Gpfsel1Spec {}
#[doc = "`write(|w| ..)` method takes [`gpfsel1::W`](W) writer structure"]
impl crate::Writable for Gpfsel1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}

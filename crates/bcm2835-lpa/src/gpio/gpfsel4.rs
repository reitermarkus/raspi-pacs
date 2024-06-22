#[doc = "Register `GPFSEL4` reader"]
pub type R = crate::R<Gpfsel4Spec>;
#[doc = "Register `GPFSEL4` writer"]
pub type W = crate::W<Gpfsel4Spec>;
#[doc = "Function Select 40"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel40 {
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
    #[doc = "2: Pin is connected to TXD1"]
    Txd1 = 2,
}
impl From<Fsel40> for u8 {
    #[inline(always)]
    fn from(variant: Fsel40) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel40 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel40 {}
#[doc = "Field `FSEL40` reader - Function Select 40"]
pub type Fsel40R = crate::FieldReader<Fsel40>;
impl Fsel40R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel40 {
        match self.bits {
            0 => Fsel40::Input,
            1 => Fsel40::Output,
            4 => Fsel40::Pwm0_0,
            5 => Fsel40::Sd4,
            6 => Fsel40::Reserved2,
            7 => Fsel40::Reserved3,
            3 => Fsel40::Reserved4,
            2 => Fsel40::Txd1,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel40::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel40::Output
    }
    #[doc = "Pin is connected to PWM0_0"]
    #[inline(always)]
    pub fn is_pwm0_0(&self) -> bool {
        *self == Fsel40::Pwm0_0
    }
    #[doc = "Pin is connected to SD4"]
    #[inline(always)]
    pub fn is_sd4(&self) -> bool {
        *self == Fsel40::Sd4
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel40::Reserved2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == Fsel40::Reserved3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fsel40::Reserved4
    }
    #[doc = "Pin is connected to TXD1"]
    #[inline(always)]
    pub fn is_txd1(&self) -> bool {
        *self == Fsel40::Txd1
    }
}
#[doc = "Field `FSEL40` writer - Function Select 40"]
pub type Fsel40W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel40, crate::Safe>;
impl<'a, REG> Fsel40W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel40::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel40::Output)
    }
    #[doc = "Pin is connected to PWM0_0"]
    #[inline(always)]
    pub fn pwm0_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel40::Pwm0_0)
    }
    #[doc = "Pin is connected to SD4"]
    #[inline(always)]
    pub fn sd4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel40::Sd4)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel40::Reserved2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel40::Reserved3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel40::Reserved4)
    }
    #[doc = "Pin is connected to TXD1"]
    #[inline(always)]
    pub fn txd1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel40::Txd1)
    }
}
#[doc = "Function Select 41"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel41 {
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
    #[doc = "2: Pin is connected to RXD1"]
    Rxd1 = 2,
}
impl From<Fsel41> for u8 {
    #[inline(always)]
    fn from(variant: Fsel41) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel41 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel41 {}
#[doc = "Field `FSEL41` reader - Function Select 41"]
pub type Fsel41R = crate::FieldReader<Fsel41>;
impl Fsel41R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel41 {
        match self.bits {
            0 => Fsel41::Input,
            1 => Fsel41::Output,
            4 => Fsel41::Pwm0_1,
            5 => Fsel41::Sd5,
            6 => Fsel41::Reserved2,
            7 => Fsel41::Reserved3,
            3 => Fsel41::Reserved4,
            2 => Fsel41::Rxd1,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel41::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel41::Output
    }
    #[doc = "Pin is connected to PWM0_1"]
    #[inline(always)]
    pub fn is_pwm0_1(&self) -> bool {
        *self == Fsel41::Pwm0_1
    }
    #[doc = "Pin is connected to SD5"]
    #[inline(always)]
    pub fn is_sd5(&self) -> bool {
        *self == Fsel41::Sd5
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel41::Reserved2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == Fsel41::Reserved3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fsel41::Reserved4
    }
    #[doc = "Pin is connected to RXD1"]
    #[inline(always)]
    pub fn is_rxd1(&self) -> bool {
        *self == Fsel41::Rxd1
    }
}
#[doc = "Field `FSEL41` writer - Function Select 41"]
pub type Fsel41W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel41, crate::Safe>;
impl<'a, REG> Fsel41W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel41::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel41::Output)
    }
    #[doc = "Pin is connected to PWM0_1"]
    #[inline(always)]
    pub fn pwm0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel41::Pwm0_1)
    }
    #[doc = "Pin is connected to SD5"]
    #[inline(always)]
    pub fn sd5(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel41::Sd5)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel41::Reserved2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel41::Reserved3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel41::Reserved4)
    }
    #[doc = "Pin is connected to RXD1"]
    #[inline(always)]
    pub fn rxd1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel41::Rxd1)
    }
}
#[doc = "Function Select 42"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel42 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Pin is connected to GPCLK1"]
    Gpclk1 = 4,
    #[doc = "5: Pin is connected to SD6"]
    Sd6 = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    Reserved3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    Reserved4 = 3,
    #[doc = "2: Pin is connected to CTS1"]
    Cts1 = 2,
}
impl From<Fsel42> for u8 {
    #[inline(always)]
    fn from(variant: Fsel42) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel42 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel42 {}
#[doc = "Field `FSEL42` reader - Function Select 42"]
pub type Fsel42R = crate::FieldReader<Fsel42>;
impl Fsel42R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel42 {
        match self.bits {
            0 => Fsel42::Input,
            1 => Fsel42::Output,
            4 => Fsel42::Gpclk1,
            5 => Fsel42::Sd6,
            6 => Fsel42::Reserved2,
            7 => Fsel42::Reserved3,
            3 => Fsel42::Reserved4,
            2 => Fsel42::Cts1,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel42::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel42::Output
    }
    #[doc = "Pin is connected to GPCLK1"]
    #[inline(always)]
    pub fn is_gpclk1(&self) -> bool {
        *self == Fsel42::Gpclk1
    }
    #[doc = "Pin is connected to SD6"]
    #[inline(always)]
    pub fn is_sd6(&self) -> bool {
        *self == Fsel42::Sd6
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel42::Reserved2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == Fsel42::Reserved3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fsel42::Reserved4
    }
    #[doc = "Pin is connected to CTS1"]
    #[inline(always)]
    pub fn is_cts1(&self) -> bool {
        *self == Fsel42::Cts1
    }
}
#[doc = "Field `FSEL42` writer - Function Select 42"]
pub type Fsel42W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel42, crate::Safe>;
impl<'a, REG> Fsel42W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel42::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel42::Output)
    }
    #[doc = "Pin is connected to GPCLK1"]
    #[inline(always)]
    pub fn gpclk1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel42::Gpclk1)
    }
    #[doc = "Pin is connected to SD6"]
    #[inline(always)]
    pub fn sd6(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel42::Sd6)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel42::Reserved2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel42::Reserved3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel42::Reserved4)
    }
    #[doc = "Pin is connected to CTS1"]
    #[inline(always)]
    pub fn cts1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel42::Cts1)
    }
}
#[doc = "Function Select 43"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel43 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Pin is connected to GPCLK2"]
    Gpclk2 = 4,
    #[doc = "5: Pin is connected to SD7"]
    Sd7 = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    Reserved3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    Reserved4 = 3,
    #[doc = "2: Pin is connected to RTS1"]
    Rts1 = 2,
}
impl From<Fsel43> for u8 {
    #[inline(always)]
    fn from(variant: Fsel43) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel43 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel43 {}
#[doc = "Field `FSEL43` reader - Function Select 43"]
pub type Fsel43R = crate::FieldReader<Fsel43>;
impl Fsel43R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel43 {
        match self.bits {
            0 => Fsel43::Input,
            1 => Fsel43::Output,
            4 => Fsel43::Gpclk2,
            5 => Fsel43::Sd7,
            6 => Fsel43::Reserved2,
            7 => Fsel43::Reserved3,
            3 => Fsel43::Reserved4,
            2 => Fsel43::Rts1,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel43::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel43::Output
    }
    #[doc = "Pin is connected to GPCLK2"]
    #[inline(always)]
    pub fn is_gpclk2(&self) -> bool {
        *self == Fsel43::Gpclk2
    }
    #[doc = "Pin is connected to SD7"]
    #[inline(always)]
    pub fn is_sd7(&self) -> bool {
        *self == Fsel43::Sd7
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel43::Reserved2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == Fsel43::Reserved3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fsel43::Reserved4
    }
    #[doc = "Pin is connected to RTS1"]
    #[inline(always)]
    pub fn is_rts1(&self) -> bool {
        *self == Fsel43::Rts1
    }
}
#[doc = "Field `FSEL43` writer - Function Select 43"]
pub type Fsel43W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel43, crate::Safe>;
impl<'a, REG> Fsel43W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel43::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel43::Output)
    }
    #[doc = "Pin is connected to GPCLK2"]
    #[inline(always)]
    pub fn gpclk2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel43::Gpclk2)
    }
    #[doc = "Pin is connected to SD7"]
    #[inline(always)]
    pub fn sd7(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel43::Sd7)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel43::Reserved2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel43::Reserved3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel43::Reserved4)
    }
    #[doc = "Pin is connected to RTS1"]
    #[inline(always)]
    pub fn rts1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel43::Rts1)
    }
}
#[doc = "Function Select 44"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel44 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Pin is connected to GPCLK1"]
    Gpclk1 = 4,
    #[doc = "5: Pin is connected to SDA0"]
    Sda0 = 5,
    #[doc = "6: Pin is connected to SDA1"]
    Sda1 = 6,
    #[doc = "7: Alt function 3 reserved"]
    Reserved3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    Reserved4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    Reserved5 = 2,
}
impl From<Fsel44> for u8 {
    #[inline(always)]
    fn from(variant: Fsel44) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel44 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel44 {}
#[doc = "Field `FSEL44` reader - Function Select 44"]
pub type Fsel44R = crate::FieldReader<Fsel44>;
impl Fsel44R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel44 {
        match self.bits {
            0 => Fsel44::Input,
            1 => Fsel44::Output,
            4 => Fsel44::Gpclk1,
            5 => Fsel44::Sda0,
            6 => Fsel44::Sda1,
            7 => Fsel44::Reserved3,
            3 => Fsel44::Reserved4,
            2 => Fsel44::Reserved5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel44::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel44::Output
    }
    #[doc = "Pin is connected to GPCLK1"]
    #[inline(always)]
    pub fn is_gpclk1(&self) -> bool {
        *self == Fsel44::Gpclk1
    }
    #[doc = "Pin is connected to SDA0"]
    #[inline(always)]
    pub fn is_sda0(&self) -> bool {
        *self == Fsel44::Sda0
    }
    #[doc = "Pin is connected to SDA1"]
    #[inline(always)]
    pub fn is_sda1(&self) -> bool {
        *self == Fsel44::Sda1
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == Fsel44::Reserved3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fsel44::Reserved4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fsel44::Reserved5
    }
}
#[doc = "Field `FSEL44` writer - Function Select 44"]
pub type Fsel44W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel44, crate::Safe>;
impl<'a, REG> Fsel44W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel44::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel44::Output)
    }
    #[doc = "Pin is connected to GPCLK1"]
    #[inline(always)]
    pub fn gpclk1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel44::Gpclk1)
    }
    #[doc = "Pin is connected to SDA0"]
    #[inline(always)]
    pub fn sda0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel44::Sda0)
    }
    #[doc = "Pin is connected to SDA1"]
    #[inline(always)]
    pub fn sda1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel44::Sda1)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel44::Reserved3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel44::Reserved4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel44::Reserved5)
    }
}
#[doc = "Function Select 45"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel45 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Pin is connected to PWM0_1"]
    Pwm0_1 = 4,
    #[doc = "5: Pin is connected to SCL0"]
    Scl0 = 5,
    #[doc = "6: Pin is connected to SCL1"]
    Scl1 = 6,
    #[doc = "7: Alt function 3 reserved"]
    Reserved3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    Reserved4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    Reserved5 = 2,
}
impl From<Fsel45> for u8 {
    #[inline(always)]
    fn from(variant: Fsel45) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel45 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel45 {}
#[doc = "Field `FSEL45` reader - Function Select 45"]
pub type Fsel45R = crate::FieldReader<Fsel45>;
impl Fsel45R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel45 {
        match self.bits {
            0 => Fsel45::Input,
            1 => Fsel45::Output,
            4 => Fsel45::Pwm0_1,
            5 => Fsel45::Scl0,
            6 => Fsel45::Scl1,
            7 => Fsel45::Reserved3,
            3 => Fsel45::Reserved4,
            2 => Fsel45::Reserved5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel45::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel45::Output
    }
    #[doc = "Pin is connected to PWM0_1"]
    #[inline(always)]
    pub fn is_pwm0_1(&self) -> bool {
        *self == Fsel45::Pwm0_1
    }
    #[doc = "Pin is connected to SCL0"]
    #[inline(always)]
    pub fn is_scl0(&self) -> bool {
        *self == Fsel45::Scl0
    }
    #[doc = "Pin is connected to SCL1"]
    #[inline(always)]
    pub fn is_scl1(&self) -> bool {
        *self == Fsel45::Scl1
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == Fsel45::Reserved3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fsel45::Reserved4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fsel45::Reserved5
    }
}
#[doc = "Field `FSEL45` writer - Function Select 45"]
pub type Fsel45W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel45, crate::Safe>;
impl<'a, REG> Fsel45W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel45::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel45::Output)
    }
    #[doc = "Pin is connected to PWM0_1"]
    #[inline(always)]
    pub fn pwm0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel45::Pwm0_1)
    }
    #[doc = "Pin is connected to SCL0"]
    #[inline(always)]
    pub fn scl0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel45::Scl0)
    }
    #[doc = "Pin is connected to SCL1"]
    #[inline(always)]
    pub fn scl1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel45::Scl1)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel45::Reserved3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel45::Reserved4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel45::Reserved5)
    }
}
#[doc = "Function Select 46"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel46 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Alt function 0 reserved"]
    Reserved0 = 4,
    #[doc = "5: Alt function 1 reserved"]
    Reserved1 = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    Reserved3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    Reserved4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    Reserved5 = 2,
}
impl From<Fsel46> for u8 {
    #[inline(always)]
    fn from(variant: Fsel46) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel46 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel46 {}
#[doc = "Field `FSEL46` reader - Function Select 46"]
pub type Fsel46R = crate::FieldReader<Fsel46>;
impl Fsel46R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel46 {
        match self.bits {
            0 => Fsel46::Input,
            1 => Fsel46::Output,
            4 => Fsel46::Reserved0,
            5 => Fsel46::Reserved1,
            6 => Fsel46::Reserved2,
            7 => Fsel46::Reserved3,
            3 => Fsel46::Reserved4,
            2 => Fsel46::Reserved5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel46::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel46::Output
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fsel46::Reserved0
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == Fsel46::Reserved1
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel46::Reserved2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == Fsel46::Reserved3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fsel46::Reserved4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fsel46::Reserved5
    }
}
#[doc = "Field `FSEL46` writer - Function Select 46"]
pub type Fsel46W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel46, crate::Safe>;
impl<'a, REG> Fsel46W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel46::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel46::Output)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel46::Reserved0)
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel46::Reserved1)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel46::Reserved2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel46::Reserved3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel46::Reserved4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel46::Reserved5)
    }
}
#[doc = "Function Select 47"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel47 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Alt function 0 reserved"]
    Reserved0 = 4,
    #[doc = "5: Alt function 1 reserved"]
    Reserved1 = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    Reserved3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    Reserved4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    Reserved5 = 2,
}
impl From<Fsel47> for u8 {
    #[inline(always)]
    fn from(variant: Fsel47) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel47 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel47 {}
#[doc = "Field `FSEL47` reader - Function Select 47"]
pub type Fsel47R = crate::FieldReader<Fsel47>;
impl Fsel47R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel47 {
        match self.bits {
            0 => Fsel47::Input,
            1 => Fsel47::Output,
            4 => Fsel47::Reserved0,
            5 => Fsel47::Reserved1,
            6 => Fsel47::Reserved2,
            7 => Fsel47::Reserved3,
            3 => Fsel47::Reserved4,
            2 => Fsel47::Reserved5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel47::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel47::Output
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fsel47::Reserved0
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == Fsel47::Reserved1
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel47::Reserved2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == Fsel47::Reserved3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fsel47::Reserved4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fsel47::Reserved5
    }
}
#[doc = "Field `FSEL47` writer - Function Select 47"]
pub type Fsel47W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel47, crate::Safe>;
impl<'a, REG> Fsel47W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel47::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel47::Output)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel47::Reserved0)
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel47::Reserved1)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel47::Reserved2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel47::Reserved3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel47::Reserved4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel47::Reserved5)
    }
}
#[doc = "Function Select 48"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel48 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Alt function 0 reserved"]
    Reserved0 = 4,
    #[doc = "5: Alt function 1 reserved"]
    Reserved1 = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Pin is connected to SD1_CLK"]
    Sd1Clk = 7,
    #[doc = "3: Alt function 4 reserved"]
    Reserved4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    Reserved5 = 2,
}
impl From<Fsel48> for u8 {
    #[inline(always)]
    fn from(variant: Fsel48) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel48 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel48 {}
#[doc = "Field `FSEL48` reader - Function Select 48"]
pub type Fsel48R = crate::FieldReader<Fsel48>;
impl Fsel48R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel48 {
        match self.bits {
            0 => Fsel48::Input,
            1 => Fsel48::Output,
            4 => Fsel48::Reserved0,
            5 => Fsel48::Reserved1,
            6 => Fsel48::Reserved2,
            7 => Fsel48::Sd1Clk,
            3 => Fsel48::Reserved4,
            2 => Fsel48::Reserved5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel48::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel48::Output
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fsel48::Reserved0
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == Fsel48::Reserved1
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel48::Reserved2
    }
    #[doc = "Pin is connected to SD1_CLK"]
    #[inline(always)]
    pub fn is_sd1_clk(&self) -> bool {
        *self == Fsel48::Sd1Clk
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fsel48::Reserved4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fsel48::Reserved5
    }
}
#[doc = "Field `FSEL48` writer - Function Select 48"]
pub type Fsel48W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel48, crate::Safe>;
impl<'a, REG> Fsel48W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel48::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel48::Output)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel48::Reserved0)
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel48::Reserved1)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel48::Reserved2)
    }
    #[doc = "Pin is connected to SD1_CLK"]
    #[inline(always)]
    pub fn sd1_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel48::Sd1Clk)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel48::Reserved4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel48::Reserved5)
    }
}
#[doc = "Function Select 49"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel49 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Alt function 0 reserved"]
    Reserved0 = 4,
    #[doc = "5: Alt function 1 reserved"]
    Reserved1 = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Pin is connected to SD1_CMD"]
    Sd1Cmd = 7,
    #[doc = "3: Alt function 4 reserved"]
    Reserved4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    Reserved5 = 2,
}
impl From<Fsel49> for u8 {
    #[inline(always)]
    fn from(variant: Fsel49) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel49 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel49 {}
#[doc = "Field `FSEL49` reader - Function Select 49"]
pub type Fsel49R = crate::FieldReader<Fsel49>;
impl Fsel49R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel49 {
        match self.bits {
            0 => Fsel49::Input,
            1 => Fsel49::Output,
            4 => Fsel49::Reserved0,
            5 => Fsel49::Reserved1,
            6 => Fsel49::Reserved2,
            7 => Fsel49::Sd1Cmd,
            3 => Fsel49::Reserved4,
            2 => Fsel49::Reserved5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel49::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel49::Output
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fsel49::Reserved0
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == Fsel49::Reserved1
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel49::Reserved2
    }
    #[doc = "Pin is connected to SD1_CMD"]
    #[inline(always)]
    pub fn is_sd1_cmd(&self) -> bool {
        *self == Fsel49::Sd1Cmd
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fsel49::Reserved4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fsel49::Reserved5
    }
}
#[doc = "Field `FSEL49` writer - Function Select 49"]
pub type Fsel49W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel49, crate::Safe>;
impl<'a, REG> Fsel49W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel49::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel49::Output)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel49::Reserved0)
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel49::Reserved1)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel49::Reserved2)
    }
    #[doc = "Pin is connected to SD1_CMD"]
    #[inline(always)]
    pub fn sd1_cmd(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel49::Sd1Cmd)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel49::Reserved4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel49::Reserved5)
    }
}
impl R {
    #[doc = "Bits 0:2 - Function Select 40"]
    #[inline(always)]
    pub fn fsel40(&self) -> Fsel40R {
        Fsel40R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Function Select 41"]
    #[inline(always)]
    pub fn fsel41(&self) -> Fsel41R {
        Fsel41R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Function Select 42"]
    #[inline(always)]
    pub fn fsel42(&self) -> Fsel42R {
        Fsel42R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Function Select 43"]
    #[inline(always)]
    pub fn fsel43(&self) -> Fsel43R {
        Fsel43R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Function Select 44"]
    #[inline(always)]
    pub fn fsel44(&self) -> Fsel44R {
        Fsel44R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Function Select 45"]
    #[inline(always)]
    pub fn fsel45(&self) -> Fsel45R {
        Fsel45R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Function Select 46"]
    #[inline(always)]
    pub fn fsel46(&self) -> Fsel46R {
        Fsel46R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Function Select 47"]
    #[inline(always)]
    pub fn fsel47(&self) -> Fsel47R {
        Fsel47R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Function Select 48"]
    #[inline(always)]
    pub fn fsel48(&self) -> Fsel48R {
        Fsel48R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - Function Select 49"]
    #[inline(always)]
    pub fn fsel49(&self) -> Fsel49R {
        Fsel49R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPFSEL4")
            .field("fsel40", &self.fsel40())
            .field("fsel41", &self.fsel41())
            .field("fsel42", &self.fsel42())
            .field("fsel43", &self.fsel43())
            .field("fsel44", &self.fsel44())
            .field("fsel45", &self.fsel45())
            .field("fsel46", &self.fsel46())
            .field("fsel47", &self.fsel47())
            .field("fsel48", &self.fsel48())
            .field("fsel49", &self.fsel49())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Function Select 40"]
    #[inline(always)]
    #[must_use]
    pub fn fsel40(&mut self) -> Fsel40W<Gpfsel4Spec> {
        Fsel40W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Function Select 41"]
    #[inline(always)]
    #[must_use]
    pub fn fsel41(&mut self) -> Fsel41W<Gpfsel4Spec> {
        Fsel41W::new(self, 3)
    }
    #[doc = "Bits 6:8 - Function Select 42"]
    #[inline(always)]
    #[must_use]
    pub fn fsel42(&mut self) -> Fsel42W<Gpfsel4Spec> {
        Fsel42W::new(self, 6)
    }
    #[doc = "Bits 9:11 - Function Select 43"]
    #[inline(always)]
    #[must_use]
    pub fn fsel43(&mut self) -> Fsel43W<Gpfsel4Spec> {
        Fsel43W::new(self, 9)
    }
    #[doc = "Bits 12:14 - Function Select 44"]
    #[inline(always)]
    #[must_use]
    pub fn fsel44(&mut self) -> Fsel44W<Gpfsel4Spec> {
        Fsel44W::new(self, 12)
    }
    #[doc = "Bits 15:17 - Function Select 45"]
    #[inline(always)]
    #[must_use]
    pub fn fsel45(&mut self) -> Fsel45W<Gpfsel4Spec> {
        Fsel45W::new(self, 15)
    }
    #[doc = "Bits 18:20 - Function Select 46"]
    #[inline(always)]
    #[must_use]
    pub fn fsel46(&mut self) -> Fsel46W<Gpfsel4Spec> {
        Fsel46W::new(self, 18)
    }
    #[doc = "Bits 21:23 - Function Select 47"]
    #[inline(always)]
    #[must_use]
    pub fn fsel47(&mut self) -> Fsel47W<Gpfsel4Spec> {
        Fsel47W::new(self, 21)
    }
    #[doc = "Bits 24:26 - Function Select 48"]
    #[inline(always)]
    #[must_use]
    pub fn fsel48(&mut self) -> Fsel48W<Gpfsel4Spec> {
        Fsel48W::new(self, 24)
    }
    #[doc = "Bits 27:29 - Function Select 49"]
    #[inline(always)]
    #[must_use]
    pub fn fsel49(&mut self) -> Fsel49W<Gpfsel4Spec> {
        Fsel49W::new(self, 27)
    }
}
#[doc = "GPIO Function Select 4\n\nYou can [`read`](crate::Reg::read) this register and get [`gpfsel4::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpfsel4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpfsel4Spec;
impl crate::RegisterSpec for Gpfsel4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpfsel4::R`](R) reader structure"]
impl crate::Readable for Gpfsel4Spec {}
#[doc = "`write(|w| ..)` method takes [`gpfsel4::W`](W) writer structure"]
impl crate::Writable for Gpfsel4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}

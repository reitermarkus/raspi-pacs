#[doc = "Register `GPFSEL2` reader"]
pub struct R(crate::R<GPFSEL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPFSEL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPFSEL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPFSEL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPFSEL2` writer"]
pub struct W(crate::W<GPFSEL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPFSEL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GPFSEL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPFSEL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSEL20` reader - Function Select 20"]
pub type FSEL20_R = crate::FieldReader<u8, FSEL20_A>;
#[doc = "Function Select 20"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL20_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Pin is connected to PCM_DIN"]
    PCM_DIN = 4,
    #[doc = "5: Pin is connected to SD12"]
    SD12 = 5,
    #[doc = "6: Alt function 2 reserved"]
    RESERVED2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    RESERVED3 = 7,
    #[doc = "3: Pin is connected to SPI1_MOSI"]
    SPI1_MOSI = 3,
    #[doc = "2: Pin is connected to GPCLK0"]
    GPCLK0 = 2,
}
impl From<FSEL20_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL20_A) -> Self {
        variant as _
    }
}
impl FSEL20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSEL20_A {
        match self.bits {
            0 => FSEL20_A::INPUT,
            1 => FSEL20_A::OUTPUT,
            4 => FSEL20_A::PCM_DIN,
            5 => FSEL20_A::SD12,
            6 => FSEL20_A::RESERVED2,
            7 => FSEL20_A::RESERVED3,
            3 => FSEL20_A::SPI1_MOSI,
            2 => FSEL20_A::GPCLK0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL20_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL20_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `PCM_DIN`"]
    #[inline(always)]
    pub fn is_pcm_din(&self) -> bool {
        *self == FSEL20_A::PCM_DIN
    }
    #[doc = "Checks if the value of the field is `SD12`"]
    #[inline(always)]
    pub fn is_sd12(&self) -> bool {
        *self == FSEL20_A::SD12
    }
    #[doc = "Checks if the value of the field is `RESERVED2`"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL20_A::RESERVED2
    }
    #[doc = "Checks if the value of the field is `RESERVED3`"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == FSEL20_A::RESERVED3
    }
    #[doc = "Checks if the value of the field is `SPI1_MOSI`"]
    #[inline(always)]
    pub fn is_spi1_mosi(&self) -> bool {
        *self == FSEL20_A::SPI1_MOSI
    }
    #[doc = "Checks if the value of the field is `GPCLK0`"]
    #[inline(always)]
    pub fn is_gpclk0(&self) -> bool {
        *self == FSEL20_A::GPCLK0
    }
}
#[doc = "Field `FSEL20` writer - Function Select 20"]
pub type FSEL20_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPFSEL2_SPEC, u8, FSEL20_A, 3, O>;
impl<'a, const O: u8> FSEL20_W<'a, O> {
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FSEL20_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(FSEL20_A::OUTPUT)
    }
    #[doc = "Pin is connected to PCM_DIN"]
    #[inline(always)]
    pub fn pcm_din(self) -> &'a mut W {
        self.variant(FSEL20_A::PCM_DIN)
    }
    #[doc = "Pin is connected to SD12"]
    #[inline(always)]
    pub fn sd12(self) -> &'a mut W {
        self.variant(FSEL20_A::SD12)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut W {
        self.variant(FSEL20_A::RESERVED2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut W {
        self.variant(FSEL20_A::RESERVED3)
    }
    #[doc = "Pin is connected to SPI1_MOSI"]
    #[inline(always)]
    pub fn spi1_mosi(self) -> &'a mut W {
        self.variant(FSEL20_A::SPI1_MOSI)
    }
    #[doc = "Pin is connected to GPCLK0"]
    #[inline(always)]
    pub fn gpclk0(self) -> &'a mut W {
        self.variant(FSEL20_A::GPCLK0)
    }
}
#[doc = "Field `FSEL21` reader - Function Select 21"]
pub type FSEL21_R = crate::FieldReader<u8, FSEL21_A>;
#[doc = "Function Select 21"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL21_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Pin is connected to PCM_DOUT"]
    PCM_DOUT = 4,
    #[doc = "5: Pin is connected to SD13"]
    SD13 = 5,
    #[doc = "6: Alt function 2 reserved"]
    RESERVED2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    RESERVED3 = 7,
    #[doc = "3: Pin is connected to SPI1_SCLK"]
    SPI1_SCLK = 3,
    #[doc = "2: Pin is connected to GPCLK1"]
    GPCLK1 = 2,
}
impl From<FSEL21_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL21_A) -> Self {
        variant as _
    }
}
impl FSEL21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSEL21_A {
        match self.bits {
            0 => FSEL21_A::INPUT,
            1 => FSEL21_A::OUTPUT,
            4 => FSEL21_A::PCM_DOUT,
            5 => FSEL21_A::SD13,
            6 => FSEL21_A::RESERVED2,
            7 => FSEL21_A::RESERVED3,
            3 => FSEL21_A::SPI1_SCLK,
            2 => FSEL21_A::GPCLK1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL21_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL21_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `PCM_DOUT`"]
    #[inline(always)]
    pub fn is_pcm_dout(&self) -> bool {
        *self == FSEL21_A::PCM_DOUT
    }
    #[doc = "Checks if the value of the field is `SD13`"]
    #[inline(always)]
    pub fn is_sd13(&self) -> bool {
        *self == FSEL21_A::SD13
    }
    #[doc = "Checks if the value of the field is `RESERVED2`"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL21_A::RESERVED2
    }
    #[doc = "Checks if the value of the field is `RESERVED3`"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == FSEL21_A::RESERVED3
    }
    #[doc = "Checks if the value of the field is `SPI1_SCLK`"]
    #[inline(always)]
    pub fn is_spi1_sclk(&self) -> bool {
        *self == FSEL21_A::SPI1_SCLK
    }
    #[doc = "Checks if the value of the field is `GPCLK1`"]
    #[inline(always)]
    pub fn is_gpclk1(&self) -> bool {
        *self == FSEL21_A::GPCLK1
    }
}
#[doc = "Field `FSEL21` writer - Function Select 21"]
pub type FSEL21_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPFSEL2_SPEC, u8, FSEL21_A, 3, O>;
impl<'a, const O: u8> FSEL21_W<'a, O> {
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FSEL21_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(FSEL21_A::OUTPUT)
    }
    #[doc = "Pin is connected to PCM_DOUT"]
    #[inline(always)]
    pub fn pcm_dout(self) -> &'a mut W {
        self.variant(FSEL21_A::PCM_DOUT)
    }
    #[doc = "Pin is connected to SD13"]
    #[inline(always)]
    pub fn sd13(self) -> &'a mut W {
        self.variant(FSEL21_A::SD13)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut W {
        self.variant(FSEL21_A::RESERVED2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut W {
        self.variant(FSEL21_A::RESERVED3)
    }
    #[doc = "Pin is connected to SPI1_SCLK"]
    #[inline(always)]
    pub fn spi1_sclk(self) -> &'a mut W {
        self.variant(FSEL21_A::SPI1_SCLK)
    }
    #[doc = "Pin is connected to GPCLK1"]
    #[inline(always)]
    pub fn gpclk1(self) -> &'a mut W {
        self.variant(FSEL21_A::GPCLK1)
    }
}
#[doc = "Field `FSEL22` reader - Function Select 22"]
pub type FSEL22_R = crate::FieldReader<u8, FSEL22_A>;
#[doc = "Function Select 22"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL22_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Alt function 0 reserved"]
    RESERVED0 = 4,
    #[doc = "5: Pin is connected to SD14"]
    SD14 = 5,
    #[doc = "6: Alt function 2 reserved"]
    RESERVED2 = 6,
    #[doc = "7: Pin is connected to SD1_CLK"]
    SD1_CLK = 7,
    #[doc = "3: Pin is connected to ARM_TRST"]
    ARM_TRST = 3,
    #[doc = "2: Alt function 5 reserved"]
    RESERVED5 = 2,
}
impl From<FSEL22_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL22_A) -> Self {
        variant as _
    }
}
impl FSEL22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSEL22_A {
        match self.bits {
            0 => FSEL22_A::INPUT,
            1 => FSEL22_A::OUTPUT,
            4 => FSEL22_A::RESERVED0,
            5 => FSEL22_A::SD14,
            6 => FSEL22_A::RESERVED2,
            7 => FSEL22_A::SD1_CLK,
            3 => FSEL22_A::ARM_TRST,
            2 => FSEL22_A::RESERVED5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL22_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL22_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `RESERVED0`"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == FSEL22_A::RESERVED0
    }
    #[doc = "Checks if the value of the field is `SD14`"]
    #[inline(always)]
    pub fn is_sd14(&self) -> bool {
        *self == FSEL22_A::SD14
    }
    #[doc = "Checks if the value of the field is `RESERVED2`"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL22_A::RESERVED2
    }
    #[doc = "Checks if the value of the field is `SD1_CLK`"]
    #[inline(always)]
    pub fn is_sd1_clk(&self) -> bool {
        *self == FSEL22_A::SD1_CLK
    }
    #[doc = "Checks if the value of the field is `ARM_TRST`"]
    #[inline(always)]
    pub fn is_arm_trst(&self) -> bool {
        *self == FSEL22_A::ARM_TRST
    }
    #[doc = "Checks if the value of the field is `RESERVED5`"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == FSEL22_A::RESERVED5
    }
}
#[doc = "Field `FSEL22` writer - Function Select 22"]
pub type FSEL22_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPFSEL2_SPEC, u8, FSEL22_A, 3, O>;
impl<'a, const O: u8> FSEL22_W<'a, O> {
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FSEL22_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(FSEL22_A::OUTPUT)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut W {
        self.variant(FSEL22_A::RESERVED0)
    }
    #[doc = "Pin is connected to SD14"]
    #[inline(always)]
    pub fn sd14(self) -> &'a mut W {
        self.variant(FSEL22_A::SD14)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut W {
        self.variant(FSEL22_A::RESERVED2)
    }
    #[doc = "Pin is connected to SD1_CLK"]
    #[inline(always)]
    pub fn sd1_clk(self) -> &'a mut W {
        self.variant(FSEL22_A::SD1_CLK)
    }
    #[doc = "Pin is connected to ARM_TRST"]
    #[inline(always)]
    pub fn arm_trst(self) -> &'a mut W {
        self.variant(FSEL22_A::ARM_TRST)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut W {
        self.variant(FSEL22_A::RESERVED5)
    }
}
#[doc = "Field `FSEL23` reader - Function Select 23"]
pub type FSEL23_R = crate::FieldReader<u8, FSEL23_A>;
#[doc = "Function Select 23"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL23_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Alt function 0 reserved"]
    RESERVED0 = 4,
    #[doc = "5: Pin is connected to SD15"]
    SD15 = 5,
    #[doc = "6: Alt function 2 reserved"]
    RESERVED2 = 6,
    #[doc = "7: Pin is connected to SD1_CMD"]
    SD1_CMD = 7,
    #[doc = "3: Pin is connected to ARM_RTCK"]
    ARM_RTCK = 3,
    #[doc = "2: Alt function 5 reserved"]
    RESERVED5 = 2,
}
impl From<FSEL23_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL23_A) -> Self {
        variant as _
    }
}
impl FSEL23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSEL23_A {
        match self.bits {
            0 => FSEL23_A::INPUT,
            1 => FSEL23_A::OUTPUT,
            4 => FSEL23_A::RESERVED0,
            5 => FSEL23_A::SD15,
            6 => FSEL23_A::RESERVED2,
            7 => FSEL23_A::SD1_CMD,
            3 => FSEL23_A::ARM_RTCK,
            2 => FSEL23_A::RESERVED5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL23_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL23_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `RESERVED0`"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == FSEL23_A::RESERVED0
    }
    #[doc = "Checks if the value of the field is `SD15`"]
    #[inline(always)]
    pub fn is_sd15(&self) -> bool {
        *self == FSEL23_A::SD15
    }
    #[doc = "Checks if the value of the field is `RESERVED2`"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL23_A::RESERVED2
    }
    #[doc = "Checks if the value of the field is `SD1_CMD`"]
    #[inline(always)]
    pub fn is_sd1_cmd(&self) -> bool {
        *self == FSEL23_A::SD1_CMD
    }
    #[doc = "Checks if the value of the field is `ARM_RTCK`"]
    #[inline(always)]
    pub fn is_arm_rtck(&self) -> bool {
        *self == FSEL23_A::ARM_RTCK
    }
    #[doc = "Checks if the value of the field is `RESERVED5`"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == FSEL23_A::RESERVED5
    }
}
#[doc = "Field `FSEL23` writer - Function Select 23"]
pub type FSEL23_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPFSEL2_SPEC, u8, FSEL23_A, 3, O>;
impl<'a, const O: u8> FSEL23_W<'a, O> {
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FSEL23_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(FSEL23_A::OUTPUT)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut W {
        self.variant(FSEL23_A::RESERVED0)
    }
    #[doc = "Pin is connected to SD15"]
    #[inline(always)]
    pub fn sd15(self) -> &'a mut W {
        self.variant(FSEL23_A::SD15)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut W {
        self.variant(FSEL23_A::RESERVED2)
    }
    #[doc = "Pin is connected to SD1_CMD"]
    #[inline(always)]
    pub fn sd1_cmd(self) -> &'a mut W {
        self.variant(FSEL23_A::SD1_CMD)
    }
    #[doc = "Pin is connected to ARM_RTCK"]
    #[inline(always)]
    pub fn arm_rtck(self) -> &'a mut W {
        self.variant(FSEL23_A::ARM_RTCK)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut W {
        self.variant(FSEL23_A::RESERVED5)
    }
}
#[doc = "Field `FSEL24` reader - Function Select 24"]
pub type FSEL24_R = crate::FieldReader<u8, FSEL24_A>;
#[doc = "Function Select 24"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL24_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Alt function 0 reserved"]
    RESERVED0 = 4,
    #[doc = "5: Pin is connected to SD16"]
    SD16 = 5,
    #[doc = "6: Alt function 2 reserved"]
    RESERVED2 = 6,
    #[doc = "7: Pin is connected to SD1_DAT0"]
    SD1_DAT0 = 7,
    #[doc = "3: Pin is connected to ARM_TDO"]
    ARM_TDO = 3,
    #[doc = "2: Alt function 5 reserved"]
    RESERVED5 = 2,
}
impl From<FSEL24_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL24_A) -> Self {
        variant as _
    }
}
impl FSEL24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSEL24_A {
        match self.bits {
            0 => FSEL24_A::INPUT,
            1 => FSEL24_A::OUTPUT,
            4 => FSEL24_A::RESERVED0,
            5 => FSEL24_A::SD16,
            6 => FSEL24_A::RESERVED2,
            7 => FSEL24_A::SD1_DAT0,
            3 => FSEL24_A::ARM_TDO,
            2 => FSEL24_A::RESERVED5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL24_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL24_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `RESERVED0`"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == FSEL24_A::RESERVED0
    }
    #[doc = "Checks if the value of the field is `SD16`"]
    #[inline(always)]
    pub fn is_sd16(&self) -> bool {
        *self == FSEL24_A::SD16
    }
    #[doc = "Checks if the value of the field is `RESERVED2`"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL24_A::RESERVED2
    }
    #[doc = "Checks if the value of the field is `SD1_DAT0`"]
    #[inline(always)]
    pub fn is_sd1_dat0(&self) -> bool {
        *self == FSEL24_A::SD1_DAT0
    }
    #[doc = "Checks if the value of the field is `ARM_TDO`"]
    #[inline(always)]
    pub fn is_arm_tdo(&self) -> bool {
        *self == FSEL24_A::ARM_TDO
    }
    #[doc = "Checks if the value of the field is `RESERVED5`"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == FSEL24_A::RESERVED5
    }
}
#[doc = "Field `FSEL24` writer - Function Select 24"]
pub type FSEL24_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPFSEL2_SPEC, u8, FSEL24_A, 3, O>;
impl<'a, const O: u8> FSEL24_W<'a, O> {
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FSEL24_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(FSEL24_A::OUTPUT)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut W {
        self.variant(FSEL24_A::RESERVED0)
    }
    #[doc = "Pin is connected to SD16"]
    #[inline(always)]
    pub fn sd16(self) -> &'a mut W {
        self.variant(FSEL24_A::SD16)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut W {
        self.variant(FSEL24_A::RESERVED2)
    }
    #[doc = "Pin is connected to SD1_DAT0"]
    #[inline(always)]
    pub fn sd1_dat0(self) -> &'a mut W {
        self.variant(FSEL24_A::SD1_DAT0)
    }
    #[doc = "Pin is connected to ARM_TDO"]
    #[inline(always)]
    pub fn arm_tdo(self) -> &'a mut W {
        self.variant(FSEL24_A::ARM_TDO)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut W {
        self.variant(FSEL24_A::RESERVED5)
    }
}
#[doc = "Field `FSEL25` reader - Function Select 25"]
pub type FSEL25_R = crate::FieldReader<u8, FSEL25_A>;
#[doc = "Function Select 25"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL25_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Alt function 0 reserved"]
    RESERVED0 = 4,
    #[doc = "5: Pin is connected to SD17"]
    SD17 = 5,
    #[doc = "6: Alt function 2 reserved"]
    RESERVED2 = 6,
    #[doc = "7: Pin is connected to SD1_DAT1"]
    SD1_DAT1 = 7,
    #[doc = "3: Pin is connected to ARM_TCK"]
    ARM_TCK = 3,
    #[doc = "2: Alt function 5 reserved"]
    RESERVED5 = 2,
}
impl From<FSEL25_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL25_A) -> Self {
        variant as _
    }
}
impl FSEL25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSEL25_A {
        match self.bits {
            0 => FSEL25_A::INPUT,
            1 => FSEL25_A::OUTPUT,
            4 => FSEL25_A::RESERVED0,
            5 => FSEL25_A::SD17,
            6 => FSEL25_A::RESERVED2,
            7 => FSEL25_A::SD1_DAT1,
            3 => FSEL25_A::ARM_TCK,
            2 => FSEL25_A::RESERVED5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL25_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL25_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `RESERVED0`"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == FSEL25_A::RESERVED0
    }
    #[doc = "Checks if the value of the field is `SD17`"]
    #[inline(always)]
    pub fn is_sd17(&self) -> bool {
        *self == FSEL25_A::SD17
    }
    #[doc = "Checks if the value of the field is `RESERVED2`"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL25_A::RESERVED2
    }
    #[doc = "Checks if the value of the field is `SD1_DAT1`"]
    #[inline(always)]
    pub fn is_sd1_dat1(&self) -> bool {
        *self == FSEL25_A::SD1_DAT1
    }
    #[doc = "Checks if the value of the field is `ARM_TCK`"]
    #[inline(always)]
    pub fn is_arm_tck(&self) -> bool {
        *self == FSEL25_A::ARM_TCK
    }
    #[doc = "Checks if the value of the field is `RESERVED5`"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == FSEL25_A::RESERVED5
    }
}
#[doc = "Field `FSEL25` writer - Function Select 25"]
pub type FSEL25_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPFSEL2_SPEC, u8, FSEL25_A, 3, O>;
impl<'a, const O: u8> FSEL25_W<'a, O> {
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FSEL25_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(FSEL25_A::OUTPUT)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut W {
        self.variant(FSEL25_A::RESERVED0)
    }
    #[doc = "Pin is connected to SD17"]
    #[inline(always)]
    pub fn sd17(self) -> &'a mut W {
        self.variant(FSEL25_A::SD17)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut W {
        self.variant(FSEL25_A::RESERVED2)
    }
    #[doc = "Pin is connected to SD1_DAT1"]
    #[inline(always)]
    pub fn sd1_dat1(self) -> &'a mut W {
        self.variant(FSEL25_A::SD1_DAT1)
    }
    #[doc = "Pin is connected to ARM_TCK"]
    #[inline(always)]
    pub fn arm_tck(self) -> &'a mut W {
        self.variant(FSEL25_A::ARM_TCK)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut W {
        self.variant(FSEL25_A::RESERVED5)
    }
}
#[doc = "Field `FSEL26` reader - Function Select 26"]
pub type FSEL26_R = crate::FieldReader<u8, FSEL26_A>;
#[doc = "Function Select 26"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL26_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Alt function 0 reserved"]
    RESERVED0 = 4,
    #[doc = "5: Alt function 1 reserved"]
    RESERVED1 = 5,
    #[doc = "6: Alt function 2 reserved"]
    RESERVED2 = 6,
    #[doc = "7: Pin is connected to SD1_DAT2"]
    SD1_DAT2 = 7,
    #[doc = "3: Pin is connected to ARM_TDI"]
    ARM_TDI = 3,
    #[doc = "2: Alt function 5 reserved"]
    RESERVED5 = 2,
}
impl From<FSEL26_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL26_A) -> Self {
        variant as _
    }
}
impl FSEL26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSEL26_A {
        match self.bits {
            0 => FSEL26_A::INPUT,
            1 => FSEL26_A::OUTPUT,
            4 => FSEL26_A::RESERVED0,
            5 => FSEL26_A::RESERVED1,
            6 => FSEL26_A::RESERVED2,
            7 => FSEL26_A::SD1_DAT2,
            3 => FSEL26_A::ARM_TDI,
            2 => FSEL26_A::RESERVED5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL26_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL26_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `RESERVED0`"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == FSEL26_A::RESERVED0
    }
    #[doc = "Checks if the value of the field is `RESERVED1`"]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == FSEL26_A::RESERVED1
    }
    #[doc = "Checks if the value of the field is `RESERVED2`"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL26_A::RESERVED2
    }
    #[doc = "Checks if the value of the field is `SD1_DAT2`"]
    #[inline(always)]
    pub fn is_sd1_dat2(&self) -> bool {
        *self == FSEL26_A::SD1_DAT2
    }
    #[doc = "Checks if the value of the field is `ARM_TDI`"]
    #[inline(always)]
    pub fn is_arm_tdi(&self) -> bool {
        *self == FSEL26_A::ARM_TDI
    }
    #[doc = "Checks if the value of the field is `RESERVED5`"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == FSEL26_A::RESERVED5
    }
}
#[doc = "Field `FSEL26` writer - Function Select 26"]
pub type FSEL26_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPFSEL2_SPEC, u8, FSEL26_A, 3, O>;
impl<'a, const O: u8> FSEL26_W<'a, O> {
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FSEL26_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(FSEL26_A::OUTPUT)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut W {
        self.variant(FSEL26_A::RESERVED0)
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut W {
        self.variant(FSEL26_A::RESERVED1)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut W {
        self.variant(FSEL26_A::RESERVED2)
    }
    #[doc = "Pin is connected to SD1_DAT2"]
    #[inline(always)]
    pub fn sd1_dat2(self) -> &'a mut W {
        self.variant(FSEL26_A::SD1_DAT2)
    }
    #[doc = "Pin is connected to ARM_TDI"]
    #[inline(always)]
    pub fn arm_tdi(self) -> &'a mut W {
        self.variant(FSEL26_A::ARM_TDI)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut W {
        self.variant(FSEL26_A::RESERVED5)
    }
}
#[doc = "Field `FSEL27` reader - Function Select 27"]
pub type FSEL27_R = crate::FieldReader<u8, FSEL27_A>;
#[doc = "Function Select 27"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL27_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Alt function 0 reserved"]
    RESERVED0 = 4,
    #[doc = "5: Alt function 1 reserved"]
    RESERVED1 = 5,
    #[doc = "6: Alt function 2 reserved"]
    RESERVED2 = 6,
    #[doc = "7: Pin is connected to SD1_DAT3"]
    SD1_DAT3 = 7,
    #[doc = "3: Pin is connected to ARM_TMS"]
    ARM_TMS = 3,
    #[doc = "2: Alt function 5 reserved"]
    RESERVED5 = 2,
}
impl From<FSEL27_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL27_A) -> Self {
        variant as _
    }
}
impl FSEL27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSEL27_A {
        match self.bits {
            0 => FSEL27_A::INPUT,
            1 => FSEL27_A::OUTPUT,
            4 => FSEL27_A::RESERVED0,
            5 => FSEL27_A::RESERVED1,
            6 => FSEL27_A::RESERVED2,
            7 => FSEL27_A::SD1_DAT3,
            3 => FSEL27_A::ARM_TMS,
            2 => FSEL27_A::RESERVED5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL27_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL27_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `RESERVED0`"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == FSEL27_A::RESERVED0
    }
    #[doc = "Checks if the value of the field is `RESERVED1`"]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == FSEL27_A::RESERVED1
    }
    #[doc = "Checks if the value of the field is `RESERVED2`"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL27_A::RESERVED2
    }
    #[doc = "Checks if the value of the field is `SD1_DAT3`"]
    #[inline(always)]
    pub fn is_sd1_dat3(&self) -> bool {
        *self == FSEL27_A::SD1_DAT3
    }
    #[doc = "Checks if the value of the field is `ARM_TMS`"]
    #[inline(always)]
    pub fn is_arm_tms(&self) -> bool {
        *self == FSEL27_A::ARM_TMS
    }
    #[doc = "Checks if the value of the field is `RESERVED5`"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == FSEL27_A::RESERVED5
    }
}
#[doc = "Field `FSEL27` writer - Function Select 27"]
pub type FSEL27_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPFSEL2_SPEC, u8, FSEL27_A, 3, O>;
impl<'a, const O: u8> FSEL27_W<'a, O> {
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FSEL27_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(FSEL27_A::OUTPUT)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut W {
        self.variant(FSEL27_A::RESERVED0)
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut W {
        self.variant(FSEL27_A::RESERVED1)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut W {
        self.variant(FSEL27_A::RESERVED2)
    }
    #[doc = "Pin is connected to SD1_DAT3"]
    #[inline(always)]
    pub fn sd1_dat3(self) -> &'a mut W {
        self.variant(FSEL27_A::SD1_DAT3)
    }
    #[doc = "Pin is connected to ARM_TMS"]
    #[inline(always)]
    pub fn arm_tms(self) -> &'a mut W {
        self.variant(FSEL27_A::ARM_TMS)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut W {
        self.variant(FSEL27_A::RESERVED5)
    }
}
#[doc = "Field `FSEL28` reader - Function Select 28"]
pub type FSEL28_R = crate::FieldReader<u8, FSEL28_A>;
#[doc = "Function Select 28"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL28_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Pin is connected to SDA0"]
    SDA0 = 4,
    #[doc = "5: Pin is connected to SA5"]
    SA5 = 5,
    #[doc = "6: Pin is connected to PCM_CLK"]
    PCM_CLK = 6,
    #[doc = "7: Alt function 3 reserved"]
    RESERVED3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    RESERVED4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    RESERVED5 = 2,
}
impl From<FSEL28_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL28_A) -> Self {
        variant as _
    }
}
impl FSEL28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSEL28_A {
        match self.bits {
            0 => FSEL28_A::INPUT,
            1 => FSEL28_A::OUTPUT,
            4 => FSEL28_A::SDA0,
            5 => FSEL28_A::SA5,
            6 => FSEL28_A::PCM_CLK,
            7 => FSEL28_A::RESERVED3,
            3 => FSEL28_A::RESERVED4,
            2 => FSEL28_A::RESERVED5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL28_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL28_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `SDA0`"]
    #[inline(always)]
    pub fn is_sda0(&self) -> bool {
        *self == FSEL28_A::SDA0
    }
    #[doc = "Checks if the value of the field is `SA5`"]
    #[inline(always)]
    pub fn is_sa5(&self) -> bool {
        *self == FSEL28_A::SA5
    }
    #[doc = "Checks if the value of the field is `PCM_CLK`"]
    #[inline(always)]
    pub fn is_pcm_clk(&self) -> bool {
        *self == FSEL28_A::PCM_CLK
    }
    #[doc = "Checks if the value of the field is `RESERVED3`"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == FSEL28_A::RESERVED3
    }
    #[doc = "Checks if the value of the field is `RESERVED4`"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL28_A::RESERVED4
    }
    #[doc = "Checks if the value of the field is `RESERVED5`"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == FSEL28_A::RESERVED5
    }
}
#[doc = "Field `FSEL28` writer - Function Select 28"]
pub type FSEL28_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPFSEL2_SPEC, u8, FSEL28_A, 3, O>;
impl<'a, const O: u8> FSEL28_W<'a, O> {
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FSEL28_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(FSEL28_A::OUTPUT)
    }
    #[doc = "Pin is connected to SDA0"]
    #[inline(always)]
    pub fn sda0(self) -> &'a mut W {
        self.variant(FSEL28_A::SDA0)
    }
    #[doc = "Pin is connected to SA5"]
    #[inline(always)]
    pub fn sa5(self) -> &'a mut W {
        self.variant(FSEL28_A::SA5)
    }
    #[doc = "Pin is connected to PCM_CLK"]
    #[inline(always)]
    pub fn pcm_clk(self) -> &'a mut W {
        self.variant(FSEL28_A::PCM_CLK)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut W {
        self.variant(FSEL28_A::RESERVED3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut W {
        self.variant(FSEL28_A::RESERVED4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut W {
        self.variant(FSEL28_A::RESERVED5)
    }
}
#[doc = "Field `FSEL29` reader - Function Select 29"]
pub type FSEL29_R = crate::FieldReader<u8, FSEL29_A>;
#[doc = "Function Select 29"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL29_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Pin is connected to SCL0"]
    SCL0 = 4,
    #[doc = "5: Pin is connected to SA4"]
    SA4 = 5,
    #[doc = "6: Pin is connected to PCM_FS"]
    PCM_FS = 6,
    #[doc = "7: Alt function 3 reserved"]
    RESERVED3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    RESERVED4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    RESERVED5 = 2,
}
impl From<FSEL29_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL29_A) -> Self {
        variant as _
    }
}
impl FSEL29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSEL29_A {
        match self.bits {
            0 => FSEL29_A::INPUT,
            1 => FSEL29_A::OUTPUT,
            4 => FSEL29_A::SCL0,
            5 => FSEL29_A::SA4,
            6 => FSEL29_A::PCM_FS,
            7 => FSEL29_A::RESERVED3,
            3 => FSEL29_A::RESERVED4,
            2 => FSEL29_A::RESERVED5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL29_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL29_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `SCL0`"]
    #[inline(always)]
    pub fn is_scl0(&self) -> bool {
        *self == FSEL29_A::SCL0
    }
    #[doc = "Checks if the value of the field is `SA4`"]
    #[inline(always)]
    pub fn is_sa4(&self) -> bool {
        *self == FSEL29_A::SA4
    }
    #[doc = "Checks if the value of the field is `PCM_FS`"]
    #[inline(always)]
    pub fn is_pcm_fs(&self) -> bool {
        *self == FSEL29_A::PCM_FS
    }
    #[doc = "Checks if the value of the field is `RESERVED3`"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == FSEL29_A::RESERVED3
    }
    #[doc = "Checks if the value of the field is `RESERVED4`"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL29_A::RESERVED4
    }
    #[doc = "Checks if the value of the field is `RESERVED5`"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == FSEL29_A::RESERVED5
    }
}
#[doc = "Field `FSEL29` writer - Function Select 29"]
pub type FSEL29_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPFSEL2_SPEC, u8, FSEL29_A, 3, O>;
impl<'a, const O: u8> FSEL29_W<'a, O> {
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FSEL29_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(FSEL29_A::OUTPUT)
    }
    #[doc = "Pin is connected to SCL0"]
    #[inline(always)]
    pub fn scl0(self) -> &'a mut W {
        self.variant(FSEL29_A::SCL0)
    }
    #[doc = "Pin is connected to SA4"]
    #[inline(always)]
    pub fn sa4(self) -> &'a mut W {
        self.variant(FSEL29_A::SA4)
    }
    #[doc = "Pin is connected to PCM_FS"]
    #[inline(always)]
    pub fn pcm_fs(self) -> &'a mut W {
        self.variant(FSEL29_A::PCM_FS)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut W {
        self.variant(FSEL29_A::RESERVED3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut W {
        self.variant(FSEL29_A::RESERVED4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut W {
        self.variant(FSEL29_A::RESERVED5)
    }
}
impl R {
    #[doc = "Bits 0:2 - Function Select 20"]
    #[inline(always)]
    pub fn fsel20(&self) -> FSEL20_R {
        FSEL20_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Function Select 21"]
    #[inline(always)]
    pub fn fsel21(&self) -> FSEL21_R {
        FSEL21_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Function Select 22"]
    #[inline(always)]
    pub fn fsel22(&self) -> FSEL22_R {
        FSEL22_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Function Select 23"]
    #[inline(always)]
    pub fn fsel23(&self) -> FSEL23_R {
        FSEL23_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Function Select 24"]
    #[inline(always)]
    pub fn fsel24(&self) -> FSEL24_R {
        FSEL24_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Function Select 25"]
    #[inline(always)]
    pub fn fsel25(&self) -> FSEL25_R {
        FSEL25_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Function Select 26"]
    #[inline(always)]
    pub fn fsel26(&self) -> FSEL26_R {
        FSEL26_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Function Select 27"]
    #[inline(always)]
    pub fn fsel27(&self) -> FSEL27_R {
        FSEL27_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Function Select 28"]
    #[inline(always)]
    pub fn fsel28(&self) -> FSEL28_R {
        FSEL28_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - Function Select 29"]
    #[inline(always)]
    pub fn fsel29(&self) -> FSEL29_R {
        FSEL29_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Function Select 20"]
    #[inline(always)]
    #[must_use]
    pub fn fsel20(&mut self) -> FSEL20_W<0> {
        FSEL20_W::new(self)
    }
    #[doc = "Bits 3:5 - Function Select 21"]
    #[inline(always)]
    #[must_use]
    pub fn fsel21(&mut self) -> FSEL21_W<3> {
        FSEL21_W::new(self)
    }
    #[doc = "Bits 6:8 - Function Select 22"]
    #[inline(always)]
    #[must_use]
    pub fn fsel22(&mut self) -> FSEL22_W<6> {
        FSEL22_W::new(self)
    }
    #[doc = "Bits 9:11 - Function Select 23"]
    #[inline(always)]
    #[must_use]
    pub fn fsel23(&mut self) -> FSEL23_W<9> {
        FSEL23_W::new(self)
    }
    #[doc = "Bits 12:14 - Function Select 24"]
    #[inline(always)]
    #[must_use]
    pub fn fsel24(&mut self) -> FSEL24_W<12> {
        FSEL24_W::new(self)
    }
    #[doc = "Bits 15:17 - Function Select 25"]
    #[inline(always)]
    #[must_use]
    pub fn fsel25(&mut self) -> FSEL25_W<15> {
        FSEL25_W::new(self)
    }
    #[doc = "Bits 18:20 - Function Select 26"]
    #[inline(always)]
    #[must_use]
    pub fn fsel26(&mut self) -> FSEL26_W<18> {
        FSEL26_W::new(self)
    }
    #[doc = "Bits 21:23 - Function Select 27"]
    #[inline(always)]
    #[must_use]
    pub fn fsel27(&mut self) -> FSEL27_W<21> {
        FSEL27_W::new(self)
    }
    #[doc = "Bits 24:26 - Function Select 28"]
    #[inline(always)]
    #[must_use]
    pub fn fsel28(&mut self) -> FSEL28_W<24> {
        FSEL28_W::new(self)
    }
    #[doc = "Bits 27:29 - Function Select 29"]
    #[inline(always)]
    #[must_use]
    pub fn fsel29(&mut self) -> FSEL29_W<27> {
        FSEL29_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Function Select 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpfsel2](index.html) module"]
pub struct GPFSEL2_SPEC;
impl crate::RegisterSpec for GPFSEL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpfsel2::R](R) reader structure"]
impl crate::Readable for GPFSEL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpfsel2::W](W) writer structure"]
impl crate::Writable for GPFSEL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
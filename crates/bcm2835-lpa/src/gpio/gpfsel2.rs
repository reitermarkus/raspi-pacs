#[doc = "Register `GPFSEL2` reader"]
pub type R = crate::R<Gpfsel2Spec>;
#[doc = "Register `GPFSEL2` writer"]
pub type W = crate::W<Gpfsel2Spec>;
#[doc = "Function Select 20"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel20 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Pin is connected to PCM_DIN"]
    PcmDin = 4,
    #[doc = "5: Pin is connected to SD12"]
    Sd12 = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    Reserved3 = 7,
    #[doc = "3: Pin is connected to SPI1_MOSI"]
    Spi1Mosi = 3,
    #[doc = "2: Pin is connected to GPCLK0"]
    Gpclk0 = 2,
}
impl From<Fsel20> for u8 {
    #[inline(always)]
    fn from(variant: Fsel20) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel20 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel20 {}
#[doc = "Field `FSEL20` reader - Function Select 20"]
pub type Fsel20R = crate::FieldReader<Fsel20>;
impl Fsel20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel20 {
        match self.bits {
            0 => Fsel20::Input,
            1 => Fsel20::Output,
            4 => Fsel20::PcmDin,
            5 => Fsel20::Sd12,
            6 => Fsel20::Reserved2,
            7 => Fsel20::Reserved3,
            3 => Fsel20::Spi1Mosi,
            2 => Fsel20::Gpclk0,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel20::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel20::Output
    }
    #[doc = "Pin is connected to PCM_DIN"]
    #[inline(always)]
    pub fn is_pcm_din(&self) -> bool {
        *self == Fsel20::PcmDin
    }
    #[doc = "Pin is connected to SD12"]
    #[inline(always)]
    pub fn is_sd12(&self) -> bool {
        *self == Fsel20::Sd12
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel20::Reserved2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == Fsel20::Reserved3
    }
    #[doc = "Pin is connected to SPI1_MOSI"]
    #[inline(always)]
    pub fn is_spi1_mosi(&self) -> bool {
        *self == Fsel20::Spi1Mosi
    }
    #[doc = "Pin is connected to GPCLK0"]
    #[inline(always)]
    pub fn is_gpclk0(&self) -> bool {
        *self == Fsel20::Gpclk0
    }
}
#[doc = "Field `FSEL20` writer - Function Select 20"]
pub type Fsel20W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel20, crate::Safe>;
impl<'a, REG> Fsel20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel20::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel20::Output)
    }
    #[doc = "Pin is connected to PCM_DIN"]
    #[inline(always)]
    pub fn pcm_din(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel20::PcmDin)
    }
    #[doc = "Pin is connected to SD12"]
    #[inline(always)]
    pub fn sd12(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel20::Sd12)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel20::Reserved2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel20::Reserved3)
    }
    #[doc = "Pin is connected to SPI1_MOSI"]
    #[inline(always)]
    pub fn spi1_mosi(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel20::Spi1Mosi)
    }
    #[doc = "Pin is connected to GPCLK0"]
    #[inline(always)]
    pub fn gpclk0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel20::Gpclk0)
    }
}
#[doc = "Function Select 21"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel21 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Pin is connected to PCM_DOUT"]
    PcmDout = 4,
    #[doc = "5: Pin is connected to SD13"]
    Sd13 = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    Reserved3 = 7,
    #[doc = "3: Pin is connected to SPI1_SCLK"]
    Spi1Sclk = 3,
    #[doc = "2: Pin is connected to GPCLK1"]
    Gpclk1 = 2,
}
impl From<Fsel21> for u8 {
    #[inline(always)]
    fn from(variant: Fsel21) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel21 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel21 {}
#[doc = "Field `FSEL21` reader - Function Select 21"]
pub type Fsel21R = crate::FieldReader<Fsel21>;
impl Fsel21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel21 {
        match self.bits {
            0 => Fsel21::Input,
            1 => Fsel21::Output,
            4 => Fsel21::PcmDout,
            5 => Fsel21::Sd13,
            6 => Fsel21::Reserved2,
            7 => Fsel21::Reserved3,
            3 => Fsel21::Spi1Sclk,
            2 => Fsel21::Gpclk1,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel21::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel21::Output
    }
    #[doc = "Pin is connected to PCM_DOUT"]
    #[inline(always)]
    pub fn is_pcm_dout(&self) -> bool {
        *self == Fsel21::PcmDout
    }
    #[doc = "Pin is connected to SD13"]
    #[inline(always)]
    pub fn is_sd13(&self) -> bool {
        *self == Fsel21::Sd13
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel21::Reserved2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == Fsel21::Reserved3
    }
    #[doc = "Pin is connected to SPI1_SCLK"]
    #[inline(always)]
    pub fn is_spi1_sclk(&self) -> bool {
        *self == Fsel21::Spi1Sclk
    }
    #[doc = "Pin is connected to GPCLK1"]
    #[inline(always)]
    pub fn is_gpclk1(&self) -> bool {
        *self == Fsel21::Gpclk1
    }
}
#[doc = "Field `FSEL21` writer - Function Select 21"]
pub type Fsel21W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel21, crate::Safe>;
impl<'a, REG> Fsel21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel21::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel21::Output)
    }
    #[doc = "Pin is connected to PCM_DOUT"]
    #[inline(always)]
    pub fn pcm_dout(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel21::PcmDout)
    }
    #[doc = "Pin is connected to SD13"]
    #[inline(always)]
    pub fn sd13(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel21::Sd13)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel21::Reserved2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel21::Reserved3)
    }
    #[doc = "Pin is connected to SPI1_SCLK"]
    #[inline(always)]
    pub fn spi1_sclk(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel21::Spi1Sclk)
    }
    #[doc = "Pin is connected to GPCLK1"]
    #[inline(always)]
    pub fn gpclk1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel21::Gpclk1)
    }
}
#[doc = "Function Select 22"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel22 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Alt function 0 reserved"]
    Reserved0 = 4,
    #[doc = "5: Pin is connected to SD14"]
    Sd14 = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Pin is connected to SD1_CLK"]
    Sd1Clk = 7,
    #[doc = "3: Pin is connected to ARM_TRST"]
    ArmTrst = 3,
    #[doc = "2: Alt function 5 reserved"]
    Reserved5 = 2,
}
impl From<Fsel22> for u8 {
    #[inline(always)]
    fn from(variant: Fsel22) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel22 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel22 {}
#[doc = "Field `FSEL22` reader - Function Select 22"]
pub type Fsel22R = crate::FieldReader<Fsel22>;
impl Fsel22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel22 {
        match self.bits {
            0 => Fsel22::Input,
            1 => Fsel22::Output,
            4 => Fsel22::Reserved0,
            5 => Fsel22::Sd14,
            6 => Fsel22::Reserved2,
            7 => Fsel22::Sd1Clk,
            3 => Fsel22::ArmTrst,
            2 => Fsel22::Reserved5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel22::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel22::Output
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fsel22::Reserved0
    }
    #[doc = "Pin is connected to SD14"]
    #[inline(always)]
    pub fn is_sd14(&self) -> bool {
        *self == Fsel22::Sd14
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel22::Reserved2
    }
    #[doc = "Pin is connected to SD1_CLK"]
    #[inline(always)]
    pub fn is_sd1_clk(&self) -> bool {
        *self == Fsel22::Sd1Clk
    }
    #[doc = "Pin is connected to ARM_TRST"]
    #[inline(always)]
    pub fn is_arm_trst(&self) -> bool {
        *self == Fsel22::ArmTrst
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fsel22::Reserved5
    }
}
#[doc = "Field `FSEL22` writer - Function Select 22"]
pub type Fsel22W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel22, crate::Safe>;
impl<'a, REG> Fsel22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel22::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel22::Output)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel22::Reserved0)
    }
    #[doc = "Pin is connected to SD14"]
    #[inline(always)]
    pub fn sd14(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel22::Sd14)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel22::Reserved2)
    }
    #[doc = "Pin is connected to SD1_CLK"]
    #[inline(always)]
    pub fn sd1_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel22::Sd1Clk)
    }
    #[doc = "Pin is connected to ARM_TRST"]
    #[inline(always)]
    pub fn arm_trst(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel22::ArmTrst)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel22::Reserved5)
    }
}
#[doc = "Function Select 23"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel23 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Alt function 0 reserved"]
    Reserved0 = 4,
    #[doc = "5: Pin is connected to SD15"]
    Sd15 = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Pin is connected to SD1_CMD"]
    Sd1Cmd = 7,
    #[doc = "3: Pin is connected to ARM_RTCK"]
    ArmRtck = 3,
    #[doc = "2: Alt function 5 reserved"]
    Reserved5 = 2,
}
impl From<Fsel23> for u8 {
    #[inline(always)]
    fn from(variant: Fsel23) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel23 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel23 {}
#[doc = "Field `FSEL23` reader - Function Select 23"]
pub type Fsel23R = crate::FieldReader<Fsel23>;
impl Fsel23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel23 {
        match self.bits {
            0 => Fsel23::Input,
            1 => Fsel23::Output,
            4 => Fsel23::Reserved0,
            5 => Fsel23::Sd15,
            6 => Fsel23::Reserved2,
            7 => Fsel23::Sd1Cmd,
            3 => Fsel23::ArmRtck,
            2 => Fsel23::Reserved5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel23::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel23::Output
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fsel23::Reserved0
    }
    #[doc = "Pin is connected to SD15"]
    #[inline(always)]
    pub fn is_sd15(&self) -> bool {
        *self == Fsel23::Sd15
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel23::Reserved2
    }
    #[doc = "Pin is connected to SD1_CMD"]
    #[inline(always)]
    pub fn is_sd1_cmd(&self) -> bool {
        *self == Fsel23::Sd1Cmd
    }
    #[doc = "Pin is connected to ARM_RTCK"]
    #[inline(always)]
    pub fn is_arm_rtck(&self) -> bool {
        *self == Fsel23::ArmRtck
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fsel23::Reserved5
    }
}
#[doc = "Field `FSEL23` writer - Function Select 23"]
pub type Fsel23W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel23, crate::Safe>;
impl<'a, REG> Fsel23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel23::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel23::Output)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel23::Reserved0)
    }
    #[doc = "Pin is connected to SD15"]
    #[inline(always)]
    pub fn sd15(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel23::Sd15)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel23::Reserved2)
    }
    #[doc = "Pin is connected to SD1_CMD"]
    #[inline(always)]
    pub fn sd1_cmd(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel23::Sd1Cmd)
    }
    #[doc = "Pin is connected to ARM_RTCK"]
    #[inline(always)]
    pub fn arm_rtck(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel23::ArmRtck)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel23::Reserved5)
    }
}
#[doc = "Function Select 24"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel24 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Alt function 0 reserved"]
    Reserved0 = 4,
    #[doc = "5: Pin is connected to SD16"]
    Sd16 = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Pin is connected to SD1_DAT0"]
    Sd1Dat0 = 7,
    #[doc = "3: Pin is connected to ARM_TDO"]
    ArmTdo = 3,
    #[doc = "2: Alt function 5 reserved"]
    Reserved5 = 2,
}
impl From<Fsel24> for u8 {
    #[inline(always)]
    fn from(variant: Fsel24) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel24 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel24 {}
#[doc = "Field `FSEL24` reader - Function Select 24"]
pub type Fsel24R = crate::FieldReader<Fsel24>;
impl Fsel24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel24 {
        match self.bits {
            0 => Fsel24::Input,
            1 => Fsel24::Output,
            4 => Fsel24::Reserved0,
            5 => Fsel24::Sd16,
            6 => Fsel24::Reserved2,
            7 => Fsel24::Sd1Dat0,
            3 => Fsel24::ArmTdo,
            2 => Fsel24::Reserved5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel24::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel24::Output
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fsel24::Reserved0
    }
    #[doc = "Pin is connected to SD16"]
    #[inline(always)]
    pub fn is_sd16(&self) -> bool {
        *self == Fsel24::Sd16
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel24::Reserved2
    }
    #[doc = "Pin is connected to SD1_DAT0"]
    #[inline(always)]
    pub fn is_sd1_dat0(&self) -> bool {
        *self == Fsel24::Sd1Dat0
    }
    #[doc = "Pin is connected to ARM_TDO"]
    #[inline(always)]
    pub fn is_arm_tdo(&self) -> bool {
        *self == Fsel24::ArmTdo
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fsel24::Reserved5
    }
}
#[doc = "Field `FSEL24` writer - Function Select 24"]
pub type Fsel24W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel24, crate::Safe>;
impl<'a, REG> Fsel24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel24::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel24::Output)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel24::Reserved0)
    }
    #[doc = "Pin is connected to SD16"]
    #[inline(always)]
    pub fn sd16(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel24::Sd16)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel24::Reserved2)
    }
    #[doc = "Pin is connected to SD1_DAT0"]
    #[inline(always)]
    pub fn sd1_dat0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel24::Sd1Dat0)
    }
    #[doc = "Pin is connected to ARM_TDO"]
    #[inline(always)]
    pub fn arm_tdo(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel24::ArmTdo)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel24::Reserved5)
    }
}
#[doc = "Function Select 25"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel25 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Alt function 0 reserved"]
    Reserved0 = 4,
    #[doc = "5: Pin is connected to SD17"]
    Sd17 = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Pin is connected to SD1_DAT1"]
    Sd1Dat1 = 7,
    #[doc = "3: Pin is connected to ARM_TCK"]
    ArmTck = 3,
    #[doc = "2: Alt function 5 reserved"]
    Reserved5 = 2,
}
impl From<Fsel25> for u8 {
    #[inline(always)]
    fn from(variant: Fsel25) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel25 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel25 {}
#[doc = "Field `FSEL25` reader - Function Select 25"]
pub type Fsel25R = crate::FieldReader<Fsel25>;
impl Fsel25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel25 {
        match self.bits {
            0 => Fsel25::Input,
            1 => Fsel25::Output,
            4 => Fsel25::Reserved0,
            5 => Fsel25::Sd17,
            6 => Fsel25::Reserved2,
            7 => Fsel25::Sd1Dat1,
            3 => Fsel25::ArmTck,
            2 => Fsel25::Reserved5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel25::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel25::Output
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fsel25::Reserved0
    }
    #[doc = "Pin is connected to SD17"]
    #[inline(always)]
    pub fn is_sd17(&self) -> bool {
        *self == Fsel25::Sd17
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel25::Reserved2
    }
    #[doc = "Pin is connected to SD1_DAT1"]
    #[inline(always)]
    pub fn is_sd1_dat1(&self) -> bool {
        *self == Fsel25::Sd1Dat1
    }
    #[doc = "Pin is connected to ARM_TCK"]
    #[inline(always)]
    pub fn is_arm_tck(&self) -> bool {
        *self == Fsel25::ArmTck
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fsel25::Reserved5
    }
}
#[doc = "Field `FSEL25` writer - Function Select 25"]
pub type Fsel25W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel25, crate::Safe>;
impl<'a, REG> Fsel25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel25::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel25::Output)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel25::Reserved0)
    }
    #[doc = "Pin is connected to SD17"]
    #[inline(always)]
    pub fn sd17(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel25::Sd17)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel25::Reserved2)
    }
    #[doc = "Pin is connected to SD1_DAT1"]
    #[inline(always)]
    pub fn sd1_dat1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel25::Sd1Dat1)
    }
    #[doc = "Pin is connected to ARM_TCK"]
    #[inline(always)]
    pub fn arm_tck(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel25::ArmTck)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel25::Reserved5)
    }
}
#[doc = "Function Select 26"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel26 {
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
    #[doc = "7: Pin is connected to SD1_DAT2"]
    Sd1Dat2 = 7,
    #[doc = "3: Pin is connected to ARM_TDI"]
    ArmTdi = 3,
    #[doc = "2: Alt function 5 reserved"]
    Reserved5 = 2,
}
impl From<Fsel26> for u8 {
    #[inline(always)]
    fn from(variant: Fsel26) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel26 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel26 {}
#[doc = "Field `FSEL26` reader - Function Select 26"]
pub type Fsel26R = crate::FieldReader<Fsel26>;
impl Fsel26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel26 {
        match self.bits {
            0 => Fsel26::Input,
            1 => Fsel26::Output,
            4 => Fsel26::Reserved0,
            5 => Fsel26::Reserved1,
            6 => Fsel26::Reserved2,
            7 => Fsel26::Sd1Dat2,
            3 => Fsel26::ArmTdi,
            2 => Fsel26::Reserved5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel26::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel26::Output
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fsel26::Reserved0
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == Fsel26::Reserved1
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel26::Reserved2
    }
    #[doc = "Pin is connected to SD1_DAT2"]
    #[inline(always)]
    pub fn is_sd1_dat2(&self) -> bool {
        *self == Fsel26::Sd1Dat2
    }
    #[doc = "Pin is connected to ARM_TDI"]
    #[inline(always)]
    pub fn is_arm_tdi(&self) -> bool {
        *self == Fsel26::ArmTdi
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fsel26::Reserved5
    }
}
#[doc = "Field `FSEL26` writer - Function Select 26"]
pub type Fsel26W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel26, crate::Safe>;
impl<'a, REG> Fsel26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel26::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel26::Output)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel26::Reserved0)
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel26::Reserved1)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel26::Reserved2)
    }
    #[doc = "Pin is connected to SD1_DAT2"]
    #[inline(always)]
    pub fn sd1_dat2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel26::Sd1Dat2)
    }
    #[doc = "Pin is connected to ARM_TDI"]
    #[inline(always)]
    pub fn arm_tdi(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel26::ArmTdi)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel26::Reserved5)
    }
}
#[doc = "Function Select 27"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel27 {
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
    #[doc = "7: Pin is connected to SD1_DAT3"]
    Sd1Dat3 = 7,
    #[doc = "3: Pin is connected to ARM_TMS"]
    ArmTms = 3,
    #[doc = "2: Alt function 5 reserved"]
    Reserved5 = 2,
}
impl From<Fsel27> for u8 {
    #[inline(always)]
    fn from(variant: Fsel27) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel27 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel27 {}
#[doc = "Field `FSEL27` reader - Function Select 27"]
pub type Fsel27R = crate::FieldReader<Fsel27>;
impl Fsel27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel27 {
        match self.bits {
            0 => Fsel27::Input,
            1 => Fsel27::Output,
            4 => Fsel27::Reserved0,
            5 => Fsel27::Reserved1,
            6 => Fsel27::Reserved2,
            7 => Fsel27::Sd1Dat3,
            3 => Fsel27::ArmTms,
            2 => Fsel27::Reserved5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel27::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel27::Output
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fsel27::Reserved0
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == Fsel27::Reserved1
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel27::Reserved2
    }
    #[doc = "Pin is connected to SD1_DAT3"]
    #[inline(always)]
    pub fn is_sd1_dat3(&self) -> bool {
        *self == Fsel27::Sd1Dat3
    }
    #[doc = "Pin is connected to ARM_TMS"]
    #[inline(always)]
    pub fn is_arm_tms(&self) -> bool {
        *self == Fsel27::ArmTms
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fsel27::Reserved5
    }
}
#[doc = "Field `FSEL27` writer - Function Select 27"]
pub type Fsel27W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel27, crate::Safe>;
impl<'a, REG> Fsel27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel27::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel27::Output)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel27::Reserved0)
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel27::Reserved1)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel27::Reserved2)
    }
    #[doc = "Pin is connected to SD1_DAT3"]
    #[inline(always)]
    pub fn sd1_dat3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel27::Sd1Dat3)
    }
    #[doc = "Pin is connected to ARM_TMS"]
    #[inline(always)]
    pub fn arm_tms(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel27::ArmTms)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel27::Reserved5)
    }
}
#[doc = "Function Select 28"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel28 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Pin is connected to SDA0"]
    Sda0 = 4,
    #[doc = "5: Pin is connected to SA5"]
    Sa5 = 5,
    #[doc = "6: Pin is connected to PCM_CLK"]
    PcmClk = 6,
    #[doc = "7: Alt function 3 reserved"]
    Reserved3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    Reserved4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    Reserved5 = 2,
}
impl From<Fsel28> for u8 {
    #[inline(always)]
    fn from(variant: Fsel28) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel28 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel28 {}
#[doc = "Field `FSEL28` reader - Function Select 28"]
pub type Fsel28R = crate::FieldReader<Fsel28>;
impl Fsel28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel28 {
        match self.bits {
            0 => Fsel28::Input,
            1 => Fsel28::Output,
            4 => Fsel28::Sda0,
            5 => Fsel28::Sa5,
            6 => Fsel28::PcmClk,
            7 => Fsel28::Reserved3,
            3 => Fsel28::Reserved4,
            2 => Fsel28::Reserved5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel28::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel28::Output
    }
    #[doc = "Pin is connected to SDA0"]
    #[inline(always)]
    pub fn is_sda0(&self) -> bool {
        *self == Fsel28::Sda0
    }
    #[doc = "Pin is connected to SA5"]
    #[inline(always)]
    pub fn is_sa5(&self) -> bool {
        *self == Fsel28::Sa5
    }
    #[doc = "Pin is connected to PCM_CLK"]
    #[inline(always)]
    pub fn is_pcm_clk(&self) -> bool {
        *self == Fsel28::PcmClk
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == Fsel28::Reserved3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fsel28::Reserved4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fsel28::Reserved5
    }
}
#[doc = "Field `FSEL28` writer - Function Select 28"]
pub type Fsel28W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel28, crate::Safe>;
impl<'a, REG> Fsel28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel28::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel28::Output)
    }
    #[doc = "Pin is connected to SDA0"]
    #[inline(always)]
    pub fn sda0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel28::Sda0)
    }
    #[doc = "Pin is connected to SA5"]
    #[inline(always)]
    pub fn sa5(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel28::Sa5)
    }
    #[doc = "Pin is connected to PCM_CLK"]
    #[inline(always)]
    pub fn pcm_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel28::PcmClk)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel28::Reserved3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel28::Reserved4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel28::Reserved5)
    }
}
#[doc = "Function Select 29"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel29 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Pin is connected to SCL0"]
    Scl0 = 4,
    #[doc = "5: Pin is connected to SA4"]
    Sa4 = 5,
    #[doc = "6: Pin is connected to PCM_FS"]
    PcmFs = 6,
    #[doc = "7: Alt function 3 reserved"]
    Reserved3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    Reserved4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    Reserved5 = 2,
}
impl From<Fsel29> for u8 {
    #[inline(always)]
    fn from(variant: Fsel29) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel29 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel29 {}
#[doc = "Field `FSEL29` reader - Function Select 29"]
pub type Fsel29R = crate::FieldReader<Fsel29>;
impl Fsel29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel29 {
        match self.bits {
            0 => Fsel29::Input,
            1 => Fsel29::Output,
            4 => Fsel29::Scl0,
            5 => Fsel29::Sa4,
            6 => Fsel29::PcmFs,
            7 => Fsel29::Reserved3,
            3 => Fsel29::Reserved4,
            2 => Fsel29::Reserved5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel29::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel29::Output
    }
    #[doc = "Pin is connected to SCL0"]
    #[inline(always)]
    pub fn is_scl0(&self) -> bool {
        *self == Fsel29::Scl0
    }
    #[doc = "Pin is connected to SA4"]
    #[inline(always)]
    pub fn is_sa4(&self) -> bool {
        *self == Fsel29::Sa4
    }
    #[doc = "Pin is connected to PCM_FS"]
    #[inline(always)]
    pub fn is_pcm_fs(&self) -> bool {
        *self == Fsel29::PcmFs
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == Fsel29::Reserved3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fsel29::Reserved4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fsel29::Reserved5
    }
}
#[doc = "Field `FSEL29` writer - Function Select 29"]
pub type Fsel29W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel29, crate::Safe>;
impl<'a, REG> Fsel29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel29::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel29::Output)
    }
    #[doc = "Pin is connected to SCL0"]
    #[inline(always)]
    pub fn scl0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel29::Scl0)
    }
    #[doc = "Pin is connected to SA4"]
    #[inline(always)]
    pub fn sa4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel29::Sa4)
    }
    #[doc = "Pin is connected to PCM_FS"]
    #[inline(always)]
    pub fn pcm_fs(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel29::PcmFs)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel29::Reserved3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel29::Reserved4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel29::Reserved5)
    }
}
impl R {
    #[doc = "Bits 0:2 - Function Select 20"]
    #[inline(always)]
    pub fn fsel20(&self) -> Fsel20R {
        Fsel20R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Function Select 21"]
    #[inline(always)]
    pub fn fsel21(&self) -> Fsel21R {
        Fsel21R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Function Select 22"]
    #[inline(always)]
    pub fn fsel22(&self) -> Fsel22R {
        Fsel22R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Function Select 23"]
    #[inline(always)]
    pub fn fsel23(&self) -> Fsel23R {
        Fsel23R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Function Select 24"]
    #[inline(always)]
    pub fn fsel24(&self) -> Fsel24R {
        Fsel24R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Function Select 25"]
    #[inline(always)]
    pub fn fsel25(&self) -> Fsel25R {
        Fsel25R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Function Select 26"]
    #[inline(always)]
    pub fn fsel26(&self) -> Fsel26R {
        Fsel26R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Function Select 27"]
    #[inline(always)]
    pub fn fsel27(&self) -> Fsel27R {
        Fsel27R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Function Select 28"]
    #[inline(always)]
    pub fn fsel28(&self) -> Fsel28R {
        Fsel28R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - Function Select 29"]
    #[inline(always)]
    pub fn fsel29(&self) -> Fsel29R {
        Fsel29R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPFSEL2")
            .field("fsel20", &self.fsel20())
            .field("fsel21", &self.fsel21())
            .field("fsel22", &self.fsel22())
            .field("fsel23", &self.fsel23())
            .field("fsel24", &self.fsel24())
            .field("fsel25", &self.fsel25())
            .field("fsel26", &self.fsel26())
            .field("fsel27", &self.fsel27())
            .field("fsel28", &self.fsel28())
            .field("fsel29", &self.fsel29())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Function Select 20"]
    #[inline(always)]
    #[must_use]
    pub fn fsel20(&mut self) -> Fsel20W<Gpfsel2Spec> {
        Fsel20W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Function Select 21"]
    #[inline(always)]
    #[must_use]
    pub fn fsel21(&mut self) -> Fsel21W<Gpfsel2Spec> {
        Fsel21W::new(self, 3)
    }
    #[doc = "Bits 6:8 - Function Select 22"]
    #[inline(always)]
    #[must_use]
    pub fn fsel22(&mut self) -> Fsel22W<Gpfsel2Spec> {
        Fsel22W::new(self, 6)
    }
    #[doc = "Bits 9:11 - Function Select 23"]
    #[inline(always)]
    #[must_use]
    pub fn fsel23(&mut self) -> Fsel23W<Gpfsel2Spec> {
        Fsel23W::new(self, 9)
    }
    #[doc = "Bits 12:14 - Function Select 24"]
    #[inline(always)]
    #[must_use]
    pub fn fsel24(&mut self) -> Fsel24W<Gpfsel2Spec> {
        Fsel24W::new(self, 12)
    }
    #[doc = "Bits 15:17 - Function Select 25"]
    #[inline(always)]
    #[must_use]
    pub fn fsel25(&mut self) -> Fsel25W<Gpfsel2Spec> {
        Fsel25W::new(self, 15)
    }
    #[doc = "Bits 18:20 - Function Select 26"]
    #[inline(always)]
    #[must_use]
    pub fn fsel26(&mut self) -> Fsel26W<Gpfsel2Spec> {
        Fsel26W::new(self, 18)
    }
    #[doc = "Bits 21:23 - Function Select 27"]
    #[inline(always)]
    #[must_use]
    pub fn fsel27(&mut self) -> Fsel27W<Gpfsel2Spec> {
        Fsel27W::new(self, 21)
    }
    #[doc = "Bits 24:26 - Function Select 28"]
    #[inline(always)]
    #[must_use]
    pub fn fsel28(&mut self) -> Fsel28W<Gpfsel2Spec> {
        Fsel28W::new(self, 24)
    }
    #[doc = "Bits 27:29 - Function Select 29"]
    #[inline(always)]
    #[must_use]
    pub fn fsel29(&mut self) -> Fsel29W<Gpfsel2Spec> {
        Fsel29W::new(self, 27)
    }
}
#[doc = "GPIO Function Select 2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpfsel2::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpfsel2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpfsel2Spec;
impl crate::RegisterSpec for Gpfsel2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpfsel2::R`](R) reader structure"]
impl crate::Readable for Gpfsel2Spec {}
#[doc = "`write(|w| ..)` method takes [`gpfsel2::W`](W) writer structure"]
impl crate::Writable for Gpfsel2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}

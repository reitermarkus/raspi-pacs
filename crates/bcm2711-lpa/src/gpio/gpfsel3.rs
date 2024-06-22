#[doc = "Register `GPFSEL3` reader"]
pub type R = crate::R<Gpfsel3Spec>;
#[doc = "Register `GPFSEL3` writer"]
pub type W = crate::W<Gpfsel3Spec>;
#[doc = "Function Select 30"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel30 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Alt function 0 reserved"]
    Reserved0 = 4,
    #[doc = "5: Pin is connected to SA3"]
    Sa3 = 5,
    #[doc = "6: Pin is connected to PCM_DIN"]
    PcmDin = 6,
    #[doc = "7: Pin is connected to CTS0"]
    Cts0 = 7,
    #[doc = "3: Pin is connected to MII_A_CRS"]
    MiiACrs = 3,
    #[doc = "2: Pin is connected to CTS1"]
    Cts1 = 2,
}
impl From<Fsel30> for u8 {
    #[inline(always)]
    fn from(variant: Fsel30) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel30 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel30 {}
#[doc = "Field `FSEL30` reader - Function Select 30"]
pub type Fsel30R = crate::FieldReader<Fsel30>;
impl Fsel30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel30 {
        match self.bits {
            0 => Fsel30::Input,
            1 => Fsel30::Output,
            4 => Fsel30::Reserved0,
            5 => Fsel30::Sa3,
            6 => Fsel30::PcmDin,
            7 => Fsel30::Cts0,
            3 => Fsel30::MiiACrs,
            2 => Fsel30::Cts1,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel30::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel30::Output
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fsel30::Reserved0
    }
    #[doc = "Pin is connected to SA3"]
    #[inline(always)]
    pub fn is_sa3(&self) -> bool {
        *self == Fsel30::Sa3
    }
    #[doc = "Pin is connected to PCM_DIN"]
    #[inline(always)]
    pub fn is_pcm_din(&self) -> bool {
        *self == Fsel30::PcmDin
    }
    #[doc = "Pin is connected to CTS0"]
    #[inline(always)]
    pub fn is_cts0(&self) -> bool {
        *self == Fsel30::Cts0
    }
    #[doc = "Pin is connected to MII_A_CRS"]
    #[inline(always)]
    pub fn is_mii_a_crs(&self) -> bool {
        *self == Fsel30::MiiACrs
    }
    #[doc = "Pin is connected to CTS1"]
    #[inline(always)]
    pub fn is_cts1(&self) -> bool {
        *self == Fsel30::Cts1
    }
}
#[doc = "Field `FSEL30` writer - Function Select 30"]
pub type Fsel30W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel30, crate::Safe>;
impl<'a, REG> Fsel30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel30::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel30::Output)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel30::Reserved0)
    }
    #[doc = "Pin is connected to SA3"]
    #[inline(always)]
    pub fn sa3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel30::Sa3)
    }
    #[doc = "Pin is connected to PCM_DIN"]
    #[inline(always)]
    pub fn pcm_din(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel30::PcmDin)
    }
    #[doc = "Pin is connected to CTS0"]
    #[inline(always)]
    pub fn cts0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel30::Cts0)
    }
    #[doc = "Pin is connected to MII_A_CRS"]
    #[inline(always)]
    pub fn mii_a_crs(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel30::MiiACrs)
    }
    #[doc = "Pin is connected to CTS1"]
    #[inline(always)]
    pub fn cts1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel30::Cts1)
    }
}
#[doc = "Function Select 31"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel31 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Alt function 0 reserved"]
    Reserved0 = 4,
    #[doc = "5: Pin is connected to SA2"]
    Sa2 = 5,
    #[doc = "6: Pin is connected to PCM_DOUT"]
    PcmDout = 6,
    #[doc = "7: Pin is connected to RTS0"]
    Rts0 = 7,
    #[doc = "3: Pin is connected to MII_A_COL"]
    MiiACol = 3,
    #[doc = "2: Pin is connected to RTS1"]
    Rts1 = 2,
}
impl From<Fsel31> for u8 {
    #[inline(always)]
    fn from(variant: Fsel31) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel31 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel31 {}
#[doc = "Field `FSEL31` reader - Function Select 31"]
pub type Fsel31R = crate::FieldReader<Fsel31>;
impl Fsel31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel31 {
        match self.bits {
            0 => Fsel31::Input,
            1 => Fsel31::Output,
            4 => Fsel31::Reserved0,
            5 => Fsel31::Sa2,
            6 => Fsel31::PcmDout,
            7 => Fsel31::Rts0,
            3 => Fsel31::MiiACol,
            2 => Fsel31::Rts1,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel31::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel31::Output
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fsel31::Reserved0
    }
    #[doc = "Pin is connected to SA2"]
    #[inline(always)]
    pub fn is_sa2(&self) -> bool {
        *self == Fsel31::Sa2
    }
    #[doc = "Pin is connected to PCM_DOUT"]
    #[inline(always)]
    pub fn is_pcm_dout(&self) -> bool {
        *self == Fsel31::PcmDout
    }
    #[doc = "Pin is connected to RTS0"]
    #[inline(always)]
    pub fn is_rts0(&self) -> bool {
        *self == Fsel31::Rts0
    }
    #[doc = "Pin is connected to MII_A_COL"]
    #[inline(always)]
    pub fn is_mii_a_col(&self) -> bool {
        *self == Fsel31::MiiACol
    }
    #[doc = "Pin is connected to RTS1"]
    #[inline(always)]
    pub fn is_rts1(&self) -> bool {
        *self == Fsel31::Rts1
    }
}
#[doc = "Field `FSEL31` writer - Function Select 31"]
pub type Fsel31W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel31, crate::Safe>;
impl<'a, REG> Fsel31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel31::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel31::Output)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel31::Reserved0)
    }
    #[doc = "Pin is connected to SA2"]
    #[inline(always)]
    pub fn sa2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel31::Sa2)
    }
    #[doc = "Pin is connected to PCM_DOUT"]
    #[inline(always)]
    pub fn pcm_dout(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel31::PcmDout)
    }
    #[doc = "Pin is connected to RTS0"]
    #[inline(always)]
    pub fn rts0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel31::Rts0)
    }
    #[doc = "Pin is connected to MII_A_COL"]
    #[inline(always)]
    pub fn mii_a_col(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel31::MiiACol)
    }
    #[doc = "Pin is connected to RTS1"]
    #[inline(always)]
    pub fn rts1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel31::Rts1)
    }
}
#[doc = "Function Select 32"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel32 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Pin is connected to GPCLK0"]
    Gpclk0 = 4,
    #[doc = "5: Pin is connected to SA1"]
    Sa1 = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Pin is connected to TXD0"]
    Txd0 = 7,
    #[doc = "3: Pin is connected to SD_CARD_PRES"]
    SdCardPres = 3,
    #[doc = "2: Pin is connected to TXD1"]
    Txd1 = 2,
}
impl From<Fsel32> for u8 {
    #[inline(always)]
    fn from(variant: Fsel32) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel32 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel32 {}
#[doc = "Field `FSEL32` reader - Function Select 32"]
pub type Fsel32R = crate::FieldReader<Fsel32>;
impl Fsel32R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel32 {
        match self.bits {
            0 => Fsel32::Input,
            1 => Fsel32::Output,
            4 => Fsel32::Gpclk0,
            5 => Fsel32::Sa1,
            6 => Fsel32::Reserved2,
            7 => Fsel32::Txd0,
            3 => Fsel32::SdCardPres,
            2 => Fsel32::Txd1,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel32::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel32::Output
    }
    #[doc = "Pin is connected to GPCLK0"]
    #[inline(always)]
    pub fn is_gpclk0(&self) -> bool {
        *self == Fsel32::Gpclk0
    }
    #[doc = "Pin is connected to SA1"]
    #[inline(always)]
    pub fn is_sa1(&self) -> bool {
        *self == Fsel32::Sa1
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel32::Reserved2
    }
    #[doc = "Pin is connected to TXD0"]
    #[inline(always)]
    pub fn is_txd0(&self) -> bool {
        *self == Fsel32::Txd0
    }
    #[doc = "Pin is connected to SD_CARD_PRES"]
    #[inline(always)]
    pub fn is_sd_card_pres(&self) -> bool {
        *self == Fsel32::SdCardPres
    }
    #[doc = "Pin is connected to TXD1"]
    #[inline(always)]
    pub fn is_txd1(&self) -> bool {
        *self == Fsel32::Txd1
    }
}
#[doc = "Field `FSEL32` writer - Function Select 32"]
pub type Fsel32W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel32, crate::Safe>;
impl<'a, REG> Fsel32W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel32::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel32::Output)
    }
    #[doc = "Pin is connected to GPCLK0"]
    #[inline(always)]
    pub fn gpclk0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel32::Gpclk0)
    }
    #[doc = "Pin is connected to SA1"]
    #[inline(always)]
    pub fn sa1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel32::Sa1)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel32::Reserved2)
    }
    #[doc = "Pin is connected to TXD0"]
    #[inline(always)]
    pub fn txd0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel32::Txd0)
    }
    #[doc = "Pin is connected to SD_CARD_PRES"]
    #[inline(always)]
    pub fn sd_card_pres(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel32::SdCardPres)
    }
    #[doc = "Pin is connected to TXD1"]
    #[inline(always)]
    pub fn txd1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel32::Txd1)
    }
}
#[doc = "Function Select 33"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel33 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Alt function 0 reserved"]
    Reserved0 = 4,
    #[doc = "5: Pin is connected to SA0"]
    Sa0 = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Pin is connected to RXD0"]
    Rxd0 = 7,
    #[doc = "3: Pin is connected to SD_CARD_WRPROT"]
    SdCardWrprot = 3,
    #[doc = "2: Pin is connected to RXD1"]
    Rxd1 = 2,
}
impl From<Fsel33> for u8 {
    #[inline(always)]
    fn from(variant: Fsel33) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel33 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel33 {}
#[doc = "Field `FSEL33` reader - Function Select 33"]
pub type Fsel33R = crate::FieldReader<Fsel33>;
impl Fsel33R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel33 {
        match self.bits {
            0 => Fsel33::Input,
            1 => Fsel33::Output,
            4 => Fsel33::Reserved0,
            5 => Fsel33::Sa0,
            6 => Fsel33::Reserved2,
            7 => Fsel33::Rxd0,
            3 => Fsel33::SdCardWrprot,
            2 => Fsel33::Rxd1,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel33::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel33::Output
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == Fsel33::Reserved0
    }
    #[doc = "Pin is connected to SA0"]
    #[inline(always)]
    pub fn is_sa0(&self) -> bool {
        *self == Fsel33::Sa0
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel33::Reserved2
    }
    #[doc = "Pin is connected to RXD0"]
    #[inline(always)]
    pub fn is_rxd0(&self) -> bool {
        *self == Fsel33::Rxd0
    }
    #[doc = "Pin is connected to SD_CARD_WRPROT"]
    #[inline(always)]
    pub fn is_sd_card_wrprot(&self) -> bool {
        *self == Fsel33::SdCardWrprot
    }
    #[doc = "Pin is connected to RXD1"]
    #[inline(always)]
    pub fn is_rxd1(&self) -> bool {
        *self == Fsel33::Rxd1
    }
}
#[doc = "Field `FSEL33` writer - Function Select 33"]
pub type Fsel33W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel33, crate::Safe>;
impl<'a, REG> Fsel33W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel33::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel33::Output)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel33::Reserved0)
    }
    #[doc = "Pin is connected to SA0"]
    #[inline(always)]
    pub fn sa0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel33::Sa0)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel33::Reserved2)
    }
    #[doc = "Pin is connected to RXD0"]
    #[inline(always)]
    pub fn rxd0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel33::Rxd0)
    }
    #[doc = "Pin is connected to SD_CARD_WRPROT"]
    #[inline(always)]
    pub fn sd_card_wrprot(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel33::SdCardWrprot)
    }
    #[doc = "Pin is connected to RXD1"]
    #[inline(always)]
    pub fn rxd1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel33::Rxd1)
    }
}
#[doc = "Function Select 34"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel34 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Pin is connected to GPCLK0"]
    Gpclk0 = 4,
    #[doc = "5: Pin is connected to SOE_N"]
    SoeN = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Pin is connected to SD1_CLK"]
    Sd1Clk = 7,
    #[doc = "3: Pin is connected to SD_CARD_LED"]
    SdCardLed = 3,
    #[doc = "2: Pin is connected to RGMII_IRQ"]
    RgmiiIrq = 2,
}
impl From<Fsel34> for u8 {
    #[inline(always)]
    fn from(variant: Fsel34) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel34 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel34 {}
#[doc = "Field `FSEL34` reader - Function Select 34"]
pub type Fsel34R = crate::FieldReader<Fsel34>;
impl Fsel34R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel34 {
        match self.bits {
            0 => Fsel34::Input,
            1 => Fsel34::Output,
            4 => Fsel34::Gpclk0,
            5 => Fsel34::SoeN,
            6 => Fsel34::Reserved2,
            7 => Fsel34::Sd1Clk,
            3 => Fsel34::SdCardLed,
            2 => Fsel34::RgmiiIrq,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel34::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel34::Output
    }
    #[doc = "Pin is connected to GPCLK0"]
    #[inline(always)]
    pub fn is_gpclk0(&self) -> bool {
        *self == Fsel34::Gpclk0
    }
    #[doc = "Pin is connected to SOE_N"]
    #[inline(always)]
    pub fn is_soe_n(&self) -> bool {
        *self == Fsel34::SoeN
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel34::Reserved2
    }
    #[doc = "Pin is connected to SD1_CLK"]
    #[inline(always)]
    pub fn is_sd1_clk(&self) -> bool {
        *self == Fsel34::Sd1Clk
    }
    #[doc = "Pin is connected to SD_CARD_LED"]
    #[inline(always)]
    pub fn is_sd_card_led(&self) -> bool {
        *self == Fsel34::SdCardLed
    }
    #[doc = "Pin is connected to RGMII_IRQ"]
    #[inline(always)]
    pub fn is_rgmii_irq(&self) -> bool {
        *self == Fsel34::RgmiiIrq
    }
}
#[doc = "Field `FSEL34` writer - Function Select 34"]
pub type Fsel34W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel34, crate::Safe>;
impl<'a, REG> Fsel34W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel34::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel34::Output)
    }
    #[doc = "Pin is connected to GPCLK0"]
    #[inline(always)]
    pub fn gpclk0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel34::Gpclk0)
    }
    #[doc = "Pin is connected to SOE_N"]
    #[inline(always)]
    pub fn soe_n(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel34::SoeN)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel34::Reserved2)
    }
    #[doc = "Pin is connected to SD1_CLK"]
    #[inline(always)]
    pub fn sd1_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel34::Sd1Clk)
    }
    #[doc = "Pin is connected to SD_CARD_LED"]
    #[inline(always)]
    pub fn sd_card_led(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel34::SdCardLed)
    }
    #[doc = "Pin is connected to RGMII_IRQ"]
    #[inline(always)]
    pub fn rgmii_irq(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel34::RgmiiIrq)
    }
}
#[doc = "Function Select 35"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel35 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Pin is connected to SPI0_CE1_N"]
    Spi0Ce1N = 4,
    #[doc = "5: Pin is connected to SWE_N"]
    SweN = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Pin is connected to SD1_CMD"]
    Sd1Cmd = 7,
    #[doc = "3: Pin is connected to RGMII_START_STOP"]
    RgmiiStartStop = 3,
    #[doc = "2: Alt function 5 reserved"]
    Reserved5 = 2,
}
impl From<Fsel35> for u8 {
    #[inline(always)]
    fn from(variant: Fsel35) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel35 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel35 {}
#[doc = "Field `FSEL35` reader - Function Select 35"]
pub type Fsel35R = crate::FieldReader<Fsel35>;
impl Fsel35R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel35 {
        match self.bits {
            0 => Fsel35::Input,
            1 => Fsel35::Output,
            4 => Fsel35::Spi0Ce1N,
            5 => Fsel35::SweN,
            6 => Fsel35::Reserved2,
            7 => Fsel35::Sd1Cmd,
            3 => Fsel35::RgmiiStartStop,
            2 => Fsel35::Reserved5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel35::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel35::Output
    }
    #[doc = "Pin is connected to SPI0_CE1_N"]
    #[inline(always)]
    pub fn is_spi0_ce1_n(&self) -> bool {
        *self == Fsel35::Spi0Ce1N
    }
    #[doc = "Pin is connected to SWE_N"]
    #[inline(always)]
    pub fn is_swe_n(&self) -> bool {
        *self == Fsel35::SweN
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel35::Reserved2
    }
    #[doc = "Pin is connected to SD1_CMD"]
    #[inline(always)]
    pub fn is_sd1_cmd(&self) -> bool {
        *self == Fsel35::Sd1Cmd
    }
    #[doc = "Pin is connected to RGMII_START_STOP"]
    #[inline(always)]
    pub fn is_rgmii_start_stop(&self) -> bool {
        *self == Fsel35::RgmiiStartStop
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fsel35::Reserved5
    }
}
#[doc = "Field `FSEL35` writer - Function Select 35"]
pub type Fsel35W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel35, crate::Safe>;
impl<'a, REG> Fsel35W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel35::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel35::Output)
    }
    #[doc = "Pin is connected to SPI0_CE1_N"]
    #[inline(always)]
    pub fn spi0_ce1_n(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel35::Spi0Ce1N)
    }
    #[doc = "Pin is connected to SWE_N"]
    #[inline(always)]
    pub fn swe_n(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel35::SweN)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel35::Reserved2)
    }
    #[doc = "Pin is connected to SD1_CMD"]
    #[inline(always)]
    pub fn sd1_cmd(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel35::Sd1Cmd)
    }
    #[doc = "Pin is connected to RGMII_START_STOP"]
    #[inline(always)]
    pub fn rgmii_start_stop(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel35::RgmiiStartStop)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel35::Reserved5)
    }
}
#[doc = "Function Select 36"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel36 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Pin is connected to SPI0_CE0_N"]
    Spi0Ce0N = 4,
    #[doc = "5: Pin is connected to SD0"]
    Sd0 = 5,
    #[doc = "6: Pin is connected to TXD0"]
    Txd0 = 6,
    #[doc = "7: Pin is connected to SD1_DAT0"]
    Sd1Dat0 = 7,
    #[doc = "3: Pin is connected to RGMII_RX_OK"]
    RgmiiRxOk = 3,
    #[doc = "2: Pin is connected to MII_A_RX_ERR"]
    MiiARxErr = 2,
}
impl From<Fsel36> for u8 {
    #[inline(always)]
    fn from(variant: Fsel36) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel36 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel36 {}
#[doc = "Field `FSEL36` reader - Function Select 36"]
pub type Fsel36R = crate::FieldReader<Fsel36>;
impl Fsel36R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel36 {
        match self.bits {
            0 => Fsel36::Input,
            1 => Fsel36::Output,
            4 => Fsel36::Spi0Ce0N,
            5 => Fsel36::Sd0,
            6 => Fsel36::Txd0,
            7 => Fsel36::Sd1Dat0,
            3 => Fsel36::RgmiiRxOk,
            2 => Fsel36::MiiARxErr,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel36::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel36::Output
    }
    #[doc = "Pin is connected to SPI0_CE0_N"]
    #[inline(always)]
    pub fn is_spi0_ce0_n(&self) -> bool {
        *self == Fsel36::Spi0Ce0N
    }
    #[doc = "Pin is connected to SD0"]
    #[inline(always)]
    pub fn is_sd0(&self) -> bool {
        *self == Fsel36::Sd0
    }
    #[doc = "Pin is connected to TXD0"]
    #[inline(always)]
    pub fn is_txd0(&self) -> bool {
        *self == Fsel36::Txd0
    }
    #[doc = "Pin is connected to SD1_DAT0"]
    #[inline(always)]
    pub fn is_sd1_dat0(&self) -> bool {
        *self == Fsel36::Sd1Dat0
    }
    #[doc = "Pin is connected to RGMII_RX_OK"]
    #[inline(always)]
    pub fn is_rgmii_rx_ok(&self) -> bool {
        *self == Fsel36::RgmiiRxOk
    }
    #[doc = "Pin is connected to MII_A_RX_ERR"]
    #[inline(always)]
    pub fn is_mii_a_rx_err(&self) -> bool {
        *self == Fsel36::MiiARxErr
    }
}
#[doc = "Field `FSEL36` writer - Function Select 36"]
pub type Fsel36W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel36, crate::Safe>;
impl<'a, REG> Fsel36W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel36::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel36::Output)
    }
    #[doc = "Pin is connected to SPI0_CE0_N"]
    #[inline(always)]
    pub fn spi0_ce0_n(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel36::Spi0Ce0N)
    }
    #[doc = "Pin is connected to SD0"]
    #[inline(always)]
    pub fn sd0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel36::Sd0)
    }
    #[doc = "Pin is connected to TXD0"]
    #[inline(always)]
    pub fn txd0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel36::Txd0)
    }
    #[doc = "Pin is connected to SD1_DAT0"]
    #[inline(always)]
    pub fn sd1_dat0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel36::Sd1Dat0)
    }
    #[doc = "Pin is connected to RGMII_RX_OK"]
    #[inline(always)]
    pub fn rgmii_rx_ok(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel36::RgmiiRxOk)
    }
    #[doc = "Pin is connected to MII_A_RX_ERR"]
    #[inline(always)]
    pub fn mii_a_rx_err(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel36::MiiARxErr)
    }
}
#[doc = "Function Select 37"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel37 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Pin is connected to SPI0_MISO"]
    Spi0Miso = 4,
    #[doc = "5: Pin is connected to SD1"]
    Sd1 = 5,
    #[doc = "6: Pin is connected to RXD0"]
    Rxd0 = 6,
    #[doc = "7: Pin is connected to SD1_DAT1"]
    Sd1Dat1 = 7,
    #[doc = "3: Pin is connected to RGMII_MDIO"]
    RgmiiMdio = 3,
    #[doc = "2: Pin is connected to MII_A_TX_ERR"]
    MiiATxErr = 2,
}
impl From<Fsel37> for u8 {
    #[inline(always)]
    fn from(variant: Fsel37) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel37 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel37 {}
#[doc = "Field `FSEL37` reader - Function Select 37"]
pub type Fsel37R = crate::FieldReader<Fsel37>;
impl Fsel37R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel37 {
        match self.bits {
            0 => Fsel37::Input,
            1 => Fsel37::Output,
            4 => Fsel37::Spi0Miso,
            5 => Fsel37::Sd1,
            6 => Fsel37::Rxd0,
            7 => Fsel37::Sd1Dat1,
            3 => Fsel37::RgmiiMdio,
            2 => Fsel37::MiiATxErr,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel37::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel37::Output
    }
    #[doc = "Pin is connected to SPI0_MISO"]
    #[inline(always)]
    pub fn is_spi0_miso(&self) -> bool {
        *self == Fsel37::Spi0Miso
    }
    #[doc = "Pin is connected to SD1"]
    #[inline(always)]
    pub fn is_sd1(&self) -> bool {
        *self == Fsel37::Sd1
    }
    #[doc = "Pin is connected to RXD0"]
    #[inline(always)]
    pub fn is_rxd0(&self) -> bool {
        *self == Fsel37::Rxd0
    }
    #[doc = "Pin is connected to SD1_DAT1"]
    #[inline(always)]
    pub fn is_sd1_dat1(&self) -> bool {
        *self == Fsel37::Sd1Dat1
    }
    #[doc = "Pin is connected to RGMII_MDIO"]
    #[inline(always)]
    pub fn is_rgmii_mdio(&self) -> bool {
        *self == Fsel37::RgmiiMdio
    }
    #[doc = "Pin is connected to MII_A_TX_ERR"]
    #[inline(always)]
    pub fn is_mii_a_tx_err(&self) -> bool {
        *self == Fsel37::MiiATxErr
    }
}
#[doc = "Field `FSEL37` writer - Function Select 37"]
pub type Fsel37W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel37, crate::Safe>;
impl<'a, REG> Fsel37W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel37::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel37::Output)
    }
    #[doc = "Pin is connected to SPI0_MISO"]
    #[inline(always)]
    pub fn spi0_miso(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel37::Spi0Miso)
    }
    #[doc = "Pin is connected to SD1"]
    #[inline(always)]
    pub fn sd1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel37::Sd1)
    }
    #[doc = "Pin is connected to RXD0"]
    #[inline(always)]
    pub fn rxd0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel37::Rxd0)
    }
    #[doc = "Pin is connected to SD1_DAT1"]
    #[inline(always)]
    pub fn sd1_dat1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel37::Sd1Dat1)
    }
    #[doc = "Pin is connected to RGMII_MDIO"]
    #[inline(always)]
    pub fn rgmii_mdio(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel37::RgmiiMdio)
    }
    #[doc = "Pin is connected to MII_A_TX_ERR"]
    #[inline(always)]
    pub fn mii_a_tx_err(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel37::MiiATxErr)
    }
}
#[doc = "Function Select 38"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel38 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Pin is connected to SPI0_MOSI"]
    Spi0Mosi = 4,
    #[doc = "5: Pin is connected to SD2"]
    Sd2 = 5,
    #[doc = "6: Pin is connected to CTS0"]
    Cts0 = 6,
    #[doc = "7: Pin is connected to SD1_DAT2"]
    Sd1Dat2 = 7,
    #[doc = "3: Pin is connected to RGMII_MDC"]
    RgmiiMdc = 3,
    #[doc = "2: Pin is connected to MII_A_CRS"]
    MiiACrs = 2,
}
impl From<Fsel38> for u8 {
    #[inline(always)]
    fn from(variant: Fsel38) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel38 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel38 {}
#[doc = "Field `FSEL38` reader - Function Select 38"]
pub type Fsel38R = crate::FieldReader<Fsel38>;
impl Fsel38R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel38 {
        match self.bits {
            0 => Fsel38::Input,
            1 => Fsel38::Output,
            4 => Fsel38::Spi0Mosi,
            5 => Fsel38::Sd2,
            6 => Fsel38::Cts0,
            7 => Fsel38::Sd1Dat2,
            3 => Fsel38::RgmiiMdc,
            2 => Fsel38::MiiACrs,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel38::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel38::Output
    }
    #[doc = "Pin is connected to SPI0_MOSI"]
    #[inline(always)]
    pub fn is_spi0_mosi(&self) -> bool {
        *self == Fsel38::Spi0Mosi
    }
    #[doc = "Pin is connected to SD2"]
    #[inline(always)]
    pub fn is_sd2(&self) -> bool {
        *self == Fsel38::Sd2
    }
    #[doc = "Pin is connected to CTS0"]
    #[inline(always)]
    pub fn is_cts0(&self) -> bool {
        *self == Fsel38::Cts0
    }
    #[doc = "Pin is connected to SD1_DAT2"]
    #[inline(always)]
    pub fn is_sd1_dat2(&self) -> bool {
        *self == Fsel38::Sd1Dat2
    }
    #[doc = "Pin is connected to RGMII_MDC"]
    #[inline(always)]
    pub fn is_rgmii_mdc(&self) -> bool {
        *self == Fsel38::RgmiiMdc
    }
    #[doc = "Pin is connected to MII_A_CRS"]
    #[inline(always)]
    pub fn is_mii_a_crs(&self) -> bool {
        *self == Fsel38::MiiACrs
    }
}
#[doc = "Field `FSEL38` writer - Function Select 38"]
pub type Fsel38W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel38, crate::Safe>;
impl<'a, REG> Fsel38W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel38::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel38::Output)
    }
    #[doc = "Pin is connected to SPI0_MOSI"]
    #[inline(always)]
    pub fn spi0_mosi(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel38::Spi0Mosi)
    }
    #[doc = "Pin is connected to SD2"]
    #[inline(always)]
    pub fn sd2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel38::Sd2)
    }
    #[doc = "Pin is connected to CTS0"]
    #[inline(always)]
    pub fn cts0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel38::Cts0)
    }
    #[doc = "Pin is connected to SD1_DAT2"]
    #[inline(always)]
    pub fn sd1_dat2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel38::Sd1Dat2)
    }
    #[doc = "Pin is connected to RGMII_MDC"]
    #[inline(always)]
    pub fn rgmii_mdc(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel38::RgmiiMdc)
    }
    #[doc = "Pin is connected to MII_A_CRS"]
    #[inline(always)]
    pub fn mii_a_crs(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel38::MiiACrs)
    }
}
#[doc = "Function Select 39"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel39 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Pin is connected to SPI0_SCLK"]
    Spi0Sclk = 4,
    #[doc = "5: Pin is connected to SD3"]
    Sd3 = 5,
    #[doc = "6: Pin is connected to RTS0"]
    Rts0 = 6,
    #[doc = "7: Pin is connected to SD1_DAT3"]
    Sd1Dat3 = 7,
    #[doc = "3: Pin is connected to RGMII_IRQ"]
    RgmiiIrq = 3,
    #[doc = "2: Pin is connected to MII_A_COL"]
    MiiACol = 2,
}
impl From<Fsel39> for u8 {
    #[inline(always)]
    fn from(variant: Fsel39) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel39 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel39 {}
#[doc = "Field `FSEL39` reader - Function Select 39"]
pub type Fsel39R = crate::FieldReader<Fsel39>;
impl Fsel39R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel39 {
        match self.bits {
            0 => Fsel39::Input,
            1 => Fsel39::Output,
            4 => Fsel39::Spi0Sclk,
            5 => Fsel39::Sd3,
            6 => Fsel39::Rts0,
            7 => Fsel39::Sd1Dat3,
            3 => Fsel39::RgmiiIrq,
            2 => Fsel39::MiiACol,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel39::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel39::Output
    }
    #[doc = "Pin is connected to SPI0_SCLK"]
    #[inline(always)]
    pub fn is_spi0_sclk(&self) -> bool {
        *self == Fsel39::Spi0Sclk
    }
    #[doc = "Pin is connected to SD3"]
    #[inline(always)]
    pub fn is_sd3(&self) -> bool {
        *self == Fsel39::Sd3
    }
    #[doc = "Pin is connected to RTS0"]
    #[inline(always)]
    pub fn is_rts0(&self) -> bool {
        *self == Fsel39::Rts0
    }
    #[doc = "Pin is connected to SD1_DAT3"]
    #[inline(always)]
    pub fn is_sd1_dat3(&self) -> bool {
        *self == Fsel39::Sd1Dat3
    }
    #[doc = "Pin is connected to RGMII_IRQ"]
    #[inline(always)]
    pub fn is_rgmii_irq(&self) -> bool {
        *self == Fsel39::RgmiiIrq
    }
    #[doc = "Pin is connected to MII_A_COL"]
    #[inline(always)]
    pub fn is_mii_a_col(&self) -> bool {
        *self == Fsel39::MiiACol
    }
}
#[doc = "Field `FSEL39` writer - Function Select 39"]
pub type Fsel39W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel39, crate::Safe>;
impl<'a, REG> Fsel39W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel39::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel39::Output)
    }
    #[doc = "Pin is connected to SPI0_SCLK"]
    #[inline(always)]
    pub fn spi0_sclk(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel39::Spi0Sclk)
    }
    #[doc = "Pin is connected to SD3"]
    #[inline(always)]
    pub fn sd3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel39::Sd3)
    }
    #[doc = "Pin is connected to RTS0"]
    #[inline(always)]
    pub fn rts0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel39::Rts0)
    }
    #[doc = "Pin is connected to SD1_DAT3"]
    #[inline(always)]
    pub fn sd1_dat3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel39::Sd1Dat3)
    }
    #[doc = "Pin is connected to RGMII_IRQ"]
    #[inline(always)]
    pub fn rgmii_irq(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel39::RgmiiIrq)
    }
    #[doc = "Pin is connected to MII_A_COL"]
    #[inline(always)]
    pub fn mii_a_col(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel39::MiiACol)
    }
}
impl R {
    #[doc = "Bits 0:2 - Function Select 30"]
    #[inline(always)]
    pub fn fsel30(&self) -> Fsel30R {
        Fsel30R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Function Select 31"]
    #[inline(always)]
    pub fn fsel31(&self) -> Fsel31R {
        Fsel31R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Function Select 32"]
    #[inline(always)]
    pub fn fsel32(&self) -> Fsel32R {
        Fsel32R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Function Select 33"]
    #[inline(always)]
    pub fn fsel33(&self) -> Fsel33R {
        Fsel33R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Function Select 34"]
    #[inline(always)]
    pub fn fsel34(&self) -> Fsel34R {
        Fsel34R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Function Select 35"]
    #[inline(always)]
    pub fn fsel35(&self) -> Fsel35R {
        Fsel35R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Function Select 36"]
    #[inline(always)]
    pub fn fsel36(&self) -> Fsel36R {
        Fsel36R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Function Select 37"]
    #[inline(always)]
    pub fn fsel37(&self) -> Fsel37R {
        Fsel37R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Function Select 38"]
    #[inline(always)]
    pub fn fsel38(&self) -> Fsel38R {
        Fsel38R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - Function Select 39"]
    #[inline(always)]
    pub fn fsel39(&self) -> Fsel39R {
        Fsel39R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPFSEL3")
            .field("fsel30", &self.fsel30())
            .field("fsel31", &self.fsel31())
            .field("fsel32", &self.fsel32())
            .field("fsel33", &self.fsel33())
            .field("fsel34", &self.fsel34())
            .field("fsel35", &self.fsel35())
            .field("fsel36", &self.fsel36())
            .field("fsel37", &self.fsel37())
            .field("fsel38", &self.fsel38())
            .field("fsel39", &self.fsel39())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Function Select 30"]
    #[inline(always)]
    #[must_use]
    pub fn fsel30(&mut self) -> Fsel30W<Gpfsel3Spec> {
        Fsel30W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Function Select 31"]
    #[inline(always)]
    #[must_use]
    pub fn fsel31(&mut self) -> Fsel31W<Gpfsel3Spec> {
        Fsel31W::new(self, 3)
    }
    #[doc = "Bits 6:8 - Function Select 32"]
    #[inline(always)]
    #[must_use]
    pub fn fsel32(&mut self) -> Fsel32W<Gpfsel3Spec> {
        Fsel32W::new(self, 6)
    }
    #[doc = "Bits 9:11 - Function Select 33"]
    #[inline(always)]
    #[must_use]
    pub fn fsel33(&mut self) -> Fsel33W<Gpfsel3Spec> {
        Fsel33W::new(self, 9)
    }
    #[doc = "Bits 12:14 - Function Select 34"]
    #[inline(always)]
    #[must_use]
    pub fn fsel34(&mut self) -> Fsel34W<Gpfsel3Spec> {
        Fsel34W::new(self, 12)
    }
    #[doc = "Bits 15:17 - Function Select 35"]
    #[inline(always)]
    #[must_use]
    pub fn fsel35(&mut self) -> Fsel35W<Gpfsel3Spec> {
        Fsel35W::new(self, 15)
    }
    #[doc = "Bits 18:20 - Function Select 36"]
    #[inline(always)]
    #[must_use]
    pub fn fsel36(&mut self) -> Fsel36W<Gpfsel3Spec> {
        Fsel36W::new(self, 18)
    }
    #[doc = "Bits 21:23 - Function Select 37"]
    #[inline(always)]
    #[must_use]
    pub fn fsel37(&mut self) -> Fsel37W<Gpfsel3Spec> {
        Fsel37W::new(self, 21)
    }
    #[doc = "Bits 24:26 - Function Select 38"]
    #[inline(always)]
    #[must_use]
    pub fn fsel38(&mut self) -> Fsel38W<Gpfsel3Spec> {
        Fsel38W::new(self, 24)
    }
    #[doc = "Bits 27:29 - Function Select 39"]
    #[inline(always)]
    #[must_use]
    pub fn fsel39(&mut self) -> Fsel39W<Gpfsel3Spec> {
        Fsel39W::new(self, 27)
    }
}
#[doc = "GPIO Function Select 3\n\nYou can [`read`](crate::Reg::read) this register and get [`gpfsel3::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpfsel3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpfsel3Spec;
impl crate::RegisterSpec for Gpfsel3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpfsel3::R`](R) reader structure"]
impl crate::Readable for Gpfsel3Spec {}
#[doc = "`write(|w| ..)` method takes [`gpfsel3::W`](W) writer structure"]
impl crate::Writable for Gpfsel3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}

#[doc = "Register `GICD_ICFGR28` reader"]
pub type R = crate::R<GicdIcfgr28Spec>;
#[doc = "Register `GICD_ICFGR28` writer"]
pub type W = crate::W<GicdIcfgr28Spec>;
#[doc = "DMA 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma0 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Dma0> for bool {
    #[inline(always)]
    fn from(variant: Dma0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_0` reader - DMA 0"]
pub type Dma0R = crate::BitReader<Dma0>;
impl Dma0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma0 {
        match self.bits {
            false => Dma0::Level,
            true => Dma0::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Dma0::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Dma0::Edge
    }
}
#[doc = "Field `DMA_0` writer - DMA 0"]
pub type Dma0W<'a, REG> = crate::BitWriter<'a, REG, Dma0>;
impl<'a, REG> Dma0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0::Edge)
    }
}
#[doc = "DMA 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma1 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Dma1> for bool {
    #[inline(always)]
    fn from(variant: Dma1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_1` reader - DMA 1"]
pub type Dma1R = crate::BitReader<Dma1>;
impl Dma1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma1 {
        match self.bits {
            false => Dma1::Level,
            true => Dma1::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Dma1::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Dma1::Edge
    }
}
#[doc = "Field `DMA_1` writer - DMA 1"]
pub type Dma1W<'a, REG> = crate::BitWriter<'a, REG, Dma1>;
impl<'a, REG> Dma1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1::Edge)
    }
}
#[doc = "DMA 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma2 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Dma2> for bool {
    #[inline(always)]
    fn from(variant: Dma2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_2` reader - DMA 2"]
pub type Dma2R = crate::BitReader<Dma2>;
impl Dma2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma2 {
        match self.bits {
            false => Dma2::Level,
            true => Dma2::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Dma2::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Dma2::Edge
    }
}
#[doc = "Field `DMA_2` writer - DMA 2"]
pub type Dma2W<'a, REG> = crate::BitWriter<'a, REG, Dma2>;
impl<'a, REG> Dma2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2::Edge)
    }
}
#[doc = "DMA 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma3 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Dma3> for bool {
    #[inline(always)]
    fn from(variant: Dma3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_3` reader - DMA 3"]
pub type Dma3R = crate::BitReader<Dma3>;
impl Dma3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma3 {
        match self.bits {
            false => Dma3::Level,
            true => Dma3::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Dma3::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Dma3::Edge
    }
}
#[doc = "Field `DMA_3` writer - DMA 3"]
pub type Dma3W<'a, REG> = crate::BitWriter<'a, REG, Dma3>;
impl<'a, REG> Dma3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Dma3::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Dma3::Edge)
    }
}
#[doc = "DMA 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma4 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Dma4> for bool {
    #[inline(always)]
    fn from(variant: Dma4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_4` reader - DMA 4"]
pub type Dma4R = crate::BitReader<Dma4>;
impl Dma4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma4 {
        match self.bits {
            false => Dma4::Level,
            true => Dma4::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Dma4::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Dma4::Edge
    }
}
#[doc = "Field `DMA_4` writer - DMA 4"]
pub type Dma4W<'a, REG> = crate::BitWriter<'a, REG, Dma4>;
impl<'a, REG> Dma4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Dma4::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Dma4::Edge)
    }
}
#[doc = "DMA 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma5 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Dma5> for bool {
    #[inline(always)]
    fn from(variant: Dma5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_5` reader - DMA 5"]
pub type Dma5R = crate::BitReader<Dma5>;
impl Dma5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma5 {
        match self.bits {
            false => Dma5::Level,
            true => Dma5::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Dma5::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Dma5::Edge
    }
}
#[doc = "Field `DMA_5` writer - DMA 5"]
pub type Dma5W<'a, REG> = crate::BitWriter<'a, REG, Dma5>;
impl<'a, REG> Dma5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Dma5::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Dma5::Edge)
    }
}
#[doc = "DMA 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma6 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Dma6> for bool {
    #[inline(always)]
    fn from(variant: Dma6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_6` reader - DMA 6"]
pub type Dma6R = crate::BitReader<Dma6>;
impl Dma6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma6 {
        match self.bits {
            false => Dma6::Level,
            true => Dma6::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Dma6::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Dma6::Edge
    }
}
#[doc = "Field `DMA_6` writer - DMA 6"]
pub type Dma6W<'a, REG> = crate::BitWriter<'a, REG, Dma6>;
impl<'a, REG> Dma6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Dma6::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Dma6::Edge)
    }
}
#[doc = "OR of DMA 7 and 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma7_8 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Dma7_8> for bool {
    #[inline(always)]
    fn from(variant: Dma7_8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_7_8` reader - OR of DMA 7 and 8"]
pub type Dma7_8R = crate::BitReader<Dma7_8>;
impl Dma7_8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma7_8 {
        match self.bits {
            false => Dma7_8::Level,
            true => Dma7_8::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Dma7_8::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Dma7_8::Edge
    }
}
#[doc = "Field `DMA_7_8` writer - OR of DMA 7 and 8"]
pub type Dma7_8W<'a, REG> = crate::BitWriter<'a, REG, Dma7_8>;
impl<'a, REG> Dma7_8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Dma7_8::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Dma7_8::Edge)
    }
}
#[doc = "OR of DMA 9 and 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma9_10 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Dma9_10> for bool {
    #[inline(always)]
    fn from(variant: Dma9_10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_9_10` reader - OR of DMA 9 and 10"]
pub type Dma9_10R = crate::BitReader<Dma9_10>;
impl Dma9_10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma9_10 {
        match self.bits {
            false => Dma9_10::Level,
            true => Dma9_10::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Dma9_10::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Dma9_10::Edge
    }
}
#[doc = "Field `DMA_9_10` writer - OR of DMA 9 and 10"]
pub type Dma9_10W<'a, REG> = crate::BitWriter<'a, REG, Dma9_10>;
impl<'a, REG> Dma9_10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Dma9_10::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Dma9_10::Edge)
    }
}
#[doc = "DMA 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma11 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Dma11> for bool {
    #[inline(always)]
    fn from(variant: Dma11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_11` reader - DMA 11"]
pub type Dma11R = crate::BitReader<Dma11>;
impl Dma11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma11 {
        match self.bits {
            false => Dma11::Level,
            true => Dma11::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Dma11::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Dma11::Edge
    }
}
#[doc = "Field `DMA_11` writer - DMA 11"]
pub type Dma11W<'a, REG> = crate::BitWriter<'a, REG, Dma11>;
impl<'a, REG> Dma11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Dma11::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Dma11::Edge)
    }
}
#[doc = "DMA 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma12 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Dma12> for bool {
    #[inline(always)]
    fn from(variant: Dma12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_12` reader - DMA 12"]
pub type Dma12R = crate::BitReader<Dma12>;
impl Dma12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma12 {
        match self.bits {
            false => Dma12::Level,
            true => Dma12::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Dma12::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Dma12::Edge
    }
}
#[doc = "Field `DMA_12` writer - DMA 12"]
pub type Dma12W<'a, REG> = crate::BitWriter<'a, REG, Dma12>;
impl<'a, REG> Dma12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Dma12::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Dma12::Edge)
    }
}
#[doc = "DMA 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma13 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Dma13> for bool {
    #[inline(always)]
    fn from(variant: Dma13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_13` reader - DMA 13"]
pub type Dma13R = crate::BitReader<Dma13>;
impl Dma13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma13 {
        match self.bits {
            false => Dma13::Level,
            true => Dma13::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Dma13::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Dma13::Edge
    }
}
#[doc = "Field `DMA_13` writer - DMA 13"]
pub type Dma13W<'a, REG> = crate::BitWriter<'a, REG, Dma13>;
impl<'a, REG> Dma13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Dma13::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Dma13::Edge)
    }
}
#[doc = "DMA 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma14 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Dma14> for bool {
    #[inline(always)]
    fn from(variant: Dma14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_14` reader - DMA 14"]
pub type Dma14R = crate::BitReader<Dma14>;
impl Dma14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma14 {
        match self.bits {
            false => Dma14::Level,
            true => Dma14::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Dma14::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Dma14::Edge
    }
}
#[doc = "Field `DMA_14` writer - DMA 14"]
pub type Dma14W<'a, REG> = crate::BitWriter<'a, REG, Dma14>;
impl<'a, REG> Dma14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Dma14::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Dma14::Edge)
    }
}
#[doc = "OR of UART1, SPI1 and SPI2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aux {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Aux> for bool {
    #[inline(always)]
    fn from(variant: Aux) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX` reader - OR of UART1, SPI1 and SPI2"]
pub type AuxR = crate::BitReader<Aux>;
impl AuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aux {
        match self.bits {
            false => Aux::Level,
            true => Aux::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Aux::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Aux::Edge
    }
}
#[doc = "Field `AUX` writer - OR of UART1, SPI1 and SPI2"]
pub type AuxW<'a, REG> = crate::BitWriter<'a, REG, Aux>;
impl<'a, REG> AuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Aux::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Aux::Edge)
    }
}
#[doc = "ARM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Arm> for bool {
    #[inline(always)]
    fn from(variant: Arm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARM` reader - ARM"]
pub type ArmR = crate::BitReader<Arm>;
impl ArmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Arm {
        match self.bits {
            false => Arm::Level,
            true => Arm::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Arm::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Arm::Edge
    }
}
#[doc = "Field `ARM` writer - ARM"]
pub type ArmW<'a, REG> = crate::BitWriter<'a, REG, Arm>;
impl<'a, REG> ArmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Arm::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Arm::Edge)
    }
}
#[doc = "DMA 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma15 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Dma15> for bool {
    #[inline(always)]
    fn from(variant: Dma15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_15` reader - DMA 15"]
pub type Dma15R = crate::BitReader<Dma15>;
impl Dma15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma15 {
        match self.bits {
            false => Dma15::Level,
            true => Dma15::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Dma15::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Dma15::Edge
    }
}
#[doc = "Field `DMA_15` writer - DMA 15"]
pub type Dma15W<'a, REG> = crate::BitWriter<'a, REG, Dma15>;
impl<'a, REG> Dma15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Dma15::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Dma15::Edge)
    }
}
impl R {
    #[doc = "Bit 1 - DMA 0"]
    #[inline(always)]
    pub fn dma_0(&self) -> Dma0R {
        Dma0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA 1"]
    #[inline(always)]
    pub fn dma_1(&self) -> Dma1R {
        Dma1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA 2"]
    #[inline(always)]
    pub fn dma_2(&self) -> Dma2R {
        Dma2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA 3"]
    #[inline(always)]
    pub fn dma_3(&self) -> Dma3R {
        Dma3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - DMA 4"]
    #[inline(always)]
    pub fn dma_4(&self) -> Dma4R {
        Dma4R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - DMA 5"]
    #[inline(always)]
    pub fn dma_5(&self) -> Dma5R {
        Dma5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - DMA 6"]
    #[inline(always)]
    pub fn dma_6(&self) -> Dma6R {
        Dma6R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - OR of DMA 7 and 8"]
    #[inline(always)]
    pub fn dma_7_8(&self) -> Dma7_8R {
        Dma7_8R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - OR of DMA 9 and 10"]
    #[inline(always)]
    pub fn dma_9_10(&self) -> Dma9_10R {
        Dma9_10R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - DMA 11"]
    #[inline(always)]
    pub fn dma_11(&self) -> Dma11R {
        Dma11R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA 12"]
    #[inline(always)]
    pub fn dma_12(&self) -> Dma12R {
        Dma12R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - DMA 13"]
    #[inline(always)]
    pub fn dma_13(&self) -> Dma13R {
        Dma13R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - DMA 14"]
    #[inline(always)]
    pub fn dma_14(&self) -> Dma14R {
        Dma14R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - OR of UART1, SPI1 and SPI2"]
    #[inline(always)]
    pub fn aux(&self) -> AuxR {
        AuxR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - ARM"]
    #[inline(always)]
    pub fn arm(&self) -> ArmR {
        ArmR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - DMA 15"]
    #[inline(always)]
    pub fn dma_15(&self) -> Dma15R {
        Dma15R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ICFGR28")
            .field("dma_0", &self.dma_0())
            .field("dma_1", &self.dma_1())
            .field("dma_2", &self.dma_2())
            .field("dma_3", &self.dma_3())
            .field("dma_4", &self.dma_4())
            .field("dma_5", &self.dma_5())
            .field("dma_6", &self.dma_6())
            .field("dma_7_8", &self.dma_7_8())
            .field("dma_9_10", &self.dma_9_10())
            .field("dma_11", &self.dma_11())
            .field("dma_12", &self.dma_12())
            .field("dma_13", &self.dma_13())
            .field("dma_14", &self.dma_14())
            .field("aux", &self.aux())
            .field("arm", &self.arm())
            .field("dma_15", &self.dma_15())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - DMA 0"]
    #[inline(always)]
    #[must_use]
    pub fn dma_0(&mut self) -> Dma0W<GicdIcfgr28Spec> {
        Dma0W::new(self, 1)
    }
    #[doc = "Bit 3 - DMA 1"]
    #[inline(always)]
    #[must_use]
    pub fn dma_1(&mut self) -> Dma1W<GicdIcfgr28Spec> {
        Dma1W::new(self, 3)
    }
    #[doc = "Bit 5 - DMA 2"]
    #[inline(always)]
    #[must_use]
    pub fn dma_2(&mut self) -> Dma2W<GicdIcfgr28Spec> {
        Dma2W::new(self, 5)
    }
    #[doc = "Bit 7 - DMA 3"]
    #[inline(always)]
    #[must_use]
    pub fn dma_3(&mut self) -> Dma3W<GicdIcfgr28Spec> {
        Dma3W::new(self, 7)
    }
    #[doc = "Bit 9 - DMA 4"]
    #[inline(always)]
    #[must_use]
    pub fn dma_4(&mut self) -> Dma4W<GicdIcfgr28Spec> {
        Dma4W::new(self, 9)
    }
    #[doc = "Bit 11 - DMA 5"]
    #[inline(always)]
    #[must_use]
    pub fn dma_5(&mut self) -> Dma5W<GicdIcfgr28Spec> {
        Dma5W::new(self, 11)
    }
    #[doc = "Bit 13 - DMA 6"]
    #[inline(always)]
    #[must_use]
    pub fn dma_6(&mut self) -> Dma6W<GicdIcfgr28Spec> {
        Dma6W::new(self, 13)
    }
    #[doc = "Bit 15 - OR of DMA 7 and 8"]
    #[inline(always)]
    #[must_use]
    pub fn dma_7_8(&mut self) -> Dma7_8W<GicdIcfgr28Spec> {
        Dma7_8W::new(self, 15)
    }
    #[doc = "Bit 17 - OR of DMA 9 and 10"]
    #[inline(always)]
    #[must_use]
    pub fn dma_9_10(&mut self) -> Dma9_10W<GicdIcfgr28Spec> {
        Dma9_10W::new(self, 17)
    }
    #[doc = "Bit 19 - DMA 11"]
    #[inline(always)]
    #[must_use]
    pub fn dma_11(&mut self) -> Dma11W<GicdIcfgr28Spec> {
        Dma11W::new(self, 19)
    }
    #[doc = "Bit 21 - DMA 12"]
    #[inline(always)]
    #[must_use]
    pub fn dma_12(&mut self) -> Dma12W<GicdIcfgr28Spec> {
        Dma12W::new(self, 21)
    }
    #[doc = "Bit 23 - DMA 13"]
    #[inline(always)]
    #[must_use]
    pub fn dma_13(&mut self) -> Dma13W<GicdIcfgr28Spec> {
        Dma13W::new(self, 23)
    }
    #[doc = "Bit 25 - DMA 14"]
    #[inline(always)]
    #[must_use]
    pub fn dma_14(&mut self) -> Dma14W<GicdIcfgr28Spec> {
        Dma14W::new(self, 25)
    }
    #[doc = "Bit 27 - OR of UART1, SPI1 and SPI2"]
    #[inline(always)]
    #[must_use]
    pub fn aux(&mut self) -> AuxW<GicdIcfgr28Spec> {
        AuxW::new(self, 27)
    }
    #[doc = "Bit 29 - ARM"]
    #[inline(always)]
    #[must_use]
    pub fn arm(&mut self) -> ArmW<GicdIcfgr28Spec> {
        ArmW::new(self, 29)
    }
    #[doc = "Bit 31 - DMA 15"]
    #[inline(always)]
    #[must_use]
    pub fn dma_15(&mut self) -> Dma15W<GicdIcfgr28Spec> {
        Dma15W::new(self, 31)
    }
}
#[doc = "Interrupt Configuration 112 - 127\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icfgr28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icfgr28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIcfgr28Spec;
impl crate::RegisterSpec for GicdIcfgr28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_icfgr28::R`](R) reader structure"]
impl crate::Readable for GicdIcfgr28Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_icfgr28::W`](W) writer structure"]
impl crate::Writable for GicdIcfgr28Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ICFGR28 to value 0"]
impl crate::Resettable for GicdIcfgr28Spec {
    const RESET_VALUE: u32 = 0;
}

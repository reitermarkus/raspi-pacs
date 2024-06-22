#[doc = "Register `GICD_ICFGR36` reader"]
pub type R = crate::R<GicdIcfgr36Spec>;
#[doc = "Register `GICD_ICFGR36` writer"]
pub type W = crate::W<GicdIcfgr36Spec>;
#[doc = "SMI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smi {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Smi> for bool {
    #[inline(always)]
    fn from(variant: Smi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMI` reader - SMI"]
pub type SmiR = crate::BitReader<Smi>;
impl SmiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smi {
        match self.bits {
            false => Smi::Level,
            true => Smi::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Smi::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Smi::Edge
    }
}
#[doc = "Field `SMI` writer - SMI"]
pub type SmiW<'a, REG> = crate::BitWriter<'a, REG, Smi>;
impl<'a, REG> SmiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Smi::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Smi::Edge)
    }
}
#[doc = "GPIO 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio0 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Gpio0> for bool {
    #[inline(always)]
    fn from(variant: Gpio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_0` reader - GPIO 0"]
pub type Gpio0R = crate::BitReader<Gpio0>;
impl Gpio0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio0 {
        match self.bits {
            false => Gpio0::Level,
            true => Gpio0::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Gpio0::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Gpio0::Edge
    }
}
#[doc = "Field `GPIO_0` writer - GPIO 0"]
pub type Gpio0W<'a, REG> = crate::BitWriter<'a, REG, Gpio0>;
impl<'a, REG> Gpio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0::Edge)
    }
}
#[doc = "GPIO 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio1 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Gpio1> for bool {
    #[inline(always)]
    fn from(variant: Gpio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_1` reader - GPIO 1"]
pub type Gpio1R = crate::BitReader<Gpio1>;
impl Gpio1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio1 {
        match self.bits {
            false => Gpio1::Level,
            true => Gpio1::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Gpio1::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Gpio1::Edge
    }
}
#[doc = "Field `GPIO_1` writer - GPIO 1"]
pub type Gpio1W<'a, REG> = crate::BitWriter<'a, REG, Gpio1>;
impl<'a, REG> Gpio1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1::Edge)
    }
}
#[doc = "GPIO 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio2 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Gpio2> for bool {
    #[inline(always)]
    fn from(variant: Gpio2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_2` reader - GPIO 2"]
pub type Gpio2R = crate::BitReader<Gpio2>;
impl Gpio2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2 {
        match self.bits {
            false => Gpio2::Level,
            true => Gpio2::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Gpio2::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Gpio2::Edge
    }
}
#[doc = "Field `GPIO_2` writer - GPIO 2"]
pub type Gpio2W<'a, REG> = crate::BitWriter<'a, REG, Gpio2>;
impl<'a, REG> Gpio2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2::Edge)
    }
}
#[doc = "GPIO 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio3 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Gpio3> for bool {
    #[inline(always)]
    fn from(variant: Gpio3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_3` reader - GPIO 3"]
pub type Gpio3R = crate::BitReader<Gpio3>;
impl Gpio3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3 {
        match self.bits {
            false => Gpio3::Level,
            true => Gpio3::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Gpio3::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Gpio3::Edge
    }
}
#[doc = "Field `GPIO_3` writer - GPIO 3"]
pub type Gpio3W<'a, REG> = crate::BitWriter<'a, REG, Gpio3>;
impl<'a, REG> Gpio3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3::Edge)
    }
}
#[doc = "OR of all I2C\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<I2c> for bool {
    #[inline(always)]
    fn from(variant: I2c) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C` reader - OR of all I2C"]
pub type I2cR = crate::BitReader<I2c>;
impl I2cR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c {
        match self.bits {
            false => I2c::Level,
            true => I2c::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == I2c::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == I2c::Edge
    }
}
#[doc = "Field `I2C` writer - OR of all I2C"]
pub type I2cW<'a, REG> = crate::BitWriter<'a, REG, I2c>;
impl<'a, REG> I2cW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(I2c::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(I2c::Edge)
    }
}
#[doc = "OR of all SPI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spi {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Spi> for bool {
    #[inline(always)]
    fn from(variant: Spi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI` reader - OR of all SPI"]
pub type SpiR = crate::BitReader<Spi>;
impl SpiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spi {
        match self.bits {
            false => Spi::Level,
            true => Spi::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Spi::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Spi::Edge
    }
}
#[doc = "Field `SPI` writer - OR of all SPI"]
pub type SpiW<'a, REG> = crate::BitWriter<'a, REG, Spi>;
impl<'a, REG> SpiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Spi::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Spi::Edge)
    }
}
#[doc = "PCM/I2S\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PcmI2s {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<PcmI2s> for bool {
    #[inline(always)]
    fn from(variant: PcmI2s) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCM_I2S` reader - PCM/I2S"]
pub type PcmI2sR = crate::BitReader<PcmI2s>;
impl PcmI2sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PcmI2s {
        match self.bits {
            false => PcmI2s::Level,
            true => PcmI2s::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == PcmI2s::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == PcmI2s::Edge
    }
}
#[doc = "Field `PCM_I2S` writer - PCM/I2S"]
pub type PcmI2sW<'a, REG> = crate::BitWriter<'a, REG, PcmI2s>;
impl<'a, REG> PcmI2sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(PcmI2s::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(PcmI2s::Edge)
    }
}
#[doc = "SDHOST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdhost {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Sdhost> for bool {
    #[inline(always)]
    fn from(variant: Sdhost) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDHOST` reader - SDHOST"]
pub type SdhostR = crate::BitReader<Sdhost>;
impl SdhostR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdhost {
        match self.bits {
            false => Sdhost::Level,
            true => Sdhost::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Sdhost::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Sdhost::Edge
    }
}
#[doc = "Field `SDHOST` writer - SDHOST"]
pub type SdhostW<'a, REG> = crate::BitWriter<'a, REG, Sdhost>;
impl<'a, REG> SdhostW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Sdhost::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Sdhost::Edge)
    }
}
#[doc = "OR of all PL011 UARTs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Uart> for bool {
    #[inline(always)]
    fn from(variant: Uart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART` reader - OR of all PL011 UARTs"]
pub type UartR = crate::BitReader<Uart>;
impl UartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart {
        match self.bits {
            false => Uart::Level,
            true => Uart::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Uart::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Uart::Edge
    }
}
#[doc = "Field `UART` writer - OR of all PL011 UARTs"]
pub type UartW<'a, REG> = crate::BitWriter<'a, REG, Uart>;
impl<'a, REG> UartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Uart::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Uart::Edge)
    }
}
#[doc = "OR of all ETH_PCIe L2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EthPcie {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<EthPcie> for bool {
    #[inline(always)]
    fn from(variant: EthPcie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETH_PCIE` reader - OR of all ETH_PCIe L2"]
pub type EthPcieR = crate::BitReader<EthPcie>;
impl EthPcieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EthPcie {
        match self.bits {
            false => EthPcie::Level,
            true => EthPcie::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == EthPcie::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == EthPcie::Edge
    }
}
#[doc = "Field `ETH_PCIE` writer - OR of all ETH_PCIe L2"]
pub type EthPcieW<'a, REG> = crate::BitWriter<'a, REG, EthPcie>;
impl<'a, REG> EthPcieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(EthPcie::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(EthPcie::Edge)
    }
}
#[doc = "VEC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vec {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Vec> for bool {
    #[inline(always)]
    fn from(variant: Vec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VEC` reader - VEC"]
pub type VecR = crate::BitReader<Vec>;
impl VecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vec {
        match self.bits {
            false => Vec::Level,
            true => Vec::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Vec::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Vec::Edge
    }
}
#[doc = "Field `VEC` writer - VEC"]
pub type VecW<'a, REG> = crate::BitWriter<'a, REG, Vec>;
impl<'a, REG> VecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Vec::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Vec::Edge)
    }
}
#[doc = "CPG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpg {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Cpg> for bool {
    #[inline(always)]
    fn from(variant: Cpg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPG` reader - CPG"]
pub type CpgR = crate::BitReader<Cpg>;
impl CpgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpg {
        match self.bits {
            false => Cpg::Level,
            true => Cpg::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Cpg::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Cpg::Edge
    }
}
#[doc = "Field `CPG` writer - CPG"]
pub type CpgW<'a, REG> = crate::BitWriter<'a, REG, Cpg>;
impl<'a, REG> CpgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Cpg::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cpg::Edge)
    }
}
#[doc = "RNG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rng {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Rng> for bool {
    #[inline(always)]
    fn from(variant: Rng) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNG` reader - RNG"]
pub type RngR = crate::BitReader<Rng>;
impl RngR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rng {
        match self.bits {
            false => Rng::Level,
            true => Rng::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Rng::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Rng::Edge
    }
}
#[doc = "Field `RNG` writer - RNG"]
pub type RngW<'a, REG> = crate::BitWriter<'a, REG, Rng>;
impl<'a, REG> RngW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Rng::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Rng::Edge)
    }
}
#[doc = "OR of EMMC and EMMC2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Emmc {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Emmc> for bool {
    #[inline(always)]
    fn from(variant: Emmc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMMC` reader - OR of EMMC and EMMC2"]
pub type EmmcR = crate::BitReader<Emmc>;
impl EmmcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Emmc {
        match self.bits {
            false => Emmc::Level,
            true => Emmc::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Emmc::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Emmc::Edge
    }
}
#[doc = "Field `EMMC` writer - OR of EMMC and EMMC2"]
pub type EmmcW<'a, REG> = crate::BitWriter<'a, REG, Emmc>;
impl<'a, REG> EmmcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Emmc::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Emmc::Edge)
    }
}
#[doc = "ETH_PCIe secure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EthPcieSecure {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<EthPcieSecure> for bool {
    #[inline(always)]
    fn from(variant: EthPcieSecure) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETH_PCIE_SECURE` reader - ETH_PCIe secure"]
pub type EthPcieSecureR = crate::BitReader<EthPcieSecure>;
impl EthPcieSecureR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EthPcieSecure {
        match self.bits {
            false => EthPcieSecure::Level,
            true => EthPcieSecure::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == EthPcieSecure::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == EthPcieSecure::Edge
    }
}
#[doc = "Field `ETH_PCIE_SECURE` writer - ETH_PCIe secure"]
pub type EthPcieSecureW<'a, REG> = crate::BitWriter<'a, REG, EthPcieSecure>;
impl<'a, REG> EthPcieSecureW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(EthPcieSecure::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(EthPcieSecure::Edge)
    }
}
impl R {
    #[doc = "Bit 1 - SMI"]
    #[inline(always)]
    pub fn smi(&self) -> SmiR {
        SmiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO 0"]
    #[inline(always)]
    pub fn gpio_0(&self) -> Gpio0R {
        Gpio0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO 1"]
    #[inline(always)]
    pub fn gpio_1(&self) -> Gpio1R {
        Gpio1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO 2"]
    #[inline(always)]
    pub fn gpio_2(&self) -> Gpio2R {
        Gpio2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIO 3"]
    #[inline(always)]
    pub fn gpio_3(&self) -> Gpio3R {
        Gpio3R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - OR of all I2C"]
    #[inline(always)]
    pub fn i2c(&self) -> I2cR {
        I2cR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - OR of all SPI"]
    #[inline(always)]
    pub fn spi(&self) -> SpiR {
        SpiR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - PCM/I2S"]
    #[inline(always)]
    pub fn pcm_i2s(&self) -> PcmI2sR {
        PcmI2sR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - SDHOST"]
    #[inline(always)]
    pub fn sdhost(&self) -> SdhostR {
        SdhostR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - OR of all PL011 UARTs"]
    #[inline(always)]
    pub fn uart(&self) -> UartR {
        UartR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - OR of all ETH_PCIe L2"]
    #[inline(always)]
    pub fn eth_pcie(&self) -> EthPcieR {
        EthPcieR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - VEC"]
    #[inline(always)]
    pub fn vec(&self) -> VecR {
        VecR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - CPG"]
    #[inline(always)]
    pub fn cpg(&self) -> CpgR {
        CpgR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - RNG"]
    #[inline(always)]
    pub fn rng(&self) -> RngR {
        RngR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - OR of EMMC and EMMC2"]
    #[inline(always)]
    pub fn emmc(&self) -> EmmcR {
        EmmcR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - ETH_PCIe secure"]
    #[inline(always)]
    pub fn eth_pcie_secure(&self) -> EthPcieSecureR {
        EthPcieSecureR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ICFGR36")
            .field("smi", &self.smi())
            .field("gpio_0", &self.gpio_0())
            .field("gpio_1", &self.gpio_1())
            .field("gpio_2", &self.gpio_2())
            .field("gpio_3", &self.gpio_3())
            .field("i2c", &self.i2c())
            .field("spi", &self.spi())
            .field("pcm_i2s", &self.pcm_i2s())
            .field("sdhost", &self.sdhost())
            .field("uart", &self.uart())
            .field("eth_pcie", &self.eth_pcie())
            .field("vec", &self.vec())
            .field("cpg", &self.cpg())
            .field("rng", &self.rng())
            .field("emmc", &self.emmc())
            .field("eth_pcie_secure", &self.eth_pcie_secure())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - SMI"]
    #[inline(always)]
    #[must_use]
    pub fn smi(&mut self) -> SmiW<GicdIcfgr36Spec> {
        SmiW::new(self, 1)
    }
    #[doc = "Bit 3 - GPIO 0"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_0(&mut self) -> Gpio0W<GicdIcfgr36Spec> {
        Gpio0W::new(self, 3)
    }
    #[doc = "Bit 5 - GPIO 1"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_1(&mut self) -> Gpio1W<GicdIcfgr36Spec> {
        Gpio1W::new(self, 5)
    }
    #[doc = "Bit 7 - GPIO 2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_2(&mut self) -> Gpio2W<GicdIcfgr36Spec> {
        Gpio2W::new(self, 7)
    }
    #[doc = "Bit 9 - GPIO 3"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_3(&mut self) -> Gpio3W<GicdIcfgr36Spec> {
        Gpio3W::new(self, 9)
    }
    #[doc = "Bit 11 - OR of all I2C"]
    #[inline(always)]
    #[must_use]
    pub fn i2c(&mut self) -> I2cW<GicdIcfgr36Spec> {
        I2cW::new(self, 11)
    }
    #[doc = "Bit 13 - OR of all SPI"]
    #[inline(always)]
    #[must_use]
    pub fn spi(&mut self) -> SpiW<GicdIcfgr36Spec> {
        SpiW::new(self, 13)
    }
    #[doc = "Bit 15 - PCM/I2S"]
    #[inline(always)]
    #[must_use]
    pub fn pcm_i2s(&mut self) -> PcmI2sW<GicdIcfgr36Spec> {
        PcmI2sW::new(self, 15)
    }
    #[doc = "Bit 17 - SDHOST"]
    #[inline(always)]
    #[must_use]
    pub fn sdhost(&mut self) -> SdhostW<GicdIcfgr36Spec> {
        SdhostW::new(self, 17)
    }
    #[doc = "Bit 19 - OR of all PL011 UARTs"]
    #[inline(always)]
    #[must_use]
    pub fn uart(&mut self) -> UartW<GicdIcfgr36Spec> {
        UartW::new(self, 19)
    }
    #[doc = "Bit 21 - OR of all ETH_PCIe L2"]
    #[inline(always)]
    #[must_use]
    pub fn eth_pcie(&mut self) -> EthPcieW<GicdIcfgr36Spec> {
        EthPcieW::new(self, 21)
    }
    #[doc = "Bit 23 - VEC"]
    #[inline(always)]
    #[must_use]
    pub fn vec(&mut self) -> VecW<GicdIcfgr36Spec> {
        VecW::new(self, 23)
    }
    #[doc = "Bit 25 - CPG"]
    #[inline(always)]
    #[must_use]
    pub fn cpg(&mut self) -> CpgW<GicdIcfgr36Spec> {
        CpgW::new(self, 25)
    }
    #[doc = "Bit 27 - RNG"]
    #[inline(always)]
    #[must_use]
    pub fn rng(&mut self) -> RngW<GicdIcfgr36Spec> {
        RngW::new(self, 27)
    }
    #[doc = "Bit 29 - OR of EMMC and EMMC2"]
    #[inline(always)]
    #[must_use]
    pub fn emmc(&mut self) -> EmmcW<GicdIcfgr36Spec> {
        EmmcW::new(self, 29)
    }
    #[doc = "Bit 31 - ETH_PCIe secure"]
    #[inline(always)]
    #[must_use]
    pub fn eth_pcie_secure(&mut self) -> EthPcieSecureW<GicdIcfgr36Spec> {
        EthPcieSecureW::new(self, 31)
    }
}
#[doc = "Interrupt Configuration 144 - 159\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icfgr36::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icfgr36::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIcfgr36Spec;
impl crate::RegisterSpec for GicdIcfgr36Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_icfgr36::R`](R) reader structure"]
impl crate::Readable for GicdIcfgr36Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_icfgr36::W`](W) writer structure"]
impl crate::Writable for GicdIcfgr36Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ICFGR36 to value 0"]
impl crate::Resettable for GicdIcfgr36Spec {
    const RESET_VALUE: u32 = 0;
}

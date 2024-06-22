#[doc = "Register `GICD_ICFGR32` reader"]
pub type R = crate::R<GicdIcfgr32Spec>;
#[doc = "Register `GICD_ICFGR32` writer"]
pub type W = crate::W<GicdIcfgr32Spec>;
#[doc = "HDMI CEC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HdmiCec {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<HdmiCec> for bool {
    #[inline(always)]
    fn from(variant: HdmiCec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDMI_CEC` reader - HDMI CEC"]
pub type HdmiCecR = crate::BitReader<HdmiCec>;
impl HdmiCecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HdmiCec {
        match self.bits {
            false => HdmiCec::Level,
            true => HdmiCec::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == HdmiCec::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == HdmiCec::Edge
    }
}
#[doc = "Field `HDMI_CEC` writer - HDMI CEC"]
pub type HdmiCecW<'a, REG> = crate::BitWriter<'a, REG, HdmiCec>;
impl<'a, REG> HdmiCecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(HdmiCec::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(HdmiCec::Edge)
    }
}
#[doc = "HVS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hvs {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Hvs> for bool {
    #[inline(always)]
    fn from(variant: Hvs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HVS` reader - HVS"]
pub type HvsR = crate::BitReader<Hvs>;
impl HvsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hvs {
        match self.bits {
            false => Hvs::Level,
            true => Hvs::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Hvs::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Hvs::Edge
    }
}
#[doc = "Field `HVS` writer - HVS"]
pub type HvsW<'a, REG> = crate::BitWriter<'a, REG, Hvs>;
impl<'a, REG> HvsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Hvs::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Hvs::Edge)
    }
}
#[doc = "RPIVID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rpivid {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Rpivid> for bool {
    #[inline(always)]
    fn from(variant: Rpivid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPIVID` reader - RPIVID"]
pub type RpividR = crate::BitReader<Rpivid>;
impl RpividR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rpivid {
        match self.bits {
            false => Rpivid::Level,
            true => Rpivid::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Rpivid::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Rpivid::Edge
    }
}
#[doc = "Field `RPIVID` writer - RPIVID"]
pub type RpividW<'a, REG> = crate::BitWriter<'a, REG, Rpivid>;
impl<'a, REG> RpividW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Rpivid::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Rpivid::Edge)
    }
}
#[doc = "SDC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdc {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Sdc> for bool {
    #[inline(always)]
    fn from(variant: Sdc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDC` reader - SDC"]
pub type SdcR = crate::BitReader<Sdc>;
impl SdcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdc {
        match self.bits {
            false => Sdc::Level,
            true => Sdc::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Sdc::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Sdc::Edge
    }
}
#[doc = "Field `SDC` writer - SDC"]
pub type SdcW<'a, REG> = crate::BitWriter<'a, REG, Sdc>;
impl<'a, REG> SdcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Sdc::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Sdc::Edge)
    }
}
#[doc = "DSI 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsi0 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Dsi0> for bool {
    #[inline(always)]
    fn from(variant: Dsi0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSI_0` reader - DSI 0"]
pub type Dsi0R = crate::BitReader<Dsi0>;
impl Dsi0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dsi0 {
        match self.bits {
            false => Dsi0::Level,
            true => Dsi0::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Dsi0::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Dsi0::Edge
    }
}
#[doc = "Field `DSI_0` writer - DSI 0"]
pub type Dsi0W<'a, REG> = crate::BitWriter<'a, REG, Dsi0>;
impl<'a, REG> Dsi0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Dsi0::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Dsi0::Edge)
    }
}
#[doc = "Pixel Valve 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PixelValve2 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<PixelValve2> for bool {
    #[inline(always)]
    fn from(variant: PixelValve2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIXEL_VALVE_2` reader - Pixel Valve 2"]
pub type PixelValve2R = crate::BitReader<PixelValve2>;
impl PixelValve2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PixelValve2 {
        match self.bits {
            false => PixelValve2::Level,
            true => PixelValve2::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == PixelValve2::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == PixelValve2::Edge
    }
}
#[doc = "Field `PIXEL_VALVE_2` writer - Pixel Valve 2"]
pub type PixelValve2W<'a, REG> = crate::BitWriter<'a, REG, PixelValve2>;
impl<'a, REG> PixelValve2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(PixelValve2::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(PixelValve2::Edge)
    }
}
#[doc = "Camera 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Camera0 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Camera0> for bool {
    #[inline(always)]
    fn from(variant: Camera0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAMERA_0` reader - Camera 0"]
pub type Camera0R = crate::BitReader<Camera0>;
impl Camera0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Camera0 {
        match self.bits {
            false => Camera0::Level,
            true => Camera0::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Camera0::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Camera0::Edge
    }
}
#[doc = "Field `CAMERA_0` writer - Camera 0"]
pub type Camera0W<'a, REG> = crate::BitWriter<'a, REG, Camera0>;
impl<'a, REG> Camera0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Camera0::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Camera0::Edge)
    }
}
#[doc = "Camera 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Camera1 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Camera1> for bool {
    #[inline(always)]
    fn from(variant: Camera1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAMERA_1` reader - Camera 1"]
pub type Camera1R = crate::BitReader<Camera1>;
impl Camera1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Camera1 {
        match self.bits {
            false => Camera1::Level,
            true => Camera1::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Camera1::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Camera1::Edge
    }
}
#[doc = "Field `CAMERA_1` writer - Camera 1"]
pub type Camera1W<'a, REG> = crate::BitWriter<'a, REG, Camera1>;
impl<'a, REG> Camera1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Camera1::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Camera1::Edge)
    }
}
#[doc = "HDMI 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hdmi0 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Hdmi0> for bool {
    #[inline(always)]
    fn from(variant: Hdmi0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDMI_0` reader - HDMI 0"]
pub type Hdmi0R = crate::BitReader<Hdmi0>;
impl Hdmi0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hdmi0 {
        match self.bits {
            false => Hdmi0::Level,
            true => Hdmi0::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Hdmi0::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Hdmi0::Edge
    }
}
#[doc = "Field `HDMI_0` writer - HDMI 0"]
pub type Hdmi0W<'a, REG> = crate::BitWriter<'a, REG, Hdmi0>;
impl<'a, REG> Hdmi0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Hdmi0::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Hdmi0::Edge)
    }
}
#[doc = "HDMI 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hdmi1 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Hdmi1> for bool {
    #[inline(always)]
    fn from(variant: Hdmi1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDMI_1` reader - HDMI 1"]
pub type Hdmi1R = crate::BitReader<Hdmi1>;
impl Hdmi1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hdmi1 {
        match self.bits {
            false => Hdmi1::Level,
            true => Hdmi1::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Hdmi1::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Hdmi1::Edge
    }
}
#[doc = "Field `HDMI_1` writer - HDMI 1"]
pub type Hdmi1W<'a, REG> = crate::BitWriter<'a, REG, Hdmi1>;
impl<'a, REG> Hdmi1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Hdmi1::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Hdmi1::Edge)
    }
}
#[doc = "Pixel Valve 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PixelValve3 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<PixelValve3> for bool {
    #[inline(always)]
    fn from(variant: PixelValve3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIXEL_VALVE_3` reader - Pixel Valve 3"]
pub type PixelValve3R = crate::BitReader<PixelValve3>;
impl PixelValve3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PixelValve3 {
        match self.bits {
            false => PixelValve3::Level,
            true => PixelValve3::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == PixelValve3::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == PixelValve3::Edge
    }
}
#[doc = "Field `PIXEL_VALVE_3` writer - Pixel Valve 3"]
pub type PixelValve3W<'a, REG> = crate::BitWriter<'a, REG, PixelValve3>;
impl<'a, REG> PixelValve3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(PixelValve3::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(PixelValve3::Edge)
    }
}
#[doc = "SPI/BSC Slave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpiBscSlave {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<SpiBscSlave> for bool {
    #[inline(always)]
    fn from(variant: SpiBscSlave) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI_BSC_SLAVE` reader - SPI/BSC Slave"]
pub type SpiBscSlaveR = crate::BitReader<SpiBscSlave>;
impl SpiBscSlaveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SpiBscSlave {
        match self.bits {
            false => SpiBscSlave::Level,
            true => SpiBscSlave::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == SpiBscSlave::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == SpiBscSlave::Edge
    }
}
#[doc = "Field `SPI_BSC_SLAVE` writer - SPI/BSC Slave"]
pub type SpiBscSlaveW<'a, REG> = crate::BitWriter<'a, REG, SpiBscSlave>;
impl<'a, REG> SpiBscSlaveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(SpiBscSlave::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(SpiBscSlave::Edge)
    }
}
#[doc = "DSI 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsi1 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Dsi1> for bool {
    #[inline(always)]
    fn from(variant: Dsi1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSI_1` reader - DSI 1"]
pub type Dsi1R = crate::BitReader<Dsi1>;
impl Dsi1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dsi1 {
        match self.bits {
            false => Dsi1::Level,
            true => Dsi1::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Dsi1::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Dsi1::Edge
    }
}
#[doc = "Field `DSI_1` writer - DSI 1"]
pub type Dsi1W<'a, REG> = crate::BitWriter<'a, REG, Dsi1>;
impl<'a, REG> Dsi1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Dsi1::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Dsi1::Edge)
    }
}
#[doc = "Pixel Valve 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PixelValve0 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<PixelValve0> for bool {
    #[inline(always)]
    fn from(variant: PixelValve0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIXEL_VALVE_0` reader - Pixel Valve 0"]
pub type PixelValve0R = crate::BitReader<PixelValve0>;
impl PixelValve0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PixelValve0 {
        match self.bits {
            false => PixelValve0::Level,
            true => PixelValve0::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == PixelValve0::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == PixelValve0::Edge
    }
}
#[doc = "Field `PIXEL_VALVE_0` writer - Pixel Valve 0"]
pub type PixelValve0W<'a, REG> = crate::BitWriter<'a, REG, PixelValve0>;
impl<'a, REG> PixelValve0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(PixelValve0::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(PixelValve0::Edge)
    }
}
#[doc = "OR of Pixel Valve 1 and 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PixelValve1_2 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<PixelValve1_2> for bool {
    #[inline(always)]
    fn from(variant: PixelValve1_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIXEL_VALVE_1_2` reader - OR of Pixel Valve 1 and 2"]
pub type PixelValve1_2R = crate::BitReader<PixelValve1_2>;
impl PixelValve1_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PixelValve1_2 {
        match self.bits {
            false => PixelValve1_2::Level,
            true => PixelValve1_2::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == PixelValve1_2::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == PixelValve1_2::Edge
    }
}
#[doc = "Field `PIXEL_VALVE_1_2` writer - OR of Pixel Valve 1 and 2"]
pub type PixelValve1_2W<'a, REG> = crate::BitWriter<'a, REG, PixelValve1_2>;
impl<'a, REG> PixelValve1_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(PixelValve1_2::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(PixelValve1_2::Edge)
    }
}
#[doc = "CPR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpr {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Cpr> for bool {
    #[inline(always)]
    fn from(variant: Cpr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPR` reader - CPR"]
pub type CprR = crate::BitReader<Cpr>;
impl CprR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpr {
        match self.bits {
            false => Cpr::Level,
            true => Cpr::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Cpr::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Cpr::Edge
    }
}
#[doc = "Field `CPR` writer - CPR"]
pub type CprW<'a, REG> = crate::BitWriter<'a, REG, Cpr>;
impl<'a, REG> CprW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Cpr::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cpr::Edge)
    }
}
impl R {
    #[doc = "Bit 1 - HDMI CEC"]
    #[inline(always)]
    pub fn hdmi_cec(&self) -> HdmiCecR {
        HdmiCecR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - HVS"]
    #[inline(always)]
    pub fn hvs(&self) -> HvsR {
        HvsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - RPIVID"]
    #[inline(always)]
    pub fn rpivid(&self) -> RpividR {
        RpividR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - SDC"]
    #[inline(always)]
    pub fn sdc(&self) -> SdcR {
        SdcR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - DSI 0"]
    #[inline(always)]
    pub fn dsi_0(&self) -> Dsi0R {
        Dsi0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Pixel Valve 2"]
    #[inline(always)]
    pub fn pixel_valve_2(&self) -> PixelValve2R {
        PixelValve2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Camera 0"]
    #[inline(always)]
    pub fn camera_0(&self) -> Camera0R {
        Camera0R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Camera 1"]
    #[inline(always)]
    pub fn camera_1(&self) -> Camera1R {
        Camera1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - HDMI 0"]
    #[inline(always)]
    pub fn hdmi_0(&self) -> Hdmi0R {
        Hdmi0R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - HDMI 1"]
    #[inline(always)]
    pub fn hdmi_1(&self) -> Hdmi1R {
        Hdmi1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Pixel Valve 3"]
    #[inline(always)]
    pub fn pixel_valve_3(&self) -> PixelValve3R {
        PixelValve3R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - SPI/BSC Slave"]
    #[inline(always)]
    pub fn spi_bsc_slave(&self) -> SpiBscSlaveR {
        SpiBscSlaveR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - DSI 1"]
    #[inline(always)]
    pub fn dsi_1(&self) -> Dsi1R {
        Dsi1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Pixel Valve 0"]
    #[inline(always)]
    pub fn pixel_valve_0(&self) -> PixelValve0R {
        PixelValve0R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - OR of Pixel Valve 1 and 2"]
    #[inline(always)]
    pub fn pixel_valve_1_2(&self) -> PixelValve1_2R {
        PixelValve1_2R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - CPR"]
    #[inline(always)]
    pub fn cpr(&self) -> CprR {
        CprR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ICFGR32")
            .field("hdmi_cec", &self.hdmi_cec())
            .field("hvs", &self.hvs())
            .field("rpivid", &self.rpivid())
            .field("sdc", &self.sdc())
            .field("dsi_0", &self.dsi_0())
            .field("pixel_valve_2", &self.pixel_valve_2())
            .field("camera_0", &self.camera_0())
            .field("camera_1", &self.camera_1())
            .field("hdmi_0", &self.hdmi_0())
            .field("hdmi_1", &self.hdmi_1())
            .field("pixel_valve_3", &self.pixel_valve_3())
            .field("spi_bsc_slave", &self.spi_bsc_slave())
            .field("dsi_1", &self.dsi_1())
            .field("pixel_valve_0", &self.pixel_valve_0())
            .field("pixel_valve_1_2", &self.pixel_valve_1_2())
            .field("cpr", &self.cpr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - HDMI CEC"]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_cec(&mut self) -> HdmiCecW<GicdIcfgr32Spec> {
        HdmiCecW::new(self, 1)
    }
    #[doc = "Bit 3 - HVS"]
    #[inline(always)]
    #[must_use]
    pub fn hvs(&mut self) -> HvsW<GicdIcfgr32Spec> {
        HvsW::new(self, 3)
    }
    #[doc = "Bit 5 - RPIVID"]
    #[inline(always)]
    #[must_use]
    pub fn rpivid(&mut self) -> RpividW<GicdIcfgr32Spec> {
        RpividW::new(self, 5)
    }
    #[doc = "Bit 7 - SDC"]
    #[inline(always)]
    #[must_use]
    pub fn sdc(&mut self) -> SdcW<GicdIcfgr32Spec> {
        SdcW::new(self, 7)
    }
    #[doc = "Bit 9 - DSI 0"]
    #[inline(always)]
    #[must_use]
    pub fn dsi_0(&mut self) -> Dsi0W<GicdIcfgr32Spec> {
        Dsi0W::new(self, 9)
    }
    #[doc = "Bit 11 - Pixel Valve 2"]
    #[inline(always)]
    #[must_use]
    pub fn pixel_valve_2(&mut self) -> PixelValve2W<GicdIcfgr32Spec> {
        PixelValve2W::new(self, 11)
    }
    #[doc = "Bit 13 - Camera 0"]
    #[inline(always)]
    #[must_use]
    pub fn camera_0(&mut self) -> Camera0W<GicdIcfgr32Spec> {
        Camera0W::new(self, 13)
    }
    #[doc = "Bit 15 - Camera 1"]
    #[inline(always)]
    #[must_use]
    pub fn camera_1(&mut self) -> Camera1W<GicdIcfgr32Spec> {
        Camera1W::new(self, 15)
    }
    #[doc = "Bit 17 - HDMI 0"]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_0(&mut self) -> Hdmi0W<GicdIcfgr32Spec> {
        Hdmi0W::new(self, 17)
    }
    #[doc = "Bit 19 - HDMI 1"]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_1(&mut self) -> Hdmi1W<GicdIcfgr32Spec> {
        Hdmi1W::new(self, 19)
    }
    #[doc = "Bit 21 - Pixel Valve 3"]
    #[inline(always)]
    #[must_use]
    pub fn pixel_valve_3(&mut self) -> PixelValve3W<GicdIcfgr32Spec> {
        PixelValve3W::new(self, 21)
    }
    #[doc = "Bit 23 - SPI/BSC Slave"]
    #[inline(always)]
    #[must_use]
    pub fn spi_bsc_slave(&mut self) -> SpiBscSlaveW<GicdIcfgr32Spec> {
        SpiBscSlaveW::new(self, 23)
    }
    #[doc = "Bit 25 - DSI 1"]
    #[inline(always)]
    #[must_use]
    pub fn dsi_1(&mut self) -> Dsi1W<GicdIcfgr32Spec> {
        Dsi1W::new(self, 25)
    }
    #[doc = "Bit 27 - Pixel Valve 0"]
    #[inline(always)]
    #[must_use]
    pub fn pixel_valve_0(&mut self) -> PixelValve0W<GicdIcfgr32Spec> {
        PixelValve0W::new(self, 27)
    }
    #[doc = "Bit 29 - OR of Pixel Valve 1 and 2"]
    #[inline(always)]
    #[must_use]
    pub fn pixel_valve_1_2(&mut self) -> PixelValve1_2W<GicdIcfgr32Spec> {
        PixelValve1_2W::new(self, 29)
    }
    #[doc = "Bit 31 - CPR"]
    #[inline(always)]
    #[must_use]
    pub fn cpr(&mut self) -> CprW<GicdIcfgr32Spec> {
        CprW::new(self, 31)
    }
}
#[doc = "Interrupt Configuration 128 - 143\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icfgr32::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icfgr32::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIcfgr32Spec;
impl crate::RegisterSpec for GicdIcfgr32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_icfgr32::R`](R) reader structure"]
impl crate::Readable for GicdIcfgr32Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_icfgr32::W`](W) writer structure"]
impl crate::Writable for GicdIcfgr32Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ICFGR32 to value 0"]
impl crate::Resettable for GicdIcfgr32Spec {
    const RESET_VALUE: u32 = 0;
}

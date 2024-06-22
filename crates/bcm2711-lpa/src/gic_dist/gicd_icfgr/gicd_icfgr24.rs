#[doc = "Register `GICD_ICFGR24` reader"]
pub type R = crate::R<GicdIcfgr24Spec>;
#[doc = "Register `GICD_ICFGR24` writer"]
pub type W = crate::W<GicdIcfgr24Spec>;
#[doc = "Timer 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer0 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Timer0> for bool {
    #[inline(always)]
    fn from(variant: Timer0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER_0` reader - Timer 0"]
pub type Timer0R = crate::BitReader<Timer0>;
impl Timer0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer0 {
        match self.bits {
            false => Timer0::Level,
            true => Timer0::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Timer0::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Timer0::Edge
    }
}
#[doc = "Field `TIMER_0` writer - Timer 0"]
pub type Timer0W<'a, REG> = crate::BitWriter<'a, REG, Timer0>;
impl<'a, REG> Timer0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0::Edge)
    }
}
#[doc = "Timer 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer1 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Timer1> for bool {
    #[inline(always)]
    fn from(variant: Timer1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER_1` reader - Timer 1"]
pub type Timer1R = crate::BitReader<Timer1>;
impl Timer1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer1 {
        match self.bits {
            false => Timer1::Level,
            true => Timer1::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Timer1::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Timer1::Edge
    }
}
#[doc = "Field `TIMER_1` writer - Timer 1"]
pub type Timer1W<'a, REG> = crate::BitWriter<'a, REG, Timer1>;
impl<'a, REG> Timer1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Timer1::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Timer1::Edge)
    }
}
#[doc = "Timer 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer2 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Timer2> for bool {
    #[inline(always)]
    fn from(variant: Timer2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER_2` reader - Timer 2"]
pub type Timer2R = crate::BitReader<Timer2>;
impl Timer2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer2 {
        match self.bits {
            false => Timer2::Level,
            true => Timer2::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Timer2::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Timer2::Edge
    }
}
#[doc = "Field `TIMER_2` writer - Timer 2"]
pub type Timer2W<'a, REG> = crate::BitWriter<'a, REG, Timer2>;
impl<'a, REG> Timer2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Timer2::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Timer2::Edge)
    }
}
#[doc = "Timer 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer3 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Timer3> for bool {
    #[inline(always)]
    fn from(variant: Timer3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER_3` reader - Timer 3"]
pub type Timer3R = crate::BitReader<Timer3>;
impl Timer3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer3 {
        match self.bits {
            false => Timer3::Level,
            true => Timer3::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Timer3::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Timer3::Edge
    }
}
#[doc = "Field `TIMER_3` writer - Timer 3"]
pub type Timer3W<'a, REG> = crate::BitWriter<'a, REG, Timer3>;
impl<'a, REG> Timer3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Timer3::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Timer3::Edge)
    }
}
#[doc = "H264 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum H264_0 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<H264_0> for bool {
    #[inline(always)]
    fn from(variant: H264_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `H264_0` reader - H264 0"]
pub type H264_0R = crate::BitReader<H264_0>;
impl H264_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H264_0 {
        match self.bits {
            false => H264_0::Level,
            true => H264_0::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == H264_0::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == H264_0::Edge
    }
}
#[doc = "Field `H264_0` writer - H264 0"]
pub type H264_0W<'a, REG> = crate::BitWriter<'a, REG, H264_0>;
impl<'a, REG> H264_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(H264_0::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(H264_0::Edge)
    }
}
#[doc = "H264 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum H264_1 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<H264_1> for bool {
    #[inline(always)]
    fn from(variant: H264_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `H264_1` reader - H264 1"]
pub type H264_1R = crate::BitReader<H264_1>;
impl H264_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H264_1 {
        match self.bits {
            false => H264_1::Level,
            true => H264_1::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == H264_1::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == H264_1::Edge
    }
}
#[doc = "Field `H264_1` writer - H264 1"]
pub type H264_1W<'a, REG> = crate::BitWriter<'a, REG, H264_1>;
impl<'a, REG> H264_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(H264_1::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(H264_1::Edge)
    }
}
#[doc = "H264 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum H264_2 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<H264_2> for bool {
    #[inline(always)]
    fn from(variant: H264_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `H264_2` reader - H264 2"]
pub type H264_2R = crate::BitReader<H264_2>;
impl H264_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H264_2 {
        match self.bits {
            false => H264_2::Level,
            true => H264_2::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == H264_2::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == H264_2::Edge
    }
}
#[doc = "Field `H264_2` writer - H264 2"]
pub type H264_2W<'a, REG> = crate::BitWriter<'a, REG, H264_2>;
impl<'a, REG> H264_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(H264_2::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(H264_2::Edge)
    }
}
#[doc = "JPEG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Jpeg {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Jpeg> for bool {
    #[inline(always)]
    fn from(variant: Jpeg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JPEG` reader - JPEG"]
pub type JpegR = crate::BitReader<Jpeg>;
impl JpegR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Jpeg {
        match self.bits {
            false => Jpeg::Level,
            true => Jpeg::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Jpeg::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Jpeg::Edge
    }
}
#[doc = "Field `JPEG` writer - JPEG"]
pub type JpegW<'a, REG> = crate::BitWriter<'a, REG, Jpeg>;
impl<'a, REG> JpegW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Jpeg::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Jpeg::Edge)
    }
}
#[doc = "ISP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isp {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Isp> for bool {
    #[inline(always)]
    fn from(variant: Isp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISP` reader - ISP"]
pub type IspR = crate::BitReader<Isp>;
impl IspR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isp {
        match self.bits {
            false => Isp::Level,
            true => Isp::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Isp::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Isp::Edge
    }
}
#[doc = "Field `ISP` writer - ISP"]
pub type IspW<'a, REG> = crate::BitWriter<'a, REG, Isp>;
impl<'a, REG> IspW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Isp::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Isp::Edge)
    }
}
#[doc = "USB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Usb> for bool {
    #[inline(always)]
    fn from(variant: Usb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB` reader - USB"]
pub type UsbR = crate::BitReader<Usb>;
impl UsbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usb {
        match self.bits {
            false => Usb::Level,
            true => Usb::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Usb::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Usb::Edge
    }
}
#[doc = "Field `USB` writer - USB"]
pub type UsbW<'a, REG> = crate::BitWriter<'a, REG, Usb>;
impl<'a, REG> UsbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Usb::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Usb::Edge)
    }
}
#[doc = "V3D\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum V3d {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<V3d> for bool {
    #[inline(always)]
    fn from(variant: V3d) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `V3D` reader - V3D"]
pub type V3dR = crate::BitReader<V3d>;
impl V3dR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> V3d {
        match self.bits {
            false => V3d::Level,
            true => V3d::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == V3d::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == V3d::Edge
    }
}
#[doc = "Field `V3D` writer - V3D"]
pub type V3dW<'a, REG> = crate::BitWriter<'a, REG, V3d>;
impl<'a, REG> V3dW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(V3d::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(V3d::Edge)
    }
}
#[doc = "Transposer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Transposer {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Transposer> for bool {
    #[inline(always)]
    fn from(variant: Transposer) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRANSPOSER` reader - Transposer"]
pub type TransposerR = crate::BitReader<Transposer>;
impl TransposerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Transposer {
        match self.bits {
            false => Transposer::Level,
            true => Transposer::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Transposer::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Transposer::Edge
    }
}
#[doc = "Field `TRANSPOSER` writer - Transposer"]
pub type TransposerW<'a, REG> = crate::BitWriter<'a, REG, Transposer>;
impl<'a, REG> TransposerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Transposer::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Transposer::Edge)
    }
}
#[doc = "Multicore Sync 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MulticoreSync0 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<MulticoreSync0> for bool {
    #[inline(always)]
    fn from(variant: MulticoreSync0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MULTICORE_SYNC_0` reader - Multicore Sync 0"]
pub type MulticoreSync0R = crate::BitReader<MulticoreSync0>;
impl MulticoreSync0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MulticoreSync0 {
        match self.bits {
            false => MulticoreSync0::Level,
            true => MulticoreSync0::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == MulticoreSync0::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == MulticoreSync0::Edge
    }
}
#[doc = "Field `MULTICORE_SYNC_0` writer - Multicore Sync 0"]
pub type MulticoreSync0W<'a, REG> = crate::BitWriter<'a, REG, MulticoreSync0>;
impl<'a, REG> MulticoreSync0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(MulticoreSync0::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(MulticoreSync0::Edge)
    }
}
#[doc = "Multicore Sync 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MulticoreSync1 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<MulticoreSync1> for bool {
    #[inline(always)]
    fn from(variant: MulticoreSync1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MULTICORE_SYNC_1` reader - Multicore Sync 1"]
pub type MulticoreSync1R = crate::BitReader<MulticoreSync1>;
impl MulticoreSync1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MulticoreSync1 {
        match self.bits {
            false => MulticoreSync1::Level,
            true => MulticoreSync1::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == MulticoreSync1::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == MulticoreSync1::Edge
    }
}
#[doc = "Field `MULTICORE_SYNC_1` writer - Multicore Sync 1"]
pub type MulticoreSync1W<'a, REG> = crate::BitWriter<'a, REG, MulticoreSync1>;
impl<'a, REG> MulticoreSync1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(MulticoreSync1::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(MulticoreSync1::Edge)
    }
}
#[doc = "Multicore Sync 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MulticoreSync2 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<MulticoreSync2> for bool {
    #[inline(always)]
    fn from(variant: MulticoreSync2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MULTICORE_SYNC_2` reader - Multicore Sync 2"]
pub type MulticoreSync2R = crate::BitReader<MulticoreSync2>;
impl MulticoreSync2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MulticoreSync2 {
        match self.bits {
            false => MulticoreSync2::Level,
            true => MulticoreSync2::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == MulticoreSync2::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == MulticoreSync2::Edge
    }
}
#[doc = "Field `MULTICORE_SYNC_2` writer - Multicore Sync 2"]
pub type MulticoreSync2W<'a, REG> = crate::BitWriter<'a, REG, MulticoreSync2>;
impl<'a, REG> MulticoreSync2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(MulticoreSync2::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(MulticoreSync2::Edge)
    }
}
#[doc = "Multicore Sync 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MulticoreSync3 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<MulticoreSync3> for bool {
    #[inline(always)]
    fn from(variant: MulticoreSync3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MULTICORE_SYNC_3` reader - Multicore Sync 3"]
pub type MulticoreSync3R = crate::BitReader<MulticoreSync3>;
impl MulticoreSync3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MulticoreSync3 {
        match self.bits {
            false => MulticoreSync3::Level,
            true => MulticoreSync3::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == MulticoreSync3::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == MulticoreSync3::Edge
    }
}
#[doc = "Field `MULTICORE_SYNC_3` writer - Multicore Sync 3"]
pub type MulticoreSync3W<'a, REG> = crate::BitWriter<'a, REG, MulticoreSync3>;
impl<'a, REG> MulticoreSync3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(MulticoreSync3::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(MulticoreSync3::Edge)
    }
}
impl R {
    #[doc = "Bit 1 - Timer 0"]
    #[inline(always)]
    pub fn timer_0(&self) -> Timer0R {
        Timer0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer 1"]
    #[inline(always)]
    pub fn timer_1(&self) -> Timer1R {
        Timer1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer 2"]
    #[inline(always)]
    pub fn timer_2(&self) -> Timer2R {
        Timer2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer 3"]
    #[inline(always)]
    pub fn timer_3(&self) -> Timer3R {
        Timer3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - H264 0"]
    #[inline(always)]
    pub fn h264_0(&self) -> H264_0R {
        H264_0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - H264 1"]
    #[inline(always)]
    pub fn h264_1(&self) -> H264_1R {
        H264_1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - H264 2"]
    #[inline(always)]
    pub fn h264_2(&self) -> H264_2R {
        H264_2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - JPEG"]
    #[inline(always)]
    pub fn jpeg(&self) -> JpegR {
        JpegR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - ISP"]
    #[inline(always)]
    pub fn isp(&self) -> IspR {
        IspR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - USB"]
    #[inline(always)]
    pub fn usb(&self) -> UsbR {
        UsbR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - V3D"]
    #[inline(always)]
    pub fn v3d(&self) -> V3dR {
        V3dR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Transposer"]
    #[inline(always)]
    pub fn transposer(&self) -> TransposerR {
        TransposerR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Multicore Sync 0"]
    #[inline(always)]
    pub fn multicore_sync_0(&self) -> MulticoreSync0R {
        MulticoreSync0R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Multicore Sync 1"]
    #[inline(always)]
    pub fn multicore_sync_1(&self) -> MulticoreSync1R {
        MulticoreSync1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - Multicore Sync 2"]
    #[inline(always)]
    pub fn multicore_sync_2(&self) -> MulticoreSync2R {
        MulticoreSync2R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Multicore Sync 3"]
    #[inline(always)]
    pub fn multicore_sync_3(&self) -> MulticoreSync3R {
        MulticoreSync3R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ICFGR24")
            .field("timer_0", &self.timer_0())
            .field("timer_1", &self.timer_1())
            .field("timer_2", &self.timer_2())
            .field("timer_3", &self.timer_3())
            .field("h264_0", &self.h264_0())
            .field("h264_1", &self.h264_1())
            .field("h264_2", &self.h264_2())
            .field("jpeg", &self.jpeg())
            .field("isp", &self.isp())
            .field("usb", &self.usb())
            .field("v3d", &self.v3d())
            .field("transposer", &self.transposer())
            .field("multicore_sync_0", &self.multicore_sync_0())
            .field("multicore_sync_1", &self.multicore_sync_1())
            .field("multicore_sync_2", &self.multicore_sync_2())
            .field("multicore_sync_3", &self.multicore_sync_3())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Timer 0"]
    #[inline(always)]
    #[must_use]
    pub fn timer_0(&mut self) -> Timer0W<GicdIcfgr24Spec> {
        Timer0W::new(self, 1)
    }
    #[doc = "Bit 3 - Timer 1"]
    #[inline(always)]
    #[must_use]
    pub fn timer_1(&mut self) -> Timer1W<GicdIcfgr24Spec> {
        Timer1W::new(self, 3)
    }
    #[doc = "Bit 5 - Timer 2"]
    #[inline(always)]
    #[must_use]
    pub fn timer_2(&mut self) -> Timer2W<GicdIcfgr24Spec> {
        Timer2W::new(self, 5)
    }
    #[doc = "Bit 7 - Timer 3"]
    #[inline(always)]
    #[must_use]
    pub fn timer_3(&mut self) -> Timer3W<GicdIcfgr24Spec> {
        Timer3W::new(self, 7)
    }
    #[doc = "Bit 9 - H264 0"]
    #[inline(always)]
    #[must_use]
    pub fn h264_0(&mut self) -> H264_0W<GicdIcfgr24Spec> {
        H264_0W::new(self, 9)
    }
    #[doc = "Bit 11 - H264 1"]
    #[inline(always)]
    #[must_use]
    pub fn h264_1(&mut self) -> H264_1W<GicdIcfgr24Spec> {
        H264_1W::new(self, 11)
    }
    #[doc = "Bit 13 - H264 2"]
    #[inline(always)]
    #[must_use]
    pub fn h264_2(&mut self) -> H264_2W<GicdIcfgr24Spec> {
        H264_2W::new(self, 13)
    }
    #[doc = "Bit 15 - JPEG"]
    #[inline(always)]
    #[must_use]
    pub fn jpeg(&mut self) -> JpegW<GicdIcfgr24Spec> {
        JpegW::new(self, 15)
    }
    #[doc = "Bit 17 - ISP"]
    #[inline(always)]
    #[must_use]
    pub fn isp(&mut self) -> IspW<GicdIcfgr24Spec> {
        IspW::new(self, 17)
    }
    #[doc = "Bit 19 - USB"]
    #[inline(always)]
    #[must_use]
    pub fn usb(&mut self) -> UsbW<GicdIcfgr24Spec> {
        UsbW::new(self, 19)
    }
    #[doc = "Bit 21 - V3D"]
    #[inline(always)]
    #[must_use]
    pub fn v3d(&mut self) -> V3dW<GicdIcfgr24Spec> {
        V3dW::new(self, 21)
    }
    #[doc = "Bit 23 - Transposer"]
    #[inline(always)]
    #[must_use]
    pub fn transposer(&mut self) -> TransposerW<GicdIcfgr24Spec> {
        TransposerW::new(self, 23)
    }
    #[doc = "Bit 25 - Multicore Sync 0"]
    #[inline(always)]
    #[must_use]
    pub fn multicore_sync_0(&mut self) -> MulticoreSync0W<GicdIcfgr24Spec> {
        MulticoreSync0W::new(self, 25)
    }
    #[doc = "Bit 27 - Multicore Sync 1"]
    #[inline(always)]
    #[must_use]
    pub fn multicore_sync_1(&mut self) -> MulticoreSync1W<GicdIcfgr24Spec> {
        MulticoreSync1W::new(self, 27)
    }
    #[doc = "Bit 29 - Multicore Sync 2"]
    #[inline(always)]
    #[must_use]
    pub fn multicore_sync_2(&mut self) -> MulticoreSync2W<GicdIcfgr24Spec> {
        MulticoreSync2W::new(self, 29)
    }
    #[doc = "Bit 31 - Multicore Sync 3"]
    #[inline(always)]
    #[must_use]
    pub fn multicore_sync_3(&mut self) -> MulticoreSync3W<GicdIcfgr24Spec> {
        MulticoreSync3W::new(self, 31)
    }
}
#[doc = "Interrupt Configuration 96 - 111\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icfgr24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icfgr24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIcfgr24Spec;
impl crate::RegisterSpec for GicdIcfgr24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_icfgr24::R`](R) reader structure"]
impl crate::Readable for GicdIcfgr24Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_icfgr24::W`](W) writer structure"]
impl crate::Writable for GicdIcfgr24Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ICFGR24 to value 0"]
impl crate::Resettable for GicdIcfgr24Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `GICD_ICFGR52` reader"]
pub type R = crate::R<GicdIcfgr52Spec>;
#[doc = "Register `GICD_ICFGR52` writer"]
pub type W = crate::W<GicdIcfgr52Spec>;
#[doc = "Interrupt 208\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int208 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int208> for bool {
    #[inline(always)]
    fn from(variant: Int208) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT208` reader - Interrupt 208"]
pub type Int208R = crate::BitReader<Int208>;
impl Int208R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int208 {
        match self.bits {
            false => Int208::Level,
            true => Int208::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int208::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int208::Edge
    }
}
#[doc = "Field `INT208` writer - Interrupt 208"]
pub type Int208W<'a, REG> = crate::BitWriter<'a, REG, Int208>;
impl<'a, REG> Int208W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int208::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int208::Edge)
    }
}
#[doc = "Interrupt 209\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int209 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int209> for bool {
    #[inline(always)]
    fn from(variant: Int209) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT209` reader - Interrupt 209"]
pub type Int209R = crate::BitReader<Int209>;
impl Int209R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int209 {
        match self.bits {
            false => Int209::Level,
            true => Int209::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int209::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int209::Edge
    }
}
#[doc = "Field `INT209` writer - Interrupt 209"]
pub type Int209W<'a, REG> = crate::BitWriter<'a, REG, Int209>;
impl<'a, REG> Int209W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int209::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int209::Edge)
    }
}
#[doc = "Interrupt 210\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int210 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int210> for bool {
    #[inline(always)]
    fn from(variant: Int210) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT210` reader - Interrupt 210"]
pub type Int210R = crate::BitReader<Int210>;
impl Int210R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int210 {
        match self.bits {
            false => Int210::Level,
            true => Int210::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int210::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int210::Edge
    }
}
#[doc = "Field `INT210` writer - Interrupt 210"]
pub type Int210W<'a, REG> = crate::BitWriter<'a, REG, Int210>;
impl<'a, REG> Int210W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int210::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int210::Edge)
    }
}
#[doc = "Interrupt 211\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int211 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int211> for bool {
    #[inline(always)]
    fn from(variant: Int211) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT211` reader - Interrupt 211"]
pub type Int211R = crate::BitReader<Int211>;
impl Int211R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int211 {
        match self.bits {
            false => Int211::Level,
            true => Int211::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int211::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int211::Edge
    }
}
#[doc = "Field `INT211` writer - Interrupt 211"]
pub type Int211W<'a, REG> = crate::BitWriter<'a, REG, Int211>;
impl<'a, REG> Int211W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int211::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int211::Edge)
    }
}
#[doc = "Interrupt 212\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int212 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int212> for bool {
    #[inline(always)]
    fn from(variant: Int212) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT212` reader - Interrupt 212"]
pub type Int212R = crate::BitReader<Int212>;
impl Int212R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int212 {
        match self.bits {
            false => Int212::Level,
            true => Int212::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int212::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int212::Edge
    }
}
#[doc = "Field `INT212` writer - Interrupt 212"]
pub type Int212W<'a, REG> = crate::BitWriter<'a, REG, Int212>;
impl<'a, REG> Int212W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int212::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int212::Edge)
    }
}
#[doc = "Interrupt 213\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int213 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int213> for bool {
    #[inline(always)]
    fn from(variant: Int213) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT213` reader - Interrupt 213"]
pub type Int213R = crate::BitReader<Int213>;
impl Int213R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int213 {
        match self.bits {
            false => Int213::Level,
            true => Int213::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int213::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int213::Edge
    }
}
#[doc = "Field `INT213` writer - Interrupt 213"]
pub type Int213W<'a, REG> = crate::BitWriter<'a, REG, Int213>;
impl<'a, REG> Int213W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int213::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int213::Edge)
    }
}
#[doc = "Interrupt 214\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int214 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int214> for bool {
    #[inline(always)]
    fn from(variant: Int214) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT214` reader - Interrupt 214"]
pub type Int214R = crate::BitReader<Int214>;
impl Int214R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int214 {
        match self.bits {
            false => Int214::Level,
            true => Int214::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int214::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int214::Edge
    }
}
#[doc = "Field `INT214` writer - Interrupt 214"]
pub type Int214W<'a, REG> = crate::BitWriter<'a, REG, Int214>;
impl<'a, REG> Int214W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int214::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int214::Edge)
    }
}
#[doc = "Interrupt 215\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int215 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int215> for bool {
    #[inline(always)]
    fn from(variant: Int215) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT215` reader - Interrupt 215"]
pub type Int215R = crate::BitReader<Int215>;
impl Int215R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int215 {
        match self.bits {
            false => Int215::Level,
            true => Int215::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int215::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int215::Edge
    }
}
#[doc = "Field `INT215` writer - Interrupt 215"]
pub type Int215W<'a, REG> = crate::BitWriter<'a, REG, Int215>;
impl<'a, REG> Int215W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int215::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int215::Edge)
    }
}
#[doc = "Interrupt 216\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int216 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int216> for bool {
    #[inline(always)]
    fn from(variant: Int216) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT216` reader - Interrupt 216"]
pub type Int216R = crate::BitReader<Int216>;
impl Int216R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int216 {
        match self.bits {
            false => Int216::Level,
            true => Int216::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int216::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int216::Edge
    }
}
#[doc = "Field `INT216` writer - Interrupt 216"]
pub type Int216W<'a, REG> = crate::BitWriter<'a, REG, Int216>;
impl<'a, REG> Int216W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int216::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int216::Edge)
    }
}
#[doc = "Interrupt 217\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int217 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int217> for bool {
    #[inline(always)]
    fn from(variant: Int217) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT217` reader - Interrupt 217"]
pub type Int217R = crate::BitReader<Int217>;
impl Int217R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int217 {
        match self.bits {
            false => Int217::Level,
            true => Int217::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int217::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int217::Edge
    }
}
#[doc = "Field `INT217` writer - Interrupt 217"]
pub type Int217W<'a, REG> = crate::BitWriter<'a, REG, Int217>;
impl<'a, REG> Int217W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int217::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int217::Edge)
    }
}
#[doc = "Interrupt 218\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int218 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int218> for bool {
    #[inline(always)]
    fn from(variant: Int218) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT218` reader - Interrupt 218"]
pub type Int218R = crate::BitReader<Int218>;
impl Int218R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int218 {
        match self.bits {
            false => Int218::Level,
            true => Int218::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int218::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int218::Edge
    }
}
#[doc = "Field `INT218` writer - Interrupt 218"]
pub type Int218W<'a, REG> = crate::BitWriter<'a, REG, Int218>;
impl<'a, REG> Int218W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int218::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int218::Edge)
    }
}
#[doc = "Interrupt 219\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int219 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int219> for bool {
    #[inline(always)]
    fn from(variant: Int219) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT219` reader - Interrupt 219"]
pub type Int219R = crate::BitReader<Int219>;
impl Int219R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int219 {
        match self.bits {
            false => Int219::Level,
            true => Int219::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int219::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int219::Edge
    }
}
#[doc = "Field `INT219` writer - Interrupt 219"]
pub type Int219W<'a, REG> = crate::BitWriter<'a, REG, Int219>;
impl<'a, REG> Int219W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int219::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int219::Edge)
    }
}
#[doc = "Interrupt 220\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int220 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int220> for bool {
    #[inline(always)]
    fn from(variant: Int220) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT220` reader - Interrupt 220"]
pub type Int220R = crate::BitReader<Int220>;
impl Int220R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int220 {
        match self.bits {
            false => Int220::Level,
            true => Int220::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int220::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int220::Edge
    }
}
#[doc = "Field `INT220` writer - Interrupt 220"]
pub type Int220W<'a, REG> = crate::BitWriter<'a, REG, Int220>;
impl<'a, REG> Int220W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int220::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int220::Edge)
    }
}
#[doc = "Interrupt 221\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int221 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int221> for bool {
    #[inline(always)]
    fn from(variant: Int221) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT221` reader - Interrupt 221"]
pub type Int221R = crate::BitReader<Int221>;
impl Int221R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int221 {
        match self.bits {
            false => Int221::Level,
            true => Int221::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int221::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int221::Edge
    }
}
#[doc = "Field `INT221` writer - Interrupt 221"]
pub type Int221W<'a, REG> = crate::BitWriter<'a, REG, Int221>;
impl<'a, REG> Int221W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int221::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int221::Edge)
    }
}
#[doc = "Interrupt 222\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int222 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int222> for bool {
    #[inline(always)]
    fn from(variant: Int222) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT222` reader - Interrupt 222"]
pub type Int222R = crate::BitReader<Int222>;
impl Int222R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int222 {
        match self.bits {
            false => Int222::Level,
            true => Int222::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int222::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int222::Edge
    }
}
#[doc = "Field `INT222` writer - Interrupt 222"]
pub type Int222W<'a, REG> = crate::BitWriter<'a, REG, Int222>;
impl<'a, REG> Int222W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int222::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int222::Edge)
    }
}
#[doc = "Interrupt 223\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int223 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int223> for bool {
    #[inline(always)]
    fn from(variant: Int223) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT223` reader - Interrupt 223"]
pub type Int223R = crate::BitReader<Int223>;
impl Int223R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int223 {
        match self.bits {
            false => Int223::Level,
            true => Int223::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int223::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int223::Edge
    }
}
#[doc = "Field `INT223` writer - Interrupt 223"]
pub type Int223W<'a, REG> = crate::BitWriter<'a, REG, Int223>;
impl<'a, REG> Int223W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int223::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int223::Edge)
    }
}
impl R {
    #[doc = "Bit 1 - Interrupt 208"]
    #[inline(always)]
    pub fn int208(&self) -> Int208R {
        Int208R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt 209"]
    #[inline(always)]
    pub fn int209(&self) -> Int209R {
        Int209R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt 210"]
    #[inline(always)]
    pub fn int210(&self) -> Int210R {
        Int210R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt 211"]
    #[inline(always)]
    pub fn int211(&self) -> Int211R {
        Int211R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt 212"]
    #[inline(always)]
    pub fn int212(&self) -> Int212R {
        Int212R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt 213"]
    #[inline(always)]
    pub fn int213(&self) -> Int213R {
        Int213R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt 214"]
    #[inline(always)]
    pub fn int214(&self) -> Int214R {
        Int214R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt 215"]
    #[inline(always)]
    pub fn int215(&self) -> Int215R {
        Int215R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt 216"]
    #[inline(always)]
    pub fn int216(&self) -> Int216R {
        Int216R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt 217"]
    #[inline(always)]
    pub fn int217(&self) -> Int217R {
        Int217R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt 218"]
    #[inline(always)]
    pub fn int218(&self) -> Int218R {
        Int218R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt 219"]
    #[inline(always)]
    pub fn int219(&self) -> Int219R {
        Int219R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt 220"]
    #[inline(always)]
    pub fn int220(&self) -> Int220R {
        Int220R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt 221"]
    #[inline(always)]
    pub fn int221(&self) -> Int221R {
        Int221R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt 222"]
    #[inline(always)]
    pub fn int222(&self) -> Int222R {
        Int222R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt 223"]
    #[inline(always)]
    pub fn int223(&self) -> Int223R {
        Int223R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ICFGR52")
            .field("int208", &self.int208())
            .field("int209", &self.int209())
            .field("int210", &self.int210())
            .field("int211", &self.int211())
            .field("int212", &self.int212())
            .field("int213", &self.int213())
            .field("int214", &self.int214())
            .field("int215", &self.int215())
            .field("int216", &self.int216())
            .field("int217", &self.int217())
            .field("int218", &self.int218())
            .field("int219", &self.int219())
            .field("int220", &self.int220())
            .field("int221", &self.int221())
            .field("int222", &self.int222())
            .field("int223", &self.int223())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Interrupt 208"]
    #[inline(always)]
    #[must_use]
    pub fn int208(&mut self) -> Int208W<GicdIcfgr52Spec> {
        Int208W::new(self, 1)
    }
    #[doc = "Bit 3 - Interrupt 209"]
    #[inline(always)]
    #[must_use]
    pub fn int209(&mut self) -> Int209W<GicdIcfgr52Spec> {
        Int209W::new(self, 3)
    }
    #[doc = "Bit 5 - Interrupt 210"]
    #[inline(always)]
    #[must_use]
    pub fn int210(&mut self) -> Int210W<GicdIcfgr52Spec> {
        Int210W::new(self, 5)
    }
    #[doc = "Bit 7 - Interrupt 211"]
    #[inline(always)]
    #[must_use]
    pub fn int211(&mut self) -> Int211W<GicdIcfgr52Spec> {
        Int211W::new(self, 7)
    }
    #[doc = "Bit 9 - Interrupt 212"]
    #[inline(always)]
    #[must_use]
    pub fn int212(&mut self) -> Int212W<GicdIcfgr52Spec> {
        Int212W::new(self, 9)
    }
    #[doc = "Bit 11 - Interrupt 213"]
    #[inline(always)]
    #[must_use]
    pub fn int213(&mut self) -> Int213W<GicdIcfgr52Spec> {
        Int213W::new(self, 11)
    }
    #[doc = "Bit 13 - Interrupt 214"]
    #[inline(always)]
    #[must_use]
    pub fn int214(&mut self) -> Int214W<GicdIcfgr52Spec> {
        Int214W::new(self, 13)
    }
    #[doc = "Bit 15 - Interrupt 215"]
    #[inline(always)]
    #[must_use]
    pub fn int215(&mut self) -> Int215W<GicdIcfgr52Spec> {
        Int215W::new(self, 15)
    }
    #[doc = "Bit 17 - Interrupt 216"]
    #[inline(always)]
    #[must_use]
    pub fn int216(&mut self) -> Int216W<GicdIcfgr52Spec> {
        Int216W::new(self, 17)
    }
    #[doc = "Bit 19 - Interrupt 217"]
    #[inline(always)]
    #[must_use]
    pub fn int217(&mut self) -> Int217W<GicdIcfgr52Spec> {
        Int217W::new(self, 19)
    }
    #[doc = "Bit 21 - Interrupt 218"]
    #[inline(always)]
    #[must_use]
    pub fn int218(&mut self) -> Int218W<GicdIcfgr52Spec> {
        Int218W::new(self, 21)
    }
    #[doc = "Bit 23 - Interrupt 219"]
    #[inline(always)]
    #[must_use]
    pub fn int219(&mut self) -> Int219W<GicdIcfgr52Spec> {
        Int219W::new(self, 23)
    }
    #[doc = "Bit 25 - Interrupt 220"]
    #[inline(always)]
    #[must_use]
    pub fn int220(&mut self) -> Int220W<GicdIcfgr52Spec> {
        Int220W::new(self, 25)
    }
    #[doc = "Bit 27 - Interrupt 221"]
    #[inline(always)]
    #[must_use]
    pub fn int221(&mut self) -> Int221W<GicdIcfgr52Spec> {
        Int221W::new(self, 27)
    }
    #[doc = "Bit 29 - Interrupt 222"]
    #[inline(always)]
    #[must_use]
    pub fn int222(&mut self) -> Int222W<GicdIcfgr52Spec> {
        Int222W::new(self, 29)
    }
    #[doc = "Bit 31 - Interrupt 223"]
    #[inline(always)]
    #[must_use]
    pub fn int223(&mut self) -> Int223W<GicdIcfgr52Spec> {
        Int223W::new(self, 31)
    }
}
#[doc = "Interrupt Configuration 208 - 223\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icfgr52::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icfgr52::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIcfgr52Spec;
impl crate::RegisterSpec for GicdIcfgr52Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_icfgr52::R`](R) reader structure"]
impl crate::Readable for GicdIcfgr52Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_icfgr52::W`](W) writer structure"]
impl crate::Writable for GicdIcfgr52Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ICFGR52 to value 0"]
impl crate::Resettable for GicdIcfgr52Spec {
    const RESET_VALUE: u32 = 0;
}

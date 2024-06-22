#[doc = "Register `GICD_ICFGR8` reader"]
pub type R = crate::R<GicdIcfgr8Spec>;
#[doc = "Register `GICD_ICFGR8` writer"]
pub type W = crate::W<GicdIcfgr8Spec>;
#[doc = "Interrupt 32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int32 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int32> for bool {
    #[inline(always)]
    fn from(variant: Int32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT32` reader - Interrupt 32"]
pub type Int32R = crate::BitReader<Int32>;
impl Int32R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int32 {
        match self.bits {
            false => Int32::Level,
            true => Int32::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int32::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int32::Edge
    }
}
#[doc = "Field `INT32` writer - Interrupt 32"]
pub type Int32W<'a, REG> = crate::BitWriter<'a, REG, Int32>;
impl<'a, REG> Int32W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int32::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int32::Edge)
    }
}
#[doc = "Interrupt 33\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int33 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int33> for bool {
    #[inline(always)]
    fn from(variant: Int33) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT33` reader - Interrupt 33"]
pub type Int33R = crate::BitReader<Int33>;
impl Int33R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int33 {
        match self.bits {
            false => Int33::Level,
            true => Int33::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int33::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int33::Edge
    }
}
#[doc = "Field `INT33` writer - Interrupt 33"]
pub type Int33W<'a, REG> = crate::BitWriter<'a, REG, Int33>;
impl<'a, REG> Int33W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int33::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int33::Edge)
    }
}
#[doc = "Interrupt 34\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int34 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int34> for bool {
    #[inline(always)]
    fn from(variant: Int34) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT34` reader - Interrupt 34"]
pub type Int34R = crate::BitReader<Int34>;
impl Int34R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int34 {
        match self.bits {
            false => Int34::Level,
            true => Int34::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int34::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int34::Edge
    }
}
#[doc = "Field `INT34` writer - Interrupt 34"]
pub type Int34W<'a, REG> = crate::BitWriter<'a, REG, Int34>;
impl<'a, REG> Int34W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int34::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int34::Edge)
    }
}
#[doc = "Interrupt 35\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int35 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int35> for bool {
    #[inline(always)]
    fn from(variant: Int35) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT35` reader - Interrupt 35"]
pub type Int35R = crate::BitReader<Int35>;
impl Int35R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int35 {
        match self.bits {
            false => Int35::Level,
            true => Int35::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int35::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int35::Edge
    }
}
#[doc = "Field `INT35` writer - Interrupt 35"]
pub type Int35W<'a, REG> = crate::BitWriter<'a, REG, Int35>;
impl<'a, REG> Int35W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int35::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int35::Edge)
    }
}
#[doc = "Interrupt 36\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int36 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int36> for bool {
    #[inline(always)]
    fn from(variant: Int36) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT36` reader - Interrupt 36"]
pub type Int36R = crate::BitReader<Int36>;
impl Int36R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int36 {
        match self.bits {
            false => Int36::Level,
            true => Int36::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int36::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int36::Edge
    }
}
#[doc = "Field `INT36` writer - Interrupt 36"]
pub type Int36W<'a, REG> = crate::BitWriter<'a, REG, Int36>;
impl<'a, REG> Int36W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int36::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int36::Edge)
    }
}
#[doc = "Interrupt 37\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int37 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int37> for bool {
    #[inline(always)]
    fn from(variant: Int37) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT37` reader - Interrupt 37"]
pub type Int37R = crate::BitReader<Int37>;
impl Int37R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int37 {
        match self.bits {
            false => Int37::Level,
            true => Int37::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int37::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int37::Edge
    }
}
#[doc = "Field `INT37` writer - Interrupt 37"]
pub type Int37W<'a, REG> = crate::BitWriter<'a, REG, Int37>;
impl<'a, REG> Int37W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int37::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int37::Edge)
    }
}
#[doc = "Interrupt 38\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int38 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int38> for bool {
    #[inline(always)]
    fn from(variant: Int38) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT38` reader - Interrupt 38"]
pub type Int38R = crate::BitReader<Int38>;
impl Int38R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int38 {
        match self.bits {
            false => Int38::Level,
            true => Int38::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int38::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int38::Edge
    }
}
#[doc = "Field `INT38` writer - Interrupt 38"]
pub type Int38W<'a, REG> = crate::BitWriter<'a, REG, Int38>;
impl<'a, REG> Int38W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int38::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int38::Edge)
    }
}
#[doc = "Interrupt 39\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int39 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int39> for bool {
    #[inline(always)]
    fn from(variant: Int39) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT39` reader - Interrupt 39"]
pub type Int39R = crate::BitReader<Int39>;
impl Int39R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int39 {
        match self.bits {
            false => Int39::Level,
            true => Int39::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int39::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int39::Edge
    }
}
#[doc = "Field `INT39` writer - Interrupt 39"]
pub type Int39W<'a, REG> = crate::BitWriter<'a, REG, Int39>;
impl<'a, REG> Int39W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int39::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int39::Edge)
    }
}
#[doc = "Interrupt 40\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int40 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int40> for bool {
    #[inline(always)]
    fn from(variant: Int40) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT40` reader - Interrupt 40"]
pub type Int40R = crate::BitReader<Int40>;
impl Int40R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int40 {
        match self.bits {
            false => Int40::Level,
            true => Int40::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int40::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int40::Edge
    }
}
#[doc = "Field `INT40` writer - Interrupt 40"]
pub type Int40W<'a, REG> = crate::BitWriter<'a, REG, Int40>;
impl<'a, REG> Int40W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int40::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int40::Edge)
    }
}
#[doc = "Interrupt 41\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int41 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int41> for bool {
    #[inline(always)]
    fn from(variant: Int41) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT41` reader - Interrupt 41"]
pub type Int41R = crate::BitReader<Int41>;
impl Int41R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int41 {
        match self.bits {
            false => Int41::Level,
            true => Int41::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int41::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int41::Edge
    }
}
#[doc = "Field `INT41` writer - Interrupt 41"]
pub type Int41W<'a, REG> = crate::BitWriter<'a, REG, Int41>;
impl<'a, REG> Int41W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int41::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int41::Edge)
    }
}
#[doc = "Interrupt 42\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int42 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int42> for bool {
    #[inline(always)]
    fn from(variant: Int42) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT42` reader - Interrupt 42"]
pub type Int42R = crate::BitReader<Int42>;
impl Int42R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int42 {
        match self.bits {
            false => Int42::Level,
            true => Int42::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int42::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int42::Edge
    }
}
#[doc = "Field `INT42` writer - Interrupt 42"]
pub type Int42W<'a, REG> = crate::BitWriter<'a, REG, Int42>;
impl<'a, REG> Int42W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int42::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int42::Edge)
    }
}
#[doc = "Interrupt 43\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int43 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int43> for bool {
    #[inline(always)]
    fn from(variant: Int43) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT43` reader - Interrupt 43"]
pub type Int43R = crate::BitReader<Int43>;
impl Int43R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int43 {
        match self.bits {
            false => Int43::Level,
            true => Int43::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int43::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int43::Edge
    }
}
#[doc = "Field `INT43` writer - Interrupt 43"]
pub type Int43W<'a, REG> = crate::BitWriter<'a, REG, Int43>;
impl<'a, REG> Int43W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int43::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int43::Edge)
    }
}
#[doc = "Interrupt 44\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int44 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int44> for bool {
    #[inline(always)]
    fn from(variant: Int44) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT44` reader - Interrupt 44"]
pub type Int44R = crate::BitReader<Int44>;
impl Int44R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int44 {
        match self.bits {
            false => Int44::Level,
            true => Int44::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int44::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int44::Edge
    }
}
#[doc = "Field `INT44` writer - Interrupt 44"]
pub type Int44W<'a, REG> = crate::BitWriter<'a, REG, Int44>;
impl<'a, REG> Int44W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int44::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int44::Edge)
    }
}
#[doc = "Interrupt 45\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int45 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int45> for bool {
    #[inline(always)]
    fn from(variant: Int45) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT45` reader - Interrupt 45"]
pub type Int45R = crate::BitReader<Int45>;
impl Int45R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int45 {
        match self.bits {
            false => Int45::Level,
            true => Int45::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int45::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int45::Edge
    }
}
#[doc = "Field `INT45` writer - Interrupt 45"]
pub type Int45W<'a, REG> = crate::BitWriter<'a, REG, Int45>;
impl<'a, REG> Int45W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int45::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int45::Edge)
    }
}
#[doc = "Interrupt 46\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int46 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int46> for bool {
    #[inline(always)]
    fn from(variant: Int46) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT46` reader - Interrupt 46"]
pub type Int46R = crate::BitReader<Int46>;
impl Int46R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int46 {
        match self.bits {
            false => Int46::Level,
            true => Int46::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int46::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int46::Edge
    }
}
#[doc = "Field `INT46` writer - Interrupt 46"]
pub type Int46W<'a, REG> = crate::BitWriter<'a, REG, Int46>;
impl<'a, REG> Int46W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int46::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int46::Edge)
    }
}
#[doc = "Interrupt 47\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int47 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int47> for bool {
    #[inline(always)]
    fn from(variant: Int47) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT47` reader - Interrupt 47"]
pub type Int47R = crate::BitReader<Int47>;
impl Int47R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int47 {
        match self.bits {
            false => Int47::Level,
            true => Int47::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int47::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int47::Edge
    }
}
#[doc = "Field `INT47` writer - Interrupt 47"]
pub type Int47W<'a, REG> = crate::BitWriter<'a, REG, Int47>;
impl<'a, REG> Int47W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int47::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int47::Edge)
    }
}
impl R {
    #[doc = "Bit 1 - Interrupt 32"]
    #[inline(always)]
    pub fn int32(&self) -> Int32R {
        Int32R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt 33"]
    #[inline(always)]
    pub fn int33(&self) -> Int33R {
        Int33R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt 34"]
    #[inline(always)]
    pub fn int34(&self) -> Int34R {
        Int34R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt 35"]
    #[inline(always)]
    pub fn int35(&self) -> Int35R {
        Int35R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt 36"]
    #[inline(always)]
    pub fn int36(&self) -> Int36R {
        Int36R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt 37"]
    #[inline(always)]
    pub fn int37(&self) -> Int37R {
        Int37R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt 38"]
    #[inline(always)]
    pub fn int38(&self) -> Int38R {
        Int38R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt 39"]
    #[inline(always)]
    pub fn int39(&self) -> Int39R {
        Int39R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt 40"]
    #[inline(always)]
    pub fn int40(&self) -> Int40R {
        Int40R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt 41"]
    #[inline(always)]
    pub fn int41(&self) -> Int41R {
        Int41R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt 42"]
    #[inline(always)]
    pub fn int42(&self) -> Int42R {
        Int42R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt 43"]
    #[inline(always)]
    pub fn int43(&self) -> Int43R {
        Int43R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt 44"]
    #[inline(always)]
    pub fn int44(&self) -> Int44R {
        Int44R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt 45"]
    #[inline(always)]
    pub fn int45(&self) -> Int45R {
        Int45R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt 46"]
    #[inline(always)]
    pub fn int46(&self) -> Int46R {
        Int46R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt 47"]
    #[inline(always)]
    pub fn int47(&self) -> Int47R {
        Int47R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ICFGR8")
            .field("int32", &self.int32())
            .field("int33", &self.int33())
            .field("int34", &self.int34())
            .field("int35", &self.int35())
            .field("int36", &self.int36())
            .field("int37", &self.int37())
            .field("int38", &self.int38())
            .field("int39", &self.int39())
            .field("int40", &self.int40())
            .field("int41", &self.int41())
            .field("int42", &self.int42())
            .field("int43", &self.int43())
            .field("int44", &self.int44())
            .field("int45", &self.int45())
            .field("int46", &self.int46())
            .field("int47", &self.int47())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Interrupt 32"]
    #[inline(always)]
    #[must_use]
    pub fn int32(&mut self) -> Int32W<GicdIcfgr8Spec> {
        Int32W::new(self, 1)
    }
    #[doc = "Bit 3 - Interrupt 33"]
    #[inline(always)]
    #[must_use]
    pub fn int33(&mut self) -> Int33W<GicdIcfgr8Spec> {
        Int33W::new(self, 3)
    }
    #[doc = "Bit 5 - Interrupt 34"]
    #[inline(always)]
    #[must_use]
    pub fn int34(&mut self) -> Int34W<GicdIcfgr8Spec> {
        Int34W::new(self, 5)
    }
    #[doc = "Bit 7 - Interrupt 35"]
    #[inline(always)]
    #[must_use]
    pub fn int35(&mut self) -> Int35W<GicdIcfgr8Spec> {
        Int35W::new(self, 7)
    }
    #[doc = "Bit 9 - Interrupt 36"]
    #[inline(always)]
    #[must_use]
    pub fn int36(&mut self) -> Int36W<GicdIcfgr8Spec> {
        Int36W::new(self, 9)
    }
    #[doc = "Bit 11 - Interrupt 37"]
    #[inline(always)]
    #[must_use]
    pub fn int37(&mut self) -> Int37W<GicdIcfgr8Spec> {
        Int37W::new(self, 11)
    }
    #[doc = "Bit 13 - Interrupt 38"]
    #[inline(always)]
    #[must_use]
    pub fn int38(&mut self) -> Int38W<GicdIcfgr8Spec> {
        Int38W::new(self, 13)
    }
    #[doc = "Bit 15 - Interrupt 39"]
    #[inline(always)]
    #[must_use]
    pub fn int39(&mut self) -> Int39W<GicdIcfgr8Spec> {
        Int39W::new(self, 15)
    }
    #[doc = "Bit 17 - Interrupt 40"]
    #[inline(always)]
    #[must_use]
    pub fn int40(&mut self) -> Int40W<GicdIcfgr8Spec> {
        Int40W::new(self, 17)
    }
    #[doc = "Bit 19 - Interrupt 41"]
    #[inline(always)]
    #[must_use]
    pub fn int41(&mut self) -> Int41W<GicdIcfgr8Spec> {
        Int41W::new(self, 19)
    }
    #[doc = "Bit 21 - Interrupt 42"]
    #[inline(always)]
    #[must_use]
    pub fn int42(&mut self) -> Int42W<GicdIcfgr8Spec> {
        Int42W::new(self, 21)
    }
    #[doc = "Bit 23 - Interrupt 43"]
    #[inline(always)]
    #[must_use]
    pub fn int43(&mut self) -> Int43W<GicdIcfgr8Spec> {
        Int43W::new(self, 23)
    }
    #[doc = "Bit 25 - Interrupt 44"]
    #[inline(always)]
    #[must_use]
    pub fn int44(&mut self) -> Int44W<GicdIcfgr8Spec> {
        Int44W::new(self, 25)
    }
    #[doc = "Bit 27 - Interrupt 45"]
    #[inline(always)]
    #[must_use]
    pub fn int45(&mut self) -> Int45W<GicdIcfgr8Spec> {
        Int45W::new(self, 27)
    }
    #[doc = "Bit 29 - Interrupt 46"]
    #[inline(always)]
    #[must_use]
    pub fn int46(&mut self) -> Int46W<GicdIcfgr8Spec> {
        Int46W::new(self, 29)
    }
    #[doc = "Bit 31 - Interrupt 47"]
    #[inline(always)]
    #[must_use]
    pub fn int47(&mut self) -> Int47W<GicdIcfgr8Spec> {
        Int47W::new(self, 31)
    }
}
#[doc = "Interrupt Configuration 32 - 47\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icfgr8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icfgr8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIcfgr8Spec;
impl crate::RegisterSpec for GicdIcfgr8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_icfgr8::R`](R) reader structure"]
impl crate::Readable for GicdIcfgr8Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_icfgr8::W`](W) writer structure"]
impl crate::Writable for GicdIcfgr8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ICFGR8 to value 0"]
impl crate::Resettable for GicdIcfgr8Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `GICD_ICFGR4` reader"]
pub type R = crate::R<GicdIcfgr4Spec>;
#[doc = "Register `GICD_ICFGR4` writer"]
pub type W = crate::W<GicdIcfgr4Spec>;
#[doc = "Interrupt 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int16 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int16> for bool {
    #[inline(always)]
    fn from(variant: Int16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT16` reader - Interrupt 16"]
pub type Int16R = crate::BitReader<Int16>;
impl Int16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int16 {
        match self.bits {
            false => Int16::Level,
            true => Int16::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int16::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int16::Edge
    }
}
#[doc = "Field `INT16` writer - Interrupt 16"]
pub type Int16W<'a, REG> = crate::BitWriter<'a, REG, Int16>;
impl<'a, REG> Int16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int16::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int16::Edge)
    }
}
#[doc = "Interrupt 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int17 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int17> for bool {
    #[inline(always)]
    fn from(variant: Int17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT17` reader - Interrupt 17"]
pub type Int17R = crate::BitReader<Int17>;
impl Int17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int17 {
        match self.bits {
            false => Int17::Level,
            true => Int17::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int17::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int17::Edge
    }
}
#[doc = "Field `INT17` writer - Interrupt 17"]
pub type Int17W<'a, REG> = crate::BitWriter<'a, REG, Int17>;
impl<'a, REG> Int17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int17::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int17::Edge)
    }
}
#[doc = "Interrupt 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int18 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int18> for bool {
    #[inline(always)]
    fn from(variant: Int18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT18` reader - Interrupt 18"]
pub type Int18R = crate::BitReader<Int18>;
impl Int18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int18 {
        match self.bits {
            false => Int18::Level,
            true => Int18::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int18::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int18::Edge
    }
}
#[doc = "Field `INT18` writer - Interrupt 18"]
pub type Int18W<'a, REG> = crate::BitWriter<'a, REG, Int18>;
impl<'a, REG> Int18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int18::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int18::Edge)
    }
}
#[doc = "Interrupt 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int19 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int19> for bool {
    #[inline(always)]
    fn from(variant: Int19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT19` reader - Interrupt 19"]
pub type Int19R = crate::BitReader<Int19>;
impl Int19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int19 {
        match self.bits {
            false => Int19::Level,
            true => Int19::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int19::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int19::Edge
    }
}
#[doc = "Field `INT19` writer - Interrupt 19"]
pub type Int19W<'a, REG> = crate::BitWriter<'a, REG, Int19>;
impl<'a, REG> Int19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int19::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int19::Edge)
    }
}
#[doc = "Interrupt 20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int20 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int20> for bool {
    #[inline(always)]
    fn from(variant: Int20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT20` reader - Interrupt 20"]
pub type Int20R = crate::BitReader<Int20>;
impl Int20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int20 {
        match self.bits {
            false => Int20::Level,
            true => Int20::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int20::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int20::Edge
    }
}
#[doc = "Field `INT20` writer - Interrupt 20"]
pub type Int20W<'a, REG> = crate::BitWriter<'a, REG, Int20>;
impl<'a, REG> Int20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int20::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int20::Edge)
    }
}
#[doc = "Interrupt 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int21 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int21> for bool {
    #[inline(always)]
    fn from(variant: Int21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT21` reader - Interrupt 21"]
pub type Int21R = crate::BitReader<Int21>;
impl Int21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int21 {
        match self.bits {
            false => Int21::Level,
            true => Int21::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int21::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int21::Edge
    }
}
#[doc = "Field `INT21` writer - Interrupt 21"]
pub type Int21W<'a, REG> = crate::BitWriter<'a, REG, Int21>;
impl<'a, REG> Int21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int21::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int21::Edge)
    }
}
#[doc = "Interrupt 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int22 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int22> for bool {
    #[inline(always)]
    fn from(variant: Int22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT22` reader - Interrupt 22"]
pub type Int22R = crate::BitReader<Int22>;
impl Int22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int22 {
        match self.bits {
            false => Int22::Level,
            true => Int22::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int22::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int22::Edge
    }
}
#[doc = "Field `INT22` writer - Interrupt 22"]
pub type Int22W<'a, REG> = crate::BitWriter<'a, REG, Int22>;
impl<'a, REG> Int22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int22::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int22::Edge)
    }
}
#[doc = "Interrupt 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int23 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int23> for bool {
    #[inline(always)]
    fn from(variant: Int23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT23` reader - Interrupt 23"]
pub type Int23R = crate::BitReader<Int23>;
impl Int23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int23 {
        match self.bits {
            false => Int23::Level,
            true => Int23::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int23::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int23::Edge
    }
}
#[doc = "Field `INT23` writer - Interrupt 23"]
pub type Int23W<'a, REG> = crate::BitWriter<'a, REG, Int23>;
impl<'a, REG> Int23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int23::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int23::Edge)
    }
}
#[doc = "Interrupt 24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int24 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int24> for bool {
    #[inline(always)]
    fn from(variant: Int24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT24` reader - Interrupt 24"]
pub type Int24R = crate::BitReader<Int24>;
impl Int24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int24 {
        match self.bits {
            false => Int24::Level,
            true => Int24::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int24::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int24::Edge
    }
}
#[doc = "Field `INT24` writer - Interrupt 24"]
pub type Int24W<'a, REG> = crate::BitWriter<'a, REG, Int24>;
impl<'a, REG> Int24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int24::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int24::Edge)
    }
}
#[doc = "Interrupt 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int25 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int25> for bool {
    #[inline(always)]
    fn from(variant: Int25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT25` reader - Interrupt 25"]
pub type Int25R = crate::BitReader<Int25>;
impl Int25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int25 {
        match self.bits {
            false => Int25::Level,
            true => Int25::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int25::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int25::Edge
    }
}
#[doc = "Field `INT25` writer - Interrupt 25"]
pub type Int25W<'a, REG> = crate::BitWriter<'a, REG, Int25>;
impl<'a, REG> Int25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int25::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int25::Edge)
    }
}
#[doc = "Interrupt 26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int26 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int26> for bool {
    #[inline(always)]
    fn from(variant: Int26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT26` reader - Interrupt 26"]
pub type Int26R = crate::BitReader<Int26>;
impl Int26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int26 {
        match self.bits {
            false => Int26::Level,
            true => Int26::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int26::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int26::Edge
    }
}
#[doc = "Field `INT26` writer - Interrupt 26"]
pub type Int26W<'a, REG> = crate::BitWriter<'a, REG, Int26>;
impl<'a, REG> Int26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int26::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int26::Edge)
    }
}
#[doc = "Interrupt 27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int27 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int27> for bool {
    #[inline(always)]
    fn from(variant: Int27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT27` reader - Interrupt 27"]
pub type Int27R = crate::BitReader<Int27>;
impl Int27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int27 {
        match self.bits {
            false => Int27::Level,
            true => Int27::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int27::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int27::Edge
    }
}
#[doc = "Field `INT27` writer - Interrupt 27"]
pub type Int27W<'a, REG> = crate::BitWriter<'a, REG, Int27>;
impl<'a, REG> Int27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int27::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int27::Edge)
    }
}
#[doc = "Interrupt 28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int28 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int28> for bool {
    #[inline(always)]
    fn from(variant: Int28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT28` reader - Interrupt 28"]
pub type Int28R = crate::BitReader<Int28>;
impl Int28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int28 {
        match self.bits {
            false => Int28::Level,
            true => Int28::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int28::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int28::Edge
    }
}
#[doc = "Field `INT28` writer - Interrupt 28"]
pub type Int28W<'a, REG> = crate::BitWriter<'a, REG, Int28>;
impl<'a, REG> Int28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int28::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int28::Edge)
    }
}
#[doc = "Interrupt 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int29 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int29> for bool {
    #[inline(always)]
    fn from(variant: Int29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT29` reader - Interrupt 29"]
pub type Int29R = crate::BitReader<Int29>;
impl Int29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int29 {
        match self.bits {
            false => Int29::Level,
            true => Int29::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int29::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int29::Edge
    }
}
#[doc = "Field `INT29` writer - Interrupt 29"]
pub type Int29W<'a, REG> = crate::BitWriter<'a, REG, Int29>;
impl<'a, REG> Int29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int29::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int29::Edge)
    }
}
#[doc = "Interrupt 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int30 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int30> for bool {
    #[inline(always)]
    fn from(variant: Int30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT30` reader - Interrupt 30"]
pub type Int30R = crate::BitReader<Int30>;
impl Int30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int30 {
        match self.bits {
            false => Int30::Level,
            true => Int30::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int30::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int30::Edge
    }
}
#[doc = "Field `INT30` writer - Interrupt 30"]
pub type Int30W<'a, REG> = crate::BitWriter<'a, REG, Int30>;
impl<'a, REG> Int30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int30::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int30::Edge)
    }
}
#[doc = "Interrupt 31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int31 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int31> for bool {
    #[inline(always)]
    fn from(variant: Int31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT31` reader - Interrupt 31"]
pub type Int31R = crate::BitReader<Int31>;
impl Int31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int31 {
        match self.bits {
            false => Int31::Level,
            true => Int31::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int31::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int31::Edge
    }
}
#[doc = "Field `INT31` writer - Interrupt 31"]
pub type Int31W<'a, REG> = crate::BitWriter<'a, REG, Int31>;
impl<'a, REG> Int31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int31::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int31::Edge)
    }
}
impl R {
    #[doc = "Bit 1 - Interrupt 16"]
    #[inline(always)]
    pub fn int16(&self) -> Int16R {
        Int16R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt 17"]
    #[inline(always)]
    pub fn int17(&self) -> Int17R {
        Int17R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt 18"]
    #[inline(always)]
    pub fn int18(&self) -> Int18R {
        Int18R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt 19"]
    #[inline(always)]
    pub fn int19(&self) -> Int19R {
        Int19R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt 20"]
    #[inline(always)]
    pub fn int20(&self) -> Int20R {
        Int20R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt 21"]
    #[inline(always)]
    pub fn int21(&self) -> Int21R {
        Int21R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt 22"]
    #[inline(always)]
    pub fn int22(&self) -> Int22R {
        Int22R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt 23"]
    #[inline(always)]
    pub fn int23(&self) -> Int23R {
        Int23R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt 24"]
    #[inline(always)]
    pub fn int24(&self) -> Int24R {
        Int24R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt 25"]
    #[inline(always)]
    pub fn int25(&self) -> Int25R {
        Int25R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt 26"]
    #[inline(always)]
    pub fn int26(&self) -> Int26R {
        Int26R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt 27"]
    #[inline(always)]
    pub fn int27(&self) -> Int27R {
        Int27R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt 28"]
    #[inline(always)]
    pub fn int28(&self) -> Int28R {
        Int28R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt 29"]
    #[inline(always)]
    pub fn int29(&self) -> Int29R {
        Int29R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt 30"]
    #[inline(always)]
    pub fn int30(&self) -> Int30R {
        Int30R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt 31"]
    #[inline(always)]
    pub fn int31(&self) -> Int31R {
        Int31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ICFGR4")
            .field("int16", &self.int16())
            .field("int17", &self.int17())
            .field("int18", &self.int18())
            .field("int19", &self.int19())
            .field("int20", &self.int20())
            .field("int21", &self.int21())
            .field("int22", &self.int22())
            .field("int23", &self.int23())
            .field("int24", &self.int24())
            .field("int25", &self.int25())
            .field("int26", &self.int26())
            .field("int27", &self.int27())
            .field("int28", &self.int28())
            .field("int29", &self.int29())
            .field("int30", &self.int30())
            .field("int31", &self.int31())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Interrupt 16"]
    #[inline(always)]
    #[must_use]
    pub fn int16(&mut self) -> Int16W<GicdIcfgr4Spec> {
        Int16W::new(self, 1)
    }
    #[doc = "Bit 3 - Interrupt 17"]
    #[inline(always)]
    #[must_use]
    pub fn int17(&mut self) -> Int17W<GicdIcfgr4Spec> {
        Int17W::new(self, 3)
    }
    #[doc = "Bit 5 - Interrupt 18"]
    #[inline(always)]
    #[must_use]
    pub fn int18(&mut self) -> Int18W<GicdIcfgr4Spec> {
        Int18W::new(self, 5)
    }
    #[doc = "Bit 7 - Interrupt 19"]
    #[inline(always)]
    #[must_use]
    pub fn int19(&mut self) -> Int19W<GicdIcfgr4Spec> {
        Int19W::new(self, 7)
    }
    #[doc = "Bit 9 - Interrupt 20"]
    #[inline(always)]
    #[must_use]
    pub fn int20(&mut self) -> Int20W<GicdIcfgr4Spec> {
        Int20W::new(self, 9)
    }
    #[doc = "Bit 11 - Interrupt 21"]
    #[inline(always)]
    #[must_use]
    pub fn int21(&mut self) -> Int21W<GicdIcfgr4Spec> {
        Int21W::new(self, 11)
    }
    #[doc = "Bit 13 - Interrupt 22"]
    #[inline(always)]
    #[must_use]
    pub fn int22(&mut self) -> Int22W<GicdIcfgr4Spec> {
        Int22W::new(self, 13)
    }
    #[doc = "Bit 15 - Interrupt 23"]
    #[inline(always)]
    #[must_use]
    pub fn int23(&mut self) -> Int23W<GicdIcfgr4Spec> {
        Int23W::new(self, 15)
    }
    #[doc = "Bit 17 - Interrupt 24"]
    #[inline(always)]
    #[must_use]
    pub fn int24(&mut self) -> Int24W<GicdIcfgr4Spec> {
        Int24W::new(self, 17)
    }
    #[doc = "Bit 19 - Interrupt 25"]
    #[inline(always)]
    #[must_use]
    pub fn int25(&mut self) -> Int25W<GicdIcfgr4Spec> {
        Int25W::new(self, 19)
    }
    #[doc = "Bit 21 - Interrupt 26"]
    #[inline(always)]
    #[must_use]
    pub fn int26(&mut self) -> Int26W<GicdIcfgr4Spec> {
        Int26W::new(self, 21)
    }
    #[doc = "Bit 23 - Interrupt 27"]
    #[inline(always)]
    #[must_use]
    pub fn int27(&mut self) -> Int27W<GicdIcfgr4Spec> {
        Int27W::new(self, 23)
    }
    #[doc = "Bit 25 - Interrupt 28"]
    #[inline(always)]
    #[must_use]
    pub fn int28(&mut self) -> Int28W<GicdIcfgr4Spec> {
        Int28W::new(self, 25)
    }
    #[doc = "Bit 27 - Interrupt 29"]
    #[inline(always)]
    #[must_use]
    pub fn int29(&mut self) -> Int29W<GicdIcfgr4Spec> {
        Int29W::new(self, 27)
    }
    #[doc = "Bit 29 - Interrupt 30"]
    #[inline(always)]
    #[must_use]
    pub fn int30(&mut self) -> Int30W<GicdIcfgr4Spec> {
        Int30W::new(self, 29)
    }
    #[doc = "Bit 31 - Interrupt 31"]
    #[inline(always)]
    #[must_use]
    pub fn int31(&mut self) -> Int31W<GicdIcfgr4Spec> {
        Int31W::new(self, 31)
    }
}
#[doc = "Interrupt Configuration 16 - 31\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icfgr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icfgr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIcfgr4Spec;
impl crate::RegisterSpec for GicdIcfgr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_icfgr4::R`](R) reader structure"]
impl crate::Readable for GicdIcfgr4Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_icfgr4::W`](W) writer structure"]
impl crate::Writable for GicdIcfgr4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ICFGR4 to value 0"]
impl crate::Resettable for GicdIcfgr4Spec {
    const RESET_VALUE: u32 = 0;
}

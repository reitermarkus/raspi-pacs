#[doc = "Register `GICD_ICFGR12` reader"]
pub type R = crate::R<GicdIcfgr12Spec>;
#[doc = "Register `GICD_ICFGR12` writer"]
pub type W = crate::W<GicdIcfgr12Spec>;
#[doc = "Interrupt 48\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int48 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int48> for bool {
    #[inline(always)]
    fn from(variant: Int48) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT48` reader - Interrupt 48"]
pub type Int48R = crate::BitReader<Int48>;
impl Int48R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int48 {
        match self.bits {
            false => Int48::Level,
            true => Int48::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int48::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int48::Edge
    }
}
#[doc = "Field `INT48` writer - Interrupt 48"]
pub type Int48W<'a, REG> = crate::BitWriter<'a, REG, Int48>;
impl<'a, REG> Int48W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int48::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int48::Edge)
    }
}
#[doc = "Interrupt 49\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int49 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int49> for bool {
    #[inline(always)]
    fn from(variant: Int49) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT49` reader - Interrupt 49"]
pub type Int49R = crate::BitReader<Int49>;
impl Int49R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int49 {
        match self.bits {
            false => Int49::Level,
            true => Int49::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int49::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int49::Edge
    }
}
#[doc = "Field `INT49` writer - Interrupt 49"]
pub type Int49W<'a, REG> = crate::BitWriter<'a, REG, Int49>;
impl<'a, REG> Int49W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int49::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int49::Edge)
    }
}
#[doc = "Interrupt 50\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int50 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int50> for bool {
    #[inline(always)]
    fn from(variant: Int50) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT50` reader - Interrupt 50"]
pub type Int50R = crate::BitReader<Int50>;
impl Int50R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int50 {
        match self.bits {
            false => Int50::Level,
            true => Int50::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int50::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int50::Edge
    }
}
#[doc = "Field `INT50` writer - Interrupt 50"]
pub type Int50W<'a, REG> = crate::BitWriter<'a, REG, Int50>;
impl<'a, REG> Int50W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int50::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int50::Edge)
    }
}
#[doc = "Interrupt 51\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int51 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int51> for bool {
    #[inline(always)]
    fn from(variant: Int51) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT51` reader - Interrupt 51"]
pub type Int51R = crate::BitReader<Int51>;
impl Int51R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int51 {
        match self.bits {
            false => Int51::Level,
            true => Int51::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int51::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int51::Edge
    }
}
#[doc = "Field `INT51` writer - Interrupt 51"]
pub type Int51W<'a, REG> = crate::BitWriter<'a, REG, Int51>;
impl<'a, REG> Int51W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int51::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int51::Edge)
    }
}
#[doc = "Interrupt 52\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int52 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int52> for bool {
    #[inline(always)]
    fn from(variant: Int52) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT52` reader - Interrupt 52"]
pub type Int52R = crate::BitReader<Int52>;
impl Int52R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int52 {
        match self.bits {
            false => Int52::Level,
            true => Int52::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int52::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int52::Edge
    }
}
#[doc = "Field `INT52` writer - Interrupt 52"]
pub type Int52W<'a, REG> = crate::BitWriter<'a, REG, Int52>;
impl<'a, REG> Int52W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int52::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int52::Edge)
    }
}
#[doc = "Interrupt 53\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int53 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int53> for bool {
    #[inline(always)]
    fn from(variant: Int53) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT53` reader - Interrupt 53"]
pub type Int53R = crate::BitReader<Int53>;
impl Int53R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int53 {
        match self.bits {
            false => Int53::Level,
            true => Int53::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int53::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int53::Edge
    }
}
#[doc = "Field `INT53` writer - Interrupt 53"]
pub type Int53W<'a, REG> = crate::BitWriter<'a, REG, Int53>;
impl<'a, REG> Int53W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int53::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int53::Edge)
    }
}
#[doc = "Interrupt 54\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int54 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int54> for bool {
    #[inline(always)]
    fn from(variant: Int54) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT54` reader - Interrupt 54"]
pub type Int54R = crate::BitReader<Int54>;
impl Int54R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int54 {
        match self.bits {
            false => Int54::Level,
            true => Int54::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int54::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int54::Edge
    }
}
#[doc = "Field `INT54` writer - Interrupt 54"]
pub type Int54W<'a, REG> = crate::BitWriter<'a, REG, Int54>;
impl<'a, REG> Int54W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int54::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int54::Edge)
    }
}
#[doc = "Interrupt 55\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int55 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int55> for bool {
    #[inline(always)]
    fn from(variant: Int55) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT55` reader - Interrupt 55"]
pub type Int55R = crate::BitReader<Int55>;
impl Int55R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int55 {
        match self.bits {
            false => Int55::Level,
            true => Int55::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int55::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int55::Edge
    }
}
#[doc = "Field `INT55` writer - Interrupt 55"]
pub type Int55W<'a, REG> = crate::BitWriter<'a, REG, Int55>;
impl<'a, REG> Int55W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int55::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int55::Edge)
    }
}
#[doc = "Interrupt 56\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int56 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int56> for bool {
    #[inline(always)]
    fn from(variant: Int56) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT56` reader - Interrupt 56"]
pub type Int56R = crate::BitReader<Int56>;
impl Int56R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int56 {
        match self.bits {
            false => Int56::Level,
            true => Int56::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int56::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int56::Edge
    }
}
#[doc = "Field `INT56` writer - Interrupt 56"]
pub type Int56W<'a, REG> = crate::BitWriter<'a, REG, Int56>;
impl<'a, REG> Int56W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int56::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int56::Edge)
    }
}
#[doc = "Interrupt 57\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int57 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int57> for bool {
    #[inline(always)]
    fn from(variant: Int57) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT57` reader - Interrupt 57"]
pub type Int57R = crate::BitReader<Int57>;
impl Int57R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int57 {
        match self.bits {
            false => Int57::Level,
            true => Int57::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int57::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int57::Edge
    }
}
#[doc = "Field `INT57` writer - Interrupt 57"]
pub type Int57W<'a, REG> = crate::BitWriter<'a, REG, Int57>;
impl<'a, REG> Int57W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int57::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int57::Edge)
    }
}
#[doc = "Interrupt 58\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int58 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int58> for bool {
    #[inline(always)]
    fn from(variant: Int58) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT58` reader - Interrupt 58"]
pub type Int58R = crate::BitReader<Int58>;
impl Int58R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int58 {
        match self.bits {
            false => Int58::Level,
            true => Int58::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int58::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int58::Edge
    }
}
#[doc = "Field `INT58` writer - Interrupt 58"]
pub type Int58W<'a, REG> = crate::BitWriter<'a, REG, Int58>;
impl<'a, REG> Int58W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int58::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int58::Edge)
    }
}
#[doc = "Interrupt 59\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int59 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int59> for bool {
    #[inline(always)]
    fn from(variant: Int59) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT59` reader - Interrupt 59"]
pub type Int59R = crate::BitReader<Int59>;
impl Int59R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int59 {
        match self.bits {
            false => Int59::Level,
            true => Int59::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int59::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int59::Edge
    }
}
#[doc = "Field `INT59` writer - Interrupt 59"]
pub type Int59W<'a, REG> = crate::BitWriter<'a, REG, Int59>;
impl<'a, REG> Int59W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int59::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int59::Edge)
    }
}
#[doc = "Interrupt 60\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int60 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int60> for bool {
    #[inline(always)]
    fn from(variant: Int60) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT60` reader - Interrupt 60"]
pub type Int60R = crate::BitReader<Int60>;
impl Int60R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int60 {
        match self.bits {
            false => Int60::Level,
            true => Int60::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int60::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int60::Edge
    }
}
#[doc = "Field `INT60` writer - Interrupt 60"]
pub type Int60W<'a, REG> = crate::BitWriter<'a, REG, Int60>;
impl<'a, REG> Int60W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int60::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int60::Edge)
    }
}
#[doc = "Interrupt 61\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int61 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int61> for bool {
    #[inline(always)]
    fn from(variant: Int61) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT61` reader - Interrupt 61"]
pub type Int61R = crate::BitReader<Int61>;
impl Int61R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int61 {
        match self.bits {
            false => Int61::Level,
            true => Int61::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int61::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int61::Edge
    }
}
#[doc = "Field `INT61` writer - Interrupt 61"]
pub type Int61W<'a, REG> = crate::BitWriter<'a, REG, Int61>;
impl<'a, REG> Int61W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int61::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int61::Edge)
    }
}
#[doc = "Interrupt 62\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int62 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int62> for bool {
    #[inline(always)]
    fn from(variant: Int62) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT62` reader - Interrupt 62"]
pub type Int62R = crate::BitReader<Int62>;
impl Int62R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int62 {
        match self.bits {
            false => Int62::Level,
            true => Int62::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int62::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int62::Edge
    }
}
#[doc = "Field `INT62` writer - Interrupt 62"]
pub type Int62W<'a, REG> = crate::BitWriter<'a, REG, Int62>;
impl<'a, REG> Int62W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int62::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int62::Edge)
    }
}
#[doc = "Interrupt 63\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int63 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int63> for bool {
    #[inline(always)]
    fn from(variant: Int63) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT63` reader - Interrupt 63"]
pub type Int63R = crate::BitReader<Int63>;
impl Int63R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int63 {
        match self.bits {
            false => Int63::Level,
            true => Int63::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int63::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int63::Edge
    }
}
#[doc = "Field `INT63` writer - Interrupt 63"]
pub type Int63W<'a, REG> = crate::BitWriter<'a, REG, Int63>;
impl<'a, REG> Int63W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int63::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int63::Edge)
    }
}
impl R {
    #[doc = "Bit 1 - Interrupt 48"]
    #[inline(always)]
    pub fn int48(&self) -> Int48R {
        Int48R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt 49"]
    #[inline(always)]
    pub fn int49(&self) -> Int49R {
        Int49R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt 50"]
    #[inline(always)]
    pub fn int50(&self) -> Int50R {
        Int50R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt 51"]
    #[inline(always)]
    pub fn int51(&self) -> Int51R {
        Int51R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt 52"]
    #[inline(always)]
    pub fn int52(&self) -> Int52R {
        Int52R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt 53"]
    #[inline(always)]
    pub fn int53(&self) -> Int53R {
        Int53R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt 54"]
    #[inline(always)]
    pub fn int54(&self) -> Int54R {
        Int54R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt 55"]
    #[inline(always)]
    pub fn int55(&self) -> Int55R {
        Int55R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt 56"]
    #[inline(always)]
    pub fn int56(&self) -> Int56R {
        Int56R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt 57"]
    #[inline(always)]
    pub fn int57(&self) -> Int57R {
        Int57R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt 58"]
    #[inline(always)]
    pub fn int58(&self) -> Int58R {
        Int58R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt 59"]
    #[inline(always)]
    pub fn int59(&self) -> Int59R {
        Int59R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt 60"]
    #[inline(always)]
    pub fn int60(&self) -> Int60R {
        Int60R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt 61"]
    #[inline(always)]
    pub fn int61(&self) -> Int61R {
        Int61R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt 62"]
    #[inline(always)]
    pub fn int62(&self) -> Int62R {
        Int62R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt 63"]
    #[inline(always)]
    pub fn int63(&self) -> Int63R {
        Int63R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ICFGR12")
            .field("int48", &self.int48())
            .field("int49", &self.int49())
            .field("int50", &self.int50())
            .field("int51", &self.int51())
            .field("int52", &self.int52())
            .field("int53", &self.int53())
            .field("int54", &self.int54())
            .field("int55", &self.int55())
            .field("int56", &self.int56())
            .field("int57", &self.int57())
            .field("int58", &self.int58())
            .field("int59", &self.int59())
            .field("int60", &self.int60())
            .field("int61", &self.int61())
            .field("int62", &self.int62())
            .field("int63", &self.int63())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Interrupt 48"]
    #[inline(always)]
    #[must_use]
    pub fn int48(&mut self) -> Int48W<GicdIcfgr12Spec> {
        Int48W::new(self, 1)
    }
    #[doc = "Bit 3 - Interrupt 49"]
    #[inline(always)]
    #[must_use]
    pub fn int49(&mut self) -> Int49W<GicdIcfgr12Spec> {
        Int49W::new(self, 3)
    }
    #[doc = "Bit 5 - Interrupt 50"]
    #[inline(always)]
    #[must_use]
    pub fn int50(&mut self) -> Int50W<GicdIcfgr12Spec> {
        Int50W::new(self, 5)
    }
    #[doc = "Bit 7 - Interrupt 51"]
    #[inline(always)]
    #[must_use]
    pub fn int51(&mut self) -> Int51W<GicdIcfgr12Spec> {
        Int51W::new(self, 7)
    }
    #[doc = "Bit 9 - Interrupt 52"]
    #[inline(always)]
    #[must_use]
    pub fn int52(&mut self) -> Int52W<GicdIcfgr12Spec> {
        Int52W::new(self, 9)
    }
    #[doc = "Bit 11 - Interrupt 53"]
    #[inline(always)]
    #[must_use]
    pub fn int53(&mut self) -> Int53W<GicdIcfgr12Spec> {
        Int53W::new(self, 11)
    }
    #[doc = "Bit 13 - Interrupt 54"]
    #[inline(always)]
    #[must_use]
    pub fn int54(&mut self) -> Int54W<GicdIcfgr12Spec> {
        Int54W::new(self, 13)
    }
    #[doc = "Bit 15 - Interrupt 55"]
    #[inline(always)]
    #[must_use]
    pub fn int55(&mut self) -> Int55W<GicdIcfgr12Spec> {
        Int55W::new(self, 15)
    }
    #[doc = "Bit 17 - Interrupt 56"]
    #[inline(always)]
    #[must_use]
    pub fn int56(&mut self) -> Int56W<GicdIcfgr12Spec> {
        Int56W::new(self, 17)
    }
    #[doc = "Bit 19 - Interrupt 57"]
    #[inline(always)]
    #[must_use]
    pub fn int57(&mut self) -> Int57W<GicdIcfgr12Spec> {
        Int57W::new(self, 19)
    }
    #[doc = "Bit 21 - Interrupt 58"]
    #[inline(always)]
    #[must_use]
    pub fn int58(&mut self) -> Int58W<GicdIcfgr12Spec> {
        Int58W::new(self, 21)
    }
    #[doc = "Bit 23 - Interrupt 59"]
    #[inline(always)]
    #[must_use]
    pub fn int59(&mut self) -> Int59W<GicdIcfgr12Spec> {
        Int59W::new(self, 23)
    }
    #[doc = "Bit 25 - Interrupt 60"]
    #[inline(always)]
    #[must_use]
    pub fn int60(&mut self) -> Int60W<GicdIcfgr12Spec> {
        Int60W::new(self, 25)
    }
    #[doc = "Bit 27 - Interrupt 61"]
    #[inline(always)]
    #[must_use]
    pub fn int61(&mut self) -> Int61W<GicdIcfgr12Spec> {
        Int61W::new(self, 27)
    }
    #[doc = "Bit 29 - Interrupt 62"]
    #[inline(always)]
    #[must_use]
    pub fn int62(&mut self) -> Int62W<GicdIcfgr12Spec> {
        Int62W::new(self, 29)
    }
    #[doc = "Bit 31 - Interrupt 63"]
    #[inline(always)]
    #[must_use]
    pub fn int63(&mut self) -> Int63W<GicdIcfgr12Spec> {
        Int63W::new(self, 31)
    }
}
#[doc = "Interrupt Configuration 48 - 63\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icfgr12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icfgr12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIcfgr12Spec;
impl crate::RegisterSpec for GicdIcfgr12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_icfgr12::R`](R) reader structure"]
impl crate::Readable for GicdIcfgr12Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_icfgr12::W`](W) writer structure"]
impl crate::Writable for GicdIcfgr12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ICFGR12 to value 0"]
impl crate::Resettable for GicdIcfgr12Spec {
    const RESET_VALUE: u32 = 0;
}

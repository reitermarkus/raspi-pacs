#[doc = "Register `GICD_ICFGR0` reader"]
pub type R = crate::R<GicdIcfgr0Spec>;
#[doc = "Register `GICD_ICFGR0` writer"]
pub type W = crate::W<GicdIcfgr0Spec>;
#[doc = "Interrupt 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int0 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int0> for bool {
    #[inline(always)]
    fn from(variant: Int0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT0` reader - Interrupt 0"]
pub type Int0R = crate::BitReader<Int0>;
impl Int0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int0 {
        match self.bits {
            false => Int0::Level,
            true => Int0::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int0::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int0::Edge
    }
}
#[doc = "Field `INT0` writer - Interrupt 0"]
pub type Int0W<'a, REG> = crate::BitWriter<'a, REG, Int0>;
impl<'a, REG> Int0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int0::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int0::Edge)
    }
}
#[doc = "Interrupt 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int1 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int1> for bool {
    #[inline(always)]
    fn from(variant: Int1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT1` reader - Interrupt 1"]
pub type Int1R = crate::BitReader<Int1>;
impl Int1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int1 {
        match self.bits {
            false => Int1::Level,
            true => Int1::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int1::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int1::Edge
    }
}
#[doc = "Field `INT1` writer - Interrupt 1"]
pub type Int1W<'a, REG> = crate::BitWriter<'a, REG, Int1>;
impl<'a, REG> Int1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int1::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int1::Edge)
    }
}
#[doc = "Interrupt 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int2 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int2> for bool {
    #[inline(always)]
    fn from(variant: Int2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT2` reader - Interrupt 2"]
pub type Int2R = crate::BitReader<Int2>;
impl Int2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int2 {
        match self.bits {
            false => Int2::Level,
            true => Int2::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int2::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int2::Edge
    }
}
#[doc = "Field `INT2` writer - Interrupt 2"]
pub type Int2W<'a, REG> = crate::BitWriter<'a, REG, Int2>;
impl<'a, REG> Int2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int2::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int2::Edge)
    }
}
#[doc = "Interrupt 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int3 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int3> for bool {
    #[inline(always)]
    fn from(variant: Int3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT3` reader - Interrupt 3"]
pub type Int3R = crate::BitReader<Int3>;
impl Int3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int3 {
        match self.bits {
            false => Int3::Level,
            true => Int3::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int3::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int3::Edge
    }
}
#[doc = "Field `INT3` writer - Interrupt 3"]
pub type Int3W<'a, REG> = crate::BitWriter<'a, REG, Int3>;
impl<'a, REG> Int3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int3::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int3::Edge)
    }
}
#[doc = "Interrupt 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int4 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int4> for bool {
    #[inline(always)]
    fn from(variant: Int4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT4` reader - Interrupt 4"]
pub type Int4R = crate::BitReader<Int4>;
impl Int4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int4 {
        match self.bits {
            false => Int4::Level,
            true => Int4::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int4::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int4::Edge
    }
}
#[doc = "Field `INT4` writer - Interrupt 4"]
pub type Int4W<'a, REG> = crate::BitWriter<'a, REG, Int4>;
impl<'a, REG> Int4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int4::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int4::Edge)
    }
}
#[doc = "Interrupt 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int5 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int5> for bool {
    #[inline(always)]
    fn from(variant: Int5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT5` reader - Interrupt 5"]
pub type Int5R = crate::BitReader<Int5>;
impl Int5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int5 {
        match self.bits {
            false => Int5::Level,
            true => Int5::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int5::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int5::Edge
    }
}
#[doc = "Field `INT5` writer - Interrupt 5"]
pub type Int5W<'a, REG> = crate::BitWriter<'a, REG, Int5>;
impl<'a, REG> Int5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int5::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int5::Edge)
    }
}
#[doc = "Interrupt 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int6 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int6> for bool {
    #[inline(always)]
    fn from(variant: Int6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT6` reader - Interrupt 6"]
pub type Int6R = crate::BitReader<Int6>;
impl Int6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int6 {
        match self.bits {
            false => Int6::Level,
            true => Int6::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int6::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int6::Edge
    }
}
#[doc = "Field `INT6` writer - Interrupt 6"]
pub type Int6W<'a, REG> = crate::BitWriter<'a, REG, Int6>;
impl<'a, REG> Int6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int6::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int6::Edge)
    }
}
#[doc = "Interrupt 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int7 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int7> for bool {
    #[inline(always)]
    fn from(variant: Int7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT7` reader - Interrupt 7"]
pub type Int7R = crate::BitReader<Int7>;
impl Int7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int7 {
        match self.bits {
            false => Int7::Level,
            true => Int7::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int7::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int7::Edge
    }
}
#[doc = "Field `INT7` writer - Interrupt 7"]
pub type Int7W<'a, REG> = crate::BitWriter<'a, REG, Int7>;
impl<'a, REG> Int7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int7::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int7::Edge)
    }
}
#[doc = "Interrupt 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int8 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int8> for bool {
    #[inline(always)]
    fn from(variant: Int8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT8` reader - Interrupt 8"]
pub type Int8R = crate::BitReader<Int8>;
impl Int8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int8 {
        match self.bits {
            false => Int8::Level,
            true => Int8::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int8::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int8::Edge
    }
}
#[doc = "Field `INT8` writer - Interrupt 8"]
pub type Int8W<'a, REG> = crate::BitWriter<'a, REG, Int8>;
impl<'a, REG> Int8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int8::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int8::Edge)
    }
}
#[doc = "Interrupt 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int9 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int9> for bool {
    #[inline(always)]
    fn from(variant: Int9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT9` reader - Interrupt 9"]
pub type Int9R = crate::BitReader<Int9>;
impl Int9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int9 {
        match self.bits {
            false => Int9::Level,
            true => Int9::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int9::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int9::Edge
    }
}
#[doc = "Field `INT9` writer - Interrupt 9"]
pub type Int9W<'a, REG> = crate::BitWriter<'a, REG, Int9>;
impl<'a, REG> Int9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int9::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int9::Edge)
    }
}
#[doc = "Interrupt 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int10 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int10> for bool {
    #[inline(always)]
    fn from(variant: Int10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT10` reader - Interrupt 10"]
pub type Int10R = crate::BitReader<Int10>;
impl Int10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int10 {
        match self.bits {
            false => Int10::Level,
            true => Int10::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int10::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int10::Edge
    }
}
#[doc = "Field `INT10` writer - Interrupt 10"]
pub type Int10W<'a, REG> = crate::BitWriter<'a, REG, Int10>;
impl<'a, REG> Int10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int10::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int10::Edge)
    }
}
#[doc = "Interrupt 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int11 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int11> for bool {
    #[inline(always)]
    fn from(variant: Int11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT11` reader - Interrupt 11"]
pub type Int11R = crate::BitReader<Int11>;
impl Int11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int11 {
        match self.bits {
            false => Int11::Level,
            true => Int11::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int11::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int11::Edge
    }
}
#[doc = "Field `INT11` writer - Interrupt 11"]
pub type Int11W<'a, REG> = crate::BitWriter<'a, REG, Int11>;
impl<'a, REG> Int11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int11::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int11::Edge)
    }
}
#[doc = "Interrupt 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int12 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int12> for bool {
    #[inline(always)]
    fn from(variant: Int12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT12` reader - Interrupt 12"]
pub type Int12R = crate::BitReader<Int12>;
impl Int12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int12 {
        match self.bits {
            false => Int12::Level,
            true => Int12::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int12::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int12::Edge
    }
}
#[doc = "Field `INT12` writer - Interrupt 12"]
pub type Int12W<'a, REG> = crate::BitWriter<'a, REG, Int12>;
impl<'a, REG> Int12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int12::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int12::Edge)
    }
}
#[doc = "Interrupt 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int13 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int13> for bool {
    #[inline(always)]
    fn from(variant: Int13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT13` reader - Interrupt 13"]
pub type Int13R = crate::BitReader<Int13>;
impl Int13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int13 {
        match self.bits {
            false => Int13::Level,
            true => Int13::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int13::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int13::Edge
    }
}
#[doc = "Field `INT13` writer - Interrupt 13"]
pub type Int13W<'a, REG> = crate::BitWriter<'a, REG, Int13>;
impl<'a, REG> Int13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int13::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int13::Edge)
    }
}
#[doc = "Interrupt 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int14 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int14> for bool {
    #[inline(always)]
    fn from(variant: Int14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT14` reader - Interrupt 14"]
pub type Int14R = crate::BitReader<Int14>;
impl Int14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int14 {
        match self.bits {
            false => Int14::Level,
            true => Int14::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int14::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int14::Edge
    }
}
#[doc = "Field `INT14` writer - Interrupt 14"]
pub type Int14W<'a, REG> = crate::BitWriter<'a, REG, Int14>;
impl<'a, REG> Int14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int14::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int14::Edge)
    }
}
#[doc = "Interrupt 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int15 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int15> for bool {
    #[inline(always)]
    fn from(variant: Int15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT15` reader - Interrupt 15"]
pub type Int15R = crate::BitReader<Int15>;
impl Int15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int15 {
        match self.bits {
            false => Int15::Level,
            true => Int15::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int15::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int15::Edge
    }
}
#[doc = "Field `INT15` writer - Interrupt 15"]
pub type Int15W<'a, REG> = crate::BitWriter<'a, REG, Int15>;
impl<'a, REG> Int15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int15::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int15::Edge)
    }
}
impl R {
    #[doc = "Bit 1 - Interrupt 0"]
    #[inline(always)]
    pub fn int0(&self) -> Int0R {
        Int0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt 1"]
    #[inline(always)]
    pub fn int1(&self) -> Int1R {
        Int1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt 2"]
    #[inline(always)]
    pub fn int2(&self) -> Int2R {
        Int2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt 3"]
    #[inline(always)]
    pub fn int3(&self) -> Int3R {
        Int3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt 4"]
    #[inline(always)]
    pub fn int4(&self) -> Int4R {
        Int4R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt 5"]
    #[inline(always)]
    pub fn int5(&self) -> Int5R {
        Int5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt 6"]
    #[inline(always)]
    pub fn int6(&self) -> Int6R {
        Int6R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt 7"]
    #[inline(always)]
    pub fn int7(&self) -> Int7R {
        Int7R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt 8"]
    #[inline(always)]
    pub fn int8(&self) -> Int8R {
        Int8R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt 9"]
    #[inline(always)]
    pub fn int9(&self) -> Int9R {
        Int9R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt 10"]
    #[inline(always)]
    pub fn int10(&self) -> Int10R {
        Int10R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt 11"]
    #[inline(always)]
    pub fn int11(&self) -> Int11R {
        Int11R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt 12"]
    #[inline(always)]
    pub fn int12(&self) -> Int12R {
        Int12R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt 13"]
    #[inline(always)]
    pub fn int13(&self) -> Int13R {
        Int13R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt 14"]
    #[inline(always)]
    pub fn int14(&self) -> Int14R {
        Int14R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt 15"]
    #[inline(always)]
    pub fn int15(&self) -> Int15R {
        Int15R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ICFGR0")
            .field("int0", &self.int0())
            .field("int1", &self.int1())
            .field("int2", &self.int2())
            .field("int3", &self.int3())
            .field("int4", &self.int4())
            .field("int5", &self.int5())
            .field("int6", &self.int6())
            .field("int7", &self.int7())
            .field("int8", &self.int8())
            .field("int9", &self.int9())
            .field("int10", &self.int10())
            .field("int11", &self.int11())
            .field("int12", &self.int12())
            .field("int13", &self.int13())
            .field("int14", &self.int14())
            .field("int15", &self.int15())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Interrupt 0"]
    #[inline(always)]
    #[must_use]
    pub fn int0(&mut self) -> Int0W<GicdIcfgr0Spec> {
        Int0W::new(self, 1)
    }
    #[doc = "Bit 3 - Interrupt 1"]
    #[inline(always)]
    #[must_use]
    pub fn int1(&mut self) -> Int1W<GicdIcfgr0Spec> {
        Int1W::new(self, 3)
    }
    #[doc = "Bit 5 - Interrupt 2"]
    #[inline(always)]
    #[must_use]
    pub fn int2(&mut self) -> Int2W<GicdIcfgr0Spec> {
        Int2W::new(self, 5)
    }
    #[doc = "Bit 7 - Interrupt 3"]
    #[inline(always)]
    #[must_use]
    pub fn int3(&mut self) -> Int3W<GicdIcfgr0Spec> {
        Int3W::new(self, 7)
    }
    #[doc = "Bit 9 - Interrupt 4"]
    #[inline(always)]
    #[must_use]
    pub fn int4(&mut self) -> Int4W<GicdIcfgr0Spec> {
        Int4W::new(self, 9)
    }
    #[doc = "Bit 11 - Interrupt 5"]
    #[inline(always)]
    #[must_use]
    pub fn int5(&mut self) -> Int5W<GicdIcfgr0Spec> {
        Int5W::new(self, 11)
    }
    #[doc = "Bit 13 - Interrupt 6"]
    #[inline(always)]
    #[must_use]
    pub fn int6(&mut self) -> Int6W<GicdIcfgr0Spec> {
        Int6W::new(self, 13)
    }
    #[doc = "Bit 15 - Interrupt 7"]
    #[inline(always)]
    #[must_use]
    pub fn int7(&mut self) -> Int7W<GicdIcfgr0Spec> {
        Int7W::new(self, 15)
    }
    #[doc = "Bit 17 - Interrupt 8"]
    #[inline(always)]
    #[must_use]
    pub fn int8(&mut self) -> Int8W<GicdIcfgr0Spec> {
        Int8W::new(self, 17)
    }
    #[doc = "Bit 19 - Interrupt 9"]
    #[inline(always)]
    #[must_use]
    pub fn int9(&mut self) -> Int9W<GicdIcfgr0Spec> {
        Int9W::new(self, 19)
    }
    #[doc = "Bit 21 - Interrupt 10"]
    #[inline(always)]
    #[must_use]
    pub fn int10(&mut self) -> Int10W<GicdIcfgr0Spec> {
        Int10W::new(self, 21)
    }
    #[doc = "Bit 23 - Interrupt 11"]
    #[inline(always)]
    #[must_use]
    pub fn int11(&mut self) -> Int11W<GicdIcfgr0Spec> {
        Int11W::new(self, 23)
    }
    #[doc = "Bit 25 - Interrupt 12"]
    #[inline(always)]
    #[must_use]
    pub fn int12(&mut self) -> Int12W<GicdIcfgr0Spec> {
        Int12W::new(self, 25)
    }
    #[doc = "Bit 27 - Interrupt 13"]
    #[inline(always)]
    #[must_use]
    pub fn int13(&mut self) -> Int13W<GicdIcfgr0Spec> {
        Int13W::new(self, 27)
    }
    #[doc = "Bit 29 - Interrupt 14"]
    #[inline(always)]
    #[must_use]
    pub fn int14(&mut self) -> Int14W<GicdIcfgr0Spec> {
        Int14W::new(self, 29)
    }
    #[doc = "Bit 31 - Interrupt 15"]
    #[inline(always)]
    #[must_use]
    pub fn int15(&mut self) -> Int15W<GicdIcfgr0Spec> {
        Int15W::new(self, 31)
    }
}
#[doc = "Interrupt Configuration 0 - 15\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icfgr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icfgr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIcfgr0Spec;
impl crate::RegisterSpec for GicdIcfgr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_icfgr0::R`](R) reader structure"]
impl crate::Readable for GicdIcfgr0Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_icfgr0::W`](W) writer structure"]
impl crate::Writable for GicdIcfgr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ICFGR0 to value 0"]
impl crate::Resettable for GicdIcfgr0Spec {
    const RESET_VALUE: u32 = 0;
}

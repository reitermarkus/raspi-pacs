#[doc = "Register `GICD_ICFGR20` reader"]
pub type R = crate::R<GicdIcfgr20Spec>;
#[doc = "Register `GICD_ICFGR20` writer"]
pub type W = crate::W<GicdIcfgr20Spec>;
#[doc = "Interrupt 80\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int80 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int80> for bool {
    #[inline(always)]
    fn from(variant: Int80) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT80` reader - Interrupt 80"]
pub type Int80R = crate::BitReader<Int80>;
impl Int80R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int80 {
        match self.bits {
            false => Int80::Level,
            true => Int80::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int80::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int80::Edge
    }
}
#[doc = "Field `INT80` writer - Interrupt 80"]
pub type Int80W<'a, REG> = crate::BitWriter<'a, REG, Int80>;
impl<'a, REG> Int80W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int80::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int80::Edge)
    }
}
#[doc = "Interrupt 81\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int81 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int81> for bool {
    #[inline(always)]
    fn from(variant: Int81) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT81` reader - Interrupt 81"]
pub type Int81R = crate::BitReader<Int81>;
impl Int81R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int81 {
        match self.bits {
            false => Int81::Level,
            true => Int81::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int81::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int81::Edge
    }
}
#[doc = "Field `INT81` writer - Interrupt 81"]
pub type Int81W<'a, REG> = crate::BitWriter<'a, REG, Int81>;
impl<'a, REG> Int81W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int81::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int81::Edge)
    }
}
#[doc = "Interrupt 82\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int82 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int82> for bool {
    #[inline(always)]
    fn from(variant: Int82) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT82` reader - Interrupt 82"]
pub type Int82R = crate::BitReader<Int82>;
impl Int82R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int82 {
        match self.bits {
            false => Int82::Level,
            true => Int82::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int82::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int82::Edge
    }
}
#[doc = "Field `INT82` writer - Interrupt 82"]
pub type Int82W<'a, REG> = crate::BitWriter<'a, REG, Int82>;
impl<'a, REG> Int82W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int82::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int82::Edge)
    }
}
#[doc = "Interrupt 83\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int83 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int83> for bool {
    #[inline(always)]
    fn from(variant: Int83) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT83` reader - Interrupt 83"]
pub type Int83R = crate::BitReader<Int83>;
impl Int83R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int83 {
        match self.bits {
            false => Int83::Level,
            true => Int83::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int83::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int83::Edge
    }
}
#[doc = "Field `INT83` writer - Interrupt 83"]
pub type Int83W<'a, REG> = crate::BitWriter<'a, REG, Int83>;
impl<'a, REG> Int83W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int83::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int83::Edge)
    }
}
#[doc = "Interrupt 84\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int84 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int84> for bool {
    #[inline(always)]
    fn from(variant: Int84) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT84` reader - Interrupt 84"]
pub type Int84R = crate::BitReader<Int84>;
impl Int84R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int84 {
        match self.bits {
            false => Int84::Level,
            true => Int84::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int84::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int84::Edge
    }
}
#[doc = "Field `INT84` writer - Interrupt 84"]
pub type Int84W<'a, REG> = crate::BitWriter<'a, REG, Int84>;
impl<'a, REG> Int84W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int84::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int84::Edge)
    }
}
#[doc = "Interrupt 85\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int85 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int85> for bool {
    #[inline(always)]
    fn from(variant: Int85) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT85` reader - Interrupt 85"]
pub type Int85R = crate::BitReader<Int85>;
impl Int85R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int85 {
        match self.bits {
            false => Int85::Level,
            true => Int85::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int85::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int85::Edge
    }
}
#[doc = "Field `INT85` writer - Interrupt 85"]
pub type Int85W<'a, REG> = crate::BitWriter<'a, REG, Int85>;
impl<'a, REG> Int85W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int85::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int85::Edge)
    }
}
#[doc = "Interrupt 86\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int86 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int86> for bool {
    #[inline(always)]
    fn from(variant: Int86) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT86` reader - Interrupt 86"]
pub type Int86R = crate::BitReader<Int86>;
impl Int86R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int86 {
        match self.bits {
            false => Int86::Level,
            true => Int86::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int86::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int86::Edge
    }
}
#[doc = "Field `INT86` writer - Interrupt 86"]
pub type Int86W<'a, REG> = crate::BitWriter<'a, REG, Int86>;
impl<'a, REG> Int86W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int86::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int86::Edge)
    }
}
#[doc = "Interrupt 87\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int87 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int87> for bool {
    #[inline(always)]
    fn from(variant: Int87) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT87` reader - Interrupt 87"]
pub type Int87R = crate::BitReader<Int87>;
impl Int87R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int87 {
        match self.bits {
            false => Int87::Level,
            true => Int87::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int87::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int87::Edge
    }
}
#[doc = "Field `INT87` writer - Interrupt 87"]
pub type Int87W<'a, REG> = crate::BitWriter<'a, REG, Int87>;
impl<'a, REG> Int87W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int87::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int87::Edge)
    }
}
#[doc = "Interrupt 88\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int88 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int88> for bool {
    #[inline(always)]
    fn from(variant: Int88) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT88` reader - Interrupt 88"]
pub type Int88R = crate::BitReader<Int88>;
impl Int88R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int88 {
        match self.bits {
            false => Int88::Level,
            true => Int88::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int88::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int88::Edge
    }
}
#[doc = "Field `INT88` writer - Interrupt 88"]
pub type Int88W<'a, REG> = crate::BitWriter<'a, REG, Int88>;
impl<'a, REG> Int88W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int88::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int88::Edge)
    }
}
#[doc = "Interrupt 89\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int89 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int89> for bool {
    #[inline(always)]
    fn from(variant: Int89) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT89` reader - Interrupt 89"]
pub type Int89R = crate::BitReader<Int89>;
impl Int89R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int89 {
        match self.bits {
            false => Int89::Level,
            true => Int89::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int89::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int89::Edge
    }
}
#[doc = "Field `INT89` writer - Interrupt 89"]
pub type Int89W<'a, REG> = crate::BitWriter<'a, REG, Int89>;
impl<'a, REG> Int89W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int89::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int89::Edge)
    }
}
#[doc = "Interrupt 90\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int90 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int90> for bool {
    #[inline(always)]
    fn from(variant: Int90) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT90` reader - Interrupt 90"]
pub type Int90R = crate::BitReader<Int90>;
impl Int90R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int90 {
        match self.bits {
            false => Int90::Level,
            true => Int90::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int90::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int90::Edge
    }
}
#[doc = "Field `INT90` writer - Interrupt 90"]
pub type Int90W<'a, REG> = crate::BitWriter<'a, REG, Int90>;
impl<'a, REG> Int90W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int90::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int90::Edge)
    }
}
#[doc = "Interrupt 91\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int91 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int91> for bool {
    #[inline(always)]
    fn from(variant: Int91) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT91` reader - Interrupt 91"]
pub type Int91R = crate::BitReader<Int91>;
impl Int91R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int91 {
        match self.bits {
            false => Int91::Level,
            true => Int91::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int91::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int91::Edge
    }
}
#[doc = "Field `INT91` writer - Interrupt 91"]
pub type Int91W<'a, REG> = crate::BitWriter<'a, REG, Int91>;
impl<'a, REG> Int91W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int91::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int91::Edge)
    }
}
#[doc = "Interrupt 92\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int92 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int92> for bool {
    #[inline(always)]
    fn from(variant: Int92) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT92` reader - Interrupt 92"]
pub type Int92R = crate::BitReader<Int92>;
impl Int92R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int92 {
        match self.bits {
            false => Int92::Level,
            true => Int92::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int92::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int92::Edge
    }
}
#[doc = "Field `INT92` writer - Interrupt 92"]
pub type Int92W<'a, REG> = crate::BitWriter<'a, REG, Int92>;
impl<'a, REG> Int92W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int92::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int92::Edge)
    }
}
#[doc = "Interrupt 93\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int93 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int93> for bool {
    #[inline(always)]
    fn from(variant: Int93) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT93` reader - Interrupt 93"]
pub type Int93R = crate::BitReader<Int93>;
impl Int93R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int93 {
        match self.bits {
            false => Int93::Level,
            true => Int93::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int93::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int93::Edge
    }
}
#[doc = "Field `INT93` writer - Interrupt 93"]
pub type Int93W<'a, REG> = crate::BitWriter<'a, REG, Int93>;
impl<'a, REG> Int93W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int93::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int93::Edge)
    }
}
#[doc = "Interrupt 94\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int94 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int94> for bool {
    #[inline(always)]
    fn from(variant: Int94) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT94` reader - Interrupt 94"]
pub type Int94R = crate::BitReader<Int94>;
impl Int94R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int94 {
        match self.bits {
            false => Int94::Level,
            true => Int94::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int94::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int94::Edge
    }
}
#[doc = "Field `INT94` writer - Interrupt 94"]
pub type Int94W<'a, REG> = crate::BitWriter<'a, REG, Int94>;
impl<'a, REG> Int94W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int94::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int94::Edge)
    }
}
#[doc = "Interrupt 95\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int95 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int95> for bool {
    #[inline(always)]
    fn from(variant: Int95) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT95` reader - Interrupt 95"]
pub type Int95R = crate::BitReader<Int95>;
impl Int95R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int95 {
        match self.bits {
            false => Int95::Level,
            true => Int95::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int95::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int95::Edge
    }
}
#[doc = "Field `INT95` writer - Interrupt 95"]
pub type Int95W<'a, REG> = crate::BitWriter<'a, REG, Int95>;
impl<'a, REG> Int95W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int95::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int95::Edge)
    }
}
impl R {
    #[doc = "Bit 1 - Interrupt 80"]
    #[inline(always)]
    pub fn int80(&self) -> Int80R {
        Int80R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt 81"]
    #[inline(always)]
    pub fn int81(&self) -> Int81R {
        Int81R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt 82"]
    #[inline(always)]
    pub fn int82(&self) -> Int82R {
        Int82R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt 83"]
    #[inline(always)]
    pub fn int83(&self) -> Int83R {
        Int83R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt 84"]
    #[inline(always)]
    pub fn int84(&self) -> Int84R {
        Int84R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt 85"]
    #[inline(always)]
    pub fn int85(&self) -> Int85R {
        Int85R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt 86"]
    #[inline(always)]
    pub fn int86(&self) -> Int86R {
        Int86R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt 87"]
    #[inline(always)]
    pub fn int87(&self) -> Int87R {
        Int87R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt 88"]
    #[inline(always)]
    pub fn int88(&self) -> Int88R {
        Int88R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt 89"]
    #[inline(always)]
    pub fn int89(&self) -> Int89R {
        Int89R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt 90"]
    #[inline(always)]
    pub fn int90(&self) -> Int90R {
        Int90R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt 91"]
    #[inline(always)]
    pub fn int91(&self) -> Int91R {
        Int91R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt 92"]
    #[inline(always)]
    pub fn int92(&self) -> Int92R {
        Int92R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt 93"]
    #[inline(always)]
    pub fn int93(&self) -> Int93R {
        Int93R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt 94"]
    #[inline(always)]
    pub fn int94(&self) -> Int94R {
        Int94R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt 95"]
    #[inline(always)]
    pub fn int95(&self) -> Int95R {
        Int95R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ICFGR20")
            .field("int80", &self.int80())
            .field("int81", &self.int81())
            .field("int82", &self.int82())
            .field("int83", &self.int83())
            .field("int84", &self.int84())
            .field("int85", &self.int85())
            .field("int86", &self.int86())
            .field("int87", &self.int87())
            .field("int88", &self.int88())
            .field("int89", &self.int89())
            .field("int90", &self.int90())
            .field("int91", &self.int91())
            .field("int92", &self.int92())
            .field("int93", &self.int93())
            .field("int94", &self.int94())
            .field("int95", &self.int95())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Interrupt 80"]
    #[inline(always)]
    #[must_use]
    pub fn int80(&mut self) -> Int80W<GicdIcfgr20Spec> {
        Int80W::new(self, 1)
    }
    #[doc = "Bit 3 - Interrupt 81"]
    #[inline(always)]
    #[must_use]
    pub fn int81(&mut self) -> Int81W<GicdIcfgr20Spec> {
        Int81W::new(self, 3)
    }
    #[doc = "Bit 5 - Interrupt 82"]
    #[inline(always)]
    #[must_use]
    pub fn int82(&mut self) -> Int82W<GicdIcfgr20Spec> {
        Int82W::new(self, 5)
    }
    #[doc = "Bit 7 - Interrupt 83"]
    #[inline(always)]
    #[must_use]
    pub fn int83(&mut self) -> Int83W<GicdIcfgr20Spec> {
        Int83W::new(self, 7)
    }
    #[doc = "Bit 9 - Interrupt 84"]
    #[inline(always)]
    #[must_use]
    pub fn int84(&mut self) -> Int84W<GicdIcfgr20Spec> {
        Int84W::new(self, 9)
    }
    #[doc = "Bit 11 - Interrupt 85"]
    #[inline(always)]
    #[must_use]
    pub fn int85(&mut self) -> Int85W<GicdIcfgr20Spec> {
        Int85W::new(self, 11)
    }
    #[doc = "Bit 13 - Interrupt 86"]
    #[inline(always)]
    #[must_use]
    pub fn int86(&mut self) -> Int86W<GicdIcfgr20Spec> {
        Int86W::new(self, 13)
    }
    #[doc = "Bit 15 - Interrupt 87"]
    #[inline(always)]
    #[must_use]
    pub fn int87(&mut self) -> Int87W<GicdIcfgr20Spec> {
        Int87W::new(self, 15)
    }
    #[doc = "Bit 17 - Interrupt 88"]
    #[inline(always)]
    #[must_use]
    pub fn int88(&mut self) -> Int88W<GicdIcfgr20Spec> {
        Int88W::new(self, 17)
    }
    #[doc = "Bit 19 - Interrupt 89"]
    #[inline(always)]
    #[must_use]
    pub fn int89(&mut self) -> Int89W<GicdIcfgr20Spec> {
        Int89W::new(self, 19)
    }
    #[doc = "Bit 21 - Interrupt 90"]
    #[inline(always)]
    #[must_use]
    pub fn int90(&mut self) -> Int90W<GicdIcfgr20Spec> {
        Int90W::new(self, 21)
    }
    #[doc = "Bit 23 - Interrupt 91"]
    #[inline(always)]
    #[must_use]
    pub fn int91(&mut self) -> Int91W<GicdIcfgr20Spec> {
        Int91W::new(self, 23)
    }
    #[doc = "Bit 25 - Interrupt 92"]
    #[inline(always)]
    #[must_use]
    pub fn int92(&mut self) -> Int92W<GicdIcfgr20Spec> {
        Int92W::new(self, 25)
    }
    #[doc = "Bit 27 - Interrupt 93"]
    #[inline(always)]
    #[must_use]
    pub fn int93(&mut self) -> Int93W<GicdIcfgr20Spec> {
        Int93W::new(self, 27)
    }
    #[doc = "Bit 29 - Interrupt 94"]
    #[inline(always)]
    #[must_use]
    pub fn int94(&mut self) -> Int94W<GicdIcfgr20Spec> {
        Int94W::new(self, 29)
    }
    #[doc = "Bit 31 - Interrupt 95"]
    #[inline(always)]
    #[must_use]
    pub fn int95(&mut self) -> Int95W<GicdIcfgr20Spec> {
        Int95W::new(self, 31)
    }
}
#[doc = "Interrupt Configuration 80 - 95\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icfgr20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icfgr20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIcfgr20Spec;
impl crate::RegisterSpec for GicdIcfgr20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_icfgr20::R`](R) reader structure"]
impl crate::Readable for GicdIcfgr20Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_icfgr20::W`](W) writer structure"]
impl crate::Writable for GicdIcfgr20Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ICFGR20 to value 0"]
impl crate::Resettable for GicdIcfgr20Spec {
    const RESET_VALUE: u32 = 0;
}

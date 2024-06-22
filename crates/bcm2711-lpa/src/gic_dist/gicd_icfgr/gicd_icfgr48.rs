#[doc = "Register `GICD_ICFGR48` reader"]
pub type R = crate::R<GicdIcfgr48Spec>;
#[doc = "Register `GICD_ICFGR48` writer"]
pub type W = crate::W<GicdIcfgr48Spec>;
#[doc = "Interrupt 192\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int192 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int192> for bool {
    #[inline(always)]
    fn from(variant: Int192) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT192` reader - Interrupt 192"]
pub type Int192R = crate::BitReader<Int192>;
impl Int192R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int192 {
        match self.bits {
            false => Int192::Level,
            true => Int192::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int192::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int192::Edge
    }
}
#[doc = "Field `INT192` writer - Interrupt 192"]
pub type Int192W<'a, REG> = crate::BitWriter<'a, REG, Int192>;
impl<'a, REG> Int192W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int192::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int192::Edge)
    }
}
#[doc = "Interrupt 193\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int193 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int193> for bool {
    #[inline(always)]
    fn from(variant: Int193) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT193` reader - Interrupt 193"]
pub type Int193R = crate::BitReader<Int193>;
impl Int193R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int193 {
        match self.bits {
            false => Int193::Level,
            true => Int193::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int193::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int193::Edge
    }
}
#[doc = "Field `INT193` writer - Interrupt 193"]
pub type Int193W<'a, REG> = crate::BitWriter<'a, REG, Int193>;
impl<'a, REG> Int193W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int193::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int193::Edge)
    }
}
#[doc = "Interrupt 194\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int194 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int194> for bool {
    #[inline(always)]
    fn from(variant: Int194) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT194` reader - Interrupt 194"]
pub type Int194R = crate::BitReader<Int194>;
impl Int194R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int194 {
        match self.bits {
            false => Int194::Level,
            true => Int194::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int194::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int194::Edge
    }
}
#[doc = "Field `INT194` writer - Interrupt 194"]
pub type Int194W<'a, REG> = crate::BitWriter<'a, REG, Int194>;
impl<'a, REG> Int194W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int194::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int194::Edge)
    }
}
#[doc = "Interrupt 195\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int195 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int195> for bool {
    #[inline(always)]
    fn from(variant: Int195) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT195` reader - Interrupt 195"]
pub type Int195R = crate::BitReader<Int195>;
impl Int195R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int195 {
        match self.bits {
            false => Int195::Level,
            true => Int195::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int195::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int195::Edge
    }
}
#[doc = "Field `INT195` writer - Interrupt 195"]
pub type Int195W<'a, REG> = crate::BitWriter<'a, REG, Int195>;
impl<'a, REG> Int195W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int195::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int195::Edge)
    }
}
#[doc = "Interrupt 196\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int196 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int196> for bool {
    #[inline(always)]
    fn from(variant: Int196) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT196` reader - Interrupt 196"]
pub type Int196R = crate::BitReader<Int196>;
impl Int196R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int196 {
        match self.bits {
            false => Int196::Level,
            true => Int196::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int196::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int196::Edge
    }
}
#[doc = "Field `INT196` writer - Interrupt 196"]
pub type Int196W<'a, REG> = crate::BitWriter<'a, REG, Int196>;
impl<'a, REG> Int196W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int196::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int196::Edge)
    }
}
#[doc = "Interrupt 197\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int197 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int197> for bool {
    #[inline(always)]
    fn from(variant: Int197) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT197` reader - Interrupt 197"]
pub type Int197R = crate::BitReader<Int197>;
impl Int197R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int197 {
        match self.bits {
            false => Int197::Level,
            true => Int197::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int197::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int197::Edge
    }
}
#[doc = "Field `INT197` writer - Interrupt 197"]
pub type Int197W<'a, REG> = crate::BitWriter<'a, REG, Int197>;
impl<'a, REG> Int197W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int197::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int197::Edge)
    }
}
#[doc = "Interrupt 198\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int198 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int198> for bool {
    #[inline(always)]
    fn from(variant: Int198) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT198` reader - Interrupt 198"]
pub type Int198R = crate::BitReader<Int198>;
impl Int198R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int198 {
        match self.bits {
            false => Int198::Level,
            true => Int198::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int198::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int198::Edge
    }
}
#[doc = "Field `INT198` writer - Interrupt 198"]
pub type Int198W<'a, REG> = crate::BitWriter<'a, REG, Int198>;
impl<'a, REG> Int198W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int198::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int198::Edge)
    }
}
#[doc = "Interrupt 199\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int199 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int199> for bool {
    #[inline(always)]
    fn from(variant: Int199) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT199` reader - Interrupt 199"]
pub type Int199R = crate::BitReader<Int199>;
impl Int199R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int199 {
        match self.bits {
            false => Int199::Level,
            true => Int199::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int199::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int199::Edge
    }
}
#[doc = "Field `INT199` writer - Interrupt 199"]
pub type Int199W<'a, REG> = crate::BitWriter<'a, REG, Int199>;
impl<'a, REG> Int199W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int199::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int199::Edge)
    }
}
#[doc = "Interrupt 200\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int200 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int200> for bool {
    #[inline(always)]
    fn from(variant: Int200) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT200` reader - Interrupt 200"]
pub type Int200R = crate::BitReader<Int200>;
impl Int200R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int200 {
        match self.bits {
            false => Int200::Level,
            true => Int200::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int200::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int200::Edge
    }
}
#[doc = "Field `INT200` writer - Interrupt 200"]
pub type Int200W<'a, REG> = crate::BitWriter<'a, REG, Int200>;
impl<'a, REG> Int200W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int200::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int200::Edge)
    }
}
#[doc = "Interrupt 201\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int201 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int201> for bool {
    #[inline(always)]
    fn from(variant: Int201) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT201` reader - Interrupt 201"]
pub type Int201R = crate::BitReader<Int201>;
impl Int201R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int201 {
        match self.bits {
            false => Int201::Level,
            true => Int201::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int201::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int201::Edge
    }
}
#[doc = "Field `INT201` writer - Interrupt 201"]
pub type Int201W<'a, REG> = crate::BitWriter<'a, REG, Int201>;
impl<'a, REG> Int201W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int201::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int201::Edge)
    }
}
#[doc = "Interrupt 202\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int202 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int202> for bool {
    #[inline(always)]
    fn from(variant: Int202) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT202` reader - Interrupt 202"]
pub type Int202R = crate::BitReader<Int202>;
impl Int202R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int202 {
        match self.bits {
            false => Int202::Level,
            true => Int202::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int202::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int202::Edge
    }
}
#[doc = "Field `INT202` writer - Interrupt 202"]
pub type Int202W<'a, REG> = crate::BitWriter<'a, REG, Int202>;
impl<'a, REG> Int202W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int202::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int202::Edge)
    }
}
#[doc = "Interrupt 203\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int203 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int203> for bool {
    #[inline(always)]
    fn from(variant: Int203) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT203` reader - Interrupt 203"]
pub type Int203R = crate::BitReader<Int203>;
impl Int203R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int203 {
        match self.bits {
            false => Int203::Level,
            true => Int203::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int203::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int203::Edge
    }
}
#[doc = "Field `INT203` writer - Interrupt 203"]
pub type Int203W<'a, REG> = crate::BitWriter<'a, REG, Int203>;
impl<'a, REG> Int203W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int203::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int203::Edge)
    }
}
#[doc = "Interrupt 204\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int204 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int204> for bool {
    #[inline(always)]
    fn from(variant: Int204) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT204` reader - Interrupt 204"]
pub type Int204R = crate::BitReader<Int204>;
impl Int204R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int204 {
        match self.bits {
            false => Int204::Level,
            true => Int204::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int204::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int204::Edge
    }
}
#[doc = "Field `INT204` writer - Interrupt 204"]
pub type Int204W<'a, REG> = crate::BitWriter<'a, REG, Int204>;
impl<'a, REG> Int204W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int204::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int204::Edge)
    }
}
#[doc = "Interrupt 205\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int205 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int205> for bool {
    #[inline(always)]
    fn from(variant: Int205) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT205` reader - Interrupt 205"]
pub type Int205R = crate::BitReader<Int205>;
impl Int205R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int205 {
        match self.bits {
            false => Int205::Level,
            true => Int205::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int205::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int205::Edge
    }
}
#[doc = "Field `INT205` writer - Interrupt 205"]
pub type Int205W<'a, REG> = crate::BitWriter<'a, REG, Int205>;
impl<'a, REG> Int205W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int205::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int205::Edge)
    }
}
#[doc = "Interrupt 206\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int206 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int206> for bool {
    #[inline(always)]
    fn from(variant: Int206) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT206` reader - Interrupt 206"]
pub type Int206R = crate::BitReader<Int206>;
impl Int206R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int206 {
        match self.bits {
            false => Int206::Level,
            true => Int206::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int206::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int206::Edge
    }
}
#[doc = "Field `INT206` writer - Interrupt 206"]
pub type Int206W<'a, REG> = crate::BitWriter<'a, REG, Int206>;
impl<'a, REG> Int206W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int206::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int206::Edge)
    }
}
#[doc = "Interrupt 207\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int207 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int207> for bool {
    #[inline(always)]
    fn from(variant: Int207) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT207` reader - Interrupt 207"]
pub type Int207R = crate::BitReader<Int207>;
impl Int207R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int207 {
        match self.bits {
            false => Int207::Level,
            true => Int207::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int207::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int207::Edge
    }
}
#[doc = "Field `INT207` writer - Interrupt 207"]
pub type Int207W<'a, REG> = crate::BitWriter<'a, REG, Int207>;
impl<'a, REG> Int207W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int207::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int207::Edge)
    }
}
impl R {
    #[doc = "Bit 1 - Interrupt 192"]
    #[inline(always)]
    pub fn int192(&self) -> Int192R {
        Int192R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt 193"]
    #[inline(always)]
    pub fn int193(&self) -> Int193R {
        Int193R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt 194"]
    #[inline(always)]
    pub fn int194(&self) -> Int194R {
        Int194R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt 195"]
    #[inline(always)]
    pub fn int195(&self) -> Int195R {
        Int195R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt 196"]
    #[inline(always)]
    pub fn int196(&self) -> Int196R {
        Int196R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt 197"]
    #[inline(always)]
    pub fn int197(&self) -> Int197R {
        Int197R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt 198"]
    #[inline(always)]
    pub fn int198(&self) -> Int198R {
        Int198R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt 199"]
    #[inline(always)]
    pub fn int199(&self) -> Int199R {
        Int199R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt 200"]
    #[inline(always)]
    pub fn int200(&self) -> Int200R {
        Int200R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt 201"]
    #[inline(always)]
    pub fn int201(&self) -> Int201R {
        Int201R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt 202"]
    #[inline(always)]
    pub fn int202(&self) -> Int202R {
        Int202R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt 203"]
    #[inline(always)]
    pub fn int203(&self) -> Int203R {
        Int203R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt 204"]
    #[inline(always)]
    pub fn int204(&self) -> Int204R {
        Int204R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt 205"]
    #[inline(always)]
    pub fn int205(&self) -> Int205R {
        Int205R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt 206"]
    #[inline(always)]
    pub fn int206(&self) -> Int206R {
        Int206R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt 207"]
    #[inline(always)]
    pub fn int207(&self) -> Int207R {
        Int207R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ICFGR48")
            .field("int192", &self.int192())
            .field("int193", &self.int193())
            .field("int194", &self.int194())
            .field("int195", &self.int195())
            .field("int196", &self.int196())
            .field("int197", &self.int197())
            .field("int198", &self.int198())
            .field("int199", &self.int199())
            .field("int200", &self.int200())
            .field("int201", &self.int201())
            .field("int202", &self.int202())
            .field("int203", &self.int203())
            .field("int204", &self.int204())
            .field("int205", &self.int205())
            .field("int206", &self.int206())
            .field("int207", &self.int207())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Interrupt 192"]
    #[inline(always)]
    #[must_use]
    pub fn int192(&mut self) -> Int192W<GicdIcfgr48Spec> {
        Int192W::new(self, 1)
    }
    #[doc = "Bit 3 - Interrupt 193"]
    #[inline(always)]
    #[must_use]
    pub fn int193(&mut self) -> Int193W<GicdIcfgr48Spec> {
        Int193W::new(self, 3)
    }
    #[doc = "Bit 5 - Interrupt 194"]
    #[inline(always)]
    #[must_use]
    pub fn int194(&mut self) -> Int194W<GicdIcfgr48Spec> {
        Int194W::new(self, 5)
    }
    #[doc = "Bit 7 - Interrupt 195"]
    #[inline(always)]
    #[must_use]
    pub fn int195(&mut self) -> Int195W<GicdIcfgr48Spec> {
        Int195W::new(self, 7)
    }
    #[doc = "Bit 9 - Interrupt 196"]
    #[inline(always)]
    #[must_use]
    pub fn int196(&mut self) -> Int196W<GicdIcfgr48Spec> {
        Int196W::new(self, 9)
    }
    #[doc = "Bit 11 - Interrupt 197"]
    #[inline(always)]
    #[must_use]
    pub fn int197(&mut self) -> Int197W<GicdIcfgr48Spec> {
        Int197W::new(self, 11)
    }
    #[doc = "Bit 13 - Interrupt 198"]
    #[inline(always)]
    #[must_use]
    pub fn int198(&mut self) -> Int198W<GicdIcfgr48Spec> {
        Int198W::new(self, 13)
    }
    #[doc = "Bit 15 - Interrupt 199"]
    #[inline(always)]
    #[must_use]
    pub fn int199(&mut self) -> Int199W<GicdIcfgr48Spec> {
        Int199W::new(self, 15)
    }
    #[doc = "Bit 17 - Interrupt 200"]
    #[inline(always)]
    #[must_use]
    pub fn int200(&mut self) -> Int200W<GicdIcfgr48Spec> {
        Int200W::new(self, 17)
    }
    #[doc = "Bit 19 - Interrupt 201"]
    #[inline(always)]
    #[must_use]
    pub fn int201(&mut self) -> Int201W<GicdIcfgr48Spec> {
        Int201W::new(self, 19)
    }
    #[doc = "Bit 21 - Interrupt 202"]
    #[inline(always)]
    #[must_use]
    pub fn int202(&mut self) -> Int202W<GicdIcfgr48Spec> {
        Int202W::new(self, 21)
    }
    #[doc = "Bit 23 - Interrupt 203"]
    #[inline(always)]
    #[must_use]
    pub fn int203(&mut self) -> Int203W<GicdIcfgr48Spec> {
        Int203W::new(self, 23)
    }
    #[doc = "Bit 25 - Interrupt 204"]
    #[inline(always)]
    #[must_use]
    pub fn int204(&mut self) -> Int204W<GicdIcfgr48Spec> {
        Int204W::new(self, 25)
    }
    #[doc = "Bit 27 - Interrupt 205"]
    #[inline(always)]
    #[must_use]
    pub fn int205(&mut self) -> Int205W<GicdIcfgr48Spec> {
        Int205W::new(self, 27)
    }
    #[doc = "Bit 29 - Interrupt 206"]
    #[inline(always)]
    #[must_use]
    pub fn int206(&mut self) -> Int206W<GicdIcfgr48Spec> {
        Int206W::new(self, 29)
    }
    #[doc = "Bit 31 - Interrupt 207"]
    #[inline(always)]
    #[must_use]
    pub fn int207(&mut self) -> Int207W<GicdIcfgr48Spec> {
        Int207W::new(self, 31)
    }
}
#[doc = "Interrupt Configuration 192 - 207\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icfgr48::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icfgr48::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIcfgr48Spec;
impl crate::RegisterSpec for GicdIcfgr48Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_icfgr48::R`](R) reader structure"]
impl crate::Readable for GicdIcfgr48Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_icfgr48::W`](W) writer structure"]
impl crate::Writable for GicdIcfgr48Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ICFGR48 to value 0"]
impl crate::Resettable for GicdIcfgr48Spec {
    const RESET_VALUE: u32 = 0;
}

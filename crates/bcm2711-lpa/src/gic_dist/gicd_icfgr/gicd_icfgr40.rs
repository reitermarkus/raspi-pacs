#[doc = "Register `GICD_ICFGR40` reader"]
pub type R = crate::R<GicdIcfgr40Spec>;
#[doc = "Register `GICD_ICFGR40` writer"]
pub type W = crate::W<GicdIcfgr40Spec>;
#[doc = "Interrupt 160\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int160 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int160> for bool {
    #[inline(always)]
    fn from(variant: Int160) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT160` reader - Interrupt 160"]
pub type Int160R = crate::BitReader<Int160>;
impl Int160R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int160 {
        match self.bits {
            false => Int160::Level,
            true => Int160::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int160::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int160::Edge
    }
}
#[doc = "Field `INT160` writer - Interrupt 160"]
pub type Int160W<'a, REG> = crate::BitWriter<'a, REG, Int160>;
impl<'a, REG> Int160W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int160::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int160::Edge)
    }
}
#[doc = "Interrupt 161\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int161 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int161> for bool {
    #[inline(always)]
    fn from(variant: Int161) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT161` reader - Interrupt 161"]
pub type Int161R = crate::BitReader<Int161>;
impl Int161R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int161 {
        match self.bits {
            false => Int161::Level,
            true => Int161::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int161::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int161::Edge
    }
}
#[doc = "Field `INT161` writer - Interrupt 161"]
pub type Int161W<'a, REG> = crate::BitWriter<'a, REG, Int161>;
impl<'a, REG> Int161W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int161::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int161::Edge)
    }
}
#[doc = "Interrupt 162\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int162 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int162> for bool {
    #[inline(always)]
    fn from(variant: Int162) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT162` reader - Interrupt 162"]
pub type Int162R = crate::BitReader<Int162>;
impl Int162R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int162 {
        match self.bits {
            false => Int162::Level,
            true => Int162::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int162::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int162::Edge
    }
}
#[doc = "Field `INT162` writer - Interrupt 162"]
pub type Int162W<'a, REG> = crate::BitWriter<'a, REG, Int162>;
impl<'a, REG> Int162W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int162::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int162::Edge)
    }
}
#[doc = "Interrupt 163\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int163 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int163> for bool {
    #[inline(always)]
    fn from(variant: Int163) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT163` reader - Interrupt 163"]
pub type Int163R = crate::BitReader<Int163>;
impl Int163R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int163 {
        match self.bits {
            false => Int163::Level,
            true => Int163::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int163::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int163::Edge
    }
}
#[doc = "Field `INT163` writer - Interrupt 163"]
pub type Int163W<'a, REG> = crate::BitWriter<'a, REG, Int163>;
impl<'a, REG> Int163W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int163::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int163::Edge)
    }
}
#[doc = "Interrupt 164\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int164 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int164> for bool {
    #[inline(always)]
    fn from(variant: Int164) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT164` reader - Interrupt 164"]
pub type Int164R = crate::BitReader<Int164>;
impl Int164R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int164 {
        match self.bits {
            false => Int164::Level,
            true => Int164::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int164::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int164::Edge
    }
}
#[doc = "Field `INT164` writer - Interrupt 164"]
pub type Int164W<'a, REG> = crate::BitWriter<'a, REG, Int164>;
impl<'a, REG> Int164W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int164::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int164::Edge)
    }
}
#[doc = "Interrupt 165\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int165 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int165> for bool {
    #[inline(always)]
    fn from(variant: Int165) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT165` reader - Interrupt 165"]
pub type Int165R = crate::BitReader<Int165>;
impl Int165R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int165 {
        match self.bits {
            false => Int165::Level,
            true => Int165::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int165::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int165::Edge
    }
}
#[doc = "Field `INT165` writer - Interrupt 165"]
pub type Int165W<'a, REG> = crate::BitWriter<'a, REG, Int165>;
impl<'a, REG> Int165W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int165::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int165::Edge)
    }
}
#[doc = "Interrupt 166\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int166 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int166> for bool {
    #[inline(always)]
    fn from(variant: Int166) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT166` reader - Interrupt 166"]
pub type Int166R = crate::BitReader<Int166>;
impl Int166R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int166 {
        match self.bits {
            false => Int166::Level,
            true => Int166::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int166::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int166::Edge
    }
}
#[doc = "Field `INT166` writer - Interrupt 166"]
pub type Int166W<'a, REG> = crate::BitWriter<'a, REG, Int166>;
impl<'a, REG> Int166W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int166::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int166::Edge)
    }
}
#[doc = "Interrupt 167\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int167 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int167> for bool {
    #[inline(always)]
    fn from(variant: Int167) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT167` reader - Interrupt 167"]
pub type Int167R = crate::BitReader<Int167>;
impl Int167R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int167 {
        match self.bits {
            false => Int167::Level,
            true => Int167::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int167::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int167::Edge
    }
}
#[doc = "Field `INT167` writer - Interrupt 167"]
pub type Int167W<'a, REG> = crate::BitWriter<'a, REG, Int167>;
impl<'a, REG> Int167W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int167::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int167::Edge)
    }
}
#[doc = "Interrupt 168\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int168 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int168> for bool {
    #[inline(always)]
    fn from(variant: Int168) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT168` reader - Interrupt 168"]
pub type Int168R = crate::BitReader<Int168>;
impl Int168R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int168 {
        match self.bits {
            false => Int168::Level,
            true => Int168::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int168::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int168::Edge
    }
}
#[doc = "Field `INT168` writer - Interrupt 168"]
pub type Int168W<'a, REG> = crate::BitWriter<'a, REG, Int168>;
impl<'a, REG> Int168W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int168::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int168::Edge)
    }
}
#[doc = "Interrupt 169\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int169 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int169> for bool {
    #[inline(always)]
    fn from(variant: Int169) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT169` reader - Interrupt 169"]
pub type Int169R = crate::BitReader<Int169>;
impl Int169R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int169 {
        match self.bits {
            false => Int169::Level,
            true => Int169::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int169::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int169::Edge
    }
}
#[doc = "Field `INT169` writer - Interrupt 169"]
pub type Int169W<'a, REG> = crate::BitWriter<'a, REG, Int169>;
impl<'a, REG> Int169W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int169::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int169::Edge)
    }
}
#[doc = "Interrupt 170\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int170 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int170> for bool {
    #[inline(always)]
    fn from(variant: Int170) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT170` reader - Interrupt 170"]
pub type Int170R = crate::BitReader<Int170>;
impl Int170R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int170 {
        match self.bits {
            false => Int170::Level,
            true => Int170::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int170::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int170::Edge
    }
}
#[doc = "Field `INT170` writer - Interrupt 170"]
pub type Int170W<'a, REG> = crate::BitWriter<'a, REG, Int170>;
impl<'a, REG> Int170W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int170::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int170::Edge)
    }
}
#[doc = "Interrupt 171\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int171 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int171> for bool {
    #[inline(always)]
    fn from(variant: Int171) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT171` reader - Interrupt 171"]
pub type Int171R = crate::BitReader<Int171>;
impl Int171R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int171 {
        match self.bits {
            false => Int171::Level,
            true => Int171::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int171::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int171::Edge
    }
}
#[doc = "Field `INT171` writer - Interrupt 171"]
pub type Int171W<'a, REG> = crate::BitWriter<'a, REG, Int171>;
impl<'a, REG> Int171W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int171::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int171::Edge)
    }
}
#[doc = "Interrupt 172\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int172 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int172> for bool {
    #[inline(always)]
    fn from(variant: Int172) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT172` reader - Interrupt 172"]
pub type Int172R = crate::BitReader<Int172>;
impl Int172R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int172 {
        match self.bits {
            false => Int172::Level,
            true => Int172::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int172::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int172::Edge
    }
}
#[doc = "Field `INT172` writer - Interrupt 172"]
pub type Int172W<'a, REG> = crate::BitWriter<'a, REG, Int172>;
impl<'a, REG> Int172W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int172::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int172::Edge)
    }
}
#[doc = "Interrupt 173\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int173 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int173> for bool {
    #[inline(always)]
    fn from(variant: Int173) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT173` reader - Interrupt 173"]
pub type Int173R = crate::BitReader<Int173>;
impl Int173R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int173 {
        match self.bits {
            false => Int173::Level,
            true => Int173::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int173::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int173::Edge
    }
}
#[doc = "Field `INT173` writer - Interrupt 173"]
pub type Int173W<'a, REG> = crate::BitWriter<'a, REG, Int173>;
impl<'a, REG> Int173W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int173::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int173::Edge)
    }
}
#[doc = "Interrupt 174\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int174 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int174> for bool {
    #[inline(always)]
    fn from(variant: Int174) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT174` reader - Interrupt 174"]
pub type Int174R = crate::BitReader<Int174>;
impl Int174R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int174 {
        match self.bits {
            false => Int174::Level,
            true => Int174::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int174::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int174::Edge
    }
}
#[doc = "Field `INT174` writer - Interrupt 174"]
pub type Int174W<'a, REG> = crate::BitWriter<'a, REG, Int174>;
impl<'a, REG> Int174W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int174::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int174::Edge)
    }
}
#[doc = "Interrupt 175\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int175 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int175> for bool {
    #[inline(always)]
    fn from(variant: Int175) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT175` reader - Interrupt 175"]
pub type Int175R = crate::BitReader<Int175>;
impl Int175R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int175 {
        match self.bits {
            false => Int175::Level,
            true => Int175::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int175::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int175::Edge
    }
}
#[doc = "Field `INT175` writer - Interrupt 175"]
pub type Int175W<'a, REG> = crate::BitWriter<'a, REG, Int175>;
impl<'a, REG> Int175W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int175::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int175::Edge)
    }
}
impl R {
    #[doc = "Bit 1 - Interrupt 160"]
    #[inline(always)]
    pub fn int160(&self) -> Int160R {
        Int160R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt 161"]
    #[inline(always)]
    pub fn int161(&self) -> Int161R {
        Int161R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt 162"]
    #[inline(always)]
    pub fn int162(&self) -> Int162R {
        Int162R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt 163"]
    #[inline(always)]
    pub fn int163(&self) -> Int163R {
        Int163R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt 164"]
    #[inline(always)]
    pub fn int164(&self) -> Int164R {
        Int164R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt 165"]
    #[inline(always)]
    pub fn int165(&self) -> Int165R {
        Int165R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt 166"]
    #[inline(always)]
    pub fn int166(&self) -> Int166R {
        Int166R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt 167"]
    #[inline(always)]
    pub fn int167(&self) -> Int167R {
        Int167R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt 168"]
    #[inline(always)]
    pub fn int168(&self) -> Int168R {
        Int168R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt 169"]
    #[inline(always)]
    pub fn int169(&self) -> Int169R {
        Int169R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt 170"]
    #[inline(always)]
    pub fn int170(&self) -> Int170R {
        Int170R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt 171"]
    #[inline(always)]
    pub fn int171(&self) -> Int171R {
        Int171R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt 172"]
    #[inline(always)]
    pub fn int172(&self) -> Int172R {
        Int172R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt 173"]
    #[inline(always)]
    pub fn int173(&self) -> Int173R {
        Int173R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt 174"]
    #[inline(always)]
    pub fn int174(&self) -> Int174R {
        Int174R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt 175"]
    #[inline(always)]
    pub fn int175(&self) -> Int175R {
        Int175R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ICFGR40")
            .field("int160", &self.int160())
            .field("int161", &self.int161())
            .field("int162", &self.int162())
            .field("int163", &self.int163())
            .field("int164", &self.int164())
            .field("int165", &self.int165())
            .field("int166", &self.int166())
            .field("int167", &self.int167())
            .field("int168", &self.int168())
            .field("int169", &self.int169())
            .field("int170", &self.int170())
            .field("int171", &self.int171())
            .field("int172", &self.int172())
            .field("int173", &self.int173())
            .field("int174", &self.int174())
            .field("int175", &self.int175())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Interrupt 160"]
    #[inline(always)]
    #[must_use]
    pub fn int160(&mut self) -> Int160W<GicdIcfgr40Spec> {
        Int160W::new(self, 1)
    }
    #[doc = "Bit 3 - Interrupt 161"]
    #[inline(always)]
    #[must_use]
    pub fn int161(&mut self) -> Int161W<GicdIcfgr40Spec> {
        Int161W::new(self, 3)
    }
    #[doc = "Bit 5 - Interrupt 162"]
    #[inline(always)]
    #[must_use]
    pub fn int162(&mut self) -> Int162W<GicdIcfgr40Spec> {
        Int162W::new(self, 5)
    }
    #[doc = "Bit 7 - Interrupt 163"]
    #[inline(always)]
    #[must_use]
    pub fn int163(&mut self) -> Int163W<GicdIcfgr40Spec> {
        Int163W::new(self, 7)
    }
    #[doc = "Bit 9 - Interrupt 164"]
    #[inline(always)]
    #[must_use]
    pub fn int164(&mut self) -> Int164W<GicdIcfgr40Spec> {
        Int164W::new(self, 9)
    }
    #[doc = "Bit 11 - Interrupt 165"]
    #[inline(always)]
    #[must_use]
    pub fn int165(&mut self) -> Int165W<GicdIcfgr40Spec> {
        Int165W::new(self, 11)
    }
    #[doc = "Bit 13 - Interrupt 166"]
    #[inline(always)]
    #[must_use]
    pub fn int166(&mut self) -> Int166W<GicdIcfgr40Spec> {
        Int166W::new(self, 13)
    }
    #[doc = "Bit 15 - Interrupt 167"]
    #[inline(always)]
    #[must_use]
    pub fn int167(&mut self) -> Int167W<GicdIcfgr40Spec> {
        Int167W::new(self, 15)
    }
    #[doc = "Bit 17 - Interrupt 168"]
    #[inline(always)]
    #[must_use]
    pub fn int168(&mut self) -> Int168W<GicdIcfgr40Spec> {
        Int168W::new(self, 17)
    }
    #[doc = "Bit 19 - Interrupt 169"]
    #[inline(always)]
    #[must_use]
    pub fn int169(&mut self) -> Int169W<GicdIcfgr40Spec> {
        Int169W::new(self, 19)
    }
    #[doc = "Bit 21 - Interrupt 170"]
    #[inline(always)]
    #[must_use]
    pub fn int170(&mut self) -> Int170W<GicdIcfgr40Spec> {
        Int170W::new(self, 21)
    }
    #[doc = "Bit 23 - Interrupt 171"]
    #[inline(always)]
    #[must_use]
    pub fn int171(&mut self) -> Int171W<GicdIcfgr40Spec> {
        Int171W::new(self, 23)
    }
    #[doc = "Bit 25 - Interrupt 172"]
    #[inline(always)]
    #[must_use]
    pub fn int172(&mut self) -> Int172W<GicdIcfgr40Spec> {
        Int172W::new(self, 25)
    }
    #[doc = "Bit 27 - Interrupt 173"]
    #[inline(always)]
    #[must_use]
    pub fn int173(&mut self) -> Int173W<GicdIcfgr40Spec> {
        Int173W::new(self, 27)
    }
    #[doc = "Bit 29 - Interrupt 174"]
    #[inline(always)]
    #[must_use]
    pub fn int174(&mut self) -> Int174W<GicdIcfgr40Spec> {
        Int174W::new(self, 29)
    }
    #[doc = "Bit 31 - Interrupt 175"]
    #[inline(always)]
    #[must_use]
    pub fn int175(&mut self) -> Int175W<GicdIcfgr40Spec> {
        Int175W::new(self, 31)
    }
}
#[doc = "Interrupt Configuration 160 - 175\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icfgr40::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icfgr40::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIcfgr40Spec;
impl crate::RegisterSpec for GicdIcfgr40Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_icfgr40::R`](R) reader structure"]
impl crate::Readable for GicdIcfgr40Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_icfgr40::W`](W) writer structure"]
impl crate::Writable for GicdIcfgr40Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ICFGR40 to value 0"]
impl crate::Resettable for GicdIcfgr40Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `GICD_ICFGR44` reader"]
pub type R = crate::R<GicdIcfgr44Spec>;
#[doc = "Register `GICD_ICFGR44` writer"]
pub type W = crate::W<GicdIcfgr44Spec>;
#[doc = "Interrupt 176\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int176 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int176> for bool {
    #[inline(always)]
    fn from(variant: Int176) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT176` reader - Interrupt 176"]
pub type Int176R = crate::BitReader<Int176>;
impl Int176R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int176 {
        match self.bits {
            false => Int176::Level,
            true => Int176::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int176::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int176::Edge
    }
}
#[doc = "Field `INT176` writer - Interrupt 176"]
pub type Int176W<'a, REG> = crate::BitWriter<'a, REG, Int176>;
impl<'a, REG> Int176W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int176::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int176::Edge)
    }
}
#[doc = "Interrupt 177\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int177 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int177> for bool {
    #[inline(always)]
    fn from(variant: Int177) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT177` reader - Interrupt 177"]
pub type Int177R = crate::BitReader<Int177>;
impl Int177R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int177 {
        match self.bits {
            false => Int177::Level,
            true => Int177::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int177::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int177::Edge
    }
}
#[doc = "Field `INT177` writer - Interrupt 177"]
pub type Int177W<'a, REG> = crate::BitWriter<'a, REG, Int177>;
impl<'a, REG> Int177W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int177::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int177::Edge)
    }
}
#[doc = "Interrupt 178\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int178 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int178> for bool {
    #[inline(always)]
    fn from(variant: Int178) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT178` reader - Interrupt 178"]
pub type Int178R = crate::BitReader<Int178>;
impl Int178R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int178 {
        match self.bits {
            false => Int178::Level,
            true => Int178::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int178::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int178::Edge
    }
}
#[doc = "Field `INT178` writer - Interrupt 178"]
pub type Int178W<'a, REG> = crate::BitWriter<'a, REG, Int178>;
impl<'a, REG> Int178W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int178::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int178::Edge)
    }
}
#[doc = "Interrupt 179\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int179 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int179> for bool {
    #[inline(always)]
    fn from(variant: Int179) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT179` reader - Interrupt 179"]
pub type Int179R = crate::BitReader<Int179>;
impl Int179R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int179 {
        match self.bits {
            false => Int179::Level,
            true => Int179::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int179::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int179::Edge
    }
}
#[doc = "Field `INT179` writer - Interrupt 179"]
pub type Int179W<'a, REG> = crate::BitWriter<'a, REG, Int179>;
impl<'a, REG> Int179W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int179::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int179::Edge)
    }
}
#[doc = "Interrupt 180\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int180 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int180> for bool {
    #[inline(always)]
    fn from(variant: Int180) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT180` reader - Interrupt 180"]
pub type Int180R = crate::BitReader<Int180>;
impl Int180R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int180 {
        match self.bits {
            false => Int180::Level,
            true => Int180::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int180::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int180::Edge
    }
}
#[doc = "Field `INT180` writer - Interrupt 180"]
pub type Int180W<'a, REG> = crate::BitWriter<'a, REG, Int180>;
impl<'a, REG> Int180W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int180::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int180::Edge)
    }
}
#[doc = "Interrupt 181\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int181 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int181> for bool {
    #[inline(always)]
    fn from(variant: Int181) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT181` reader - Interrupt 181"]
pub type Int181R = crate::BitReader<Int181>;
impl Int181R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int181 {
        match self.bits {
            false => Int181::Level,
            true => Int181::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int181::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int181::Edge
    }
}
#[doc = "Field `INT181` writer - Interrupt 181"]
pub type Int181W<'a, REG> = crate::BitWriter<'a, REG, Int181>;
impl<'a, REG> Int181W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int181::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int181::Edge)
    }
}
#[doc = "Interrupt 182\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int182 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int182> for bool {
    #[inline(always)]
    fn from(variant: Int182) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT182` reader - Interrupt 182"]
pub type Int182R = crate::BitReader<Int182>;
impl Int182R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int182 {
        match self.bits {
            false => Int182::Level,
            true => Int182::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int182::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int182::Edge
    }
}
#[doc = "Field `INT182` writer - Interrupt 182"]
pub type Int182W<'a, REG> = crate::BitWriter<'a, REG, Int182>;
impl<'a, REG> Int182W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int182::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int182::Edge)
    }
}
#[doc = "Interrupt 183\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int183 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int183> for bool {
    #[inline(always)]
    fn from(variant: Int183) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT183` reader - Interrupt 183"]
pub type Int183R = crate::BitReader<Int183>;
impl Int183R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int183 {
        match self.bits {
            false => Int183::Level,
            true => Int183::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int183::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int183::Edge
    }
}
#[doc = "Field `INT183` writer - Interrupt 183"]
pub type Int183W<'a, REG> = crate::BitWriter<'a, REG, Int183>;
impl<'a, REG> Int183W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int183::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int183::Edge)
    }
}
#[doc = "Interrupt 184\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int184 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int184> for bool {
    #[inline(always)]
    fn from(variant: Int184) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT184` reader - Interrupt 184"]
pub type Int184R = crate::BitReader<Int184>;
impl Int184R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int184 {
        match self.bits {
            false => Int184::Level,
            true => Int184::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int184::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int184::Edge
    }
}
#[doc = "Field `INT184` writer - Interrupt 184"]
pub type Int184W<'a, REG> = crate::BitWriter<'a, REG, Int184>;
impl<'a, REG> Int184W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int184::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int184::Edge)
    }
}
#[doc = "Interrupt 185\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int185 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int185> for bool {
    #[inline(always)]
    fn from(variant: Int185) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT185` reader - Interrupt 185"]
pub type Int185R = crate::BitReader<Int185>;
impl Int185R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int185 {
        match self.bits {
            false => Int185::Level,
            true => Int185::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int185::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int185::Edge
    }
}
#[doc = "Field `INT185` writer - Interrupt 185"]
pub type Int185W<'a, REG> = crate::BitWriter<'a, REG, Int185>;
impl<'a, REG> Int185W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int185::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int185::Edge)
    }
}
#[doc = "Interrupt 186\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int186 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int186> for bool {
    #[inline(always)]
    fn from(variant: Int186) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT186` reader - Interrupt 186"]
pub type Int186R = crate::BitReader<Int186>;
impl Int186R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int186 {
        match self.bits {
            false => Int186::Level,
            true => Int186::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int186::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int186::Edge
    }
}
#[doc = "Field `INT186` writer - Interrupt 186"]
pub type Int186W<'a, REG> = crate::BitWriter<'a, REG, Int186>;
impl<'a, REG> Int186W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int186::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int186::Edge)
    }
}
#[doc = "Interrupt 187\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int187 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int187> for bool {
    #[inline(always)]
    fn from(variant: Int187) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT187` reader - Interrupt 187"]
pub type Int187R = crate::BitReader<Int187>;
impl Int187R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int187 {
        match self.bits {
            false => Int187::Level,
            true => Int187::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int187::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int187::Edge
    }
}
#[doc = "Field `INT187` writer - Interrupt 187"]
pub type Int187W<'a, REG> = crate::BitWriter<'a, REG, Int187>;
impl<'a, REG> Int187W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int187::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int187::Edge)
    }
}
#[doc = "Interrupt 188\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int188 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int188> for bool {
    #[inline(always)]
    fn from(variant: Int188) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT188` reader - Interrupt 188"]
pub type Int188R = crate::BitReader<Int188>;
impl Int188R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int188 {
        match self.bits {
            false => Int188::Level,
            true => Int188::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int188::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int188::Edge
    }
}
#[doc = "Field `INT188` writer - Interrupt 188"]
pub type Int188W<'a, REG> = crate::BitWriter<'a, REG, Int188>;
impl<'a, REG> Int188W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int188::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int188::Edge)
    }
}
#[doc = "Interrupt 189\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int189 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int189> for bool {
    #[inline(always)]
    fn from(variant: Int189) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT189` reader - Interrupt 189"]
pub type Int189R = crate::BitReader<Int189>;
impl Int189R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int189 {
        match self.bits {
            false => Int189::Level,
            true => Int189::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int189::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int189::Edge
    }
}
#[doc = "Field `INT189` writer - Interrupt 189"]
pub type Int189W<'a, REG> = crate::BitWriter<'a, REG, Int189>;
impl<'a, REG> Int189W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int189::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int189::Edge)
    }
}
#[doc = "Interrupt 190\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int190 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int190> for bool {
    #[inline(always)]
    fn from(variant: Int190) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT190` reader - Interrupt 190"]
pub type Int190R = crate::BitReader<Int190>;
impl Int190R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int190 {
        match self.bits {
            false => Int190::Level,
            true => Int190::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int190::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int190::Edge
    }
}
#[doc = "Field `INT190` writer - Interrupt 190"]
pub type Int190W<'a, REG> = crate::BitWriter<'a, REG, Int190>;
impl<'a, REG> Int190W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int190::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int190::Edge)
    }
}
#[doc = "Interrupt 191\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Int191 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Int191> for bool {
    #[inline(always)]
    fn from(variant: Int191) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT191` reader - Interrupt 191"]
pub type Int191R = crate::BitReader<Int191>;
impl Int191R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Int191 {
        match self.bits {
            false => Int191::Level,
            true => Int191::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Int191::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Int191::Edge
    }
}
#[doc = "Field `INT191` writer - Interrupt 191"]
pub type Int191W<'a, REG> = crate::BitWriter<'a, REG, Int191>;
impl<'a, REG> Int191W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Int191::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Int191::Edge)
    }
}
impl R {
    #[doc = "Bit 1 - Interrupt 176"]
    #[inline(always)]
    pub fn int176(&self) -> Int176R {
        Int176R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt 177"]
    #[inline(always)]
    pub fn int177(&self) -> Int177R {
        Int177R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt 178"]
    #[inline(always)]
    pub fn int178(&self) -> Int178R {
        Int178R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt 179"]
    #[inline(always)]
    pub fn int179(&self) -> Int179R {
        Int179R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt 180"]
    #[inline(always)]
    pub fn int180(&self) -> Int180R {
        Int180R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt 181"]
    #[inline(always)]
    pub fn int181(&self) -> Int181R {
        Int181R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt 182"]
    #[inline(always)]
    pub fn int182(&self) -> Int182R {
        Int182R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt 183"]
    #[inline(always)]
    pub fn int183(&self) -> Int183R {
        Int183R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt 184"]
    #[inline(always)]
    pub fn int184(&self) -> Int184R {
        Int184R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt 185"]
    #[inline(always)]
    pub fn int185(&self) -> Int185R {
        Int185R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt 186"]
    #[inline(always)]
    pub fn int186(&self) -> Int186R {
        Int186R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt 187"]
    #[inline(always)]
    pub fn int187(&self) -> Int187R {
        Int187R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt 188"]
    #[inline(always)]
    pub fn int188(&self) -> Int188R {
        Int188R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt 189"]
    #[inline(always)]
    pub fn int189(&self) -> Int189R {
        Int189R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt 190"]
    #[inline(always)]
    pub fn int190(&self) -> Int190R {
        Int190R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt 191"]
    #[inline(always)]
    pub fn int191(&self) -> Int191R {
        Int191R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ICFGR44")
            .field("int176", &self.int176())
            .field("int177", &self.int177())
            .field("int178", &self.int178())
            .field("int179", &self.int179())
            .field("int180", &self.int180())
            .field("int181", &self.int181())
            .field("int182", &self.int182())
            .field("int183", &self.int183())
            .field("int184", &self.int184())
            .field("int185", &self.int185())
            .field("int186", &self.int186())
            .field("int187", &self.int187())
            .field("int188", &self.int188())
            .field("int189", &self.int189())
            .field("int190", &self.int190())
            .field("int191", &self.int191())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Interrupt 176"]
    #[inline(always)]
    #[must_use]
    pub fn int176(&mut self) -> Int176W<GicdIcfgr44Spec> {
        Int176W::new(self, 1)
    }
    #[doc = "Bit 3 - Interrupt 177"]
    #[inline(always)]
    #[must_use]
    pub fn int177(&mut self) -> Int177W<GicdIcfgr44Spec> {
        Int177W::new(self, 3)
    }
    #[doc = "Bit 5 - Interrupt 178"]
    #[inline(always)]
    #[must_use]
    pub fn int178(&mut self) -> Int178W<GicdIcfgr44Spec> {
        Int178W::new(self, 5)
    }
    #[doc = "Bit 7 - Interrupt 179"]
    #[inline(always)]
    #[must_use]
    pub fn int179(&mut self) -> Int179W<GicdIcfgr44Spec> {
        Int179W::new(self, 7)
    }
    #[doc = "Bit 9 - Interrupt 180"]
    #[inline(always)]
    #[must_use]
    pub fn int180(&mut self) -> Int180W<GicdIcfgr44Spec> {
        Int180W::new(self, 9)
    }
    #[doc = "Bit 11 - Interrupt 181"]
    #[inline(always)]
    #[must_use]
    pub fn int181(&mut self) -> Int181W<GicdIcfgr44Spec> {
        Int181W::new(self, 11)
    }
    #[doc = "Bit 13 - Interrupt 182"]
    #[inline(always)]
    #[must_use]
    pub fn int182(&mut self) -> Int182W<GicdIcfgr44Spec> {
        Int182W::new(self, 13)
    }
    #[doc = "Bit 15 - Interrupt 183"]
    #[inline(always)]
    #[must_use]
    pub fn int183(&mut self) -> Int183W<GicdIcfgr44Spec> {
        Int183W::new(self, 15)
    }
    #[doc = "Bit 17 - Interrupt 184"]
    #[inline(always)]
    #[must_use]
    pub fn int184(&mut self) -> Int184W<GicdIcfgr44Spec> {
        Int184W::new(self, 17)
    }
    #[doc = "Bit 19 - Interrupt 185"]
    #[inline(always)]
    #[must_use]
    pub fn int185(&mut self) -> Int185W<GicdIcfgr44Spec> {
        Int185W::new(self, 19)
    }
    #[doc = "Bit 21 - Interrupt 186"]
    #[inline(always)]
    #[must_use]
    pub fn int186(&mut self) -> Int186W<GicdIcfgr44Spec> {
        Int186W::new(self, 21)
    }
    #[doc = "Bit 23 - Interrupt 187"]
    #[inline(always)]
    #[must_use]
    pub fn int187(&mut self) -> Int187W<GicdIcfgr44Spec> {
        Int187W::new(self, 23)
    }
    #[doc = "Bit 25 - Interrupt 188"]
    #[inline(always)]
    #[must_use]
    pub fn int188(&mut self) -> Int188W<GicdIcfgr44Spec> {
        Int188W::new(self, 25)
    }
    #[doc = "Bit 27 - Interrupt 189"]
    #[inline(always)]
    #[must_use]
    pub fn int189(&mut self) -> Int189W<GicdIcfgr44Spec> {
        Int189W::new(self, 27)
    }
    #[doc = "Bit 29 - Interrupt 190"]
    #[inline(always)]
    #[must_use]
    pub fn int190(&mut self) -> Int190W<GicdIcfgr44Spec> {
        Int190W::new(self, 29)
    }
    #[doc = "Bit 31 - Interrupt 191"]
    #[inline(always)]
    #[must_use]
    pub fn int191(&mut self) -> Int191W<GicdIcfgr44Spec> {
        Int191W::new(self, 31)
    }
}
#[doc = "Interrupt Configuration 176 - 191\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icfgr44::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icfgr44::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIcfgr44Spec;
impl crate::RegisterSpec for GicdIcfgr44Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_icfgr44::R`](R) reader structure"]
impl crate::Readable for GicdIcfgr44Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_icfgr44::W`](W) writer structure"]
impl crate::Writable for GicdIcfgr44Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ICFGR44 to value 0"]
impl crate::Resettable for GicdIcfgr44Spec {
    const RESET_VALUE: u32 = 0;
}

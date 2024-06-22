#[doc = "Register `GICD_ICFGR16` reader"]
pub type R = crate::R<GicdIcfgr16Spec>;
#[doc = "Register `GICD_ICFGR16` writer"]
pub type W = crate::W<GicdIcfgr16Spec>;
#[doc = "ARMC Timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Timer> for bool {
    #[inline(always)]
    fn from(variant: Timer) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER` reader - ARMC Timer"]
pub type TimerR = crate::BitReader<Timer>;
impl TimerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer {
        match self.bits {
            false => Timer::Level,
            true => Timer::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Timer::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Timer::Edge
    }
}
#[doc = "Field `TIMER` writer - ARMC Timer"]
pub type TimerW<'a, REG> = crate::BitWriter<'a, REG, Timer>;
impl<'a, REG> TimerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Timer::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Timer::Edge)
    }
}
#[doc = "Mailbox\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mailbox {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Mailbox> for bool {
    #[inline(always)]
    fn from(variant: Mailbox) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAILBOX` reader - Mailbox"]
pub type MailboxR = crate::BitReader<Mailbox>;
impl MailboxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mailbox {
        match self.bits {
            false => Mailbox::Level,
            true => Mailbox::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Mailbox::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Mailbox::Edge
    }
}
#[doc = "Field `MAILBOX` writer - Mailbox"]
pub type MailboxW<'a, REG> = crate::BitWriter<'a, REG, Mailbox>;
impl<'a, REG> MailboxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Mailbox::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Mailbox::Edge)
    }
}
#[doc = "Doorbell 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doorbell0 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Doorbell0> for bool {
    #[inline(always)]
    fn from(variant: Doorbell0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOORBELL0` reader - Doorbell 0"]
pub type Doorbell0R = crate::BitReader<Doorbell0>;
impl Doorbell0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doorbell0 {
        match self.bits {
            false => Doorbell0::Level,
            true => Doorbell0::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Doorbell0::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Doorbell0::Edge
    }
}
#[doc = "Field `DOORBELL0` writer - Doorbell 0"]
pub type Doorbell0W<'a, REG> = crate::BitWriter<'a, REG, Doorbell0>;
impl<'a, REG> Doorbell0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Doorbell0::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Doorbell0::Edge)
    }
}
#[doc = "Doorbell 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doorbell1 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Doorbell1> for bool {
    #[inline(always)]
    fn from(variant: Doorbell1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOORBELL1` reader - Doorbell 1"]
pub type Doorbell1R = crate::BitReader<Doorbell1>;
impl Doorbell1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doorbell1 {
        match self.bits {
            false => Doorbell1::Level,
            true => Doorbell1::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Doorbell1::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Doorbell1::Edge
    }
}
#[doc = "Field `DOORBELL1` writer - Doorbell 1"]
pub type Doorbell1W<'a, REG> = crate::BitWriter<'a, REG, Doorbell1>;
impl<'a, REG> Doorbell1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Doorbell1::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Doorbell1::Edge)
    }
}
#[doc = "VPU0 halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vpu0Halted {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Vpu0Halted> for bool {
    #[inline(always)]
    fn from(variant: Vpu0Halted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VPU0_HALTED` reader - VPU0 halted"]
pub type Vpu0HaltedR = crate::BitReader<Vpu0Halted>;
impl Vpu0HaltedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vpu0Halted {
        match self.bits {
            false => Vpu0Halted::Level,
            true => Vpu0Halted::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Vpu0Halted::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Vpu0Halted::Edge
    }
}
#[doc = "Field `VPU0_HALTED` writer - VPU0 halted"]
pub type Vpu0HaltedW<'a, REG> = crate::BitWriter<'a, REG, Vpu0Halted>;
impl<'a, REG> Vpu0HaltedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Vpu0Halted::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Vpu0Halted::Edge)
    }
}
#[doc = "VPU1 halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vpu1Halted {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Vpu1Halted> for bool {
    #[inline(always)]
    fn from(variant: Vpu1Halted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VPU1_HALTED` reader - VPU1 halted"]
pub type Vpu1HaltedR = crate::BitReader<Vpu1Halted>;
impl Vpu1HaltedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vpu1Halted {
        match self.bits {
            false => Vpu1Halted::Level,
            true => Vpu1Halted::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Vpu1Halted::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Vpu1Halted::Edge
    }
}
#[doc = "Field `VPU1_HALTED` writer - VPU1 halted"]
pub type Vpu1HaltedW<'a, REG> = crate::BitWriter<'a, REG, Vpu1Halted>;
impl<'a, REG> Vpu1HaltedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Vpu1Halted::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Vpu1Halted::Edge)
    }
}
#[doc = "ARM address error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArmAddressError {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<ArmAddressError> for bool {
    #[inline(always)]
    fn from(variant: ArmAddressError) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARM_ADDRESS_ERROR` reader - ARM address error"]
pub type ArmAddressErrorR = crate::BitReader<ArmAddressError>;
impl ArmAddressErrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ArmAddressError {
        match self.bits {
            false => ArmAddressError::Level,
            true => ArmAddressError::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == ArmAddressError::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == ArmAddressError::Edge
    }
}
#[doc = "Field `ARM_ADDRESS_ERROR` writer - ARM address error"]
pub type ArmAddressErrorW<'a, REG> = crate::BitWriter<'a, REG, ArmAddressError>;
impl<'a, REG> ArmAddressErrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(ArmAddressError::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(ArmAddressError::Edge)
    }
}
#[doc = "ARM AXI error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArmAxiError {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<ArmAxiError> for bool {
    #[inline(always)]
    fn from(variant: ArmAxiError) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARM_AXI_ERROR` reader - ARM AXI error"]
pub type ArmAxiErrorR = crate::BitReader<ArmAxiError>;
impl ArmAxiErrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ArmAxiError {
        match self.bits {
            false => ArmAxiError::Level,
            true => ArmAxiError::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == ArmAxiError::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == ArmAxiError::Edge
    }
}
#[doc = "Field `ARM_AXI_ERROR` writer - ARM AXI error"]
pub type ArmAxiErrorW<'a, REG> = crate::BitWriter<'a, REG, ArmAxiError>;
impl<'a, REG> ArmAxiErrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(ArmAxiError::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(ArmAxiError::Edge)
    }
}
#[doc = "Software interrupt 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swi0 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Swi0> for bool {
    #[inline(always)]
    fn from(variant: Swi0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI0` reader - Software interrupt 0"]
pub type Swi0R = crate::BitReader<Swi0>;
impl Swi0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swi0 {
        match self.bits {
            false => Swi0::Level,
            true => Swi0::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Swi0::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Swi0::Edge
    }
}
#[doc = "Field `SWI0` writer - Software interrupt 0"]
pub type Swi0W<'a, REG> = crate::BitWriter<'a, REG, Swi0>;
impl<'a, REG> Swi0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Swi0::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Swi0::Edge)
    }
}
#[doc = "Software interrupt 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swi1 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Swi1> for bool {
    #[inline(always)]
    fn from(variant: Swi1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI1` reader - Software interrupt 1"]
pub type Swi1R = crate::BitReader<Swi1>;
impl Swi1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swi1 {
        match self.bits {
            false => Swi1::Level,
            true => Swi1::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Swi1::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Swi1::Edge
    }
}
#[doc = "Field `SWI1` writer - Software interrupt 1"]
pub type Swi1W<'a, REG> = crate::BitWriter<'a, REG, Swi1>;
impl<'a, REG> Swi1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Swi1::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Swi1::Edge)
    }
}
#[doc = "Software interrupt 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swi2 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Swi2> for bool {
    #[inline(always)]
    fn from(variant: Swi2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI2` reader - Software interrupt 2"]
pub type Swi2R = crate::BitReader<Swi2>;
impl Swi2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swi2 {
        match self.bits {
            false => Swi2::Level,
            true => Swi2::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Swi2::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Swi2::Edge
    }
}
#[doc = "Field `SWI2` writer - Software interrupt 2"]
pub type Swi2W<'a, REG> = crate::BitWriter<'a, REG, Swi2>;
impl<'a, REG> Swi2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Swi2::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Swi2::Edge)
    }
}
#[doc = "Software interrupt 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swi3 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Swi3> for bool {
    #[inline(always)]
    fn from(variant: Swi3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI3` reader - Software interrupt 3"]
pub type Swi3R = crate::BitReader<Swi3>;
impl Swi3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swi3 {
        match self.bits {
            false => Swi3::Level,
            true => Swi3::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Swi3::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Swi3::Edge
    }
}
#[doc = "Field `SWI3` writer - Software interrupt 3"]
pub type Swi3W<'a, REG> = crate::BitWriter<'a, REG, Swi3>;
impl<'a, REG> Swi3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Swi3::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Swi3::Edge)
    }
}
#[doc = "Software interrupt 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swi4 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Swi4> for bool {
    #[inline(always)]
    fn from(variant: Swi4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI4` reader - Software interrupt 4"]
pub type Swi4R = crate::BitReader<Swi4>;
impl Swi4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swi4 {
        match self.bits {
            false => Swi4::Level,
            true => Swi4::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Swi4::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Swi4::Edge
    }
}
#[doc = "Field `SWI4` writer - Software interrupt 4"]
pub type Swi4W<'a, REG> = crate::BitWriter<'a, REG, Swi4>;
impl<'a, REG> Swi4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Swi4::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Swi4::Edge)
    }
}
#[doc = "Software interrupt 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swi5 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Swi5> for bool {
    #[inline(always)]
    fn from(variant: Swi5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI5` reader - Software interrupt 5"]
pub type Swi5R = crate::BitReader<Swi5>;
impl Swi5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swi5 {
        match self.bits {
            false => Swi5::Level,
            true => Swi5::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Swi5::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Swi5::Edge
    }
}
#[doc = "Field `SWI5` writer - Software interrupt 5"]
pub type Swi5W<'a, REG> = crate::BitWriter<'a, REG, Swi5>;
impl<'a, REG> Swi5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Swi5::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Swi5::Edge)
    }
}
#[doc = "Software interrupt 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swi6 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Swi6> for bool {
    #[inline(always)]
    fn from(variant: Swi6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI6` reader - Software interrupt 6"]
pub type Swi6R = crate::BitReader<Swi6>;
impl Swi6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swi6 {
        match self.bits {
            false => Swi6::Level,
            true => Swi6::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Swi6::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Swi6::Edge
    }
}
#[doc = "Field `SWI6` writer - Software interrupt 6"]
pub type Swi6W<'a, REG> = crate::BitWriter<'a, REG, Swi6>;
impl<'a, REG> Swi6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Swi6::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Swi6::Edge)
    }
}
#[doc = "Software interrupt 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swi7 {
    #[doc = "0: Level sensitive"]
    Level = 0,
    #[doc = "1: Edge triggered"]
    Edge = 1,
}
impl From<Swi7> for bool {
    #[inline(always)]
    fn from(variant: Swi7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI7` reader - Software interrupt 7"]
pub type Swi7R = crate::BitReader<Swi7>;
impl Swi7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swi7 {
        match self.bits {
            false => Swi7::Level,
            true => Swi7::Edge,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Swi7::Level
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Swi7::Edge
    }
}
#[doc = "Field `SWI7` writer - Software interrupt 7"]
pub type Swi7W<'a, REG> = crate::BitWriter<'a, REG, Swi7>;
impl<'a, REG> Swi7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Swi7::Level)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Swi7::Edge)
    }
}
impl R {
    #[doc = "Bit 1 - ARMC Timer"]
    #[inline(always)]
    pub fn timer(&self) -> TimerR {
        TimerR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Mailbox"]
    #[inline(always)]
    pub fn mailbox(&self) -> MailboxR {
        MailboxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Doorbell 0"]
    #[inline(always)]
    pub fn doorbell0(&self) -> Doorbell0R {
        Doorbell0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Doorbell 1"]
    #[inline(always)]
    pub fn doorbell1(&self) -> Doorbell1R {
        Doorbell1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - VPU0 halted"]
    #[inline(always)]
    pub fn vpu0_halted(&self) -> Vpu0HaltedR {
        Vpu0HaltedR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - VPU1 halted"]
    #[inline(always)]
    pub fn vpu1_halted(&self) -> Vpu1HaltedR {
        Vpu1HaltedR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - ARM address error"]
    #[inline(always)]
    pub fn arm_address_error(&self) -> ArmAddressErrorR {
        ArmAddressErrorR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - ARM AXI error"]
    #[inline(always)]
    pub fn arm_axi_error(&self) -> ArmAxiErrorR {
        ArmAxiErrorR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Software interrupt 0"]
    #[inline(always)]
    pub fn swi0(&self) -> Swi0R {
        Swi0R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Software interrupt 1"]
    #[inline(always)]
    pub fn swi1(&self) -> Swi1R {
        Swi1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Software interrupt 2"]
    #[inline(always)]
    pub fn swi2(&self) -> Swi2R {
        Swi2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Software interrupt 3"]
    #[inline(always)]
    pub fn swi3(&self) -> Swi3R {
        Swi3R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Software interrupt 4"]
    #[inline(always)]
    pub fn swi4(&self) -> Swi4R {
        Swi4R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Software interrupt 5"]
    #[inline(always)]
    pub fn swi5(&self) -> Swi5R {
        Swi5R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - Software interrupt 6"]
    #[inline(always)]
    pub fn swi6(&self) -> Swi6R {
        Swi6R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Software interrupt 7"]
    #[inline(always)]
    pub fn swi7(&self) -> Swi7R {
        Swi7R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ICFGR16")
            .field("timer", &self.timer())
            .field("mailbox", &self.mailbox())
            .field("doorbell0", &self.doorbell0())
            .field("doorbell1", &self.doorbell1())
            .field("vpu0_halted", &self.vpu0_halted())
            .field("vpu1_halted", &self.vpu1_halted())
            .field("arm_address_error", &self.arm_address_error())
            .field("arm_axi_error", &self.arm_axi_error())
            .field("swi0", &self.swi0())
            .field("swi1", &self.swi1())
            .field("swi2", &self.swi2())
            .field("swi3", &self.swi3())
            .field("swi4", &self.swi4())
            .field("swi5", &self.swi5())
            .field("swi6", &self.swi6())
            .field("swi7", &self.swi7())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - ARMC Timer"]
    #[inline(always)]
    #[must_use]
    pub fn timer(&mut self) -> TimerW<GicdIcfgr16Spec> {
        TimerW::new(self, 1)
    }
    #[doc = "Bit 3 - Mailbox"]
    #[inline(always)]
    #[must_use]
    pub fn mailbox(&mut self) -> MailboxW<GicdIcfgr16Spec> {
        MailboxW::new(self, 3)
    }
    #[doc = "Bit 5 - Doorbell 0"]
    #[inline(always)]
    #[must_use]
    pub fn doorbell0(&mut self) -> Doorbell0W<GicdIcfgr16Spec> {
        Doorbell0W::new(self, 5)
    }
    #[doc = "Bit 7 - Doorbell 1"]
    #[inline(always)]
    #[must_use]
    pub fn doorbell1(&mut self) -> Doorbell1W<GicdIcfgr16Spec> {
        Doorbell1W::new(self, 7)
    }
    #[doc = "Bit 9 - VPU0 halted"]
    #[inline(always)]
    #[must_use]
    pub fn vpu0_halted(&mut self) -> Vpu0HaltedW<GicdIcfgr16Spec> {
        Vpu0HaltedW::new(self, 9)
    }
    #[doc = "Bit 11 - VPU1 halted"]
    #[inline(always)]
    #[must_use]
    pub fn vpu1_halted(&mut self) -> Vpu1HaltedW<GicdIcfgr16Spec> {
        Vpu1HaltedW::new(self, 11)
    }
    #[doc = "Bit 13 - ARM address error"]
    #[inline(always)]
    #[must_use]
    pub fn arm_address_error(&mut self) -> ArmAddressErrorW<GicdIcfgr16Spec> {
        ArmAddressErrorW::new(self, 13)
    }
    #[doc = "Bit 15 - ARM AXI error"]
    #[inline(always)]
    #[must_use]
    pub fn arm_axi_error(&mut self) -> ArmAxiErrorW<GicdIcfgr16Spec> {
        ArmAxiErrorW::new(self, 15)
    }
    #[doc = "Bit 17 - Software interrupt 0"]
    #[inline(always)]
    #[must_use]
    pub fn swi0(&mut self) -> Swi0W<GicdIcfgr16Spec> {
        Swi0W::new(self, 17)
    }
    #[doc = "Bit 19 - Software interrupt 1"]
    #[inline(always)]
    #[must_use]
    pub fn swi1(&mut self) -> Swi1W<GicdIcfgr16Spec> {
        Swi1W::new(self, 19)
    }
    #[doc = "Bit 21 - Software interrupt 2"]
    #[inline(always)]
    #[must_use]
    pub fn swi2(&mut self) -> Swi2W<GicdIcfgr16Spec> {
        Swi2W::new(self, 21)
    }
    #[doc = "Bit 23 - Software interrupt 3"]
    #[inline(always)]
    #[must_use]
    pub fn swi3(&mut self) -> Swi3W<GicdIcfgr16Spec> {
        Swi3W::new(self, 23)
    }
    #[doc = "Bit 25 - Software interrupt 4"]
    #[inline(always)]
    #[must_use]
    pub fn swi4(&mut self) -> Swi4W<GicdIcfgr16Spec> {
        Swi4W::new(self, 25)
    }
    #[doc = "Bit 27 - Software interrupt 5"]
    #[inline(always)]
    #[must_use]
    pub fn swi5(&mut self) -> Swi5W<GicdIcfgr16Spec> {
        Swi5W::new(self, 27)
    }
    #[doc = "Bit 29 - Software interrupt 6"]
    #[inline(always)]
    #[must_use]
    pub fn swi6(&mut self) -> Swi6W<GicdIcfgr16Spec> {
        Swi6W::new(self, 29)
    }
    #[doc = "Bit 31 - Software interrupt 7"]
    #[inline(always)]
    #[must_use]
    pub fn swi7(&mut self) -> Swi7W<GicdIcfgr16Spec> {
        Swi7W::new(self, 31)
    }
}
#[doc = "Interrupt Configuration 64 - 79\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_icfgr16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icfgr16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIcfgr16Spec;
impl crate::RegisterSpec for GicdIcfgr16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_icfgr16::R`](R) reader structure"]
impl crate::Readable for GicdIcfgr16Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_icfgr16::W`](W) writer structure"]
impl crate::Writable for GicdIcfgr16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ICFGR16 to value 0"]
impl crate::Resettable for GicdIcfgr16Spec {
    const RESET_VALUE: u32 = 0;
}

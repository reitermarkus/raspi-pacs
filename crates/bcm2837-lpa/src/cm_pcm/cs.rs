#[doc = "Register `CS` reader"]
pub type R = crate::R<CsSpec>;
#[doc = "Register `CS` writer"]
pub type W = crate::W<CsSpec>;
#[doc = "Clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src {
    #[doc = "1: `1`"]
    Xosc = 1,
    #[doc = "2: `10`"]
    Test0 = 2,
    #[doc = "3: `11`"]
    Test1 = 3,
    #[doc = "4: `100`"]
    Plla = 4,
    #[doc = "5: `101`"]
    Pllb = 5,
    #[doc = "6: `110`"]
    Pllc = 6,
    #[doc = "7: `111`"]
    Hdmi = 7,
    #[doc = "0: `0`"]
    Gnd = 0,
}
impl From<Src> for u8 {
    #[inline(always)]
    fn from(variant: Src) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Src {
    type Ux = u8;
}
impl crate::IsEnum for Src {}
#[doc = "Field `SRC` reader - Clock source"]
pub type SrcR = crate::FieldReader<Src>;
impl SrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Src {
        match self.bits {
            1 => Src::Xosc,
            2 => Src::Test0,
            3 => Src::Test1,
            4 => Src::Plla,
            5 => Src::Pllb,
            6 => Src::Pllc,
            7 => Src::Hdmi,
            _ => Src::Gnd,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_xosc(&self) -> bool {
        *self == Src::Xosc
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_test0(&self) -> bool {
        *self == Src::Test0
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_test1(&self) -> bool {
        *self == Src::Test1
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_plla(&self) -> bool {
        *self == Src::Plla
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_pllb(&self) -> bool {
        *self == Src::Pllb
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_pllc(&self) -> bool {
        *self == Src::Pllc
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_hdmi(&self) -> bool {
        *self == Src::Hdmi
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_gnd(&self) -> bool {
        matches!(self.variant(), Src::Gnd)
    }
}
#[doc = "Field `SRC` writer - Clock source"]
pub type SrcW<'a, REG> = crate::FieldWriter<'a, REG, 4, Src, crate::Safe>;
impl<'a, REG> SrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`1`"]
    #[inline(always)]
    pub fn xosc(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Xosc)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn test0(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Test0)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn test1(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Test1)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn plla(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Plla)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn pllb(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Pllb)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn pllc(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Pllc)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn hdmi(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Hdmi)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn gnd(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Gnd)
    }
}
#[doc = "Field `ENAB` reader - Enable the clock generator. (Switch SRC first.)"]
pub type EnabR = crate::BitReader;
#[doc = "Field `ENAB` writer - Enable the clock generator. (Switch SRC first.)"]
pub type EnabW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KILL` reader - Stop and reset the generator"]
pub type KillR = crate::BitReader;
#[doc = "Field `KILL` writer - Stop and reset the generator"]
pub type KillW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY` reader - Indicates the clock generator is running"]
pub type BusyR = crate::BitReader;
#[doc = "Field `FLIP` reader - Generate an edge on output. (For testing)"]
pub type FlipR = crate::BitReader;
#[doc = "Field `FLIP` writer - Generate an edge on output. (For testing)"]
pub type FlipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASH` reader - MASH control, stage count"]
pub type MashR = crate::FieldReader;
#[doc = "Field `MASH` writer - MASH control, stage count"]
pub type MashW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Password. Always 0x5a\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Passwd {
    #[doc = "90: `1011010`"]
    Passwd = 90,
}
impl From<Passwd> for u8 {
    #[inline(always)]
    fn from(variant: Passwd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Passwd {
    type Ux = u8;
}
impl crate::IsEnum for Passwd {}
#[doc = "Field `PASSWD` writer - Password. Always 0x5a"]
pub type PasswdW<'a, REG> = crate::FieldWriter<'a, REG, 8, Passwd>;
impl<'a, REG> PasswdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`1011010`"]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut crate::W<REG> {
        self.variant(Passwd::Passwd)
    }
}
impl R {
    #[doc = "Bits 0:3 - Clock source"]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Enable the clock generator. (Switch SRC first.)"]
    #[inline(always)]
    pub fn enab(&self) -> EnabR {
        EnabR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop and reset the generator"]
    #[inline(always)]
    pub fn kill(&self) -> KillR {
        KillR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates the clock generator is running"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Generate an edge on output. (For testing)"]
    #[inline(always)]
    pub fn flip(&self) -> FlipR {
        FlipR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - MASH control, stage count"]
    #[inline(always)]
    pub fn mash(&self) -> MashR {
        MashR::new(((self.bits >> 9) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CS")
            .field("mash", &self.mash())
            .field("flip", &self.flip())
            .field("busy", &self.busy())
            .field("kill", &self.kill())
            .field("enab", &self.enab())
            .field("src", &self.src())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock source"]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SrcW<CsSpec> {
        SrcW::new(self, 0)
    }
    #[doc = "Bit 4 - Enable the clock generator. (Switch SRC first.)"]
    #[inline(always)]
    #[must_use]
    pub fn enab(&mut self) -> EnabW<CsSpec> {
        EnabW::new(self, 4)
    }
    #[doc = "Bit 5 - Stop and reset the generator"]
    #[inline(always)]
    #[must_use]
    pub fn kill(&mut self) -> KillW<CsSpec> {
        KillW::new(self, 5)
    }
    #[doc = "Bit 8 - Generate an edge on output. (For testing)"]
    #[inline(always)]
    #[must_use]
    pub fn flip(&mut self) -> FlipW<CsSpec> {
        FlipW::new(self, 8)
    }
    #[doc = "Bits 9:10 - MASH control, stage count"]
    #[inline(always)]
    #[must_use]
    pub fn mash(&mut self) -> MashW<CsSpec> {
        MashW::new(self, 9)
    }
    #[doc = "Bits 24:31 - Password. Always 0x5a"]
    #[inline(always)]
    #[must_use]
    pub fn passwd(&mut self) -> PasswdW<CsSpec> {
        PasswdW::new(self, 24)
    }
}
#[doc = "Control / Status\n\nYou can [`read`](crate::Reg::read) this register and get [`cs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsSpec;
impl crate::RegisterSpec for CsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cs::R`](R) reader structure"]
impl crate::Readable for CsSpec {}
#[doc = "`write(|w| ..)` method takes [`cs::W`](W) writer structure"]
impl crate::Writable for CsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CS to value 0"]
impl crate::Resettable for CsSpec {
    const RESET_VALUE: u32 = 0;
}

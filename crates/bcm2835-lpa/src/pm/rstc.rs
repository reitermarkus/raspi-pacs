#[doc = "Register `RSTC` reader"]
pub type R = crate::R<RstcSpec>;
#[doc = "Register `RSTC` writer"]
pub type W = crate::W<RstcSpec>;
#[doc = "Watchdog reset config\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wrcfg {
    #[doc = "2: `10`"]
    FullReset = 2,
}
impl From<Wrcfg> for u8 {
    #[inline(always)]
    fn from(variant: Wrcfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wrcfg {
    type Ux = u8;
}
impl crate::IsEnum for Wrcfg {}
#[doc = "Field `WRCFG` reader - Watchdog reset config"]
pub type WrcfgR = crate::FieldReader<Wrcfg>;
impl WrcfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wrcfg> {
        match self.bits {
            2 => Some(Wrcfg::FullReset),
            _ => None,
        }
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_full_reset(&self) -> bool {
        *self == Wrcfg::FullReset
    }
}
#[doc = "Field `WRCFG` writer - Watchdog reset config"]
pub type WrcfgW<'a, REG> = crate::FieldWriter<'a, REG, 2, Wrcfg>;
impl<'a, REG> WrcfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`10`"]
    #[inline(always)]
    pub fn full_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Wrcfg::FullReset)
    }
}
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
    #[doc = "Bits 4:5 - Watchdog reset config"]
    #[inline(always)]
    pub fn wrcfg(&self) -> WrcfgR {
        WrcfgR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSTC")
            .field("wrcfg", &self.wrcfg())
            .finish()
    }
}
impl W {
    #[doc = "Bits 4:5 - Watchdog reset config"]
    #[inline(always)]
    #[must_use]
    pub fn wrcfg(&mut self) -> WrcfgW<RstcSpec> {
        WrcfgW::new(self, 4)
    }
    #[doc = "Bits 24:31 - Password. Always 0x5a"]
    #[inline(always)]
    #[must_use]
    pub fn passwd(&mut self) -> PasswdW<RstcSpec> {
        PasswdW::new(self, 24)
    }
}
#[doc = "Reset Control\n\nYou can [`read`](crate::Reg::read) this register and get [`rstc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstcSpec;
impl crate::RegisterSpec for RstcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstc::R`](R) reader structure"]
impl crate::Readable for RstcSpec {}
#[doc = "`write(|w| ..)` method takes [`rstc::W`](W) writer structure"]
impl crate::Writable for RstcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSTC to value 0x0102"]
impl crate::Resettable for RstcSpec {
    const RESET_VALUE: u32 = 0x0102;
}

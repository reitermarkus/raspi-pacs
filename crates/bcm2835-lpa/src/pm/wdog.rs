#[doc = "Register `WDOG` reader"]
pub type R = crate::R<WdogSpec>;
#[doc = "Register `WDOG` writer"]
pub type W = crate::W<WdogSpec>;
#[doc = "Field `TIME` reader - Time until watchdog alarm"]
pub type TimeR = crate::FieldReader<u32>;
#[doc = "Field `TIME` writer - Time until watchdog alarm"]
pub type TimeW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
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
    #[doc = "Bits 0:19 - Time until watchdog alarm"]
    #[inline(always)]
    pub fn time(&self) -> TimeR {
        TimeR::new(self.bits & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDOG").field("time", &self.time()).finish()
    }
}
impl W {
    #[doc = "Bits 0:19 - Time until watchdog alarm"]
    #[inline(always)]
    #[must_use]
    pub fn time(&mut self) -> TimeW<WdogSpec> {
        TimeW::new(self, 0)
    }
    #[doc = "Bits 24:31 - Password. Always 0x5a"]
    #[inline(always)]
    #[must_use]
    pub fn passwd(&mut self) -> PasswdW<WdogSpec> {
        PasswdW::new(self, 24)
    }
}
#[doc = "Watchdog control\n\nYou can [`read`](crate::Reg::read) this register and get [`wdog::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdog::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdogSpec;
impl crate::RegisterSpec for WdogSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdog::R`](R) reader structure"]
impl crate::Readable for WdogSpec {}
#[doc = "`write(|w| ..)` method takes [`wdog::W`](W) writer structure"]
impl crate::Writable for WdogSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDOG to value 0"]
impl crate::Resettable for WdogSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `DIV` reader"]
pub type R = crate::R<DivSpec>;
#[doc = "Register `DIV` writer"]
pub type W = crate::W<DivSpec>;
#[doc = "Field `DIVF` reader - Fractional part of divisor"]
pub type DivfR = crate::FieldReader<u16>;
#[doc = "Field `DIVF` writer - Fractional part of divisor"]
pub type DivfW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DIVI` reader - Integer part of divisor"]
pub type DiviR = crate::FieldReader<u16>;
#[doc = "Field `DIVI` writer - Integer part of divisor"]
pub type DiviW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
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
    #[doc = "Bits 0:11 - Fractional part of divisor"]
    #[inline(always)]
    pub fn divf(&self) -> DivfR {
        DivfR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - Integer part of divisor"]
    #[inline(always)]
    pub fn divi(&self) -> DiviR {
        DiviR::new(((self.bits >> 12) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIV")
            .field("divi", &self.divi())
            .field("divf", &self.divf())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Fractional part of divisor"]
    #[inline(always)]
    #[must_use]
    pub fn divf(&mut self) -> DivfW<DivSpec> {
        DivfW::new(self, 0)
    }
    #[doc = "Bits 12:23 - Integer part of divisor"]
    #[inline(always)]
    #[must_use]
    pub fn divi(&mut self) -> DiviW<DivSpec> {
        DiviW::new(self, 12)
    }
    #[doc = "Bits 24:31 - Password. Always 0x5a"]
    #[inline(always)]
    #[must_use]
    pub fn passwd(&mut self) -> PasswdW<DivSpec> {
        PasswdW::new(self, 24)
    }
}
#[doc = "Clock divisor\n\nYou can [`read`](crate::Reg::read) this register and get [`div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DivSpec;
impl crate::RegisterSpec for DivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`div::R`](R) reader structure"]
impl crate::Readable for DivSpec {}
#[doc = "`write(|w| ..)` method takes [`div::W`](W) writer structure"]
impl crate::Writable for DivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIV to value 0"]
impl crate::Resettable for DivSpec {
    const RESET_VALUE: u32 = 0;
}

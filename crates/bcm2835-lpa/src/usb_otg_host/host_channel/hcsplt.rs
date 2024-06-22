#[doc = "Register `HCSPLT` reader"]
pub type R = crate::R<HcspltSpec>;
#[doc = "Register `HCSPLT` writer"]
pub type W = crate::W<HcspltSpec>;
#[doc = "Field `PRTADDR` reader - Port address"]
pub type PrtaddrR = crate::FieldReader;
#[doc = "Field `PRTADDR` writer - Port address"]
pub type PrtaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `HUBADDR` reader - Hub address"]
pub type HubaddrR = crate::FieldReader;
#[doc = "Field `HUBADDR` writer - Hub address"]
pub type HubaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `XACTPOS` reader - XACTPOS"]
pub type XactposR = crate::FieldReader;
#[doc = "Field `XACTPOS` writer - XACTPOS"]
pub type XactposW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMPLSPLT` reader - Do complete split"]
pub type ComplspltR = crate::BitReader;
#[doc = "Field `COMPLSPLT` writer - Do complete split"]
pub type ComplspltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLITEN` reader - Split enable"]
pub type SplitenR = crate::BitReader;
#[doc = "Field `SPLITEN` writer - Split enable"]
pub type SplitenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Port address"]
    #[inline(always)]
    pub fn prtaddr(&self) -> PrtaddrR {
        PrtaddrR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - Hub address"]
    #[inline(always)]
    pub fn hubaddr(&self) -> HubaddrR {
        HubaddrR::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:15 - XACTPOS"]
    #[inline(always)]
    pub fn xactpos(&self) -> XactposR {
        XactposR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Do complete split"]
    #[inline(always)]
    pub fn complsplt(&self) -> ComplspltR {
        ComplspltR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - Split enable"]
    #[inline(always)]
    pub fn spliten(&self) -> SplitenR {
        SplitenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCSPLT")
            .field("prtaddr", &self.prtaddr())
            .field("hubaddr", &self.hubaddr())
            .field("xactpos", &self.xactpos())
            .field("complsplt", &self.complsplt())
            .field("spliten", &self.spliten())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - Port address"]
    #[inline(always)]
    #[must_use]
    pub fn prtaddr(&mut self) -> PrtaddrW<HcspltSpec> {
        PrtaddrW::new(self, 0)
    }
    #[doc = "Bits 7:13 - Hub address"]
    #[inline(always)]
    #[must_use]
    pub fn hubaddr(&mut self) -> HubaddrW<HcspltSpec> {
        HubaddrW::new(self, 7)
    }
    #[doc = "Bits 14:15 - XACTPOS"]
    #[inline(always)]
    #[must_use]
    pub fn xactpos(&mut self) -> XactposW<HcspltSpec> {
        XactposW::new(self, 14)
    }
    #[doc = "Bit 16 - Do complete split"]
    #[inline(always)]
    #[must_use]
    pub fn complsplt(&mut self) -> ComplspltW<HcspltSpec> {
        ComplspltW::new(self, 16)
    }
    #[doc = "Bit 31 - Split enable"]
    #[inline(always)]
    #[must_use]
    pub fn spliten(&mut self) -> SplitenW<HcspltSpec> {
        SplitenW::new(self, 31)
    }
}
#[doc = "Split control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsplt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcsplt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcspltSpec;
impl crate::RegisterSpec for HcspltSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcsplt::R`](R) reader structure"]
impl crate::Readable for HcspltSpec {}
#[doc = "`write(|w| ..)` method takes [`hcsplt::W`](W) writer structure"]
impl crate::Writable for HcspltSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCSPLT to value 0"]
impl crate::Resettable for HcspltSpec {
    const RESET_VALUE: u32 = 0;
}

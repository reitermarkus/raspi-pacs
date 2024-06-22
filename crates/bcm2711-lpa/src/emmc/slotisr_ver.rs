#[doc = "Register `SLOTISR_VER` reader"]
pub type R = crate::R<SlotisrVerSpec>;
#[doc = "Register `SLOTISR_VER` writer"]
pub type W = crate::W<SlotisrVerSpec>;
#[doc = "Field `SLOT_STATUS` reader - OR of interrupt and wakeup signals for each slot"]
pub type SlotStatusR = crate::FieldReader;
#[doc = "Field `SLOT_STATUS` writer - OR of interrupt and wakeup signals for each slot"]
pub type SlotStatusW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SDVERSION` reader - Host controller specification version"]
pub type SdversionR = crate::FieldReader;
#[doc = "Field `SDVERSION` writer - Host controller specification version"]
pub type SdversionW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VENDOR` reader - Vendor version number"]
pub type VendorR = crate::FieldReader;
#[doc = "Field `VENDOR` writer - Vendor version number"]
pub type VendorW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - OR of interrupt and wakeup signals for each slot"]
    #[inline(always)]
    pub fn slot_status(&self) -> SlotStatusR {
        SlotStatusR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Host controller specification version"]
    #[inline(always)]
    pub fn sdversion(&self) -> SdversionR {
        SdversionR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Vendor version number"]
    #[inline(always)]
    pub fn vendor(&self) -> VendorR {
        VendorR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLOTISR_VER")
            .field("vendor", &self.vendor())
            .field("sdversion", &self.sdversion())
            .field("slot_status", &self.slot_status())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - OR of interrupt and wakeup signals for each slot"]
    #[inline(always)]
    #[must_use]
    pub fn slot_status(&mut self) -> SlotStatusW<SlotisrVerSpec> {
        SlotStatusW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Host controller specification version"]
    #[inline(always)]
    #[must_use]
    pub fn sdversion(&mut self) -> SdversionW<SlotisrVerSpec> {
        SdversionW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Vendor version number"]
    #[inline(always)]
    #[must_use]
    pub fn vendor(&mut self) -> VendorW<SlotisrVerSpec> {
        VendorW::new(self, 24)
    }
}
#[doc = "Version information and slot interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`slotisr_ver::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slotisr_ver::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlotisrVerSpec;
impl crate::RegisterSpec for SlotisrVerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slotisr_ver::R`](R) reader structure"]
impl crate::Readable for SlotisrVerSpec {}
#[doc = "`write(|w| ..)` method takes [`slotisr_ver::W`](W) writer structure"]
impl crate::Writable for SlotisrVerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLOTISR_VER to value 0"]
impl crate::Resettable for SlotisrVerSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `GICD_ITARGETSR32` reader"]
pub type R = crate::R<GicdItargetsr32Spec>;
#[doc = "Register `GICD_ITARGETSR32` writer"]
pub type W = crate::W<GicdItargetsr32Spec>;
#[doc = "Field `HDMI_CEC` reader - HDMI CEC"]
pub type HdmiCecR = crate::FieldReader;
#[doc = "Field `HDMI_CEC` writer - HDMI CEC"]
pub type HdmiCecW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HVS` reader - HVS"]
pub type HvsR = crate::FieldReader;
#[doc = "Field `HVS` writer - HVS"]
pub type HvsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RPIVID` reader - RPIVID"]
pub type RpividR = crate::FieldReader;
#[doc = "Field `RPIVID` writer - RPIVID"]
pub type RpividW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SDC` reader - SDC"]
pub type SdcR = crate::FieldReader;
#[doc = "Field `SDC` writer - SDC"]
pub type SdcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - HDMI CEC"]
    #[inline(always)]
    pub fn hdmi_cec(&self) -> HdmiCecR {
        HdmiCecR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - HVS"]
    #[inline(always)]
    pub fn hvs(&self) -> HvsR {
        HvsR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - RPIVID"]
    #[inline(always)]
    pub fn rpivid(&self) -> RpividR {
        RpividR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - SDC"]
    #[inline(always)]
    pub fn sdc(&self) -> SdcR {
        SdcR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR32")
            .field("hdmi_cec", &self.hdmi_cec())
            .field("hvs", &self.hvs())
            .field("rpivid", &self.rpivid())
            .field("sdc", &self.sdc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - HDMI CEC"]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_cec(&mut self) -> HdmiCecW<GicdItargetsr32Spec> {
        HdmiCecW::new(self, 0)
    }
    #[doc = "Bits 8:15 - HVS"]
    #[inline(always)]
    #[must_use]
    pub fn hvs(&mut self) -> HvsW<GicdItargetsr32Spec> {
        HvsW::new(self, 8)
    }
    #[doc = "Bits 16:23 - RPIVID"]
    #[inline(always)]
    #[must_use]
    pub fn rpivid(&mut self) -> RpividW<GicdItargetsr32Spec> {
        RpividW::new(self, 16)
    }
    #[doc = "Bits 24:31 - SDC"]
    #[inline(always)]
    #[must_use]
    pub fn sdc(&mut self) -> SdcW<GicdItargetsr32Spec> {
        SdcW::new(self, 24)
    }
}
#[doc = "Interrupt Processor Target 128 - 131\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr32::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr32::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdItargetsr32Spec;
impl crate::RegisterSpec for GicdItargetsr32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr32::R`](R) reader structure"]
impl crate::Readable for GicdItargetsr32Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr32::W`](W) writer structure"]
impl crate::Writable for GicdItargetsr32Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR32 to value 0"]
impl crate::Resettable for GicdItargetsr32Spec {
    const RESET_VALUE: u32 = 0;
}

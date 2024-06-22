#[doc = "Register `GICD_ITARGETSR35` reader"]
pub type R = crate::R<GicdItargetsr35Spec>;
#[doc = "Register `GICD_ITARGETSR35` writer"]
pub type W = crate::W<GicdItargetsr35Spec>;
#[doc = "Field `DSI_1` reader - DSI 1"]
pub type Dsi1R = crate::FieldReader;
#[doc = "Field `DSI_1` writer - DSI 1"]
pub type Dsi1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PIXEL_VALVE_0` reader - Pixel Valve 0"]
pub type PixelValve0R = crate::FieldReader;
#[doc = "Field `PIXEL_VALVE_0` writer - Pixel Valve 0"]
pub type PixelValve0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PIXEL_VALVE_1_2` reader - OR of Pixel Valve 1 and 2"]
pub type PixelValve1_2R = crate::FieldReader;
#[doc = "Field `PIXEL_VALVE_1_2` writer - OR of Pixel Valve 1 and 2"]
pub type PixelValve1_2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CPR` reader - CPR"]
pub type CprR = crate::FieldReader;
#[doc = "Field `CPR` writer - CPR"]
pub type CprW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DSI 1"]
    #[inline(always)]
    pub fn dsi_1(&self) -> Dsi1R {
        Dsi1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Pixel Valve 0"]
    #[inline(always)]
    pub fn pixel_valve_0(&self) -> PixelValve0R {
        PixelValve0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - OR of Pixel Valve 1 and 2"]
    #[inline(always)]
    pub fn pixel_valve_1_2(&self) -> PixelValve1_2R {
        PixelValve1_2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - CPR"]
    #[inline(always)]
    pub fn cpr(&self) -> CprR {
        CprR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR35")
            .field("dsi_1", &self.dsi_1())
            .field("pixel_valve_0", &self.pixel_valve_0())
            .field("pixel_valve_1_2", &self.pixel_valve_1_2())
            .field("cpr", &self.cpr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - DSI 1"]
    #[inline(always)]
    #[must_use]
    pub fn dsi_1(&mut self) -> Dsi1W<GicdItargetsr35Spec> {
        Dsi1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Pixel Valve 0"]
    #[inline(always)]
    #[must_use]
    pub fn pixel_valve_0(&mut self) -> PixelValve0W<GicdItargetsr35Spec> {
        PixelValve0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - OR of Pixel Valve 1 and 2"]
    #[inline(always)]
    #[must_use]
    pub fn pixel_valve_1_2(&mut self) -> PixelValve1_2W<GicdItargetsr35Spec> {
        PixelValve1_2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - CPR"]
    #[inline(always)]
    #[must_use]
    pub fn cpr(&mut self) -> CprW<GicdItargetsr35Spec> {
        CprW::new(self, 24)
    }
}
#[doc = "Interrupt Processor Target 140 - 143\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr35::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr35::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdItargetsr35Spec;
impl crate::RegisterSpec for GicdItargetsr35Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr35::R`](R) reader structure"]
impl crate::Readable for GicdItargetsr35Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr35::W`](W) writer structure"]
impl crate::Writable for GicdItargetsr35Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR35 to value 0"]
impl crate::Resettable for GicdItargetsr35Spec {
    const RESET_VALUE: u32 = 0;
}

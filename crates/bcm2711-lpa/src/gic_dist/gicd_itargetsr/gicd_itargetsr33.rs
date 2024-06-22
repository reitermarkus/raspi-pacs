#[doc = "Register `GICD_ITARGETSR33` reader"]
pub type R = crate::R<GicdItargetsr33Spec>;
#[doc = "Register `GICD_ITARGETSR33` writer"]
pub type W = crate::W<GicdItargetsr33Spec>;
#[doc = "Field `DSI_0` reader - DSI 0"]
pub type Dsi0R = crate::FieldReader;
#[doc = "Field `DSI_0` writer - DSI 0"]
pub type Dsi0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PIXEL_VALVE_2` reader - Pixel Valve 2"]
pub type PixelValve2R = crate::FieldReader;
#[doc = "Field `PIXEL_VALVE_2` writer - Pixel Valve 2"]
pub type PixelValve2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CAMERA_0` reader - Camera 0"]
pub type Camera0R = crate::FieldReader;
#[doc = "Field `CAMERA_0` writer - Camera 0"]
pub type Camera0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CAMERA_1` reader - Camera 1"]
pub type Camera1R = crate::FieldReader;
#[doc = "Field `CAMERA_1` writer - Camera 1"]
pub type Camera1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DSI 0"]
    #[inline(always)]
    pub fn dsi_0(&self) -> Dsi0R {
        Dsi0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Pixel Valve 2"]
    #[inline(always)]
    pub fn pixel_valve_2(&self) -> PixelValve2R {
        PixelValve2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Camera 0"]
    #[inline(always)]
    pub fn camera_0(&self) -> Camera0R {
        Camera0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Camera 1"]
    #[inline(always)]
    pub fn camera_1(&self) -> Camera1R {
        Camera1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR33")
            .field("dsi_0", &self.dsi_0())
            .field("pixel_valve_2", &self.pixel_valve_2())
            .field("camera_0", &self.camera_0())
            .field("camera_1", &self.camera_1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - DSI 0"]
    #[inline(always)]
    #[must_use]
    pub fn dsi_0(&mut self) -> Dsi0W<GicdItargetsr33Spec> {
        Dsi0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Pixel Valve 2"]
    #[inline(always)]
    #[must_use]
    pub fn pixel_valve_2(&mut self) -> PixelValve2W<GicdItargetsr33Spec> {
        PixelValve2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Camera 0"]
    #[inline(always)]
    #[must_use]
    pub fn camera_0(&mut self) -> Camera0W<GicdItargetsr33Spec> {
        Camera0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Camera 1"]
    #[inline(always)]
    #[must_use]
    pub fn camera_1(&mut self) -> Camera1W<GicdItargetsr33Spec> {
        Camera1W::new(self, 24)
    }
}
#[doc = "Interrupt Processor Target 132 - 135\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr33::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr33::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdItargetsr33Spec;
impl crate::RegisterSpec for GicdItargetsr33Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr33::R`](R) reader structure"]
impl crate::Readable for GicdItargetsr33Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr33::W`](W) writer structure"]
impl crate::Writable for GicdItargetsr33Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR33 to value 0"]
impl crate::Resettable for GicdItargetsr33Spec {
    const RESET_VALUE: u32 = 0;
}

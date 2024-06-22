#[doc = "Register `GICD_ITARGETSR34` reader"]
pub type R = crate::R<GicdItargetsr34Spec>;
#[doc = "Register `GICD_ITARGETSR34` writer"]
pub type W = crate::W<GicdItargetsr34Spec>;
#[doc = "Field `HDMI_0` reader - HDMI 0"]
pub type Hdmi0R = crate::FieldReader;
#[doc = "Field `HDMI_0` writer - HDMI 0"]
pub type Hdmi0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HDMI_1` reader - HDMI 1"]
pub type Hdmi1R = crate::FieldReader;
#[doc = "Field `HDMI_1` writer - HDMI 1"]
pub type Hdmi1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PIXEL_VALVE_3` reader - Pixel Valve 3"]
pub type PixelValve3R = crate::FieldReader;
#[doc = "Field `PIXEL_VALVE_3` writer - Pixel Valve 3"]
pub type PixelValve3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPI_BSC_SLAVE` reader - SPI/BSC Slave"]
pub type SpiBscSlaveR = crate::FieldReader;
#[doc = "Field `SPI_BSC_SLAVE` writer - SPI/BSC Slave"]
pub type SpiBscSlaveW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - HDMI 0"]
    #[inline(always)]
    pub fn hdmi_0(&self) -> Hdmi0R {
        Hdmi0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - HDMI 1"]
    #[inline(always)]
    pub fn hdmi_1(&self) -> Hdmi1R {
        Hdmi1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Pixel Valve 3"]
    #[inline(always)]
    pub fn pixel_valve_3(&self) -> PixelValve3R {
        PixelValve3R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - SPI/BSC Slave"]
    #[inline(always)]
    pub fn spi_bsc_slave(&self) -> SpiBscSlaveR {
        SpiBscSlaveR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR34")
            .field("hdmi_0", &self.hdmi_0())
            .field("hdmi_1", &self.hdmi_1())
            .field("pixel_valve_3", &self.pixel_valve_3())
            .field("spi_bsc_slave", &self.spi_bsc_slave())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - HDMI 0"]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_0(&mut self) -> Hdmi0W<GicdItargetsr34Spec> {
        Hdmi0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - HDMI 1"]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_1(&mut self) -> Hdmi1W<GicdItargetsr34Spec> {
        Hdmi1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Pixel Valve 3"]
    #[inline(always)]
    #[must_use]
    pub fn pixel_valve_3(&mut self) -> PixelValve3W<GicdItargetsr34Spec> {
        PixelValve3W::new(self, 16)
    }
    #[doc = "Bits 24:31 - SPI/BSC Slave"]
    #[inline(always)]
    #[must_use]
    pub fn spi_bsc_slave(&mut self) -> SpiBscSlaveW<GicdItargetsr34Spec> {
        SpiBscSlaveW::new(self, 24)
    }
}
#[doc = "Interrupt Processor Target 136 - 139\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr34::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr34::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdItargetsr34Spec;
impl crate::RegisterSpec for GicdItargetsr34Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr34::R`](R) reader structure"]
impl crate::Readable for GicdItargetsr34Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr34::W`](W) writer structure"]
impl crate::Writable for GicdItargetsr34Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR34 to value 0"]
impl crate::Resettable for GicdItargetsr34Spec {
    const RESET_VALUE: u32 = 0;
}

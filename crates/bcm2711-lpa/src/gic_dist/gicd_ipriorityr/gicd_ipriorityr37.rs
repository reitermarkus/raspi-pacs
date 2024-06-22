#[doc = "Register `GICD_IPRIORITYR37` reader"]
pub type R = crate::R<GicdIpriorityr37Spec>;
#[doc = "Register `GICD_IPRIORITYR37` writer"]
pub type W = crate::W<GicdIpriorityr37Spec>;
#[doc = "Field `GPIO_3` reader - GPIO 3"]
pub type Gpio3R = crate::FieldReader;
#[doc = "Field `GPIO_3` writer - GPIO 3"]
pub type Gpio3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `I2C` reader - OR of all I2C"]
pub type I2cR = crate::FieldReader;
#[doc = "Field `I2C` writer - OR of all I2C"]
pub type I2cW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPI` reader - OR of all SPI"]
pub type SpiR = crate::FieldReader;
#[doc = "Field `SPI` writer - OR of all SPI"]
pub type SpiW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PCM_I2S` reader - PCM/I2S"]
pub type PcmI2sR = crate::FieldReader;
#[doc = "Field `PCM_I2S` writer - PCM/I2S"]
pub type PcmI2sW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO 3"]
    #[inline(always)]
    pub fn gpio_3(&self) -> Gpio3R {
        Gpio3R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - OR of all I2C"]
    #[inline(always)]
    pub fn i2c(&self) -> I2cR {
        I2cR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - OR of all SPI"]
    #[inline(always)]
    pub fn spi(&self) -> SpiR {
        SpiR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - PCM/I2S"]
    #[inline(always)]
    pub fn pcm_i2s(&self) -> PcmI2sR {
        PcmI2sR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR37")
            .field("gpio_3", &self.gpio_3())
            .field("i2c", &self.i2c())
            .field("spi", &self.spi())
            .field("pcm_i2s", &self.pcm_i2s())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO 3"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_3(&mut self) -> Gpio3W<GicdIpriorityr37Spec> {
        Gpio3W::new(self, 0)
    }
    #[doc = "Bits 8:15 - OR of all I2C"]
    #[inline(always)]
    #[must_use]
    pub fn i2c(&mut self) -> I2cW<GicdIpriorityr37Spec> {
        I2cW::new(self, 8)
    }
    #[doc = "Bits 16:23 - OR of all SPI"]
    #[inline(always)]
    #[must_use]
    pub fn spi(&mut self) -> SpiW<GicdIpriorityr37Spec> {
        SpiW::new(self, 16)
    }
    #[doc = "Bits 24:31 - PCM/I2S"]
    #[inline(always)]
    #[must_use]
    pub fn pcm_i2s(&mut self) -> PcmI2sW<GicdIpriorityr37Spec> {
        PcmI2sW::new(self, 24)
    }
}
#[doc = "Interrupt Priority 148 - 151 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr37::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr37::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIpriorityr37Spec;
impl crate::RegisterSpec for GicdIpriorityr37Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr37::R`](R) reader structure"]
impl crate::Readable for GicdIpriorityr37Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr37::W`](W) writer structure"]
impl crate::Writable for GicdIpriorityr37Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR37 to value 0"]
impl crate::Resettable for GicdIpriorityr37Spec {
    const RESET_VALUE: u32 = 0;
}

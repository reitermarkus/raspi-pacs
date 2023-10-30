#[doc = "Register `GICD_IPRIORITYR37` reader"]
pub type R = crate::R<GICD_IPRIORITYR37_SPEC>;
#[doc = "Register `GICD_IPRIORITYR37` writer"]
pub type W = crate::W<GICD_IPRIORITYR37_SPEC>;
#[doc = "Field `GPIO_3` reader - GPIO 3"]
pub type GPIO_3_R = crate::FieldReader;
#[doc = "Field `GPIO_3` writer - GPIO 3"]
pub type GPIO_3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `I2C` reader - OR of all I2C"]
pub type I2C_R = crate::FieldReader;
#[doc = "Field `I2C` writer - OR of all I2C"]
pub type I2C_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SPI` reader - OR of all SPI"]
pub type SPI_R = crate::FieldReader;
#[doc = "Field `SPI` writer - OR of all SPI"]
pub type SPI_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `PCM_I2S` reader - PCM/I2S"]
pub type PCM_I2S_R = crate::FieldReader;
#[doc = "Field `PCM_I2S` writer - PCM/I2S"]
pub type PCM_I2S_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - GPIO 3"]
    #[inline(always)]
    pub fn gpio_3(&self) -> GPIO_3_R {
        GPIO_3_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - OR of all I2C"]
    #[inline(always)]
    pub fn i2c(&self) -> I2C_R {
        I2C_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - OR of all SPI"]
    #[inline(always)]
    pub fn spi(&self) -> SPI_R {
        SPI_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - PCM/I2S"]
    #[inline(always)]
    pub fn pcm_i2s(&self) -> PCM_I2S_R {
        PCM_I2S_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR37")
            .field("gpio_3", &format_args!("{}", self.gpio_3().bits()))
            .field("i2c", &format_args!("{}", self.i2c().bits()))
            .field("spi", &format_args!("{}", self.spi().bits()))
            .field("pcm_i2s", &format_args!("{}", self.pcm_i2s().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_IPRIORITYR37_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO 3"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_3(&mut self) -> GPIO_3_W<GICD_IPRIORITYR37_SPEC, 0> {
        GPIO_3_W::new(self)
    }
    #[doc = "Bits 8:15 - OR of all I2C"]
    #[inline(always)]
    #[must_use]
    pub fn i2c(&mut self) -> I2C_W<GICD_IPRIORITYR37_SPEC, 8> {
        I2C_W::new(self)
    }
    #[doc = "Bits 16:23 - OR of all SPI"]
    #[inline(always)]
    #[must_use]
    pub fn spi(&mut self) -> SPI_W<GICD_IPRIORITYR37_SPEC, 16> {
        SPI_W::new(self)
    }
    #[doc = "Bits 24:31 - PCM/I2S"]
    #[inline(always)]
    #[must_use]
    pub fn pcm_i2s(&mut self) -> PCM_I2S_W<GICD_IPRIORITYR37_SPEC, 24> {
        PCM_I2S_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Priority 148 - 151 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr37::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr37::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_IPRIORITYR37_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR37_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr37::R`](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR37_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr37::W`](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR37_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR37 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR37_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

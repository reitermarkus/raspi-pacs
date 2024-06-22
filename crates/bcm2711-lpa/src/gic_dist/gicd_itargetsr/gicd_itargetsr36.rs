#[doc = "Register `GICD_ITARGETSR36` reader"]
pub type R = crate::R<GicdItargetsr36Spec>;
#[doc = "Register `GICD_ITARGETSR36` writer"]
pub type W = crate::W<GicdItargetsr36Spec>;
#[doc = "Field `SMI` reader - SMI"]
pub type SmiR = crate::FieldReader;
#[doc = "Field `SMI` writer - SMI"]
pub type SmiW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO_0` reader - GPIO 0"]
pub type Gpio0R = crate::FieldReader;
#[doc = "Field `GPIO_0` writer - GPIO 0"]
pub type Gpio0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO_1` reader - GPIO 1"]
pub type Gpio1R = crate::FieldReader;
#[doc = "Field `GPIO_1` writer - GPIO 1"]
pub type Gpio1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO_2` reader - GPIO 2"]
pub type Gpio2R = crate::FieldReader;
#[doc = "Field `GPIO_2` writer - GPIO 2"]
pub type Gpio2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SMI"]
    #[inline(always)]
    pub fn smi(&self) -> SmiR {
        SmiR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO 0"]
    #[inline(always)]
    pub fn gpio_0(&self) -> Gpio0R {
        Gpio0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO 1"]
    #[inline(always)]
    pub fn gpio_1(&self) -> Gpio1R {
        Gpio1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO 2"]
    #[inline(always)]
    pub fn gpio_2(&self) -> Gpio2R {
        Gpio2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR36")
            .field("smi", &self.smi())
            .field("gpio_0", &self.gpio_0())
            .field("gpio_1", &self.gpio_1())
            .field("gpio_2", &self.gpio_2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - SMI"]
    #[inline(always)]
    #[must_use]
    pub fn smi(&mut self) -> SmiW<GicdItargetsr36Spec> {
        SmiW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO 0"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_0(&mut self) -> Gpio0W<GicdItargetsr36Spec> {
        Gpio0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO 1"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_1(&mut self) -> Gpio1W<GicdItargetsr36Spec> {
        Gpio1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO 2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_2(&mut self) -> Gpio2W<GicdItargetsr36Spec> {
        Gpio2W::new(self, 24)
    }
}
#[doc = "Interrupt Processor Target 144 - 147\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr36::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr36::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdItargetsr36Spec;
impl crate::RegisterSpec for GicdItargetsr36Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr36::R`](R) reader structure"]
impl crate::Readable for GicdItargetsr36Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr36::W`](W) writer structure"]
impl crate::Writable for GicdItargetsr36Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR36 to value 0"]
impl crate::Resettable for GicdItargetsr36Spec {
    const RESET_VALUE: u32 = 0;
}

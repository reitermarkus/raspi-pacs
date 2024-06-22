#[doc = "Register `GICD_IPRIORITYR24` reader"]
pub type R = crate::R<GicdIpriorityr24Spec>;
#[doc = "Register `GICD_IPRIORITYR24` writer"]
pub type W = crate::W<GicdIpriorityr24Spec>;
#[doc = "Field `TIMER_0` reader - Timer 0"]
pub type Timer0R = crate::FieldReader;
#[doc = "Field `TIMER_0` writer - Timer 0"]
pub type Timer0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TIMER_1` reader - Timer 1"]
pub type Timer1R = crate::FieldReader;
#[doc = "Field `TIMER_1` writer - Timer 1"]
pub type Timer1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TIMER_2` reader - Timer 2"]
pub type Timer2R = crate::FieldReader;
#[doc = "Field `TIMER_2` writer - Timer 2"]
pub type Timer2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TIMER_3` reader - Timer 3"]
pub type Timer3R = crate::FieldReader;
#[doc = "Field `TIMER_3` writer - Timer 3"]
pub type Timer3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Timer 0"]
    #[inline(always)]
    pub fn timer_0(&self) -> Timer0R {
        Timer0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Timer 1"]
    #[inline(always)]
    pub fn timer_1(&self) -> Timer1R {
        Timer1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Timer 2"]
    #[inline(always)]
    pub fn timer_2(&self) -> Timer2R {
        Timer2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Timer 3"]
    #[inline(always)]
    pub fn timer_3(&self) -> Timer3R {
        Timer3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR24")
            .field("timer_0", &self.timer_0())
            .field("timer_1", &self.timer_1())
            .field("timer_2", &self.timer_2())
            .field("timer_3", &self.timer_3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Timer 0"]
    #[inline(always)]
    #[must_use]
    pub fn timer_0(&mut self) -> Timer0W<GicdIpriorityr24Spec> {
        Timer0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Timer 1"]
    #[inline(always)]
    #[must_use]
    pub fn timer_1(&mut self) -> Timer1W<GicdIpriorityr24Spec> {
        Timer1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Timer 2"]
    #[inline(always)]
    #[must_use]
    pub fn timer_2(&mut self) -> Timer2W<GicdIpriorityr24Spec> {
        Timer2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Timer 3"]
    #[inline(always)]
    #[must_use]
    pub fn timer_3(&mut self) -> Timer3W<GicdIpriorityr24Spec> {
        Timer3W::new(self, 24)
    }
}
#[doc = "Interrupt Priority 96 - 99 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIpriorityr24Spec;
impl crate::RegisterSpec for GicdIpriorityr24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr24::R`](R) reader structure"]
impl crate::Readable for GicdIpriorityr24Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr24::W`](W) writer structure"]
impl crate::Writable for GicdIpriorityr24Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR24 to value 0"]
impl crate::Resettable for GicdIpriorityr24Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `GICD_IPRIORITYR20` reader"]
pub type R = crate::R<GicdIpriorityr20Spec>;
#[doc = "Register `GICD_IPRIORITYR20` writer"]
pub type W = crate::W<GicdIpriorityr20Spec>;
#[doc = "Field `INT80` reader - Interrupt 80"]
pub type Int80R = crate::FieldReader;
#[doc = "Field `INT80` writer - Interrupt 80"]
pub type Int80W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT81` reader - Interrupt 81"]
pub type Int81R = crate::FieldReader;
#[doc = "Field `INT81` writer - Interrupt 81"]
pub type Int81W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT82` reader - Interrupt 82"]
pub type Int82R = crate::FieldReader;
#[doc = "Field `INT82` writer - Interrupt 82"]
pub type Int82W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT83` reader - Interrupt 83"]
pub type Int83R = crate::FieldReader;
#[doc = "Field `INT83` writer - Interrupt 83"]
pub type Int83W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 80"]
    #[inline(always)]
    pub fn int80(&self) -> Int80R {
        Int80R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 81"]
    #[inline(always)]
    pub fn int81(&self) -> Int81R {
        Int81R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 82"]
    #[inline(always)]
    pub fn int82(&self) -> Int82R {
        Int82R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 83"]
    #[inline(always)]
    pub fn int83(&self) -> Int83R {
        Int83R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR20")
            .field("int80", &self.int80())
            .field("int81", &self.int81())
            .field("int82", &self.int82())
            .field("int83", &self.int83())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 80"]
    #[inline(always)]
    #[must_use]
    pub fn int80(&mut self) -> Int80W<GicdIpriorityr20Spec> {
        Int80W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 81"]
    #[inline(always)]
    #[must_use]
    pub fn int81(&mut self) -> Int81W<GicdIpriorityr20Spec> {
        Int81W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 82"]
    #[inline(always)]
    #[must_use]
    pub fn int82(&mut self) -> Int82W<GicdIpriorityr20Spec> {
        Int82W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 83"]
    #[inline(always)]
    #[must_use]
    pub fn int83(&mut self) -> Int83W<GicdIpriorityr20Spec> {
        Int83W::new(self, 24)
    }
}
#[doc = "Interrupt Priority 80 - 83 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIpriorityr20Spec;
impl crate::RegisterSpec for GicdIpriorityr20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr20::R`](R) reader structure"]
impl crate::Readable for GicdIpriorityr20Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr20::W`](W) writer structure"]
impl crate::Writable for GicdIpriorityr20Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR20 to value 0"]
impl crate::Resettable for GicdIpriorityr20Spec {
    const RESET_VALUE: u32 = 0;
}

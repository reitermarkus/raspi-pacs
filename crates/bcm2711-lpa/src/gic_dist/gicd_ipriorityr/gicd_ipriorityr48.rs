#[doc = "Register `GICD_IPRIORITYR48` reader"]
pub type R = crate::R<GicdIpriorityr48Spec>;
#[doc = "Register `GICD_IPRIORITYR48` writer"]
pub type W = crate::W<GicdIpriorityr48Spec>;
#[doc = "Field `INT192` reader - Interrupt 192"]
pub type Int192R = crate::FieldReader;
#[doc = "Field `INT192` writer - Interrupt 192"]
pub type Int192W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT193` reader - Interrupt 193"]
pub type Int193R = crate::FieldReader;
#[doc = "Field `INT193` writer - Interrupt 193"]
pub type Int193W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT194` reader - Interrupt 194"]
pub type Int194R = crate::FieldReader;
#[doc = "Field `INT194` writer - Interrupt 194"]
pub type Int194W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT195` reader - Interrupt 195"]
pub type Int195R = crate::FieldReader;
#[doc = "Field `INT195` writer - Interrupt 195"]
pub type Int195W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 192"]
    #[inline(always)]
    pub fn int192(&self) -> Int192R {
        Int192R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 193"]
    #[inline(always)]
    pub fn int193(&self) -> Int193R {
        Int193R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 194"]
    #[inline(always)]
    pub fn int194(&self) -> Int194R {
        Int194R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 195"]
    #[inline(always)]
    pub fn int195(&self) -> Int195R {
        Int195R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR48")
            .field("int192", &self.int192())
            .field("int193", &self.int193())
            .field("int194", &self.int194())
            .field("int195", &self.int195())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 192"]
    #[inline(always)]
    #[must_use]
    pub fn int192(&mut self) -> Int192W<GicdIpriorityr48Spec> {
        Int192W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 193"]
    #[inline(always)]
    #[must_use]
    pub fn int193(&mut self) -> Int193W<GicdIpriorityr48Spec> {
        Int193W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 194"]
    #[inline(always)]
    #[must_use]
    pub fn int194(&mut self) -> Int194W<GicdIpriorityr48Spec> {
        Int194W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 195"]
    #[inline(always)]
    #[must_use]
    pub fn int195(&mut self) -> Int195W<GicdIpriorityr48Spec> {
        Int195W::new(self, 24)
    }
}
#[doc = "Interrupt Priority 192 - 195 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr48::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr48::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIpriorityr48Spec;
impl crate::RegisterSpec for GicdIpriorityr48Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr48::R`](R) reader structure"]
impl crate::Readable for GicdIpriorityr48Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr48::W`](W) writer structure"]
impl crate::Writable for GicdIpriorityr48Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR48 to value 0"]
impl crate::Resettable for GicdIpriorityr48Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `GICD_IPRIORITYR13` reader"]
pub type R = crate::R<GicdIpriorityr13Spec>;
#[doc = "Register `GICD_IPRIORITYR13` writer"]
pub type W = crate::W<GicdIpriorityr13Spec>;
#[doc = "Field `INT52` reader - Interrupt 52"]
pub type Int52R = crate::FieldReader;
#[doc = "Field `INT52` writer - Interrupt 52"]
pub type Int52W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT53` reader - Interrupt 53"]
pub type Int53R = crate::FieldReader;
#[doc = "Field `INT53` writer - Interrupt 53"]
pub type Int53W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT54` reader - Interrupt 54"]
pub type Int54R = crate::FieldReader;
#[doc = "Field `INT54` writer - Interrupt 54"]
pub type Int54W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT55` reader - Interrupt 55"]
pub type Int55R = crate::FieldReader;
#[doc = "Field `INT55` writer - Interrupt 55"]
pub type Int55W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 52"]
    #[inline(always)]
    pub fn int52(&self) -> Int52R {
        Int52R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 53"]
    #[inline(always)]
    pub fn int53(&self) -> Int53R {
        Int53R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 54"]
    #[inline(always)]
    pub fn int54(&self) -> Int54R {
        Int54R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 55"]
    #[inline(always)]
    pub fn int55(&self) -> Int55R {
        Int55R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR13")
            .field("int52", &self.int52())
            .field("int53", &self.int53())
            .field("int54", &self.int54())
            .field("int55", &self.int55())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 52"]
    #[inline(always)]
    #[must_use]
    pub fn int52(&mut self) -> Int52W<GicdIpriorityr13Spec> {
        Int52W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 53"]
    #[inline(always)]
    #[must_use]
    pub fn int53(&mut self) -> Int53W<GicdIpriorityr13Spec> {
        Int53W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 54"]
    #[inline(always)]
    #[must_use]
    pub fn int54(&mut self) -> Int54W<GicdIpriorityr13Spec> {
        Int54W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 55"]
    #[inline(always)]
    #[must_use]
    pub fn int55(&mut self) -> Int55W<GicdIpriorityr13Spec> {
        Int55W::new(self, 24)
    }
}
#[doc = "Interrupt Priority 52 - 55 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIpriorityr13Spec;
impl crate::RegisterSpec for GicdIpriorityr13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr13::R`](R) reader structure"]
impl crate::Readable for GicdIpriorityr13Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr13::W`](W) writer structure"]
impl crate::Writable for GicdIpriorityr13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR13 to value 0"]
impl crate::Resettable for GicdIpriorityr13Spec {
    const RESET_VALUE: u32 = 0;
}

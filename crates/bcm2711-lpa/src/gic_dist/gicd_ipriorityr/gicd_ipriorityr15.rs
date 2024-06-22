#[doc = "Register `GICD_IPRIORITYR15` reader"]
pub type R = crate::R<GicdIpriorityr15Spec>;
#[doc = "Register `GICD_IPRIORITYR15` writer"]
pub type W = crate::W<GicdIpriorityr15Spec>;
#[doc = "Field `INT60` reader - Interrupt 60"]
pub type Int60R = crate::FieldReader;
#[doc = "Field `INT60` writer - Interrupt 60"]
pub type Int60W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT61` reader - Interrupt 61"]
pub type Int61R = crate::FieldReader;
#[doc = "Field `INT61` writer - Interrupt 61"]
pub type Int61W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT62` reader - Interrupt 62"]
pub type Int62R = crate::FieldReader;
#[doc = "Field `INT62` writer - Interrupt 62"]
pub type Int62W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT63` reader - Interrupt 63"]
pub type Int63R = crate::FieldReader;
#[doc = "Field `INT63` writer - Interrupt 63"]
pub type Int63W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 60"]
    #[inline(always)]
    pub fn int60(&self) -> Int60R {
        Int60R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 61"]
    #[inline(always)]
    pub fn int61(&self) -> Int61R {
        Int61R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 62"]
    #[inline(always)]
    pub fn int62(&self) -> Int62R {
        Int62R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 63"]
    #[inline(always)]
    pub fn int63(&self) -> Int63R {
        Int63R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR15")
            .field("int60", &self.int60())
            .field("int61", &self.int61())
            .field("int62", &self.int62())
            .field("int63", &self.int63())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 60"]
    #[inline(always)]
    #[must_use]
    pub fn int60(&mut self) -> Int60W<GicdIpriorityr15Spec> {
        Int60W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 61"]
    #[inline(always)]
    #[must_use]
    pub fn int61(&mut self) -> Int61W<GicdIpriorityr15Spec> {
        Int61W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 62"]
    #[inline(always)]
    #[must_use]
    pub fn int62(&mut self) -> Int62W<GicdIpriorityr15Spec> {
        Int62W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 63"]
    #[inline(always)]
    #[must_use]
    pub fn int63(&mut self) -> Int63W<GicdIpriorityr15Spec> {
        Int63W::new(self, 24)
    }
}
#[doc = "Interrupt Priority 60 - 63 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIpriorityr15Spec;
impl crate::RegisterSpec for GicdIpriorityr15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr15::R`](R) reader structure"]
impl crate::Readable for GicdIpriorityr15Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr15::W`](W) writer structure"]
impl crate::Writable for GicdIpriorityr15Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR15 to value 0"]
impl crate::Resettable for GicdIpriorityr15Spec {
    const RESET_VALUE: u32 = 0;
}

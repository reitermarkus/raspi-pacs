#[doc = "Register `GICD_IPRIORITYR4` reader"]
pub type R = crate::R<GicdIpriorityr4Spec>;
#[doc = "Register `GICD_IPRIORITYR4` writer"]
pub type W = crate::W<GicdIpriorityr4Spec>;
#[doc = "Field `INT16` reader - Interrupt 16"]
pub type Int16R = crate::FieldReader;
#[doc = "Field `INT16` writer - Interrupt 16"]
pub type Int16W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT17` reader - Interrupt 17"]
pub type Int17R = crate::FieldReader;
#[doc = "Field `INT17` writer - Interrupt 17"]
pub type Int17W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT18` reader - Interrupt 18"]
pub type Int18R = crate::FieldReader;
#[doc = "Field `INT18` writer - Interrupt 18"]
pub type Int18W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT19` reader - Interrupt 19"]
pub type Int19R = crate::FieldReader;
#[doc = "Field `INT19` writer - Interrupt 19"]
pub type Int19W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 16"]
    #[inline(always)]
    pub fn int16(&self) -> Int16R {
        Int16R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 17"]
    #[inline(always)]
    pub fn int17(&self) -> Int17R {
        Int17R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 18"]
    #[inline(always)]
    pub fn int18(&self) -> Int18R {
        Int18R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 19"]
    #[inline(always)]
    pub fn int19(&self) -> Int19R {
        Int19R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR4")
            .field("int16", &self.int16())
            .field("int17", &self.int17())
            .field("int18", &self.int18())
            .field("int19", &self.int19())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 16"]
    #[inline(always)]
    #[must_use]
    pub fn int16(&mut self) -> Int16W<GicdIpriorityr4Spec> {
        Int16W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 17"]
    #[inline(always)]
    #[must_use]
    pub fn int17(&mut self) -> Int17W<GicdIpriorityr4Spec> {
        Int17W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 18"]
    #[inline(always)]
    #[must_use]
    pub fn int18(&mut self) -> Int18W<GicdIpriorityr4Spec> {
        Int18W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 19"]
    #[inline(always)]
    #[must_use]
    pub fn int19(&mut self) -> Int19W<GicdIpriorityr4Spec> {
        Int19W::new(self, 24)
    }
}
#[doc = "Interrupt Priority 16 - 19 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIpriorityr4Spec;
impl crate::RegisterSpec for GicdIpriorityr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr4::R`](R) reader structure"]
impl crate::Readable for GicdIpriorityr4Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr4::W`](W) writer structure"]
impl crate::Writable for GicdIpriorityr4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR4 to value 0"]
impl crate::Resettable for GicdIpriorityr4Spec {
    const RESET_VALUE: u32 = 0;
}

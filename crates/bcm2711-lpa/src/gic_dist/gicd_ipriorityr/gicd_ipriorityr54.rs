#[doc = "Register `GICD_IPRIORITYR54` reader"]
pub type R = crate::R<GicdIpriorityr54Spec>;
#[doc = "Register `GICD_IPRIORITYR54` writer"]
pub type W = crate::W<GicdIpriorityr54Spec>;
#[doc = "Field `INT216` reader - Interrupt 216"]
pub type Int216R = crate::FieldReader;
#[doc = "Field `INT216` writer - Interrupt 216"]
pub type Int216W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT217` reader - Interrupt 217"]
pub type Int217R = crate::FieldReader;
#[doc = "Field `INT217` writer - Interrupt 217"]
pub type Int217W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT218` reader - Interrupt 218"]
pub type Int218R = crate::FieldReader;
#[doc = "Field `INT218` writer - Interrupt 218"]
pub type Int218W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT219` reader - Interrupt 219"]
pub type Int219R = crate::FieldReader;
#[doc = "Field `INT219` writer - Interrupt 219"]
pub type Int219W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 216"]
    #[inline(always)]
    pub fn int216(&self) -> Int216R {
        Int216R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 217"]
    #[inline(always)]
    pub fn int217(&self) -> Int217R {
        Int217R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 218"]
    #[inline(always)]
    pub fn int218(&self) -> Int218R {
        Int218R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 219"]
    #[inline(always)]
    pub fn int219(&self) -> Int219R {
        Int219R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR54")
            .field("int216", &self.int216())
            .field("int217", &self.int217())
            .field("int218", &self.int218())
            .field("int219", &self.int219())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 216"]
    #[inline(always)]
    #[must_use]
    pub fn int216(&mut self) -> Int216W<GicdIpriorityr54Spec> {
        Int216W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 217"]
    #[inline(always)]
    #[must_use]
    pub fn int217(&mut self) -> Int217W<GicdIpriorityr54Spec> {
        Int217W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 218"]
    #[inline(always)]
    #[must_use]
    pub fn int218(&mut self) -> Int218W<GicdIpriorityr54Spec> {
        Int218W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 219"]
    #[inline(always)]
    #[must_use]
    pub fn int219(&mut self) -> Int219W<GicdIpriorityr54Spec> {
        Int219W::new(self, 24)
    }
}
#[doc = "Interrupt Priority 216 - 219 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr54::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr54::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIpriorityr54Spec;
impl crate::RegisterSpec for GicdIpriorityr54Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr54::R`](R) reader structure"]
impl crate::Readable for GicdIpriorityr54Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr54::W`](W) writer structure"]
impl crate::Writable for GicdIpriorityr54Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR54 to value 0"]
impl crate::Resettable for GicdIpriorityr54Spec {
    const RESET_VALUE: u32 = 0;
}

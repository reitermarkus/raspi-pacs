#[doc = "Register `GICD_IPRIORITYR2` reader"]
pub type R = crate::R<GicdIpriorityr2Spec>;
#[doc = "Register `GICD_IPRIORITYR2` writer"]
pub type W = crate::W<GicdIpriorityr2Spec>;
#[doc = "Field `INT8` reader - Interrupt 8"]
pub type Int8R = crate::FieldReader;
#[doc = "Field `INT8` writer - Interrupt 8"]
pub type Int8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT9` reader - Interrupt 9"]
pub type Int9R = crate::FieldReader;
#[doc = "Field `INT9` writer - Interrupt 9"]
pub type Int9W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT10` reader - Interrupt 10"]
pub type Int10R = crate::FieldReader;
#[doc = "Field `INT10` writer - Interrupt 10"]
pub type Int10W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT11` reader - Interrupt 11"]
pub type Int11R = crate::FieldReader;
#[doc = "Field `INT11` writer - Interrupt 11"]
pub type Int11W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 8"]
    #[inline(always)]
    pub fn int8(&self) -> Int8R {
        Int8R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 9"]
    #[inline(always)]
    pub fn int9(&self) -> Int9R {
        Int9R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 10"]
    #[inline(always)]
    pub fn int10(&self) -> Int10R {
        Int10R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 11"]
    #[inline(always)]
    pub fn int11(&self) -> Int11R {
        Int11R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR2")
            .field("int8", &self.int8())
            .field("int9", &self.int9())
            .field("int10", &self.int10())
            .field("int11", &self.int11())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 8"]
    #[inline(always)]
    #[must_use]
    pub fn int8(&mut self) -> Int8W<GicdIpriorityr2Spec> {
        Int8W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 9"]
    #[inline(always)]
    #[must_use]
    pub fn int9(&mut self) -> Int9W<GicdIpriorityr2Spec> {
        Int9W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 10"]
    #[inline(always)]
    #[must_use]
    pub fn int10(&mut self) -> Int10W<GicdIpriorityr2Spec> {
        Int10W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 11"]
    #[inline(always)]
    #[must_use]
    pub fn int11(&mut self) -> Int11W<GicdIpriorityr2Spec> {
        Int11W::new(self, 24)
    }
}
#[doc = "Interrupt Priority 8 - 11 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIpriorityr2Spec;
impl crate::RegisterSpec for GicdIpriorityr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr2::R`](R) reader structure"]
impl crate::Readable for GicdIpriorityr2Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr2::W`](W) writer structure"]
impl crate::Writable for GicdIpriorityr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR2 to value 0"]
impl crate::Resettable for GicdIpriorityr2Spec {
    const RESET_VALUE: u32 = 0;
}

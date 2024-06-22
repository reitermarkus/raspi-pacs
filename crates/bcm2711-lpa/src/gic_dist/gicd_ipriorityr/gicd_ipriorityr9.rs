#[doc = "Register `GICD_IPRIORITYR9` reader"]
pub type R = crate::R<GicdIpriorityr9Spec>;
#[doc = "Register `GICD_IPRIORITYR9` writer"]
pub type W = crate::W<GicdIpriorityr9Spec>;
#[doc = "Field `INT36` reader - Interrupt 36"]
pub type Int36R = crate::FieldReader;
#[doc = "Field `INT36` writer - Interrupt 36"]
pub type Int36W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT37` reader - Interrupt 37"]
pub type Int37R = crate::FieldReader;
#[doc = "Field `INT37` writer - Interrupt 37"]
pub type Int37W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT38` reader - Interrupt 38"]
pub type Int38R = crate::FieldReader;
#[doc = "Field `INT38` writer - Interrupt 38"]
pub type Int38W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT39` reader - Interrupt 39"]
pub type Int39R = crate::FieldReader;
#[doc = "Field `INT39` writer - Interrupt 39"]
pub type Int39W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 36"]
    #[inline(always)]
    pub fn int36(&self) -> Int36R {
        Int36R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 37"]
    #[inline(always)]
    pub fn int37(&self) -> Int37R {
        Int37R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 38"]
    #[inline(always)]
    pub fn int38(&self) -> Int38R {
        Int38R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 39"]
    #[inline(always)]
    pub fn int39(&self) -> Int39R {
        Int39R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR9")
            .field("int36", &self.int36())
            .field("int37", &self.int37())
            .field("int38", &self.int38())
            .field("int39", &self.int39())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 36"]
    #[inline(always)]
    #[must_use]
    pub fn int36(&mut self) -> Int36W<GicdIpriorityr9Spec> {
        Int36W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 37"]
    #[inline(always)]
    #[must_use]
    pub fn int37(&mut self) -> Int37W<GicdIpriorityr9Spec> {
        Int37W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 38"]
    #[inline(always)]
    #[must_use]
    pub fn int38(&mut self) -> Int38W<GicdIpriorityr9Spec> {
        Int38W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 39"]
    #[inline(always)]
    #[must_use]
    pub fn int39(&mut self) -> Int39W<GicdIpriorityr9Spec> {
        Int39W::new(self, 24)
    }
}
#[doc = "Interrupt Priority 36 - 39 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIpriorityr9Spec;
impl crate::RegisterSpec for GicdIpriorityr9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr9::R`](R) reader structure"]
impl crate::Readable for GicdIpriorityr9Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr9::W`](W) writer structure"]
impl crate::Writable for GicdIpriorityr9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR9 to value 0"]
impl crate::Resettable for GicdIpriorityr9Spec {
    const RESET_VALUE: u32 = 0;
}

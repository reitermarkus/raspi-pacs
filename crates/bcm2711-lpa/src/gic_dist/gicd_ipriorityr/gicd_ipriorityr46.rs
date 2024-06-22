#[doc = "Register `GICD_IPRIORITYR46` reader"]
pub type R = crate::R<GicdIpriorityr46Spec>;
#[doc = "Register `GICD_IPRIORITYR46` writer"]
pub type W = crate::W<GicdIpriorityr46Spec>;
#[doc = "Field `INT184` reader - Interrupt 184"]
pub type Int184R = crate::FieldReader;
#[doc = "Field `INT184` writer - Interrupt 184"]
pub type Int184W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT185` reader - Interrupt 185"]
pub type Int185R = crate::FieldReader;
#[doc = "Field `INT185` writer - Interrupt 185"]
pub type Int185W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT186` reader - Interrupt 186"]
pub type Int186R = crate::FieldReader;
#[doc = "Field `INT186` writer - Interrupt 186"]
pub type Int186W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT187` reader - Interrupt 187"]
pub type Int187R = crate::FieldReader;
#[doc = "Field `INT187` writer - Interrupt 187"]
pub type Int187W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 184"]
    #[inline(always)]
    pub fn int184(&self) -> Int184R {
        Int184R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 185"]
    #[inline(always)]
    pub fn int185(&self) -> Int185R {
        Int185R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 186"]
    #[inline(always)]
    pub fn int186(&self) -> Int186R {
        Int186R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 187"]
    #[inline(always)]
    pub fn int187(&self) -> Int187R {
        Int187R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR46")
            .field("int184", &self.int184())
            .field("int185", &self.int185())
            .field("int186", &self.int186())
            .field("int187", &self.int187())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 184"]
    #[inline(always)]
    #[must_use]
    pub fn int184(&mut self) -> Int184W<GicdIpriorityr46Spec> {
        Int184W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 185"]
    #[inline(always)]
    #[must_use]
    pub fn int185(&mut self) -> Int185W<GicdIpriorityr46Spec> {
        Int185W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 186"]
    #[inline(always)]
    #[must_use]
    pub fn int186(&mut self) -> Int186W<GicdIpriorityr46Spec> {
        Int186W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 187"]
    #[inline(always)]
    #[must_use]
    pub fn int187(&mut self) -> Int187W<GicdIpriorityr46Spec> {
        Int187W::new(self, 24)
    }
}
#[doc = "Interrupt Priority 184 - 187 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr46::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr46::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIpriorityr46Spec;
impl crate::RegisterSpec for GicdIpriorityr46Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr46::R`](R) reader structure"]
impl crate::Readable for GicdIpriorityr46Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr46::W`](W) writer structure"]
impl crate::Writable for GicdIpriorityr46Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR46 to value 0"]
impl crate::Resettable for GicdIpriorityr46Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `GICD_IPRIORITYR49` reader"]
pub type R = crate::R<GicdIpriorityr49Spec>;
#[doc = "Register `GICD_IPRIORITYR49` writer"]
pub type W = crate::W<GicdIpriorityr49Spec>;
#[doc = "Field `INT196` reader - Interrupt 196"]
pub type Int196R = crate::FieldReader;
#[doc = "Field `INT196` writer - Interrupt 196"]
pub type Int196W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT197` reader - Interrupt 197"]
pub type Int197R = crate::FieldReader;
#[doc = "Field `INT197` writer - Interrupt 197"]
pub type Int197W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT198` reader - Interrupt 198"]
pub type Int198R = crate::FieldReader;
#[doc = "Field `INT198` writer - Interrupt 198"]
pub type Int198W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT199` reader - Interrupt 199"]
pub type Int199R = crate::FieldReader;
#[doc = "Field `INT199` writer - Interrupt 199"]
pub type Int199W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 196"]
    #[inline(always)]
    pub fn int196(&self) -> Int196R {
        Int196R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 197"]
    #[inline(always)]
    pub fn int197(&self) -> Int197R {
        Int197R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 198"]
    #[inline(always)]
    pub fn int198(&self) -> Int198R {
        Int198R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 199"]
    #[inline(always)]
    pub fn int199(&self) -> Int199R {
        Int199R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR49")
            .field("int196", &self.int196())
            .field("int197", &self.int197())
            .field("int198", &self.int198())
            .field("int199", &self.int199())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 196"]
    #[inline(always)]
    #[must_use]
    pub fn int196(&mut self) -> Int196W<GicdIpriorityr49Spec> {
        Int196W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 197"]
    #[inline(always)]
    #[must_use]
    pub fn int197(&mut self) -> Int197W<GicdIpriorityr49Spec> {
        Int197W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 198"]
    #[inline(always)]
    #[must_use]
    pub fn int198(&mut self) -> Int198W<GicdIpriorityr49Spec> {
        Int198W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 199"]
    #[inline(always)]
    #[must_use]
    pub fn int199(&mut self) -> Int199W<GicdIpriorityr49Spec> {
        Int199W::new(self, 24)
    }
}
#[doc = "Interrupt Priority 196 - 199 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr49::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr49::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIpriorityr49Spec;
impl crate::RegisterSpec for GicdIpriorityr49Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr49::R`](R) reader structure"]
impl crate::Readable for GicdIpriorityr49Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr49::W`](W) writer structure"]
impl crate::Writable for GicdIpriorityr49Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR49 to value 0"]
impl crate::Resettable for GicdIpriorityr49Spec {
    const RESET_VALUE: u32 = 0;
}

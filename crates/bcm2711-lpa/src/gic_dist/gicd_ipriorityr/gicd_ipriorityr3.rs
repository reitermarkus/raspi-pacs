#[doc = "Register `GICD_IPRIORITYR3` reader"]
pub type R = crate::R<GicdIpriorityr3Spec>;
#[doc = "Register `GICD_IPRIORITYR3` writer"]
pub type W = crate::W<GicdIpriorityr3Spec>;
#[doc = "Field `INT12` reader - Interrupt 12"]
pub type Int12R = crate::FieldReader;
#[doc = "Field `INT12` writer - Interrupt 12"]
pub type Int12W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT13` reader - Interrupt 13"]
pub type Int13R = crate::FieldReader;
#[doc = "Field `INT13` writer - Interrupt 13"]
pub type Int13W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT14` reader - Interrupt 14"]
pub type Int14R = crate::FieldReader;
#[doc = "Field `INT14` writer - Interrupt 14"]
pub type Int14W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT15` reader - Interrupt 15"]
pub type Int15R = crate::FieldReader;
#[doc = "Field `INT15` writer - Interrupt 15"]
pub type Int15W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 12"]
    #[inline(always)]
    pub fn int12(&self) -> Int12R {
        Int12R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 13"]
    #[inline(always)]
    pub fn int13(&self) -> Int13R {
        Int13R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 14"]
    #[inline(always)]
    pub fn int14(&self) -> Int14R {
        Int14R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 15"]
    #[inline(always)]
    pub fn int15(&self) -> Int15R {
        Int15R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR3")
            .field("int12", &self.int12())
            .field("int13", &self.int13())
            .field("int14", &self.int14())
            .field("int15", &self.int15())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 12"]
    #[inline(always)]
    #[must_use]
    pub fn int12(&mut self) -> Int12W<GicdIpriorityr3Spec> {
        Int12W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 13"]
    #[inline(always)]
    #[must_use]
    pub fn int13(&mut self) -> Int13W<GicdIpriorityr3Spec> {
        Int13W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 14"]
    #[inline(always)]
    #[must_use]
    pub fn int14(&mut self) -> Int14W<GicdIpriorityr3Spec> {
        Int14W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 15"]
    #[inline(always)]
    #[must_use]
    pub fn int15(&mut self) -> Int15W<GicdIpriorityr3Spec> {
        Int15W::new(self, 24)
    }
}
#[doc = "Interrupt Priority 12 - 15 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIpriorityr3Spec;
impl crate::RegisterSpec for GicdIpriorityr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr3::R`](R) reader structure"]
impl crate::Readable for GicdIpriorityr3Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr3::W`](W) writer structure"]
impl crate::Writable for GicdIpriorityr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR3 to value 0"]
impl crate::Resettable for GicdIpriorityr3Spec {
    const RESET_VALUE: u32 = 0;
}

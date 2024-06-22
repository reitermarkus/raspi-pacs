#[doc = "Register `GICD_IPRIORITYR22` reader"]
pub type R = crate::R<GicdIpriorityr22Spec>;
#[doc = "Register `GICD_IPRIORITYR22` writer"]
pub type W = crate::W<GicdIpriorityr22Spec>;
#[doc = "Field `INT88` reader - Interrupt 88"]
pub type Int88R = crate::FieldReader;
#[doc = "Field `INT88` writer - Interrupt 88"]
pub type Int88W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT89` reader - Interrupt 89"]
pub type Int89R = crate::FieldReader;
#[doc = "Field `INT89` writer - Interrupt 89"]
pub type Int89W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT90` reader - Interrupt 90"]
pub type Int90R = crate::FieldReader;
#[doc = "Field `INT90` writer - Interrupt 90"]
pub type Int90W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT91` reader - Interrupt 91"]
pub type Int91R = crate::FieldReader;
#[doc = "Field `INT91` writer - Interrupt 91"]
pub type Int91W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 88"]
    #[inline(always)]
    pub fn int88(&self) -> Int88R {
        Int88R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 89"]
    #[inline(always)]
    pub fn int89(&self) -> Int89R {
        Int89R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 90"]
    #[inline(always)]
    pub fn int90(&self) -> Int90R {
        Int90R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 91"]
    #[inline(always)]
    pub fn int91(&self) -> Int91R {
        Int91R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR22")
            .field("int88", &self.int88())
            .field("int89", &self.int89())
            .field("int90", &self.int90())
            .field("int91", &self.int91())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 88"]
    #[inline(always)]
    #[must_use]
    pub fn int88(&mut self) -> Int88W<GicdIpriorityr22Spec> {
        Int88W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 89"]
    #[inline(always)]
    #[must_use]
    pub fn int89(&mut self) -> Int89W<GicdIpriorityr22Spec> {
        Int89W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 90"]
    #[inline(always)]
    #[must_use]
    pub fn int90(&mut self) -> Int90W<GicdIpriorityr22Spec> {
        Int90W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 91"]
    #[inline(always)]
    #[must_use]
    pub fn int91(&mut self) -> Int91W<GicdIpriorityr22Spec> {
        Int91W::new(self, 24)
    }
}
#[doc = "Interrupt Priority 88 - 91 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr22::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr22::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIpriorityr22Spec;
impl crate::RegisterSpec for GicdIpriorityr22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr22::R`](R) reader structure"]
impl crate::Readable for GicdIpriorityr22Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr22::W`](W) writer structure"]
impl crate::Writable for GicdIpriorityr22Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR22 to value 0"]
impl crate::Resettable for GicdIpriorityr22Spec {
    const RESET_VALUE: u32 = 0;
}

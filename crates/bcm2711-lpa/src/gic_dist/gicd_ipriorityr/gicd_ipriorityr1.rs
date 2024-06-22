#[doc = "Register `GICD_IPRIORITYR1` reader"]
pub type R = crate::R<GicdIpriorityr1Spec>;
#[doc = "Register `GICD_IPRIORITYR1` writer"]
pub type W = crate::W<GicdIpriorityr1Spec>;
#[doc = "Field `INT4` reader - Interrupt 4"]
pub type Int4R = crate::FieldReader;
#[doc = "Field `INT4` writer - Interrupt 4"]
pub type Int4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT5` reader - Interrupt 5"]
pub type Int5R = crate::FieldReader;
#[doc = "Field `INT5` writer - Interrupt 5"]
pub type Int5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT6` reader - Interrupt 6"]
pub type Int6R = crate::FieldReader;
#[doc = "Field `INT6` writer - Interrupt 6"]
pub type Int6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT7` reader - Interrupt 7"]
pub type Int7R = crate::FieldReader;
#[doc = "Field `INT7` writer - Interrupt 7"]
pub type Int7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 4"]
    #[inline(always)]
    pub fn int4(&self) -> Int4R {
        Int4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 5"]
    #[inline(always)]
    pub fn int5(&self) -> Int5R {
        Int5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 6"]
    #[inline(always)]
    pub fn int6(&self) -> Int6R {
        Int6R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 7"]
    #[inline(always)]
    pub fn int7(&self) -> Int7R {
        Int7R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR1")
            .field("int4", &self.int4())
            .field("int5", &self.int5())
            .field("int6", &self.int6())
            .field("int7", &self.int7())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 4"]
    #[inline(always)]
    #[must_use]
    pub fn int4(&mut self) -> Int4W<GicdIpriorityr1Spec> {
        Int4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 5"]
    #[inline(always)]
    #[must_use]
    pub fn int5(&mut self) -> Int5W<GicdIpriorityr1Spec> {
        Int5W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 6"]
    #[inline(always)]
    #[must_use]
    pub fn int6(&mut self) -> Int6W<GicdIpriorityr1Spec> {
        Int6W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 7"]
    #[inline(always)]
    #[must_use]
    pub fn int7(&mut self) -> Int7W<GicdIpriorityr1Spec> {
        Int7W::new(self, 24)
    }
}
#[doc = "Interrupt Priority 4 - 7 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIpriorityr1Spec;
impl crate::RegisterSpec for GicdIpriorityr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr1::R`](R) reader structure"]
impl crate::Readable for GicdIpriorityr1Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr1::W`](W) writer structure"]
impl crate::Writable for GicdIpriorityr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR1 to value 0"]
impl crate::Resettable for GicdIpriorityr1Spec {
    const RESET_VALUE: u32 = 0;
}

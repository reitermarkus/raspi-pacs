#[doc = "Register `GICD_IPRIORITYR5` reader"]
pub type R = crate::R<GicdIpriorityr5Spec>;
#[doc = "Register `GICD_IPRIORITYR5` writer"]
pub type W = crate::W<GicdIpriorityr5Spec>;
#[doc = "Field `INT20` reader - Interrupt 20"]
pub type Int20R = crate::FieldReader;
#[doc = "Field `INT20` writer - Interrupt 20"]
pub type Int20W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT21` reader - Interrupt 21"]
pub type Int21R = crate::FieldReader;
#[doc = "Field `INT21` writer - Interrupt 21"]
pub type Int21W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT22` reader - Interrupt 22"]
pub type Int22R = crate::FieldReader;
#[doc = "Field `INT22` writer - Interrupt 22"]
pub type Int22W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT23` reader - Interrupt 23"]
pub type Int23R = crate::FieldReader;
#[doc = "Field `INT23` writer - Interrupt 23"]
pub type Int23W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 20"]
    #[inline(always)]
    pub fn int20(&self) -> Int20R {
        Int20R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 21"]
    #[inline(always)]
    pub fn int21(&self) -> Int21R {
        Int21R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 22"]
    #[inline(always)]
    pub fn int22(&self) -> Int22R {
        Int22R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 23"]
    #[inline(always)]
    pub fn int23(&self) -> Int23R {
        Int23R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR5")
            .field("int20", &self.int20())
            .field("int21", &self.int21())
            .field("int22", &self.int22())
            .field("int23", &self.int23())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 20"]
    #[inline(always)]
    #[must_use]
    pub fn int20(&mut self) -> Int20W<GicdIpriorityr5Spec> {
        Int20W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 21"]
    #[inline(always)]
    #[must_use]
    pub fn int21(&mut self) -> Int21W<GicdIpriorityr5Spec> {
        Int21W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 22"]
    #[inline(always)]
    #[must_use]
    pub fn int22(&mut self) -> Int22W<GicdIpriorityr5Spec> {
        Int22W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 23"]
    #[inline(always)]
    #[must_use]
    pub fn int23(&mut self) -> Int23W<GicdIpriorityr5Spec> {
        Int23W::new(self, 24)
    }
}
#[doc = "Interrupt Priority 20 - 23 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIpriorityr5Spec;
impl crate::RegisterSpec for GicdIpriorityr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr5::R`](R) reader structure"]
impl crate::Readable for GicdIpriorityr5Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr5::W`](W) writer structure"]
impl crate::Writable for GicdIpriorityr5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR5 to value 0"]
impl crate::Resettable for GicdIpriorityr5Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `GICD_IPRIORITYR45` reader"]
pub type R = crate::R<GicdIpriorityr45Spec>;
#[doc = "Register `GICD_IPRIORITYR45` writer"]
pub type W = crate::W<GicdIpriorityr45Spec>;
#[doc = "Field `INT180` reader - Interrupt 180"]
pub type Int180R = crate::FieldReader;
#[doc = "Field `INT180` writer - Interrupt 180"]
pub type Int180W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT181` reader - Interrupt 181"]
pub type Int181R = crate::FieldReader;
#[doc = "Field `INT181` writer - Interrupt 181"]
pub type Int181W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT182` reader - Interrupt 182"]
pub type Int182R = crate::FieldReader;
#[doc = "Field `INT182` writer - Interrupt 182"]
pub type Int182W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT183` reader - Interrupt 183"]
pub type Int183R = crate::FieldReader;
#[doc = "Field `INT183` writer - Interrupt 183"]
pub type Int183W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 180"]
    #[inline(always)]
    pub fn int180(&self) -> Int180R {
        Int180R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 181"]
    #[inline(always)]
    pub fn int181(&self) -> Int181R {
        Int181R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 182"]
    #[inline(always)]
    pub fn int182(&self) -> Int182R {
        Int182R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 183"]
    #[inline(always)]
    pub fn int183(&self) -> Int183R {
        Int183R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR45")
            .field("int180", &self.int180())
            .field("int181", &self.int181())
            .field("int182", &self.int182())
            .field("int183", &self.int183())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 180"]
    #[inline(always)]
    #[must_use]
    pub fn int180(&mut self) -> Int180W<GicdIpriorityr45Spec> {
        Int180W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 181"]
    #[inline(always)]
    #[must_use]
    pub fn int181(&mut self) -> Int181W<GicdIpriorityr45Spec> {
        Int181W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 182"]
    #[inline(always)]
    #[must_use]
    pub fn int182(&mut self) -> Int182W<GicdIpriorityr45Spec> {
        Int182W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 183"]
    #[inline(always)]
    #[must_use]
    pub fn int183(&mut self) -> Int183W<GicdIpriorityr45Spec> {
        Int183W::new(self, 24)
    }
}
#[doc = "Interrupt Priority 180 - 183 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr45::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr45::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIpriorityr45Spec;
impl crate::RegisterSpec for GicdIpriorityr45Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr45::R`](R) reader structure"]
impl crate::Readable for GicdIpriorityr45Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr45::W`](W) writer structure"]
impl crate::Writable for GicdIpriorityr45Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR45 to value 0"]
impl crate::Resettable for GicdIpriorityr45Spec {
    const RESET_VALUE: u32 = 0;
}

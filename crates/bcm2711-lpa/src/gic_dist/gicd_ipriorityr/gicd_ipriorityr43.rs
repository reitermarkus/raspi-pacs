#[doc = "Register `GICD_IPRIORITYR43` reader"]
pub type R = crate::R<GicdIpriorityr43Spec>;
#[doc = "Register `GICD_IPRIORITYR43` writer"]
pub type W = crate::W<GicdIpriorityr43Spec>;
#[doc = "Field `INT172` reader - Interrupt 172"]
pub type Int172R = crate::FieldReader;
#[doc = "Field `INT172` writer - Interrupt 172"]
pub type Int172W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT173` reader - Interrupt 173"]
pub type Int173R = crate::FieldReader;
#[doc = "Field `INT173` writer - Interrupt 173"]
pub type Int173W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT174` reader - Interrupt 174"]
pub type Int174R = crate::FieldReader;
#[doc = "Field `INT174` writer - Interrupt 174"]
pub type Int174W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT175` reader - Interrupt 175"]
pub type Int175R = crate::FieldReader;
#[doc = "Field `INT175` writer - Interrupt 175"]
pub type Int175W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 172"]
    #[inline(always)]
    pub fn int172(&self) -> Int172R {
        Int172R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 173"]
    #[inline(always)]
    pub fn int173(&self) -> Int173R {
        Int173R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 174"]
    #[inline(always)]
    pub fn int174(&self) -> Int174R {
        Int174R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 175"]
    #[inline(always)]
    pub fn int175(&self) -> Int175R {
        Int175R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR43")
            .field("int172", &self.int172())
            .field("int173", &self.int173())
            .field("int174", &self.int174())
            .field("int175", &self.int175())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 172"]
    #[inline(always)]
    #[must_use]
    pub fn int172(&mut self) -> Int172W<GicdIpriorityr43Spec> {
        Int172W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 173"]
    #[inline(always)]
    #[must_use]
    pub fn int173(&mut self) -> Int173W<GicdIpriorityr43Spec> {
        Int173W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 174"]
    #[inline(always)]
    #[must_use]
    pub fn int174(&mut self) -> Int174W<GicdIpriorityr43Spec> {
        Int174W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 175"]
    #[inline(always)]
    #[must_use]
    pub fn int175(&mut self) -> Int175W<GicdIpriorityr43Spec> {
        Int175W::new(self, 24)
    }
}
#[doc = "Interrupt Priority 172 - 175 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr43::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr43::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIpriorityr43Spec;
impl crate::RegisterSpec for GicdIpriorityr43Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr43::R`](R) reader structure"]
impl crate::Readable for GicdIpriorityr43Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr43::W`](W) writer structure"]
impl crate::Writable for GicdIpriorityr43Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR43 to value 0"]
impl crate::Resettable for GicdIpriorityr43Spec {
    const RESET_VALUE: u32 = 0;
}

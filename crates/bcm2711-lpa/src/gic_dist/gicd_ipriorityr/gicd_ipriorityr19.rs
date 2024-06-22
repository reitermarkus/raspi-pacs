#[doc = "Register `GICD_IPRIORITYR19` reader"]
pub type R = crate::R<GicdIpriorityr19Spec>;
#[doc = "Register `GICD_IPRIORITYR19` writer"]
pub type W = crate::W<GicdIpriorityr19Spec>;
#[doc = "Field `SWI4` reader - Software interrupt 4"]
pub type Swi4R = crate::FieldReader;
#[doc = "Field `SWI4` writer - Software interrupt 4"]
pub type Swi4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SWI5` reader - Software interrupt 5"]
pub type Swi5R = crate::FieldReader;
#[doc = "Field `SWI5` writer - Software interrupt 5"]
pub type Swi5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SWI6` reader - Software interrupt 6"]
pub type Swi6R = crate::FieldReader;
#[doc = "Field `SWI6` writer - Software interrupt 6"]
pub type Swi6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SWI7` reader - Software interrupt 7"]
pub type Swi7R = crate::FieldReader;
#[doc = "Field `SWI7` writer - Software interrupt 7"]
pub type Swi7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Software interrupt 4"]
    #[inline(always)]
    pub fn swi4(&self) -> Swi4R {
        Swi4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Software interrupt 5"]
    #[inline(always)]
    pub fn swi5(&self) -> Swi5R {
        Swi5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Software interrupt 6"]
    #[inline(always)]
    pub fn swi6(&self) -> Swi6R {
        Swi6R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Software interrupt 7"]
    #[inline(always)]
    pub fn swi7(&self) -> Swi7R {
        Swi7R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR19")
            .field("swi4", &self.swi4())
            .field("swi5", &self.swi5())
            .field("swi6", &self.swi6())
            .field("swi7", &self.swi7())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Software interrupt 4"]
    #[inline(always)]
    #[must_use]
    pub fn swi4(&mut self) -> Swi4W<GicdIpriorityr19Spec> {
        Swi4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Software interrupt 5"]
    #[inline(always)]
    #[must_use]
    pub fn swi5(&mut self) -> Swi5W<GicdIpriorityr19Spec> {
        Swi5W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Software interrupt 6"]
    #[inline(always)]
    #[must_use]
    pub fn swi6(&mut self) -> Swi6W<GicdIpriorityr19Spec> {
        Swi6W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Software interrupt 7"]
    #[inline(always)]
    #[must_use]
    pub fn swi7(&mut self) -> Swi7W<GicdIpriorityr19Spec> {
        Swi7W::new(self, 24)
    }
}
#[doc = "Interrupt Priority 76 - 79 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr19::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr19::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIpriorityr19Spec;
impl crate::RegisterSpec for GicdIpriorityr19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr19::R`](R) reader structure"]
impl crate::Readable for GicdIpriorityr19Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr19::W`](W) writer structure"]
impl crate::Writable for GicdIpriorityr19Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR19 to value 0"]
impl crate::Resettable for GicdIpriorityr19Spec {
    const RESET_VALUE: u32 = 0;
}

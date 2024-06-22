#[doc = "Register `GICD_IPRIORITYR50` reader"]
pub type R = crate::R<GicdIpriorityr50Spec>;
#[doc = "Register `GICD_IPRIORITYR50` writer"]
pub type W = crate::W<GicdIpriorityr50Spec>;
#[doc = "Field `INT200` reader - Interrupt 200"]
pub type Int200R = crate::FieldReader;
#[doc = "Field `INT200` writer - Interrupt 200"]
pub type Int200W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT201` reader - Interrupt 201"]
pub type Int201R = crate::FieldReader;
#[doc = "Field `INT201` writer - Interrupt 201"]
pub type Int201W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT202` reader - Interrupt 202"]
pub type Int202R = crate::FieldReader;
#[doc = "Field `INT202` writer - Interrupt 202"]
pub type Int202W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT203` reader - Interrupt 203"]
pub type Int203R = crate::FieldReader;
#[doc = "Field `INT203` writer - Interrupt 203"]
pub type Int203W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 200"]
    #[inline(always)]
    pub fn int200(&self) -> Int200R {
        Int200R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 201"]
    #[inline(always)]
    pub fn int201(&self) -> Int201R {
        Int201R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 202"]
    #[inline(always)]
    pub fn int202(&self) -> Int202R {
        Int202R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 203"]
    #[inline(always)]
    pub fn int203(&self) -> Int203R {
        Int203R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR50")
            .field("int200", &self.int200())
            .field("int201", &self.int201())
            .field("int202", &self.int202())
            .field("int203", &self.int203())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 200"]
    #[inline(always)]
    #[must_use]
    pub fn int200(&mut self) -> Int200W<GicdIpriorityr50Spec> {
        Int200W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 201"]
    #[inline(always)]
    #[must_use]
    pub fn int201(&mut self) -> Int201W<GicdIpriorityr50Spec> {
        Int201W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 202"]
    #[inline(always)]
    #[must_use]
    pub fn int202(&mut self) -> Int202W<GicdIpriorityr50Spec> {
        Int202W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 203"]
    #[inline(always)]
    #[must_use]
    pub fn int203(&mut self) -> Int203W<GicdIpriorityr50Spec> {
        Int203W::new(self, 24)
    }
}
#[doc = "Interrupt Priority 200 - 203 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr50::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr50::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIpriorityr50Spec;
impl crate::RegisterSpec for GicdIpriorityr50Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr50::R`](R) reader structure"]
impl crate::Readable for GicdIpriorityr50Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr50::W`](W) writer structure"]
impl crate::Writable for GicdIpriorityr50Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR50 to value 0"]
impl crate::Resettable for GicdIpriorityr50Spec {
    const RESET_VALUE: u32 = 0;
}

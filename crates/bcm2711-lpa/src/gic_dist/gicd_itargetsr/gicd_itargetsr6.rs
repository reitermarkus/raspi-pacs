#[doc = "Register `GICD_ITARGETSR6` reader"]
pub type R = crate::R<GicdItargetsr6Spec>;
#[doc = "Register `GICD_ITARGETSR6` writer"]
pub type W = crate::W<GicdItargetsr6Spec>;
#[doc = "Field `INT24` reader - Interrupt 24"]
pub type Int24R = crate::FieldReader;
#[doc = "Field `INT24` writer - Interrupt 24"]
pub type Int24W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT25` reader - Interrupt 25"]
pub type Int25R = crate::FieldReader;
#[doc = "Field `INT25` writer - Interrupt 25"]
pub type Int25W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT26` reader - Interrupt 26"]
pub type Int26R = crate::FieldReader;
#[doc = "Field `INT26` writer - Interrupt 26"]
pub type Int26W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT27` reader - Interrupt 27"]
pub type Int27R = crate::FieldReader;
#[doc = "Field `INT27` writer - Interrupt 27"]
pub type Int27W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 24"]
    #[inline(always)]
    pub fn int24(&self) -> Int24R {
        Int24R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 25"]
    #[inline(always)]
    pub fn int25(&self) -> Int25R {
        Int25R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 26"]
    #[inline(always)]
    pub fn int26(&self) -> Int26R {
        Int26R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 27"]
    #[inline(always)]
    pub fn int27(&self) -> Int27R {
        Int27R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR6")
            .field("int24", &self.int24())
            .field("int25", &self.int25())
            .field("int26", &self.int26())
            .field("int27", &self.int27())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 24"]
    #[inline(always)]
    #[must_use]
    pub fn int24(&mut self) -> Int24W<GicdItargetsr6Spec> {
        Int24W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 25"]
    #[inline(always)]
    #[must_use]
    pub fn int25(&mut self) -> Int25W<GicdItargetsr6Spec> {
        Int25W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 26"]
    #[inline(always)]
    #[must_use]
    pub fn int26(&mut self) -> Int26W<GicdItargetsr6Spec> {
        Int26W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 27"]
    #[inline(always)]
    #[must_use]
    pub fn int27(&mut self) -> Int27W<GicdItargetsr6Spec> {
        Int27W::new(self, 24)
    }
}
#[doc = "Interrupt Processor Target 24 - 27\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdItargetsr6Spec;
impl crate::RegisterSpec for GicdItargetsr6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr6::R`](R) reader structure"]
impl crate::Readable for GicdItargetsr6Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr6::W`](W) writer structure"]
impl crate::Writable for GicdItargetsr6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR6 to value 0"]
impl crate::Resettable for GicdItargetsr6Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `GICD_ITARGETSR0` reader"]
pub type R = crate::R<GicdItargetsr0Spec>;
#[doc = "Register `GICD_ITARGETSR0` writer"]
pub type W = crate::W<GicdItargetsr0Spec>;
#[doc = "Field `INT0` reader - Interrupt 0"]
pub type Int0R = crate::FieldReader;
#[doc = "Field `INT0` writer - Interrupt 0"]
pub type Int0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT1` reader - Interrupt 1"]
pub type Int1R = crate::FieldReader;
#[doc = "Field `INT1` writer - Interrupt 1"]
pub type Int1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT2` reader - Interrupt 2"]
pub type Int2R = crate::FieldReader;
#[doc = "Field `INT2` writer - Interrupt 2"]
pub type Int2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT3` reader - Interrupt 3"]
pub type Int3R = crate::FieldReader;
#[doc = "Field `INT3` writer - Interrupt 3"]
pub type Int3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 0"]
    #[inline(always)]
    pub fn int0(&self) -> Int0R {
        Int0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 1"]
    #[inline(always)]
    pub fn int1(&self) -> Int1R {
        Int1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 2"]
    #[inline(always)]
    pub fn int2(&self) -> Int2R {
        Int2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 3"]
    #[inline(always)]
    pub fn int3(&self) -> Int3R {
        Int3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR0")
            .field("int0", &self.int0())
            .field("int1", &self.int1())
            .field("int2", &self.int2())
            .field("int3", &self.int3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 0"]
    #[inline(always)]
    #[must_use]
    pub fn int0(&mut self) -> Int0W<GicdItargetsr0Spec> {
        Int0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 1"]
    #[inline(always)]
    #[must_use]
    pub fn int1(&mut self) -> Int1W<GicdItargetsr0Spec> {
        Int1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 2"]
    #[inline(always)]
    #[must_use]
    pub fn int2(&mut self) -> Int2W<GicdItargetsr0Spec> {
        Int2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 3"]
    #[inline(always)]
    #[must_use]
    pub fn int3(&mut self) -> Int3W<GicdItargetsr0Spec> {
        Int3W::new(self, 24)
    }
}
#[doc = "Interrupt Processor Target 0 - 3\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdItargetsr0Spec;
impl crate::RegisterSpec for GicdItargetsr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr0::R`](R) reader structure"]
impl crate::Readable for GicdItargetsr0Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr0::W`](W) writer structure"]
impl crate::Writable for GicdItargetsr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR0 to value 0"]
impl crate::Resettable for GicdItargetsr0Spec {
    const RESET_VALUE: u32 = 0;
}

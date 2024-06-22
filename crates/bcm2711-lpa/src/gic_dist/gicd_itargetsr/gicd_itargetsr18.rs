#[doc = "Register `GICD_ITARGETSR18` reader"]
pub type R = crate::R<GicdItargetsr18Spec>;
#[doc = "Register `GICD_ITARGETSR18` writer"]
pub type W = crate::W<GicdItargetsr18Spec>;
#[doc = "Field `SWI0` reader - Software interrupt 0"]
pub type Swi0R = crate::FieldReader;
#[doc = "Field `SWI0` writer - Software interrupt 0"]
pub type Swi0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SWI1` reader - Software interrupt 1"]
pub type Swi1R = crate::FieldReader;
#[doc = "Field `SWI1` writer - Software interrupt 1"]
pub type Swi1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SWI2` reader - Software interrupt 2"]
pub type Swi2R = crate::FieldReader;
#[doc = "Field `SWI2` writer - Software interrupt 2"]
pub type Swi2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SWI3` reader - Software interrupt 3"]
pub type Swi3R = crate::FieldReader;
#[doc = "Field `SWI3` writer - Software interrupt 3"]
pub type Swi3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Software interrupt 0"]
    #[inline(always)]
    pub fn swi0(&self) -> Swi0R {
        Swi0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Software interrupt 1"]
    #[inline(always)]
    pub fn swi1(&self) -> Swi1R {
        Swi1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Software interrupt 2"]
    #[inline(always)]
    pub fn swi2(&self) -> Swi2R {
        Swi2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Software interrupt 3"]
    #[inline(always)]
    pub fn swi3(&self) -> Swi3R {
        Swi3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR18")
            .field("swi0", &self.swi0())
            .field("swi1", &self.swi1())
            .field("swi2", &self.swi2())
            .field("swi3", &self.swi3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Software interrupt 0"]
    #[inline(always)]
    #[must_use]
    pub fn swi0(&mut self) -> Swi0W<GicdItargetsr18Spec> {
        Swi0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Software interrupt 1"]
    #[inline(always)]
    #[must_use]
    pub fn swi1(&mut self) -> Swi1W<GicdItargetsr18Spec> {
        Swi1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Software interrupt 2"]
    #[inline(always)]
    #[must_use]
    pub fn swi2(&mut self) -> Swi2W<GicdItargetsr18Spec> {
        Swi2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Software interrupt 3"]
    #[inline(always)]
    #[must_use]
    pub fn swi3(&mut self) -> Swi3W<GicdItargetsr18Spec> {
        Swi3W::new(self, 24)
    }
}
#[doc = "Interrupt Processor Target 72 - 75\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdItargetsr18Spec;
impl crate::RegisterSpec for GicdItargetsr18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr18::R`](R) reader structure"]
impl crate::Readable for GicdItargetsr18Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr18::W`](W) writer structure"]
impl crate::Writable for GicdItargetsr18Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR18 to value 0"]
impl crate::Resettable for GicdItargetsr18Spec {
    const RESET_VALUE: u32 = 0;
}

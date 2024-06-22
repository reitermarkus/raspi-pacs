#[doc = "Register `GICD_ITARGETSR27` reader"]
pub type R = crate::R<GicdItargetsr27Spec>;
#[doc = "Register `GICD_ITARGETSR27` writer"]
pub type W = crate::W<GicdItargetsr27Spec>;
#[doc = "Field `MULTICORE_SYNC_0` reader - Multicore Sync 0"]
pub type MulticoreSync0R = crate::FieldReader;
#[doc = "Field `MULTICORE_SYNC_0` writer - Multicore Sync 0"]
pub type MulticoreSync0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MULTICORE_SYNC_1` reader - Multicore Sync 1"]
pub type MulticoreSync1R = crate::FieldReader;
#[doc = "Field `MULTICORE_SYNC_1` writer - Multicore Sync 1"]
pub type MulticoreSync1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MULTICORE_SYNC_2` reader - Multicore Sync 2"]
pub type MulticoreSync2R = crate::FieldReader;
#[doc = "Field `MULTICORE_SYNC_2` writer - Multicore Sync 2"]
pub type MulticoreSync2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MULTICORE_SYNC_3` reader - Multicore Sync 3"]
pub type MulticoreSync3R = crate::FieldReader;
#[doc = "Field `MULTICORE_SYNC_3` writer - Multicore Sync 3"]
pub type MulticoreSync3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Multicore Sync 0"]
    #[inline(always)]
    pub fn multicore_sync_0(&self) -> MulticoreSync0R {
        MulticoreSync0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Multicore Sync 1"]
    #[inline(always)]
    pub fn multicore_sync_1(&self) -> MulticoreSync1R {
        MulticoreSync1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Multicore Sync 2"]
    #[inline(always)]
    pub fn multicore_sync_2(&self) -> MulticoreSync2R {
        MulticoreSync2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Multicore Sync 3"]
    #[inline(always)]
    pub fn multicore_sync_3(&self) -> MulticoreSync3R {
        MulticoreSync3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR27")
            .field("multicore_sync_0", &self.multicore_sync_0())
            .field("multicore_sync_1", &self.multicore_sync_1())
            .field("multicore_sync_2", &self.multicore_sync_2())
            .field("multicore_sync_3", &self.multicore_sync_3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Multicore Sync 0"]
    #[inline(always)]
    #[must_use]
    pub fn multicore_sync_0(&mut self) -> MulticoreSync0W<GicdItargetsr27Spec> {
        MulticoreSync0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Multicore Sync 1"]
    #[inline(always)]
    #[must_use]
    pub fn multicore_sync_1(&mut self) -> MulticoreSync1W<GicdItargetsr27Spec> {
        MulticoreSync1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Multicore Sync 2"]
    #[inline(always)]
    #[must_use]
    pub fn multicore_sync_2(&mut self) -> MulticoreSync2W<GicdItargetsr27Spec> {
        MulticoreSync2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Multicore Sync 3"]
    #[inline(always)]
    #[must_use]
    pub fn multicore_sync_3(&mut self) -> MulticoreSync3W<GicdItargetsr27Spec> {
        MulticoreSync3W::new(self, 24)
    }
}
#[doc = "Interrupt Processor Target 108 - 111\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr27::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr27::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdItargetsr27Spec;
impl crate::RegisterSpec for GicdItargetsr27Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr27::R`](R) reader structure"]
impl crate::Readable for GicdItargetsr27Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr27::W`](W) writer structure"]
impl crate::Writable for GicdItargetsr27Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR27 to value 0"]
impl crate::Resettable for GicdItargetsr27Spec {
    const RESET_VALUE: u32 = 0;
}

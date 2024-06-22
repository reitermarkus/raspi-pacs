#[doc = "Register `GICD_ITARGETSR30` reader"]
pub type R = crate::R<GicdItargetsr30Spec>;
#[doc = "Register `GICD_ITARGETSR30` writer"]
pub type W = crate::W<GicdItargetsr30Spec>;
#[doc = "Field `DMA_9_10` reader - OR of DMA 9 and 10"]
pub type Dma9_10R = crate::FieldReader;
#[doc = "Field `DMA_9_10` writer - OR of DMA 9 and 10"]
pub type Dma9_10W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DMA_11` reader - DMA 11"]
pub type Dma11R = crate::FieldReader;
#[doc = "Field `DMA_11` writer - DMA 11"]
pub type Dma11W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DMA_12` reader - DMA 12"]
pub type Dma12R = crate::FieldReader;
#[doc = "Field `DMA_12` writer - DMA 12"]
pub type Dma12W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DMA_13` reader - DMA 13"]
pub type Dma13R = crate::FieldReader;
#[doc = "Field `DMA_13` writer - DMA 13"]
pub type Dma13W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - OR of DMA 9 and 10"]
    #[inline(always)]
    pub fn dma_9_10(&self) -> Dma9_10R {
        Dma9_10R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DMA 11"]
    #[inline(always)]
    pub fn dma_11(&self) -> Dma11R {
        Dma11R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DMA 12"]
    #[inline(always)]
    pub fn dma_12(&self) -> Dma12R {
        Dma12R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DMA 13"]
    #[inline(always)]
    pub fn dma_13(&self) -> Dma13R {
        Dma13R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR30")
            .field("dma_9_10", &self.dma_9_10())
            .field("dma_11", &self.dma_11())
            .field("dma_12", &self.dma_12())
            .field("dma_13", &self.dma_13())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - OR of DMA 9 and 10"]
    #[inline(always)]
    #[must_use]
    pub fn dma_9_10(&mut self) -> Dma9_10W<GicdItargetsr30Spec> {
        Dma9_10W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DMA 11"]
    #[inline(always)]
    #[must_use]
    pub fn dma_11(&mut self) -> Dma11W<GicdItargetsr30Spec> {
        Dma11W::new(self, 8)
    }
    #[doc = "Bits 16:23 - DMA 12"]
    #[inline(always)]
    #[must_use]
    pub fn dma_12(&mut self) -> Dma12W<GicdItargetsr30Spec> {
        Dma12W::new(self, 16)
    }
    #[doc = "Bits 24:31 - DMA 13"]
    #[inline(always)]
    #[must_use]
    pub fn dma_13(&mut self) -> Dma13W<GicdItargetsr30Spec> {
        Dma13W::new(self, 24)
    }
}
#[doc = "Interrupt Processor Target 120 - 123\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdItargetsr30Spec;
impl crate::RegisterSpec for GicdItargetsr30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr30::R`](R) reader structure"]
impl crate::Readable for GicdItargetsr30Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr30::W`](W) writer structure"]
impl crate::Writable for GicdItargetsr30Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR30 to value 0"]
impl crate::Resettable for GicdItargetsr30Spec {
    const RESET_VALUE: u32 = 0;
}

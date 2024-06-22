#[doc = "Register `GICD_ITARGETSR28` reader"]
pub type R = crate::R<GicdItargetsr28Spec>;
#[doc = "Register `GICD_ITARGETSR28` writer"]
pub type W = crate::W<GicdItargetsr28Spec>;
#[doc = "Field `DMA_0` reader - DMA 0"]
pub type Dma0R = crate::FieldReader;
#[doc = "Field `DMA_0` writer - DMA 0"]
pub type Dma0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DMA_1` reader - DMA 1"]
pub type Dma1R = crate::FieldReader;
#[doc = "Field `DMA_1` writer - DMA 1"]
pub type Dma1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DMA_2` reader - DMA 2"]
pub type Dma2R = crate::FieldReader;
#[doc = "Field `DMA_2` writer - DMA 2"]
pub type Dma2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DMA_3` reader - DMA 3"]
pub type Dma3R = crate::FieldReader;
#[doc = "Field `DMA_3` writer - DMA 3"]
pub type Dma3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DMA 0"]
    #[inline(always)]
    pub fn dma_0(&self) -> Dma0R {
        Dma0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DMA 1"]
    #[inline(always)]
    pub fn dma_1(&self) -> Dma1R {
        Dma1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DMA 2"]
    #[inline(always)]
    pub fn dma_2(&self) -> Dma2R {
        Dma2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DMA 3"]
    #[inline(always)]
    pub fn dma_3(&self) -> Dma3R {
        Dma3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR28")
            .field("dma_0", &self.dma_0())
            .field("dma_1", &self.dma_1())
            .field("dma_2", &self.dma_2())
            .field("dma_3", &self.dma_3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - DMA 0"]
    #[inline(always)]
    #[must_use]
    pub fn dma_0(&mut self) -> Dma0W<GicdItargetsr28Spec> {
        Dma0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DMA 1"]
    #[inline(always)]
    #[must_use]
    pub fn dma_1(&mut self) -> Dma1W<GicdItargetsr28Spec> {
        Dma1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - DMA 2"]
    #[inline(always)]
    #[must_use]
    pub fn dma_2(&mut self) -> Dma2W<GicdItargetsr28Spec> {
        Dma2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - DMA 3"]
    #[inline(always)]
    #[must_use]
    pub fn dma_3(&mut self) -> Dma3W<GicdItargetsr28Spec> {
        Dma3W::new(self, 24)
    }
}
#[doc = "Interrupt Processor Target 112 - 115\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdItargetsr28Spec;
impl crate::RegisterSpec for GicdItargetsr28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr28::R`](R) reader structure"]
impl crate::Readable for GicdItargetsr28Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr28::W`](W) writer structure"]
impl crate::Writable for GicdItargetsr28Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR28 to value 0"]
impl crate::Resettable for GicdItargetsr28Spec {
    const RESET_VALUE: u32 = 0;
}

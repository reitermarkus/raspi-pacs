#[doc = "Register `GICD_ITARGETSR29` reader"]
pub type R = crate::R<GicdItargetsr29Spec>;
#[doc = "Register `GICD_ITARGETSR29` writer"]
pub type W = crate::W<GicdItargetsr29Spec>;
#[doc = "Field `DMA_4` reader - DMA 4"]
pub type Dma4R = crate::FieldReader;
#[doc = "Field `DMA_4` writer - DMA 4"]
pub type Dma4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DMA_5` reader - DMA 5"]
pub type Dma5R = crate::FieldReader;
#[doc = "Field `DMA_5` writer - DMA 5"]
pub type Dma5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DMA_6` reader - DMA 6"]
pub type Dma6R = crate::FieldReader;
#[doc = "Field `DMA_6` writer - DMA 6"]
pub type Dma6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DMA_7_8` reader - OR of DMA 7 and 8"]
pub type Dma7_8R = crate::FieldReader;
#[doc = "Field `DMA_7_8` writer - OR of DMA 7 and 8"]
pub type Dma7_8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DMA 4"]
    #[inline(always)]
    pub fn dma_4(&self) -> Dma4R {
        Dma4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DMA 5"]
    #[inline(always)]
    pub fn dma_5(&self) -> Dma5R {
        Dma5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DMA 6"]
    #[inline(always)]
    pub fn dma_6(&self) -> Dma6R {
        Dma6R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - OR of DMA 7 and 8"]
    #[inline(always)]
    pub fn dma_7_8(&self) -> Dma7_8R {
        Dma7_8R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR29")
            .field("dma_4", &self.dma_4())
            .field("dma_5", &self.dma_5())
            .field("dma_6", &self.dma_6())
            .field("dma_7_8", &self.dma_7_8())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - DMA 4"]
    #[inline(always)]
    #[must_use]
    pub fn dma_4(&mut self) -> Dma4W<GicdItargetsr29Spec> {
        Dma4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DMA 5"]
    #[inline(always)]
    #[must_use]
    pub fn dma_5(&mut self) -> Dma5W<GicdItargetsr29Spec> {
        Dma5W::new(self, 8)
    }
    #[doc = "Bits 16:23 - DMA 6"]
    #[inline(always)]
    #[must_use]
    pub fn dma_6(&mut self) -> Dma6W<GicdItargetsr29Spec> {
        Dma6W::new(self, 16)
    }
    #[doc = "Bits 24:31 - OR of DMA 7 and 8"]
    #[inline(always)]
    #[must_use]
    pub fn dma_7_8(&mut self) -> Dma7_8W<GicdItargetsr29Spec> {
        Dma7_8W::new(self, 24)
    }
}
#[doc = "Interrupt Processor Target 116 - 119\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr29::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr29::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdItargetsr29Spec;
impl crate::RegisterSpec for GicdItargetsr29Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr29::R`](R) reader structure"]
impl crate::Readable for GicdItargetsr29Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr29::W`](W) writer structure"]
impl crate::Writable for GicdItargetsr29Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR29 to value 0"]
impl crate::Resettable for GicdItargetsr29Spec {
    const RESET_VALUE: u32 = 0;
}

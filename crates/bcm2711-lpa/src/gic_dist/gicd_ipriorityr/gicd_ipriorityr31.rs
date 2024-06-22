#[doc = "Register `GICD_IPRIORITYR31` reader"]
pub type R = crate::R<GicdIpriorityr31Spec>;
#[doc = "Register `GICD_IPRIORITYR31` writer"]
pub type W = crate::W<GicdIpriorityr31Spec>;
#[doc = "Field `DMA_14` reader - DMA 14"]
pub type Dma14R = crate::FieldReader;
#[doc = "Field `DMA_14` writer - DMA 14"]
pub type Dma14W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AUX` reader - OR of UART1, SPI1 and SPI2"]
pub type AuxR = crate::FieldReader;
#[doc = "Field `AUX` writer - OR of UART1, SPI1 and SPI2"]
pub type AuxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ARM` reader - ARM"]
pub type ArmR = crate::FieldReader;
#[doc = "Field `ARM` writer - ARM"]
pub type ArmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DMA_15` reader - DMA 15"]
pub type Dma15R = crate::FieldReader;
#[doc = "Field `DMA_15` writer - DMA 15"]
pub type Dma15W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DMA 14"]
    #[inline(always)]
    pub fn dma_14(&self) -> Dma14R {
        Dma14R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - OR of UART1, SPI1 and SPI2"]
    #[inline(always)]
    pub fn aux(&self) -> AuxR {
        AuxR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ARM"]
    #[inline(always)]
    pub fn arm(&self) -> ArmR {
        ArmR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DMA 15"]
    #[inline(always)]
    pub fn dma_15(&self) -> Dma15R {
        Dma15R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR31")
            .field("dma_14", &self.dma_14())
            .field("aux", &self.aux())
            .field("arm", &self.arm())
            .field("dma_15", &self.dma_15())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - DMA 14"]
    #[inline(always)]
    #[must_use]
    pub fn dma_14(&mut self) -> Dma14W<GicdIpriorityr31Spec> {
        Dma14W::new(self, 0)
    }
    #[doc = "Bits 8:15 - OR of UART1, SPI1 and SPI2"]
    #[inline(always)]
    #[must_use]
    pub fn aux(&mut self) -> AuxW<GicdIpriorityr31Spec> {
        AuxW::new(self, 8)
    }
    #[doc = "Bits 16:23 - ARM"]
    #[inline(always)]
    #[must_use]
    pub fn arm(&mut self) -> ArmW<GicdIpriorityr31Spec> {
        ArmW::new(self, 16)
    }
    #[doc = "Bits 24:31 - DMA 15"]
    #[inline(always)]
    #[must_use]
    pub fn dma_15(&mut self) -> Dma15W<GicdIpriorityr31Spec> {
        Dma15W::new(self, 24)
    }
}
#[doc = "Interrupt Priority 124 - 127 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr31::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr31::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIpriorityr31Spec;
impl crate::RegisterSpec for GicdIpriorityr31Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr31::R`](R) reader structure"]
impl crate::Readable for GicdIpriorityr31Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr31::W`](W) writer structure"]
impl crate::Writable for GicdIpriorityr31Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR31 to value 0"]
impl crate::Resettable for GicdIpriorityr31Spec {
    const RESET_VALUE: u32 = 0;
}

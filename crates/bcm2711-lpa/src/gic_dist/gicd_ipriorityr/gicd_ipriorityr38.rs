#[doc = "Register `GICD_IPRIORITYR38` reader"]
pub type R = crate::R<GicdIpriorityr38Spec>;
#[doc = "Register `GICD_IPRIORITYR38` writer"]
pub type W = crate::W<GicdIpriorityr38Spec>;
#[doc = "Field `SDHOST` reader - SDHOST"]
pub type SdhostR = crate::FieldReader;
#[doc = "Field `SDHOST` writer - SDHOST"]
pub type SdhostW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `UART` reader - OR of all PL011 UARTs"]
pub type UartR = crate::FieldReader;
#[doc = "Field `UART` writer - OR of all PL011 UARTs"]
pub type UartW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ETH_PCIE` reader - OR of all ETH_PCIe L2"]
pub type EthPcieR = crate::FieldReader;
#[doc = "Field `ETH_PCIE` writer - OR of all ETH_PCIe L2"]
pub type EthPcieW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VEC` reader - VEC"]
pub type VecR = crate::FieldReader;
#[doc = "Field `VEC` writer - VEC"]
pub type VecW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SDHOST"]
    #[inline(always)]
    pub fn sdhost(&self) -> SdhostR {
        SdhostR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - OR of all PL011 UARTs"]
    #[inline(always)]
    pub fn uart(&self) -> UartR {
        UartR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - OR of all ETH_PCIe L2"]
    #[inline(always)]
    pub fn eth_pcie(&self) -> EthPcieR {
        EthPcieR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - VEC"]
    #[inline(always)]
    pub fn vec(&self) -> VecR {
        VecR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR38")
            .field("sdhost", &self.sdhost())
            .field("uart", &self.uart())
            .field("eth_pcie", &self.eth_pcie())
            .field("vec", &self.vec())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - SDHOST"]
    #[inline(always)]
    #[must_use]
    pub fn sdhost(&mut self) -> SdhostW<GicdIpriorityr38Spec> {
        SdhostW::new(self, 0)
    }
    #[doc = "Bits 8:15 - OR of all PL011 UARTs"]
    #[inline(always)]
    #[must_use]
    pub fn uart(&mut self) -> UartW<GicdIpriorityr38Spec> {
        UartW::new(self, 8)
    }
    #[doc = "Bits 16:23 - OR of all ETH_PCIe L2"]
    #[inline(always)]
    #[must_use]
    pub fn eth_pcie(&mut self) -> EthPcieW<GicdIpriorityr38Spec> {
        EthPcieW::new(self, 16)
    }
    #[doc = "Bits 24:31 - VEC"]
    #[inline(always)]
    #[must_use]
    pub fn vec(&mut self) -> VecW<GicdIpriorityr38Spec> {
        VecW::new(self, 24)
    }
}
#[doc = "Interrupt Priority 152 - 155 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr38::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr38::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIpriorityr38Spec;
impl crate::RegisterSpec for GicdIpriorityr38Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr38::R`](R) reader structure"]
impl crate::Readable for GicdIpriorityr38Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr38::W`](W) writer structure"]
impl crate::Writable for GicdIpriorityr38Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR38 to value 0"]
impl crate::Resettable for GicdIpriorityr38Spec {
    const RESET_VALUE: u32 = 0;
}

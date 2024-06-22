#[doc = "Register `GICD_ITARGETSR39` reader"]
pub type R = crate::R<GicdItargetsr39Spec>;
#[doc = "Register `GICD_ITARGETSR39` writer"]
pub type W = crate::W<GicdItargetsr39Spec>;
#[doc = "Field `CPG` reader - CPG"]
pub type CpgR = crate::FieldReader;
#[doc = "Field `CPG` writer - CPG"]
pub type CpgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RNG` reader - RNG"]
pub type RngR = crate::FieldReader;
#[doc = "Field `RNG` writer - RNG"]
pub type RngW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EMMC` reader - OR of EMMC and EMMC2"]
pub type EmmcR = crate::FieldReader;
#[doc = "Field `EMMC` writer - OR of EMMC and EMMC2"]
pub type EmmcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ETH_PCIE_SECURE` reader - ETH_PCIe secure"]
pub type EthPcieSecureR = crate::FieldReader;
#[doc = "Field `ETH_PCIE_SECURE` writer - ETH_PCIe secure"]
pub type EthPcieSecureW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - CPG"]
    #[inline(always)]
    pub fn cpg(&self) -> CpgR {
        CpgR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - RNG"]
    #[inline(always)]
    pub fn rng(&self) -> RngR {
        RngR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - OR of EMMC and EMMC2"]
    #[inline(always)]
    pub fn emmc(&self) -> EmmcR {
        EmmcR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - ETH_PCIe secure"]
    #[inline(always)]
    pub fn eth_pcie_secure(&self) -> EthPcieSecureR {
        EthPcieSecureR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR39")
            .field("cpg", &self.cpg())
            .field("rng", &self.rng())
            .field("emmc", &self.emmc())
            .field("eth_pcie_secure", &self.eth_pcie_secure())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - CPG"]
    #[inline(always)]
    #[must_use]
    pub fn cpg(&mut self) -> CpgW<GicdItargetsr39Spec> {
        CpgW::new(self, 0)
    }
    #[doc = "Bits 8:15 - RNG"]
    #[inline(always)]
    #[must_use]
    pub fn rng(&mut self) -> RngW<GicdItargetsr39Spec> {
        RngW::new(self, 8)
    }
    #[doc = "Bits 16:23 - OR of EMMC and EMMC2"]
    #[inline(always)]
    #[must_use]
    pub fn emmc(&mut self) -> EmmcW<GicdItargetsr39Spec> {
        EmmcW::new(self, 16)
    }
    #[doc = "Bits 24:31 - ETH_PCIe secure"]
    #[inline(always)]
    #[must_use]
    pub fn eth_pcie_secure(&mut self) -> EthPcieSecureW<GicdItargetsr39Spec> {
        EthPcieSecureW::new(self, 24)
    }
}
#[doc = "Interrupt Processor Target 156 - 159\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr39::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr39::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdItargetsr39Spec;
impl crate::RegisterSpec for GicdItargetsr39Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr39::R`](R) reader structure"]
impl crate::Readable for GicdItargetsr39Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr39::W`](W) writer structure"]
impl crate::Writable for GicdItargetsr39Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR39 to value 0"]
impl crate::Resettable for GicdItargetsr39Spec {
    const RESET_VALUE: u32 = 0;
}

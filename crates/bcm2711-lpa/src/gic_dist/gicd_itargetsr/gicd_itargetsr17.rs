#[doc = "Register `GICD_ITARGETSR17` reader"]
pub type R = crate::R<GicdItargetsr17Spec>;
#[doc = "Register `GICD_ITARGETSR17` writer"]
pub type W = crate::W<GicdItargetsr17Spec>;
#[doc = "Field `VPU0_HALTED` reader - VPU0 halted"]
pub type Vpu0HaltedR = crate::FieldReader;
#[doc = "Field `VPU0_HALTED` writer - VPU0 halted"]
pub type Vpu0HaltedW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VPU1_HALTED` reader - VPU1 halted"]
pub type Vpu1HaltedR = crate::FieldReader;
#[doc = "Field `VPU1_HALTED` writer - VPU1 halted"]
pub type Vpu1HaltedW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ARM_ADDRESS_ERROR` reader - ARM address error"]
pub type ArmAddressErrorR = crate::FieldReader;
#[doc = "Field `ARM_ADDRESS_ERROR` writer - ARM address error"]
pub type ArmAddressErrorW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ARM_AXI_ERROR` reader - ARM AXI error"]
pub type ArmAxiErrorR = crate::FieldReader;
#[doc = "Field `ARM_AXI_ERROR` writer - ARM AXI error"]
pub type ArmAxiErrorW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - VPU0 halted"]
    #[inline(always)]
    pub fn vpu0_halted(&self) -> Vpu0HaltedR {
        Vpu0HaltedR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - VPU1 halted"]
    #[inline(always)]
    pub fn vpu1_halted(&self) -> Vpu1HaltedR {
        Vpu1HaltedR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ARM address error"]
    #[inline(always)]
    pub fn arm_address_error(&self) -> ArmAddressErrorR {
        ArmAddressErrorR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - ARM AXI error"]
    #[inline(always)]
    pub fn arm_axi_error(&self) -> ArmAxiErrorR {
        ArmAxiErrorR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR17")
            .field("vpu0_halted", &self.vpu0_halted())
            .field("vpu1_halted", &self.vpu1_halted())
            .field("arm_address_error", &self.arm_address_error())
            .field("arm_axi_error", &self.arm_axi_error())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - VPU0 halted"]
    #[inline(always)]
    #[must_use]
    pub fn vpu0_halted(&mut self) -> Vpu0HaltedW<GicdItargetsr17Spec> {
        Vpu0HaltedW::new(self, 0)
    }
    #[doc = "Bits 8:15 - VPU1 halted"]
    #[inline(always)]
    #[must_use]
    pub fn vpu1_halted(&mut self) -> Vpu1HaltedW<GicdItargetsr17Spec> {
        Vpu1HaltedW::new(self, 8)
    }
    #[doc = "Bits 16:23 - ARM address error"]
    #[inline(always)]
    #[must_use]
    pub fn arm_address_error(&mut self) -> ArmAddressErrorW<GicdItargetsr17Spec> {
        ArmAddressErrorW::new(self, 16)
    }
    #[doc = "Bits 24:31 - ARM AXI error"]
    #[inline(always)]
    #[must_use]
    pub fn arm_axi_error(&mut self) -> ArmAxiErrorW<GicdItargetsr17Spec> {
        ArmAxiErrorW::new(self, 24)
    }
}
#[doc = "Interrupt Processor Target 68 - 71\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr17::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr17::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdItargetsr17Spec;
impl crate::RegisterSpec for GicdItargetsr17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr17::R`](R) reader structure"]
impl crate::Readable for GicdItargetsr17Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr17::W`](W) writer structure"]
impl crate::Writable for GicdItargetsr17Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR17 to value 0"]
impl crate::Resettable for GicdItargetsr17Spec {
    const RESET_VALUE: u32 = 0;
}

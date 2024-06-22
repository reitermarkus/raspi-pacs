#[doc = "Register `ENABLE_BASIC` reader"]
pub type R = crate::R<EnableBasicSpec>;
#[doc = "Register `ENABLE_BASIC` writer"]
pub type W = crate::W<EnableBasicSpec>;
#[doc = "Field `TIMER` reader - ARMC Timer"]
pub type TimerR = crate::BitReader;
#[doc = "Field `TIMER` writer - ARMC Timer"]
pub type TimerW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `MAILBOX` reader - Mailbox"]
pub type MailboxR = crate::BitReader;
#[doc = "Field `MAILBOX` writer - Mailbox"]
pub type MailboxW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `DOORBELL0` reader - Doorbell 0"]
pub type Doorbell0R = crate::BitReader;
#[doc = "Field `DOORBELL0` writer - Doorbell 0"]
pub type Doorbell0W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `DOORBELL1` reader - Doorbell 1"]
pub type Doorbell1R = crate::BitReader;
#[doc = "Field `DOORBELL1` writer - Doorbell 1"]
pub type Doorbell1W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `VPU0_HALTED` reader - VPU0 halted"]
pub type Vpu0HaltedR = crate::BitReader;
#[doc = "Field `VPU0_HALTED` writer - VPU0 halted"]
pub type Vpu0HaltedW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `VPU1_HALTED` reader - VPU1 halted"]
pub type Vpu1HaltedR = crate::BitReader;
#[doc = "Field `VPU1_HALTED` writer - VPU1 halted"]
pub type Vpu1HaltedW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `ARM_ADDRESS_ERROR` reader - ARM address error"]
pub type ArmAddressErrorR = crate::BitReader;
#[doc = "Field `ARM_ADDRESS_ERROR` writer - ARM address error"]
pub type ArmAddressErrorW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `ARM_AXI_ERROR` reader - ARM AXI error"]
pub type ArmAxiErrorR = crate::BitReader;
#[doc = "Field `ARM_AXI_ERROR` writer - ARM AXI error"]
pub type ArmAxiErrorW<'a, REG> = crate::BitWriter1S<'a, REG>;
impl R {
    #[doc = "Bit 0 - ARMC Timer"]
    #[inline(always)]
    pub fn timer(&self) -> TimerR {
        TimerR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mailbox"]
    #[inline(always)]
    pub fn mailbox(&self) -> MailboxR {
        MailboxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Doorbell 0"]
    #[inline(always)]
    pub fn doorbell0(&self) -> Doorbell0R {
        Doorbell0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Doorbell 1"]
    #[inline(always)]
    pub fn doorbell1(&self) -> Doorbell1R {
        Doorbell1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VPU0 halted"]
    #[inline(always)]
    pub fn vpu0_halted(&self) -> Vpu0HaltedR {
        Vpu0HaltedR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - VPU1 halted"]
    #[inline(always)]
    pub fn vpu1_halted(&self) -> Vpu1HaltedR {
        Vpu1HaltedR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ARM address error"]
    #[inline(always)]
    pub fn arm_address_error(&self) -> ArmAddressErrorR {
        ArmAddressErrorR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ARM AXI error"]
    #[inline(always)]
    pub fn arm_axi_error(&self) -> ArmAxiErrorR {
        ArmAxiErrorR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENABLE_BASIC")
            .field("timer", &self.timer())
            .field("mailbox", &self.mailbox())
            .field("doorbell0", &self.doorbell0())
            .field("doorbell1", &self.doorbell1())
            .field("vpu0_halted", &self.vpu0_halted())
            .field("vpu1_halted", &self.vpu1_halted())
            .field("arm_address_error", &self.arm_address_error())
            .field("arm_axi_error", &self.arm_axi_error())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - ARMC Timer"]
    #[inline(always)]
    #[must_use]
    pub fn timer(&mut self) -> TimerW<EnableBasicSpec> {
        TimerW::new(self, 0)
    }
    #[doc = "Bit 1 - Mailbox"]
    #[inline(always)]
    #[must_use]
    pub fn mailbox(&mut self) -> MailboxW<EnableBasicSpec> {
        MailboxW::new(self, 1)
    }
    #[doc = "Bit 2 - Doorbell 0"]
    #[inline(always)]
    #[must_use]
    pub fn doorbell0(&mut self) -> Doorbell0W<EnableBasicSpec> {
        Doorbell0W::new(self, 2)
    }
    #[doc = "Bit 3 - Doorbell 1"]
    #[inline(always)]
    #[must_use]
    pub fn doorbell1(&mut self) -> Doorbell1W<EnableBasicSpec> {
        Doorbell1W::new(self, 3)
    }
    #[doc = "Bit 4 - VPU0 halted"]
    #[inline(always)]
    #[must_use]
    pub fn vpu0_halted(&mut self) -> Vpu0HaltedW<EnableBasicSpec> {
        Vpu0HaltedW::new(self, 4)
    }
    #[doc = "Bit 5 - VPU1 halted"]
    #[inline(always)]
    #[must_use]
    pub fn vpu1_halted(&mut self) -> Vpu1HaltedW<EnableBasicSpec> {
        Vpu1HaltedW::new(self, 5)
    }
    #[doc = "Bit 6 - ARM address error"]
    #[inline(always)]
    #[must_use]
    pub fn arm_address_error(&mut self) -> ArmAddressErrorW<EnableBasicSpec> {
        ArmAddressErrorW::new(self, 6)
    }
    #[doc = "Bit 7 - ARM AXI error"]
    #[inline(always)]
    #[must_use]
    pub fn arm_axi_error(&mut self) -> ArmAxiErrorW<EnableBasicSpec> {
        ArmAxiErrorW::new(self, 7)
    }
}
#[doc = "Enable basic interrupts\n\nYou can [`read`](crate::Reg::read) this register and get [`enable_basic::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable_basic::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnableBasicSpec;
impl crate::RegisterSpec for EnableBasicSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enable_basic::R`](R) reader structure"]
impl crate::Readable for EnableBasicSpec {}
#[doc = "`write(|w| ..)` method takes [`enable_basic::W`](W) writer structure"]
impl crate::Writable for EnableBasicSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets ENABLE_BASIC to value 0"]
impl crate::Resettable for EnableBasicSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `BASIC_PENDING` reader"]
pub type R = crate::R<BasicPendingSpec>;
#[doc = "Field `TIMER` reader - ARMC Timer"]
pub type TimerR = crate::BitReader;
#[doc = "Field `MAILBOX` reader - Mailbox"]
pub type MailboxR = crate::BitReader;
#[doc = "Field `DOORBELL0` reader - Doorbell 0"]
pub type Doorbell0R = crate::BitReader;
#[doc = "Field `DOORBELL1` reader - Doorbell 1"]
pub type Doorbell1R = crate::BitReader;
#[doc = "Field `VPU0_HALTED` reader - VPU0 halted"]
pub type Vpu0HaltedR = crate::BitReader;
#[doc = "Field `VPU1_HALTED` reader - VPU1 halted"]
pub type Vpu1HaltedR = crate::BitReader;
#[doc = "Field `ARM_ADDRESS_ERROR` reader - ARM address error"]
pub type ArmAddressErrorR = crate::BitReader;
#[doc = "Field `ARM_AXI_ERROR` reader - ARM AXI error"]
pub type ArmAxiErrorR = crate::BitReader;
#[doc = "Field `PENDING_1` reader - One or more bits are set in PENDING_1 (ignores 7, 9, 10, 18, 19)"]
pub type Pending1R = crate::BitReader;
#[doc = "Field `PENDING_2` reader - One or more bits are set in PENDING_2 (ignores 53 - 57, 62)"]
pub type Pending2R = crate::BitReader;
#[doc = "Field `INT7` reader - Interrupt 7"]
pub type Int7R = crate::BitReader;
#[doc = "Field `INT9` reader - Interrupt 9"]
pub type Int9R = crate::BitReader;
#[doc = "Field `INT10` reader - Interrupt 10"]
pub type Int10R = crate::BitReader;
#[doc = "Field `INT18` reader - Interrupt 18"]
pub type Int18R = crate::BitReader;
#[doc = "Field `INT19` reader - Interrupt 19"]
pub type Int19R = crate::BitReader;
#[doc = "Field `INT53` reader - Interrupt 53"]
pub type Int53R = crate::BitReader;
#[doc = "Field `INT54` reader - Interrupt 54"]
pub type Int54R = crate::BitReader;
#[doc = "Field `INT55` reader - Interrupt 55"]
pub type Int55R = crate::BitReader;
#[doc = "Field `INT56` reader - Interrupt 56"]
pub type Int56R = crate::BitReader;
#[doc = "Field `INT57` reader - Interrupt 57"]
pub type Int57R = crate::BitReader;
#[doc = "Field `INT62` reader - Interrupt 62"]
pub type Int62R = crate::BitReader;
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
    #[doc = "Bit 8 - One or more bits are set in PENDING_1 (ignores 7, 9, 10, 18, 19)"]
    #[inline(always)]
    pub fn pending_1(&self) -> Pending1R {
        Pending1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - One or more bits are set in PENDING_2 (ignores 53 - 57, 62)"]
    #[inline(always)]
    pub fn pending_2(&self) -> Pending2R {
        Pending2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt 7"]
    #[inline(always)]
    pub fn int7(&self) -> Int7R {
        Int7R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt 9"]
    #[inline(always)]
    pub fn int9(&self) -> Int9R {
        Int9R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt 10"]
    #[inline(always)]
    pub fn int10(&self) -> Int10R {
        Int10R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt 18"]
    #[inline(always)]
    pub fn int18(&self) -> Int18R {
        Int18R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt 19"]
    #[inline(always)]
    pub fn int19(&self) -> Int19R {
        Int19R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt 53"]
    #[inline(always)]
    pub fn int53(&self) -> Int53R {
        Int53R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt 54"]
    #[inline(always)]
    pub fn int54(&self) -> Int54R {
        Int54R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt 55"]
    #[inline(always)]
    pub fn int55(&self) -> Int55R {
        Int55R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt 56"]
    #[inline(always)]
    pub fn int56(&self) -> Int56R {
        Int56R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt 57"]
    #[inline(always)]
    pub fn int57(&self) -> Int57R {
        Int57R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt 62"]
    #[inline(always)]
    pub fn int62(&self) -> Int62R {
        Int62R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BASIC_PENDING")
            .field("timer", &self.timer())
            .field("mailbox", &self.mailbox())
            .field("doorbell0", &self.doorbell0())
            .field("doorbell1", &self.doorbell1())
            .field("vpu0_halted", &self.vpu0_halted())
            .field("vpu1_halted", &self.vpu1_halted())
            .field("arm_address_error", &self.arm_address_error())
            .field("arm_axi_error", &self.arm_axi_error())
            .field("pending_1", &self.pending_1())
            .field("pending_2", &self.pending_2())
            .field("int7", &self.int7())
            .field("int9", &self.int9())
            .field("int10", &self.int10())
            .field("int18", &self.int18())
            .field("int19", &self.int19())
            .field("int53", &self.int53())
            .field("int54", &self.int54())
            .field("int55", &self.int55())
            .field("int56", &self.int56())
            .field("int57", &self.int57())
            .field("int62", &self.int62())
            .finish()
    }
}
#[doc = "Basic pending info\n\nYou can [`read`](crate::Reg::read) this register and get [`basic_pending::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BasicPendingSpec;
impl crate::RegisterSpec for BasicPendingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`basic_pending::R`](R) reader structure"]
impl crate::Readable for BasicPendingSpec {}
#[doc = "`reset()` method sets BASIC_PENDING to value 0"]
impl crate::Resettable for BasicPendingSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `GICD_SPISR1` reader"]
pub type R = crate::R<GicdSpisr1Spec>;
#[doc = "Register `GICD_SPISR1` writer"]
pub type W = crate::W<GicdSpisr1Spec>;
#[doc = "Field `TIMER` reader - ARMC Timer"]
pub type TimerR = crate::BitReader;
#[doc = "Field `TIMER` writer - ARMC Timer"]
pub type TimerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAILBOX` reader - Mailbox"]
pub type MailboxR = crate::BitReader;
#[doc = "Field `MAILBOX` writer - Mailbox"]
pub type MailboxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOORBELL0` reader - Doorbell 0"]
pub type Doorbell0R = crate::BitReader;
#[doc = "Field `DOORBELL0` writer - Doorbell 0"]
pub type Doorbell0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOORBELL1` reader - Doorbell 1"]
pub type Doorbell1R = crate::BitReader;
#[doc = "Field `DOORBELL1` writer - Doorbell 1"]
pub type Doorbell1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VPU0_HALTED` reader - VPU0 halted"]
pub type Vpu0HaltedR = crate::BitReader;
#[doc = "Field `VPU0_HALTED` writer - VPU0 halted"]
pub type Vpu0HaltedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VPU1_HALTED` reader - VPU1 halted"]
pub type Vpu1HaltedR = crate::BitReader;
#[doc = "Field `VPU1_HALTED` writer - VPU1 halted"]
pub type Vpu1HaltedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARM_ADDRESS_ERROR` reader - ARM address error"]
pub type ArmAddressErrorR = crate::BitReader;
#[doc = "Field `ARM_ADDRESS_ERROR` writer - ARM address error"]
pub type ArmAddressErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARM_AXI_ERROR` reader - ARM AXI error"]
pub type ArmAxiErrorR = crate::BitReader;
#[doc = "Field `ARM_AXI_ERROR` writer - ARM AXI error"]
pub type ArmAxiErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI0` reader - Software interrupt 0"]
pub type Swi0R = crate::BitReader;
#[doc = "Field `SWI0` writer - Software interrupt 0"]
pub type Swi0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI1` reader - Software interrupt 1"]
pub type Swi1R = crate::BitReader;
#[doc = "Field `SWI1` writer - Software interrupt 1"]
pub type Swi1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI2` reader - Software interrupt 2"]
pub type Swi2R = crate::BitReader;
#[doc = "Field `SWI2` writer - Software interrupt 2"]
pub type Swi2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI3` reader - Software interrupt 3"]
pub type Swi3R = crate::BitReader;
#[doc = "Field `SWI3` writer - Software interrupt 3"]
pub type Swi3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI4` reader - Software interrupt 4"]
pub type Swi4R = crate::BitReader;
#[doc = "Field `SWI4` writer - Software interrupt 4"]
pub type Swi4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI5` reader - Software interrupt 5"]
pub type Swi5R = crate::BitReader;
#[doc = "Field `SWI5` writer - Software interrupt 5"]
pub type Swi5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI6` reader - Software interrupt 6"]
pub type Swi6R = crate::BitReader;
#[doc = "Field `SWI6` writer - Software interrupt 6"]
pub type Swi6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI7` reader - Software interrupt 7"]
pub type Swi7R = crate::BitReader;
#[doc = "Field `SWI7` writer - Software interrupt 7"]
pub type Swi7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI80` reader - Shared interrupt 80"]
pub type Spi80R = crate::BitReader;
#[doc = "Field `SPI80` writer - Shared interrupt 80"]
pub type Spi80W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI81` reader - Shared interrupt 81"]
pub type Spi81R = crate::BitReader;
#[doc = "Field `SPI81` writer - Shared interrupt 81"]
pub type Spi81W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI82` reader - Shared interrupt 82"]
pub type Spi82R = crate::BitReader;
#[doc = "Field `SPI82` writer - Shared interrupt 82"]
pub type Spi82W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI83` reader - Shared interrupt 83"]
pub type Spi83R = crate::BitReader;
#[doc = "Field `SPI83` writer - Shared interrupt 83"]
pub type Spi83W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI84` reader - Shared interrupt 84"]
pub type Spi84R = crate::BitReader;
#[doc = "Field `SPI84` writer - Shared interrupt 84"]
pub type Spi84W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI85` reader - Shared interrupt 85"]
pub type Spi85R = crate::BitReader;
#[doc = "Field `SPI85` writer - Shared interrupt 85"]
pub type Spi85W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI86` reader - Shared interrupt 86"]
pub type Spi86R = crate::BitReader;
#[doc = "Field `SPI86` writer - Shared interrupt 86"]
pub type Spi86W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI87` reader - Shared interrupt 87"]
pub type Spi87R = crate::BitReader;
#[doc = "Field `SPI87` writer - Shared interrupt 87"]
pub type Spi87W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI88` reader - Shared interrupt 88"]
pub type Spi88R = crate::BitReader;
#[doc = "Field `SPI88` writer - Shared interrupt 88"]
pub type Spi88W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI89` reader - Shared interrupt 89"]
pub type Spi89R = crate::BitReader;
#[doc = "Field `SPI89` writer - Shared interrupt 89"]
pub type Spi89W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI90` reader - Shared interrupt 90"]
pub type Spi90R = crate::BitReader;
#[doc = "Field `SPI90` writer - Shared interrupt 90"]
pub type Spi90W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI91` reader - Shared interrupt 91"]
pub type Spi91R = crate::BitReader;
#[doc = "Field `SPI91` writer - Shared interrupt 91"]
pub type Spi91W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI92` reader - Shared interrupt 92"]
pub type Spi92R = crate::BitReader;
#[doc = "Field `SPI92` writer - Shared interrupt 92"]
pub type Spi92W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI93` reader - Shared interrupt 93"]
pub type Spi93R = crate::BitReader;
#[doc = "Field `SPI93` writer - Shared interrupt 93"]
pub type Spi93W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI94` reader - Shared interrupt 94"]
pub type Spi94R = crate::BitReader;
#[doc = "Field `SPI94` writer - Shared interrupt 94"]
pub type Spi94W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI95` reader - Shared interrupt 95"]
pub type Spi95R = crate::BitReader;
#[doc = "Field `SPI95` writer - Shared interrupt 95"]
pub type Spi95W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 8 - Software interrupt 0"]
    #[inline(always)]
    pub fn swi0(&self) -> Swi0R {
        Swi0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software interrupt 1"]
    #[inline(always)]
    pub fn swi1(&self) -> Swi1R {
        Swi1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Software interrupt 2"]
    #[inline(always)]
    pub fn swi2(&self) -> Swi2R {
        Swi2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Software interrupt 3"]
    #[inline(always)]
    pub fn swi3(&self) -> Swi3R {
        Swi3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Software interrupt 4"]
    #[inline(always)]
    pub fn swi4(&self) -> Swi4R {
        Swi4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Software interrupt 5"]
    #[inline(always)]
    pub fn swi5(&self) -> Swi5R {
        Swi5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Software interrupt 6"]
    #[inline(always)]
    pub fn swi6(&self) -> Swi6R {
        Swi6R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Software interrupt 7"]
    #[inline(always)]
    pub fn swi7(&self) -> Swi7R {
        Swi7R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Shared interrupt 80"]
    #[inline(always)]
    pub fn spi80(&self) -> Spi80R {
        Spi80R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Shared interrupt 81"]
    #[inline(always)]
    pub fn spi81(&self) -> Spi81R {
        Spi81R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Shared interrupt 82"]
    #[inline(always)]
    pub fn spi82(&self) -> Spi82R {
        Spi82R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Shared interrupt 83"]
    #[inline(always)]
    pub fn spi83(&self) -> Spi83R {
        Spi83R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Shared interrupt 84"]
    #[inline(always)]
    pub fn spi84(&self) -> Spi84R {
        Spi84R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Shared interrupt 85"]
    #[inline(always)]
    pub fn spi85(&self) -> Spi85R {
        Spi85R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Shared interrupt 86"]
    #[inline(always)]
    pub fn spi86(&self) -> Spi86R {
        Spi86R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Shared interrupt 87"]
    #[inline(always)]
    pub fn spi87(&self) -> Spi87R {
        Spi87R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Shared interrupt 88"]
    #[inline(always)]
    pub fn spi88(&self) -> Spi88R {
        Spi88R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Shared interrupt 89"]
    #[inline(always)]
    pub fn spi89(&self) -> Spi89R {
        Spi89R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Shared interrupt 90"]
    #[inline(always)]
    pub fn spi90(&self) -> Spi90R {
        Spi90R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Shared interrupt 91"]
    #[inline(always)]
    pub fn spi91(&self) -> Spi91R {
        Spi91R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Shared interrupt 92"]
    #[inline(always)]
    pub fn spi92(&self) -> Spi92R {
        Spi92R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Shared interrupt 93"]
    #[inline(always)]
    pub fn spi93(&self) -> Spi93R {
        Spi93R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Shared interrupt 94"]
    #[inline(always)]
    pub fn spi94(&self) -> Spi94R {
        Spi94R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Shared interrupt 95"]
    #[inline(always)]
    pub fn spi95(&self) -> Spi95R {
        Spi95R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_SPISR1")
            .field("timer", &self.timer())
            .field("mailbox", &self.mailbox())
            .field("doorbell0", &self.doorbell0())
            .field("doorbell1", &self.doorbell1())
            .field("vpu0_halted", &self.vpu0_halted())
            .field("vpu1_halted", &self.vpu1_halted())
            .field("arm_address_error", &self.arm_address_error())
            .field("arm_axi_error", &self.arm_axi_error())
            .field("swi0", &self.swi0())
            .field("swi1", &self.swi1())
            .field("swi2", &self.swi2())
            .field("swi3", &self.swi3())
            .field("swi4", &self.swi4())
            .field("swi5", &self.swi5())
            .field("swi6", &self.swi6())
            .field("swi7", &self.swi7())
            .field("spi80", &self.spi80())
            .field("spi81", &self.spi81())
            .field("spi82", &self.spi82())
            .field("spi83", &self.spi83())
            .field("spi84", &self.spi84())
            .field("spi85", &self.spi85())
            .field("spi86", &self.spi86())
            .field("spi87", &self.spi87())
            .field("spi88", &self.spi88())
            .field("spi89", &self.spi89())
            .field("spi90", &self.spi90())
            .field("spi91", &self.spi91())
            .field("spi92", &self.spi92())
            .field("spi93", &self.spi93())
            .field("spi94", &self.spi94())
            .field("spi95", &self.spi95())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - ARMC Timer"]
    #[inline(always)]
    #[must_use]
    pub fn timer(&mut self) -> TimerW<GicdSpisr1Spec> {
        TimerW::new(self, 0)
    }
    #[doc = "Bit 1 - Mailbox"]
    #[inline(always)]
    #[must_use]
    pub fn mailbox(&mut self) -> MailboxW<GicdSpisr1Spec> {
        MailboxW::new(self, 1)
    }
    #[doc = "Bit 2 - Doorbell 0"]
    #[inline(always)]
    #[must_use]
    pub fn doorbell0(&mut self) -> Doorbell0W<GicdSpisr1Spec> {
        Doorbell0W::new(self, 2)
    }
    #[doc = "Bit 3 - Doorbell 1"]
    #[inline(always)]
    #[must_use]
    pub fn doorbell1(&mut self) -> Doorbell1W<GicdSpisr1Spec> {
        Doorbell1W::new(self, 3)
    }
    #[doc = "Bit 4 - VPU0 halted"]
    #[inline(always)]
    #[must_use]
    pub fn vpu0_halted(&mut self) -> Vpu0HaltedW<GicdSpisr1Spec> {
        Vpu0HaltedW::new(self, 4)
    }
    #[doc = "Bit 5 - VPU1 halted"]
    #[inline(always)]
    #[must_use]
    pub fn vpu1_halted(&mut self) -> Vpu1HaltedW<GicdSpisr1Spec> {
        Vpu1HaltedW::new(self, 5)
    }
    #[doc = "Bit 6 - ARM address error"]
    #[inline(always)]
    #[must_use]
    pub fn arm_address_error(&mut self) -> ArmAddressErrorW<GicdSpisr1Spec> {
        ArmAddressErrorW::new(self, 6)
    }
    #[doc = "Bit 7 - ARM AXI error"]
    #[inline(always)]
    #[must_use]
    pub fn arm_axi_error(&mut self) -> ArmAxiErrorW<GicdSpisr1Spec> {
        ArmAxiErrorW::new(self, 7)
    }
    #[doc = "Bit 8 - Software interrupt 0"]
    #[inline(always)]
    #[must_use]
    pub fn swi0(&mut self) -> Swi0W<GicdSpisr1Spec> {
        Swi0W::new(self, 8)
    }
    #[doc = "Bit 9 - Software interrupt 1"]
    #[inline(always)]
    #[must_use]
    pub fn swi1(&mut self) -> Swi1W<GicdSpisr1Spec> {
        Swi1W::new(self, 9)
    }
    #[doc = "Bit 10 - Software interrupt 2"]
    #[inline(always)]
    #[must_use]
    pub fn swi2(&mut self) -> Swi2W<GicdSpisr1Spec> {
        Swi2W::new(self, 10)
    }
    #[doc = "Bit 11 - Software interrupt 3"]
    #[inline(always)]
    #[must_use]
    pub fn swi3(&mut self) -> Swi3W<GicdSpisr1Spec> {
        Swi3W::new(self, 11)
    }
    #[doc = "Bit 12 - Software interrupt 4"]
    #[inline(always)]
    #[must_use]
    pub fn swi4(&mut self) -> Swi4W<GicdSpisr1Spec> {
        Swi4W::new(self, 12)
    }
    #[doc = "Bit 13 - Software interrupt 5"]
    #[inline(always)]
    #[must_use]
    pub fn swi5(&mut self) -> Swi5W<GicdSpisr1Spec> {
        Swi5W::new(self, 13)
    }
    #[doc = "Bit 14 - Software interrupt 6"]
    #[inline(always)]
    #[must_use]
    pub fn swi6(&mut self) -> Swi6W<GicdSpisr1Spec> {
        Swi6W::new(self, 14)
    }
    #[doc = "Bit 15 - Software interrupt 7"]
    #[inline(always)]
    #[must_use]
    pub fn swi7(&mut self) -> Swi7W<GicdSpisr1Spec> {
        Swi7W::new(self, 15)
    }
    #[doc = "Bit 16 - Shared interrupt 80"]
    #[inline(always)]
    #[must_use]
    pub fn spi80(&mut self) -> Spi80W<GicdSpisr1Spec> {
        Spi80W::new(self, 16)
    }
    #[doc = "Bit 17 - Shared interrupt 81"]
    #[inline(always)]
    #[must_use]
    pub fn spi81(&mut self) -> Spi81W<GicdSpisr1Spec> {
        Spi81W::new(self, 17)
    }
    #[doc = "Bit 18 - Shared interrupt 82"]
    #[inline(always)]
    #[must_use]
    pub fn spi82(&mut self) -> Spi82W<GicdSpisr1Spec> {
        Spi82W::new(self, 18)
    }
    #[doc = "Bit 19 - Shared interrupt 83"]
    #[inline(always)]
    #[must_use]
    pub fn spi83(&mut self) -> Spi83W<GicdSpisr1Spec> {
        Spi83W::new(self, 19)
    }
    #[doc = "Bit 20 - Shared interrupt 84"]
    #[inline(always)]
    #[must_use]
    pub fn spi84(&mut self) -> Spi84W<GicdSpisr1Spec> {
        Spi84W::new(self, 20)
    }
    #[doc = "Bit 21 - Shared interrupt 85"]
    #[inline(always)]
    #[must_use]
    pub fn spi85(&mut self) -> Spi85W<GicdSpisr1Spec> {
        Spi85W::new(self, 21)
    }
    #[doc = "Bit 22 - Shared interrupt 86"]
    #[inline(always)]
    #[must_use]
    pub fn spi86(&mut self) -> Spi86W<GicdSpisr1Spec> {
        Spi86W::new(self, 22)
    }
    #[doc = "Bit 23 - Shared interrupt 87"]
    #[inline(always)]
    #[must_use]
    pub fn spi87(&mut self) -> Spi87W<GicdSpisr1Spec> {
        Spi87W::new(self, 23)
    }
    #[doc = "Bit 24 - Shared interrupt 88"]
    #[inline(always)]
    #[must_use]
    pub fn spi88(&mut self) -> Spi88W<GicdSpisr1Spec> {
        Spi88W::new(self, 24)
    }
    #[doc = "Bit 25 - Shared interrupt 89"]
    #[inline(always)]
    #[must_use]
    pub fn spi89(&mut self) -> Spi89W<GicdSpisr1Spec> {
        Spi89W::new(self, 25)
    }
    #[doc = "Bit 26 - Shared interrupt 90"]
    #[inline(always)]
    #[must_use]
    pub fn spi90(&mut self) -> Spi90W<GicdSpisr1Spec> {
        Spi90W::new(self, 26)
    }
    #[doc = "Bit 27 - Shared interrupt 91"]
    #[inline(always)]
    #[must_use]
    pub fn spi91(&mut self) -> Spi91W<GicdSpisr1Spec> {
        Spi91W::new(self, 27)
    }
    #[doc = "Bit 28 - Shared interrupt 92"]
    #[inline(always)]
    #[must_use]
    pub fn spi92(&mut self) -> Spi92W<GicdSpisr1Spec> {
        Spi92W::new(self, 28)
    }
    #[doc = "Bit 29 - Shared interrupt 93"]
    #[inline(always)]
    #[must_use]
    pub fn spi93(&mut self) -> Spi93W<GicdSpisr1Spec> {
        Spi93W::new(self, 29)
    }
    #[doc = "Bit 30 - Shared interrupt 94"]
    #[inline(always)]
    #[must_use]
    pub fn spi94(&mut self) -> Spi94W<GicdSpisr1Spec> {
        Spi94W::new(self, 30)
    }
    #[doc = "Bit 31 - Shared interrupt 95"]
    #[inline(always)]
    #[must_use]
    pub fn spi95(&mut self) -> Spi95W<GicdSpisr1Spec> {
        Spi95W::new(self, 31)
    }
}
#[doc = "Shared Peripheral Interrupt Status Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_spisr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_spisr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdSpisr1Spec;
impl crate::RegisterSpec for GicdSpisr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_spisr1::R`](R) reader structure"]
impl crate::Readable for GicdSpisr1Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_spisr1::W`](W) writer structure"]
impl crate::Writable for GicdSpisr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_SPISR1 to value 0"]
impl crate::Resettable for GicdSpisr1Spec {
    const RESET_VALUE: u32 = 0;
}

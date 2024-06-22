#[doc = "Register `GICD_ISACTIVER2` reader"]
pub type R = crate::R<GicdIsactiver2Spec>;
#[doc = "Register `GICD_ISACTIVER2` writer"]
pub type W = crate::W<GicdIsactiver2Spec>;
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
#[doc = "Field `SWI0` reader - Software interrupt 0"]
pub type Swi0R = crate::BitReader;
#[doc = "Field `SWI0` writer - Software interrupt 0"]
pub type Swi0W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SWI1` reader - Software interrupt 1"]
pub type Swi1R = crate::BitReader;
#[doc = "Field `SWI1` writer - Software interrupt 1"]
pub type Swi1W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SWI2` reader - Software interrupt 2"]
pub type Swi2R = crate::BitReader;
#[doc = "Field `SWI2` writer - Software interrupt 2"]
pub type Swi2W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SWI3` reader - Software interrupt 3"]
pub type Swi3R = crate::BitReader;
#[doc = "Field `SWI3` writer - Software interrupt 3"]
pub type Swi3W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SWI4` reader - Software interrupt 4"]
pub type Swi4R = crate::BitReader;
#[doc = "Field `SWI4` writer - Software interrupt 4"]
pub type Swi4W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SWI5` reader - Software interrupt 5"]
pub type Swi5R = crate::BitReader;
#[doc = "Field `SWI5` writer - Software interrupt 5"]
pub type Swi5W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SWI6` reader - Software interrupt 6"]
pub type Swi6R = crate::BitReader;
#[doc = "Field `SWI6` writer - Software interrupt 6"]
pub type Swi6W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SWI7` reader - Software interrupt 7"]
pub type Swi7R = crate::BitReader;
#[doc = "Field `SWI7` writer - Software interrupt 7"]
pub type Swi7W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `INT80` reader - Interrupt 80"]
pub type Int80R = crate::BitReader;
#[doc = "Field `INT80` writer - Interrupt 80"]
pub type Int80W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `INT81` reader - Interrupt 81"]
pub type Int81R = crate::BitReader;
#[doc = "Field `INT81` writer - Interrupt 81"]
pub type Int81W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `INT82` reader - Interrupt 82"]
pub type Int82R = crate::BitReader;
#[doc = "Field `INT82` writer - Interrupt 82"]
pub type Int82W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `INT83` reader - Interrupt 83"]
pub type Int83R = crate::BitReader;
#[doc = "Field `INT83` writer - Interrupt 83"]
pub type Int83W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `INT84` reader - Interrupt 84"]
pub type Int84R = crate::BitReader;
#[doc = "Field `INT84` writer - Interrupt 84"]
pub type Int84W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `INT85` reader - Interrupt 85"]
pub type Int85R = crate::BitReader;
#[doc = "Field `INT85` writer - Interrupt 85"]
pub type Int85W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `INT86` reader - Interrupt 86"]
pub type Int86R = crate::BitReader;
#[doc = "Field `INT86` writer - Interrupt 86"]
pub type Int86W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `INT87` reader - Interrupt 87"]
pub type Int87R = crate::BitReader;
#[doc = "Field `INT87` writer - Interrupt 87"]
pub type Int87W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `INT88` reader - Interrupt 88"]
pub type Int88R = crate::BitReader;
#[doc = "Field `INT88` writer - Interrupt 88"]
pub type Int88W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `INT89` reader - Interrupt 89"]
pub type Int89R = crate::BitReader;
#[doc = "Field `INT89` writer - Interrupt 89"]
pub type Int89W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `INT90` reader - Interrupt 90"]
pub type Int90R = crate::BitReader;
#[doc = "Field `INT90` writer - Interrupt 90"]
pub type Int90W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `INT91` reader - Interrupt 91"]
pub type Int91R = crate::BitReader;
#[doc = "Field `INT91` writer - Interrupt 91"]
pub type Int91W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `INT92` reader - Interrupt 92"]
pub type Int92R = crate::BitReader;
#[doc = "Field `INT92` writer - Interrupt 92"]
pub type Int92W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `INT93` reader - Interrupt 93"]
pub type Int93R = crate::BitReader;
#[doc = "Field `INT93` writer - Interrupt 93"]
pub type Int93W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `INT94` reader - Interrupt 94"]
pub type Int94R = crate::BitReader;
#[doc = "Field `INT94` writer - Interrupt 94"]
pub type Int94W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `INT95` reader - Interrupt 95"]
pub type Int95R = crate::BitReader;
#[doc = "Field `INT95` writer - Interrupt 95"]
pub type Int95W<'a, REG> = crate::BitWriter1S<'a, REG>;
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
    #[doc = "Bit 16 - Interrupt 80"]
    #[inline(always)]
    pub fn int80(&self) -> Int80R {
        Int80R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt 81"]
    #[inline(always)]
    pub fn int81(&self) -> Int81R {
        Int81R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt 82"]
    #[inline(always)]
    pub fn int82(&self) -> Int82R {
        Int82R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt 83"]
    #[inline(always)]
    pub fn int83(&self) -> Int83R {
        Int83R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt 84"]
    #[inline(always)]
    pub fn int84(&self) -> Int84R {
        Int84R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt 85"]
    #[inline(always)]
    pub fn int85(&self) -> Int85R {
        Int85R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt 86"]
    #[inline(always)]
    pub fn int86(&self) -> Int86R {
        Int86R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt 87"]
    #[inline(always)]
    pub fn int87(&self) -> Int87R {
        Int87R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt 88"]
    #[inline(always)]
    pub fn int88(&self) -> Int88R {
        Int88R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt 89"]
    #[inline(always)]
    pub fn int89(&self) -> Int89R {
        Int89R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt 90"]
    #[inline(always)]
    pub fn int90(&self) -> Int90R {
        Int90R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt 91"]
    #[inline(always)]
    pub fn int91(&self) -> Int91R {
        Int91R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt 92"]
    #[inline(always)]
    pub fn int92(&self) -> Int92R {
        Int92R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt 93"]
    #[inline(always)]
    pub fn int93(&self) -> Int93R {
        Int93R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt 94"]
    #[inline(always)]
    pub fn int94(&self) -> Int94R {
        Int94R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt 95"]
    #[inline(always)]
    pub fn int95(&self) -> Int95R {
        Int95R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ISACTIVER2")
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
            .field("int80", &self.int80())
            .field("int81", &self.int81())
            .field("int82", &self.int82())
            .field("int83", &self.int83())
            .field("int84", &self.int84())
            .field("int85", &self.int85())
            .field("int86", &self.int86())
            .field("int87", &self.int87())
            .field("int88", &self.int88())
            .field("int89", &self.int89())
            .field("int90", &self.int90())
            .field("int91", &self.int91())
            .field("int92", &self.int92())
            .field("int93", &self.int93())
            .field("int94", &self.int94())
            .field("int95", &self.int95())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - ARMC Timer"]
    #[inline(always)]
    #[must_use]
    pub fn timer(&mut self) -> TimerW<GicdIsactiver2Spec> {
        TimerW::new(self, 0)
    }
    #[doc = "Bit 1 - Mailbox"]
    #[inline(always)]
    #[must_use]
    pub fn mailbox(&mut self) -> MailboxW<GicdIsactiver2Spec> {
        MailboxW::new(self, 1)
    }
    #[doc = "Bit 2 - Doorbell 0"]
    #[inline(always)]
    #[must_use]
    pub fn doorbell0(&mut self) -> Doorbell0W<GicdIsactiver2Spec> {
        Doorbell0W::new(self, 2)
    }
    #[doc = "Bit 3 - Doorbell 1"]
    #[inline(always)]
    #[must_use]
    pub fn doorbell1(&mut self) -> Doorbell1W<GicdIsactiver2Spec> {
        Doorbell1W::new(self, 3)
    }
    #[doc = "Bit 4 - VPU0 halted"]
    #[inline(always)]
    #[must_use]
    pub fn vpu0_halted(&mut self) -> Vpu0HaltedW<GicdIsactiver2Spec> {
        Vpu0HaltedW::new(self, 4)
    }
    #[doc = "Bit 5 - VPU1 halted"]
    #[inline(always)]
    #[must_use]
    pub fn vpu1_halted(&mut self) -> Vpu1HaltedW<GicdIsactiver2Spec> {
        Vpu1HaltedW::new(self, 5)
    }
    #[doc = "Bit 6 - ARM address error"]
    #[inline(always)]
    #[must_use]
    pub fn arm_address_error(&mut self) -> ArmAddressErrorW<GicdIsactiver2Spec> {
        ArmAddressErrorW::new(self, 6)
    }
    #[doc = "Bit 7 - ARM AXI error"]
    #[inline(always)]
    #[must_use]
    pub fn arm_axi_error(&mut self) -> ArmAxiErrorW<GicdIsactiver2Spec> {
        ArmAxiErrorW::new(self, 7)
    }
    #[doc = "Bit 8 - Software interrupt 0"]
    #[inline(always)]
    #[must_use]
    pub fn swi0(&mut self) -> Swi0W<GicdIsactiver2Spec> {
        Swi0W::new(self, 8)
    }
    #[doc = "Bit 9 - Software interrupt 1"]
    #[inline(always)]
    #[must_use]
    pub fn swi1(&mut self) -> Swi1W<GicdIsactiver2Spec> {
        Swi1W::new(self, 9)
    }
    #[doc = "Bit 10 - Software interrupt 2"]
    #[inline(always)]
    #[must_use]
    pub fn swi2(&mut self) -> Swi2W<GicdIsactiver2Spec> {
        Swi2W::new(self, 10)
    }
    #[doc = "Bit 11 - Software interrupt 3"]
    #[inline(always)]
    #[must_use]
    pub fn swi3(&mut self) -> Swi3W<GicdIsactiver2Spec> {
        Swi3W::new(self, 11)
    }
    #[doc = "Bit 12 - Software interrupt 4"]
    #[inline(always)]
    #[must_use]
    pub fn swi4(&mut self) -> Swi4W<GicdIsactiver2Spec> {
        Swi4W::new(self, 12)
    }
    #[doc = "Bit 13 - Software interrupt 5"]
    #[inline(always)]
    #[must_use]
    pub fn swi5(&mut self) -> Swi5W<GicdIsactiver2Spec> {
        Swi5W::new(self, 13)
    }
    #[doc = "Bit 14 - Software interrupt 6"]
    #[inline(always)]
    #[must_use]
    pub fn swi6(&mut self) -> Swi6W<GicdIsactiver2Spec> {
        Swi6W::new(self, 14)
    }
    #[doc = "Bit 15 - Software interrupt 7"]
    #[inline(always)]
    #[must_use]
    pub fn swi7(&mut self) -> Swi7W<GicdIsactiver2Spec> {
        Swi7W::new(self, 15)
    }
    #[doc = "Bit 16 - Interrupt 80"]
    #[inline(always)]
    #[must_use]
    pub fn int80(&mut self) -> Int80W<GicdIsactiver2Spec> {
        Int80W::new(self, 16)
    }
    #[doc = "Bit 17 - Interrupt 81"]
    #[inline(always)]
    #[must_use]
    pub fn int81(&mut self) -> Int81W<GicdIsactiver2Spec> {
        Int81W::new(self, 17)
    }
    #[doc = "Bit 18 - Interrupt 82"]
    #[inline(always)]
    #[must_use]
    pub fn int82(&mut self) -> Int82W<GicdIsactiver2Spec> {
        Int82W::new(self, 18)
    }
    #[doc = "Bit 19 - Interrupt 83"]
    #[inline(always)]
    #[must_use]
    pub fn int83(&mut self) -> Int83W<GicdIsactiver2Spec> {
        Int83W::new(self, 19)
    }
    #[doc = "Bit 20 - Interrupt 84"]
    #[inline(always)]
    #[must_use]
    pub fn int84(&mut self) -> Int84W<GicdIsactiver2Spec> {
        Int84W::new(self, 20)
    }
    #[doc = "Bit 21 - Interrupt 85"]
    #[inline(always)]
    #[must_use]
    pub fn int85(&mut self) -> Int85W<GicdIsactiver2Spec> {
        Int85W::new(self, 21)
    }
    #[doc = "Bit 22 - Interrupt 86"]
    #[inline(always)]
    #[must_use]
    pub fn int86(&mut self) -> Int86W<GicdIsactiver2Spec> {
        Int86W::new(self, 22)
    }
    #[doc = "Bit 23 - Interrupt 87"]
    #[inline(always)]
    #[must_use]
    pub fn int87(&mut self) -> Int87W<GicdIsactiver2Spec> {
        Int87W::new(self, 23)
    }
    #[doc = "Bit 24 - Interrupt 88"]
    #[inline(always)]
    #[must_use]
    pub fn int88(&mut self) -> Int88W<GicdIsactiver2Spec> {
        Int88W::new(self, 24)
    }
    #[doc = "Bit 25 - Interrupt 89"]
    #[inline(always)]
    #[must_use]
    pub fn int89(&mut self) -> Int89W<GicdIsactiver2Spec> {
        Int89W::new(self, 25)
    }
    #[doc = "Bit 26 - Interrupt 90"]
    #[inline(always)]
    #[must_use]
    pub fn int90(&mut self) -> Int90W<GicdIsactiver2Spec> {
        Int90W::new(self, 26)
    }
    #[doc = "Bit 27 - Interrupt 91"]
    #[inline(always)]
    #[must_use]
    pub fn int91(&mut self) -> Int91W<GicdIsactiver2Spec> {
        Int91W::new(self, 27)
    }
    #[doc = "Bit 28 - Interrupt 92"]
    #[inline(always)]
    #[must_use]
    pub fn int92(&mut self) -> Int92W<GicdIsactiver2Spec> {
        Int92W::new(self, 28)
    }
    #[doc = "Bit 29 - Interrupt 93"]
    #[inline(always)]
    #[must_use]
    pub fn int93(&mut self) -> Int93W<GicdIsactiver2Spec> {
        Int93W::new(self, 29)
    }
    #[doc = "Bit 30 - Interrupt 94"]
    #[inline(always)]
    #[must_use]
    pub fn int94(&mut self) -> Int94W<GicdIsactiver2Spec> {
        Int94W::new(self, 30)
    }
    #[doc = "Bit 31 - Interrupt 95"]
    #[inline(always)]
    #[must_use]
    pub fn int95(&mut self) -> Int95W<GicdIsactiver2Spec> {
        Int95W::new(self, 31)
    }
}
#[doc = "Interrupt Set-Active\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_isactiver2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_isactiver2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIsactiver2Spec;
impl crate::RegisterSpec for GicdIsactiver2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_isactiver2::R`](R) reader structure"]
impl crate::Readable for GicdIsactiver2Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_isactiver2::W`](W) writer structure"]
impl crate::Writable for GicdIsactiver2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff_ffff;
}
#[doc = "`reset()` method sets GICD_ISACTIVER2 to value 0"]
impl crate::Resettable for GicdIsactiver2Spec {
    const RESET_VALUE: u32 = 0;
}

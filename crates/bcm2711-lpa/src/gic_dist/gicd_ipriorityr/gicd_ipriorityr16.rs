#[doc = "Register `GICD_IPRIORITYR16` reader"]
pub type R = crate::R<GicdIpriorityr16Spec>;
#[doc = "Register `GICD_IPRIORITYR16` writer"]
pub type W = crate::W<GicdIpriorityr16Spec>;
#[doc = "Field `TIMER` reader - ARMC Timer"]
pub type TimerR = crate::FieldReader;
#[doc = "Field `TIMER` writer - ARMC Timer"]
pub type TimerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MAILBOX` reader - Mailbox"]
pub type MailboxR = crate::FieldReader;
#[doc = "Field `MAILBOX` writer - Mailbox"]
pub type MailboxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DOORBELL0` reader - Doorbell 0"]
pub type Doorbell0R = crate::FieldReader;
#[doc = "Field `DOORBELL0` writer - Doorbell 0"]
pub type Doorbell0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DOORBELL1` reader - Doorbell 1"]
pub type Doorbell1R = crate::FieldReader;
#[doc = "Field `DOORBELL1` writer - Doorbell 1"]
pub type Doorbell1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - ARMC Timer"]
    #[inline(always)]
    pub fn timer(&self) -> TimerR {
        TimerR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Mailbox"]
    #[inline(always)]
    pub fn mailbox(&self) -> MailboxR {
        MailboxR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Doorbell 0"]
    #[inline(always)]
    pub fn doorbell0(&self) -> Doorbell0R {
        Doorbell0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Doorbell 1"]
    #[inline(always)]
    pub fn doorbell1(&self) -> Doorbell1R {
        Doorbell1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR16")
            .field("timer", &self.timer())
            .field("mailbox", &self.mailbox())
            .field("doorbell0", &self.doorbell0())
            .field("doorbell1", &self.doorbell1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - ARMC Timer"]
    #[inline(always)]
    #[must_use]
    pub fn timer(&mut self) -> TimerW<GicdIpriorityr16Spec> {
        TimerW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Mailbox"]
    #[inline(always)]
    #[must_use]
    pub fn mailbox(&mut self) -> MailboxW<GicdIpriorityr16Spec> {
        MailboxW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Doorbell 0"]
    #[inline(always)]
    #[must_use]
    pub fn doorbell0(&mut self) -> Doorbell0W<GicdIpriorityr16Spec> {
        Doorbell0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Doorbell 1"]
    #[inline(always)]
    #[must_use]
    pub fn doorbell1(&mut self) -> Doorbell1W<GicdIpriorityr16Spec> {
        Doorbell1W::new(self, 24)
    }
}
#[doc = "Interrupt Priority 64 - 67 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIpriorityr16Spec;
impl crate::RegisterSpec for GicdIpriorityr16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr16::R`](R) reader structure"]
impl crate::Readable for GicdIpriorityr16Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr16::W`](W) writer structure"]
impl crate::Writable for GicdIpriorityr16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR16 to value 0"]
impl crate::Resettable for GicdIpriorityr16Spec {
    const RESET_VALUE: u32 = 0;
}

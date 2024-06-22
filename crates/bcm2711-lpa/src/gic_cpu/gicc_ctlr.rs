#[doc = "Register `GICC_CTLR` reader"]
pub type R = crate::R<GiccCtlrSpec>;
#[doc = "Register `GICC_CTLR` writer"]
pub type W = crate::W<GiccCtlrSpec>;
#[doc = "Field `ENABLE_GROUP_0` reader - Enable signaling of group 0"]
pub type EnableGroup0R = crate::BitReader;
#[doc = "Field `ENABLE_GROUP_0` writer - Enable signaling of group 0"]
pub type EnableGroup0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_GROUP_1` reader - Enable signaling of group 1"]
pub type EnableGroup1R = crate::BitReader;
#[doc = "Field `ENABLE_GROUP_1` writer - Enable signaling of group 1"]
pub type EnableGroup1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKCTL` reader - Whether a read of IAR acknowledges the interrupt"]
pub type AckctlR = crate::BitReader;
#[doc = "Field `ACKCTL` writer - Whether a read of IAR acknowledges the interrupt"]
pub type AckctlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIQEN` reader - Group 0 triggers FIQ"]
pub type FiqenR = crate::BitReader;
#[doc = "Field `FIQEN` writer - Group 0 triggers FIQ"]
pub type FiqenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBPR` reader - Common control of interrupts through GICC_BPR"]
pub type CbprR = crate::BitReader;
#[doc = "Field `CBPR` writer - Common control of interrupts through GICC_BPR"]
pub type CbprW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIQBYPDISGRP0` reader - Bypass FIQ is not signaled to processor"]
pub type Fiqbypdisgrp0R = crate::BitReader;
#[doc = "Field `FIQBYPDISGRP0` writer - Bypass FIQ is not signaled to processor"]
pub type Fiqbypdisgrp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQBYPDISGRP0` reader - Bypass IRQ is not signaled to processor"]
pub type Irqbypdisgrp0R = crate::BitReader;
#[doc = "Field `IRQBYPDISGRP0` writer - Bypass IRQ is not signaled to processor"]
pub type Irqbypdisgrp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIQBYPDISGRP1` reader - Alias of group 1 FIQ bypass disable"]
pub type Fiqbypdisgrp1R = crate::BitReader;
#[doc = "Field `FIQBYPDISGRP1` writer - Alias of group 1 FIQ bypass disable"]
pub type Fiqbypdisgrp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQBYPDISGRP1` reader - Alias of group 1 IRQ bypass disable"]
pub type Irqbypdisgrp1R = crate::BitReader;
#[doc = "Field `IRQBYPDISGRP1` writer - Alias of group 1 IRQ bypass disable"]
pub type Irqbypdisgrp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOIMODES` reader - Secure EOIR does priority drop. DIR does deactivate."]
pub type EoimodesR = crate::BitReader;
#[doc = "Field `EOIMODES` writer - Secure EOIR does priority drop. DIR does deactivate."]
pub type EoimodesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOIMODENS` reader - Non-Secure EOIR does priority drop. DIR does deactivate."]
pub type EoimodensR = crate::BitReader;
#[doc = "Field `EOIMODENS` writer - Non-Secure EOIR does priority drop. DIR does deactivate."]
pub type EoimodensW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable signaling of group 0"]
    #[inline(always)]
    pub fn enable_group_0(&self) -> EnableGroup0R {
        EnableGroup0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable signaling of group 1"]
    #[inline(always)]
    pub fn enable_group_1(&self) -> EnableGroup1R {
        EnableGroup1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Whether a read of IAR acknowledges the interrupt"]
    #[inline(always)]
    pub fn ackctl(&self) -> AckctlR {
        AckctlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Group 0 triggers FIQ"]
    #[inline(always)]
    pub fn fiqen(&self) -> FiqenR {
        FiqenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Common control of interrupts through GICC_BPR"]
    #[inline(always)]
    pub fn cbpr(&self) -> CbprR {
        CbprR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bypass FIQ is not signaled to processor"]
    #[inline(always)]
    pub fn fiqbypdisgrp0(&self) -> Fiqbypdisgrp0R {
        Fiqbypdisgrp0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bypass IRQ is not signaled to processor"]
    #[inline(always)]
    pub fn irqbypdisgrp0(&self) -> Irqbypdisgrp0R {
        Irqbypdisgrp0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Alias of group 1 FIQ bypass disable"]
    #[inline(always)]
    pub fn fiqbypdisgrp1(&self) -> Fiqbypdisgrp1R {
        Fiqbypdisgrp1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Alias of group 1 IRQ bypass disable"]
    #[inline(always)]
    pub fn irqbypdisgrp1(&self) -> Irqbypdisgrp1R {
        Irqbypdisgrp1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Secure EOIR does priority drop. DIR does deactivate."]
    #[inline(always)]
    pub fn eoimodes(&self) -> EoimodesR {
        EoimodesR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Non-Secure EOIR does priority drop. DIR does deactivate."]
    #[inline(always)]
    pub fn eoimodens(&self) -> EoimodensR {
        EoimodensR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICC_CTLR")
            .field("enable_group_0", &self.enable_group_0())
            .field("enable_group_1", &self.enable_group_1())
            .field("ackctl", &self.ackctl())
            .field("fiqen", &self.fiqen())
            .field("cbpr", &self.cbpr())
            .field("fiqbypdisgrp0", &self.fiqbypdisgrp0())
            .field("irqbypdisgrp0", &self.irqbypdisgrp0())
            .field("fiqbypdisgrp1", &self.fiqbypdisgrp1())
            .field("irqbypdisgrp1", &self.irqbypdisgrp1())
            .field("eoimodes", &self.eoimodes())
            .field("eoimodens", &self.eoimodens())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enable signaling of group 0"]
    #[inline(always)]
    #[must_use]
    pub fn enable_group_0(&mut self) -> EnableGroup0W<GiccCtlrSpec> {
        EnableGroup0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable signaling of group 1"]
    #[inline(always)]
    #[must_use]
    pub fn enable_group_1(&mut self) -> EnableGroup1W<GiccCtlrSpec> {
        EnableGroup1W::new(self, 1)
    }
    #[doc = "Bit 2 - Whether a read of IAR acknowledges the interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ackctl(&mut self) -> AckctlW<GiccCtlrSpec> {
        AckctlW::new(self, 2)
    }
    #[doc = "Bit 3 - Group 0 triggers FIQ"]
    #[inline(always)]
    #[must_use]
    pub fn fiqen(&mut self) -> FiqenW<GiccCtlrSpec> {
        FiqenW::new(self, 3)
    }
    #[doc = "Bit 4 - Common control of interrupts through GICC_BPR"]
    #[inline(always)]
    #[must_use]
    pub fn cbpr(&mut self) -> CbprW<GiccCtlrSpec> {
        CbprW::new(self, 4)
    }
    #[doc = "Bit 5 - Bypass FIQ is not signaled to processor"]
    #[inline(always)]
    #[must_use]
    pub fn fiqbypdisgrp0(&mut self) -> Fiqbypdisgrp0W<GiccCtlrSpec> {
        Fiqbypdisgrp0W::new(self, 5)
    }
    #[doc = "Bit 6 - Bypass IRQ is not signaled to processor"]
    #[inline(always)]
    #[must_use]
    pub fn irqbypdisgrp0(&mut self) -> Irqbypdisgrp0W<GiccCtlrSpec> {
        Irqbypdisgrp0W::new(self, 6)
    }
    #[doc = "Bit 7 - Alias of group 1 FIQ bypass disable"]
    #[inline(always)]
    #[must_use]
    pub fn fiqbypdisgrp1(&mut self) -> Fiqbypdisgrp1W<GiccCtlrSpec> {
        Fiqbypdisgrp1W::new(self, 7)
    }
    #[doc = "Bit 8 - Alias of group 1 IRQ bypass disable"]
    #[inline(always)]
    #[must_use]
    pub fn irqbypdisgrp1(&mut self) -> Irqbypdisgrp1W<GiccCtlrSpec> {
        Irqbypdisgrp1W::new(self, 8)
    }
    #[doc = "Bit 9 - Secure EOIR does priority drop. DIR does deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn eoimodes(&mut self) -> EoimodesW<GiccCtlrSpec> {
        EoimodesW::new(self, 9)
    }
    #[doc = "Bit 10 - Non-Secure EOIR does priority drop. DIR does deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn eoimodens(&mut self) -> EoimodensW<GiccCtlrSpec> {
        EoimodensW::new(self, 10)
    }
}
#[doc = "CPU Interface Control\n\nYou can [`read`](crate::Reg::read) this register and get [`gicc_ctlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicc_ctlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiccCtlrSpec;
impl crate::RegisterSpec for GiccCtlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicc_ctlr::R`](R) reader structure"]
impl crate::Readable for GiccCtlrSpec {}
#[doc = "`write(|w| ..)` method takes [`gicc_ctlr::W`](W) writer structure"]
impl crate::Writable for GiccCtlrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICC_CTLR to value 0"]
impl crate::Resettable for GiccCtlrSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `PCGCCTL` reader"]
pub type R = crate::R<PcgcctlSpec>;
#[doc = "Register `PCGCCTL` writer"]
pub type W = crate::W<PcgcctlSpec>;
#[doc = "Field `STPPCLK` reader - Stop PHY clock"]
pub type StppclkR = crate::BitReader;
#[doc = "Field `STPPCLK` writer - Stop PHY clock"]
pub type StppclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GATEHCLK` reader - Gate HCLK"]
pub type GatehclkR = crate::BitReader;
#[doc = "Field `GATEHCLK` writer - Gate HCLK"]
pub type GatehclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRCLMP` reader - Power clamp"]
pub type PwrclmpR = crate::BitReader;
#[doc = "Field `PWRCLMP` writer - Power clamp"]
pub type PwrclmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTPDWNMODULE` reader - Power down modules"]
pub type RstpdwnmoduleR = crate::BitReader;
#[doc = "Field `RSTPDWNMODULE` writer - Power down modules"]
pub type RstpdwnmoduleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHYSUSP` reader - PHY Suspended"]
pub type PhysuspR = crate::BitReader;
#[doc = "Field `PHYSUSP` writer - PHY Suspended"]
pub type PhysuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_L1GATING` reader - Enable sleep clock gating"]
pub type EnableL1gatingR = crate::BitReader;
#[doc = "Field `ENABLE_L1GATING` writer - Enable sleep clock gating"]
pub type EnableL1gatingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHYSLEEP` reader - PHY is in sleep mode"]
pub type PhysleepR = crate::BitReader;
#[doc = "Field `PHYSLEEP` writer - PHY is in sleep mode"]
pub type PhysleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEEPSLEEP` reader - PHY is in deep sleep"]
pub type DeepsleepR = crate::BitReader;
#[doc = "Field `DEEPSLEEP` writer - PHY is in deep sleep"]
pub type DeepsleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETAFTERSUSP` reader - Reset after suspend"]
pub type ResetaftersuspR = crate::BitReader;
#[doc = "Field `RESETAFTERSUSP` writer - Reset after suspend"]
pub type ResetaftersuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESTOREMODE` reader - Restore mode"]
pub type RestoremodeR = crate::BitReader;
#[doc = "Field `RESTOREMODE` writer - Restore mode"]
pub type RestoremodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENEXTNDEDHIBER` reader - Enable extended hibernation"]
pub type EnextndedhiberR = crate::BitReader;
#[doc = "Field `ENEXTNDEDHIBER` writer - Enable extended hibernation"]
pub type EnextndedhiberW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTNDEDHIBERNATIONCLAMP` reader - Extended hibernation clamp"]
pub type ExtndedhibernationclampR = crate::BitReader;
#[doc = "Field `EXTNDEDHIBERNATIONCLAMP` writer - Extended hibernation clamp"]
pub type ExtndedhibernationclampW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTNDEDHIBERNATIONSWITCH` reader - Extended hibernation switch"]
pub type ExtndedhibernationswitchR = crate::BitReader;
#[doc = "Field `EXTNDEDHIBERNATIONSWITCH` writer - Extended hibernation switch"]
pub type ExtndedhibernationswitchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESSREGRESTORED` reader - Essential register values restored"]
pub type EssregrestoredR = crate::BitReader;
#[doc = "Field `ESSREGRESTORED` writer - Essential register values restored"]
pub type EssregrestoredW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESTORE_VALUE` reader - Restore value"]
pub type RestoreValueR = crate::FieldReader<u32>;
#[doc = "Field `RESTORE_VALUE` writer - Restore value"]
pub type RestoreValueW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bit 0 - Stop PHY clock"]
    #[inline(always)]
    pub fn stppclk(&self) -> StppclkR {
        StppclkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Gate HCLK"]
    #[inline(always)]
    pub fn gatehclk(&self) -> GatehclkR {
        GatehclkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power clamp"]
    #[inline(always)]
    pub fn pwrclmp(&self) -> PwrclmpR {
        PwrclmpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Power down modules"]
    #[inline(always)]
    pub fn rstpdwnmodule(&self) -> RstpdwnmoduleR {
        RstpdwnmoduleR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PHY Suspended"]
    #[inline(always)]
    pub fn physusp(&self) -> PhysuspR {
        PhysuspR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable sleep clock gating"]
    #[inline(always)]
    pub fn enable_l1gating(&self) -> EnableL1gatingR {
        EnableL1gatingR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PHY is in sleep mode"]
    #[inline(always)]
    pub fn physleep(&self) -> PhysleepR {
        PhysleepR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PHY is in deep sleep"]
    #[inline(always)]
    pub fn deepsleep(&self) -> DeepsleepR {
        DeepsleepR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reset after suspend"]
    #[inline(always)]
    pub fn resetaftersusp(&self) -> ResetaftersuspR {
        ResetaftersuspR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Restore mode"]
    #[inline(always)]
    pub fn restoremode(&self) -> RestoremodeR {
        RestoremodeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable extended hibernation"]
    #[inline(always)]
    pub fn enextndedhiber(&self) -> EnextndedhiberR {
        EnextndedhiberR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Extended hibernation clamp"]
    #[inline(always)]
    pub fn extndedhibernationclamp(&self) -> ExtndedhibernationclampR {
        ExtndedhibernationclampR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Extended hibernation switch"]
    #[inline(always)]
    pub fn extndedhibernationswitch(&self) -> ExtndedhibernationswitchR {
        ExtndedhibernationswitchR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Essential register values restored"]
    #[inline(always)]
    pub fn essregrestored(&self) -> EssregrestoredR {
        EssregrestoredR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:31 - Restore value"]
    #[inline(always)]
    pub fn restore_value(&self) -> RestoreValueR {
        RestoreValueR::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCGCCTL")
            .field("stppclk", &self.stppclk())
            .field("gatehclk", &self.gatehclk())
            .field("pwrclmp", &self.pwrclmp())
            .field("rstpdwnmodule", &self.rstpdwnmodule())
            .field("physusp", &self.physusp())
            .field("enable_l1gating", &self.enable_l1gating())
            .field("physleep", &self.physleep())
            .field("deepsleep", &self.deepsleep())
            .field("resetaftersusp", &self.resetaftersusp())
            .field("restoremode", &self.restoremode())
            .field("enextndedhiber", &self.enextndedhiber())
            .field("extndedhibernationclamp", &self.extndedhibernationclamp())
            .field("extndedhibernationswitch", &self.extndedhibernationswitch())
            .field("essregrestored", &self.essregrestored())
            .field("restore_value", &self.restore_value())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Stop PHY clock"]
    #[inline(always)]
    #[must_use]
    pub fn stppclk(&mut self) -> StppclkW<PcgcctlSpec> {
        StppclkW::new(self, 0)
    }
    #[doc = "Bit 1 - Gate HCLK"]
    #[inline(always)]
    #[must_use]
    pub fn gatehclk(&mut self) -> GatehclkW<PcgcctlSpec> {
        GatehclkW::new(self, 1)
    }
    #[doc = "Bit 2 - Power clamp"]
    #[inline(always)]
    #[must_use]
    pub fn pwrclmp(&mut self) -> PwrclmpW<PcgcctlSpec> {
        PwrclmpW::new(self, 2)
    }
    #[doc = "Bit 3 - Power down modules"]
    #[inline(always)]
    #[must_use]
    pub fn rstpdwnmodule(&mut self) -> RstpdwnmoduleW<PcgcctlSpec> {
        RstpdwnmoduleW::new(self, 3)
    }
    #[doc = "Bit 4 - PHY Suspended"]
    #[inline(always)]
    #[must_use]
    pub fn physusp(&mut self) -> PhysuspW<PcgcctlSpec> {
        PhysuspW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable sleep clock gating"]
    #[inline(always)]
    #[must_use]
    pub fn enable_l1gating(&mut self) -> EnableL1gatingW<PcgcctlSpec> {
        EnableL1gatingW::new(self, 5)
    }
    #[doc = "Bit 6 - PHY is in sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn physleep(&mut self) -> PhysleepW<PcgcctlSpec> {
        PhysleepW::new(self, 6)
    }
    #[doc = "Bit 7 - PHY is in deep sleep"]
    #[inline(always)]
    #[must_use]
    pub fn deepsleep(&mut self) -> DeepsleepW<PcgcctlSpec> {
        DeepsleepW::new(self, 7)
    }
    #[doc = "Bit 8 - Reset after suspend"]
    #[inline(always)]
    #[must_use]
    pub fn resetaftersusp(&mut self) -> ResetaftersuspW<PcgcctlSpec> {
        ResetaftersuspW::new(self, 8)
    }
    #[doc = "Bit 9 - Restore mode"]
    #[inline(always)]
    #[must_use]
    pub fn restoremode(&mut self) -> RestoremodeW<PcgcctlSpec> {
        RestoremodeW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable extended hibernation"]
    #[inline(always)]
    #[must_use]
    pub fn enextndedhiber(&mut self) -> EnextndedhiberW<PcgcctlSpec> {
        EnextndedhiberW::new(self, 10)
    }
    #[doc = "Bit 11 - Extended hibernation clamp"]
    #[inline(always)]
    #[must_use]
    pub fn extndedhibernationclamp(&mut self) -> ExtndedhibernationclampW<PcgcctlSpec> {
        ExtndedhibernationclampW::new(self, 11)
    }
    #[doc = "Bit 12 - Extended hibernation switch"]
    #[inline(always)]
    #[must_use]
    pub fn extndedhibernationswitch(&mut self) -> ExtndedhibernationswitchW<PcgcctlSpec> {
        ExtndedhibernationswitchW::new(self, 12)
    }
    #[doc = "Bit 13 - Essential register values restored"]
    #[inline(always)]
    #[must_use]
    pub fn essregrestored(&mut self) -> EssregrestoredW<PcgcctlSpec> {
        EssregrestoredW::new(self, 13)
    }
    #[doc = "Bits 14:31 - Restore value"]
    #[inline(always)]
    #[must_use]
    pub fn restore_value(&mut self) -> RestoreValueW<PcgcctlSpec> {
        RestoreValueW::new(self, 14)
    }
}
#[doc = "power and clock gating control\n\nYou can [`read`](crate::Reg::read) this register and get [`pcgcctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcgcctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcgcctlSpec;
impl crate::RegisterSpec for PcgcctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcgcctl::R`](R) reader structure"]
impl crate::Readable for PcgcctlSpec {}
#[doc = "`write(|w| ..)` method takes [`pcgcctl::W`](W) writer structure"]
impl crate::Writable for PcgcctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCGCCTL to value 0x200b_8000"]
impl crate::Resettable for PcgcctlSpec {
    const RESET_VALUE: u32 = 0x200b_8000;
}

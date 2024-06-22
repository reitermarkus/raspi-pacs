#[doc = "Register `8_CS` reader"]
pub type R = crate::R<_8CsSpec>;
#[doc = "Register `8_CS` writer"]
pub type W = crate::W<_8CsSpec>;
#[doc = "Field `ACTIVE` reader - Activate the DMA"]
pub type ActiveR = crate::BitReader;
#[doc = "Field `ACTIVE` writer - Activate the DMA"]
pub type ActiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `END` reader - DMA End Flag"]
pub type EndR = crate::BitReader;
#[doc = "Field `END` writer - DMA End Flag"]
pub type EndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT` reader - Interrupt Status"]
pub type IntR = crate::BitReader;
#[doc = "Field `INT` writer - Interrupt Status"]
pub type IntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DREQ` reader - DREQ State"]
pub type DreqR = crate::BitReader;
#[doc = "Field `PAUSED` reader - DMA Paused State"]
pub type PausedR = crate::BitReader;
#[doc = "Field `DREQ_STOPS_DMA` reader - DMA Paused by DREQ State"]
pub type DreqStopsDmaR = crate::BitReader;
#[doc = "Field `WAITING_FOR_OUTSTANDING_WRITES` reader - DMA is Waiting for the Last Write to be Received"]
pub type WaitingForOutstandingWritesR = crate::BitReader;
#[doc = "Field `ERROR` reader - DMA Error"]
pub type ErrorR = crate::BitReader;
#[doc = "Field `PRIORITY` reader - AXI Priority Level"]
pub type PriorityR = crate::FieldReader;
#[doc = "Field `PRIORITY` writer - AXI Priority Level"]
pub type PriorityW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PANIC_PRIORITY` reader - AXI Panic Priority Level"]
pub type PanicPriorityR = crate::FieldReader;
#[doc = "Field `PANIC_PRIORITY` writer - AXI Panic Priority Level"]
pub type PanicPriorityW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WAIT_FOR_OUTSTANDING_WRITES` reader - Wait for outstanding writes"]
pub type WaitForOutstandingWritesR = crate::BitReader;
#[doc = "Field `WAIT_FOR_OUTSTANDING_WRITES` writer - Wait for outstanding writes"]
pub type WaitForOutstandingWritesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISDEBUG` reader - Disable debug pause signal"]
pub type DisdebugR = crate::BitReader;
#[doc = "Field `DISDEBUG` writer - Disable debug pause signal"]
pub type DisdebugW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABORT` writer - Abort DMA"]
pub type AbortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` writer - DMA Channel Reset"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Activate the DMA"]
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA End Flag"]
    #[inline(always)]
    pub fn end(&self) -> EndR {
        EndR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Status"]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DREQ State"]
    #[inline(always)]
    pub fn dreq(&self) -> DreqR {
        DreqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA Paused State"]
    #[inline(always)]
    pub fn paused(&self) -> PausedR {
        PausedR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA Paused by DREQ State"]
    #[inline(always)]
    pub fn dreq_stops_dma(&self) -> DreqStopsDmaR {
        DreqStopsDmaR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA is Waiting for the Last Write to be Received"]
    #[inline(always)]
    pub fn waiting_for_outstanding_writes(&self) -> WaitingForOutstandingWritesR {
        WaitingForOutstandingWritesR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA Error"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:19 - AXI Priority Level"]
    #[inline(always)]
    pub fn priority(&self) -> PriorityR {
        PriorityR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - AXI Panic Priority Level"]
    #[inline(always)]
    pub fn panic_priority(&self) -> PanicPriorityR {
        PanicPriorityR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Wait for outstanding writes"]
    #[inline(always)]
    pub fn wait_for_outstanding_writes(&self) -> WaitForOutstandingWritesR {
        WaitForOutstandingWritesR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Disable debug pause signal"]
    #[inline(always)]
    pub fn disdebug(&self) -> DisdebugR {
        DisdebugR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("8_CS")
            .field("disdebug", &self.disdebug())
            .field(
                "wait_for_outstanding_writes",
                &self.wait_for_outstanding_writes(),
            )
            .field("panic_priority", &self.panic_priority())
            .field("priority", &self.priority())
            .field("error", &self.error())
            .field(
                "waiting_for_outstanding_writes",
                &self.waiting_for_outstanding_writes(),
            )
            .field("dreq_stops_dma", &self.dreq_stops_dma())
            .field("paused", &self.paused())
            .field("dreq", &self.dreq())
            .field("int", &self.int())
            .field("end", &self.end())
            .field("active", &self.active())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Activate the DMA"]
    #[inline(always)]
    #[must_use]
    pub fn active(&mut self) -> ActiveW<_8CsSpec> {
        ActiveW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA End Flag"]
    #[inline(always)]
    #[must_use]
    pub fn end(&mut self) -> EndW<_8CsSpec> {
        EndW::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> IntW<_8CsSpec> {
        IntW::new(self, 2)
    }
    #[doc = "Bits 16:19 - AXI Priority Level"]
    #[inline(always)]
    #[must_use]
    pub fn priority(&mut self) -> PriorityW<_8CsSpec> {
        PriorityW::new(self, 16)
    }
    #[doc = "Bits 24:27 - AXI Panic Priority Level"]
    #[inline(always)]
    #[must_use]
    pub fn panic_priority(&mut self) -> PanicPriorityW<_8CsSpec> {
        PanicPriorityW::new(self, 24)
    }
    #[doc = "Bit 28 - Wait for outstanding writes"]
    #[inline(always)]
    #[must_use]
    pub fn wait_for_outstanding_writes(&mut self) -> WaitForOutstandingWritesW<_8CsSpec> {
        WaitForOutstandingWritesW::new(self, 28)
    }
    #[doc = "Bit 29 - Disable debug pause signal"]
    #[inline(always)]
    #[must_use]
    pub fn disdebug(&mut self) -> DisdebugW<_8CsSpec> {
        DisdebugW::new(self, 29)
    }
    #[doc = "Bit 30 - Abort DMA"]
    #[inline(always)]
    #[must_use]
    pub fn abort(&mut self) -> AbortW<_8CsSpec> {
        AbortW::new(self, 30)
    }
    #[doc = "Bit 31 - DMA Channel Reset"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> ResetW<_8CsSpec> {
        ResetW::new(self, 31)
    }
}
#[doc = "DMA Channel 8 Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`_8_cs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_8_cs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _8CsSpec;
impl crate::RegisterSpec for _8CsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_8_cs::R`](R) reader structure"]
impl crate::Readable for _8CsSpec {}
#[doc = "`write(|w| ..)` method takes [`_8_cs::W`](W) writer structure"]
impl crate::Writable for _8CsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets 8_CS to value 0"]
impl crate::Resettable for _8CsSpec {
    const RESET_VALUE: u32 = 0;
}

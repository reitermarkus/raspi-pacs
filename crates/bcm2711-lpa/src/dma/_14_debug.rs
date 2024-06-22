#[doc = "Register `14_DEBUG` reader"]
pub type R = crate::R<_14DebugSpec>;
#[doc = "Register `14_DEBUG` writer"]
pub type W = crate::W<_14DebugSpec>;
#[doc = "Field `READ_LAST_NOT_SET_ERROR` reader - Read Last Not Set Error"]
pub type ReadLastNotSetErrorR = crate::BitReader;
#[doc = "Field `READ_LAST_NOT_SET_ERROR` writer - Read Last Not Set Error"]
pub type ReadLastNotSetErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_ERROR` reader - Fifo Error"]
pub type FifoErrorR = crate::BitReader;
#[doc = "Field `FIFO_ERROR` writer - Fifo Error"]
pub type FifoErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_ERROR` reader - Slave Read Response Error"]
pub type ReadErrorR = crate::BitReader;
#[doc = "Field `READ_ERROR` writer - Slave Read Response Error"]
pub type ReadErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTSTANDING_WRITES` reader - DMA Outstanding Writes Counter"]
pub type OutstandingWritesR = crate::FieldReader;
#[doc = "Field `DMA_ID` reader - DMA ID"]
pub type DmaIdR = crate::FieldReader;
#[doc = "Field `DMA_STATE` reader - DMA State Machine State"]
pub type DmaStateR = crate::FieldReader<u16>;
#[doc = "Field `VERSION` reader - DMA Version"]
pub type VersionR = crate::FieldReader;
#[doc = "Field `LITE` reader - DMA Lite"]
pub type LiteR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read Last Not Set Error"]
    #[inline(always)]
    pub fn read_last_not_set_error(&self) -> ReadLastNotSetErrorR {
        ReadLastNotSetErrorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fifo Error"]
    #[inline(always)]
    pub fn fifo_error(&self) -> FifoErrorR {
        FifoErrorR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slave Read Response Error"]
    #[inline(always)]
    pub fn read_error(&self) -> ReadErrorR {
        ReadErrorR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7 - DMA Outstanding Writes Counter"]
    #[inline(always)]
    pub fn outstanding_writes(&self) -> OutstandingWritesR {
        OutstandingWritesR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - DMA ID"]
    #[inline(always)]
    pub fn dma_id(&self) -> DmaIdR {
        DmaIdR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:24 - DMA State Machine State"]
    #[inline(always)]
    pub fn dma_state(&self) -> DmaStateR {
        DmaStateR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:27 - DMA Version"]
    #[inline(always)]
    pub fn version(&self) -> VersionR {
        VersionR::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - DMA Lite"]
    #[inline(always)]
    pub fn lite(&self) -> LiteR {
        LiteR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("14_DEBUG")
            .field("lite", &self.lite())
            .field("version", &self.version())
            .field("dma_state", &self.dma_state())
            .field("dma_id", &self.dma_id())
            .field("outstanding_writes", &self.outstanding_writes())
            .field("read_error", &self.read_error())
            .field("fifo_error", &self.fifo_error())
            .field("read_last_not_set_error", &self.read_last_not_set_error())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Read Last Not Set Error"]
    #[inline(always)]
    #[must_use]
    pub fn read_last_not_set_error(&mut self) -> ReadLastNotSetErrorW<_14DebugSpec> {
        ReadLastNotSetErrorW::new(self, 0)
    }
    #[doc = "Bit 1 - Fifo Error"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_error(&mut self) -> FifoErrorW<_14DebugSpec> {
        FifoErrorW::new(self, 1)
    }
    #[doc = "Bit 2 - Slave Read Response Error"]
    #[inline(always)]
    #[must_use]
    pub fn read_error(&mut self) -> ReadErrorW<_14DebugSpec> {
        ReadErrorW::new(self, 2)
    }
}
#[doc = "DMA Channel 14 Debug\n\nYou can [`read`](crate::Reg::read) this register and get [`_14_debug::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_14_debug::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _14DebugSpec;
impl crate::RegisterSpec for _14DebugSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_14_debug::R`](R) reader structure"]
impl crate::Readable for _14DebugSpec {}
#[doc = "`write(|w| ..)` method takes [`_14_debug::W`](W) writer structure"]
impl crate::Writable for _14DebugSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets 14_DEBUG to value 0"]
impl crate::Resettable for _14DebugSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "Field `DATA_READY` reader - Receive FIFO has at least one symbol"]
pub type DataReadyR = crate::BitReader;
#[doc = "Field `DATA_READY` writer - Receive FIFO has at least one symbol"]
pub type DataReadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_READY` reader - Transmit FIFO has space for at least one symbol"]
pub type TxReadyR = crate::BitReader;
#[doc = "Field `TX_READY` writer - Transmit FIFO has space for at least one symbol"]
pub type TxReadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_IDLE` reader - Receiver is idle"]
pub type RxIdleR = crate::BitReader;
#[doc = "Field `RX_IDLE` writer - Receiver is idle"]
pub type RxIdleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_IDLE` reader - Transmitter is idle"]
pub type TxIdleR = crate::BitReader;
#[doc = "Field `TX_IDLE` writer - Transmitter is idle"]
pub type TxIdleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_OVERRUN` reader - Receive FIFO overrun"]
pub type RxOverrunR = crate::BitReader;
#[doc = "Field `RX_OVERRUN` writer - Receive FIFO overrun"]
pub type RxOverrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FULL` reader - Transmit FIFO is full"]
pub type TxFullR = crate::BitReader;
#[doc = "Field `TX_FULL` writer - Transmit FIFO is full"]
pub type TxFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTS_STATUS` reader - RTS state"]
pub type RtsStatusR = crate::BitReader;
#[doc = "Field `RTS_STATUS` writer - RTS state"]
pub type RtsStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTS_STATUS` reader - CTS state"]
pub type CtsStatusR = crate::BitReader;
#[doc = "Field `CTS_STATUS` writer - CTS state"]
pub type CtsStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_EMPTY` reader - Transmit FIFO is completely empty"]
pub type TxEmptyR = crate::BitReader;
#[doc = "Field `TX_EMPTY` writer - Transmit FIFO is completely empty"]
pub type TxEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DONE` reader - Transmit FIFO is empty and transmitter is idle"]
pub type TxDoneR = crate::BitReader;
#[doc = "Field `TX_DONE` writer - Transmit FIFO is empty and transmitter is idle"]
pub type TxDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_LEVEL` reader - How many entries are filled in the RX FIFO"]
pub type RxFifoLevelR = crate::FieldReader;
#[doc = "Field `RX_FIFO_LEVEL` writer - How many entries are filled in the RX FIFO"]
pub type RxFifoLevelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TX_FIFO_LEVEL` reader - How many entries are filled in the TX FIFO"]
pub type TxFifoLevelR = crate::FieldReader;
#[doc = "Field `TX_FIFO_LEVEL` writer - How many entries are filled in the TX FIFO"]
pub type TxFifoLevelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Receive FIFO has at least one symbol"]
    #[inline(always)]
    pub fn data_ready(&self) -> DataReadyR {
        DataReadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO has space for at least one symbol"]
    #[inline(always)]
    pub fn tx_ready(&self) -> TxReadyR {
        TxReadyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver is idle"]
    #[inline(always)]
    pub fn rx_idle(&self) -> RxIdleR {
        RxIdleR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter is idle"]
    #[inline(always)]
    pub fn tx_idle(&self) -> TxIdleR {
        TxIdleR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO overrun"]
    #[inline(always)]
    pub fn rx_overrun(&self) -> RxOverrunR {
        RxOverrunR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit FIFO is full"]
    #[inline(always)]
    pub fn tx_full(&self) -> TxFullR {
        TxFullR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RTS state"]
    #[inline(always)]
    pub fn rts_status(&self) -> RtsStatusR {
        RtsStatusR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CTS state"]
    #[inline(always)]
    pub fn cts_status(&self) -> CtsStatusR {
        CtsStatusR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit FIFO is completely empty"]
    #[inline(always)]
    pub fn tx_empty(&self) -> TxEmptyR {
        TxEmptyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmit FIFO is empty and transmitter is idle"]
    #[inline(always)]
    pub fn tx_done(&self) -> TxDoneR {
        TxDoneR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:19 - How many entries are filled in the RX FIFO"]
    #[inline(always)]
    pub fn rx_fifo_level(&self) -> RxFifoLevelR {
        RxFifoLevelR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - How many entries are filled in the TX FIFO"]
    #[inline(always)]
    pub fn tx_fifo_level(&self) -> TxFifoLevelR {
        TxFifoLevelR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STAT")
            .field("tx_fifo_level", &self.tx_fifo_level())
            .field("rx_fifo_level", &self.rx_fifo_level())
            .field("tx_done", &self.tx_done())
            .field("tx_empty", &self.tx_empty())
            .field("cts_status", &self.cts_status())
            .field("rts_status", &self.rts_status())
            .field("tx_full", &self.tx_full())
            .field("rx_overrun", &self.rx_overrun())
            .field("tx_idle", &self.tx_idle())
            .field("rx_idle", &self.rx_idle())
            .field("tx_ready", &self.tx_ready())
            .field("data_ready", &self.data_ready())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Receive FIFO has at least one symbol"]
    #[inline(always)]
    #[must_use]
    pub fn data_ready(&mut self) -> DataReadyW<StatSpec> {
        DataReadyW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit FIFO has space for at least one symbol"]
    #[inline(always)]
    #[must_use]
    pub fn tx_ready(&mut self) -> TxReadyW<StatSpec> {
        TxReadyW::new(self, 1)
    }
    #[doc = "Bit 2 - Receiver is idle"]
    #[inline(always)]
    #[must_use]
    pub fn rx_idle(&mut self) -> RxIdleW<StatSpec> {
        RxIdleW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter is idle"]
    #[inline(always)]
    #[must_use]
    pub fn tx_idle(&mut self) -> TxIdleW<StatSpec> {
        TxIdleW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive FIFO overrun"]
    #[inline(always)]
    #[must_use]
    pub fn rx_overrun(&mut self) -> RxOverrunW<StatSpec> {
        RxOverrunW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit FIFO is full"]
    #[inline(always)]
    #[must_use]
    pub fn tx_full(&mut self) -> TxFullW<StatSpec> {
        TxFullW::new(self, 5)
    }
    #[doc = "Bit 6 - RTS state"]
    #[inline(always)]
    #[must_use]
    pub fn rts_status(&mut self) -> RtsStatusW<StatSpec> {
        RtsStatusW::new(self, 6)
    }
    #[doc = "Bit 7 - CTS state"]
    #[inline(always)]
    #[must_use]
    pub fn cts_status(&mut self) -> CtsStatusW<StatSpec> {
        CtsStatusW::new(self, 7)
    }
    #[doc = "Bit 8 - Transmit FIFO is completely empty"]
    #[inline(always)]
    #[must_use]
    pub fn tx_empty(&mut self) -> TxEmptyW<StatSpec> {
        TxEmptyW::new(self, 8)
    }
    #[doc = "Bit 9 - Transmit FIFO is empty and transmitter is idle"]
    #[inline(always)]
    #[must_use]
    pub fn tx_done(&mut self) -> TxDoneW<StatSpec> {
        TxDoneW::new(self, 9)
    }
    #[doc = "Bits 16:19 - How many entries are filled in the RX FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_level(&mut self) -> RxFifoLevelW<StatSpec> {
        RxFifoLevelW::new(self, 16)
    }
    #[doc = "Bits 24:27 - How many entries are filled in the TX FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_level(&mut self) -> TxFifoLevelW<StatSpec> {
        TxFifoLevelW::new(self, 24)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for StatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0;
}

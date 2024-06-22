#[doc = "Register `LSR` reader"]
pub type R = crate::R<LsrSpec>;
#[doc = "Register `LSR` writer"]
pub type W = crate::W<LsrSpec>;
#[doc = "Field `DATA_READY` reader - Receive FIFO has at least one byte"]
pub type DataReadyR = crate::BitReader;
#[doc = "Field `DATA_READY` writer - Receive FIFO has at least one byte"]
pub type DataReadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_OVERRUN` reader - Receive FIFO overrun"]
pub type RxOverrunR = crate::BitReader;
#[doc = "Field `RX_OVERRUN` writer - Receive FIFO overrun"]
pub type RxOverrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_EMPTY` reader - Transmit FIFO has room for at least one byte"]
pub type TxEmptyR = crate::BitReader;
#[doc = "Field `TX_EMPTY` writer - Transmit FIFO has room for at least one byte"]
pub type TxEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_IDLE` reader - Transmit FIFO empty and all bits shifted out"]
pub type TxIdleR = crate::BitReader;
#[doc = "Field `TX_IDLE` writer - Transmit FIFO empty and all bits shifted out"]
pub type TxIdleW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receive FIFO has at least one byte"]
    #[inline(always)]
    pub fn data_ready(&self) -> DataReadyR {
        DataReadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive FIFO overrun"]
    #[inline(always)]
    pub fn rx_overrun(&self) -> RxOverrunR {
        RxOverrunR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit FIFO has room for at least one byte"]
    #[inline(always)]
    pub fn tx_empty(&self) -> TxEmptyR {
        TxEmptyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit FIFO empty and all bits shifted out"]
    #[inline(always)]
    pub fn tx_idle(&self) -> TxIdleR {
        TxIdleR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LSR")
            .field("tx_idle", &self.tx_idle())
            .field("tx_empty", &self.tx_empty())
            .field("rx_overrun", &self.rx_overrun())
            .field("data_ready", &self.data_ready())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Receive FIFO has at least one byte"]
    #[inline(always)]
    #[must_use]
    pub fn data_ready(&mut self) -> DataReadyW<LsrSpec> {
        DataReadyW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive FIFO overrun"]
    #[inline(always)]
    #[must_use]
    pub fn rx_overrun(&mut self) -> RxOverrunW<LsrSpec> {
        RxOverrunW::new(self, 1)
    }
    #[doc = "Bit 5 - Transmit FIFO has room for at least one byte"]
    #[inline(always)]
    #[must_use]
    pub fn tx_empty(&mut self) -> TxEmptyW<LsrSpec> {
        TxEmptyW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmit FIFO empty and all bits shifted out"]
    #[inline(always)]
    #[must_use]
    pub fn tx_idle(&mut self) -> TxIdleW<LsrSpec> {
        TxIdleW::new(self, 6)
    }
}
#[doc = "Line Status\n\nYou can [`read`](crate::Reg::read) this register and get [`lsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LsrSpec;
impl crate::RegisterSpec for LsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsr::R`](R) reader structure"]
impl crate::Readable for LsrSpec {}
#[doc = "`write(|w| ..)` method takes [`lsr::W`](W) writer structure"]
impl crate::Writable for LsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LSR to value 0"]
impl crate::Resettable for LsrSpec {
    const RESET_VALUE: u32 = 0;
}

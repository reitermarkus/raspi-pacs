#[doc = "Register `IIR` reader"]
pub type R = crate::R<IirSpec>;
#[doc = "Register `IIR` writer"]
pub type W = crate::W<IirSpec>;
#[doc = "Field `nPENDING` reader - No pending interrupt"]
pub type NPendingR = crate::BitReader;
#[doc = "Field `nPENDING` writer - No pending interrupt"]
pub type NPendingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_READY` reader - Receive FIFO has at least 1 byte"]
pub type DataReadyR = crate::BitReader;
#[doc = "Field `DATA_READY` writer - Receive FIFO has at least 1 byte"]
pub type DataReadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_READY` reader - Transmit FIFO is empty"]
pub type TxReadyR = crate::BitReader;
#[doc = "Field `TX_READY` writer - Transmit FIFO is empty"]
pub type TxReadyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - No pending interrupt"]
    #[inline(always)]
    pub fn n_pending(&self) -> NPendingR {
        NPendingR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive FIFO has at least 1 byte"]
    #[inline(always)]
    pub fn data_ready(&self) -> DataReadyR {
        DataReadyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO is empty"]
    #[inline(always)]
    pub fn tx_ready(&self) -> TxReadyR {
        TxReadyR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IIR")
            .field("tx_ready", &self.tx_ready())
            .field("data_ready", &self.data_ready())
            .field("n_pending", &self.n_pending())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - No pending interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn n_pending(&mut self) -> NPendingW<IirSpec> {
        NPendingW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive FIFO has at least 1 byte"]
    #[inline(always)]
    #[must_use]
    pub fn data_ready(&mut self) -> DataReadyW<IirSpec> {
        DataReadyW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit FIFO is empty"]
    #[inline(always)]
    #[must_use]
    pub fn tx_ready(&mut self) -> TxReadyW<IirSpec> {
        TxReadyW::new(self, 2)
    }
}
#[doc = "Interrupt Identify\n\nYou can [`read`](crate::Reg::read) this register and get [`iir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IirSpec;
impl crate::RegisterSpec for IirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iir::R`](R) reader structure"]
impl crate::Readable for IirSpec {}
#[doc = "`write(|w| ..)` method takes [`iir::W`](W) writer structure"]
impl crate::Writable for IirSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IIR to value 0xb001"]
impl crate::Resettable for IirSpec {
    const RESET_VALUE: u32 = 0xb001;
}

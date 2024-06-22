#[doc = "Register `PENDING_1` reader"]
pub type R = crate::R<Pending1Spec>;
#[doc = "Field `TIMER_0` reader - Timer 0"]
pub type Timer0R = crate::BitReader;
#[doc = "Field `TIMER_1` reader - Timer 1"]
pub type Timer1R = crate::BitReader;
#[doc = "Field `TIMER_2` reader - Timer 2"]
pub type Timer2R = crate::BitReader;
#[doc = "Field `TIMER_3` reader - Timer 3"]
pub type Timer3R = crate::BitReader;
#[doc = "Field `H264_0` reader - H264 0"]
pub type H264_0R = crate::BitReader;
#[doc = "Field `H264_1` reader - H264 1"]
pub type H264_1R = crate::BitReader;
#[doc = "Field `H264_2` reader - H264 2"]
pub type H264_2R = crate::BitReader;
#[doc = "Field `JPEG` reader - JPEG"]
pub type JpegR = crate::BitReader;
#[doc = "Field `ISP` reader - ISP"]
pub type IspR = crate::BitReader;
#[doc = "Field `USB` reader - USB"]
pub type UsbR = crate::BitReader;
#[doc = "Field `V3D` reader - V3D"]
pub type V3dR = crate::BitReader;
#[doc = "Field `TRANSPOSER` reader - Transposer"]
pub type TransposerR = crate::BitReader;
#[doc = "Field `MULTICORE_SYNC_0` reader - Multicore Sync 0"]
pub type MulticoreSync0R = crate::BitReader;
#[doc = "Field `MULTICORE_SYNC_1` reader - Multicore Sync 1"]
pub type MulticoreSync1R = crate::BitReader;
#[doc = "Field `MULTICORE_SYNC_2` reader - Multicore Sync 2"]
pub type MulticoreSync2R = crate::BitReader;
#[doc = "Field `MULTICORE_SYNC_3` reader - Multicore Sync 3"]
pub type MulticoreSync3R = crate::BitReader;
#[doc = "Field `DMA_0` reader - DMA 0"]
pub type Dma0R = crate::BitReader;
#[doc = "Field `DMA_1` reader - DMA 1"]
pub type Dma1R = crate::BitReader;
#[doc = "Field `DMA_2` reader - DMA 2"]
pub type Dma2R = crate::BitReader;
#[doc = "Field `DMA_3` reader - DMA 3"]
pub type Dma3R = crate::BitReader;
#[doc = "Field `DMA_4` reader - DMA 4"]
pub type Dma4R = crate::BitReader;
#[doc = "Field `DMA_5` reader - DMA 5"]
pub type Dma5R = crate::BitReader;
#[doc = "Field `DMA_6` reader - DMA 6"]
pub type Dma6R = crate::BitReader;
#[doc = "Field `DMA_7_8` reader - OR of DMA 7 and 8"]
pub type Dma7_8R = crate::BitReader;
#[doc = "Field `DMA_9_10` reader - OR of DMA 9 and 10"]
pub type Dma9_10R = crate::BitReader;
#[doc = "Field `DMA_11` reader - DMA 11"]
pub type Dma11R = crate::BitReader;
#[doc = "Field `DMA_12` reader - DMA 12"]
pub type Dma12R = crate::BitReader;
#[doc = "Field `DMA_13` reader - DMA 13"]
pub type Dma13R = crate::BitReader;
#[doc = "Field `DMA_14` reader - DMA 14"]
pub type Dma14R = crate::BitReader;
#[doc = "Field `AUX` reader - OR of UART1, SPI1 and SPI2"]
pub type AuxR = crate::BitReader;
#[doc = "Field `ARM` reader - ARM"]
pub type ArmR = crate::BitReader;
#[doc = "Field `DMA_15` reader - DMA 15"]
pub type Dma15R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Timer 0"]
    #[inline(always)]
    pub fn timer_0(&self) -> Timer0R {
        Timer0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer 1"]
    #[inline(always)]
    pub fn timer_1(&self) -> Timer1R {
        Timer1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer 2"]
    #[inline(always)]
    pub fn timer_2(&self) -> Timer2R {
        Timer2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer 3"]
    #[inline(always)]
    pub fn timer_3(&self) -> Timer3R {
        Timer3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - H264 0"]
    #[inline(always)]
    pub fn h264_0(&self) -> H264_0R {
        H264_0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - H264 1"]
    #[inline(always)]
    pub fn h264_1(&self) -> H264_1R {
        H264_1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - H264 2"]
    #[inline(always)]
    pub fn h264_2(&self) -> H264_2R {
        H264_2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - JPEG"]
    #[inline(always)]
    pub fn jpeg(&self) -> JpegR {
        JpegR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ISP"]
    #[inline(always)]
    pub fn isp(&self) -> IspR {
        IspR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - USB"]
    #[inline(always)]
    pub fn usb(&self) -> UsbR {
        UsbR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - V3D"]
    #[inline(always)]
    pub fn v3d(&self) -> V3dR {
        V3dR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transposer"]
    #[inline(always)]
    pub fn transposer(&self) -> TransposerR {
        TransposerR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Multicore Sync 0"]
    #[inline(always)]
    pub fn multicore_sync_0(&self) -> MulticoreSync0R {
        MulticoreSync0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Multicore Sync 1"]
    #[inline(always)]
    pub fn multicore_sync_1(&self) -> MulticoreSync1R {
        MulticoreSync1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Multicore Sync 2"]
    #[inline(always)]
    pub fn multicore_sync_2(&self) -> MulticoreSync2R {
        MulticoreSync2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Multicore Sync 3"]
    #[inline(always)]
    pub fn multicore_sync_3(&self) -> MulticoreSync3R {
        MulticoreSync3R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DMA 0"]
    #[inline(always)]
    pub fn dma_0(&self) -> Dma0R {
        Dma0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DMA 1"]
    #[inline(always)]
    pub fn dma_1(&self) -> Dma1R {
        Dma1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DMA 2"]
    #[inline(always)]
    pub fn dma_2(&self) -> Dma2R {
        Dma2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DMA 3"]
    #[inline(always)]
    pub fn dma_3(&self) -> Dma3R {
        Dma3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DMA 4"]
    #[inline(always)]
    pub fn dma_4(&self) -> Dma4R {
        Dma4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA 5"]
    #[inline(always)]
    pub fn dma_5(&self) -> Dma5R {
        Dma5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA 6"]
    #[inline(always)]
    pub fn dma_6(&self) -> Dma6R {
        Dma6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - OR of DMA 7 and 8"]
    #[inline(always)]
    pub fn dma_7_8(&self) -> Dma7_8R {
        Dma7_8R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - OR of DMA 9 and 10"]
    #[inline(always)]
    pub fn dma_9_10(&self) -> Dma9_10R {
        Dma9_10R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DMA 11"]
    #[inline(always)]
    pub fn dma_11(&self) -> Dma11R {
        Dma11R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DMA 12"]
    #[inline(always)]
    pub fn dma_12(&self) -> Dma12R {
        Dma12R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DMA 13"]
    #[inline(always)]
    pub fn dma_13(&self) -> Dma13R {
        Dma13R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DMA 14"]
    #[inline(always)]
    pub fn dma_14(&self) -> Dma14R {
        Dma14R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - OR of UART1, SPI1 and SPI2"]
    #[inline(always)]
    pub fn aux(&self) -> AuxR {
        AuxR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - ARM"]
    #[inline(always)]
    pub fn arm(&self) -> ArmR {
        ArmR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DMA 15"]
    #[inline(always)]
    pub fn dma_15(&self) -> Dma15R {
        Dma15R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PENDING_1")
            .field("timer_0", &self.timer_0())
            .field("timer_1", &self.timer_1())
            .field("timer_2", &self.timer_2())
            .field("timer_3", &self.timer_3())
            .field("h264_0", &self.h264_0())
            .field("h264_1", &self.h264_1())
            .field("h264_2", &self.h264_2())
            .field("jpeg", &self.jpeg())
            .field("isp", &self.isp())
            .field("usb", &self.usb())
            .field("v3d", &self.v3d())
            .field("transposer", &self.transposer())
            .field("multicore_sync_0", &self.multicore_sync_0())
            .field("multicore_sync_1", &self.multicore_sync_1())
            .field("multicore_sync_2", &self.multicore_sync_2())
            .field("multicore_sync_3", &self.multicore_sync_3())
            .field("dma_0", &self.dma_0())
            .field("dma_1", &self.dma_1())
            .field("dma_2", &self.dma_2())
            .field("dma_3", &self.dma_3())
            .field("dma_4", &self.dma_4())
            .field("dma_5", &self.dma_5())
            .field("dma_6", &self.dma_6())
            .field("dma_7_8", &self.dma_7_8())
            .field("dma_9_10", &self.dma_9_10())
            .field("dma_11", &self.dma_11())
            .field("dma_12", &self.dma_12())
            .field("dma_13", &self.dma_13())
            .field("dma_14", &self.dma_14())
            .field("aux", &self.aux())
            .field("arm", &self.arm())
            .field("dma_15", &self.dma_15())
            .finish()
    }
}
#[doc = "Pending state for interrupts 1 - 31\n\nYou can [`read`](crate::Reg::read) this register and get [`pending_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pending1Spec;
impl crate::RegisterSpec for Pending1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pending_1::R`](R) reader structure"]
impl crate::Readable for Pending1Spec {}
#[doc = "`reset()` method sets PENDING_1 to value 0"]
impl crate::Resettable for Pending1Spec {
    const RESET_VALUE: u32 = 0;
}

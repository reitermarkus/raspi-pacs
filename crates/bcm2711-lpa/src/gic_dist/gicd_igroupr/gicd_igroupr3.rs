#[doc = "Register `GICD_IGROUPR3` reader"]
pub type R = crate::R<GicdIgroupr3Spec>;
#[doc = "Register `GICD_IGROUPR3` writer"]
pub type W = crate::W<GicdIgroupr3Spec>;
#[doc = "Field `TIMER_0` reader - Timer 0"]
pub type Timer0R = crate::BitReader;
#[doc = "Field `TIMER_0` writer - Timer 0"]
pub type Timer0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_1` reader - Timer 1"]
pub type Timer1R = crate::BitReader;
#[doc = "Field `TIMER_1` writer - Timer 1"]
pub type Timer1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_2` reader - Timer 2"]
pub type Timer2R = crate::BitReader;
#[doc = "Field `TIMER_2` writer - Timer 2"]
pub type Timer2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_3` reader - Timer 3"]
pub type Timer3R = crate::BitReader;
#[doc = "Field `TIMER_3` writer - Timer 3"]
pub type Timer3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H264_0` reader - H264 0"]
pub type H264_0R = crate::BitReader;
#[doc = "Field `H264_0` writer - H264 0"]
pub type H264_0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H264_1` reader - H264 1"]
pub type H264_1R = crate::BitReader;
#[doc = "Field `H264_1` writer - H264 1"]
pub type H264_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H264_2` reader - H264 2"]
pub type H264_2R = crate::BitReader;
#[doc = "Field `H264_2` writer - H264 2"]
pub type H264_2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JPEG` reader - JPEG"]
pub type JpegR = crate::BitReader;
#[doc = "Field `JPEG` writer - JPEG"]
pub type JpegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISP` reader - ISP"]
pub type IspR = crate::BitReader;
#[doc = "Field `ISP` writer - ISP"]
pub type IspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB` reader - USB"]
pub type UsbR = crate::BitReader;
#[doc = "Field `USB` writer - USB"]
pub type UsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `V3D` reader - V3D"]
pub type V3dR = crate::BitReader;
#[doc = "Field `V3D` writer - V3D"]
pub type V3dW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSPOSER` reader - Transposer"]
pub type TransposerR = crate::BitReader;
#[doc = "Field `TRANSPOSER` writer - Transposer"]
pub type TransposerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MULTICORE_SYNC_0` reader - Multicore Sync 0"]
pub type MulticoreSync0R = crate::BitReader;
#[doc = "Field `MULTICORE_SYNC_0` writer - Multicore Sync 0"]
pub type MulticoreSync0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MULTICORE_SYNC_1` reader - Multicore Sync 1"]
pub type MulticoreSync1R = crate::BitReader;
#[doc = "Field `MULTICORE_SYNC_1` writer - Multicore Sync 1"]
pub type MulticoreSync1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MULTICORE_SYNC_2` reader - Multicore Sync 2"]
pub type MulticoreSync2R = crate::BitReader;
#[doc = "Field `MULTICORE_SYNC_2` writer - Multicore Sync 2"]
pub type MulticoreSync2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MULTICORE_SYNC_3` reader - Multicore Sync 3"]
pub type MulticoreSync3R = crate::BitReader;
#[doc = "Field `MULTICORE_SYNC_3` writer - Multicore Sync 3"]
pub type MulticoreSync3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_0` reader - DMA 0"]
pub type Dma0R = crate::BitReader;
#[doc = "Field `DMA_0` writer - DMA 0"]
pub type Dma0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_1` reader - DMA 1"]
pub type Dma1R = crate::BitReader;
#[doc = "Field `DMA_1` writer - DMA 1"]
pub type Dma1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_2` reader - DMA 2"]
pub type Dma2R = crate::BitReader;
#[doc = "Field `DMA_2` writer - DMA 2"]
pub type Dma2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_3` reader - DMA 3"]
pub type Dma3R = crate::BitReader;
#[doc = "Field `DMA_3` writer - DMA 3"]
pub type Dma3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_4` reader - DMA 4"]
pub type Dma4R = crate::BitReader;
#[doc = "Field `DMA_4` writer - DMA 4"]
pub type Dma4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_5` reader - DMA 5"]
pub type Dma5R = crate::BitReader;
#[doc = "Field `DMA_5` writer - DMA 5"]
pub type Dma5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_6` reader - DMA 6"]
pub type Dma6R = crate::BitReader;
#[doc = "Field `DMA_6` writer - DMA 6"]
pub type Dma6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_7_8` reader - OR of DMA 7 and 8"]
pub type Dma7_8R = crate::BitReader;
#[doc = "Field `DMA_7_8` writer - OR of DMA 7 and 8"]
pub type Dma7_8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_9_10` reader - OR of DMA 9 and 10"]
pub type Dma9_10R = crate::BitReader;
#[doc = "Field `DMA_9_10` writer - OR of DMA 9 and 10"]
pub type Dma9_10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_11` reader - DMA 11"]
pub type Dma11R = crate::BitReader;
#[doc = "Field `DMA_11` writer - DMA 11"]
pub type Dma11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_12` reader - DMA 12"]
pub type Dma12R = crate::BitReader;
#[doc = "Field `DMA_12` writer - DMA 12"]
pub type Dma12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_13` reader - DMA 13"]
pub type Dma13R = crate::BitReader;
#[doc = "Field `DMA_13` writer - DMA 13"]
pub type Dma13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_14` reader - DMA 14"]
pub type Dma14R = crate::BitReader;
#[doc = "Field `DMA_14` writer - DMA 14"]
pub type Dma14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX` reader - OR of UART1, SPI1 and SPI2"]
pub type AuxR = crate::BitReader;
#[doc = "Field `AUX` writer - OR of UART1, SPI1 and SPI2"]
pub type AuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARM` reader - ARM"]
pub type ArmR = crate::BitReader;
#[doc = "Field `ARM` writer - ARM"]
pub type ArmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_15` reader - DMA 15"]
pub type Dma15R = crate::BitReader;
#[doc = "Field `DMA_15` writer - DMA 15"]
pub type Dma15W<'a, REG> = crate::BitWriter<'a, REG>;
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
        f.debug_struct("GICD_IGROUPR3")
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
impl W {
    #[doc = "Bit 0 - Timer 0"]
    #[inline(always)]
    #[must_use]
    pub fn timer_0(&mut self) -> Timer0W<GicdIgroupr3Spec> {
        Timer0W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer 1"]
    #[inline(always)]
    #[must_use]
    pub fn timer_1(&mut self) -> Timer1W<GicdIgroupr3Spec> {
        Timer1W::new(self, 1)
    }
    #[doc = "Bit 2 - Timer 2"]
    #[inline(always)]
    #[must_use]
    pub fn timer_2(&mut self) -> Timer2W<GicdIgroupr3Spec> {
        Timer2W::new(self, 2)
    }
    #[doc = "Bit 3 - Timer 3"]
    #[inline(always)]
    #[must_use]
    pub fn timer_3(&mut self) -> Timer3W<GicdIgroupr3Spec> {
        Timer3W::new(self, 3)
    }
    #[doc = "Bit 4 - H264 0"]
    #[inline(always)]
    #[must_use]
    pub fn h264_0(&mut self) -> H264_0W<GicdIgroupr3Spec> {
        H264_0W::new(self, 4)
    }
    #[doc = "Bit 5 - H264 1"]
    #[inline(always)]
    #[must_use]
    pub fn h264_1(&mut self) -> H264_1W<GicdIgroupr3Spec> {
        H264_1W::new(self, 5)
    }
    #[doc = "Bit 6 - H264 2"]
    #[inline(always)]
    #[must_use]
    pub fn h264_2(&mut self) -> H264_2W<GicdIgroupr3Spec> {
        H264_2W::new(self, 6)
    }
    #[doc = "Bit 7 - JPEG"]
    #[inline(always)]
    #[must_use]
    pub fn jpeg(&mut self) -> JpegW<GicdIgroupr3Spec> {
        JpegW::new(self, 7)
    }
    #[doc = "Bit 8 - ISP"]
    #[inline(always)]
    #[must_use]
    pub fn isp(&mut self) -> IspW<GicdIgroupr3Spec> {
        IspW::new(self, 8)
    }
    #[doc = "Bit 9 - USB"]
    #[inline(always)]
    #[must_use]
    pub fn usb(&mut self) -> UsbW<GicdIgroupr3Spec> {
        UsbW::new(self, 9)
    }
    #[doc = "Bit 10 - V3D"]
    #[inline(always)]
    #[must_use]
    pub fn v3d(&mut self) -> V3dW<GicdIgroupr3Spec> {
        V3dW::new(self, 10)
    }
    #[doc = "Bit 11 - Transposer"]
    #[inline(always)]
    #[must_use]
    pub fn transposer(&mut self) -> TransposerW<GicdIgroupr3Spec> {
        TransposerW::new(self, 11)
    }
    #[doc = "Bit 12 - Multicore Sync 0"]
    #[inline(always)]
    #[must_use]
    pub fn multicore_sync_0(&mut self) -> MulticoreSync0W<GicdIgroupr3Spec> {
        MulticoreSync0W::new(self, 12)
    }
    #[doc = "Bit 13 - Multicore Sync 1"]
    #[inline(always)]
    #[must_use]
    pub fn multicore_sync_1(&mut self) -> MulticoreSync1W<GicdIgroupr3Spec> {
        MulticoreSync1W::new(self, 13)
    }
    #[doc = "Bit 14 - Multicore Sync 2"]
    #[inline(always)]
    #[must_use]
    pub fn multicore_sync_2(&mut self) -> MulticoreSync2W<GicdIgroupr3Spec> {
        MulticoreSync2W::new(self, 14)
    }
    #[doc = "Bit 15 - Multicore Sync 3"]
    #[inline(always)]
    #[must_use]
    pub fn multicore_sync_3(&mut self) -> MulticoreSync3W<GicdIgroupr3Spec> {
        MulticoreSync3W::new(self, 15)
    }
    #[doc = "Bit 16 - DMA 0"]
    #[inline(always)]
    #[must_use]
    pub fn dma_0(&mut self) -> Dma0W<GicdIgroupr3Spec> {
        Dma0W::new(self, 16)
    }
    #[doc = "Bit 17 - DMA 1"]
    #[inline(always)]
    #[must_use]
    pub fn dma_1(&mut self) -> Dma1W<GicdIgroupr3Spec> {
        Dma1W::new(self, 17)
    }
    #[doc = "Bit 18 - DMA 2"]
    #[inline(always)]
    #[must_use]
    pub fn dma_2(&mut self) -> Dma2W<GicdIgroupr3Spec> {
        Dma2W::new(self, 18)
    }
    #[doc = "Bit 19 - DMA 3"]
    #[inline(always)]
    #[must_use]
    pub fn dma_3(&mut self) -> Dma3W<GicdIgroupr3Spec> {
        Dma3W::new(self, 19)
    }
    #[doc = "Bit 20 - DMA 4"]
    #[inline(always)]
    #[must_use]
    pub fn dma_4(&mut self) -> Dma4W<GicdIgroupr3Spec> {
        Dma4W::new(self, 20)
    }
    #[doc = "Bit 21 - DMA 5"]
    #[inline(always)]
    #[must_use]
    pub fn dma_5(&mut self) -> Dma5W<GicdIgroupr3Spec> {
        Dma5W::new(self, 21)
    }
    #[doc = "Bit 22 - DMA 6"]
    #[inline(always)]
    #[must_use]
    pub fn dma_6(&mut self) -> Dma6W<GicdIgroupr3Spec> {
        Dma6W::new(self, 22)
    }
    #[doc = "Bit 23 - OR of DMA 7 and 8"]
    #[inline(always)]
    #[must_use]
    pub fn dma_7_8(&mut self) -> Dma7_8W<GicdIgroupr3Spec> {
        Dma7_8W::new(self, 23)
    }
    #[doc = "Bit 24 - OR of DMA 9 and 10"]
    #[inline(always)]
    #[must_use]
    pub fn dma_9_10(&mut self) -> Dma9_10W<GicdIgroupr3Spec> {
        Dma9_10W::new(self, 24)
    }
    #[doc = "Bit 25 - DMA 11"]
    #[inline(always)]
    #[must_use]
    pub fn dma_11(&mut self) -> Dma11W<GicdIgroupr3Spec> {
        Dma11W::new(self, 25)
    }
    #[doc = "Bit 26 - DMA 12"]
    #[inline(always)]
    #[must_use]
    pub fn dma_12(&mut self) -> Dma12W<GicdIgroupr3Spec> {
        Dma12W::new(self, 26)
    }
    #[doc = "Bit 27 - DMA 13"]
    #[inline(always)]
    #[must_use]
    pub fn dma_13(&mut self) -> Dma13W<GicdIgroupr3Spec> {
        Dma13W::new(self, 27)
    }
    #[doc = "Bit 28 - DMA 14"]
    #[inline(always)]
    #[must_use]
    pub fn dma_14(&mut self) -> Dma14W<GicdIgroupr3Spec> {
        Dma14W::new(self, 28)
    }
    #[doc = "Bit 29 - OR of UART1, SPI1 and SPI2"]
    #[inline(always)]
    #[must_use]
    pub fn aux(&mut self) -> AuxW<GicdIgroupr3Spec> {
        AuxW::new(self, 29)
    }
    #[doc = "Bit 30 - ARM"]
    #[inline(always)]
    #[must_use]
    pub fn arm(&mut self) -> ArmW<GicdIgroupr3Spec> {
        ArmW::new(self, 30)
    }
    #[doc = "Bit 31 - DMA 15"]
    #[inline(always)]
    #[must_use]
    pub fn dma_15(&mut self) -> Dma15W<GicdIgroupr3Spec> {
        Dma15W::new(self, 31)
    }
}
#[doc = "Interrupt Group\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_igroupr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_igroupr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIgroupr3Spec;
impl crate::RegisterSpec for GicdIgroupr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_igroupr3::R`](R) reader structure"]
impl crate::Readable for GicdIgroupr3Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_igroupr3::W`](W) writer structure"]
impl crate::Writable for GicdIgroupr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IGROUPR3 to value 0"]
impl crate::Resettable for GicdIgroupr3Spec {
    const RESET_VALUE: u32 = 0;
}

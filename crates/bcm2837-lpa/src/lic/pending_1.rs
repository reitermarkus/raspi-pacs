#[doc = "Register `PENDING_1` reader"]
pub struct R(crate::R<PENDING_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PENDING_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PENDING_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PENDING_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMER_0` reader - Timer 0"]
pub type TIMER_0_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_1` reader - Timer 1"]
pub type TIMER_1_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_2` reader - Timer 2"]
pub type TIMER_2_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_3` reader - Timer 3"]
pub type TIMER_3_R = crate::BitReader<bool>;
#[doc = "Field `H264_0` reader - H264 0"]
pub type H264_0_R = crate::BitReader<bool>;
#[doc = "Field `H264_1` reader - H264 1"]
pub type H264_1_R = crate::BitReader<bool>;
#[doc = "Field `H264_2` reader - H264 2"]
pub type H264_2_R = crate::BitReader<bool>;
#[doc = "Field `JPEG` reader - JPEG"]
pub type JPEG_R = crate::BitReader<bool>;
#[doc = "Field `ISP` reader - ISP"]
pub type ISP_R = crate::BitReader<bool>;
#[doc = "Field `USB` reader - USB"]
pub type USB_R = crate::BitReader<bool>;
#[doc = "Field `V3D` reader - V3D"]
pub type V3D_R = crate::BitReader<bool>;
#[doc = "Field `TRANSPOSER` reader - Transposer"]
pub type TRANSPOSER_R = crate::BitReader<bool>;
#[doc = "Field `MULTICORE_SYNC_0` reader - Multicore Sync 0"]
pub type MULTICORE_SYNC_0_R = crate::BitReader<bool>;
#[doc = "Field `MULTICORE_SYNC_1` reader - Multicore Sync 1"]
pub type MULTICORE_SYNC_1_R = crate::BitReader<bool>;
#[doc = "Field `MULTICORE_SYNC_2` reader - Multicore Sync 2"]
pub type MULTICORE_SYNC_2_R = crate::BitReader<bool>;
#[doc = "Field `MULTICORE_SYNC_3` reader - Multicore Sync 3"]
pub type MULTICORE_SYNC_3_R = crate::BitReader<bool>;
#[doc = "Field `DMA_0` reader - DMA 0"]
pub type DMA_0_R = crate::BitReader<bool>;
#[doc = "Field `DMA_1` reader - DMA 1"]
pub type DMA_1_R = crate::BitReader<bool>;
#[doc = "Field `DMA_2` reader - DMA 2"]
pub type DMA_2_R = crate::BitReader<bool>;
#[doc = "Field `DMA_3` reader - DMA 3"]
pub type DMA_3_R = crate::BitReader<bool>;
#[doc = "Field `DMA_4` reader - DMA 4"]
pub type DMA_4_R = crate::BitReader<bool>;
#[doc = "Field `DMA_5` reader - DMA 5"]
pub type DMA_5_R = crate::BitReader<bool>;
#[doc = "Field `DMA_6` reader - DMA 6"]
pub type DMA_6_R = crate::BitReader<bool>;
#[doc = "Field `DMA_7_8` reader - OR of DMA 7 and 8"]
pub type DMA_7_8_R = crate::BitReader<bool>;
#[doc = "Field `DMA_9_10` reader - OR of DMA 9 and 10"]
pub type DMA_9_10_R = crate::BitReader<bool>;
#[doc = "Field `DMA_11` reader - DMA 11"]
pub type DMA_11_R = crate::BitReader<bool>;
#[doc = "Field `DMA_12` reader - DMA 12"]
pub type DMA_12_R = crate::BitReader<bool>;
#[doc = "Field `DMA_13` reader - DMA 13"]
pub type DMA_13_R = crate::BitReader<bool>;
#[doc = "Field `DMA_14` reader - DMA 14"]
pub type DMA_14_R = crate::BitReader<bool>;
#[doc = "Field `AUX` reader - OR of UART1, SPI1 and SPI2"]
pub type AUX_R = crate::BitReader<bool>;
#[doc = "Field `ARM` reader - ARM"]
pub type ARM_R = crate::BitReader<bool>;
#[doc = "Field `DMA_15` reader - DMA 15"]
pub type DMA_15_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Timer 0"]
    #[inline(always)]
    pub fn timer_0(&self) -> TIMER_0_R {
        TIMER_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer 1"]
    #[inline(always)]
    pub fn timer_1(&self) -> TIMER_1_R {
        TIMER_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer 2"]
    #[inline(always)]
    pub fn timer_2(&self) -> TIMER_2_R {
        TIMER_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer 3"]
    #[inline(always)]
    pub fn timer_3(&self) -> TIMER_3_R {
        TIMER_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - H264 0"]
    #[inline(always)]
    pub fn h264_0(&self) -> H264_0_R {
        H264_0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - H264 1"]
    #[inline(always)]
    pub fn h264_1(&self) -> H264_1_R {
        H264_1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - H264 2"]
    #[inline(always)]
    pub fn h264_2(&self) -> H264_2_R {
        H264_2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - JPEG"]
    #[inline(always)]
    pub fn jpeg(&self) -> JPEG_R {
        JPEG_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ISP"]
    #[inline(always)]
    pub fn isp(&self) -> ISP_R {
        ISP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - USB"]
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - V3D"]
    #[inline(always)]
    pub fn v3d(&self) -> V3D_R {
        V3D_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transposer"]
    #[inline(always)]
    pub fn transposer(&self) -> TRANSPOSER_R {
        TRANSPOSER_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Multicore Sync 0"]
    #[inline(always)]
    pub fn multicore_sync_0(&self) -> MULTICORE_SYNC_0_R {
        MULTICORE_SYNC_0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Multicore Sync 1"]
    #[inline(always)]
    pub fn multicore_sync_1(&self) -> MULTICORE_SYNC_1_R {
        MULTICORE_SYNC_1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Multicore Sync 2"]
    #[inline(always)]
    pub fn multicore_sync_2(&self) -> MULTICORE_SYNC_2_R {
        MULTICORE_SYNC_2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Multicore Sync 3"]
    #[inline(always)]
    pub fn multicore_sync_3(&self) -> MULTICORE_SYNC_3_R {
        MULTICORE_SYNC_3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DMA 0"]
    #[inline(always)]
    pub fn dma_0(&self) -> DMA_0_R {
        DMA_0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DMA 1"]
    #[inline(always)]
    pub fn dma_1(&self) -> DMA_1_R {
        DMA_1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DMA 2"]
    #[inline(always)]
    pub fn dma_2(&self) -> DMA_2_R {
        DMA_2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DMA 3"]
    #[inline(always)]
    pub fn dma_3(&self) -> DMA_3_R {
        DMA_3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DMA 4"]
    #[inline(always)]
    pub fn dma_4(&self) -> DMA_4_R {
        DMA_4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA 5"]
    #[inline(always)]
    pub fn dma_5(&self) -> DMA_5_R {
        DMA_5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA 6"]
    #[inline(always)]
    pub fn dma_6(&self) -> DMA_6_R {
        DMA_6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - OR of DMA 7 and 8"]
    #[inline(always)]
    pub fn dma_7_8(&self) -> DMA_7_8_R {
        DMA_7_8_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - OR of DMA 9 and 10"]
    #[inline(always)]
    pub fn dma_9_10(&self) -> DMA_9_10_R {
        DMA_9_10_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DMA 11"]
    #[inline(always)]
    pub fn dma_11(&self) -> DMA_11_R {
        DMA_11_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DMA 12"]
    #[inline(always)]
    pub fn dma_12(&self) -> DMA_12_R {
        DMA_12_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DMA 13"]
    #[inline(always)]
    pub fn dma_13(&self) -> DMA_13_R {
        DMA_13_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DMA 14"]
    #[inline(always)]
    pub fn dma_14(&self) -> DMA_14_R {
        DMA_14_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - OR of UART1, SPI1 and SPI2"]
    #[inline(always)]
    pub fn aux(&self) -> AUX_R {
        AUX_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - ARM"]
    #[inline(always)]
    pub fn arm(&self) -> ARM_R {
        ARM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DMA 15"]
    #[inline(always)]
    pub fn dma_15(&self) -> DMA_15_R {
        DMA_15_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Pending state for interrupts 1 - 31\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pending_1](index.html) module"]
pub struct PENDING_1_SPEC;
impl crate::RegisterSpec for PENDING_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pending_1::R](R) reader structure"]
impl crate::Readable for PENDING_1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PENDING_1 to value 0"]
impl crate::Resettable for PENDING_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

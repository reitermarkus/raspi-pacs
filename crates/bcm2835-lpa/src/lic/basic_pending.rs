#[doc = "Register `BASIC_PENDING` reader"]
pub type R = crate::R<BasicPendingSpec>;
#[doc = "Field `TIMER` reader - ARMC Timer"]
pub type TimerR = crate::BitReader;
#[doc = "Field `MAILBOX` reader - Mailbox"]
pub type MailboxR = crate::BitReader;
#[doc = "Field `DOORBELL0` reader - Doorbell 0"]
pub type Doorbell0R = crate::BitReader;
#[doc = "Field `DOORBELL1` reader - Doorbell 1"]
pub type Doorbell1R = crate::BitReader;
#[doc = "Field `VPU0_HALTED` reader - VPU0 halted"]
pub type Vpu0HaltedR = crate::BitReader;
#[doc = "Field `VPU1_HALTED` reader - VPU1 halted"]
pub type Vpu1HaltedR = crate::BitReader;
#[doc = "Field `ARM_ADDRESS_ERROR` reader - ARM address error"]
pub type ArmAddressErrorR = crate::BitReader;
#[doc = "Field `ARM_AXI_ERROR` reader - ARM AXI error"]
pub type ArmAxiErrorR = crate::BitReader;
#[doc = "Field `PENDING_1` reader - One or more bits are set in PENDING_1 (ignores 7, 9, 10, 18, 19)"]
pub type Pending1R = crate::BitReader;
#[doc = "Field `PENDING_2` reader - One or more bits are set in PENDING_2 (ignores 53 - 57, 62)"]
pub type Pending2R = crate::BitReader;
#[doc = "Field `JPEG` reader - JPEG"]
pub type JpegR = crate::BitReader;
#[doc = "Field `USB` reader - USB"]
pub type UsbR = crate::BitReader;
#[doc = "Field `V3D` reader - V3D"]
pub type V3dR = crate::BitReader;
#[doc = "Field `DMA_2` reader - DMA 2"]
pub type Dma2R = crate::BitReader;
#[doc = "Field `DMA_3` reader - DMA 3"]
pub type Dma3R = crate::BitReader;
#[doc = "Field `I2C` reader - OR of all I2C"]
pub type I2cR = crate::BitReader;
#[doc = "Field `SPI` reader - OR of all SPI"]
pub type SpiR = crate::BitReader;
#[doc = "Field `PCM_I2S` reader - PCM/I2S"]
pub type PcmI2sR = crate::BitReader;
#[doc = "Field `SDHOST` reader - SDHOST"]
pub type SdhostR = crate::BitReader;
#[doc = "Field `UART` reader - OR of all PL011 UARTs"]
pub type UartR = crate::BitReader;
#[doc = "Field `EMMC` reader - OR of EMMC and EMMC2"]
pub type EmmcR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ARMC Timer"]
    #[inline(always)]
    pub fn timer(&self) -> TimerR {
        TimerR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mailbox"]
    #[inline(always)]
    pub fn mailbox(&self) -> MailboxR {
        MailboxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Doorbell 0"]
    #[inline(always)]
    pub fn doorbell0(&self) -> Doorbell0R {
        Doorbell0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Doorbell 1"]
    #[inline(always)]
    pub fn doorbell1(&self) -> Doorbell1R {
        Doorbell1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VPU0 halted"]
    #[inline(always)]
    pub fn vpu0_halted(&self) -> Vpu0HaltedR {
        Vpu0HaltedR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - VPU1 halted"]
    #[inline(always)]
    pub fn vpu1_halted(&self) -> Vpu1HaltedR {
        Vpu1HaltedR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ARM address error"]
    #[inline(always)]
    pub fn arm_address_error(&self) -> ArmAddressErrorR {
        ArmAddressErrorR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ARM AXI error"]
    #[inline(always)]
    pub fn arm_axi_error(&self) -> ArmAxiErrorR {
        ArmAxiErrorR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - One or more bits are set in PENDING_1 (ignores 7, 9, 10, 18, 19)"]
    #[inline(always)]
    pub fn pending_1(&self) -> Pending1R {
        Pending1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - One or more bits are set in PENDING_2 (ignores 53 - 57, 62)"]
    #[inline(always)]
    pub fn pending_2(&self) -> Pending2R {
        Pending2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - JPEG"]
    #[inline(always)]
    pub fn jpeg(&self) -> JpegR {
        JpegR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USB"]
    #[inline(always)]
    pub fn usb(&self) -> UsbR {
        UsbR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - V3D"]
    #[inline(always)]
    pub fn v3d(&self) -> V3dR {
        V3dR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DMA 2"]
    #[inline(always)]
    pub fn dma_2(&self) -> Dma2R {
        Dma2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DMA 3"]
    #[inline(always)]
    pub fn dma_3(&self) -> Dma3R {
        Dma3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - OR of all I2C"]
    #[inline(always)]
    pub fn i2c(&self) -> I2cR {
        I2cR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - OR of all SPI"]
    #[inline(always)]
    pub fn spi(&self) -> SpiR {
        SpiR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PCM/I2S"]
    #[inline(always)]
    pub fn pcm_i2s(&self) -> PcmI2sR {
        PcmI2sR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SDHOST"]
    #[inline(always)]
    pub fn sdhost(&self) -> SdhostR {
        SdhostR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OR of all PL011 UARTs"]
    #[inline(always)]
    pub fn uart(&self) -> UartR {
        UartR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - OR of EMMC and EMMC2"]
    #[inline(always)]
    pub fn emmc(&self) -> EmmcR {
        EmmcR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BASIC_PENDING")
            .field("timer", &self.timer())
            .field("mailbox", &self.mailbox())
            .field("doorbell0", &self.doorbell0())
            .field("doorbell1", &self.doorbell1())
            .field("vpu0_halted", &self.vpu0_halted())
            .field("vpu1_halted", &self.vpu1_halted())
            .field("arm_address_error", &self.arm_address_error())
            .field("arm_axi_error", &self.arm_axi_error())
            .field("pending_1", &self.pending_1())
            .field("pending_2", &self.pending_2())
            .field("jpeg", &self.jpeg())
            .field("usb", &self.usb())
            .field("v3d", &self.v3d())
            .field("dma_2", &self.dma_2())
            .field("dma_3", &self.dma_3())
            .field("i2c", &self.i2c())
            .field("spi", &self.spi())
            .field("pcm_i2s", &self.pcm_i2s())
            .field("sdhost", &self.sdhost())
            .field("uart", &self.uart())
            .field("emmc", &self.emmc())
            .finish()
    }
}
#[doc = "Basic pending info\n\nYou can [`read`](crate::Reg::read) this register and get [`basic_pending::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BasicPendingSpec;
impl crate::RegisterSpec for BasicPendingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`basic_pending::R`](R) reader structure"]
impl crate::Readable for BasicPendingSpec {}
#[doc = "`reset()` method sets BASIC_PENDING to value 0"]
impl crate::Resettable for BasicPendingSpec {
    const RESET_VALUE: u32 = 0;
}

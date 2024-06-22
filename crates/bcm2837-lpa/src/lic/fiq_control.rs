#[doc = "Register `FIQ_CONTROL` reader"]
pub type R = crate::R<FiqControlSpec>;
#[doc = "Register `FIQ_CONTROL` writer"]
pub type W = crate::W<FiqControlSpec>;
#[doc = "FIQ Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Source {
    #[doc = "0: Timer 0"]
    Timer0 = 0,
    #[doc = "1: Timer 1"]
    Timer1 = 1,
    #[doc = "2: Timer 2"]
    Timer2 = 2,
    #[doc = "3: Timer 3"]
    Timer3 = 3,
    #[doc = "4: H264 0"]
    H264_0 = 4,
    #[doc = "5: H264 1"]
    H264_1 = 5,
    #[doc = "6: H264 2"]
    H264_2 = 6,
    #[doc = "7: JPEG"]
    Jpeg = 7,
    #[doc = "8: ISP"]
    Isp = 8,
    #[doc = "9: USB"]
    Usb = 9,
    #[doc = "10: V3D"]
    V3d = 10,
    #[doc = "11: Transposer"]
    Transposer = 11,
    #[doc = "12: Multicore Sync 0"]
    MulticoreSync0 = 12,
    #[doc = "13: Multicore Sync 1"]
    MulticoreSync1 = 13,
    #[doc = "14: Multicore Sync 2"]
    MulticoreSync2 = 14,
    #[doc = "15: Multicore Sync 3"]
    MulticoreSync3 = 15,
    #[doc = "16: DMA 0"]
    Dma0 = 16,
    #[doc = "17: DMA 1"]
    Dma1 = 17,
    #[doc = "18: DMA 2"]
    Dma2 = 18,
    #[doc = "19: DMA 3"]
    Dma3 = 19,
    #[doc = "20: DMA 4"]
    Dma4 = 20,
    #[doc = "21: DMA 5"]
    Dma5 = 21,
    #[doc = "22: DMA 6"]
    Dma6 = 22,
    #[doc = "23: OR of DMA 7 and 8"]
    Dma7_8 = 23,
    #[doc = "24: OR of DMA 9 and 10"]
    Dma9_10 = 24,
    #[doc = "25: DMA 11"]
    Dma11 = 25,
    #[doc = "26: DMA 12"]
    Dma12 = 26,
    #[doc = "27: DMA 13"]
    Dma13 = 27,
    #[doc = "28: DMA 14"]
    Dma14 = 28,
    #[doc = "29: OR of UART1, SPI1 and SPI2"]
    Aux = 29,
    #[doc = "30: ARM"]
    Arm = 30,
    #[doc = "31: DMA 15"]
    Dma15 = 31,
    #[doc = "32: HDMI CEC"]
    HdmiCec = 32,
    #[doc = "33: HVS"]
    Hvs = 33,
    #[doc = "34: RPIVID"]
    Rpivid = 34,
    #[doc = "35: SDC"]
    Sdc = 35,
    #[doc = "36: DSI 0"]
    Dsi0 = 36,
    #[doc = "37: Pixel Valve 2"]
    PixelValve2 = 37,
    #[doc = "38: Camera 0"]
    Camera0 = 38,
    #[doc = "39: Camera 1"]
    Camera1 = 39,
    #[doc = "40: HDMI 0"]
    Hdmi0 = 40,
    #[doc = "41: HDMI 1"]
    Hdmi1 = 41,
    #[doc = "42: Pixel Valve 3"]
    PixelValve3 = 42,
    #[doc = "43: SPI/BSC Slave"]
    SpiBscSlave = 43,
    #[doc = "44: DSI 1"]
    Dsi1 = 44,
    #[doc = "45: Pixel Valve 0"]
    PixelValve0 = 45,
    #[doc = "46: OR of Pixel Valve 1 and 2"]
    PixelValve1_2 = 46,
    #[doc = "47: CPR"]
    Cpr = 47,
    #[doc = "48: SMI"]
    Smi = 48,
    #[doc = "49: GPIO 0"]
    Gpio0 = 49,
    #[doc = "50: GPIO 1"]
    Gpio1 = 50,
    #[doc = "51: GPIO 2"]
    Gpio2 = 51,
    #[doc = "52: GPIO 3"]
    Gpio3 = 52,
    #[doc = "53: OR of all I2C"]
    I2c = 53,
    #[doc = "54: OR of all SPI"]
    Spi = 54,
    #[doc = "55: PCM/I2S"]
    PcmI2s = 55,
    #[doc = "56: SDHOST"]
    Sdhost = 56,
    #[doc = "57: OR of all PL011 UARTs"]
    Uart = 57,
    #[doc = "58: OR of all ETH_PCIe L2"]
    EthPcie = 58,
    #[doc = "59: VEC"]
    Vec = 59,
    #[doc = "60: CPG"]
    Cpg = 60,
    #[doc = "61: RNG"]
    Rng = 61,
    #[doc = "62: OR of EMMC and EMMC2"]
    Emmc = 62,
    #[doc = "63: ETH_PCIe secure"]
    EthPcieSecure = 63,
    #[doc = "64: ARMC Timer"]
    Timer = 64,
    #[doc = "65: Mailbox"]
    Mailbox = 65,
    #[doc = "66: Doorbell 0"]
    Doorbell0 = 66,
    #[doc = "67: Doorbell 1"]
    Doorbell1 = 67,
    #[doc = "68: VPU0 halted"]
    Vpu0Halted = 68,
    #[doc = "69: VPU1 halted"]
    Vpu1Halted = 69,
    #[doc = "70: ARM address error"]
    ArmAddressError = 70,
    #[doc = "71: ARM AXI error"]
    ArmAxiError = 71,
}
impl From<Source> for u8 {
    #[inline(always)]
    fn from(variant: Source) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Source {
    type Ux = u8;
}
impl crate::IsEnum for Source {}
#[doc = "Field `SOURCE` reader - FIQ Source"]
pub type SourceR = crate::FieldReader<Source>;
impl SourceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Source> {
        match self.bits {
            0 => Some(Source::Timer0),
            1 => Some(Source::Timer1),
            2 => Some(Source::Timer2),
            3 => Some(Source::Timer3),
            4 => Some(Source::H264_0),
            5 => Some(Source::H264_1),
            6 => Some(Source::H264_2),
            7 => Some(Source::Jpeg),
            8 => Some(Source::Isp),
            9 => Some(Source::Usb),
            10 => Some(Source::V3d),
            11 => Some(Source::Transposer),
            12 => Some(Source::MulticoreSync0),
            13 => Some(Source::MulticoreSync1),
            14 => Some(Source::MulticoreSync2),
            15 => Some(Source::MulticoreSync3),
            16 => Some(Source::Dma0),
            17 => Some(Source::Dma1),
            18 => Some(Source::Dma2),
            19 => Some(Source::Dma3),
            20 => Some(Source::Dma4),
            21 => Some(Source::Dma5),
            22 => Some(Source::Dma6),
            23 => Some(Source::Dma7_8),
            24 => Some(Source::Dma9_10),
            25 => Some(Source::Dma11),
            26 => Some(Source::Dma12),
            27 => Some(Source::Dma13),
            28 => Some(Source::Dma14),
            29 => Some(Source::Aux),
            30 => Some(Source::Arm),
            31 => Some(Source::Dma15),
            32 => Some(Source::HdmiCec),
            33 => Some(Source::Hvs),
            34 => Some(Source::Rpivid),
            35 => Some(Source::Sdc),
            36 => Some(Source::Dsi0),
            37 => Some(Source::PixelValve2),
            38 => Some(Source::Camera0),
            39 => Some(Source::Camera1),
            40 => Some(Source::Hdmi0),
            41 => Some(Source::Hdmi1),
            42 => Some(Source::PixelValve3),
            43 => Some(Source::SpiBscSlave),
            44 => Some(Source::Dsi1),
            45 => Some(Source::PixelValve0),
            46 => Some(Source::PixelValve1_2),
            47 => Some(Source::Cpr),
            48 => Some(Source::Smi),
            49 => Some(Source::Gpio0),
            50 => Some(Source::Gpio1),
            51 => Some(Source::Gpio2),
            52 => Some(Source::Gpio3),
            53 => Some(Source::I2c),
            54 => Some(Source::Spi),
            55 => Some(Source::PcmI2s),
            56 => Some(Source::Sdhost),
            57 => Some(Source::Uart),
            58 => Some(Source::EthPcie),
            59 => Some(Source::Vec),
            60 => Some(Source::Cpg),
            61 => Some(Source::Rng),
            62 => Some(Source::Emmc),
            63 => Some(Source::EthPcieSecure),
            64 => Some(Source::Timer),
            65 => Some(Source::Mailbox),
            66 => Some(Source::Doorbell0),
            67 => Some(Source::Doorbell1),
            68 => Some(Source::Vpu0Halted),
            69 => Some(Source::Vpu1Halted),
            70 => Some(Source::ArmAddressError),
            71 => Some(Source::ArmAxiError),
            _ => None,
        }
    }
    #[doc = "Timer 0"]
    #[inline(always)]
    pub fn is_timer_0(&self) -> bool {
        *self == Source::Timer0
    }
    #[doc = "Timer 1"]
    #[inline(always)]
    pub fn is_timer_1(&self) -> bool {
        *self == Source::Timer1
    }
    #[doc = "Timer 2"]
    #[inline(always)]
    pub fn is_timer_2(&self) -> bool {
        *self == Source::Timer2
    }
    #[doc = "Timer 3"]
    #[inline(always)]
    pub fn is_timer_3(&self) -> bool {
        *self == Source::Timer3
    }
    #[doc = "H264 0"]
    #[inline(always)]
    pub fn is_h264_0(&self) -> bool {
        *self == Source::H264_0
    }
    #[doc = "H264 1"]
    #[inline(always)]
    pub fn is_h264_1(&self) -> bool {
        *self == Source::H264_1
    }
    #[doc = "H264 2"]
    #[inline(always)]
    pub fn is_h264_2(&self) -> bool {
        *self == Source::H264_2
    }
    #[doc = "JPEG"]
    #[inline(always)]
    pub fn is_jpeg(&self) -> bool {
        *self == Source::Jpeg
    }
    #[doc = "ISP"]
    #[inline(always)]
    pub fn is_isp(&self) -> bool {
        *self == Source::Isp
    }
    #[doc = "USB"]
    #[inline(always)]
    pub fn is_usb(&self) -> bool {
        *self == Source::Usb
    }
    #[doc = "V3D"]
    #[inline(always)]
    pub fn is_v3d(&self) -> bool {
        *self == Source::V3d
    }
    #[doc = "Transposer"]
    #[inline(always)]
    pub fn is_transposer(&self) -> bool {
        *self == Source::Transposer
    }
    #[doc = "Multicore Sync 0"]
    #[inline(always)]
    pub fn is_multicore_sync_0(&self) -> bool {
        *self == Source::MulticoreSync0
    }
    #[doc = "Multicore Sync 1"]
    #[inline(always)]
    pub fn is_multicore_sync_1(&self) -> bool {
        *self == Source::MulticoreSync1
    }
    #[doc = "Multicore Sync 2"]
    #[inline(always)]
    pub fn is_multicore_sync_2(&self) -> bool {
        *self == Source::MulticoreSync2
    }
    #[doc = "Multicore Sync 3"]
    #[inline(always)]
    pub fn is_multicore_sync_3(&self) -> bool {
        *self == Source::MulticoreSync3
    }
    #[doc = "DMA 0"]
    #[inline(always)]
    pub fn is_dma_0(&self) -> bool {
        *self == Source::Dma0
    }
    #[doc = "DMA 1"]
    #[inline(always)]
    pub fn is_dma_1(&self) -> bool {
        *self == Source::Dma1
    }
    #[doc = "DMA 2"]
    #[inline(always)]
    pub fn is_dma_2(&self) -> bool {
        *self == Source::Dma2
    }
    #[doc = "DMA 3"]
    #[inline(always)]
    pub fn is_dma_3(&self) -> bool {
        *self == Source::Dma3
    }
    #[doc = "DMA 4"]
    #[inline(always)]
    pub fn is_dma_4(&self) -> bool {
        *self == Source::Dma4
    }
    #[doc = "DMA 5"]
    #[inline(always)]
    pub fn is_dma_5(&self) -> bool {
        *self == Source::Dma5
    }
    #[doc = "DMA 6"]
    #[inline(always)]
    pub fn is_dma_6(&self) -> bool {
        *self == Source::Dma6
    }
    #[doc = "OR of DMA 7 and 8"]
    #[inline(always)]
    pub fn is_dma_7_8(&self) -> bool {
        *self == Source::Dma7_8
    }
    #[doc = "OR of DMA 9 and 10"]
    #[inline(always)]
    pub fn is_dma_9_10(&self) -> bool {
        *self == Source::Dma9_10
    }
    #[doc = "DMA 11"]
    #[inline(always)]
    pub fn is_dma_11(&self) -> bool {
        *self == Source::Dma11
    }
    #[doc = "DMA 12"]
    #[inline(always)]
    pub fn is_dma_12(&self) -> bool {
        *self == Source::Dma12
    }
    #[doc = "DMA 13"]
    #[inline(always)]
    pub fn is_dma_13(&self) -> bool {
        *self == Source::Dma13
    }
    #[doc = "DMA 14"]
    #[inline(always)]
    pub fn is_dma_14(&self) -> bool {
        *self == Source::Dma14
    }
    #[doc = "OR of UART1, SPI1 and SPI2"]
    #[inline(always)]
    pub fn is_aux(&self) -> bool {
        *self == Source::Aux
    }
    #[doc = "ARM"]
    #[inline(always)]
    pub fn is_arm(&self) -> bool {
        *self == Source::Arm
    }
    #[doc = "DMA 15"]
    #[inline(always)]
    pub fn is_dma_15(&self) -> bool {
        *self == Source::Dma15
    }
    #[doc = "HDMI CEC"]
    #[inline(always)]
    pub fn is_hdmi_cec(&self) -> bool {
        *self == Source::HdmiCec
    }
    #[doc = "HVS"]
    #[inline(always)]
    pub fn is_hvs(&self) -> bool {
        *self == Source::Hvs
    }
    #[doc = "RPIVID"]
    #[inline(always)]
    pub fn is_rpivid(&self) -> bool {
        *self == Source::Rpivid
    }
    #[doc = "SDC"]
    #[inline(always)]
    pub fn is_sdc(&self) -> bool {
        *self == Source::Sdc
    }
    #[doc = "DSI 0"]
    #[inline(always)]
    pub fn is_dsi_0(&self) -> bool {
        *self == Source::Dsi0
    }
    #[doc = "Pixel Valve 2"]
    #[inline(always)]
    pub fn is_pixel_valve_2(&self) -> bool {
        *self == Source::PixelValve2
    }
    #[doc = "Camera 0"]
    #[inline(always)]
    pub fn is_camera_0(&self) -> bool {
        *self == Source::Camera0
    }
    #[doc = "Camera 1"]
    #[inline(always)]
    pub fn is_camera_1(&self) -> bool {
        *self == Source::Camera1
    }
    #[doc = "HDMI 0"]
    #[inline(always)]
    pub fn is_hdmi_0(&self) -> bool {
        *self == Source::Hdmi0
    }
    #[doc = "HDMI 1"]
    #[inline(always)]
    pub fn is_hdmi_1(&self) -> bool {
        *self == Source::Hdmi1
    }
    #[doc = "Pixel Valve 3"]
    #[inline(always)]
    pub fn is_pixel_valve_3(&self) -> bool {
        *self == Source::PixelValve3
    }
    #[doc = "SPI/BSC Slave"]
    #[inline(always)]
    pub fn is_spi_bsc_slave(&self) -> bool {
        *self == Source::SpiBscSlave
    }
    #[doc = "DSI 1"]
    #[inline(always)]
    pub fn is_dsi_1(&self) -> bool {
        *self == Source::Dsi1
    }
    #[doc = "Pixel Valve 0"]
    #[inline(always)]
    pub fn is_pixel_valve_0(&self) -> bool {
        *self == Source::PixelValve0
    }
    #[doc = "OR of Pixel Valve 1 and 2"]
    #[inline(always)]
    pub fn is_pixel_valve_1_2(&self) -> bool {
        *self == Source::PixelValve1_2
    }
    #[doc = "CPR"]
    #[inline(always)]
    pub fn is_cpr(&self) -> bool {
        *self == Source::Cpr
    }
    #[doc = "SMI"]
    #[inline(always)]
    pub fn is_smi(&self) -> bool {
        *self == Source::Smi
    }
    #[doc = "GPIO 0"]
    #[inline(always)]
    pub fn is_gpio_0(&self) -> bool {
        *self == Source::Gpio0
    }
    #[doc = "GPIO 1"]
    #[inline(always)]
    pub fn is_gpio_1(&self) -> bool {
        *self == Source::Gpio1
    }
    #[doc = "GPIO 2"]
    #[inline(always)]
    pub fn is_gpio_2(&self) -> bool {
        *self == Source::Gpio2
    }
    #[doc = "GPIO 3"]
    #[inline(always)]
    pub fn is_gpio_3(&self) -> bool {
        *self == Source::Gpio3
    }
    #[doc = "OR of all I2C"]
    #[inline(always)]
    pub fn is_i2c(&self) -> bool {
        *self == Source::I2c
    }
    #[doc = "OR of all SPI"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == Source::Spi
    }
    #[doc = "PCM/I2S"]
    #[inline(always)]
    pub fn is_pcm_i2s(&self) -> bool {
        *self == Source::PcmI2s
    }
    #[doc = "SDHOST"]
    #[inline(always)]
    pub fn is_sdhost(&self) -> bool {
        *self == Source::Sdhost
    }
    #[doc = "OR of all PL011 UARTs"]
    #[inline(always)]
    pub fn is_uart(&self) -> bool {
        *self == Source::Uart
    }
    #[doc = "OR of all ETH_PCIe L2"]
    #[inline(always)]
    pub fn is_eth_pcie(&self) -> bool {
        *self == Source::EthPcie
    }
    #[doc = "VEC"]
    #[inline(always)]
    pub fn is_vec(&self) -> bool {
        *self == Source::Vec
    }
    #[doc = "CPG"]
    #[inline(always)]
    pub fn is_cpg(&self) -> bool {
        *self == Source::Cpg
    }
    #[doc = "RNG"]
    #[inline(always)]
    pub fn is_rng(&self) -> bool {
        *self == Source::Rng
    }
    #[doc = "OR of EMMC and EMMC2"]
    #[inline(always)]
    pub fn is_emmc(&self) -> bool {
        *self == Source::Emmc
    }
    #[doc = "ETH_PCIe secure"]
    #[inline(always)]
    pub fn is_eth_pcie_secure(&self) -> bool {
        *self == Source::EthPcieSecure
    }
    #[doc = "ARMC Timer"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == Source::Timer
    }
    #[doc = "Mailbox"]
    #[inline(always)]
    pub fn is_mailbox(&self) -> bool {
        *self == Source::Mailbox
    }
    #[doc = "Doorbell 0"]
    #[inline(always)]
    pub fn is_doorbell0(&self) -> bool {
        *self == Source::Doorbell0
    }
    #[doc = "Doorbell 1"]
    #[inline(always)]
    pub fn is_doorbell1(&self) -> bool {
        *self == Source::Doorbell1
    }
    #[doc = "VPU0 halted"]
    #[inline(always)]
    pub fn is_vpu0_halted(&self) -> bool {
        *self == Source::Vpu0Halted
    }
    #[doc = "VPU1 halted"]
    #[inline(always)]
    pub fn is_vpu1_halted(&self) -> bool {
        *self == Source::Vpu1Halted
    }
    #[doc = "ARM address error"]
    #[inline(always)]
    pub fn is_arm_address_error(&self) -> bool {
        *self == Source::ArmAddressError
    }
    #[doc = "ARM AXI error"]
    #[inline(always)]
    pub fn is_arm_axi_error(&self) -> bool {
        *self == Source::ArmAxiError
    }
}
#[doc = "Field `SOURCE` writer - FIQ Source"]
pub type SourceW<'a, REG> = crate::FieldWriter<'a, REG, 7, Source>;
impl<'a, REG> SourceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer 0"]
    #[inline(always)]
    pub fn timer_0(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Timer0)
    }
    #[doc = "Timer 1"]
    #[inline(always)]
    pub fn timer_1(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Timer1)
    }
    #[doc = "Timer 2"]
    #[inline(always)]
    pub fn timer_2(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Timer2)
    }
    #[doc = "Timer 3"]
    #[inline(always)]
    pub fn timer_3(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Timer3)
    }
    #[doc = "H264 0"]
    #[inline(always)]
    pub fn h264_0(self) -> &'a mut crate::W<REG> {
        self.variant(Source::H264_0)
    }
    #[doc = "H264 1"]
    #[inline(always)]
    pub fn h264_1(self) -> &'a mut crate::W<REG> {
        self.variant(Source::H264_1)
    }
    #[doc = "H264 2"]
    #[inline(always)]
    pub fn h264_2(self) -> &'a mut crate::W<REG> {
        self.variant(Source::H264_2)
    }
    #[doc = "JPEG"]
    #[inline(always)]
    pub fn jpeg(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Jpeg)
    }
    #[doc = "ISP"]
    #[inline(always)]
    pub fn isp(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Isp)
    }
    #[doc = "USB"]
    #[inline(always)]
    pub fn usb(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Usb)
    }
    #[doc = "V3D"]
    #[inline(always)]
    pub fn v3d(self) -> &'a mut crate::W<REG> {
        self.variant(Source::V3d)
    }
    #[doc = "Transposer"]
    #[inline(always)]
    pub fn transposer(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Transposer)
    }
    #[doc = "Multicore Sync 0"]
    #[inline(always)]
    pub fn multicore_sync_0(self) -> &'a mut crate::W<REG> {
        self.variant(Source::MulticoreSync0)
    }
    #[doc = "Multicore Sync 1"]
    #[inline(always)]
    pub fn multicore_sync_1(self) -> &'a mut crate::W<REG> {
        self.variant(Source::MulticoreSync1)
    }
    #[doc = "Multicore Sync 2"]
    #[inline(always)]
    pub fn multicore_sync_2(self) -> &'a mut crate::W<REG> {
        self.variant(Source::MulticoreSync2)
    }
    #[doc = "Multicore Sync 3"]
    #[inline(always)]
    pub fn multicore_sync_3(self) -> &'a mut crate::W<REG> {
        self.variant(Source::MulticoreSync3)
    }
    #[doc = "DMA 0"]
    #[inline(always)]
    pub fn dma_0(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Dma0)
    }
    #[doc = "DMA 1"]
    #[inline(always)]
    pub fn dma_1(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Dma1)
    }
    #[doc = "DMA 2"]
    #[inline(always)]
    pub fn dma_2(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Dma2)
    }
    #[doc = "DMA 3"]
    #[inline(always)]
    pub fn dma_3(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Dma3)
    }
    #[doc = "DMA 4"]
    #[inline(always)]
    pub fn dma_4(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Dma4)
    }
    #[doc = "DMA 5"]
    #[inline(always)]
    pub fn dma_5(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Dma5)
    }
    #[doc = "DMA 6"]
    #[inline(always)]
    pub fn dma_6(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Dma6)
    }
    #[doc = "OR of DMA 7 and 8"]
    #[inline(always)]
    pub fn dma_7_8(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Dma7_8)
    }
    #[doc = "OR of DMA 9 and 10"]
    #[inline(always)]
    pub fn dma_9_10(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Dma9_10)
    }
    #[doc = "DMA 11"]
    #[inline(always)]
    pub fn dma_11(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Dma11)
    }
    #[doc = "DMA 12"]
    #[inline(always)]
    pub fn dma_12(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Dma12)
    }
    #[doc = "DMA 13"]
    #[inline(always)]
    pub fn dma_13(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Dma13)
    }
    #[doc = "DMA 14"]
    #[inline(always)]
    pub fn dma_14(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Dma14)
    }
    #[doc = "OR of UART1, SPI1 and SPI2"]
    #[inline(always)]
    pub fn aux(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Aux)
    }
    #[doc = "ARM"]
    #[inline(always)]
    pub fn arm(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Arm)
    }
    #[doc = "DMA 15"]
    #[inline(always)]
    pub fn dma_15(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Dma15)
    }
    #[doc = "HDMI CEC"]
    #[inline(always)]
    pub fn hdmi_cec(self) -> &'a mut crate::W<REG> {
        self.variant(Source::HdmiCec)
    }
    #[doc = "HVS"]
    #[inline(always)]
    pub fn hvs(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Hvs)
    }
    #[doc = "RPIVID"]
    #[inline(always)]
    pub fn rpivid(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Rpivid)
    }
    #[doc = "SDC"]
    #[inline(always)]
    pub fn sdc(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Sdc)
    }
    #[doc = "DSI 0"]
    #[inline(always)]
    pub fn dsi_0(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Dsi0)
    }
    #[doc = "Pixel Valve 2"]
    #[inline(always)]
    pub fn pixel_valve_2(self) -> &'a mut crate::W<REG> {
        self.variant(Source::PixelValve2)
    }
    #[doc = "Camera 0"]
    #[inline(always)]
    pub fn camera_0(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Camera0)
    }
    #[doc = "Camera 1"]
    #[inline(always)]
    pub fn camera_1(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Camera1)
    }
    #[doc = "HDMI 0"]
    #[inline(always)]
    pub fn hdmi_0(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Hdmi0)
    }
    #[doc = "HDMI 1"]
    #[inline(always)]
    pub fn hdmi_1(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Hdmi1)
    }
    #[doc = "Pixel Valve 3"]
    #[inline(always)]
    pub fn pixel_valve_3(self) -> &'a mut crate::W<REG> {
        self.variant(Source::PixelValve3)
    }
    #[doc = "SPI/BSC Slave"]
    #[inline(always)]
    pub fn spi_bsc_slave(self) -> &'a mut crate::W<REG> {
        self.variant(Source::SpiBscSlave)
    }
    #[doc = "DSI 1"]
    #[inline(always)]
    pub fn dsi_1(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Dsi1)
    }
    #[doc = "Pixel Valve 0"]
    #[inline(always)]
    pub fn pixel_valve_0(self) -> &'a mut crate::W<REG> {
        self.variant(Source::PixelValve0)
    }
    #[doc = "OR of Pixel Valve 1 and 2"]
    #[inline(always)]
    pub fn pixel_valve_1_2(self) -> &'a mut crate::W<REG> {
        self.variant(Source::PixelValve1_2)
    }
    #[doc = "CPR"]
    #[inline(always)]
    pub fn cpr(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Cpr)
    }
    #[doc = "SMI"]
    #[inline(always)]
    pub fn smi(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Smi)
    }
    #[doc = "GPIO 0"]
    #[inline(always)]
    pub fn gpio_0(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Gpio0)
    }
    #[doc = "GPIO 1"]
    #[inline(always)]
    pub fn gpio_1(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Gpio1)
    }
    #[doc = "GPIO 2"]
    #[inline(always)]
    pub fn gpio_2(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Gpio2)
    }
    #[doc = "GPIO 3"]
    #[inline(always)]
    pub fn gpio_3(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Gpio3)
    }
    #[doc = "OR of all I2C"]
    #[inline(always)]
    pub fn i2c(self) -> &'a mut crate::W<REG> {
        self.variant(Source::I2c)
    }
    #[doc = "OR of all SPI"]
    #[inline(always)]
    pub fn spi(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Spi)
    }
    #[doc = "PCM/I2S"]
    #[inline(always)]
    pub fn pcm_i2s(self) -> &'a mut crate::W<REG> {
        self.variant(Source::PcmI2s)
    }
    #[doc = "SDHOST"]
    #[inline(always)]
    pub fn sdhost(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Sdhost)
    }
    #[doc = "OR of all PL011 UARTs"]
    #[inline(always)]
    pub fn uart(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Uart)
    }
    #[doc = "OR of all ETH_PCIe L2"]
    #[inline(always)]
    pub fn eth_pcie(self) -> &'a mut crate::W<REG> {
        self.variant(Source::EthPcie)
    }
    #[doc = "VEC"]
    #[inline(always)]
    pub fn vec(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Vec)
    }
    #[doc = "CPG"]
    #[inline(always)]
    pub fn cpg(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Cpg)
    }
    #[doc = "RNG"]
    #[inline(always)]
    pub fn rng(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Rng)
    }
    #[doc = "OR of EMMC and EMMC2"]
    #[inline(always)]
    pub fn emmc(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Emmc)
    }
    #[doc = "ETH_PCIe secure"]
    #[inline(always)]
    pub fn eth_pcie_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Source::EthPcieSecure)
    }
    #[doc = "ARMC Timer"]
    #[inline(always)]
    pub fn timer(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Timer)
    }
    #[doc = "Mailbox"]
    #[inline(always)]
    pub fn mailbox(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Mailbox)
    }
    #[doc = "Doorbell 0"]
    #[inline(always)]
    pub fn doorbell0(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Doorbell0)
    }
    #[doc = "Doorbell 1"]
    #[inline(always)]
    pub fn doorbell1(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Doorbell1)
    }
    #[doc = "VPU0 halted"]
    #[inline(always)]
    pub fn vpu0_halted(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Vpu0Halted)
    }
    #[doc = "VPU1 halted"]
    #[inline(always)]
    pub fn vpu1_halted(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Vpu1Halted)
    }
    #[doc = "ARM address error"]
    #[inline(always)]
    pub fn arm_address_error(self) -> &'a mut crate::W<REG> {
        self.variant(Source::ArmAddressError)
    }
    #[doc = "ARM AXI error"]
    #[inline(always)]
    pub fn arm_axi_error(self) -> &'a mut crate::W<REG> {
        self.variant(Source::ArmAxiError)
    }
}
#[doc = "Field `ENABLE` reader - FIQ Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - FIQ Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - FIQ Source"]
    #[inline(always)]
    pub fn source(&self) -> SourceR {
        SourceR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - FIQ Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIQ_CONTROL")
            .field("enable", &self.enable())
            .field("source", &self.source())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - FIQ Source"]
    #[inline(always)]
    #[must_use]
    pub fn source(&mut self) -> SourceW<FiqControlSpec> {
        SourceW::new(self, 0)
    }
    #[doc = "Bit 7 - FIQ Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<FiqControlSpec> {
        EnableW::new(self, 7)
    }
}
#[doc = "FIQ control\n\nYou can [`read`](crate::Reg::read) this register and get [`fiq_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiq_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FiqControlSpec;
impl crate::RegisterSpec for FiqControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fiq_control::R`](R) reader structure"]
impl crate::Readable for FiqControlSpec {}
#[doc = "`write(|w| ..)` method takes [`fiq_control::W`](W) writer structure"]
impl crate::Writable for FiqControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIQ_CONTROL to value 0"]
impl crate::Resettable for FiqControlSpec {
    const RESET_VALUE: u32 = 0;
}

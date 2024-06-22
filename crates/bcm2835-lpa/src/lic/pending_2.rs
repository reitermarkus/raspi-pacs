#[doc = "Register `PENDING_2` reader"]
pub type R = crate::R<Pending2Spec>;
#[doc = "Field `HDMI_CEC` reader - HDMI CEC"]
pub type HdmiCecR = crate::BitReader;
#[doc = "Field `HVS` reader - HVS"]
pub type HvsR = crate::BitReader;
#[doc = "Field `RPIVID` reader - RPIVID"]
pub type RpividR = crate::BitReader;
#[doc = "Field `SDC` reader - SDC"]
pub type SdcR = crate::BitReader;
#[doc = "Field `DSI_0` reader - DSI 0"]
pub type Dsi0R = crate::BitReader;
#[doc = "Field `PIXEL_VALVE_2` reader - Pixel Valve 2"]
pub type PixelValve2R = crate::BitReader;
#[doc = "Field `CAMERA_0` reader - Camera 0"]
pub type Camera0R = crate::BitReader;
#[doc = "Field `CAMERA_1` reader - Camera 1"]
pub type Camera1R = crate::BitReader;
#[doc = "Field `HDMI_0` reader - HDMI 0"]
pub type Hdmi0R = crate::BitReader;
#[doc = "Field `HDMI_1` reader - HDMI 1"]
pub type Hdmi1R = crate::BitReader;
#[doc = "Field `PIXEL_VALVE_3` reader - Pixel Valve 3"]
pub type PixelValve3R = crate::BitReader;
#[doc = "Field `SPI_BSC_SLAVE` reader - SPI/BSC Slave"]
pub type SpiBscSlaveR = crate::BitReader;
#[doc = "Field `DSI_1` reader - DSI 1"]
pub type Dsi1R = crate::BitReader;
#[doc = "Field `PIXEL_VALVE_0` reader - Pixel Valve 0"]
pub type PixelValve0R = crate::BitReader;
#[doc = "Field `PIXEL_VALVE_1_2` reader - OR of Pixel Valve 1 and 2"]
pub type PixelValve1_2R = crate::BitReader;
#[doc = "Field `CPR` reader - CPR"]
pub type CprR = crate::BitReader;
#[doc = "Field `SMI` reader - SMI"]
pub type SmiR = crate::BitReader;
#[doc = "Field `GPIO_0` reader - GPIO 0"]
pub type Gpio0R = crate::BitReader;
#[doc = "Field `GPIO_1` reader - GPIO 1"]
pub type Gpio1R = crate::BitReader;
#[doc = "Field `GPIO_2` reader - GPIO 2"]
pub type Gpio2R = crate::BitReader;
#[doc = "Field `GPIO_3` reader - GPIO 3"]
pub type Gpio3R = crate::BitReader;
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
#[doc = "Field `ETH_PCIE` reader - OR of all ETH_PCIe L2"]
pub type EthPcieR = crate::BitReader;
#[doc = "Field `VEC` reader - VEC"]
pub type VecR = crate::BitReader;
#[doc = "Field `CPG` reader - CPG"]
pub type CpgR = crate::BitReader;
#[doc = "Field `RNG` reader - RNG"]
pub type RngR = crate::BitReader;
#[doc = "Field `EMMC` reader - OR of EMMC and EMMC2"]
pub type EmmcR = crate::BitReader;
#[doc = "Field `ETH_PCIE_SECURE` reader - ETH_PCIe secure"]
pub type EthPcieSecureR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - HDMI CEC"]
    #[inline(always)]
    pub fn hdmi_cec(&self) -> HdmiCecR {
        HdmiCecR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HVS"]
    #[inline(always)]
    pub fn hvs(&self) -> HvsR {
        HvsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RPIVID"]
    #[inline(always)]
    pub fn rpivid(&self) -> RpividR {
        RpividR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SDC"]
    #[inline(always)]
    pub fn sdc(&self) -> SdcR {
        SdcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DSI 0"]
    #[inline(always)]
    pub fn dsi_0(&self) -> Dsi0R {
        Dsi0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pixel Valve 2"]
    #[inline(always)]
    pub fn pixel_valve_2(&self) -> PixelValve2R {
        PixelValve2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Camera 0"]
    #[inline(always)]
    pub fn camera_0(&self) -> Camera0R {
        Camera0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Camera 1"]
    #[inline(always)]
    pub fn camera_1(&self) -> Camera1R {
        Camera1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - HDMI 0"]
    #[inline(always)]
    pub fn hdmi_0(&self) -> Hdmi0R {
        Hdmi0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HDMI 1"]
    #[inline(always)]
    pub fn hdmi_1(&self) -> Hdmi1R {
        Hdmi1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pixel Valve 3"]
    #[inline(always)]
    pub fn pixel_valve_3(&self) -> PixelValve3R {
        PixelValve3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SPI/BSC Slave"]
    #[inline(always)]
    pub fn spi_bsc_slave(&self) -> SpiBscSlaveR {
        SpiBscSlaveR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DSI 1"]
    #[inline(always)]
    pub fn dsi_1(&self) -> Dsi1R {
        Dsi1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pixel Valve 0"]
    #[inline(always)]
    pub fn pixel_valve_0(&self) -> PixelValve0R {
        PixelValve0R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - OR of Pixel Valve 1 and 2"]
    #[inline(always)]
    pub fn pixel_valve_1_2(&self) -> PixelValve1_2R {
        PixelValve1_2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CPR"]
    #[inline(always)]
    pub fn cpr(&self) -> CprR {
        CprR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SMI"]
    #[inline(always)]
    pub fn smi(&self) -> SmiR {
        SmiR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - GPIO 0"]
    #[inline(always)]
    pub fn gpio_0(&self) -> Gpio0R {
        Gpio0R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GPIO 1"]
    #[inline(always)]
    pub fn gpio_1(&self) -> Gpio1R {
        Gpio1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GPIO 2"]
    #[inline(always)]
    pub fn gpio_2(&self) -> Gpio2R {
        Gpio2R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPIO 3"]
    #[inline(always)]
    pub fn gpio_3(&self) -> Gpio3R {
        Gpio3R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - OR of all I2C"]
    #[inline(always)]
    pub fn i2c(&self) -> I2cR {
        I2cR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - OR of all SPI"]
    #[inline(always)]
    pub fn spi(&self) -> SpiR {
        SpiR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PCM/I2S"]
    #[inline(always)]
    pub fn pcm_i2s(&self) -> PcmI2sR {
        PcmI2sR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - SDHOST"]
    #[inline(always)]
    pub fn sdhost(&self) -> SdhostR {
        SdhostR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - OR of all PL011 UARTs"]
    #[inline(always)]
    pub fn uart(&self) -> UartR {
        UartR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - OR of all ETH_PCIe L2"]
    #[inline(always)]
    pub fn eth_pcie(&self) -> EthPcieR {
        EthPcieR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - VEC"]
    #[inline(always)]
    pub fn vec(&self) -> VecR {
        VecR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - CPG"]
    #[inline(always)]
    pub fn cpg(&self) -> CpgR {
        CpgR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - RNG"]
    #[inline(always)]
    pub fn rng(&self) -> RngR {
        RngR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - OR of EMMC and EMMC2"]
    #[inline(always)]
    pub fn emmc(&self) -> EmmcR {
        EmmcR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ETH_PCIe secure"]
    #[inline(always)]
    pub fn eth_pcie_secure(&self) -> EthPcieSecureR {
        EthPcieSecureR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PENDING_2")
            .field("hdmi_cec", &self.hdmi_cec())
            .field("hvs", &self.hvs())
            .field("rpivid", &self.rpivid())
            .field("sdc", &self.sdc())
            .field("dsi_0", &self.dsi_0())
            .field("pixel_valve_2", &self.pixel_valve_2())
            .field("camera_0", &self.camera_0())
            .field("camera_1", &self.camera_1())
            .field("hdmi_0", &self.hdmi_0())
            .field("hdmi_1", &self.hdmi_1())
            .field("pixel_valve_3", &self.pixel_valve_3())
            .field("spi_bsc_slave", &self.spi_bsc_slave())
            .field("dsi_1", &self.dsi_1())
            .field("pixel_valve_0", &self.pixel_valve_0())
            .field("pixel_valve_1_2", &self.pixel_valve_1_2())
            .field("cpr", &self.cpr())
            .field("smi", &self.smi())
            .field("gpio_0", &self.gpio_0())
            .field("gpio_1", &self.gpio_1())
            .field("gpio_2", &self.gpio_2())
            .field("gpio_3", &self.gpio_3())
            .field("i2c", &self.i2c())
            .field("spi", &self.spi())
            .field("pcm_i2s", &self.pcm_i2s())
            .field("sdhost", &self.sdhost())
            .field("uart", &self.uart())
            .field("eth_pcie", &self.eth_pcie())
            .field("vec", &self.vec())
            .field("cpg", &self.cpg())
            .field("rng", &self.rng())
            .field("emmc", &self.emmc())
            .field("eth_pcie_secure", &self.eth_pcie_secure())
            .finish()
    }
}
#[doc = "Pending state for interrupts 32 - 63\n\nYou can [`read`](crate::Reg::read) this register and get [`pending_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pending2Spec;
impl crate::RegisterSpec for Pending2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pending_2::R`](R) reader structure"]
impl crate::Readable for Pending2Spec {}
#[doc = "`reset()` method sets PENDING_2 to value 0"]
impl crate::Resettable for Pending2Spec {
    const RESET_VALUE: u32 = 0;
}

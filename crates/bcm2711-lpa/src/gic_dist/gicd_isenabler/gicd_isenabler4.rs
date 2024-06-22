#[doc = "Register `GICD_ISENABLER4` reader"]
pub type R = crate::R<GicdIsenabler4Spec>;
#[doc = "Register `GICD_ISENABLER4` writer"]
pub type W = crate::W<GicdIsenabler4Spec>;
#[doc = "Field `HDMI_CEC` reader - HDMI CEC"]
pub type HdmiCecR = crate::BitReader;
#[doc = "Field `HDMI_CEC` writer - HDMI CEC"]
pub type HdmiCecW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `HVS` reader - HVS"]
pub type HvsR = crate::BitReader;
#[doc = "Field `HVS` writer - HVS"]
pub type HvsW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `RPIVID` reader - RPIVID"]
pub type RpividR = crate::BitReader;
#[doc = "Field `RPIVID` writer - RPIVID"]
pub type RpividW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SDC` reader - SDC"]
pub type SdcR = crate::BitReader;
#[doc = "Field `SDC` writer - SDC"]
pub type SdcW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `DSI_0` reader - DSI 0"]
pub type Dsi0R = crate::BitReader;
#[doc = "Field `DSI_0` writer - DSI 0"]
pub type Dsi0W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `PIXEL_VALVE_2` reader - Pixel Valve 2"]
pub type PixelValve2R = crate::BitReader;
#[doc = "Field `PIXEL_VALVE_2` writer - Pixel Valve 2"]
pub type PixelValve2W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `CAMERA_0` reader - Camera 0"]
pub type Camera0R = crate::BitReader;
#[doc = "Field `CAMERA_0` writer - Camera 0"]
pub type Camera0W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `CAMERA_1` reader - Camera 1"]
pub type Camera1R = crate::BitReader;
#[doc = "Field `CAMERA_1` writer - Camera 1"]
pub type Camera1W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `HDMI_0` reader - HDMI 0"]
pub type Hdmi0R = crate::BitReader;
#[doc = "Field `HDMI_0` writer - HDMI 0"]
pub type Hdmi0W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `HDMI_1` reader - HDMI 1"]
pub type Hdmi1R = crate::BitReader;
#[doc = "Field `HDMI_1` writer - HDMI 1"]
pub type Hdmi1W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `PIXEL_VALVE_3` reader - Pixel Valve 3"]
pub type PixelValve3R = crate::BitReader;
#[doc = "Field `PIXEL_VALVE_3` writer - Pixel Valve 3"]
pub type PixelValve3W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SPI_BSC_SLAVE` reader - SPI/BSC Slave"]
pub type SpiBscSlaveR = crate::BitReader;
#[doc = "Field `SPI_BSC_SLAVE` writer - SPI/BSC Slave"]
pub type SpiBscSlaveW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `DSI_1` reader - DSI 1"]
pub type Dsi1R = crate::BitReader;
#[doc = "Field `DSI_1` writer - DSI 1"]
pub type Dsi1W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `PIXEL_VALVE_0` reader - Pixel Valve 0"]
pub type PixelValve0R = crate::BitReader;
#[doc = "Field `PIXEL_VALVE_0` writer - Pixel Valve 0"]
pub type PixelValve0W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `PIXEL_VALVE_1_2` reader - OR of Pixel Valve 1 and 2"]
pub type PixelValve1_2R = crate::BitReader;
#[doc = "Field `PIXEL_VALVE_1_2` writer - OR of Pixel Valve 1 and 2"]
pub type PixelValve1_2W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `CPR` reader - CPR"]
pub type CprR = crate::BitReader;
#[doc = "Field `CPR` writer - CPR"]
pub type CprW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SMI` reader - SMI"]
pub type SmiR = crate::BitReader;
#[doc = "Field `SMI` writer - SMI"]
pub type SmiW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `GPIO_0` reader - GPIO 0"]
pub type Gpio0R = crate::BitReader;
#[doc = "Field `GPIO_0` writer - GPIO 0"]
pub type Gpio0W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `GPIO_1` reader - GPIO 1"]
pub type Gpio1R = crate::BitReader;
#[doc = "Field `GPIO_1` writer - GPIO 1"]
pub type Gpio1W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `GPIO_2` reader - GPIO 2"]
pub type Gpio2R = crate::BitReader;
#[doc = "Field `GPIO_2` writer - GPIO 2"]
pub type Gpio2W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `GPIO_3` reader - GPIO 3"]
pub type Gpio3R = crate::BitReader;
#[doc = "Field `GPIO_3` writer - GPIO 3"]
pub type Gpio3W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `I2C` reader - OR of all I2C"]
pub type I2cR = crate::BitReader;
#[doc = "Field `I2C` writer - OR of all I2C"]
pub type I2cW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SPI` reader - OR of all SPI"]
pub type SpiR = crate::BitReader;
#[doc = "Field `SPI` writer - OR of all SPI"]
pub type SpiW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `PCM_I2S` reader - PCM/I2S"]
pub type PcmI2sR = crate::BitReader;
#[doc = "Field `PCM_I2S` writer - PCM/I2S"]
pub type PcmI2sW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SDHOST` reader - SDHOST"]
pub type SdhostR = crate::BitReader;
#[doc = "Field `SDHOST` writer - SDHOST"]
pub type SdhostW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `UART` reader - OR of all PL011 UARTs"]
pub type UartR = crate::BitReader;
#[doc = "Field `UART` writer - OR of all PL011 UARTs"]
pub type UartW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `ETH_PCIE` reader - OR of all ETH_PCIe L2"]
pub type EthPcieR = crate::BitReader;
#[doc = "Field `ETH_PCIE` writer - OR of all ETH_PCIe L2"]
pub type EthPcieW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `VEC` reader - VEC"]
pub type VecR = crate::BitReader;
#[doc = "Field `VEC` writer - VEC"]
pub type VecW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `CPG` reader - CPG"]
pub type CpgR = crate::BitReader;
#[doc = "Field `CPG` writer - CPG"]
pub type CpgW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `RNG` reader - RNG"]
pub type RngR = crate::BitReader;
#[doc = "Field `RNG` writer - RNG"]
pub type RngW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `EMMC` reader - OR of EMMC and EMMC2"]
pub type EmmcR = crate::BitReader;
#[doc = "Field `EMMC` writer - OR of EMMC and EMMC2"]
pub type EmmcW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `ETH_PCIE_SECURE` reader - ETH_PCIe secure"]
pub type EthPcieSecureR = crate::BitReader;
#[doc = "Field `ETH_PCIE_SECURE` writer - ETH_PCIe secure"]
pub type EthPcieSecureW<'a, REG> = crate::BitWriter1S<'a, REG>;
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
        f.debug_struct("GICD_ISENABLER4")
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
impl W {
    #[doc = "Bit 0 - HDMI CEC"]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_cec(&mut self) -> HdmiCecW<GicdIsenabler4Spec> {
        HdmiCecW::new(self, 0)
    }
    #[doc = "Bit 1 - HVS"]
    #[inline(always)]
    #[must_use]
    pub fn hvs(&mut self) -> HvsW<GicdIsenabler4Spec> {
        HvsW::new(self, 1)
    }
    #[doc = "Bit 2 - RPIVID"]
    #[inline(always)]
    #[must_use]
    pub fn rpivid(&mut self) -> RpividW<GicdIsenabler4Spec> {
        RpividW::new(self, 2)
    }
    #[doc = "Bit 3 - SDC"]
    #[inline(always)]
    #[must_use]
    pub fn sdc(&mut self) -> SdcW<GicdIsenabler4Spec> {
        SdcW::new(self, 3)
    }
    #[doc = "Bit 4 - DSI 0"]
    #[inline(always)]
    #[must_use]
    pub fn dsi_0(&mut self) -> Dsi0W<GicdIsenabler4Spec> {
        Dsi0W::new(self, 4)
    }
    #[doc = "Bit 5 - Pixel Valve 2"]
    #[inline(always)]
    #[must_use]
    pub fn pixel_valve_2(&mut self) -> PixelValve2W<GicdIsenabler4Spec> {
        PixelValve2W::new(self, 5)
    }
    #[doc = "Bit 6 - Camera 0"]
    #[inline(always)]
    #[must_use]
    pub fn camera_0(&mut self) -> Camera0W<GicdIsenabler4Spec> {
        Camera0W::new(self, 6)
    }
    #[doc = "Bit 7 - Camera 1"]
    #[inline(always)]
    #[must_use]
    pub fn camera_1(&mut self) -> Camera1W<GicdIsenabler4Spec> {
        Camera1W::new(self, 7)
    }
    #[doc = "Bit 8 - HDMI 0"]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_0(&mut self) -> Hdmi0W<GicdIsenabler4Spec> {
        Hdmi0W::new(self, 8)
    }
    #[doc = "Bit 9 - HDMI 1"]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_1(&mut self) -> Hdmi1W<GicdIsenabler4Spec> {
        Hdmi1W::new(self, 9)
    }
    #[doc = "Bit 10 - Pixel Valve 3"]
    #[inline(always)]
    #[must_use]
    pub fn pixel_valve_3(&mut self) -> PixelValve3W<GicdIsenabler4Spec> {
        PixelValve3W::new(self, 10)
    }
    #[doc = "Bit 11 - SPI/BSC Slave"]
    #[inline(always)]
    #[must_use]
    pub fn spi_bsc_slave(&mut self) -> SpiBscSlaveW<GicdIsenabler4Spec> {
        SpiBscSlaveW::new(self, 11)
    }
    #[doc = "Bit 12 - DSI 1"]
    #[inline(always)]
    #[must_use]
    pub fn dsi_1(&mut self) -> Dsi1W<GicdIsenabler4Spec> {
        Dsi1W::new(self, 12)
    }
    #[doc = "Bit 13 - Pixel Valve 0"]
    #[inline(always)]
    #[must_use]
    pub fn pixel_valve_0(&mut self) -> PixelValve0W<GicdIsenabler4Spec> {
        PixelValve0W::new(self, 13)
    }
    #[doc = "Bit 14 - OR of Pixel Valve 1 and 2"]
    #[inline(always)]
    #[must_use]
    pub fn pixel_valve_1_2(&mut self) -> PixelValve1_2W<GicdIsenabler4Spec> {
        PixelValve1_2W::new(self, 14)
    }
    #[doc = "Bit 15 - CPR"]
    #[inline(always)]
    #[must_use]
    pub fn cpr(&mut self) -> CprW<GicdIsenabler4Spec> {
        CprW::new(self, 15)
    }
    #[doc = "Bit 16 - SMI"]
    #[inline(always)]
    #[must_use]
    pub fn smi(&mut self) -> SmiW<GicdIsenabler4Spec> {
        SmiW::new(self, 16)
    }
    #[doc = "Bit 17 - GPIO 0"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_0(&mut self) -> Gpio0W<GicdIsenabler4Spec> {
        Gpio0W::new(self, 17)
    }
    #[doc = "Bit 18 - GPIO 1"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_1(&mut self) -> Gpio1W<GicdIsenabler4Spec> {
        Gpio1W::new(self, 18)
    }
    #[doc = "Bit 19 - GPIO 2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_2(&mut self) -> Gpio2W<GicdIsenabler4Spec> {
        Gpio2W::new(self, 19)
    }
    #[doc = "Bit 20 - GPIO 3"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_3(&mut self) -> Gpio3W<GicdIsenabler4Spec> {
        Gpio3W::new(self, 20)
    }
    #[doc = "Bit 21 - OR of all I2C"]
    #[inline(always)]
    #[must_use]
    pub fn i2c(&mut self) -> I2cW<GicdIsenabler4Spec> {
        I2cW::new(self, 21)
    }
    #[doc = "Bit 22 - OR of all SPI"]
    #[inline(always)]
    #[must_use]
    pub fn spi(&mut self) -> SpiW<GicdIsenabler4Spec> {
        SpiW::new(self, 22)
    }
    #[doc = "Bit 23 - PCM/I2S"]
    #[inline(always)]
    #[must_use]
    pub fn pcm_i2s(&mut self) -> PcmI2sW<GicdIsenabler4Spec> {
        PcmI2sW::new(self, 23)
    }
    #[doc = "Bit 24 - SDHOST"]
    #[inline(always)]
    #[must_use]
    pub fn sdhost(&mut self) -> SdhostW<GicdIsenabler4Spec> {
        SdhostW::new(self, 24)
    }
    #[doc = "Bit 25 - OR of all PL011 UARTs"]
    #[inline(always)]
    #[must_use]
    pub fn uart(&mut self) -> UartW<GicdIsenabler4Spec> {
        UartW::new(self, 25)
    }
    #[doc = "Bit 26 - OR of all ETH_PCIe L2"]
    #[inline(always)]
    #[must_use]
    pub fn eth_pcie(&mut self) -> EthPcieW<GicdIsenabler4Spec> {
        EthPcieW::new(self, 26)
    }
    #[doc = "Bit 27 - VEC"]
    #[inline(always)]
    #[must_use]
    pub fn vec(&mut self) -> VecW<GicdIsenabler4Spec> {
        VecW::new(self, 27)
    }
    #[doc = "Bit 28 - CPG"]
    #[inline(always)]
    #[must_use]
    pub fn cpg(&mut self) -> CpgW<GicdIsenabler4Spec> {
        CpgW::new(self, 28)
    }
    #[doc = "Bit 29 - RNG"]
    #[inline(always)]
    #[must_use]
    pub fn rng(&mut self) -> RngW<GicdIsenabler4Spec> {
        RngW::new(self, 29)
    }
    #[doc = "Bit 30 - OR of EMMC and EMMC2"]
    #[inline(always)]
    #[must_use]
    pub fn emmc(&mut self) -> EmmcW<GicdIsenabler4Spec> {
        EmmcW::new(self, 30)
    }
    #[doc = "Bit 31 - ETH_PCIe secure"]
    #[inline(always)]
    #[must_use]
    pub fn eth_pcie_secure(&mut self) -> EthPcieSecureW<GicdIsenabler4Spec> {
        EthPcieSecureW::new(self, 31)
    }
}
#[doc = "Interrupt Set-Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_isenabler4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_isenabler4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIsenabler4Spec;
impl crate::RegisterSpec for GicdIsenabler4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_isenabler4::R`](R) reader structure"]
impl crate::Readable for GicdIsenabler4Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_isenabler4::W`](W) writer structure"]
impl crate::Writable for GicdIsenabler4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff_ffff;
}
#[doc = "`reset()` method sets GICD_ISENABLER4 to value 0"]
impl crate::Resettable for GicdIsenabler4Spec {
    const RESET_VALUE: u32 = 0;
}

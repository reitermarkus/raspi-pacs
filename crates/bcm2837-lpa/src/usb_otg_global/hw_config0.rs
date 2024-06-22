#[doc = "Register `HW_CONFIG0` reader"]
pub type R = crate::R<HwConfig0Spec>;
#[doc = "Operating Mode"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OperatingMode {
    #[doc = "0: `0`"]
    HnpSrpCapable = 0,
    #[doc = "1: `1`"]
    SrpOnlyCapable = 1,
    #[doc = "2: `10`"]
    NoHnpSrpCapable = 2,
    #[doc = "3: `11`"]
    SrpCapableDevice = 3,
    #[doc = "4: `100`"]
    NoSrpCapableDevice = 4,
    #[doc = "5: `101`"]
    SrpCapableHost = 5,
    #[doc = "6: `110`"]
    NoSrpCapableHost = 6,
}
impl From<OperatingMode> for u8 {
    #[inline(always)]
    fn from(variant: OperatingMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OperatingMode {
    type Ux = u8;
}
impl crate::IsEnum for OperatingMode {}
#[doc = "Field `OPERATING_MODE` reader - Operating Mode"]
pub type OperatingModeR = crate::FieldReader<OperatingMode>;
impl OperatingModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OperatingMode> {
        match self.bits {
            0 => Some(OperatingMode::HnpSrpCapable),
            1 => Some(OperatingMode::SrpOnlyCapable),
            2 => Some(OperatingMode::NoHnpSrpCapable),
            3 => Some(OperatingMode::SrpCapableDevice),
            4 => Some(OperatingMode::NoSrpCapableDevice),
            5 => Some(OperatingMode::SrpCapableHost),
            6 => Some(OperatingMode::NoSrpCapableHost),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_hnp_srp_capable(&self) -> bool {
        *self == OperatingMode::HnpSrpCapable
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_srp_only_capable(&self) -> bool {
        *self == OperatingMode::SrpOnlyCapable
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_no_hnp_srp_capable(&self) -> bool {
        *self == OperatingMode::NoHnpSrpCapable
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_srp_capable_device(&self) -> bool {
        *self == OperatingMode::SrpCapableDevice
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_no_srp_capable_device(&self) -> bool {
        *self == OperatingMode::NoSrpCapableDevice
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_srp_capable_host(&self) -> bool {
        *self == OperatingMode::SrpCapableHost
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_no_srp_capable_host(&self) -> bool {
        *self == OperatingMode::NoSrpCapableHost
    }
}
#[doc = "Architecture"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Architecture {
    #[doc = "0: `0`"]
    SlaveOnly = 0,
    #[doc = "1: `1`"]
    ExternalDma = 1,
    #[doc = "2: `10`"]
    InternalDma = 2,
}
impl From<Architecture> for u8 {
    #[inline(always)]
    fn from(variant: Architecture) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Architecture {
    type Ux = u8;
}
impl crate::IsEnum for Architecture {}
#[doc = "Field `ARCHITECTURE` reader - Architecture"]
pub type ArchitectureR = crate::FieldReader<Architecture>;
impl ArchitectureR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Architecture> {
        match self.bits {
            0 => Some(Architecture::SlaveOnly),
            1 => Some(Architecture::ExternalDma),
            2 => Some(Architecture::InternalDma),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_slave_only(&self) -> bool {
        *self == Architecture::SlaveOnly
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_external_dma(&self) -> bool {
        *self == Architecture::ExternalDma
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_internal_dma(&self) -> bool {
        *self == Architecture::InternalDma
    }
}
#[doc = "Field `POINT_TO_POINT` reader - Point to Point"]
pub type PointToPointR = crate::BitReader;
#[doc = "High Speed Physical"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HighSpeedPhy {
    #[doc = "0: `0`"]
    NotSupported = 0,
    #[doc = "1: `1`"]
    Utmi = 1,
    #[doc = "2: `10`"]
    Ulpi = 2,
    #[doc = "3: `11`"]
    UtmiUlpi = 3,
}
impl From<HighSpeedPhy> for u8 {
    #[inline(always)]
    fn from(variant: HighSpeedPhy) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HighSpeedPhy {
    type Ux = u8;
}
impl crate::IsEnum for HighSpeedPhy {}
#[doc = "Field `HIGH_SPEED_PHY` reader - High Speed Physical"]
pub type HighSpeedPhyR = crate::FieldReader<HighSpeedPhy>;
impl HighSpeedPhyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HighSpeedPhy {
        match self.bits {
            0 => HighSpeedPhy::NotSupported,
            1 => HighSpeedPhy::Utmi,
            2 => HighSpeedPhy::Ulpi,
            3 => HighSpeedPhy::UtmiUlpi,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_not_supported(&self) -> bool {
        *self == HighSpeedPhy::NotSupported
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_utmi(&self) -> bool {
        *self == HighSpeedPhy::Utmi
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_ulpi(&self) -> bool {
        *self == HighSpeedPhy::Ulpi
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_utmi_ulpi(&self) -> bool {
        *self == HighSpeedPhy::UtmiUlpi
    }
}
#[doc = "Full Speed Physical"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FullSpeedPhy {
    #[doc = "0: `0`"]
    Phy0 = 0,
    #[doc = "1: `1`"]
    Dedicated = 1,
    #[doc = "2: `10`"]
    Phy2 = 2,
    #[doc = "3: `11`"]
    Phy3 = 3,
}
impl From<FullSpeedPhy> for u8 {
    #[inline(always)]
    fn from(variant: FullSpeedPhy) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FullSpeedPhy {
    type Ux = u8;
}
impl crate::IsEnum for FullSpeedPhy {}
#[doc = "Field `FULL_SPEED_PHY` reader - Full Speed Physical"]
pub type FullSpeedPhyR = crate::FieldReader<FullSpeedPhy>;
impl FullSpeedPhyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FullSpeedPhy {
        match self.bits {
            0 => FullSpeedPhy::Phy0,
            1 => FullSpeedPhy::Dedicated,
            2 => FullSpeedPhy::Phy2,
            3 => FullSpeedPhy::Phy3,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_phy0(&self) -> bool {
        *self == FullSpeedPhy::Phy0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_dedicated(&self) -> bool {
        *self == FullSpeedPhy::Dedicated
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_phy2(&self) -> bool {
        *self == FullSpeedPhy::Phy2
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_phy3(&self) -> bool {
        *self == FullSpeedPhy::Phy3
    }
}
#[doc = "Field `DEVICE_END_POINT_COUNT` reader - Device end point count"]
pub type DeviceEndPointCountR = crate::FieldReader;
#[doc = "Field `HOST_CHANNEL_COUNT` reader - Host channel count"]
pub type HostChannelCountR = crate::FieldReader;
#[doc = "Field `SUPPORTS_PERIODIC_ENDPOINTS` reader - Supports periodic endpoints"]
pub type SupportsPeriodicEndpointsR = crate::BitReader;
#[doc = "Field `DYNAMIC_FIFO` reader - Dynamic FIFO"]
pub type DynamicFifoR = crate::BitReader;
#[doc = "Field `MULTI_PROC_INT` reader - Multi proc int"]
pub type MultiProcIntR = crate::BitReader;
#[doc = "Field `NON_PERIODIC_QUEUE_DEPTH` reader - Non periodic queue depth"]
pub type NonPeriodicQueueDepthR = crate::FieldReader;
#[doc = "Field `HOST_PERIODIC_QUEUE_DEPTH` reader - Host periodic queue depth"]
pub type HostPeriodicQueueDepthR = crate::FieldReader;
#[doc = "Field `DEVICE_TOKEN_QUEUE_DEPTH` reader - Device token queue depth"]
pub type DeviceTokenQueueDepthR = crate::FieldReader;
#[doc = "Field `ENABLE_IC_USB` reader - Enable IC USB"]
pub type EnableIcUsbR = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Operating Mode"]
    #[inline(always)]
    pub fn operating_mode(&self) -> OperatingModeR {
        OperatingModeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Architecture"]
    #[inline(always)]
    pub fn architecture(&self) -> ArchitectureR {
        ArchitectureR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Point to Point"]
    #[inline(always)]
    pub fn point_to_point(&self) -> PointToPointR {
        PointToPointR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - High Speed Physical"]
    #[inline(always)]
    pub fn high_speed_phy(&self) -> HighSpeedPhyR {
        HighSpeedPhyR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Full Speed Physical"]
    #[inline(always)]
    pub fn full_speed_phy(&self) -> FullSpeedPhyR {
        FullSpeedPhyR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:13 - Device end point count"]
    #[inline(always)]
    pub fn device_end_point_count(&self) -> DeviceEndPointCountR {
        DeviceEndPointCountR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:17 - Host channel count"]
    #[inline(always)]
    pub fn host_channel_count(&self) -> HostChannelCountR {
        HostChannelCountR::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - Supports periodic endpoints"]
    #[inline(always)]
    pub fn supports_periodic_endpoints(&self) -> SupportsPeriodicEndpointsR {
        SupportsPeriodicEndpointsR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Dynamic FIFO"]
    #[inline(always)]
    pub fn dynamic_fifo(&self) -> DynamicFifoR {
        DynamicFifoR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Multi proc int"]
    #[inline(always)]
    pub fn multi_proc_int(&self) -> MultiProcIntR {
        MultiProcIntR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Non periodic queue depth"]
    #[inline(always)]
    pub fn non_periodic_queue_depth(&self) -> NonPeriodicQueueDepthR {
        NonPeriodicQueueDepthR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Host periodic queue depth"]
    #[inline(always)]
    pub fn host_periodic_queue_depth(&self) -> HostPeriodicQueueDepthR {
        HostPeriodicQueueDepthR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:30 - Device token queue depth"]
    #[inline(always)]
    pub fn device_token_queue_depth(&self) -> DeviceTokenQueueDepthR {
        DeviceTokenQueueDepthR::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Enable IC USB"]
    #[inline(always)]
    pub fn enable_ic_usb(&self) -> EnableIcUsbR {
        EnableIcUsbR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HW_CONFIG0")
            .field("operating_mode", &self.operating_mode())
            .field("architecture", &self.architecture())
            .field("point_to_point", &self.point_to_point())
            .field("high_speed_phy", &self.high_speed_phy())
            .field("full_speed_phy", &self.full_speed_phy())
            .field("device_end_point_count", &self.device_end_point_count())
            .field("host_channel_count", &self.host_channel_count())
            .field(
                "supports_periodic_endpoints",
                &self.supports_periodic_endpoints(),
            )
            .field("dynamic_fifo", &self.dynamic_fifo())
            .field("multi_proc_int", &self.multi_proc_int())
            .field("non_periodic_queue_depth", &self.non_periodic_queue_depth())
            .field(
                "host_periodic_queue_depth",
                &self.host_periodic_queue_depth(),
            )
            .field("device_token_queue_depth", &self.device_token_queue_depth())
            .field("enable_ic_usb", &self.enable_ic_usb())
            .finish()
    }
}
#[doc = "Hardware Config 0\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_config0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwConfig0Spec;
impl crate::RegisterSpec for HwConfig0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hw_config0::R`](R) reader structure"]
impl crate::Readable for HwConfig0Spec {}

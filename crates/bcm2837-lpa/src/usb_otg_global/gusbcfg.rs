#[doc = "Register `GUSBCFG` reader"]
pub type R = crate::R<GusbcfgSpec>;
#[doc = "Register `GUSBCFG` writer"]
pub type W = crate::W<GusbcfgSpec>;
#[doc = "Field `TOCAL` reader - FS timeout calibration"]
pub type TocalR = crate::FieldReader;
#[doc = "Field `TOCAL` writer - FS timeout calibration"]
pub type TocalW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "PHY Interface width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phyif {
    #[doc = "0: `0`"]
    _8bit = 0,
    #[doc = "1: `1`"]
    _16bit = 1,
}
impl From<Phyif> for bool {
    #[inline(always)]
    fn from(variant: Phyif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHYIF` reader - PHY Interface width"]
pub type PhyifR = crate::BitReader<Phyif>;
impl PhyifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Phyif {
        match self.bits {
            false => Phyif::_8bit,
            true => Phyif::_16bit,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == Phyif::_8bit
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == Phyif::_16bit
    }
}
#[doc = "Field `PHYIF` writer - PHY Interface width"]
pub type PhyifW<'a, REG> = crate::BitWriter<'a, REG, Phyif>;
impl<'a, REG> PhyifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Phyif::_8bit)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut crate::W<REG> {
        self.variant(Phyif::_16bit)
    }
}
#[doc = "PHY Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phytype {
    #[doc = "0: `0`"]
    Utmi = 0,
    #[doc = "1: `1`"]
    Ulpi = 1,
}
impl From<Phytype> for bool {
    #[inline(always)]
    fn from(variant: Phytype) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHYTYPE` reader - PHY Type"]
pub type PhytypeR = crate::BitReader<Phytype>;
impl PhytypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Phytype {
        match self.bits {
            false => Phytype::Utmi,
            true => Phytype::Ulpi,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_utmi(&self) -> bool {
        *self == Phytype::Utmi
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_ulpi(&self) -> bool {
        *self == Phytype::Ulpi
    }
}
#[doc = "Field `PHYTYPE` writer - PHY Type"]
pub type PhytypeW<'a, REG> = crate::BitWriter<'a, REG, Phytype>;
impl<'a, REG> PhytypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn utmi(self) -> &'a mut crate::W<REG> {
        self.variant(Phytype::Utmi)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ulpi(self) -> &'a mut crate::W<REG> {
        self.variant(Phytype::Ulpi)
    }
}
#[doc = "Full speed interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fsif {
    #[doc = "0: `0`"]
    _6pin = 0,
    #[doc = "1: `1`"]
    _3pin = 1,
}
impl From<Fsif> for bool {
    #[inline(always)]
    fn from(variant: Fsif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSIF` reader - Full speed interface"]
pub type FsifR = crate::BitReader<Fsif>;
impl FsifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsif {
        match self.bits {
            false => Fsif::_6pin,
            true => Fsif::_3pin,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_6pin(&self) -> bool {
        *self == Fsif::_6pin
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_3pin(&self) -> bool {
        *self == Fsif::_3pin
    }
}
#[doc = "Field `FSIF` writer - Full speed interface"]
pub type FsifW<'a, REG> = crate::BitWriter<'a, REG, Fsif>;
impl<'a, REG> FsifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _6pin(self) -> &'a mut crate::W<REG> {
        self.variant(Fsif::_6pin)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _3pin(self) -> &'a mut crate::W<REG> {
        self.variant(Fsif::_3pin)
    }
}
#[doc = "Transceiver select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Physel {
    #[doc = "0: `0`"]
    Usb20 = 0,
    #[doc = "1: `1`"]
    Usb11 = 1,
}
impl From<Physel> for bool {
    #[inline(always)]
    fn from(variant: Physel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHYSEL` reader - Transceiver select"]
pub type PhyselR = crate::BitReader<Physel>;
impl PhyselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Physel {
        match self.bits {
            false => Physel::Usb20,
            true => Physel::Usb11,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_usb20(&self) -> bool {
        *self == Physel::Usb20
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_usb11(&self) -> bool {
        *self == Physel::Usb11
    }
}
#[doc = "Field `PHYSEL` writer - Transceiver select"]
pub type PhyselW<'a, REG> = crate::BitWriter<'a, REG, Physel>;
impl<'a, REG> PhyselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn usb20(self) -> &'a mut crate::W<REG> {
        self.variant(Physel::Usb20)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn usb11(self) -> &'a mut crate::W<REG> {
        self.variant(Physel::Usb11)
    }
}
#[doc = "ULPI data rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ddrsel {
    #[doc = "0: `0`"]
    Single = 0,
    #[doc = "1: `1`"]
    Double = 1,
}
impl From<Ddrsel> for bool {
    #[inline(always)]
    fn from(variant: Ddrsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDRSEL` reader - ULPI data rate"]
pub type DdrselR = crate::BitReader<Ddrsel>;
impl DdrselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ddrsel {
        match self.bits {
            false => Ddrsel::Single,
            true => Ddrsel::Double,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Ddrsel::Single
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_double(&self) -> bool {
        *self == Ddrsel::Double
    }
}
#[doc = "Field `DDRSEL` writer - ULPI data rate"]
pub type DdrselW<'a, REG> = crate::BitWriter<'a, REG, Ddrsel>;
impl<'a, REG> DdrselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Ddrsel::Single)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn double(self) -> &'a mut crate::W<REG> {
        self.variant(Ddrsel::Double)
    }
}
#[doc = "Field `SRPCAP` reader - SRP-capable"]
pub type SrpcapR = crate::BitReader;
#[doc = "Field `SRPCAP` writer - SRP-capable"]
pub type SrpcapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HNPCAP` reader - HNP-capable"]
pub type HnpcapR = crate::BitReader;
#[doc = "Field `HNPCAP` writer - HNP-capable"]
pub type HnpcapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRDT` reader - USB turnaround time"]
pub type TrdtR = crate::FieldReader;
#[doc = "Field `TRDT` writer - USB turnaround time"]
pub type TrdtW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHYLPCS` reader - PHY Low-power clock select"]
pub type PhylpcsR = crate::BitReader;
#[doc = "Field `PHYLPCS` writer - PHY Low-power clock select"]
pub type PhylpcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULPIFSLS` reader - ULPI FS/LS select"]
pub type UlpifslsR = crate::BitReader;
#[doc = "Field `ULPIFSLS` writer - ULPI FS/LS select"]
pub type UlpifslsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULPIAR` reader - ULPI Auto-resume"]
pub type UlpiarR = crate::BitReader;
#[doc = "Field `ULPIAR` writer - ULPI Auto-resume"]
pub type UlpiarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULPICSM` reader - ULPI Clock SuspendM"]
pub type UlpicsmR = crate::BitReader;
#[doc = "Field `ULPICSM` writer - ULPI Clock SuspendM"]
pub type UlpicsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULPIEVBUSD` reader - ULPI External VBUS Drive"]
pub type UlpievbusdR = crate::BitReader;
#[doc = "Field `ULPIEVBUSD` writer - ULPI External VBUS Drive"]
pub type UlpievbusdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULPIEVBUSI` reader - ULPI external VBUS indicator"]
pub type UlpievbusiR = crate::BitReader;
#[doc = "Field `ULPIEVBUSI` writer - ULPI external VBUS indicator"]
pub type UlpievbusiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSDPS` reader - TermSel DLine pulsing selection"]
pub type TsdpsR = crate::BitReader;
#[doc = "Field `TSDPS` writer - TermSel DLine pulsing selection"]
pub type TsdpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCCI` reader - Indicator complement"]
pub type PcciR = crate::BitReader;
#[doc = "Field `PCCI` writer - Indicator complement"]
pub type PcciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTCI` reader - Indicator pass through"]
pub type PtciR = crate::BitReader;
#[doc = "Field `PTCI` writer - Indicator pass through"]
pub type PtciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULPIIPD` reader - ULPI interface protect disable"]
pub type UlpiipdR = crate::BitReader;
#[doc = "Field `ULPIIPD` writer - ULPI interface protect disable"]
pub type UlpiipdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FHMOD` reader - Forced host mode"]
pub type FhmodR = crate::BitReader;
#[doc = "Field `FHMOD` writer - Forced host mode"]
pub type FhmodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDMOD` reader - Forced peripheral mode"]
pub type FdmodR = crate::BitReader;
#[doc = "Field `FDMOD` writer - Forced peripheral mode"]
pub type FdmodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTXPKT` reader - Corrupt Tx packet"]
pub type CtxpktR = crate::BitReader;
#[doc = "Field `CTXPKT` writer - Corrupt Tx packet"]
pub type CtxpktW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - FS timeout calibration"]
    #[inline(always)]
    pub fn tocal(&self) -> TocalR {
        TocalR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - PHY Interface width"]
    #[inline(always)]
    pub fn phyif(&self) -> PhyifR {
        PhyifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PHY Type"]
    #[inline(always)]
    pub fn phytype(&self) -> PhytypeR {
        PhytypeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Full speed interface"]
    #[inline(always)]
    pub fn fsif(&self) -> FsifR {
        FsifR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transceiver select"]
    #[inline(always)]
    pub fn physel(&self) -> PhyselR {
        PhyselR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ULPI data rate"]
    #[inline(always)]
    pub fn ddrsel(&self) -> DdrselR {
        DdrselR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SRP-capable"]
    #[inline(always)]
    pub fn srpcap(&self) -> SrpcapR {
        SrpcapR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HNP-capable"]
    #[inline(always)]
    pub fn hnpcap(&self) -> HnpcapR {
        HnpcapR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - USB turnaround time"]
    #[inline(always)]
    pub fn trdt(&self) -> TrdtR {
        TrdtR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - PHY Low-power clock select"]
    #[inline(always)]
    pub fn phylpcs(&self) -> PhylpcsR {
        PhylpcsR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - ULPI FS/LS select"]
    #[inline(always)]
    pub fn ulpifsls(&self) -> UlpifslsR {
        UlpifslsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ULPI Auto-resume"]
    #[inline(always)]
    pub fn ulpiar(&self) -> UlpiarR {
        UlpiarR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ULPI Clock SuspendM"]
    #[inline(always)]
    pub fn ulpicsm(&self) -> UlpicsmR {
        UlpicsmR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ULPI External VBUS Drive"]
    #[inline(always)]
    pub fn ulpievbusd(&self) -> UlpievbusdR {
        UlpievbusdR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ULPI external VBUS indicator"]
    #[inline(always)]
    pub fn ulpievbusi(&self) -> UlpievbusiR {
        UlpievbusiR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - TermSel DLine pulsing selection"]
    #[inline(always)]
    pub fn tsdps(&self) -> TsdpsR {
        TsdpsR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Indicator complement"]
    #[inline(always)]
    pub fn pcci(&self) -> PcciR {
        PcciR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Indicator pass through"]
    #[inline(always)]
    pub fn ptci(&self) -> PtciR {
        PtciR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ULPI interface protect disable"]
    #[inline(always)]
    pub fn ulpiipd(&self) -> UlpiipdR {
        UlpiipdR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - Forced host mode"]
    #[inline(always)]
    pub fn fhmod(&self) -> FhmodR {
        FhmodR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Forced peripheral mode"]
    #[inline(always)]
    pub fn fdmod(&self) -> FdmodR {
        FdmodR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Corrupt Tx packet"]
    #[inline(always)]
    pub fn ctxpkt(&self) -> CtxpktR {
        CtxpktR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GUSBCFG")
            .field("tocal", &self.tocal())
            .field("phyif", &self.phyif())
            .field("phytype", &self.phytype())
            .field("fsif", &self.fsif())
            .field("physel", &self.physel())
            .field("ddrsel", &self.ddrsel())
            .field("srpcap", &self.srpcap())
            .field("hnpcap", &self.hnpcap())
            .field("trdt", &self.trdt())
            .field("phylpcs", &self.phylpcs())
            .field("ulpifsls", &self.ulpifsls())
            .field("ulpiar", &self.ulpiar())
            .field("ulpicsm", &self.ulpicsm())
            .field("ulpievbusd", &self.ulpievbusd())
            .field("ulpievbusi", &self.ulpievbusi())
            .field("tsdps", &self.tsdps())
            .field("pcci", &self.pcci())
            .field("ptci", &self.ptci())
            .field("ulpiipd", &self.ulpiipd())
            .field("fhmod", &self.fhmod())
            .field("fdmod", &self.fdmod())
            .field("ctxpkt", &self.ctxpkt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - FS timeout calibration"]
    #[inline(always)]
    #[must_use]
    pub fn tocal(&mut self) -> TocalW<GusbcfgSpec> {
        TocalW::new(self, 0)
    }
    #[doc = "Bit 3 - PHY Interface width"]
    #[inline(always)]
    #[must_use]
    pub fn phyif(&mut self) -> PhyifW<GusbcfgSpec> {
        PhyifW::new(self, 3)
    }
    #[doc = "Bit 4 - PHY Type"]
    #[inline(always)]
    #[must_use]
    pub fn phytype(&mut self) -> PhytypeW<GusbcfgSpec> {
        PhytypeW::new(self, 4)
    }
    #[doc = "Bit 5 - Full speed interface"]
    #[inline(always)]
    #[must_use]
    pub fn fsif(&mut self) -> FsifW<GusbcfgSpec> {
        FsifW::new(self, 5)
    }
    #[doc = "Bit 6 - Transceiver select"]
    #[inline(always)]
    #[must_use]
    pub fn physel(&mut self) -> PhyselW<GusbcfgSpec> {
        PhyselW::new(self, 6)
    }
    #[doc = "Bit 7 - ULPI data rate"]
    #[inline(always)]
    #[must_use]
    pub fn ddrsel(&mut self) -> DdrselW<GusbcfgSpec> {
        DdrselW::new(self, 7)
    }
    #[doc = "Bit 8 - SRP-capable"]
    #[inline(always)]
    #[must_use]
    pub fn srpcap(&mut self) -> SrpcapW<GusbcfgSpec> {
        SrpcapW::new(self, 8)
    }
    #[doc = "Bit 9 - HNP-capable"]
    #[inline(always)]
    #[must_use]
    pub fn hnpcap(&mut self) -> HnpcapW<GusbcfgSpec> {
        HnpcapW::new(self, 9)
    }
    #[doc = "Bits 10:13 - USB turnaround time"]
    #[inline(always)]
    #[must_use]
    pub fn trdt(&mut self) -> TrdtW<GusbcfgSpec> {
        TrdtW::new(self, 10)
    }
    #[doc = "Bit 15 - PHY Low-power clock select"]
    #[inline(always)]
    #[must_use]
    pub fn phylpcs(&mut self) -> PhylpcsW<GusbcfgSpec> {
        PhylpcsW::new(self, 15)
    }
    #[doc = "Bit 17 - ULPI FS/LS select"]
    #[inline(always)]
    #[must_use]
    pub fn ulpifsls(&mut self) -> UlpifslsW<GusbcfgSpec> {
        UlpifslsW::new(self, 17)
    }
    #[doc = "Bit 18 - ULPI Auto-resume"]
    #[inline(always)]
    #[must_use]
    pub fn ulpiar(&mut self) -> UlpiarW<GusbcfgSpec> {
        UlpiarW::new(self, 18)
    }
    #[doc = "Bit 19 - ULPI Clock SuspendM"]
    #[inline(always)]
    #[must_use]
    pub fn ulpicsm(&mut self) -> UlpicsmW<GusbcfgSpec> {
        UlpicsmW::new(self, 19)
    }
    #[doc = "Bit 20 - ULPI External VBUS Drive"]
    #[inline(always)]
    #[must_use]
    pub fn ulpievbusd(&mut self) -> UlpievbusdW<GusbcfgSpec> {
        UlpievbusdW::new(self, 20)
    }
    #[doc = "Bit 21 - ULPI external VBUS indicator"]
    #[inline(always)]
    #[must_use]
    pub fn ulpievbusi(&mut self) -> UlpievbusiW<GusbcfgSpec> {
        UlpievbusiW::new(self, 21)
    }
    #[doc = "Bit 22 - TermSel DLine pulsing selection"]
    #[inline(always)]
    #[must_use]
    pub fn tsdps(&mut self) -> TsdpsW<GusbcfgSpec> {
        TsdpsW::new(self, 22)
    }
    #[doc = "Bit 23 - Indicator complement"]
    #[inline(always)]
    #[must_use]
    pub fn pcci(&mut self) -> PcciW<GusbcfgSpec> {
        PcciW::new(self, 23)
    }
    #[doc = "Bit 24 - Indicator pass through"]
    #[inline(always)]
    #[must_use]
    pub fn ptci(&mut self) -> PtciW<GusbcfgSpec> {
        PtciW::new(self, 24)
    }
    #[doc = "Bit 25 - ULPI interface protect disable"]
    #[inline(always)]
    #[must_use]
    pub fn ulpiipd(&mut self) -> UlpiipdW<GusbcfgSpec> {
        UlpiipdW::new(self, 25)
    }
    #[doc = "Bit 29 - Forced host mode"]
    #[inline(always)]
    #[must_use]
    pub fn fhmod(&mut self) -> FhmodW<GusbcfgSpec> {
        FhmodW::new(self, 29)
    }
    #[doc = "Bit 30 - Forced peripheral mode"]
    #[inline(always)]
    #[must_use]
    pub fn fdmod(&mut self) -> FdmodW<GusbcfgSpec> {
        FdmodW::new(self, 30)
    }
    #[doc = "Bit 31 - Corrupt Tx packet"]
    #[inline(always)]
    #[must_use]
    pub fn ctxpkt(&mut self) -> CtxpktW<GusbcfgSpec> {
        CtxpktW::new(self, 31)
    }
}
#[doc = "OTG_HS USB configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`gusbcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gusbcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GusbcfgSpec;
impl crate::RegisterSpec for GusbcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gusbcfg::R`](R) reader structure"]
impl crate::Readable for GusbcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`gusbcfg::W`](W) writer structure"]
impl crate::Writable for GusbcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GUSBCFG to value 0x0a00"]
impl crate::Resettable for GusbcfgSpec {
    const RESET_VALUE: u32 = 0x0a00;
}

#[doc = "Register `HPRT` reader"]
pub type R = crate::R<HprtSpec>;
#[doc = "Register `HPRT` writer"]
pub type W = crate::W<HprtSpec>;
#[doc = "Field `PCSTS` reader - Port connect status"]
pub type PcstsR = crate::BitReader;
#[doc = "Field `PCDET` reader - Port connect detected"]
pub type PcdetR = crate::BitReader;
#[doc = "Field `PCDET` writer - Port connect detected"]
pub type PcdetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PENA` reader - Port enable"]
pub type PenaR = crate::BitReader;
#[doc = "Field `PENA` writer - Port enable"]
pub type PenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PENCHNG` reader - Port enable/disable change"]
pub type PenchngR = crate::BitReader;
#[doc = "Field `PENCHNG` writer - Port enable/disable change"]
pub type PenchngW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POCA` reader - Port overcurrent active"]
pub type PocaR = crate::BitReader;
#[doc = "Field `POCCHNG` reader - Port overcurrent change"]
pub type PocchngR = crate::BitReader;
#[doc = "Field `POCCHNG` writer - Port overcurrent change"]
pub type PocchngW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRES` reader - Port resume"]
pub type PresR = crate::BitReader;
#[doc = "Field `PRES` writer - Port resume"]
pub type PresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSUSP` reader - Port suspend"]
pub type PsuspR = crate::BitReader;
#[doc = "Field `PSUSP` writer - Port suspend"]
pub type PsuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRST` reader - Port reset"]
pub type PrstR = crate::BitReader;
#[doc = "Field `PRST` writer - Port reset"]
pub type PrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLSTS` reader - Port line status"]
pub type PlstsR = crate::FieldReader;
#[doc = "Field `PPWR` reader - Port power"]
pub type PpwrR = crate::BitReader;
#[doc = "Field `PPWR` writer - Port power"]
pub type PpwrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTCTL` reader - Port test control"]
pub type PtctlR = crate::FieldReader;
#[doc = "Field `PTCTL` writer - Port test control"]
pub type PtctlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PSPD` reader - Port speed"]
pub type PspdR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Port connect status"]
    #[inline(always)]
    pub fn pcsts(&self) -> PcstsR {
        PcstsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port connect detected"]
    #[inline(always)]
    pub fn pcdet(&self) -> PcdetR {
        PcdetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port enable"]
    #[inline(always)]
    pub fn pena(&self) -> PenaR {
        PenaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port enable/disable change"]
    #[inline(always)]
    pub fn penchng(&self) -> PenchngR {
        PenchngR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port overcurrent active"]
    #[inline(always)]
    pub fn poca(&self) -> PocaR {
        PocaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port overcurrent change"]
    #[inline(always)]
    pub fn pocchng(&self) -> PocchngR {
        PocchngR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port resume"]
    #[inline(always)]
    pub fn pres(&self) -> PresR {
        PresR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port suspend"]
    #[inline(always)]
    pub fn psusp(&self) -> PsuspR {
        PsuspR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port reset"]
    #[inline(always)]
    pub fn prst(&self) -> PrstR {
        PrstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Port line status"]
    #[inline(always)]
    pub fn plsts(&self) -> PlstsR {
        PlstsR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Port power"]
    #[inline(always)]
    pub fn ppwr(&self) -> PpwrR {
        PpwrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:16 - Port test control"]
    #[inline(always)]
    pub fn ptctl(&self) -> PtctlR {
        PtctlR::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:18 - Port speed"]
    #[inline(always)]
    pub fn pspd(&self) -> PspdR {
        PspdR::new(((self.bits >> 17) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPRT")
            .field("pcsts", &self.pcsts())
            .field("pcdet", &self.pcdet())
            .field("pena", &self.pena())
            .field("penchng", &self.penchng())
            .field("poca", &self.poca())
            .field("pocchng", &self.pocchng())
            .field("pres", &self.pres())
            .field("psusp", &self.psusp())
            .field("prst", &self.prst())
            .field("plsts", &self.plsts())
            .field("ppwr", &self.ppwr())
            .field("ptctl", &self.ptctl())
            .field("pspd", &self.pspd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Port connect detected"]
    #[inline(always)]
    #[must_use]
    pub fn pcdet(&mut self) -> PcdetW<HprtSpec> {
        PcdetW::new(self, 1)
    }
    #[doc = "Bit 2 - Port enable"]
    #[inline(always)]
    #[must_use]
    pub fn pena(&mut self) -> PenaW<HprtSpec> {
        PenaW::new(self, 2)
    }
    #[doc = "Bit 3 - Port enable/disable change"]
    #[inline(always)]
    #[must_use]
    pub fn penchng(&mut self) -> PenchngW<HprtSpec> {
        PenchngW::new(self, 3)
    }
    #[doc = "Bit 5 - Port overcurrent change"]
    #[inline(always)]
    #[must_use]
    pub fn pocchng(&mut self) -> PocchngW<HprtSpec> {
        PocchngW::new(self, 5)
    }
    #[doc = "Bit 6 - Port resume"]
    #[inline(always)]
    #[must_use]
    pub fn pres(&mut self) -> PresW<HprtSpec> {
        PresW::new(self, 6)
    }
    #[doc = "Bit 7 - Port suspend"]
    #[inline(always)]
    #[must_use]
    pub fn psusp(&mut self) -> PsuspW<HprtSpec> {
        PsuspW::new(self, 7)
    }
    #[doc = "Bit 8 - Port reset"]
    #[inline(always)]
    #[must_use]
    pub fn prst(&mut self) -> PrstW<HprtSpec> {
        PrstW::new(self, 8)
    }
    #[doc = "Bit 12 - Port power"]
    #[inline(always)]
    #[must_use]
    pub fn ppwr(&mut self) -> PpwrW<HprtSpec> {
        PpwrW::new(self, 12)
    }
    #[doc = "Bits 13:16 - Port test control"]
    #[inline(always)]
    #[must_use]
    pub fn ptctl(&mut self) -> PtctlW<HprtSpec> {
        PtctlW::new(self, 13)
    }
}
#[doc = "OTG_HS host port control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`hprt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hprt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HprtSpec;
impl crate::RegisterSpec for HprtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hprt::R`](R) reader structure"]
impl crate::Readable for HprtSpec {}
#[doc = "`write(|w| ..)` method takes [`hprt::W`](W) writer structure"]
impl crate::Writable for HprtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HPRT to value 0"]
impl crate::Resettable for HprtSpec {
    const RESET_VALUE: u32 = 0;
}

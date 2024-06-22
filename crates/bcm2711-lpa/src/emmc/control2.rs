#[doc = "Register `CONTROL2` reader"]
pub type R = crate::R<Control2Spec>;
#[doc = "Register `CONTROL2` writer"]
pub type W = crate::W<Control2Spec>;
#[doc = "Field `ACNOX_ERR` reader - Auto command not executed due to an error"]
pub type AcnoxErrR = crate::BitReader;
#[doc = "Field `ACTO_ERR` reader - Auto command timeout"]
pub type ActoErrR = crate::BitReader;
#[doc = "Field `ACCRC_ERR` reader - Command CRC error during auto command"]
pub type AccrcErrR = crate::BitReader;
#[doc = "Field `ACEND_ERR` reader - End bit is not 1 during auto command"]
pub type AcendErrR = crate::BitReader;
#[doc = "Field `ACBAD_ERR` reader - Command index error during auto command"]
pub type AcbadErrR = crate::BitReader;
#[doc = "Field `NOTC12_ERR` reader - Error during auto CMD12"]
pub type Notc12ErrR = crate::BitReader;
#[doc = "Select the speed of the SD card\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Uhsmode {
    #[doc = "0: `0`"]
    Sdr12 = 0,
    #[doc = "1: `1`"]
    Sdr25 = 1,
    #[doc = "2: `10`"]
    Sdr50 = 2,
    #[doc = "3: `11`"]
    Sdr104 = 3,
    #[doc = "4: `100`"]
    Ddr50 = 4,
}
impl From<Uhsmode> for u8 {
    #[inline(always)]
    fn from(variant: Uhsmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Uhsmode {
    type Ux = u8;
}
impl crate::IsEnum for Uhsmode {}
#[doc = "Field `UHSMODE` reader - Select the speed of the SD card"]
pub type UhsmodeR = crate::FieldReader<Uhsmode>;
impl UhsmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Uhsmode> {
        match self.bits {
            0 => Some(Uhsmode::Sdr12),
            1 => Some(Uhsmode::Sdr25),
            2 => Some(Uhsmode::Sdr50),
            3 => Some(Uhsmode::Sdr104),
            4 => Some(Uhsmode::Ddr50),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_sdr12(&self) -> bool {
        *self == Uhsmode::Sdr12
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_sdr25(&self) -> bool {
        *self == Uhsmode::Sdr25
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_sdr50(&self) -> bool {
        *self == Uhsmode::Sdr50
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_sdr104(&self) -> bool {
        *self == Uhsmode::Sdr104
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_ddr50(&self) -> bool {
        *self == Uhsmode::Ddr50
    }
}
#[doc = "Field `UHSMODE` writer - Select the speed of the SD card"]
pub type UhsmodeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Uhsmode>;
impl<'a, REG> UhsmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn sdr12(self) -> &'a mut crate::W<REG> {
        self.variant(Uhsmode::Sdr12)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sdr25(self) -> &'a mut crate::W<REG> {
        self.variant(Uhsmode::Sdr25)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sdr50(self) -> &'a mut crate::W<REG> {
        self.variant(Uhsmode::Sdr50)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn sdr104(self) -> &'a mut crate::W<REG> {
        self.variant(Uhsmode::Sdr104)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn ddr50(self) -> &'a mut crate::W<REG> {
        self.variant(Uhsmode::Ddr50)
    }
}
#[doc = "Field `TUNEON` reader - SD Clock tune in progress"]
pub type TuneonR = crate::BitReader;
#[doc = "Field `TUNEON` writer - SD Clock tune in progress"]
pub type TuneonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUNED` reader - Tuned clock is used for sampling data"]
pub type TunedR = crate::BitReader;
#[doc = "Field `TUNED` writer - Tuned clock is used for sampling data"]
pub type TunedW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Auto command not executed due to an error"]
    #[inline(always)]
    pub fn acnox_err(&self) -> AcnoxErrR {
        AcnoxErrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Auto command timeout"]
    #[inline(always)]
    pub fn acto_err(&self) -> ActoErrR {
        ActoErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command CRC error during auto command"]
    #[inline(always)]
    pub fn accrc_err(&self) -> AccrcErrR {
        AccrcErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End bit is not 1 during auto command"]
    #[inline(always)]
    pub fn acend_err(&self) -> AcendErrR {
        AcendErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Command index error during auto command"]
    #[inline(always)]
    pub fn acbad_err(&self) -> AcbadErrR {
        AcbadErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Error during auto CMD12"]
    #[inline(always)]
    pub fn notc12_err(&self) -> Notc12ErrR {
        Notc12ErrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Select the speed of the SD card"]
    #[inline(always)]
    pub fn uhsmode(&self) -> UhsmodeR {
        UhsmodeR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 22 - SD Clock tune in progress"]
    #[inline(always)]
    pub fn tuneon(&self) -> TuneonR {
        TuneonR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Tuned clock is used for sampling data"]
    #[inline(always)]
    pub fn tuned(&self) -> TunedR {
        TunedR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONTROL2")
            .field("tuned", &self.tuned())
            .field("tuneon", &self.tuneon())
            .field("uhsmode", &self.uhsmode())
            .field("notc12_err", &self.notc12_err())
            .field("acbad_err", &self.acbad_err())
            .field("acend_err", &self.acend_err())
            .field("accrc_err", &self.accrc_err())
            .field("acto_err", &self.acto_err())
            .field("acnox_err", &self.acnox_err())
            .finish()
    }
}
impl W {
    #[doc = "Bits 16:18 - Select the speed of the SD card"]
    #[inline(always)]
    #[must_use]
    pub fn uhsmode(&mut self) -> UhsmodeW<Control2Spec> {
        UhsmodeW::new(self, 16)
    }
    #[doc = "Bit 22 - SD Clock tune in progress"]
    #[inline(always)]
    #[must_use]
    pub fn tuneon(&mut self) -> TuneonW<Control2Spec> {
        TuneonW::new(self, 22)
    }
    #[doc = "Bit 23 - Tuned clock is used for sampling data"]
    #[inline(always)]
    #[must_use]
    pub fn tuned(&mut self) -> TunedW<Control2Spec> {
        TunedW::new(self, 23)
    }
}
#[doc = "Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`control2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Control2Spec;
impl crate::RegisterSpec for Control2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control2::R`](R) reader structure"]
impl crate::Readable for Control2Spec {}
#[doc = "`write(|w| ..)` method takes [`control2::W`](W) writer structure"]
impl crate::Writable for Control2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONTROL2 to value 0"]
impl crate::Resettable for Control2Spec {
    const RESET_VALUE: u32 = 0;
}

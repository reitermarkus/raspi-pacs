#[doc = "Register `GCCFG` reader"]
pub type R = crate::R<GccfgSpec>;
#[doc = "Register `GCCFG` writer"]
pub type W = crate::W<GccfgSpec>;
#[doc = "Field `PWRDWN` reader - Power down"]
pub type PwrdwnR = crate::BitReader;
#[doc = "Field `PWRDWN` writer - Power down"]
pub type PwrdwnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2CPADEN` reader - Enable I2C bus connection for the external I2C PHY interface"]
pub type I2cpadenR = crate::BitReader;
#[doc = "Field `I2CPADEN` writer - Enable I2C bus connection for the external I2C PHY interface"]
pub type I2cpadenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSASEN` reader - Enable the VBUS sensing device"]
pub type VbusasenR = crate::BitReader;
#[doc = "Field `VBUSASEN` writer - Enable the VBUS sensing device"]
pub type VbusasenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSBSEN` reader - Enable the VBUS sensing device"]
pub type VbusbsenR = crate::BitReader;
#[doc = "Field `VBUSBSEN` writer - Enable the VBUS sensing device"]
pub type VbusbsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFOUTEN` reader - SOF output enable"]
pub type SofoutenR = crate::BitReader;
#[doc = "Field `SOFOUTEN` writer - SOF output enable"]
pub type SofoutenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOVBUSSENS` reader - VBUS sensing disable option"]
pub type NovbussensR = crate::BitReader;
#[doc = "Field `NOVBUSSENS` writer - VBUS sensing disable option"]
pub type NovbussensW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - Power down"]
    #[inline(always)]
    pub fn pwrdwn(&self) -> PwrdwnR {
        PwrdwnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable I2C bus connection for the external I2C PHY interface"]
    #[inline(always)]
    pub fn i2cpaden(&self) -> I2cpadenR {
        I2cpadenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable the VBUS sensing device"]
    #[inline(always)]
    pub fn vbusasen(&self) -> VbusasenR {
        VbusasenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable the VBUS sensing device"]
    #[inline(always)]
    pub fn vbusbsen(&self) -> VbusbsenR {
        VbusbsenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SOF output enable"]
    #[inline(always)]
    pub fn sofouten(&self) -> SofoutenR {
        SofoutenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - VBUS sensing disable option"]
    #[inline(always)]
    pub fn novbussens(&self) -> NovbussensR {
        NovbussensR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GCCFG")
            .field("pwrdwn", &self.pwrdwn())
            .field("i2cpaden", &self.i2cpaden())
            .field("vbusasen", &self.vbusasen())
            .field("vbusbsen", &self.vbusbsen())
            .field("sofouten", &self.sofouten())
            .field("novbussens", &self.novbussens())
            .finish()
    }
}
impl W {
    #[doc = "Bit 16 - Power down"]
    #[inline(always)]
    #[must_use]
    pub fn pwrdwn(&mut self) -> PwrdwnW<GccfgSpec> {
        PwrdwnW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable I2C bus connection for the external I2C PHY interface"]
    #[inline(always)]
    #[must_use]
    pub fn i2cpaden(&mut self) -> I2cpadenW<GccfgSpec> {
        I2cpadenW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable the VBUS sensing device"]
    #[inline(always)]
    #[must_use]
    pub fn vbusasen(&mut self) -> VbusasenW<GccfgSpec> {
        VbusasenW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable the VBUS sensing device"]
    #[inline(always)]
    #[must_use]
    pub fn vbusbsen(&mut self) -> VbusbsenW<GccfgSpec> {
        VbusbsenW::new(self, 19)
    }
    #[doc = "Bit 20 - SOF output enable"]
    #[inline(always)]
    #[must_use]
    pub fn sofouten(&mut self) -> SofoutenW<GccfgSpec> {
        SofoutenW::new(self, 20)
    }
    #[doc = "Bit 21 - VBUS sensing disable option"]
    #[inline(always)]
    #[must_use]
    pub fn novbussens(&mut self) -> NovbussensW<GccfgSpec> {
        NovbussensW::new(self, 21)
    }
}
#[doc = "OTG_HS general core configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`gccfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gccfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GccfgSpec;
impl crate::RegisterSpec for GccfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gccfg::R`](R) reader structure"]
impl crate::Readable for GccfgSpec {}
#[doc = "`write(|w| ..)` method takes [`gccfg::W`](W) writer structure"]
impl crate::Writable for GccfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GCCFG to value 0"]
impl crate::Resettable for GccfgSpec {
    const RESET_VALUE: u32 = 0;
}

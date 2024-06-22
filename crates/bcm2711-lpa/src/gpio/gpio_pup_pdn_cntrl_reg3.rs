#[doc = "Register `GPIO_PUP_PDN_CNTRL_REG3` reader"]
pub type R = crate::R<GpioPupPdnCntrlReg3Spec>;
#[doc = "Register `GPIO_PUP_PDN_CNTRL_REG3` writer"]
pub type W = crate::W<GpioPupPdnCntrlReg3Spec>;
#[doc = "Resistor select for 48"]
pub use super::gpio_pup_pdn_cntrl_reg0::BpPull;
#[doc = "Field `GPIO_PUP_PDN_CNTRL48` reader - Resistor select for 48"]
pub use super::gpio_pup_pdn_cntrl_reg0::GpioPupPdnCntrl0R as GpioPupPdnCntrl48R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL49` reader - Resistor select for 49"]
pub use super::gpio_pup_pdn_cntrl_reg0::GpioPupPdnCntrl0R as GpioPupPdnCntrl49R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL50` reader - Resistor select for 50"]
pub use super::gpio_pup_pdn_cntrl_reg0::GpioPupPdnCntrl0R as GpioPupPdnCntrl50R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL51` reader - Resistor select for 51"]
pub use super::gpio_pup_pdn_cntrl_reg0::GpioPupPdnCntrl0R as GpioPupPdnCntrl51R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL52` reader - Resistor select for 52"]
pub use super::gpio_pup_pdn_cntrl_reg0::GpioPupPdnCntrl0R as GpioPupPdnCntrl52R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL53` reader - Resistor select for 53"]
pub use super::gpio_pup_pdn_cntrl_reg0::GpioPupPdnCntrl0R as GpioPupPdnCntrl53R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL54` reader - Resistor select for 54"]
pub use super::gpio_pup_pdn_cntrl_reg0::GpioPupPdnCntrl0R as GpioPupPdnCntrl54R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL55` reader - Resistor select for 55"]
pub use super::gpio_pup_pdn_cntrl_reg0::GpioPupPdnCntrl0R as GpioPupPdnCntrl55R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL56` reader - Resistor select for 56"]
pub use super::gpio_pup_pdn_cntrl_reg0::GpioPupPdnCntrl0R as GpioPupPdnCntrl56R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL57` reader - Resistor select for 57"]
pub use super::gpio_pup_pdn_cntrl_reg0::GpioPupPdnCntrl0R as GpioPupPdnCntrl57R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL48` writer - Resistor select for 48"]
pub use super::gpio_pup_pdn_cntrl_reg0::GpioPupPdnCntrl0W as GpioPupPdnCntrl48W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL49` writer - Resistor select for 49"]
pub use super::gpio_pup_pdn_cntrl_reg0::GpioPupPdnCntrl0W as GpioPupPdnCntrl49W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL50` writer - Resistor select for 50"]
pub use super::gpio_pup_pdn_cntrl_reg0::GpioPupPdnCntrl0W as GpioPupPdnCntrl50W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL51` writer - Resistor select for 51"]
pub use super::gpio_pup_pdn_cntrl_reg0::GpioPupPdnCntrl0W as GpioPupPdnCntrl51W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL52` writer - Resistor select for 52"]
pub use super::gpio_pup_pdn_cntrl_reg0::GpioPupPdnCntrl0W as GpioPupPdnCntrl52W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL53` writer - Resistor select for 53"]
pub use super::gpio_pup_pdn_cntrl_reg0::GpioPupPdnCntrl0W as GpioPupPdnCntrl53W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL54` writer - Resistor select for 54"]
pub use super::gpio_pup_pdn_cntrl_reg0::GpioPupPdnCntrl0W as GpioPupPdnCntrl54W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL55` writer - Resistor select for 55"]
pub use super::gpio_pup_pdn_cntrl_reg0::GpioPupPdnCntrl0W as GpioPupPdnCntrl55W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL56` writer - Resistor select for 56"]
pub use super::gpio_pup_pdn_cntrl_reg0::GpioPupPdnCntrl0W as GpioPupPdnCntrl56W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL57` writer - Resistor select for 57"]
pub use super::gpio_pup_pdn_cntrl_reg0::GpioPupPdnCntrl0W as GpioPupPdnCntrl57W;
impl R {
    #[doc = "Bits 0:1 - Resistor select for 48"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl48(&self) -> GpioPupPdnCntrl48R {
        GpioPupPdnCntrl48R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Resistor select for 49"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl49(&self) -> GpioPupPdnCntrl49R {
        GpioPupPdnCntrl49R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Resistor select for 50"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl50(&self) -> GpioPupPdnCntrl50R {
        GpioPupPdnCntrl50R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Resistor select for 51"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl51(&self) -> GpioPupPdnCntrl51R {
        GpioPupPdnCntrl51R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Resistor select for 52"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl52(&self) -> GpioPupPdnCntrl52R {
        GpioPupPdnCntrl52R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Resistor select for 53"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl53(&self) -> GpioPupPdnCntrl53R {
        GpioPupPdnCntrl53R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Resistor select for 54"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl54(&self) -> GpioPupPdnCntrl54R {
        GpioPupPdnCntrl54R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Resistor select for 55"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl55(&self) -> GpioPupPdnCntrl55R {
        GpioPupPdnCntrl55R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Resistor select for 56"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl56(&self) -> GpioPupPdnCntrl56R {
        GpioPupPdnCntrl56R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Resistor select for 57"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl57(&self) -> GpioPupPdnCntrl57R {
        GpioPupPdnCntrl57R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_PUP_PDN_CNTRL_REG3")
            .field("gpio_pup_pdn_cntrl48", &self.gpio_pup_pdn_cntrl48())
            .field("gpio_pup_pdn_cntrl49", &self.gpio_pup_pdn_cntrl49())
            .field("gpio_pup_pdn_cntrl50", &self.gpio_pup_pdn_cntrl50())
            .field("gpio_pup_pdn_cntrl51", &self.gpio_pup_pdn_cntrl51())
            .field("gpio_pup_pdn_cntrl52", &self.gpio_pup_pdn_cntrl52())
            .field("gpio_pup_pdn_cntrl53", &self.gpio_pup_pdn_cntrl53())
            .field("gpio_pup_pdn_cntrl54", &self.gpio_pup_pdn_cntrl54())
            .field("gpio_pup_pdn_cntrl55", &self.gpio_pup_pdn_cntrl55())
            .field("gpio_pup_pdn_cntrl56", &self.gpio_pup_pdn_cntrl56())
            .field("gpio_pup_pdn_cntrl57", &self.gpio_pup_pdn_cntrl57())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Resistor select for 48"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl48(&mut self) -> GpioPupPdnCntrl48W<GpioPupPdnCntrlReg3Spec> {
        GpioPupPdnCntrl48W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Resistor select for 49"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl49(&mut self) -> GpioPupPdnCntrl49W<GpioPupPdnCntrlReg3Spec> {
        GpioPupPdnCntrl49W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Resistor select for 50"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl50(&mut self) -> GpioPupPdnCntrl50W<GpioPupPdnCntrlReg3Spec> {
        GpioPupPdnCntrl50W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Resistor select for 51"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl51(&mut self) -> GpioPupPdnCntrl51W<GpioPupPdnCntrlReg3Spec> {
        GpioPupPdnCntrl51W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Resistor select for 52"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl52(&mut self) -> GpioPupPdnCntrl52W<GpioPupPdnCntrlReg3Spec> {
        GpioPupPdnCntrl52W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Resistor select for 53"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl53(&mut self) -> GpioPupPdnCntrl53W<GpioPupPdnCntrlReg3Spec> {
        GpioPupPdnCntrl53W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Resistor select for 54"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl54(&mut self) -> GpioPupPdnCntrl54W<GpioPupPdnCntrlReg3Spec> {
        GpioPupPdnCntrl54W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Resistor select for 55"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl55(&mut self) -> GpioPupPdnCntrl55W<GpioPupPdnCntrlReg3Spec> {
        GpioPupPdnCntrl55W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Resistor select for 56"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl56(&mut self) -> GpioPupPdnCntrl56W<GpioPupPdnCntrlReg3Spec> {
        GpioPupPdnCntrl56W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Resistor select for 57"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl57(&mut self) -> GpioPupPdnCntrl57W<GpioPupPdnCntrlReg3Spec> {
        GpioPupPdnCntrl57W::new(self, 18)
    }
}
#[doc = "GPIO Pull-up / Pull-down Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_pup_pdn_cntrl_reg3::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_pup_pdn_cntrl_reg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioPupPdnCntrlReg3Spec;
impl crate::RegisterSpec for GpioPupPdnCntrlReg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_pup_pdn_cntrl_reg3::R`](R) reader structure"]
impl crate::Readable for GpioPupPdnCntrlReg3Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_pup_pdn_cntrl_reg3::W`](W) writer structure"]
impl crate::Writable for GpioPupPdnCntrlReg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}

#[doc = "Register `GPIO_PUP_PDN_CNTRL_REG0` reader"]
pub type R = crate::R<GpioPupPdnCntrlReg0Spec>;
#[doc = "Register `GPIO_PUP_PDN_CNTRL_REG0` writer"]
pub type W = crate::W<GpioPupPdnCntrlReg0Spec>;
#[doc = "Resistor select for 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BpPull {
    #[doc = "0: No pull"]
    None = 0,
    #[doc = "1: Pull up"]
    Up = 1,
    #[doc = "2: Pull down"]
    Down = 2,
}
impl From<BpPull> for u8 {
    #[inline(always)]
    fn from(variant: BpPull) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BpPull {
    type Ux = u8;
}
impl crate::IsEnum for BpPull {}
#[doc = "Field `GPIO_PUP_PDN_CNTRL0` reader - Resistor select for 0"]
pub type GpioPupPdnCntrl0R = crate::FieldReader<BpPull>;
impl GpioPupPdnCntrl0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BpPull> {
        match self.bits {
            0 => Some(BpPull::None),
            1 => Some(BpPull::Up),
            2 => Some(BpPull::Down),
            _ => None,
        }
    }
    #[doc = "No pull"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == BpPull::None
    }
    #[doc = "Pull up"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == BpPull::Up
    }
    #[doc = "Pull down"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == BpPull::Down
    }
}
#[doc = "Field `GPIO_PUP_PDN_CNTRL0` writer - Resistor select for 0"]
pub type GpioPupPdnCntrl0W<'a, REG> = crate::FieldWriter<'a, REG, 2, BpPull>;
impl<'a, REG> GpioPupPdnCntrl0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(BpPull::None)
    }
    #[doc = "Pull up"]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(BpPull::Up)
    }
    #[doc = "Pull down"]
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(BpPull::Down)
    }
}
#[doc = "Field `GPIO_PUP_PDN_CNTRL1` reader - Resistor select for 1"]
pub use GpioPupPdnCntrl0R as GpioPupPdnCntrl1R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL2` reader - Resistor select for 2"]
pub use GpioPupPdnCntrl0R as GpioPupPdnCntrl2R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL3` reader - Resistor select for 3"]
pub use GpioPupPdnCntrl0R as GpioPupPdnCntrl3R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL4` reader - Resistor select for 4"]
pub use GpioPupPdnCntrl0R as GpioPupPdnCntrl4R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL5` reader - Resistor select for 5"]
pub use GpioPupPdnCntrl0R as GpioPupPdnCntrl5R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL6` reader - Resistor select for 6"]
pub use GpioPupPdnCntrl0R as GpioPupPdnCntrl6R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL7` reader - Resistor select for 7"]
pub use GpioPupPdnCntrl0R as GpioPupPdnCntrl7R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL8` reader - Resistor select for 8"]
pub use GpioPupPdnCntrl0R as GpioPupPdnCntrl8R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL9` reader - Resistor select for 9"]
pub use GpioPupPdnCntrl0R as GpioPupPdnCntrl9R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL10` reader - Resistor select for 10"]
pub use GpioPupPdnCntrl0R as GpioPupPdnCntrl10R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL11` reader - Resistor select for 11"]
pub use GpioPupPdnCntrl0R as GpioPupPdnCntrl11R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL12` reader - Resistor select for 12"]
pub use GpioPupPdnCntrl0R as GpioPupPdnCntrl12R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL13` reader - Resistor select for 13"]
pub use GpioPupPdnCntrl0R as GpioPupPdnCntrl13R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL14` reader - Resistor select for 14"]
pub use GpioPupPdnCntrl0R as GpioPupPdnCntrl14R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL15` reader - Resistor select for 15"]
pub use GpioPupPdnCntrl0R as GpioPupPdnCntrl15R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL1` writer - Resistor select for 1"]
pub use GpioPupPdnCntrl0W as GpioPupPdnCntrl1W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL2` writer - Resistor select for 2"]
pub use GpioPupPdnCntrl0W as GpioPupPdnCntrl2W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL3` writer - Resistor select for 3"]
pub use GpioPupPdnCntrl0W as GpioPupPdnCntrl3W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL4` writer - Resistor select for 4"]
pub use GpioPupPdnCntrl0W as GpioPupPdnCntrl4W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL5` writer - Resistor select for 5"]
pub use GpioPupPdnCntrl0W as GpioPupPdnCntrl5W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL6` writer - Resistor select for 6"]
pub use GpioPupPdnCntrl0W as GpioPupPdnCntrl6W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL7` writer - Resistor select for 7"]
pub use GpioPupPdnCntrl0W as GpioPupPdnCntrl7W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL8` writer - Resistor select for 8"]
pub use GpioPupPdnCntrl0W as GpioPupPdnCntrl8W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL9` writer - Resistor select for 9"]
pub use GpioPupPdnCntrl0W as GpioPupPdnCntrl9W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL10` writer - Resistor select for 10"]
pub use GpioPupPdnCntrl0W as GpioPupPdnCntrl10W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL11` writer - Resistor select for 11"]
pub use GpioPupPdnCntrl0W as GpioPupPdnCntrl11W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL12` writer - Resistor select for 12"]
pub use GpioPupPdnCntrl0W as GpioPupPdnCntrl12W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL13` writer - Resistor select for 13"]
pub use GpioPupPdnCntrl0W as GpioPupPdnCntrl13W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL14` writer - Resistor select for 14"]
pub use GpioPupPdnCntrl0W as GpioPupPdnCntrl14W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL15` writer - Resistor select for 15"]
pub use GpioPupPdnCntrl0W as GpioPupPdnCntrl15W;
impl R {
    #[doc = "Bits 0:1 - Resistor select for 0"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl0(&self) -> GpioPupPdnCntrl0R {
        GpioPupPdnCntrl0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Resistor select for 1"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl1(&self) -> GpioPupPdnCntrl1R {
        GpioPupPdnCntrl1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Resistor select for 2"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl2(&self) -> GpioPupPdnCntrl2R {
        GpioPupPdnCntrl2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Resistor select for 3"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl3(&self) -> GpioPupPdnCntrl3R {
        GpioPupPdnCntrl3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Resistor select for 4"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl4(&self) -> GpioPupPdnCntrl4R {
        GpioPupPdnCntrl4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Resistor select for 5"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl5(&self) -> GpioPupPdnCntrl5R {
        GpioPupPdnCntrl5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Resistor select for 6"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl6(&self) -> GpioPupPdnCntrl6R {
        GpioPupPdnCntrl6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Resistor select for 7"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl7(&self) -> GpioPupPdnCntrl7R {
        GpioPupPdnCntrl7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Resistor select for 8"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl8(&self) -> GpioPupPdnCntrl8R {
        GpioPupPdnCntrl8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Resistor select for 9"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl9(&self) -> GpioPupPdnCntrl9R {
        GpioPupPdnCntrl9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Resistor select for 10"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl10(&self) -> GpioPupPdnCntrl10R {
        GpioPupPdnCntrl10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Resistor select for 11"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl11(&self) -> GpioPupPdnCntrl11R {
        GpioPupPdnCntrl11R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Resistor select for 12"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl12(&self) -> GpioPupPdnCntrl12R {
        GpioPupPdnCntrl12R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Resistor select for 13"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl13(&self) -> GpioPupPdnCntrl13R {
        GpioPupPdnCntrl13R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Resistor select for 14"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl14(&self) -> GpioPupPdnCntrl14R {
        GpioPupPdnCntrl14R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Resistor select for 15"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl15(&self) -> GpioPupPdnCntrl15R {
        GpioPupPdnCntrl15R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_PUP_PDN_CNTRL_REG0")
            .field("gpio_pup_pdn_cntrl0", &self.gpio_pup_pdn_cntrl0())
            .field("gpio_pup_pdn_cntrl1", &self.gpio_pup_pdn_cntrl1())
            .field("gpio_pup_pdn_cntrl2", &self.gpio_pup_pdn_cntrl2())
            .field("gpio_pup_pdn_cntrl3", &self.gpio_pup_pdn_cntrl3())
            .field("gpio_pup_pdn_cntrl4", &self.gpio_pup_pdn_cntrl4())
            .field("gpio_pup_pdn_cntrl5", &self.gpio_pup_pdn_cntrl5())
            .field("gpio_pup_pdn_cntrl6", &self.gpio_pup_pdn_cntrl6())
            .field("gpio_pup_pdn_cntrl7", &self.gpio_pup_pdn_cntrl7())
            .field("gpio_pup_pdn_cntrl8", &self.gpio_pup_pdn_cntrl8())
            .field("gpio_pup_pdn_cntrl9", &self.gpio_pup_pdn_cntrl9())
            .field("gpio_pup_pdn_cntrl10", &self.gpio_pup_pdn_cntrl10())
            .field("gpio_pup_pdn_cntrl11", &self.gpio_pup_pdn_cntrl11())
            .field("gpio_pup_pdn_cntrl12", &self.gpio_pup_pdn_cntrl12())
            .field("gpio_pup_pdn_cntrl13", &self.gpio_pup_pdn_cntrl13())
            .field("gpio_pup_pdn_cntrl14", &self.gpio_pup_pdn_cntrl14())
            .field("gpio_pup_pdn_cntrl15", &self.gpio_pup_pdn_cntrl15())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Resistor select for 0"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl0(&mut self) -> GpioPupPdnCntrl0W<GpioPupPdnCntrlReg0Spec> {
        GpioPupPdnCntrl0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Resistor select for 1"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl1(&mut self) -> GpioPupPdnCntrl1W<GpioPupPdnCntrlReg0Spec> {
        GpioPupPdnCntrl1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Resistor select for 2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl2(&mut self) -> GpioPupPdnCntrl2W<GpioPupPdnCntrlReg0Spec> {
        GpioPupPdnCntrl2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Resistor select for 3"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl3(&mut self) -> GpioPupPdnCntrl3W<GpioPupPdnCntrlReg0Spec> {
        GpioPupPdnCntrl3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Resistor select for 4"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl4(&mut self) -> GpioPupPdnCntrl4W<GpioPupPdnCntrlReg0Spec> {
        GpioPupPdnCntrl4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Resistor select for 5"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl5(&mut self) -> GpioPupPdnCntrl5W<GpioPupPdnCntrlReg0Spec> {
        GpioPupPdnCntrl5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Resistor select for 6"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl6(&mut self) -> GpioPupPdnCntrl6W<GpioPupPdnCntrlReg0Spec> {
        GpioPupPdnCntrl6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Resistor select for 7"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl7(&mut self) -> GpioPupPdnCntrl7W<GpioPupPdnCntrlReg0Spec> {
        GpioPupPdnCntrl7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Resistor select for 8"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl8(&mut self) -> GpioPupPdnCntrl8W<GpioPupPdnCntrlReg0Spec> {
        GpioPupPdnCntrl8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Resistor select for 9"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl9(&mut self) -> GpioPupPdnCntrl9W<GpioPupPdnCntrlReg0Spec> {
        GpioPupPdnCntrl9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Resistor select for 10"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl10(&mut self) -> GpioPupPdnCntrl10W<GpioPupPdnCntrlReg0Spec> {
        GpioPupPdnCntrl10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Resistor select for 11"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl11(&mut self) -> GpioPupPdnCntrl11W<GpioPupPdnCntrlReg0Spec> {
        GpioPupPdnCntrl11W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Resistor select for 12"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl12(&mut self) -> GpioPupPdnCntrl12W<GpioPupPdnCntrlReg0Spec> {
        GpioPupPdnCntrl12W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Resistor select for 13"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl13(&mut self) -> GpioPupPdnCntrl13W<GpioPupPdnCntrlReg0Spec> {
        GpioPupPdnCntrl13W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Resistor select for 14"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl14(&mut self) -> GpioPupPdnCntrl14W<GpioPupPdnCntrlReg0Spec> {
        GpioPupPdnCntrl14W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Resistor select for 15"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl15(&mut self) -> GpioPupPdnCntrl15W<GpioPupPdnCntrlReg0Spec> {
        GpioPupPdnCntrl15W::new(self, 30)
    }
}
#[doc = "GPIO Pull-up / Pull-down Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_pup_pdn_cntrl_reg0::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_pup_pdn_cntrl_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioPupPdnCntrlReg0Spec;
impl crate::RegisterSpec for GpioPupPdnCntrlReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_pup_pdn_cntrl_reg0::R`](R) reader structure"]
impl crate::Readable for GpioPupPdnCntrlReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_pup_pdn_cntrl_reg0::W`](W) writer structure"]
impl crate::Writable for GpioPupPdnCntrlReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}

#[doc = "Register `GPCLR1` writer"]
pub type W = crate::W<Gpclr1Spec>;
#[doc = "Field `CLR32` writer - Clear 32"]
pub type Clr32W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR33` writer - Clear 33"]
pub type Clr33W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR34` writer - Clear 34"]
pub type Clr34W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR35` writer - Clear 35"]
pub type Clr35W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR36` writer - Clear 36"]
pub type Clr36W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR37` writer - Clear 37"]
pub type Clr37W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR38` writer - Clear 38"]
pub type Clr38W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR39` writer - Clear 39"]
pub type Clr39W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR40` writer - Clear 40"]
pub type Clr40W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR41` writer - Clear 41"]
pub type Clr41W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR42` writer - Clear 42"]
pub type Clr42W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR43` writer - Clear 43"]
pub type Clr43W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR44` writer - Clear 44"]
pub type Clr44W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR45` writer - Clear 45"]
pub type Clr45W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR46` writer - Clear 46"]
pub type Clr46W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR47` writer - Clear 47"]
pub type Clr47W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR48` writer - Clear 48"]
pub type Clr48W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR49` writer - Clear 49"]
pub type Clr49W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR50` writer - Clear 50"]
pub type Clr50W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR51` writer - Clear 51"]
pub type Clr51W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR52` writer - Clear 52"]
pub type Clr52W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR53` writer - Clear 53"]
pub type Clr53W<'a, REG> = crate::BitWriter1C<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<Gpclr1Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Clear 32"]
    #[inline(always)]
    #[must_use]
    pub fn clr32(&mut self) -> Clr32W<Gpclr1Spec> {
        Clr32W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear 33"]
    #[inline(always)]
    #[must_use]
    pub fn clr33(&mut self) -> Clr33W<Gpclr1Spec> {
        Clr33W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear 34"]
    #[inline(always)]
    #[must_use]
    pub fn clr34(&mut self) -> Clr34W<Gpclr1Spec> {
        Clr34W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear 35"]
    #[inline(always)]
    #[must_use]
    pub fn clr35(&mut self) -> Clr35W<Gpclr1Spec> {
        Clr35W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear 36"]
    #[inline(always)]
    #[must_use]
    pub fn clr36(&mut self) -> Clr36W<Gpclr1Spec> {
        Clr36W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear 37"]
    #[inline(always)]
    #[must_use]
    pub fn clr37(&mut self) -> Clr37W<Gpclr1Spec> {
        Clr37W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear 38"]
    #[inline(always)]
    #[must_use]
    pub fn clr38(&mut self) -> Clr38W<Gpclr1Spec> {
        Clr38W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear 39"]
    #[inline(always)]
    #[must_use]
    pub fn clr39(&mut self) -> Clr39W<Gpclr1Spec> {
        Clr39W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear 40"]
    #[inline(always)]
    #[must_use]
    pub fn clr40(&mut self) -> Clr40W<Gpclr1Spec> {
        Clr40W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear 41"]
    #[inline(always)]
    #[must_use]
    pub fn clr41(&mut self) -> Clr41W<Gpclr1Spec> {
        Clr41W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear 42"]
    #[inline(always)]
    #[must_use]
    pub fn clr42(&mut self) -> Clr42W<Gpclr1Spec> {
        Clr42W::new(self, 10)
    }
    #[doc = "Bit 11 - Clear 43"]
    #[inline(always)]
    #[must_use]
    pub fn clr43(&mut self) -> Clr43W<Gpclr1Spec> {
        Clr43W::new(self, 11)
    }
    #[doc = "Bit 12 - Clear 44"]
    #[inline(always)]
    #[must_use]
    pub fn clr44(&mut self) -> Clr44W<Gpclr1Spec> {
        Clr44W::new(self, 12)
    }
    #[doc = "Bit 13 - Clear 45"]
    #[inline(always)]
    #[must_use]
    pub fn clr45(&mut self) -> Clr45W<Gpclr1Spec> {
        Clr45W::new(self, 13)
    }
    #[doc = "Bit 14 - Clear 46"]
    #[inline(always)]
    #[must_use]
    pub fn clr46(&mut self) -> Clr46W<Gpclr1Spec> {
        Clr46W::new(self, 14)
    }
    #[doc = "Bit 15 - Clear 47"]
    #[inline(always)]
    #[must_use]
    pub fn clr47(&mut self) -> Clr47W<Gpclr1Spec> {
        Clr47W::new(self, 15)
    }
    #[doc = "Bit 16 - Clear 48"]
    #[inline(always)]
    #[must_use]
    pub fn clr48(&mut self) -> Clr48W<Gpclr1Spec> {
        Clr48W::new(self, 16)
    }
    #[doc = "Bit 17 - Clear 49"]
    #[inline(always)]
    #[must_use]
    pub fn clr49(&mut self) -> Clr49W<Gpclr1Spec> {
        Clr49W::new(self, 17)
    }
    #[doc = "Bit 18 - Clear 50"]
    #[inline(always)]
    #[must_use]
    pub fn clr50(&mut self) -> Clr50W<Gpclr1Spec> {
        Clr50W::new(self, 18)
    }
    #[doc = "Bit 19 - Clear 51"]
    #[inline(always)]
    #[must_use]
    pub fn clr51(&mut self) -> Clr51W<Gpclr1Spec> {
        Clr51W::new(self, 19)
    }
    #[doc = "Bit 20 - Clear 52"]
    #[inline(always)]
    #[must_use]
    pub fn clr52(&mut self) -> Clr52W<Gpclr1Spec> {
        Clr52W::new(self, 20)
    }
    #[doc = "Bit 21 - Clear 53"]
    #[inline(always)]
    #[must_use]
    pub fn clr53(&mut self) -> Clr53W<Gpclr1Spec> {
        Clr53W::new(self, 21)
    }
}
#[doc = "GPIO Pin Output Clear 1\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpclr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpclr1Spec;
impl crate::RegisterSpec for Gpclr1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpclr1::W`](W) writer structure"]
impl crate::Writable for Gpclr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x003f_ffff;
}

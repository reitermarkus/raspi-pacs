#[doc = "Register `GPCLR0` writer"]
pub type W = crate::W<Gpclr0Spec>;
#[doc = "Field `CLR0` writer - Clear 0"]
pub type Clr0W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR1` writer - Clear 1"]
pub type Clr1W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR2` writer - Clear 2"]
pub type Clr2W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR3` writer - Clear 3"]
pub type Clr3W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR4` writer - Clear 4"]
pub type Clr4W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR5` writer - Clear 5"]
pub type Clr5W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR6` writer - Clear 6"]
pub type Clr6W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR7` writer - Clear 7"]
pub type Clr7W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR8` writer - Clear 8"]
pub type Clr8W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR9` writer - Clear 9"]
pub type Clr9W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR10` writer - Clear 10"]
pub type Clr10W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR11` writer - Clear 11"]
pub type Clr11W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR12` writer - Clear 12"]
pub type Clr12W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR13` writer - Clear 13"]
pub type Clr13W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR14` writer - Clear 14"]
pub type Clr14W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR15` writer - Clear 15"]
pub type Clr15W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR16` writer - Clear 16"]
pub type Clr16W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR17` writer - Clear 17"]
pub type Clr17W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR18` writer - Clear 18"]
pub type Clr18W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR19` writer - Clear 19"]
pub type Clr19W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR20` writer - Clear 20"]
pub type Clr20W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR21` writer - Clear 21"]
pub type Clr21W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR22` writer - Clear 22"]
pub type Clr22W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR23` writer - Clear 23"]
pub type Clr23W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR24` writer - Clear 24"]
pub type Clr24W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR25` writer - Clear 25"]
pub type Clr25W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR26` writer - Clear 26"]
pub type Clr26W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR27` writer - Clear 27"]
pub type Clr27W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR28` writer - Clear 28"]
pub type Clr28W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR29` writer - Clear 29"]
pub type Clr29W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR30` writer - Clear 30"]
pub type Clr30W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR31` writer - Clear 31"]
pub type Clr31W<'a, REG> = crate::BitWriter1C<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<Gpclr0Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Clear 0"]
    #[inline(always)]
    #[must_use]
    pub fn clr0(&mut self) -> Clr0W<Gpclr0Spec> {
        Clr0W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear 1"]
    #[inline(always)]
    #[must_use]
    pub fn clr1(&mut self) -> Clr1W<Gpclr0Spec> {
        Clr1W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear 2"]
    #[inline(always)]
    #[must_use]
    pub fn clr2(&mut self) -> Clr2W<Gpclr0Spec> {
        Clr2W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear 3"]
    #[inline(always)]
    #[must_use]
    pub fn clr3(&mut self) -> Clr3W<Gpclr0Spec> {
        Clr3W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear 4"]
    #[inline(always)]
    #[must_use]
    pub fn clr4(&mut self) -> Clr4W<Gpclr0Spec> {
        Clr4W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear 5"]
    #[inline(always)]
    #[must_use]
    pub fn clr5(&mut self) -> Clr5W<Gpclr0Spec> {
        Clr5W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear 6"]
    #[inline(always)]
    #[must_use]
    pub fn clr6(&mut self) -> Clr6W<Gpclr0Spec> {
        Clr6W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear 7"]
    #[inline(always)]
    #[must_use]
    pub fn clr7(&mut self) -> Clr7W<Gpclr0Spec> {
        Clr7W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear 8"]
    #[inline(always)]
    #[must_use]
    pub fn clr8(&mut self) -> Clr8W<Gpclr0Spec> {
        Clr8W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear 9"]
    #[inline(always)]
    #[must_use]
    pub fn clr9(&mut self) -> Clr9W<Gpclr0Spec> {
        Clr9W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear 10"]
    #[inline(always)]
    #[must_use]
    pub fn clr10(&mut self) -> Clr10W<Gpclr0Spec> {
        Clr10W::new(self, 10)
    }
    #[doc = "Bit 11 - Clear 11"]
    #[inline(always)]
    #[must_use]
    pub fn clr11(&mut self) -> Clr11W<Gpclr0Spec> {
        Clr11W::new(self, 11)
    }
    #[doc = "Bit 12 - Clear 12"]
    #[inline(always)]
    #[must_use]
    pub fn clr12(&mut self) -> Clr12W<Gpclr0Spec> {
        Clr12W::new(self, 12)
    }
    #[doc = "Bit 13 - Clear 13"]
    #[inline(always)]
    #[must_use]
    pub fn clr13(&mut self) -> Clr13W<Gpclr0Spec> {
        Clr13W::new(self, 13)
    }
    #[doc = "Bit 14 - Clear 14"]
    #[inline(always)]
    #[must_use]
    pub fn clr14(&mut self) -> Clr14W<Gpclr0Spec> {
        Clr14W::new(self, 14)
    }
    #[doc = "Bit 15 - Clear 15"]
    #[inline(always)]
    #[must_use]
    pub fn clr15(&mut self) -> Clr15W<Gpclr0Spec> {
        Clr15W::new(self, 15)
    }
    #[doc = "Bit 16 - Clear 16"]
    #[inline(always)]
    #[must_use]
    pub fn clr16(&mut self) -> Clr16W<Gpclr0Spec> {
        Clr16W::new(self, 16)
    }
    #[doc = "Bit 17 - Clear 17"]
    #[inline(always)]
    #[must_use]
    pub fn clr17(&mut self) -> Clr17W<Gpclr0Spec> {
        Clr17W::new(self, 17)
    }
    #[doc = "Bit 18 - Clear 18"]
    #[inline(always)]
    #[must_use]
    pub fn clr18(&mut self) -> Clr18W<Gpclr0Spec> {
        Clr18W::new(self, 18)
    }
    #[doc = "Bit 19 - Clear 19"]
    #[inline(always)]
    #[must_use]
    pub fn clr19(&mut self) -> Clr19W<Gpclr0Spec> {
        Clr19W::new(self, 19)
    }
    #[doc = "Bit 20 - Clear 20"]
    #[inline(always)]
    #[must_use]
    pub fn clr20(&mut self) -> Clr20W<Gpclr0Spec> {
        Clr20W::new(self, 20)
    }
    #[doc = "Bit 21 - Clear 21"]
    #[inline(always)]
    #[must_use]
    pub fn clr21(&mut self) -> Clr21W<Gpclr0Spec> {
        Clr21W::new(self, 21)
    }
    #[doc = "Bit 22 - Clear 22"]
    #[inline(always)]
    #[must_use]
    pub fn clr22(&mut self) -> Clr22W<Gpclr0Spec> {
        Clr22W::new(self, 22)
    }
    #[doc = "Bit 23 - Clear 23"]
    #[inline(always)]
    #[must_use]
    pub fn clr23(&mut self) -> Clr23W<Gpclr0Spec> {
        Clr23W::new(self, 23)
    }
    #[doc = "Bit 24 - Clear 24"]
    #[inline(always)]
    #[must_use]
    pub fn clr24(&mut self) -> Clr24W<Gpclr0Spec> {
        Clr24W::new(self, 24)
    }
    #[doc = "Bit 25 - Clear 25"]
    #[inline(always)]
    #[must_use]
    pub fn clr25(&mut self) -> Clr25W<Gpclr0Spec> {
        Clr25W::new(self, 25)
    }
    #[doc = "Bit 26 - Clear 26"]
    #[inline(always)]
    #[must_use]
    pub fn clr26(&mut self) -> Clr26W<Gpclr0Spec> {
        Clr26W::new(self, 26)
    }
    #[doc = "Bit 27 - Clear 27"]
    #[inline(always)]
    #[must_use]
    pub fn clr27(&mut self) -> Clr27W<Gpclr0Spec> {
        Clr27W::new(self, 27)
    }
    #[doc = "Bit 28 - Clear 28"]
    #[inline(always)]
    #[must_use]
    pub fn clr28(&mut self) -> Clr28W<Gpclr0Spec> {
        Clr28W::new(self, 28)
    }
    #[doc = "Bit 29 - Clear 29"]
    #[inline(always)]
    #[must_use]
    pub fn clr29(&mut self) -> Clr29W<Gpclr0Spec> {
        Clr29W::new(self, 29)
    }
    #[doc = "Bit 30 - Clear 30"]
    #[inline(always)]
    #[must_use]
    pub fn clr30(&mut self) -> Clr30W<Gpclr0Spec> {
        Clr30W::new(self, 30)
    }
    #[doc = "Bit 31 - Clear 31"]
    #[inline(always)]
    #[must_use]
    pub fn clr31(&mut self) -> Clr31W<Gpclr0Spec> {
        Clr31W::new(self, 31)
    }
}
#[doc = "GPIO Pin Output Clear 0\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpclr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpclr0Spec;
impl crate::RegisterSpec for Gpclr0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpclr0::W`](W) writer structure"]
impl crate::Writable for Gpclr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff_ffff;
}

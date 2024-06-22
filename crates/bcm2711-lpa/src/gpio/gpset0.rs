#[doc = "Register `GPSET0` writer"]
pub type W = crate::W<Gpset0Spec>;
#[doc = "Field `SET0` writer - Set 0"]
pub type Set0W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET1` writer - Set 1"]
pub type Set1W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET2` writer - Set 2"]
pub type Set2W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET3` writer - Set 3"]
pub type Set3W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET4` writer - Set 4"]
pub type Set4W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET5` writer - Set 5"]
pub type Set5W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET6` writer - Set 6"]
pub type Set6W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET7` writer - Set 7"]
pub type Set7W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET8` writer - Set 8"]
pub type Set8W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET9` writer - Set 9"]
pub type Set9W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET10` writer - Set 10"]
pub type Set10W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET11` writer - Set 11"]
pub type Set11W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET12` writer - Set 12"]
pub type Set12W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET13` writer - Set 13"]
pub type Set13W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET14` writer - Set 14"]
pub type Set14W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET15` writer - Set 15"]
pub type Set15W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET16` writer - Set 16"]
pub type Set16W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET17` writer - Set 17"]
pub type Set17W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET18` writer - Set 18"]
pub type Set18W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET19` writer - Set 19"]
pub type Set19W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET20` writer - Set 20"]
pub type Set20W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET21` writer - Set 21"]
pub type Set21W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET22` writer - Set 22"]
pub type Set22W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET23` writer - Set 23"]
pub type Set23W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET24` writer - Set 24"]
pub type Set24W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET25` writer - Set 25"]
pub type Set25W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET26` writer - Set 26"]
pub type Set26W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET27` writer - Set 27"]
pub type Set27W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET28` writer - Set 28"]
pub type Set28W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET29` writer - Set 29"]
pub type Set29W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET30` writer - Set 30"]
pub type Set30W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET31` writer - Set 31"]
pub type Set31W<'a, REG> = crate::BitWriter1S<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<Gpset0Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set 0"]
    #[inline(always)]
    #[must_use]
    pub fn set0(&mut self) -> Set0W<Gpset0Spec> {
        Set0W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 1"]
    #[inline(always)]
    #[must_use]
    pub fn set1(&mut self) -> Set1W<Gpset0Spec> {
        Set1W::new(self, 1)
    }
    #[doc = "Bit 2 - Set 2"]
    #[inline(always)]
    #[must_use]
    pub fn set2(&mut self) -> Set2W<Gpset0Spec> {
        Set2W::new(self, 2)
    }
    #[doc = "Bit 3 - Set 3"]
    #[inline(always)]
    #[must_use]
    pub fn set3(&mut self) -> Set3W<Gpset0Spec> {
        Set3W::new(self, 3)
    }
    #[doc = "Bit 4 - Set 4"]
    #[inline(always)]
    #[must_use]
    pub fn set4(&mut self) -> Set4W<Gpset0Spec> {
        Set4W::new(self, 4)
    }
    #[doc = "Bit 5 - Set 5"]
    #[inline(always)]
    #[must_use]
    pub fn set5(&mut self) -> Set5W<Gpset0Spec> {
        Set5W::new(self, 5)
    }
    #[doc = "Bit 6 - Set 6"]
    #[inline(always)]
    #[must_use]
    pub fn set6(&mut self) -> Set6W<Gpset0Spec> {
        Set6W::new(self, 6)
    }
    #[doc = "Bit 7 - Set 7"]
    #[inline(always)]
    #[must_use]
    pub fn set7(&mut self) -> Set7W<Gpset0Spec> {
        Set7W::new(self, 7)
    }
    #[doc = "Bit 8 - Set 8"]
    #[inline(always)]
    #[must_use]
    pub fn set8(&mut self) -> Set8W<Gpset0Spec> {
        Set8W::new(self, 8)
    }
    #[doc = "Bit 9 - Set 9"]
    #[inline(always)]
    #[must_use]
    pub fn set9(&mut self) -> Set9W<Gpset0Spec> {
        Set9W::new(self, 9)
    }
    #[doc = "Bit 10 - Set 10"]
    #[inline(always)]
    #[must_use]
    pub fn set10(&mut self) -> Set10W<Gpset0Spec> {
        Set10W::new(self, 10)
    }
    #[doc = "Bit 11 - Set 11"]
    #[inline(always)]
    #[must_use]
    pub fn set11(&mut self) -> Set11W<Gpset0Spec> {
        Set11W::new(self, 11)
    }
    #[doc = "Bit 12 - Set 12"]
    #[inline(always)]
    #[must_use]
    pub fn set12(&mut self) -> Set12W<Gpset0Spec> {
        Set12W::new(self, 12)
    }
    #[doc = "Bit 13 - Set 13"]
    #[inline(always)]
    #[must_use]
    pub fn set13(&mut self) -> Set13W<Gpset0Spec> {
        Set13W::new(self, 13)
    }
    #[doc = "Bit 14 - Set 14"]
    #[inline(always)]
    #[must_use]
    pub fn set14(&mut self) -> Set14W<Gpset0Spec> {
        Set14W::new(self, 14)
    }
    #[doc = "Bit 15 - Set 15"]
    #[inline(always)]
    #[must_use]
    pub fn set15(&mut self) -> Set15W<Gpset0Spec> {
        Set15W::new(self, 15)
    }
    #[doc = "Bit 16 - Set 16"]
    #[inline(always)]
    #[must_use]
    pub fn set16(&mut self) -> Set16W<Gpset0Spec> {
        Set16W::new(self, 16)
    }
    #[doc = "Bit 17 - Set 17"]
    #[inline(always)]
    #[must_use]
    pub fn set17(&mut self) -> Set17W<Gpset0Spec> {
        Set17W::new(self, 17)
    }
    #[doc = "Bit 18 - Set 18"]
    #[inline(always)]
    #[must_use]
    pub fn set18(&mut self) -> Set18W<Gpset0Spec> {
        Set18W::new(self, 18)
    }
    #[doc = "Bit 19 - Set 19"]
    #[inline(always)]
    #[must_use]
    pub fn set19(&mut self) -> Set19W<Gpset0Spec> {
        Set19W::new(self, 19)
    }
    #[doc = "Bit 20 - Set 20"]
    #[inline(always)]
    #[must_use]
    pub fn set20(&mut self) -> Set20W<Gpset0Spec> {
        Set20W::new(self, 20)
    }
    #[doc = "Bit 21 - Set 21"]
    #[inline(always)]
    #[must_use]
    pub fn set21(&mut self) -> Set21W<Gpset0Spec> {
        Set21W::new(self, 21)
    }
    #[doc = "Bit 22 - Set 22"]
    #[inline(always)]
    #[must_use]
    pub fn set22(&mut self) -> Set22W<Gpset0Spec> {
        Set22W::new(self, 22)
    }
    #[doc = "Bit 23 - Set 23"]
    #[inline(always)]
    #[must_use]
    pub fn set23(&mut self) -> Set23W<Gpset0Spec> {
        Set23W::new(self, 23)
    }
    #[doc = "Bit 24 - Set 24"]
    #[inline(always)]
    #[must_use]
    pub fn set24(&mut self) -> Set24W<Gpset0Spec> {
        Set24W::new(self, 24)
    }
    #[doc = "Bit 25 - Set 25"]
    #[inline(always)]
    #[must_use]
    pub fn set25(&mut self) -> Set25W<Gpset0Spec> {
        Set25W::new(self, 25)
    }
    #[doc = "Bit 26 - Set 26"]
    #[inline(always)]
    #[must_use]
    pub fn set26(&mut self) -> Set26W<Gpset0Spec> {
        Set26W::new(self, 26)
    }
    #[doc = "Bit 27 - Set 27"]
    #[inline(always)]
    #[must_use]
    pub fn set27(&mut self) -> Set27W<Gpset0Spec> {
        Set27W::new(self, 27)
    }
    #[doc = "Bit 28 - Set 28"]
    #[inline(always)]
    #[must_use]
    pub fn set28(&mut self) -> Set28W<Gpset0Spec> {
        Set28W::new(self, 28)
    }
    #[doc = "Bit 29 - Set 29"]
    #[inline(always)]
    #[must_use]
    pub fn set29(&mut self) -> Set29W<Gpset0Spec> {
        Set29W::new(self, 29)
    }
    #[doc = "Bit 30 - Set 30"]
    #[inline(always)]
    #[must_use]
    pub fn set30(&mut self) -> Set30W<Gpset0Spec> {
        Set30W::new(self, 30)
    }
    #[doc = "Bit 31 - Set 31"]
    #[inline(always)]
    #[must_use]
    pub fn set31(&mut self) -> Set31W<Gpset0Spec> {
        Set31W::new(self, 31)
    }
}
#[doc = "GPIO Pin Output Set 0\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpset0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpset0Spec;
impl crate::RegisterSpec for Gpset0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpset0::W`](W) writer structure"]
impl crate::Writable for Gpset0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff_ffff;
}

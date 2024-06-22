#[doc = "Register `GPLEV0` reader"]
pub type R = crate::R<Gplev0Spec>;
#[doc = "Field `LEV0` reader - Level 0"]
pub type Lev0R = crate::BitReader;
#[doc = "Field `LEV1` reader - Level 1"]
pub type Lev1R = crate::BitReader;
#[doc = "Field `LEV2` reader - Level 2"]
pub type Lev2R = crate::BitReader;
#[doc = "Field `LEV3` reader - Level 3"]
pub type Lev3R = crate::BitReader;
#[doc = "Field `LEV4` reader - Level 4"]
pub type Lev4R = crate::BitReader;
#[doc = "Field `LEV5` reader - Level 5"]
pub type Lev5R = crate::BitReader;
#[doc = "Field `LEV6` reader - Level 6"]
pub type Lev6R = crate::BitReader;
#[doc = "Field `LEV7` reader - Level 7"]
pub type Lev7R = crate::BitReader;
#[doc = "Field `LEV8` reader - Level 8"]
pub type Lev8R = crate::BitReader;
#[doc = "Field `LEV9` reader - Level 9"]
pub type Lev9R = crate::BitReader;
#[doc = "Field `LEV10` reader - Level 10"]
pub type Lev10R = crate::BitReader;
#[doc = "Field `LEV11` reader - Level 11"]
pub type Lev11R = crate::BitReader;
#[doc = "Field `LEV12` reader - Level 12"]
pub type Lev12R = crate::BitReader;
#[doc = "Field `LEV13` reader - Level 13"]
pub type Lev13R = crate::BitReader;
#[doc = "Field `LEV14` reader - Level 14"]
pub type Lev14R = crate::BitReader;
#[doc = "Field `LEV15` reader - Level 15"]
pub type Lev15R = crate::BitReader;
#[doc = "Field `LEV16` reader - Level 16"]
pub type Lev16R = crate::BitReader;
#[doc = "Field `LEV17` reader - Level 17"]
pub type Lev17R = crate::BitReader;
#[doc = "Field `LEV18` reader - Level 18"]
pub type Lev18R = crate::BitReader;
#[doc = "Field `LEV19` reader - Level 19"]
pub type Lev19R = crate::BitReader;
#[doc = "Field `LEV20` reader - Level 20"]
pub type Lev20R = crate::BitReader;
#[doc = "Field `LEV21` reader - Level 21"]
pub type Lev21R = crate::BitReader;
#[doc = "Field `LEV22` reader - Level 22"]
pub type Lev22R = crate::BitReader;
#[doc = "Field `LEV23` reader - Level 23"]
pub type Lev23R = crate::BitReader;
#[doc = "Field `LEV24` reader - Level 24"]
pub type Lev24R = crate::BitReader;
#[doc = "Field `LEV25` reader - Level 25"]
pub type Lev25R = crate::BitReader;
#[doc = "Field `LEV26` reader - Level 26"]
pub type Lev26R = crate::BitReader;
#[doc = "Field `LEV27` reader - Level 27"]
pub type Lev27R = crate::BitReader;
#[doc = "Field `LEV28` reader - Level 28"]
pub type Lev28R = crate::BitReader;
#[doc = "Field `LEV29` reader - Level 29"]
pub type Lev29R = crate::BitReader;
#[doc = "Field `LEV30` reader - Level 30"]
pub type Lev30R = crate::BitReader;
#[doc = "Field `LEV31` reader - Level 31"]
pub type Lev31R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Level 0"]
    #[inline(always)]
    pub fn lev0(&self) -> Lev0R {
        Lev0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Level 1"]
    #[inline(always)]
    pub fn lev1(&self) -> Lev1R {
        Lev1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Level 2"]
    #[inline(always)]
    pub fn lev2(&self) -> Lev2R {
        Lev2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Level 3"]
    #[inline(always)]
    pub fn lev3(&self) -> Lev3R {
        Lev3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Level 4"]
    #[inline(always)]
    pub fn lev4(&self) -> Lev4R {
        Lev4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Level 5"]
    #[inline(always)]
    pub fn lev5(&self) -> Lev5R {
        Lev5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Level 6"]
    #[inline(always)]
    pub fn lev6(&self) -> Lev6R {
        Lev6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Level 7"]
    #[inline(always)]
    pub fn lev7(&self) -> Lev7R {
        Lev7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Level 8"]
    #[inline(always)]
    pub fn lev8(&self) -> Lev8R {
        Lev8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Level 9"]
    #[inline(always)]
    pub fn lev9(&self) -> Lev9R {
        Lev9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Level 10"]
    #[inline(always)]
    pub fn lev10(&self) -> Lev10R {
        Lev10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Level 11"]
    #[inline(always)]
    pub fn lev11(&self) -> Lev11R {
        Lev11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Level 12"]
    #[inline(always)]
    pub fn lev12(&self) -> Lev12R {
        Lev12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Level 13"]
    #[inline(always)]
    pub fn lev13(&self) -> Lev13R {
        Lev13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Level 14"]
    #[inline(always)]
    pub fn lev14(&self) -> Lev14R {
        Lev14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Level 15"]
    #[inline(always)]
    pub fn lev15(&self) -> Lev15R {
        Lev15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Level 16"]
    #[inline(always)]
    pub fn lev16(&self) -> Lev16R {
        Lev16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Level 17"]
    #[inline(always)]
    pub fn lev17(&self) -> Lev17R {
        Lev17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Level 18"]
    #[inline(always)]
    pub fn lev18(&self) -> Lev18R {
        Lev18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Level 19"]
    #[inline(always)]
    pub fn lev19(&self) -> Lev19R {
        Lev19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Level 20"]
    #[inline(always)]
    pub fn lev20(&self) -> Lev20R {
        Lev20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Level 21"]
    #[inline(always)]
    pub fn lev21(&self) -> Lev21R {
        Lev21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Level 22"]
    #[inline(always)]
    pub fn lev22(&self) -> Lev22R {
        Lev22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Level 23"]
    #[inline(always)]
    pub fn lev23(&self) -> Lev23R {
        Lev23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Level 24"]
    #[inline(always)]
    pub fn lev24(&self) -> Lev24R {
        Lev24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Level 25"]
    #[inline(always)]
    pub fn lev25(&self) -> Lev25R {
        Lev25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Level 26"]
    #[inline(always)]
    pub fn lev26(&self) -> Lev26R {
        Lev26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Level 27"]
    #[inline(always)]
    pub fn lev27(&self) -> Lev27R {
        Lev27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Level 28"]
    #[inline(always)]
    pub fn lev28(&self) -> Lev28R {
        Lev28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Level 29"]
    #[inline(always)]
    pub fn lev29(&self) -> Lev29R {
        Lev29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Level 30"]
    #[inline(always)]
    pub fn lev30(&self) -> Lev30R {
        Lev30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Level 31"]
    #[inline(always)]
    pub fn lev31(&self) -> Lev31R {
        Lev31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPLEV0")
            .field("lev0", &self.lev0())
            .field("lev1", &self.lev1())
            .field("lev2", &self.lev2())
            .field("lev3", &self.lev3())
            .field("lev4", &self.lev4())
            .field("lev5", &self.lev5())
            .field("lev6", &self.lev6())
            .field("lev7", &self.lev7())
            .field("lev8", &self.lev8())
            .field("lev9", &self.lev9())
            .field("lev10", &self.lev10())
            .field("lev11", &self.lev11())
            .field("lev12", &self.lev12())
            .field("lev13", &self.lev13())
            .field("lev14", &self.lev14())
            .field("lev15", &self.lev15())
            .field("lev16", &self.lev16())
            .field("lev17", &self.lev17())
            .field("lev18", &self.lev18())
            .field("lev19", &self.lev19())
            .field("lev20", &self.lev20())
            .field("lev21", &self.lev21())
            .field("lev22", &self.lev22())
            .field("lev23", &self.lev23())
            .field("lev24", &self.lev24())
            .field("lev25", &self.lev25())
            .field("lev26", &self.lev26())
            .field("lev27", &self.lev27())
            .field("lev28", &self.lev28())
            .field("lev29", &self.lev29())
            .field("lev30", &self.lev30())
            .field("lev31", &self.lev31())
            .finish()
    }
}
#[doc = "GPIO Pin Level 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gplev0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gplev0Spec;
impl crate::RegisterSpec for Gplev0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gplev0::R`](R) reader structure"]
impl crate::Readable for Gplev0Spec {}

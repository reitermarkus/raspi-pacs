#[doc = "Register `GPLEV1` reader"]
pub type R = crate::R<Gplev1Spec>;
#[doc = "Field `LEV32` reader - Level 32"]
pub type Lev32R = crate::BitReader;
#[doc = "Field `LEV33` reader - Level 33"]
pub type Lev33R = crate::BitReader;
#[doc = "Field `LEV34` reader - Level 34"]
pub type Lev34R = crate::BitReader;
#[doc = "Field `LEV35` reader - Level 35"]
pub type Lev35R = crate::BitReader;
#[doc = "Field `LEV36` reader - Level 36"]
pub type Lev36R = crate::BitReader;
#[doc = "Field `LEV37` reader - Level 37"]
pub type Lev37R = crate::BitReader;
#[doc = "Field `LEV38` reader - Level 38"]
pub type Lev38R = crate::BitReader;
#[doc = "Field `LEV39` reader - Level 39"]
pub type Lev39R = crate::BitReader;
#[doc = "Field `LEV40` reader - Level 40"]
pub type Lev40R = crate::BitReader;
#[doc = "Field `LEV41` reader - Level 41"]
pub type Lev41R = crate::BitReader;
#[doc = "Field `LEV42` reader - Level 42"]
pub type Lev42R = crate::BitReader;
#[doc = "Field `LEV43` reader - Level 43"]
pub type Lev43R = crate::BitReader;
#[doc = "Field `LEV44` reader - Level 44"]
pub type Lev44R = crate::BitReader;
#[doc = "Field `LEV45` reader - Level 45"]
pub type Lev45R = crate::BitReader;
#[doc = "Field `LEV46` reader - Level 46"]
pub type Lev46R = crate::BitReader;
#[doc = "Field `LEV47` reader - Level 47"]
pub type Lev47R = crate::BitReader;
#[doc = "Field `LEV48` reader - Level 48"]
pub type Lev48R = crate::BitReader;
#[doc = "Field `LEV49` reader - Level 49"]
pub type Lev49R = crate::BitReader;
#[doc = "Field `LEV50` reader - Level 50"]
pub type Lev50R = crate::BitReader;
#[doc = "Field `LEV51` reader - Level 51"]
pub type Lev51R = crate::BitReader;
#[doc = "Field `LEV52` reader - Level 52"]
pub type Lev52R = crate::BitReader;
#[doc = "Field `LEV53` reader - Level 53"]
pub type Lev53R = crate::BitReader;
#[doc = "Field `LEV54` reader - Level 54"]
pub type Lev54R = crate::BitReader;
#[doc = "Field `LEV55` reader - Level 55"]
pub type Lev55R = crate::BitReader;
#[doc = "Field `LEV56` reader - Level 56"]
pub type Lev56R = crate::BitReader;
#[doc = "Field `LEV57` reader - Level 57"]
pub type Lev57R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Level 32"]
    #[inline(always)]
    pub fn lev32(&self) -> Lev32R {
        Lev32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Level 33"]
    #[inline(always)]
    pub fn lev33(&self) -> Lev33R {
        Lev33R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Level 34"]
    #[inline(always)]
    pub fn lev34(&self) -> Lev34R {
        Lev34R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Level 35"]
    #[inline(always)]
    pub fn lev35(&self) -> Lev35R {
        Lev35R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Level 36"]
    #[inline(always)]
    pub fn lev36(&self) -> Lev36R {
        Lev36R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Level 37"]
    #[inline(always)]
    pub fn lev37(&self) -> Lev37R {
        Lev37R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Level 38"]
    #[inline(always)]
    pub fn lev38(&self) -> Lev38R {
        Lev38R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Level 39"]
    #[inline(always)]
    pub fn lev39(&self) -> Lev39R {
        Lev39R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Level 40"]
    #[inline(always)]
    pub fn lev40(&self) -> Lev40R {
        Lev40R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Level 41"]
    #[inline(always)]
    pub fn lev41(&self) -> Lev41R {
        Lev41R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Level 42"]
    #[inline(always)]
    pub fn lev42(&self) -> Lev42R {
        Lev42R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Level 43"]
    #[inline(always)]
    pub fn lev43(&self) -> Lev43R {
        Lev43R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Level 44"]
    #[inline(always)]
    pub fn lev44(&self) -> Lev44R {
        Lev44R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Level 45"]
    #[inline(always)]
    pub fn lev45(&self) -> Lev45R {
        Lev45R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Level 46"]
    #[inline(always)]
    pub fn lev46(&self) -> Lev46R {
        Lev46R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Level 47"]
    #[inline(always)]
    pub fn lev47(&self) -> Lev47R {
        Lev47R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Level 48"]
    #[inline(always)]
    pub fn lev48(&self) -> Lev48R {
        Lev48R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Level 49"]
    #[inline(always)]
    pub fn lev49(&self) -> Lev49R {
        Lev49R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Level 50"]
    #[inline(always)]
    pub fn lev50(&self) -> Lev50R {
        Lev50R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Level 51"]
    #[inline(always)]
    pub fn lev51(&self) -> Lev51R {
        Lev51R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Level 52"]
    #[inline(always)]
    pub fn lev52(&self) -> Lev52R {
        Lev52R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Level 53"]
    #[inline(always)]
    pub fn lev53(&self) -> Lev53R {
        Lev53R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Level 54"]
    #[inline(always)]
    pub fn lev54(&self) -> Lev54R {
        Lev54R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Level 55"]
    #[inline(always)]
    pub fn lev55(&self) -> Lev55R {
        Lev55R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Level 56"]
    #[inline(always)]
    pub fn lev56(&self) -> Lev56R {
        Lev56R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Level 57"]
    #[inline(always)]
    pub fn lev57(&self) -> Lev57R {
        Lev57R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPLEV1")
            .field("lev32", &self.lev32())
            .field("lev33", &self.lev33())
            .field("lev34", &self.lev34())
            .field("lev35", &self.lev35())
            .field("lev36", &self.lev36())
            .field("lev37", &self.lev37())
            .field("lev38", &self.lev38())
            .field("lev39", &self.lev39())
            .field("lev40", &self.lev40())
            .field("lev41", &self.lev41())
            .field("lev42", &self.lev42())
            .field("lev43", &self.lev43())
            .field("lev44", &self.lev44())
            .field("lev45", &self.lev45())
            .field("lev46", &self.lev46())
            .field("lev47", &self.lev47())
            .field("lev48", &self.lev48())
            .field("lev49", &self.lev49())
            .field("lev50", &self.lev50())
            .field("lev51", &self.lev51())
            .field("lev52", &self.lev52())
            .field("lev53", &self.lev53())
            .field("lev54", &self.lev54())
            .field("lev55", &self.lev55())
            .field("lev56", &self.lev56())
            .field("lev57", &self.lev57())
            .finish()
    }
}
#[doc = "GPIO Pin Level 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gplev1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gplev1Spec;
impl crate::RegisterSpec for Gplev1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gplev1::R`](R) reader structure"]
impl crate::Readable for Gplev1Spec {}

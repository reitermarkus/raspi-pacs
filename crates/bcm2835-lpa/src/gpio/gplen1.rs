#[doc = "Register `GPLEN1` reader"]
pub type R = crate::R<Gplen1Spec>;
#[doc = "Register `GPLEN1` writer"]
pub type W = crate::W<Gplen1Spec>;
#[doc = "Field `LEN32` reader - Low detect enabled 32"]
pub type Len32R = crate::BitReader;
#[doc = "Field `LEN32` writer - Low detect enabled 32"]
pub type Len32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN33` reader - Low detect enabled 33"]
pub type Len33R = crate::BitReader;
#[doc = "Field `LEN33` writer - Low detect enabled 33"]
pub type Len33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN34` reader - Low detect enabled 34"]
pub type Len34R = crate::BitReader;
#[doc = "Field `LEN34` writer - Low detect enabled 34"]
pub type Len34W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN35` reader - Low detect enabled 35"]
pub type Len35R = crate::BitReader;
#[doc = "Field `LEN35` writer - Low detect enabled 35"]
pub type Len35W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN36` reader - Low detect enabled 36"]
pub type Len36R = crate::BitReader;
#[doc = "Field `LEN36` writer - Low detect enabled 36"]
pub type Len36W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN37` reader - Low detect enabled 37"]
pub type Len37R = crate::BitReader;
#[doc = "Field `LEN37` writer - Low detect enabled 37"]
pub type Len37W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN38` reader - Low detect enabled 38"]
pub type Len38R = crate::BitReader;
#[doc = "Field `LEN38` writer - Low detect enabled 38"]
pub type Len38W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN39` reader - Low detect enabled 39"]
pub type Len39R = crate::BitReader;
#[doc = "Field `LEN39` writer - Low detect enabled 39"]
pub type Len39W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN40` reader - Low detect enabled 40"]
pub type Len40R = crate::BitReader;
#[doc = "Field `LEN40` writer - Low detect enabled 40"]
pub type Len40W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN41` reader - Low detect enabled 41"]
pub type Len41R = crate::BitReader;
#[doc = "Field `LEN41` writer - Low detect enabled 41"]
pub type Len41W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN42` reader - Low detect enabled 42"]
pub type Len42R = crate::BitReader;
#[doc = "Field `LEN42` writer - Low detect enabled 42"]
pub type Len42W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN43` reader - Low detect enabled 43"]
pub type Len43R = crate::BitReader;
#[doc = "Field `LEN43` writer - Low detect enabled 43"]
pub type Len43W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN44` reader - Low detect enabled 44"]
pub type Len44R = crate::BitReader;
#[doc = "Field `LEN44` writer - Low detect enabled 44"]
pub type Len44W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN45` reader - Low detect enabled 45"]
pub type Len45R = crate::BitReader;
#[doc = "Field `LEN45` writer - Low detect enabled 45"]
pub type Len45W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN46` reader - Low detect enabled 46"]
pub type Len46R = crate::BitReader;
#[doc = "Field `LEN46` writer - Low detect enabled 46"]
pub type Len46W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN47` reader - Low detect enabled 47"]
pub type Len47R = crate::BitReader;
#[doc = "Field `LEN47` writer - Low detect enabled 47"]
pub type Len47W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN48` reader - Low detect enabled 48"]
pub type Len48R = crate::BitReader;
#[doc = "Field `LEN48` writer - Low detect enabled 48"]
pub type Len48W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN49` reader - Low detect enabled 49"]
pub type Len49R = crate::BitReader;
#[doc = "Field `LEN49` writer - Low detect enabled 49"]
pub type Len49W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN50` reader - Low detect enabled 50"]
pub type Len50R = crate::BitReader;
#[doc = "Field `LEN50` writer - Low detect enabled 50"]
pub type Len50W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN51` reader - Low detect enabled 51"]
pub type Len51R = crate::BitReader;
#[doc = "Field `LEN51` writer - Low detect enabled 51"]
pub type Len51W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN52` reader - Low detect enabled 52"]
pub type Len52R = crate::BitReader;
#[doc = "Field `LEN52` writer - Low detect enabled 52"]
pub type Len52W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN53` reader - Low detect enabled 53"]
pub type Len53R = crate::BitReader;
#[doc = "Field `LEN53` writer - Low detect enabled 53"]
pub type Len53W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Low detect enabled 32"]
    #[inline(always)]
    pub fn len32(&self) -> Len32R {
        Len32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low detect enabled 33"]
    #[inline(always)]
    pub fn len33(&self) -> Len33R {
        Len33R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low detect enabled 34"]
    #[inline(always)]
    pub fn len34(&self) -> Len34R {
        Len34R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Low detect enabled 35"]
    #[inline(always)]
    pub fn len35(&self) -> Len35R {
        Len35R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Low detect enabled 36"]
    #[inline(always)]
    pub fn len36(&self) -> Len36R {
        Len36R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Low detect enabled 37"]
    #[inline(always)]
    pub fn len37(&self) -> Len37R {
        Len37R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Low detect enabled 38"]
    #[inline(always)]
    pub fn len38(&self) -> Len38R {
        Len38R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Low detect enabled 39"]
    #[inline(always)]
    pub fn len39(&self) -> Len39R {
        Len39R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Low detect enabled 40"]
    #[inline(always)]
    pub fn len40(&self) -> Len40R {
        Len40R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Low detect enabled 41"]
    #[inline(always)]
    pub fn len41(&self) -> Len41R {
        Len41R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Low detect enabled 42"]
    #[inline(always)]
    pub fn len42(&self) -> Len42R {
        Len42R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Low detect enabled 43"]
    #[inline(always)]
    pub fn len43(&self) -> Len43R {
        Len43R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Low detect enabled 44"]
    #[inline(always)]
    pub fn len44(&self) -> Len44R {
        Len44R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Low detect enabled 45"]
    #[inline(always)]
    pub fn len45(&self) -> Len45R {
        Len45R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Low detect enabled 46"]
    #[inline(always)]
    pub fn len46(&self) -> Len46R {
        Len46R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Low detect enabled 47"]
    #[inline(always)]
    pub fn len47(&self) -> Len47R {
        Len47R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Low detect enabled 48"]
    #[inline(always)]
    pub fn len48(&self) -> Len48R {
        Len48R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Low detect enabled 49"]
    #[inline(always)]
    pub fn len49(&self) -> Len49R {
        Len49R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Low detect enabled 50"]
    #[inline(always)]
    pub fn len50(&self) -> Len50R {
        Len50R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Low detect enabled 51"]
    #[inline(always)]
    pub fn len51(&self) -> Len51R {
        Len51R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Low detect enabled 52"]
    #[inline(always)]
    pub fn len52(&self) -> Len52R {
        Len52R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Low detect enabled 53"]
    #[inline(always)]
    pub fn len53(&self) -> Len53R {
        Len53R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPLEN1")
            .field("len32", &self.len32())
            .field("len33", &self.len33())
            .field("len34", &self.len34())
            .field("len35", &self.len35())
            .field("len36", &self.len36())
            .field("len37", &self.len37())
            .field("len38", &self.len38())
            .field("len39", &self.len39())
            .field("len40", &self.len40())
            .field("len41", &self.len41())
            .field("len42", &self.len42())
            .field("len43", &self.len43())
            .field("len44", &self.len44())
            .field("len45", &self.len45())
            .field("len46", &self.len46())
            .field("len47", &self.len47())
            .field("len48", &self.len48())
            .field("len49", &self.len49())
            .field("len50", &self.len50())
            .field("len51", &self.len51())
            .field("len52", &self.len52())
            .field("len53", &self.len53())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Low detect enabled 32"]
    #[inline(always)]
    #[must_use]
    pub fn len32(&mut self) -> Len32W<Gplen1Spec> {
        Len32W::new(self, 0)
    }
    #[doc = "Bit 1 - Low detect enabled 33"]
    #[inline(always)]
    #[must_use]
    pub fn len33(&mut self) -> Len33W<Gplen1Spec> {
        Len33W::new(self, 1)
    }
    #[doc = "Bit 2 - Low detect enabled 34"]
    #[inline(always)]
    #[must_use]
    pub fn len34(&mut self) -> Len34W<Gplen1Spec> {
        Len34W::new(self, 2)
    }
    #[doc = "Bit 3 - Low detect enabled 35"]
    #[inline(always)]
    #[must_use]
    pub fn len35(&mut self) -> Len35W<Gplen1Spec> {
        Len35W::new(self, 3)
    }
    #[doc = "Bit 4 - Low detect enabled 36"]
    #[inline(always)]
    #[must_use]
    pub fn len36(&mut self) -> Len36W<Gplen1Spec> {
        Len36W::new(self, 4)
    }
    #[doc = "Bit 5 - Low detect enabled 37"]
    #[inline(always)]
    #[must_use]
    pub fn len37(&mut self) -> Len37W<Gplen1Spec> {
        Len37W::new(self, 5)
    }
    #[doc = "Bit 6 - Low detect enabled 38"]
    #[inline(always)]
    #[must_use]
    pub fn len38(&mut self) -> Len38W<Gplen1Spec> {
        Len38W::new(self, 6)
    }
    #[doc = "Bit 7 - Low detect enabled 39"]
    #[inline(always)]
    #[must_use]
    pub fn len39(&mut self) -> Len39W<Gplen1Spec> {
        Len39W::new(self, 7)
    }
    #[doc = "Bit 8 - Low detect enabled 40"]
    #[inline(always)]
    #[must_use]
    pub fn len40(&mut self) -> Len40W<Gplen1Spec> {
        Len40W::new(self, 8)
    }
    #[doc = "Bit 9 - Low detect enabled 41"]
    #[inline(always)]
    #[must_use]
    pub fn len41(&mut self) -> Len41W<Gplen1Spec> {
        Len41W::new(self, 9)
    }
    #[doc = "Bit 10 - Low detect enabled 42"]
    #[inline(always)]
    #[must_use]
    pub fn len42(&mut self) -> Len42W<Gplen1Spec> {
        Len42W::new(self, 10)
    }
    #[doc = "Bit 11 - Low detect enabled 43"]
    #[inline(always)]
    #[must_use]
    pub fn len43(&mut self) -> Len43W<Gplen1Spec> {
        Len43W::new(self, 11)
    }
    #[doc = "Bit 12 - Low detect enabled 44"]
    #[inline(always)]
    #[must_use]
    pub fn len44(&mut self) -> Len44W<Gplen1Spec> {
        Len44W::new(self, 12)
    }
    #[doc = "Bit 13 - Low detect enabled 45"]
    #[inline(always)]
    #[must_use]
    pub fn len45(&mut self) -> Len45W<Gplen1Spec> {
        Len45W::new(self, 13)
    }
    #[doc = "Bit 14 - Low detect enabled 46"]
    #[inline(always)]
    #[must_use]
    pub fn len46(&mut self) -> Len46W<Gplen1Spec> {
        Len46W::new(self, 14)
    }
    #[doc = "Bit 15 - Low detect enabled 47"]
    #[inline(always)]
    #[must_use]
    pub fn len47(&mut self) -> Len47W<Gplen1Spec> {
        Len47W::new(self, 15)
    }
    #[doc = "Bit 16 - Low detect enabled 48"]
    #[inline(always)]
    #[must_use]
    pub fn len48(&mut self) -> Len48W<Gplen1Spec> {
        Len48W::new(self, 16)
    }
    #[doc = "Bit 17 - Low detect enabled 49"]
    #[inline(always)]
    #[must_use]
    pub fn len49(&mut self) -> Len49W<Gplen1Spec> {
        Len49W::new(self, 17)
    }
    #[doc = "Bit 18 - Low detect enabled 50"]
    #[inline(always)]
    #[must_use]
    pub fn len50(&mut self) -> Len50W<Gplen1Spec> {
        Len50W::new(self, 18)
    }
    #[doc = "Bit 19 - Low detect enabled 51"]
    #[inline(always)]
    #[must_use]
    pub fn len51(&mut self) -> Len51W<Gplen1Spec> {
        Len51W::new(self, 19)
    }
    #[doc = "Bit 20 - Low detect enabled 52"]
    #[inline(always)]
    #[must_use]
    pub fn len52(&mut self) -> Len52W<Gplen1Spec> {
        Len52W::new(self, 20)
    }
    #[doc = "Bit 21 - Low detect enabled 53"]
    #[inline(always)]
    #[must_use]
    pub fn len53(&mut self) -> Len53W<Gplen1Spec> {
        Len53W::new(self, 21)
    }
}
#[doc = "GPIO Pin Low Detect Enable 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gplen1::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gplen1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gplen1Spec;
impl crate::RegisterSpec for Gplen1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gplen1::R`](R) reader structure"]
impl crate::Readable for Gplen1Spec {}
#[doc = "`write(|w| ..)` method takes [`gplen1::W`](W) writer structure"]
impl crate::Writable for Gplen1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}

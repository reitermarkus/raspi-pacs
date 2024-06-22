#[doc = "Register `GPAREN1` reader"]
pub type R = crate::R<Gparen1Spec>;
#[doc = "Register `GPAREN1` writer"]
pub type W = crate::W<Gparen1Spec>;
#[doc = "Field `AREN32` reader - Async rising enabled 32"]
pub type Aren32R = crate::BitReader;
#[doc = "Field `AREN32` writer - Async rising enabled 32"]
pub type Aren32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AREN33` reader - Async rising enabled 33"]
pub type Aren33R = crate::BitReader;
#[doc = "Field `AREN33` writer - Async rising enabled 33"]
pub type Aren33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AREN34` reader - Async rising enabled 34"]
pub type Aren34R = crate::BitReader;
#[doc = "Field `AREN34` writer - Async rising enabled 34"]
pub type Aren34W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AREN35` reader - Async rising enabled 35"]
pub type Aren35R = crate::BitReader;
#[doc = "Field `AREN35` writer - Async rising enabled 35"]
pub type Aren35W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AREN36` reader - Async rising enabled 36"]
pub type Aren36R = crate::BitReader;
#[doc = "Field `AREN36` writer - Async rising enabled 36"]
pub type Aren36W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AREN37` reader - Async rising enabled 37"]
pub type Aren37R = crate::BitReader;
#[doc = "Field `AREN37` writer - Async rising enabled 37"]
pub type Aren37W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AREN38` reader - Async rising enabled 38"]
pub type Aren38R = crate::BitReader;
#[doc = "Field `AREN38` writer - Async rising enabled 38"]
pub type Aren38W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AREN39` reader - Async rising enabled 39"]
pub type Aren39R = crate::BitReader;
#[doc = "Field `AREN39` writer - Async rising enabled 39"]
pub type Aren39W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AREN40` reader - Async rising enabled 40"]
pub type Aren40R = crate::BitReader;
#[doc = "Field `AREN40` writer - Async rising enabled 40"]
pub type Aren40W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AREN41` reader - Async rising enabled 41"]
pub type Aren41R = crate::BitReader;
#[doc = "Field `AREN41` writer - Async rising enabled 41"]
pub type Aren41W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AREN42` reader - Async rising enabled 42"]
pub type Aren42R = crate::BitReader;
#[doc = "Field `AREN42` writer - Async rising enabled 42"]
pub type Aren42W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AREN43` reader - Async rising enabled 43"]
pub type Aren43R = crate::BitReader;
#[doc = "Field `AREN43` writer - Async rising enabled 43"]
pub type Aren43W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AREN44` reader - Async rising enabled 44"]
pub type Aren44R = crate::BitReader;
#[doc = "Field `AREN44` writer - Async rising enabled 44"]
pub type Aren44W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AREN45` reader - Async rising enabled 45"]
pub type Aren45R = crate::BitReader;
#[doc = "Field `AREN45` writer - Async rising enabled 45"]
pub type Aren45W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AREN46` reader - Async rising enabled 46"]
pub type Aren46R = crate::BitReader;
#[doc = "Field `AREN46` writer - Async rising enabled 46"]
pub type Aren46W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AREN47` reader - Async rising enabled 47"]
pub type Aren47R = crate::BitReader;
#[doc = "Field `AREN47` writer - Async rising enabled 47"]
pub type Aren47W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AREN48` reader - Async rising enabled 48"]
pub type Aren48R = crate::BitReader;
#[doc = "Field `AREN48` writer - Async rising enabled 48"]
pub type Aren48W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AREN49` reader - Async rising enabled 49"]
pub type Aren49R = crate::BitReader;
#[doc = "Field `AREN49` writer - Async rising enabled 49"]
pub type Aren49W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AREN50` reader - Async rising enabled 50"]
pub type Aren50R = crate::BitReader;
#[doc = "Field `AREN50` writer - Async rising enabled 50"]
pub type Aren50W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AREN51` reader - Async rising enabled 51"]
pub type Aren51R = crate::BitReader;
#[doc = "Field `AREN51` writer - Async rising enabled 51"]
pub type Aren51W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AREN52` reader - Async rising enabled 52"]
pub type Aren52R = crate::BitReader;
#[doc = "Field `AREN52` writer - Async rising enabled 52"]
pub type Aren52W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AREN53` reader - Async rising enabled 53"]
pub type Aren53R = crate::BitReader;
#[doc = "Field `AREN53` writer - Async rising enabled 53"]
pub type Aren53W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Async rising enabled 32"]
    #[inline(always)]
    pub fn aren32(&self) -> Aren32R {
        Aren32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Async rising enabled 33"]
    #[inline(always)]
    pub fn aren33(&self) -> Aren33R {
        Aren33R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Async rising enabled 34"]
    #[inline(always)]
    pub fn aren34(&self) -> Aren34R {
        Aren34R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Async rising enabled 35"]
    #[inline(always)]
    pub fn aren35(&self) -> Aren35R {
        Aren35R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Async rising enabled 36"]
    #[inline(always)]
    pub fn aren36(&self) -> Aren36R {
        Aren36R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Async rising enabled 37"]
    #[inline(always)]
    pub fn aren37(&self) -> Aren37R {
        Aren37R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Async rising enabled 38"]
    #[inline(always)]
    pub fn aren38(&self) -> Aren38R {
        Aren38R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Async rising enabled 39"]
    #[inline(always)]
    pub fn aren39(&self) -> Aren39R {
        Aren39R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Async rising enabled 40"]
    #[inline(always)]
    pub fn aren40(&self) -> Aren40R {
        Aren40R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Async rising enabled 41"]
    #[inline(always)]
    pub fn aren41(&self) -> Aren41R {
        Aren41R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Async rising enabled 42"]
    #[inline(always)]
    pub fn aren42(&self) -> Aren42R {
        Aren42R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Async rising enabled 43"]
    #[inline(always)]
    pub fn aren43(&self) -> Aren43R {
        Aren43R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Async rising enabled 44"]
    #[inline(always)]
    pub fn aren44(&self) -> Aren44R {
        Aren44R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Async rising enabled 45"]
    #[inline(always)]
    pub fn aren45(&self) -> Aren45R {
        Aren45R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Async rising enabled 46"]
    #[inline(always)]
    pub fn aren46(&self) -> Aren46R {
        Aren46R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Async rising enabled 47"]
    #[inline(always)]
    pub fn aren47(&self) -> Aren47R {
        Aren47R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Async rising enabled 48"]
    #[inline(always)]
    pub fn aren48(&self) -> Aren48R {
        Aren48R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Async rising enabled 49"]
    #[inline(always)]
    pub fn aren49(&self) -> Aren49R {
        Aren49R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Async rising enabled 50"]
    #[inline(always)]
    pub fn aren50(&self) -> Aren50R {
        Aren50R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Async rising enabled 51"]
    #[inline(always)]
    pub fn aren51(&self) -> Aren51R {
        Aren51R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Async rising enabled 52"]
    #[inline(always)]
    pub fn aren52(&self) -> Aren52R {
        Aren52R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Async rising enabled 53"]
    #[inline(always)]
    pub fn aren53(&self) -> Aren53R {
        Aren53R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPAREN1")
            .field("aren32", &self.aren32())
            .field("aren33", &self.aren33())
            .field("aren34", &self.aren34())
            .field("aren35", &self.aren35())
            .field("aren36", &self.aren36())
            .field("aren37", &self.aren37())
            .field("aren38", &self.aren38())
            .field("aren39", &self.aren39())
            .field("aren40", &self.aren40())
            .field("aren41", &self.aren41())
            .field("aren42", &self.aren42())
            .field("aren43", &self.aren43())
            .field("aren44", &self.aren44())
            .field("aren45", &self.aren45())
            .field("aren46", &self.aren46())
            .field("aren47", &self.aren47())
            .field("aren48", &self.aren48())
            .field("aren49", &self.aren49())
            .field("aren50", &self.aren50())
            .field("aren51", &self.aren51())
            .field("aren52", &self.aren52())
            .field("aren53", &self.aren53())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Async rising enabled 32"]
    #[inline(always)]
    #[must_use]
    pub fn aren32(&mut self) -> Aren32W<Gparen1Spec> {
        Aren32W::new(self, 0)
    }
    #[doc = "Bit 1 - Async rising enabled 33"]
    #[inline(always)]
    #[must_use]
    pub fn aren33(&mut self) -> Aren33W<Gparen1Spec> {
        Aren33W::new(self, 1)
    }
    #[doc = "Bit 2 - Async rising enabled 34"]
    #[inline(always)]
    #[must_use]
    pub fn aren34(&mut self) -> Aren34W<Gparen1Spec> {
        Aren34W::new(self, 2)
    }
    #[doc = "Bit 3 - Async rising enabled 35"]
    #[inline(always)]
    #[must_use]
    pub fn aren35(&mut self) -> Aren35W<Gparen1Spec> {
        Aren35W::new(self, 3)
    }
    #[doc = "Bit 4 - Async rising enabled 36"]
    #[inline(always)]
    #[must_use]
    pub fn aren36(&mut self) -> Aren36W<Gparen1Spec> {
        Aren36W::new(self, 4)
    }
    #[doc = "Bit 5 - Async rising enabled 37"]
    #[inline(always)]
    #[must_use]
    pub fn aren37(&mut self) -> Aren37W<Gparen1Spec> {
        Aren37W::new(self, 5)
    }
    #[doc = "Bit 6 - Async rising enabled 38"]
    #[inline(always)]
    #[must_use]
    pub fn aren38(&mut self) -> Aren38W<Gparen1Spec> {
        Aren38W::new(self, 6)
    }
    #[doc = "Bit 7 - Async rising enabled 39"]
    #[inline(always)]
    #[must_use]
    pub fn aren39(&mut self) -> Aren39W<Gparen1Spec> {
        Aren39W::new(self, 7)
    }
    #[doc = "Bit 8 - Async rising enabled 40"]
    #[inline(always)]
    #[must_use]
    pub fn aren40(&mut self) -> Aren40W<Gparen1Spec> {
        Aren40W::new(self, 8)
    }
    #[doc = "Bit 9 - Async rising enabled 41"]
    #[inline(always)]
    #[must_use]
    pub fn aren41(&mut self) -> Aren41W<Gparen1Spec> {
        Aren41W::new(self, 9)
    }
    #[doc = "Bit 10 - Async rising enabled 42"]
    #[inline(always)]
    #[must_use]
    pub fn aren42(&mut self) -> Aren42W<Gparen1Spec> {
        Aren42W::new(self, 10)
    }
    #[doc = "Bit 11 - Async rising enabled 43"]
    #[inline(always)]
    #[must_use]
    pub fn aren43(&mut self) -> Aren43W<Gparen1Spec> {
        Aren43W::new(self, 11)
    }
    #[doc = "Bit 12 - Async rising enabled 44"]
    #[inline(always)]
    #[must_use]
    pub fn aren44(&mut self) -> Aren44W<Gparen1Spec> {
        Aren44W::new(self, 12)
    }
    #[doc = "Bit 13 - Async rising enabled 45"]
    #[inline(always)]
    #[must_use]
    pub fn aren45(&mut self) -> Aren45W<Gparen1Spec> {
        Aren45W::new(self, 13)
    }
    #[doc = "Bit 14 - Async rising enabled 46"]
    #[inline(always)]
    #[must_use]
    pub fn aren46(&mut self) -> Aren46W<Gparen1Spec> {
        Aren46W::new(self, 14)
    }
    #[doc = "Bit 15 - Async rising enabled 47"]
    #[inline(always)]
    #[must_use]
    pub fn aren47(&mut self) -> Aren47W<Gparen1Spec> {
        Aren47W::new(self, 15)
    }
    #[doc = "Bit 16 - Async rising enabled 48"]
    #[inline(always)]
    #[must_use]
    pub fn aren48(&mut self) -> Aren48W<Gparen1Spec> {
        Aren48W::new(self, 16)
    }
    #[doc = "Bit 17 - Async rising enabled 49"]
    #[inline(always)]
    #[must_use]
    pub fn aren49(&mut self) -> Aren49W<Gparen1Spec> {
        Aren49W::new(self, 17)
    }
    #[doc = "Bit 18 - Async rising enabled 50"]
    #[inline(always)]
    #[must_use]
    pub fn aren50(&mut self) -> Aren50W<Gparen1Spec> {
        Aren50W::new(self, 18)
    }
    #[doc = "Bit 19 - Async rising enabled 51"]
    #[inline(always)]
    #[must_use]
    pub fn aren51(&mut self) -> Aren51W<Gparen1Spec> {
        Aren51W::new(self, 19)
    }
    #[doc = "Bit 20 - Async rising enabled 52"]
    #[inline(always)]
    #[must_use]
    pub fn aren52(&mut self) -> Aren52W<Gparen1Spec> {
        Aren52W::new(self, 20)
    }
    #[doc = "Bit 21 - Async rising enabled 53"]
    #[inline(always)]
    #[must_use]
    pub fn aren53(&mut self) -> Aren53W<Gparen1Spec> {
        Aren53W::new(self, 21)
    }
}
#[doc = "GPIO Pin Async. Rising Edge Detect 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gparen1::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gparen1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gparen1Spec;
impl crate::RegisterSpec for Gparen1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gparen1::R`](R) reader structure"]
impl crate::Readable for Gparen1Spec {}
#[doc = "`write(|w| ..)` method takes [`gparen1::W`](W) writer structure"]
impl crate::Writable for Gparen1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}

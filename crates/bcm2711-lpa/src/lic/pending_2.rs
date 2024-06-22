#[doc = "Register `PENDING_2` reader"]
pub type R = crate::R<Pending2Spec>;
#[doc = "Field `INT32` reader - Interrupt 32"]
pub type Int32R = crate::BitReader;
#[doc = "Field `INT33` reader - Interrupt 33"]
pub type Int33R = crate::BitReader;
#[doc = "Field `INT34` reader - Interrupt 34"]
pub type Int34R = crate::BitReader;
#[doc = "Field `INT35` reader - Interrupt 35"]
pub type Int35R = crate::BitReader;
#[doc = "Field `INT36` reader - Interrupt 36"]
pub type Int36R = crate::BitReader;
#[doc = "Field `INT37` reader - Interrupt 37"]
pub type Int37R = crate::BitReader;
#[doc = "Field `INT38` reader - Interrupt 38"]
pub type Int38R = crate::BitReader;
#[doc = "Field `INT39` reader - Interrupt 39"]
pub type Int39R = crate::BitReader;
#[doc = "Field `INT40` reader - Interrupt 40"]
pub type Int40R = crate::BitReader;
#[doc = "Field `INT41` reader - Interrupt 41"]
pub type Int41R = crate::BitReader;
#[doc = "Field `INT42` reader - Interrupt 42"]
pub type Int42R = crate::BitReader;
#[doc = "Field `INT43` reader - Interrupt 43"]
pub type Int43R = crate::BitReader;
#[doc = "Field `INT44` reader - Interrupt 44"]
pub type Int44R = crate::BitReader;
#[doc = "Field `INT45` reader - Interrupt 45"]
pub type Int45R = crate::BitReader;
#[doc = "Field `INT46` reader - Interrupt 46"]
pub type Int46R = crate::BitReader;
#[doc = "Field `INT47` reader - Interrupt 47"]
pub type Int47R = crate::BitReader;
#[doc = "Field `INT48` reader - Interrupt 48"]
pub type Int48R = crate::BitReader;
#[doc = "Field `INT49` reader - Interrupt 49"]
pub type Int49R = crate::BitReader;
#[doc = "Field `INT50` reader - Interrupt 50"]
pub type Int50R = crate::BitReader;
#[doc = "Field `INT51` reader - Interrupt 51"]
pub type Int51R = crate::BitReader;
#[doc = "Field `INT52` reader - Interrupt 52"]
pub type Int52R = crate::BitReader;
#[doc = "Field `INT53` reader - Interrupt 53"]
pub type Int53R = crate::BitReader;
#[doc = "Field `INT54` reader - Interrupt 54"]
pub type Int54R = crate::BitReader;
#[doc = "Field `INT55` reader - Interrupt 55"]
pub type Int55R = crate::BitReader;
#[doc = "Field `INT56` reader - Interrupt 56"]
pub type Int56R = crate::BitReader;
#[doc = "Field `INT57` reader - Interrupt 57"]
pub type Int57R = crate::BitReader;
#[doc = "Field `INT58` reader - Interrupt 58"]
pub type Int58R = crate::BitReader;
#[doc = "Field `INT59` reader - Interrupt 59"]
pub type Int59R = crate::BitReader;
#[doc = "Field `INT60` reader - Interrupt 60"]
pub type Int60R = crate::BitReader;
#[doc = "Field `INT61` reader - Interrupt 61"]
pub type Int61R = crate::BitReader;
#[doc = "Field `INT62` reader - Interrupt 62"]
pub type Int62R = crate::BitReader;
#[doc = "Field `INT63` reader - Interrupt 63"]
pub type Int63R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt 32"]
    #[inline(always)]
    pub fn int32(&self) -> Int32R {
        Int32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt 33"]
    #[inline(always)]
    pub fn int33(&self) -> Int33R {
        Int33R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt 34"]
    #[inline(always)]
    pub fn int34(&self) -> Int34R {
        Int34R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt 35"]
    #[inline(always)]
    pub fn int35(&self) -> Int35R {
        Int35R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt 36"]
    #[inline(always)]
    pub fn int36(&self) -> Int36R {
        Int36R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt 37"]
    #[inline(always)]
    pub fn int37(&self) -> Int37R {
        Int37R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt 38"]
    #[inline(always)]
    pub fn int38(&self) -> Int38R {
        Int38R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt 39"]
    #[inline(always)]
    pub fn int39(&self) -> Int39R {
        Int39R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt 40"]
    #[inline(always)]
    pub fn int40(&self) -> Int40R {
        Int40R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt 41"]
    #[inline(always)]
    pub fn int41(&self) -> Int41R {
        Int41R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt 42"]
    #[inline(always)]
    pub fn int42(&self) -> Int42R {
        Int42R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt 43"]
    #[inline(always)]
    pub fn int43(&self) -> Int43R {
        Int43R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt 44"]
    #[inline(always)]
    pub fn int44(&self) -> Int44R {
        Int44R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt 45"]
    #[inline(always)]
    pub fn int45(&self) -> Int45R {
        Int45R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt 46"]
    #[inline(always)]
    pub fn int46(&self) -> Int46R {
        Int46R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt 47"]
    #[inline(always)]
    pub fn int47(&self) -> Int47R {
        Int47R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt 48"]
    #[inline(always)]
    pub fn int48(&self) -> Int48R {
        Int48R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt 49"]
    #[inline(always)]
    pub fn int49(&self) -> Int49R {
        Int49R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt 50"]
    #[inline(always)]
    pub fn int50(&self) -> Int50R {
        Int50R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt 51"]
    #[inline(always)]
    pub fn int51(&self) -> Int51R {
        Int51R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt 52"]
    #[inline(always)]
    pub fn int52(&self) -> Int52R {
        Int52R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt 53"]
    #[inline(always)]
    pub fn int53(&self) -> Int53R {
        Int53R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt 54"]
    #[inline(always)]
    pub fn int54(&self) -> Int54R {
        Int54R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt 55"]
    #[inline(always)]
    pub fn int55(&self) -> Int55R {
        Int55R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt 56"]
    #[inline(always)]
    pub fn int56(&self) -> Int56R {
        Int56R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt 57"]
    #[inline(always)]
    pub fn int57(&self) -> Int57R {
        Int57R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt 58"]
    #[inline(always)]
    pub fn int58(&self) -> Int58R {
        Int58R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt 59"]
    #[inline(always)]
    pub fn int59(&self) -> Int59R {
        Int59R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt 60"]
    #[inline(always)]
    pub fn int60(&self) -> Int60R {
        Int60R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt 61"]
    #[inline(always)]
    pub fn int61(&self) -> Int61R {
        Int61R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt 62"]
    #[inline(always)]
    pub fn int62(&self) -> Int62R {
        Int62R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt 63"]
    #[inline(always)]
    pub fn int63(&self) -> Int63R {
        Int63R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PENDING_2")
            .field("int32", &self.int32())
            .field("int33", &self.int33())
            .field("int34", &self.int34())
            .field("int35", &self.int35())
            .field("int36", &self.int36())
            .field("int37", &self.int37())
            .field("int38", &self.int38())
            .field("int39", &self.int39())
            .field("int40", &self.int40())
            .field("int41", &self.int41())
            .field("int42", &self.int42())
            .field("int43", &self.int43())
            .field("int44", &self.int44())
            .field("int45", &self.int45())
            .field("int46", &self.int46())
            .field("int47", &self.int47())
            .field("int48", &self.int48())
            .field("int49", &self.int49())
            .field("int50", &self.int50())
            .field("int51", &self.int51())
            .field("int52", &self.int52())
            .field("int53", &self.int53())
            .field("int54", &self.int54())
            .field("int55", &self.int55())
            .field("int56", &self.int56())
            .field("int57", &self.int57())
            .field("int58", &self.int58())
            .field("int59", &self.int59())
            .field("int60", &self.int60())
            .field("int61", &self.int61())
            .field("int62", &self.int62())
            .field("int63", &self.int63())
            .finish()
    }
}
#[doc = "Pending state for interrupts 32 - 63\n\nYou can [`read`](crate::Reg::read) this register and get [`pending_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pending2Spec;
impl crate::RegisterSpec for Pending2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pending_2::R`](R) reader structure"]
impl crate::Readable for Pending2Spec {}
#[doc = "`reset()` method sets PENDING_2 to value 0"]
impl crate::Resettable for Pending2Spec {
    const RESET_VALUE: u32 = 0;
}

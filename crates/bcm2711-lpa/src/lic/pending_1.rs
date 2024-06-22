#[doc = "Register `PENDING_1` reader"]
pub type R = crate::R<Pending1Spec>;
#[doc = "Field `INT0` reader - Interrupt 0"]
pub type Int0R = crate::BitReader;
#[doc = "Field `INT1` reader - Interrupt 1"]
pub type Int1R = crate::BitReader;
#[doc = "Field `INT2` reader - Interrupt 2"]
pub type Int2R = crate::BitReader;
#[doc = "Field `INT3` reader - Interrupt 3"]
pub type Int3R = crate::BitReader;
#[doc = "Field `INT4` reader - Interrupt 4"]
pub type Int4R = crate::BitReader;
#[doc = "Field `INT5` reader - Interrupt 5"]
pub type Int5R = crate::BitReader;
#[doc = "Field `INT6` reader - Interrupt 6"]
pub type Int6R = crate::BitReader;
#[doc = "Field `INT7` reader - Interrupt 7"]
pub type Int7R = crate::BitReader;
#[doc = "Field `INT8` reader - Interrupt 8"]
pub type Int8R = crate::BitReader;
#[doc = "Field `INT9` reader - Interrupt 9"]
pub type Int9R = crate::BitReader;
#[doc = "Field `INT10` reader - Interrupt 10"]
pub type Int10R = crate::BitReader;
#[doc = "Field `INT11` reader - Interrupt 11"]
pub type Int11R = crate::BitReader;
#[doc = "Field `INT12` reader - Interrupt 12"]
pub type Int12R = crate::BitReader;
#[doc = "Field `INT13` reader - Interrupt 13"]
pub type Int13R = crate::BitReader;
#[doc = "Field `INT14` reader - Interrupt 14"]
pub type Int14R = crate::BitReader;
#[doc = "Field `INT15` reader - Interrupt 15"]
pub type Int15R = crate::BitReader;
#[doc = "Field `INT16` reader - Interrupt 16"]
pub type Int16R = crate::BitReader;
#[doc = "Field `INT17` reader - Interrupt 17"]
pub type Int17R = crate::BitReader;
#[doc = "Field `INT18` reader - Interrupt 18"]
pub type Int18R = crate::BitReader;
#[doc = "Field `INT19` reader - Interrupt 19"]
pub type Int19R = crate::BitReader;
#[doc = "Field `INT20` reader - Interrupt 20"]
pub type Int20R = crate::BitReader;
#[doc = "Field `INT21` reader - Interrupt 21"]
pub type Int21R = crate::BitReader;
#[doc = "Field `INT22` reader - Interrupt 22"]
pub type Int22R = crate::BitReader;
#[doc = "Field `INT23` reader - Interrupt 23"]
pub type Int23R = crate::BitReader;
#[doc = "Field `INT24` reader - Interrupt 24"]
pub type Int24R = crate::BitReader;
#[doc = "Field `INT25` reader - Interrupt 25"]
pub type Int25R = crate::BitReader;
#[doc = "Field `INT26` reader - Interrupt 26"]
pub type Int26R = crate::BitReader;
#[doc = "Field `INT27` reader - Interrupt 27"]
pub type Int27R = crate::BitReader;
#[doc = "Field `INT28` reader - Interrupt 28"]
pub type Int28R = crate::BitReader;
#[doc = "Field `INT29` reader - Interrupt 29"]
pub type Int29R = crate::BitReader;
#[doc = "Field `INT30` reader - Interrupt 30"]
pub type Int30R = crate::BitReader;
#[doc = "Field `INT31` reader - Interrupt 31"]
pub type Int31R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt 0"]
    #[inline(always)]
    pub fn int0(&self) -> Int0R {
        Int0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt 1"]
    #[inline(always)]
    pub fn int1(&self) -> Int1R {
        Int1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt 2"]
    #[inline(always)]
    pub fn int2(&self) -> Int2R {
        Int2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt 3"]
    #[inline(always)]
    pub fn int3(&self) -> Int3R {
        Int3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt 4"]
    #[inline(always)]
    pub fn int4(&self) -> Int4R {
        Int4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt 5"]
    #[inline(always)]
    pub fn int5(&self) -> Int5R {
        Int5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt 6"]
    #[inline(always)]
    pub fn int6(&self) -> Int6R {
        Int6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt 7"]
    #[inline(always)]
    pub fn int7(&self) -> Int7R {
        Int7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt 8"]
    #[inline(always)]
    pub fn int8(&self) -> Int8R {
        Int8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt 9"]
    #[inline(always)]
    pub fn int9(&self) -> Int9R {
        Int9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt 10"]
    #[inline(always)]
    pub fn int10(&self) -> Int10R {
        Int10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt 11"]
    #[inline(always)]
    pub fn int11(&self) -> Int11R {
        Int11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt 12"]
    #[inline(always)]
    pub fn int12(&self) -> Int12R {
        Int12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt 13"]
    #[inline(always)]
    pub fn int13(&self) -> Int13R {
        Int13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt 14"]
    #[inline(always)]
    pub fn int14(&self) -> Int14R {
        Int14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt 15"]
    #[inline(always)]
    pub fn int15(&self) -> Int15R {
        Int15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt 16"]
    #[inline(always)]
    pub fn int16(&self) -> Int16R {
        Int16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt 17"]
    #[inline(always)]
    pub fn int17(&self) -> Int17R {
        Int17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt 18"]
    #[inline(always)]
    pub fn int18(&self) -> Int18R {
        Int18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt 19"]
    #[inline(always)]
    pub fn int19(&self) -> Int19R {
        Int19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt 20"]
    #[inline(always)]
    pub fn int20(&self) -> Int20R {
        Int20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt 21"]
    #[inline(always)]
    pub fn int21(&self) -> Int21R {
        Int21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt 22"]
    #[inline(always)]
    pub fn int22(&self) -> Int22R {
        Int22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt 23"]
    #[inline(always)]
    pub fn int23(&self) -> Int23R {
        Int23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt 24"]
    #[inline(always)]
    pub fn int24(&self) -> Int24R {
        Int24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt 25"]
    #[inline(always)]
    pub fn int25(&self) -> Int25R {
        Int25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt 26"]
    #[inline(always)]
    pub fn int26(&self) -> Int26R {
        Int26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt 27"]
    #[inline(always)]
    pub fn int27(&self) -> Int27R {
        Int27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt 28"]
    #[inline(always)]
    pub fn int28(&self) -> Int28R {
        Int28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt 29"]
    #[inline(always)]
    pub fn int29(&self) -> Int29R {
        Int29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt 30"]
    #[inline(always)]
    pub fn int30(&self) -> Int30R {
        Int30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt 31"]
    #[inline(always)]
    pub fn int31(&self) -> Int31R {
        Int31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PENDING_1")
            .field("int0", &self.int0())
            .field("int1", &self.int1())
            .field("int2", &self.int2())
            .field("int3", &self.int3())
            .field("int4", &self.int4())
            .field("int5", &self.int5())
            .field("int6", &self.int6())
            .field("int7", &self.int7())
            .field("int8", &self.int8())
            .field("int9", &self.int9())
            .field("int10", &self.int10())
            .field("int11", &self.int11())
            .field("int12", &self.int12())
            .field("int13", &self.int13())
            .field("int14", &self.int14())
            .field("int15", &self.int15())
            .field("int16", &self.int16())
            .field("int17", &self.int17())
            .field("int18", &self.int18())
            .field("int19", &self.int19())
            .field("int20", &self.int20())
            .field("int21", &self.int21())
            .field("int22", &self.int22())
            .field("int23", &self.int23())
            .field("int24", &self.int24())
            .field("int25", &self.int25())
            .field("int26", &self.int26())
            .field("int27", &self.int27())
            .field("int28", &self.int28())
            .field("int29", &self.int29())
            .field("int30", &self.int30())
            .field("int31", &self.int31())
            .finish()
    }
}
#[doc = "Pending state for interrupts 1 - 31\n\nYou can [`read`](crate::Reg::read) this register and get [`pending_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pending1Spec;
impl crate::RegisterSpec for Pending1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pending_1::R`](R) reader structure"]
impl crate::Readable for Pending1Spec {}
#[doc = "`reset()` method sets PENDING_1 to value 0"]
impl crate::Resettable for Pending1Spec {
    const RESET_VALUE: u32 = 0;
}

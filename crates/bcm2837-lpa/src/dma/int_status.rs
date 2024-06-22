#[doc = "Register `INT_STATUS` reader"]
pub type R = crate::R<IntStatusSpec>;
#[doc = "Register `INT_STATUS` writer"]
pub type W = crate::W<IntStatusSpec>;
#[doc = "Field `INT0` reader - Interrupt status of DMA engine 0"]
pub type Int0R = crate::BitReader;
#[doc = "Field `INT0` writer - Interrupt status of DMA engine 0"]
pub type Int0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT1` reader - Interrupt status of DMA engine 1"]
pub type Int1R = crate::BitReader;
#[doc = "Field `INT1` writer - Interrupt status of DMA engine 1"]
pub type Int1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT2` reader - Interrupt status of DMA engine 2"]
pub type Int2R = crate::BitReader;
#[doc = "Field `INT2` writer - Interrupt status of DMA engine 2"]
pub type Int2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT3` reader - Interrupt status of DMA engine 3"]
pub type Int3R = crate::BitReader;
#[doc = "Field `INT3` writer - Interrupt status of DMA engine 3"]
pub type Int3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT4` reader - Interrupt status of DMA engine 4"]
pub type Int4R = crate::BitReader;
#[doc = "Field `INT4` writer - Interrupt status of DMA engine 4"]
pub type Int4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT5` reader - Interrupt status of DMA engine 5"]
pub type Int5R = crate::BitReader;
#[doc = "Field `INT5` writer - Interrupt status of DMA engine 5"]
pub type Int5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT6` reader - Interrupt status of DMA engine 6"]
pub type Int6R = crate::BitReader;
#[doc = "Field `INT6` writer - Interrupt status of DMA engine 6"]
pub type Int6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT7` reader - Interrupt status of DMA engine 7"]
pub type Int7R = crate::BitReader;
#[doc = "Field `INT7` writer - Interrupt status of DMA engine 7"]
pub type Int7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT8` reader - Interrupt status of DMA engine 8"]
pub type Int8R = crate::BitReader;
#[doc = "Field `INT8` writer - Interrupt status of DMA engine 8"]
pub type Int8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT9` reader - Interrupt status of DMA engine 9"]
pub type Int9R = crate::BitReader;
#[doc = "Field `INT9` writer - Interrupt status of DMA engine 9"]
pub type Int9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT10` reader - Interrupt status of DMA engine 10"]
pub type Int10R = crate::BitReader;
#[doc = "Field `INT10` writer - Interrupt status of DMA engine 10"]
pub type Int10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT11` reader - Interrupt status of DMA engine 11"]
pub type Int11R = crate::BitReader;
#[doc = "Field `INT11` writer - Interrupt status of DMA engine 11"]
pub type Int11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT12` reader - Interrupt status of DMA engine 12"]
pub type Int12R = crate::BitReader;
#[doc = "Field `INT12` writer - Interrupt status of DMA engine 12"]
pub type Int12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT13` reader - Interrupt status of DMA engine 13"]
pub type Int13R = crate::BitReader;
#[doc = "Field `INT13` writer - Interrupt status of DMA engine 13"]
pub type Int13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT14` reader - Interrupt status of DMA engine 14"]
pub type Int14R = crate::BitReader;
#[doc = "Field `INT14` writer - Interrupt status of DMA engine 14"]
pub type Int14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT15` reader - Interrupt status of DMA engine 15"]
pub type Int15R = crate::BitReader;
#[doc = "Field `INT15` writer - Interrupt status of DMA engine 15"]
pub type Int15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt status of DMA engine 0"]
    #[inline(always)]
    pub fn int0(&self) -> Int0R {
        Int0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt status of DMA engine 1"]
    #[inline(always)]
    pub fn int1(&self) -> Int1R {
        Int1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt status of DMA engine 2"]
    #[inline(always)]
    pub fn int2(&self) -> Int2R {
        Int2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt status of DMA engine 3"]
    #[inline(always)]
    pub fn int3(&self) -> Int3R {
        Int3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt status of DMA engine 4"]
    #[inline(always)]
    pub fn int4(&self) -> Int4R {
        Int4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt status of DMA engine 5"]
    #[inline(always)]
    pub fn int5(&self) -> Int5R {
        Int5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt status of DMA engine 6"]
    #[inline(always)]
    pub fn int6(&self) -> Int6R {
        Int6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt status of DMA engine 7"]
    #[inline(always)]
    pub fn int7(&self) -> Int7R {
        Int7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt status of DMA engine 8"]
    #[inline(always)]
    pub fn int8(&self) -> Int8R {
        Int8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt status of DMA engine 9"]
    #[inline(always)]
    pub fn int9(&self) -> Int9R {
        Int9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt status of DMA engine 10"]
    #[inline(always)]
    pub fn int10(&self) -> Int10R {
        Int10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt status of DMA engine 11"]
    #[inline(always)]
    pub fn int11(&self) -> Int11R {
        Int11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt status of DMA engine 12"]
    #[inline(always)]
    pub fn int12(&self) -> Int12R {
        Int12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt status of DMA engine 13"]
    #[inline(always)]
    pub fn int13(&self) -> Int13R {
        Int13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt status of DMA engine 14"]
    #[inline(always)]
    pub fn int14(&self) -> Int14R {
        Int14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt status of DMA engine 15"]
    #[inline(always)]
    pub fn int15(&self) -> Int15R {
        Int15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_STATUS")
            .field("int15", &self.int15())
            .field("int14", &self.int14())
            .field("int13", &self.int13())
            .field("int12", &self.int12())
            .field("int11", &self.int11())
            .field("int10", &self.int10())
            .field("int9", &self.int9())
            .field("int8", &self.int8())
            .field("int7", &self.int7())
            .field("int6", &self.int6())
            .field("int5", &self.int5())
            .field("int4", &self.int4())
            .field("int3", &self.int3())
            .field("int2", &self.int2())
            .field("int1", &self.int1())
            .field("int0", &self.int0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt status of DMA engine 0"]
    #[inline(always)]
    #[must_use]
    pub fn int0(&mut self) -> Int0W<IntStatusSpec> {
        Int0W::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt status of DMA engine 1"]
    #[inline(always)]
    #[must_use]
    pub fn int1(&mut self) -> Int1W<IntStatusSpec> {
        Int1W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt status of DMA engine 2"]
    #[inline(always)]
    #[must_use]
    pub fn int2(&mut self) -> Int2W<IntStatusSpec> {
        Int2W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt status of DMA engine 3"]
    #[inline(always)]
    #[must_use]
    pub fn int3(&mut self) -> Int3W<IntStatusSpec> {
        Int3W::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt status of DMA engine 4"]
    #[inline(always)]
    #[must_use]
    pub fn int4(&mut self) -> Int4W<IntStatusSpec> {
        Int4W::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt status of DMA engine 5"]
    #[inline(always)]
    #[must_use]
    pub fn int5(&mut self) -> Int5W<IntStatusSpec> {
        Int5W::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt status of DMA engine 6"]
    #[inline(always)]
    #[must_use]
    pub fn int6(&mut self) -> Int6W<IntStatusSpec> {
        Int6W::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt status of DMA engine 7"]
    #[inline(always)]
    #[must_use]
    pub fn int7(&mut self) -> Int7W<IntStatusSpec> {
        Int7W::new(self, 7)
    }
    #[doc = "Bit 8 - Interrupt status of DMA engine 8"]
    #[inline(always)]
    #[must_use]
    pub fn int8(&mut self) -> Int8W<IntStatusSpec> {
        Int8W::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt status of DMA engine 9"]
    #[inline(always)]
    #[must_use]
    pub fn int9(&mut self) -> Int9W<IntStatusSpec> {
        Int9W::new(self, 9)
    }
    #[doc = "Bit 10 - Interrupt status of DMA engine 10"]
    #[inline(always)]
    #[must_use]
    pub fn int10(&mut self) -> Int10W<IntStatusSpec> {
        Int10W::new(self, 10)
    }
    #[doc = "Bit 11 - Interrupt status of DMA engine 11"]
    #[inline(always)]
    #[must_use]
    pub fn int11(&mut self) -> Int11W<IntStatusSpec> {
        Int11W::new(self, 11)
    }
    #[doc = "Bit 12 - Interrupt status of DMA engine 12"]
    #[inline(always)]
    #[must_use]
    pub fn int12(&mut self) -> Int12W<IntStatusSpec> {
        Int12W::new(self, 12)
    }
    #[doc = "Bit 13 - Interrupt status of DMA engine 13"]
    #[inline(always)]
    #[must_use]
    pub fn int13(&mut self) -> Int13W<IntStatusSpec> {
        Int13W::new(self, 13)
    }
    #[doc = "Bit 14 - Interrupt status of DMA engine 14"]
    #[inline(always)]
    #[must_use]
    pub fn int14(&mut self) -> Int14W<IntStatusSpec> {
        Int14W::new(self, 14)
    }
    #[doc = "Bit 15 - Interrupt status of DMA engine 15"]
    #[inline(always)]
    #[must_use]
    pub fn int15(&mut self) -> Int15W<IntStatusSpec> {
        Int15W::new(self, 15)
    }
}
#[doc = "Interrupt status of each DMA channel\n\nYou can [`read`](crate::Reg::read) this register and get [`int_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStatusSpec;
impl crate::RegisterSpec for IntStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_status::R`](R) reader structure"]
impl crate::Readable for IntStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`int_status::W`](W) writer structure"]
impl crate::Writable for IntStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_STATUS to value 0"]
impl crate::Resettable for IntStatusSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `HW_DIRECTION` reader"]
pub type R = crate::R<HwDirectionSpec>;
#[doc = "Direction %s"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Direction {
    #[doc = "0: `0`"]
    Bidir = 0,
    #[doc = "1: `1`"]
    In = 1,
    #[doc = "2: `10`"]
    Out = 2,
}
impl From<Direction> for u8 {
    #[inline(always)]
    fn from(variant: Direction) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Direction {
    type Ux = u8;
}
impl crate::IsEnum for Direction {}
#[doc = "Field `DIRECTION(0-15)` reader - Direction %s"]
pub type DirectionR = crate::FieldReader<Direction>;
impl DirectionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Direction> {
        match self.bits {
            0 => Some(Direction::Bidir),
            1 => Some(Direction::In),
            2 => Some(Direction::Out),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_bidir(&self) -> bool {
        *self == Direction::Bidir
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == Direction::In
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == Direction::Out
    }
}
impl R {
    #[doc = "Direction (0-15)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DIRECTION0` field.</div>"]
    #[inline(always)]
    pub fn direction(&self, n: u8) -> DirectionR {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        DirectionR::new(((self.bits >> (n * 2)) & 3) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Direction (0-15)"]
    #[inline(always)]
    pub fn direction_iter(&self) -> impl Iterator<Item = DirectionR> + '_ {
        (0..16).map(move |n| DirectionR::new(((self.bits >> (n * 2)) & 3) as u8))
    }
    #[doc = "Bits 0:1 - Direction 0"]
    #[inline(always)]
    pub fn direction0(&self) -> DirectionR {
        DirectionR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Direction 1"]
    #[inline(always)]
    pub fn direction1(&self) -> DirectionR {
        DirectionR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Direction 2"]
    #[inline(always)]
    pub fn direction2(&self) -> DirectionR {
        DirectionR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Direction 3"]
    #[inline(always)]
    pub fn direction3(&self) -> DirectionR {
        DirectionR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Direction 4"]
    #[inline(always)]
    pub fn direction4(&self) -> DirectionR {
        DirectionR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Direction 5"]
    #[inline(always)]
    pub fn direction5(&self) -> DirectionR {
        DirectionR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Direction 6"]
    #[inline(always)]
    pub fn direction6(&self) -> DirectionR {
        DirectionR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Direction 7"]
    #[inline(always)]
    pub fn direction7(&self) -> DirectionR {
        DirectionR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Direction 8"]
    #[inline(always)]
    pub fn direction8(&self) -> DirectionR {
        DirectionR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Direction 9"]
    #[inline(always)]
    pub fn direction9(&self) -> DirectionR {
        DirectionR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Direction 10"]
    #[inline(always)]
    pub fn direction10(&self) -> DirectionR {
        DirectionR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Direction 11"]
    #[inline(always)]
    pub fn direction11(&self) -> DirectionR {
        DirectionR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Direction 12"]
    #[inline(always)]
    pub fn direction12(&self) -> DirectionR {
        DirectionR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Direction 13"]
    #[inline(always)]
    pub fn direction13(&self) -> DirectionR {
        DirectionR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Direction 14"]
    #[inline(always)]
    pub fn direction14(&self) -> DirectionR {
        DirectionR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Direction 15"]
    #[inline(always)]
    pub fn direction15(&self) -> DirectionR {
        DirectionR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HW_DIRECTION")
            .field("direction0", &self.direction0())
            .field("direction1", &self.direction1())
            .field("direction2", &self.direction2())
            .field("direction3", &self.direction3())
            .field("direction4", &self.direction4())
            .field("direction5", &self.direction5())
            .field("direction6", &self.direction6())
            .field("direction7", &self.direction7())
            .field("direction8", &self.direction8())
            .field("direction9", &self.direction9())
            .field("direction10", &self.direction10())
            .field("direction11", &self.direction11())
            .field("direction12", &self.direction12())
            .field("direction13", &self.direction13())
            .field("direction14", &self.direction14())
            .field("direction15", &self.direction15())
            .finish()
    }
}
#[doc = "Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_direction::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwDirectionSpec;
impl crate::RegisterSpec for HwDirectionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hw_direction::R`](R) reader structure"]
impl crate::Readable for HwDirectionSpec {}

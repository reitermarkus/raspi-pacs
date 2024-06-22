#[doc = "Register `DBG_SEL` reader"]
pub type R = crate::R<DbgSelSpec>;
#[doc = "Register `DBG_SEL` writer"]
pub type W = crate::W<DbgSelSpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Select {
    #[doc = "0: `0`"]
    ReceiverFifo = 0,
    #[doc = "1: `1`"]
    Others = 1,
}
impl From<Select> for bool {
    #[inline(always)]
    fn from(variant: Select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELECT` reader - "]
pub type SelectR = crate::BitReader<Select>;
impl SelectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Select {
        match self.bits {
            false => Select::ReceiverFifo,
            true => Select::Others,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_receiver_fifo(&self) -> bool {
        *self == Select::ReceiverFifo
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        *self == Select::Others
    }
}
#[doc = "Field `SELECT` writer - "]
pub type SelectW<'a, REG> = crate::BitWriter<'a, REG, Select>;
impl<'a, REG> SelectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn receiver_fifo(self) -> &'a mut crate::W<REG> {
        self.variant(Select::ReceiverFifo)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Select::Others)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn select(&self) -> SelectR {
        SelectR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBG_SEL")
            .field("select", &self.select())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn select(&mut self) -> SelectW<DbgSelSpec> {
        SelectW::new(self, 0)
    }
}
#[doc = "What submodules are accessed by the debug bus\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgSelSpec;
impl crate::RegisterSpec for DbgSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_sel::R`](R) reader structure"]
impl crate::Readable for DbgSelSpec {}
#[doc = "`write(|w| ..)` method takes [`dbg_sel::W`](W) writer structure"]
impl crate::Writable for DbgSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBG_SEL to value 0"]
impl crate::Resettable for DbgSelSpec {
    const RESET_VALUE: u32 = 0;
}

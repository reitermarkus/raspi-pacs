#[doc = "Register `LCR` reader"]
pub type R = crate::R<LcrSpec>;
#[doc = "Register `LCR` writer"]
pub type W = crate::W<LcrSpec>;
#[doc = "UART word size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: 7 bit"]
    _7bit = 0,
    #[doc = "3: 8 bit"]
    _8bit = 3,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `DATA_SIZE` reader - UART word size"]
pub type DataSizeR = crate::FieldReader<Mode>;
impl DataSizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mode> {
        match self.bits {
            0 => Some(Mode::_7bit),
            3 => Some(Mode::_8bit),
            _ => None,
        }
    }
    #[doc = "7 bit"]
    #[inline(always)]
    pub fn is_7bit(&self) -> bool {
        *self == Mode::_7bit
    }
    #[doc = "8 bit"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == Mode::_8bit
    }
}
#[doc = "Field `DATA_SIZE` writer - UART word size"]
pub type DataSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode>;
impl<'a, REG> DataSizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "7 bit"]
    #[inline(always)]
    pub fn _7bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_7bit)
    }
    #[doc = "8 bit"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_8bit)
    }
}
#[doc = "Field `BREAK` reader - Pull TX low continuously to send break"]
pub type BreakR = crate::BitReader;
#[doc = "Field `BREAK` writer - Pull TX low continuously to send break"]
pub type BreakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLAB` reader - First two registers are baudrate"]
pub type DlabR = crate::BitReader;
#[doc = "Field `DLAB` writer - First two registers are baudrate"]
pub type DlabW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - UART word size"]
    #[inline(always)]
    pub fn data_size(&self) -> DataSizeR {
        DataSizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 6 - Pull TX low continuously to send break"]
    #[inline(always)]
    pub fn break_(&self) -> BreakR {
        BreakR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - First two registers are baudrate"]
    #[inline(always)]
    pub fn dlab(&self) -> DlabR {
        DlabR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCR")
            .field("dlab", &self.dlab())
            .field("break_", &self.break_())
            .field("data_size", &self.data_size())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - UART word size"]
    #[inline(always)]
    #[must_use]
    pub fn data_size(&mut self) -> DataSizeW<LcrSpec> {
        DataSizeW::new(self, 0)
    }
    #[doc = "Bit 6 - Pull TX low continuously to send break"]
    #[inline(always)]
    #[must_use]
    pub fn break_(&mut self) -> BreakW<LcrSpec> {
        BreakW::new(self, 6)
    }
    #[doc = "Bit 7 - First two registers are baudrate"]
    #[inline(always)]
    #[must_use]
    pub fn dlab(&mut self) -> DlabW<LcrSpec> {
        DlabW::new(self, 7)
    }
}
#[doc = "Line control\n\nYou can [`read`](crate::Reg::read) this register and get [`lcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcrSpec;
impl crate::RegisterSpec for LcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcr::R`](R) reader structure"]
impl crate::Readable for LcrSpec {}
#[doc = "`write(|w| ..)` method takes [`lcr::W`](W) writer structure"]
impl crate::Writable for LcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LCR to value 0"]
impl crate::Resettable for LcrSpec {
    const RESET_VALUE: u32 = 0;
}

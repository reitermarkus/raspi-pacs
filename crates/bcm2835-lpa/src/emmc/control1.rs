#[doc = "Register `CONTROL1` reader"]
pub type R = crate::R<Control1Spec>;
#[doc = "Register `CONTROL1` writer"]
pub type W = crate::W<Control1Spec>;
#[doc = "Field `CLK_INTLEN` reader - Enable internal clock"]
pub type ClkIntlenR = crate::BitReader;
#[doc = "Field `CLK_INTLEN` writer - Enable internal clock"]
pub type ClkIntlenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_STABLE` reader - SD Clock stable"]
pub type ClkStableR = crate::BitReader;
#[doc = "Field `CLK_EN` reader - SD Clock enable"]
pub type ClkEnR = crate::BitReader;
#[doc = "Field `CLK_EN` writer - SD Clock enable"]
pub type ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Mode of clock generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkGensel {
    #[doc = "0: `0`"]
    Divided = 0,
    #[doc = "1: `1`"]
    Programmable = 1,
}
impl From<ClkGensel> for bool {
    #[inline(always)]
    fn from(variant: ClkGensel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_GENSEL` reader - Mode of clock generation"]
pub type ClkGenselR = crate::BitReader<ClkGensel>;
impl ClkGenselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkGensel {
        match self.bits {
            false => ClkGensel::Divided,
            true => ClkGensel::Programmable,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_divided(&self) -> bool {
        *self == ClkGensel::Divided
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_programmable(&self) -> bool {
        *self == ClkGensel::Programmable
    }
}
#[doc = "Field `CLK_GENSEL` writer - Mode of clock generation"]
pub type ClkGenselW<'a, REG> = crate::BitWriter<'a, REG, ClkGensel>;
impl<'a, REG> ClkGenselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn divided(self) -> &'a mut crate::W<REG> {
        self.variant(ClkGensel::Divided)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn programmable(self) -> &'a mut crate::W<REG> {
        self.variant(ClkGensel::Programmable)
    }
}
#[doc = "Field `CLK_FREQ_MS2` reader - Clock base divider MSBs"]
pub type ClkFreqMs2R = crate::FieldReader;
#[doc = "Field `CLK_FREQ_MS2` writer - Clock base divider MSBs"]
pub type ClkFreqMs2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLK_FREQ8` reader - Clock base divider LSB"]
pub type ClkFreq8R = crate::FieldReader;
#[doc = "Field `CLK_FREQ8` writer - Clock base divider LSB"]
pub type ClkFreq8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA_TOUNIT` reader - Data timeout exponent (TMCLK * 2 ** (x + 13)) 1111 disabled"]
pub type DataTounitR = crate::FieldReader;
#[doc = "Field `DATA_TOUNIT` writer - Data timeout exponent (TMCLK * 2 ** (x + 13)) 1111 disabled"]
pub type DataTounitW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SRST_HC` reader - Reset the complete host circuit"]
pub type SrstHcR = crate::BitReader;
#[doc = "Field `SRST_HC` writer - Reset the complete host circuit"]
pub type SrstHcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRST_CMD` reader - Reset the command handling circuit"]
pub type SrstCmdR = crate::BitReader;
#[doc = "Field `SRST_CMD` writer - Reset the command handling circuit"]
pub type SrstCmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRST_DATA` reader - Reset the data handling circuit"]
pub type SrstDataR = crate::BitReader;
#[doc = "Field `SRST_DATA` writer - Reset the data handling circuit"]
pub type SrstDataW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable internal clock"]
    #[inline(always)]
    pub fn clk_intlen(&self) -> ClkIntlenR {
        ClkIntlenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SD Clock stable"]
    #[inline(always)]
    pub fn clk_stable(&self) -> ClkStableR {
        ClkStableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SD Clock enable"]
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Mode of clock generation"]
    #[inline(always)]
    pub fn clk_gensel(&self) -> ClkGenselR {
        ClkGenselR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Clock base divider MSBs"]
    #[inline(always)]
    pub fn clk_freq_ms2(&self) -> ClkFreqMs2R {
        ClkFreqMs2R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - Clock base divider LSB"]
    #[inline(always)]
    pub fn clk_freq8(&self) -> ClkFreq8R {
        ClkFreq8R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Data timeout exponent (TMCLK * 2 ** (x + 13)) 1111 disabled"]
    #[inline(always)]
    pub fn data_tounit(&self) -> DataTounitR {
        DataTounitR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Reset the complete host circuit"]
    #[inline(always)]
    pub fn srst_hc(&self) -> SrstHcR {
        SrstHcR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reset the command handling circuit"]
    #[inline(always)]
    pub fn srst_cmd(&self) -> SrstCmdR {
        SrstCmdR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Reset the data handling circuit"]
    #[inline(always)]
    pub fn srst_data(&self) -> SrstDataR {
        SrstDataR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONTROL1")
            .field("srst_data", &self.srst_data())
            .field("srst_cmd", &self.srst_cmd())
            .field("srst_hc", &self.srst_hc())
            .field("data_tounit", &self.data_tounit())
            .field("clk_freq8", &self.clk_freq8())
            .field("clk_freq_ms2", &self.clk_freq_ms2())
            .field("clk_gensel", &self.clk_gensel())
            .field("clk_en", &self.clk_en())
            .field("clk_stable", &self.clk_stable())
            .field("clk_intlen", &self.clk_intlen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enable internal clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_intlen(&mut self) -> ClkIntlenW<Control1Spec> {
        ClkIntlenW::new(self, 0)
    }
    #[doc = "Bit 2 - SD Clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> ClkEnW<Control1Spec> {
        ClkEnW::new(self, 2)
    }
    #[doc = "Bit 5 - Mode of clock generation"]
    #[inline(always)]
    #[must_use]
    pub fn clk_gensel(&mut self) -> ClkGenselW<Control1Spec> {
        ClkGenselW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Clock base divider MSBs"]
    #[inline(always)]
    #[must_use]
    pub fn clk_freq_ms2(&mut self) -> ClkFreqMs2W<Control1Spec> {
        ClkFreqMs2W::new(self, 6)
    }
    #[doc = "Bits 8:15 - Clock base divider LSB"]
    #[inline(always)]
    #[must_use]
    pub fn clk_freq8(&mut self) -> ClkFreq8W<Control1Spec> {
        ClkFreq8W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Data timeout exponent (TMCLK * 2 ** (x + 13)) 1111 disabled"]
    #[inline(always)]
    #[must_use]
    pub fn data_tounit(&mut self) -> DataTounitW<Control1Spec> {
        DataTounitW::new(self, 16)
    }
    #[doc = "Bit 24 - Reset the complete host circuit"]
    #[inline(always)]
    #[must_use]
    pub fn srst_hc(&mut self) -> SrstHcW<Control1Spec> {
        SrstHcW::new(self, 24)
    }
    #[doc = "Bit 25 - Reset the command handling circuit"]
    #[inline(always)]
    #[must_use]
    pub fn srst_cmd(&mut self) -> SrstCmdW<Control1Spec> {
        SrstCmdW::new(self, 25)
    }
    #[doc = "Bit 26 - Reset the data handling circuit"]
    #[inline(always)]
    #[must_use]
    pub fn srst_data(&mut self) -> SrstDataW<Control1Spec> {
        SrstDataW::new(self, 26)
    }
}
#[doc = "Configure\n\nYou can [`read`](crate::Reg::read) this register and get [`control1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Control1Spec;
impl crate::RegisterSpec for Control1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control1::R`](R) reader structure"]
impl crate::Readable for Control1Spec {}
#[doc = "`write(|w| ..)` method takes [`control1::W`](W) writer structure"]
impl crate::Writable for Control1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONTROL1 to value 0"]
impl crate::Resettable for Control1Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `CMDTM` reader"]
pub type R = crate::R<CmdtmSpec>;
#[doc = "Register `CMDTM` writer"]
pub type W = crate::W<CmdtmSpec>;
#[doc = "Field `TM_BLKCNT_EN` reader - Enable block counter"]
pub type TmBlkcntEnR = crate::BitReader;
#[doc = "Field `TM_BLKCNT_EN` writer - Enable block counter"]
pub type TmBlkcntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Command after completion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TmAutoCmdEn {
    #[doc = "0: `0`"]
    None = 0,
    #[doc = "1: `1`"]
    Cmd12 = 1,
    #[doc = "2: `10`"]
    Cmd23 = 2,
}
impl From<TmAutoCmdEn> for u8 {
    #[inline(always)]
    fn from(variant: TmAutoCmdEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TmAutoCmdEn {
    type Ux = u8;
}
impl crate::IsEnum for TmAutoCmdEn {}
#[doc = "Field `TM_AUTO_CMD_EN` reader - Command after completion"]
pub type TmAutoCmdEnR = crate::FieldReader<TmAutoCmdEn>;
impl TmAutoCmdEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TmAutoCmdEn> {
        match self.bits {
            0 => Some(TmAutoCmdEn::None),
            1 => Some(TmAutoCmdEn::Cmd12),
            2 => Some(TmAutoCmdEn::Cmd23),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == TmAutoCmdEn::None
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_cmd12(&self) -> bool {
        *self == TmAutoCmdEn::Cmd12
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_cmd23(&self) -> bool {
        *self == TmAutoCmdEn::Cmd23
    }
}
#[doc = "Field `TM_AUTO_CMD_EN` writer - Command after completion"]
pub type TmAutoCmdEnW<'a, REG> = crate::FieldWriter<'a, REG, 2, TmAutoCmdEn>;
impl<'a, REG> TmAutoCmdEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(TmAutoCmdEn::None)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn cmd12(self) -> &'a mut crate::W<REG> {
        self.variant(TmAutoCmdEn::Cmd12)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn cmd23(self) -> &'a mut crate::W<REG> {
        self.variant(TmAutoCmdEn::Cmd23)
    }
}
#[doc = "Direction of data transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TmDatDir {
    #[doc = "0: `0`"]
    HostToCard = 0,
    #[doc = "1: `1`"]
    CardToHost = 1,
}
impl From<TmDatDir> for bool {
    #[inline(always)]
    fn from(variant: TmDatDir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TM_DAT_DIR` reader - Direction of data transfer"]
pub type TmDatDirR = crate::BitReader<TmDatDir>;
impl TmDatDirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TmDatDir {
        match self.bits {
            false => TmDatDir::HostToCard,
            true => TmDatDir::CardToHost,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_host_to_card(&self) -> bool {
        *self == TmDatDir::HostToCard
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_card_to_host(&self) -> bool {
        *self == TmDatDir::CardToHost
    }
}
#[doc = "Field `TM_DAT_DIR` writer - Direction of data transfer"]
pub type TmDatDirW<'a, REG> = crate::BitWriter<'a, REG, TmDatDir>;
impl<'a, REG> TmDatDirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn host_to_card(self) -> &'a mut crate::W<REG> {
        self.variant(TmDatDir::HostToCard)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn card_to_host(self) -> &'a mut crate::W<REG> {
        self.variant(TmDatDir::CardToHost)
    }
}
#[doc = "Type of data transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TmMultiBlock {
    #[doc = "0: `0`"]
    Single = 0,
    #[doc = "1: `1`"]
    Multiple = 1,
}
impl From<TmMultiBlock> for bool {
    #[inline(always)]
    fn from(variant: TmMultiBlock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TM_MULTI_BLOCK` reader - Type of data transfer"]
pub type TmMultiBlockR = crate::BitReader<TmMultiBlock>;
impl TmMultiBlockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TmMultiBlock {
        match self.bits {
            false => TmMultiBlock::Single,
            true => TmMultiBlock::Multiple,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == TmMultiBlock::Single
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_multiple(&self) -> bool {
        *self == TmMultiBlock::Multiple
    }
}
#[doc = "Field `TM_MULTI_BLOCK` writer - Type of data transfer"]
pub type TmMultiBlockW<'a, REG> = crate::BitWriter<'a, REG, TmMultiBlock>;
impl<'a, REG> TmMultiBlockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(TmMultiBlock::Single)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn multiple(self) -> &'a mut crate::W<REG> {
        self.variant(TmMultiBlock::Multiple)
    }
}
#[doc = "Type of expected response\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Response {
    #[doc = "0: `0`"]
    None = 0,
    #[doc = "1: `1`"]
    _136bits = 1,
    #[doc = "2: `10`"]
    _48bits = 2,
    #[doc = "3: `11`"]
    _48bitsUsingBusy = 3,
}
impl From<Response> for u8 {
    #[inline(always)]
    fn from(variant: Response) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Response {
    type Ux = u8;
}
impl crate::IsEnum for Response {}
#[doc = "Field `CMD_RSPNS_TYPE` reader - Type of expected response"]
pub type CmdRspnsTypeR = crate::FieldReader<Response>;
impl CmdRspnsTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Response {
        match self.bits {
            0 => Response::None,
            1 => Response::_136bits,
            2 => Response::_48bits,
            3 => Response::_48bitsUsingBusy,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Response::None
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_136bits(&self) -> bool {
        *self == Response::_136bits
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_48bits(&self) -> bool {
        *self == Response::_48bits
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_48bits_using_busy(&self) -> bool {
        *self == Response::_48bitsUsingBusy
    }
}
#[doc = "Field `CMD_RSPNS_TYPE` writer - Type of expected response"]
pub type CmdRspnsTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Response, crate::Safe>;
impl<'a, REG> CmdRspnsTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Response::None)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _136bits(self) -> &'a mut crate::W<REG> {
        self.variant(Response::_136bits)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _48bits(self) -> &'a mut crate::W<REG> {
        self.variant(Response::_48bits)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn _48bits_using_busy(self) -> &'a mut crate::W<REG> {
        self.variant(Response::_48bitsUsingBusy)
    }
}
#[doc = "Field `CMD_CRCCHK_EN` reader - Check the responses CRC"]
pub type CmdCrcchkEnR = crate::BitReader;
#[doc = "Field `CMD_CRCCHK_EN` writer - Check the responses CRC"]
pub type CmdCrcchkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_IXCHK_EN` reader - Check that the response has the same command index"]
pub type CmdIxchkEnR = crate::BitReader;
#[doc = "Field `CMD_IXCHK_EN` writer - Check that the response has the same command index"]
pub type CmdIxchkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_ISDATA` reader - Command involves data"]
pub type CmdIsdataR = crate::BitReader;
#[doc = "Field `CMD_ISDATA` writer - Command involves data"]
pub type CmdIsdataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Type of command to be issued\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CmdType {
    #[doc = "0: `0`"]
    Normal = 0,
    #[doc = "1: `1`"]
    Suspend = 1,
    #[doc = "2: `10`"]
    Resume = 2,
    #[doc = "3: `11`"]
    Abort = 3,
}
impl From<CmdType> for u8 {
    #[inline(always)]
    fn from(variant: CmdType) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CmdType {
    type Ux = u8;
}
impl crate::IsEnum for CmdType {}
#[doc = "Field `CMD_TYPE` reader - Type of command to be issued"]
pub type CmdTypeR = crate::FieldReader<CmdType>;
impl CmdTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CmdType {
        match self.bits {
            0 => CmdType::Normal,
            1 => CmdType::Suspend,
            2 => CmdType::Resume,
            3 => CmdType::Abort,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == CmdType::Normal
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == CmdType::Suspend
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        *self == CmdType::Resume
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_abort(&self) -> bool {
        *self == CmdType::Abort
    }
}
#[doc = "Field `CMD_TYPE` writer - Type of command to be issued"]
pub type CmdTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2, CmdType, crate::Safe>;
impl<'a, REG> CmdTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(CmdType::Normal)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut crate::W<REG> {
        self.variant(CmdType::Suspend)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut crate::W<REG> {
        self.variant(CmdType::Resume)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn abort(self) -> &'a mut crate::W<REG> {
        self.variant(CmdType::Abort)
    }
}
#[doc = "Field `CMD_INDEX` reader - Command index to be issued"]
pub type CmdIndexR = crate::FieldReader;
#[doc = "Field `CMD_INDEX` writer - Command index to be issued"]
pub type CmdIndexW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 1 - Enable block counter"]
    #[inline(always)]
    pub fn tm_blkcnt_en(&self) -> TmBlkcntEnR {
        TmBlkcntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Command after completion"]
    #[inline(always)]
    pub fn tm_auto_cmd_en(&self) -> TmAutoCmdEnR {
        TmAutoCmdEnR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Direction of data transfer"]
    #[inline(always)]
    pub fn tm_dat_dir(&self) -> TmDatDirR {
        TmDatDirR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Type of data transfer"]
    #[inline(always)]
    pub fn tm_multi_block(&self) -> TmMultiBlockR {
        TmMultiBlockR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Type of expected response"]
    #[inline(always)]
    pub fn cmd_rspns_type(&self) -> CmdRspnsTypeR {
        CmdRspnsTypeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 19 - Check the responses CRC"]
    #[inline(always)]
    pub fn cmd_crcchk_en(&self) -> CmdCrcchkEnR {
        CmdCrcchkEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Check that the response has the same command index"]
    #[inline(always)]
    pub fn cmd_ixchk_en(&self) -> CmdIxchkEnR {
        CmdIxchkEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Command involves data"]
    #[inline(always)]
    pub fn cmd_isdata(&self) -> CmdIsdataR {
        CmdIsdataR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Type of command to be issued"]
    #[inline(always)]
    pub fn cmd_type(&self) -> CmdTypeR {
        CmdTypeR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:29 - Command index to be issued"]
    #[inline(always)]
    pub fn cmd_index(&self) -> CmdIndexR {
        CmdIndexR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDTM")
            .field("cmd_index", &self.cmd_index())
            .field("cmd_type", &self.cmd_type())
            .field("cmd_isdata", &self.cmd_isdata())
            .field("cmd_ixchk_en", &self.cmd_ixchk_en())
            .field("cmd_crcchk_en", &self.cmd_crcchk_en())
            .field("cmd_rspns_type", &self.cmd_rspns_type())
            .field("tm_multi_block", &self.tm_multi_block())
            .field("tm_dat_dir", &self.tm_dat_dir())
            .field("tm_auto_cmd_en", &self.tm_auto_cmd_en())
            .field("tm_blkcnt_en", &self.tm_blkcnt_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Enable block counter"]
    #[inline(always)]
    #[must_use]
    pub fn tm_blkcnt_en(&mut self) -> TmBlkcntEnW<CmdtmSpec> {
        TmBlkcntEnW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Command after completion"]
    #[inline(always)]
    #[must_use]
    pub fn tm_auto_cmd_en(&mut self) -> TmAutoCmdEnW<CmdtmSpec> {
        TmAutoCmdEnW::new(self, 2)
    }
    #[doc = "Bit 4 - Direction of data transfer"]
    #[inline(always)]
    #[must_use]
    pub fn tm_dat_dir(&mut self) -> TmDatDirW<CmdtmSpec> {
        TmDatDirW::new(self, 4)
    }
    #[doc = "Bit 5 - Type of data transfer"]
    #[inline(always)]
    #[must_use]
    pub fn tm_multi_block(&mut self) -> TmMultiBlockW<CmdtmSpec> {
        TmMultiBlockW::new(self, 5)
    }
    #[doc = "Bits 16:17 - Type of expected response"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_rspns_type(&mut self) -> CmdRspnsTypeW<CmdtmSpec> {
        CmdRspnsTypeW::new(self, 16)
    }
    #[doc = "Bit 19 - Check the responses CRC"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_crcchk_en(&mut self) -> CmdCrcchkEnW<CmdtmSpec> {
        CmdCrcchkEnW::new(self, 19)
    }
    #[doc = "Bit 20 - Check that the response has the same command index"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_ixchk_en(&mut self) -> CmdIxchkEnW<CmdtmSpec> {
        CmdIxchkEnW::new(self, 20)
    }
    #[doc = "Bit 21 - Command involves data"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_isdata(&mut self) -> CmdIsdataW<CmdtmSpec> {
        CmdIsdataW::new(self, 21)
    }
    #[doc = "Bits 22:23 - Type of command to be issued"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_type(&mut self) -> CmdTypeW<CmdtmSpec> {
        CmdTypeW::new(self, 22)
    }
    #[doc = "Bits 24:29 - Command index to be issued"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_index(&mut self) -> CmdIndexW<CmdtmSpec> {
        CmdIndexW::new(self, 24)
    }
}
#[doc = "Issue commands to the card\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdtm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdtm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdtmSpec;
impl crate::RegisterSpec for CmdtmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdtm::R`](R) reader structure"]
impl crate::Readable for CmdtmSpec {}
#[doc = "`write(|w| ..)` method takes [`cmdtm::W`](W) writer structure"]
impl crate::Writable for CmdtmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMDTM to value 0"]
impl crate::Resettable for CmdtmSpec {
    const RESET_VALUE: u32 = 0;
}

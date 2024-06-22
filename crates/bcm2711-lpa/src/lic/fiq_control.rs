#[doc = "Register `FIQ_CONTROL` reader"]
pub type R = crate::R<FiqControlSpec>;
#[doc = "Register `FIQ_CONTROL` writer"]
pub type W = crate::W<FiqControlSpec>;
#[doc = "FIQ Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Source {
    #[doc = "0: Interrupt 0"]
    Int0 = 0,
    #[doc = "1: Interrupt 1"]
    Int1 = 1,
    #[doc = "2: Interrupt 2"]
    Int2 = 2,
    #[doc = "3: Interrupt 3"]
    Int3 = 3,
    #[doc = "4: Interrupt 4"]
    Int4 = 4,
    #[doc = "5: Interrupt 5"]
    Int5 = 5,
    #[doc = "6: Interrupt 6"]
    Int6 = 6,
    #[doc = "7: Interrupt 7"]
    Int7 = 7,
    #[doc = "8: Interrupt 8"]
    Int8 = 8,
    #[doc = "9: Interrupt 9"]
    Int9 = 9,
    #[doc = "10: Interrupt 10"]
    Int10 = 10,
    #[doc = "11: Interrupt 11"]
    Int11 = 11,
    #[doc = "12: Interrupt 12"]
    Int12 = 12,
    #[doc = "13: Interrupt 13"]
    Int13 = 13,
    #[doc = "14: Interrupt 14"]
    Int14 = 14,
    #[doc = "15: Interrupt 15"]
    Int15 = 15,
    #[doc = "16: Interrupt 16"]
    Int16 = 16,
    #[doc = "17: Interrupt 17"]
    Int17 = 17,
    #[doc = "18: Interrupt 18"]
    Int18 = 18,
    #[doc = "19: Interrupt 19"]
    Int19 = 19,
    #[doc = "20: Interrupt 20"]
    Int20 = 20,
    #[doc = "21: Interrupt 21"]
    Int21 = 21,
    #[doc = "22: Interrupt 22"]
    Int22 = 22,
    #[doc = "23: Interrupt 23"]
    Int23 = 23,
    #[doc = "24: Interrupt 24"]
    Int24 = 24,
    #[doc = "25: Interrupt 25"]
    Int25 = 25,
    #[doc = "26: Interrupt 26"]
    Int26 = 26,
    #[doc = "27: Interrupt 27"]
    Int27 = 27,
    #[doc = "28: Interrupt 28"]
    Int28 = 28,
    #[doc = "29: Interrupt 29"]
    Int29 = 29,
    #[doc = "30: Interrupt 30"]
    Int30 = 30,
    #[doc = "31: Interrupt 31"]
    Int31 = 31,
    #[doc = "32: Interrupt 32"]
    Int32 = 32,
    #[doc = "33: Interrupt 33"]
    Int33 = 33,
    #[doc = "34: Interrupt 34"]
    Int34 = 34,
    #[doc = "35: Interrupt 35"]
    Int35 = 35,
    #[doc = "36: Interrupt 36"]
    Int36 = 36,
    #[doc = "37: Interrupt 37"]
    Int37 = 37,
    #[doc = "38: Interrupt 38"]
    Int38 = 38,
    #[doc = "39: Interrupt 39"]
    Int39 = 39,
    #[doc = "40: Interrupt 40"]
    Int40 = 40,
    #[doc = "41: Interrupt 41"]
    Int41 = 41,
    #[doc = "42: Interrupt 42"]
    Int42 = 42,
    #[doc = "43: Interrupt 43"]
    Int43 = 43,
    #[doc = "44: Interrupt 44"]
    Int44 = 44,
    #[doc = "45: Interrupt 45"]
    Int45 = 45,
    #[doc = "46: Interrupt 46"]
    Int46 = 46,
    #[doc = "47: Interrupt 47"]
    Int47 = 47,
    #[doc = "48: Interrupt 48"]
    Int48 = 48,
    #[doc = "49: Interrupt 49"]
    Int49 = 49,
    #[doc = "50: Interrupt 50"]
    Int50 = 50,
    #[doc = "51: Interrupt 51"]
    Int51 = 51,
    #[doc = "52: Interrupt 52"]
    Int52 = 52,
    #[doc = "53: Interrupt 53"]
    Int53 = 53,
    #[doc = "54: Interrupt 54"]
    Int54 = 54,
    #[doc = "55: Interrupt 55"]
    Int55 = 55,
    #[doc = "56: Interrupt 56"]
    Int56 = 56,
    #[doc = "57: Interrupt 57"]
    Int57 = 57,
    #[doc = "58: Interrupt 58"]
    Int58 = 58,
    #[doc = "59: Interrupt 59"]
    Int59 = 59,
    #[doc = "60: Interrupt 60"]
    Int60 = 60,
    #[doc = "61: Interrupt 61"]
    Int61 = 61,
    #[doc = "62: Interrupt 62"]
    Int62 = 62,
    #[doc = "63: Interrupt 63"]
    Int63 = 63,
    #[doc = "64: ARMC Timer"]
    Timer = 64,
    #[doc = "65: Mailbox"]
    Mailbox = 65,
    #[doc = "66: Doorbell 0"]
    Doorbell0 = 66,
    #[doc = "67: Doorbell 1"]
    Doorbell1 = 67,
    #[doc = "68: VPU0 halted"]
    Vpu0Halted = 68,
    #[doc = "69: VPU1 halted"]
    Vpu1Halted = 69,
    #[doc = "70: ARM address error"]
    ArmAddressError = 70,
    #[doc = "71: ARM AXI error"]
    ArmAxiError = 71,
}
impl From<Source> for u8 {
    #[inline(always)]
    fn from(variant: Source) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Source {
    type Ux = u8;
}
impl crate::IsEnum for Source {}
#[doc = "Field `SOURCE` reader - FIQ Source"]
pub type SourceR = crate::FieldReader<Source>;
impl SourceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Source> {
        match self.bits {
            0 => Some(Source::Int0),
            1 => Some(Source::Int1),
            2 => Some(Source::Int2),
            3 => Some(Source::Int3),
            4 => Some(Source::Int4),
            5 => Some(Source::Int5),
            6 => Some(Source::Int6),
            7 => Some(Source::Int7),
            8 => Some(Source::Int8),
            9 => Some(Source::Int9),
            10 => Some(Source::Int10),
            11 => Some(Source::Int11),
            12 => Some(Source::Int12),
            13 => Some(Source::Int13),
            14 => Some(Source::Int14),
            15 => Some(Source::Int15),
            16 => Some(Source::Int16),
            17 => Some(Source::Int17),
            18 => Some(Source::Int18),
            19 => Some(Source::Int19),
            20 => Some(Source::Int20),
            21 => Some(Source::Int21),
            22 => Some(Source::Int22),
            23 => Some(Source::Int23),
            24 => Some(Source::Int24),
            25 => Some(Source::Int25),
            26 => Some(Source::Int26),
            27 => Some(Source::Int27),
            28 => Some(Source::Int28),
            29 => Some(Source::Int29),
            30 => Some(Source::Int30),
            31 => Some(Source::Int31),
            32 => Some(Source::Int32),
            33 => Some(Source::Int33),
            34 => Some(Source::Int34),
            35 => Some(Source::Int35),
            36 => Some(Source::Int36),
            37 => Some(Source::Int37),
            38 => Some(Source::Int38),
            39 => Some(Source::Int39),
            40 => Some(Source::Int40),
            41 => Some(Source::Int41),
            42 => Some(Source::Int42),
            43 => Some(Source::Int43),
            44 => Some(Source::Int44),
            45 => Some(Source::Int45),
            46 => Some(Source::Int46),
            47 => Some(Source::Int47),
            48 => Some(Source::Int48),
            49 => Some(Source::Int49),
            50 => Some(Source::Int50),
            51 => Some(Source::Int51),
            52 => Some(Source::Int52),
            53 => Some(Source::Int53),
            54 => Some(Source::Int54),
            55 => Some(Source::Int55),
            56 => Some(Source::Int56),
            57 => Some(Source::Int57),
            58 => Some(Source::Int58),
            59 => Some(Source::Int59),
            60 => Some(Source::Int60),
            61 => Some(Source::Int61),
            62 => Some(Source::Int62),
            63 => Some(Source::Int63),
            64 => Some(Source::Timer),
            65 => Some(Source::Mailbox),
            66 => Some(Source::Doorbell0),
            67 => Some(Source::Doorbell1),
            68 => Some(Source::Vpu0Halted),
            69 => Some(Source::Vpu1Halted),
            70 => Some(Source::ArmAddressError),
            71 => Some(Source::ArmAxiError),
            _ => None,
        }
    }
    #[doc = "Interrupt 0"]
    #[inline(always)]
    pub fn is_int0(&self) -> bool {
        *self == Source::Int0
    }
    #[doc = "Interrupt 1"]
    #[inline(always)]
    pub fn is_int1(&self) -> bool {
        *self == Source::Int1
    }
    #[doc = "Interrupt 2"]
    #[inline(always)]
    pub fn is_int2(&self) -> bool {
        *self == Source::Int2
    }
    #[doc = "Interrupt 3"]
    #[inline(always)]
    pub fn is_int3(&self) -> bool {
        *self == Source::Int3
    }
    #[doc = "Interrupt 4"]
    #[inline(always)]
    pub fn is_int4(&self) -> bool {
        *self == Source::Int4
    }
    #[doc = "Interrupt 5"]
    #[inline(always)]
    pub fn is_int5(&self) -> bool {
        *self == Source::Int5
    }
    #[doc = "Interrupt 6"]
    #[inline(always)]
    pub fn is_int6(&self) -> bool {
        *self == Source::Int6
    }
    #[doc = "Interrupt 7"]
    #[inline(always)]
    pub fn is_int7(&self) -> bool {
        *self == Source::Int7
    }
    #[doc = "Interrupt 8"]
    #[inline(always)]
    pub fn is_int8(&self) -> bool {
        *self == Source::Int8
    }
    #[doc = "Interrupt 9"]
    #[inline(always)]
    pub fn is_int9(&self) -> bool {
        *self == Source::Int9
    }
    #[doc = "Interrupt 10"]
    #[inline(always)]
    pub fn is_int10(&self) -> bool {
        *self == Source::Int10
    }
    #[doc = "Interrupt 11"]
    #[inline(always)]
    pub fn is_int11(&self) -> bool {
        *self == Source::Int11
    }
    #[doc = "Interrupt 12"]
    #[inline(always)]
    pub fn is_int12(&self) -> bool {
        *self == Source::Int12
    }
    #[doc = "Interrupt 13"]
    #[inline(always)]
    pub fn is_int13(&self) -> bool {
        *self == Source::Int13
    }
    #[doc = "Interrupt 14"]
    #[inline(always)]
    pub fn is_int14(&self) -> bool {
        *self == Source::Int14
    }
    #[doc = "Interrupt 15"]
    #[inline(always)]
    pub fn is_int15(&self) -> bool {
        *self == Source::Int15
    }
    #[doc = "Interrupt 16"]
    #[inline(always)]
    pub fn is_int16(&self) -> bool {
        *self == Source::Int16
    }
    #[doc = "Interrupt 17"]
    #[inline(always)]
    pub fn is_int17(&self) -> bool {
        *self == Source::Int17
    }
    #[doc = "Interrupt 18"]
    #[inline(always)]
    pub fn is_int18(&self) -> bool {
        *self == Source::Int18
    }
    #[doc = "Interrupt 19"]
    #[inline(always)]
    pub fn is_int19(&self) -> bool {
        *self == Source::Int19
    }
    #[doc = "Interrupt 20"]
    #[inline(always)]
    pub fn is_int20(&self) -> bool {
        *self == Source::Int20
    }
    #[doc = "Interrupt 21"]
    #[inline(always)]
    pub fn is_int21(&self) -> bool {
        *self == Source::Int21
    }
    #[doc = "Interrupt 22"]
    #[inline(always)]
    pub fn is_int22(&self) -> bool {
        *self == Source::Int22
    }
    #[doc = "Interrupt 23"]
    #[inline(always)]
    pub fn is_int23(&self) -> bool {
        *self == Source::Int23
    }
    #[doc = "Interrupt 24"]
    #[inline(always)]
    pub fn is_int24(&self) -> bool {
        *self == Source::Int24
    }
    #[doc = "Interrupt 25"]
    #[inline(always)]
    pub fn is_int25(&self) -> bool {
        *self == Source::Int25
    }
    #[doc = "Interrupt 26"]
    #[inline(always)]
    pub fn is_int26(&self) -> bool {
        *self == Source::Int26
    }
    #[doc = "Interrupt 27"]
    #[inline(always)]
    pub fn is_int27(&self) -> bool {
        *self == Source::Int27
    }
    #[doc = "Interrupt 28"]
    #[inline(always)]
    pub fn is_int28(&self) -> bool {
        *self == Source::Int28
    }
    #[doc = "Interrupt 29"]
    #[inline(always)]
    pub fn is_int29(&self) -> bool {
        *self == Source::Int29
    }
    #[doc = "Interrupt 30"]
    #[inline(always)]
    pub fn is_int30(&self) -> bool {
        *self == Source::Int30
    }
    #[doc = "Interrupt 31"]
    #[inline(always)]
    pub fn is_int31(&self) -> bool {
        *self == Source::Int31
    }
    #[doc = "Interrupt 32"]
    #[inline(always)]
    pub fn is_int32(&self) -> bool {
        *self == Source::Int32
    }
    #[doc = "Interrupt 33"]
    #[inline(always)]
    pub fn is_int33(&self) -> bool {
        *self == Source::Int33
    }
    #[doc = "Interrupt 34"]
    #[inline(always)]
    pub fn is_int34(&self) -> bool {
        *self == Source::Int34
    }
    #[doc = "Interrupt 35"]
    #[inline(always)]
    pub fn is_int35(&self) -> bool {
        *self == Source::Int35
    }
    #[doc = "Interrupt 36"]
    #[inline(always)]
    pub fn is_int36(&self) -> bool {
        *self == Source::Int36
    }
    #[doc = "Interrupt 37"]
    #[inline(always)]
    pub fn is_int37(&self) -> bool {
        *self == Source::Int37
    }
    #[doc = "Interrupt 38"]
    #[inline(always)]
    pub fn is_int38(&self) -> bool {
        *self == Source::Int38
    }
    #[doc = "Interrupt 39"]
    #[inline(always)]
    pub fn is_int39(&self) -> bool {
        *self == Source::Int39
    }
    #[doc = "Interrupt 40"]
    #[inline(always)]
    pub fn is_int40(&self) -> bool {
        *self == Source::Int40
    }
    #[doc = "Interrupt 41"]
    #[inline(always)]
    pub fn is_int41(&self) -> bool {
        *self == Source::Int41
    }
    #[doc = "Interrupt 42"]
    #[inline(always)]
    pub fn is_int42(&self) -> bool {
        *self == Source::Int42
    }
    #[doc = "Interrupt 43"]
    #[inline(always)]
    pub fn is_int43(&self) -> bool {
        *self == Source::Int43
    }
    #[doc = "Interrupt 44"]
    #[inline(always)]
    pub fn is_int44(&self) -> bool {
        *self == Source::Int44
    }
    #[doc = "Interrupt 45"]
    #[inline(always)]
    pub fn is_int45(&self) -> bool {
        *self == Source::Int45
    }
    #[doc = "Interrupt 46"]
    #[inline(always)]
    pub fn is_int46(&self) -> bool {
        *self == Source::Int46
    }
    #[doc = "Interrupt 47"]
    #[inline(always)]
    pub fn is_int47(&self) -> bool {
        *self == Source::Int47
    }
    #[doc = "Interrupt 48"]
    #[inline(always)]
    pub fn is_int48(&self) -> bool {
        *self == Source::Int48
    }
    #[doc = "Interrupt 49"]
    #[inline(always)]
    pub fn is_int49(&self) -> bool {
        *self == Source::Int49
    }
    #[doc = "Interrupt 50"]
    #[inline(always)]
    pub fn is_int50(&self) -> bool {
        *self == Source::Int50
    }
    #[doc = "Interrupt 51"]
    #[inline(always)]
    pub fn is_int51(&self) -> bool {
        *self == Source::Int51
    }
    #[doc = "Interrupt 52"]
    #[inline(always)]
    pub fn is_int52(&self) -> bool {
        *self == Source::Int52
    }
    #[doc = "Interrupt 53"]
    #[inline(always)]
    pub fn is_int53(&self) -> bool {
        *self == Source::Int53
    }
    #[doc = "Interrupt 54"]
    #[inline(always)]
    pub fn is_int54(&self) -> bool {
        *self == Source::Int54
    }
    #[doc = "Interrupt 55"]
    #[inline(always)]
    pub fn is_int55(&self) -> bool {
        *self == Source::Int55
    }
    #[doc = "Interrupt 56"]
    #[inline(always)]
    pub fn is_int56(&self) -> bool {
        *self == Source::Int56
    }
    #[doc = "Interrupt 57"]
    #[inline(always)]
    pub fn is_int57(&self) -> bool {
        *self == Source::Int57
    }
    #[doc = "Interrupt 58"]
    #[inline(always)]
    pub fn is_int58(&self) -> bool {
        *self == Source::Int58
    }
    #[doc = "Interrupt 59"]
    #[inline(always)]
    pub fn is_int59(&self) -> bool {
        *self == Source::Int59
    }
    #[doc = "Interrupt 60"]
    #[inline(always)]
    pub fn is_int60(&self) -> bool {
        *self == Source::Int60
    }
    #[doc = "Interrupt 61"]
    #[inline(always)]
    pub fn is_int61(&self) -> bool {
        *self == Source::Int61
    }
    #[doc = "Interrupt 62"]
    #[inline(always)]
    pub fn is_int62(&self) -> bool {
        *self == Source::Int62
    }
    #[doc = "Interrupt 63"]
    #[inline(always)]
    pub fn is_int63(&self) -> bool {
        *self == Source::Int63
    }
    #[doc = "ARMC Timer"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == Source::Timer
    }
    #[doc = "Mailbox"]
    #[inline(always)]
    pub fn is_mailbox(&self) -> bool {
        *self == Source::Mailbox
    }
    #[doc = "Doorbell 0"]
    #[inline(always)]
    pub fn is_doorbell0(&self) -> bool {
        *self == Source::Doorbell0
    }
    #[doc = "Doorbell 1"]
    #[inline(always)]
    pub fn is_doorbell1(&self) -> bool {
        *self == Source::Doorbell1
    }
    #[doc = "VPU0 halted"]
    #[inline(always)]
    pub fn is_vpu0_halted(&self) -> bool {
        *self == Source::Vpu0Halted
    }
    #[doc = "VPU1 halted"]
    #[inline(always)]
    pub fn is_vpu1_halted(&self) -> bool {
        *self == Source::Vpu1Halted
    }
    #[doc = "ARM address error"]
    #[inline(always)]
    pub fn is_arm_address_error(&self) -> bool {
        *self == Source::ArmAddressError
    }
    #[doc = "ARM AXI error"]
    #[inline(always)]
    pub fn is_arm_axi_error(&self) -> bool {
        *self == Source::ArmAxiError
    }
}
#[doc = "Field `SOURCE` writer - FIQ Source"]
pub type SourceW<'a, REG> = crate::FieldWriter<'a, REG, 7, Source>;
impl<'a, REG> SourceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt 0"]
    #[inline(always)]
    pub fn int0(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int0)
    }
    #[doc = "Interrupt 1"]
    #[inline(always)]
    pub fn int1(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int1)
    }
    #[doc = "Interrupt 2"]
    #[inline(always)]
    pub fn int2(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int2)
    }
    #[doc = "Interrupt 3"]
    #[inline(always)]
    pub fn int3(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int3)
    }
    #[doc = "Interrupt 4"]
    #[inline(always)]
    pub fn int4(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int4)
    }
    #[doc = "Interrupt 5"]
    #[inline(always)]
    pub fn int5(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int5)
    }
    #[doc = "Interrupt 6"]
    #[inline(always)]
    pub fn int6(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int6)
    }
    #[doc = "Interrupt 7"]
    #[inline(always)]
    pub fn int7(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int7)
    }
    #[doc = "Interrupt 8"]
    #[inline(always)]
    pub fn int8(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int8)
    }
    #[doc = "Interrupt 9"]
    #[inline(always)]
    pub fn int9(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int9)
    }
    #[doc = "Interrupt 10"]
    #[inline(always)]
    pub fn int10(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int10)
    }
    #[doc = "Interrupt 11"]
    #[inline(always)]
    pub fn int11(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int11)
    }
    #[doc = "Interrupt 12"]
    #[inline(always)]
    pub fn int12(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int12)
    }
    #[doc = "Interrupt 13"]
    #[inline(always)]
    pub fn int13(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int13)
    }
    #[doc = "Interrupt 14"]
    #[inline(always)]
    pub fn int14(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int14)
    }
    #[doc = "Interrupt 15"]
    #[inline(always)]
    pub fn int15(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int15)
    }
    #[doc = "Interrupt 16"]
    #[inline(always)]
    pub fn int16(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int16)
    }
    #[doc = "Interrupt 17"]
    #[inline(always)]
    pub fn int17(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int17)
    }
    #[doc = "Interrupt 18"]
    #[inline(always)]
    pub fn int18(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int18)
    }
    #[doc = "Interrupt 19"]
    #[inline(always)]
    pub fn int19(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int19)
    }
    #[doc = "Interrupt 20"]
    #[inline(always)]
    pub fn int20(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int20)
    }
    #[doc = "Interrupt 21"]
    #[inline(always)]
    pub fn int21(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int21)
    }
    #[doc = "Interrupt 22"]
    #[inline(always)]
    pub fn int22(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int22)
    }
    #[doc = "Interrupt 23"]
    #[inline(always)]
    pub fn int23(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int23)
    }
    #[doc = "Interrupt 24"]
    #[inline(always)]
    pub fn int24(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int24)
    }
    #[doc = "Interrupt 25"]
    #[inline(always)]
    pub fn int25(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int25)
    }
    #[doc = "Interrupt 26"]
    #[inline(always)]
    pub fn int26(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int26)
    }
    #[doc = "Interrupt 27"]
    #[inline(always)]
    pub fn int27(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int27)
    }
    #[doc = "Interrupt 28"]
    #[inline(always)]
    pub fn int28(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int28)
    }
    #[doc = "Interrupt 29"]
    #[inline(always)]
    pub fn int29(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int29)
    }
    #[doc = "Interrupt 30"]
    #[inline(always)]
    pub fn int30(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int30)
    }
    #[doc = "Interrupt 31"]
    #[inline(always)]
    pub fn int31(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int31)
    }
    #[doc = "Interrupt 32"]
    #[inline(always)]
    pub fn int32(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int32)
    }
    #[doc = "Interrupt 33"]
    #[inline(always)]
    pub fn int33(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int33)
    }
    #[doc = "Interrupt 34"]
    #[inline(always)]
    pub fn int34(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int34)
    }
    #[doc = "Interrupt 35"]
    #[inline(always)]
    pub fn int35(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int35)
    }
    #[doc = "Interrupt 36"]
    #[inline(always)]
    pub fn int36(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int36)
    }
    #[doc = "Interrupt 37"]
    #[inline(always)]
    pub fn int37(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int37)
    }
    #[doc = "Interrupt 38"]
    #[inline(always)]
    pub fn int38(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int38)
    }
    #[doc = "Interrupt 39"]
    #[inline(always)]
    pub fn int39(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int39)
    }
    #[doc = "Interrupt 40"]
    #[inline(always)]
    pub fn int40(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int40)
    }
    #[doc = "Interrupt 41"]
    #[inline(always)]
    pub fn int41(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int41)
    }
    #[doc = "Interrupt 42"]
    #[inline(always)]
    pub fn int42(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int42)
    }
    #[doc = "Interrupt 43"]
    #[inline(always)]
    pub fn int43(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int43)
    }
    #[doc = "Interrupt 44"]
    #[inline(always)]
    pub fn int44(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int44)
    }
    #[doc = "Interrupt 45"]
    #[inline(always)]
    pub fn int45(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int45)
    }
    #[doc = "Interrupt 46"]
    #[inline(always)]
    pub fn int46(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int46)
    }
    #[doc = "Interrupt 47"]
    #[inline(always)]
    pub fn int47(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int47)
    }
    #[doc = "Interrupt 48"]
    #[inline(always)]
    pub fn int48(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int48)
    }
    #[doc = "Interrupt 49"]
    #[inline(always)]
    pub fn int49(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int49)
    }
    #[doc = "Interrupt 50"]
    #[inline(always)]
    pub fn int50(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int50)
    }
    #[doc = "Interrupt 51"]
    #[inline(always)]
    pub fn int51(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int51)
    }
    #[doc = "Interrupt 52"]
    #[inline(always)]
    pub fn int52(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int52)
    }
    #[doc = "Interrupt 53"]
    #[inline(always)]
    pub fn int53(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int53)
    }
    #[doc = "Interrupt 54"]
    #[inline(always)]
    pub fn int54(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int54)
    }
    #[doc = "Interrupt 55"]
    #[inline(always)]
    pub fn int55(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int55)
    }
    #[doc = "Interrupt 56"]
    #[inline(always)]
    pub fn int56(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int56)
    }
    #[doc = "Interrupt 57"]
    #[inline(always)]
    pub fn int57(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int57)
    }
    #[doc = "Interrupt 58"]
    #[inline(always)]
    pub fn int58(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int58)
    }
    #[doc = "Interrupt 59"]
    #[inline(always)]
    pub fn int59(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int59)
    }
    #[doc = "Interrupt 60"]
    #[inline(always)]
    pub fn int60(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int60)
    }
    #[doc = "Interrupt 61"]
    #[inline(always)]
    pub fn int61(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int61)
    }
    #[doc = "Interrupt 62"]
    #[inline(always)]
    pub fn int62(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int62)
    }
    #[doc = "Interrupt 63"]
    #[inline(always)]
    pub fn int63(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Int63)
    }
    #[doc = "ARMC Timer"]
    #[inline(always)]
    pub fn timer(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Timer)
    }
    #[doc = "Mailbox"]
    #[inline(always)]
    pub fn mailbox(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Mailbox)
    }
    #[doc = "Doorbell 0"]
    #[inline(always)]
    pub fn doorbell0(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Doorbell0)
    }
    #[doc = "Doorbell 1"]
    #[inline(always)]
    pub fn doorbell1(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Doorbell1)
    }
    #[doc = "VPU0 halted"]
    #[inline(always)]
    pub fn vpu0_halted(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Vpu0Halted)
    }
    #[doc = "VPU1 halted"]
    #[inline(always)]
    pub fn vpu1_halted(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Vpu1Halted)
    }
    #[doc = "ARM address error"]
    #[inline(always)]
    pub fn arm_address_error(self) -> &'a mut crate::W<REG> {
        self.variant(Source::ArmAddressError)
    }
    #[doc = "ARM AXI error"]
    #[inline(always)]
    pub fn arm_axi_error(self) -> &'a mut crate::W<REG> {
        self.variant(Source::ArmAxiError)
    }
}
#[doc = "Field `ENABLE` reader - FIQ Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - FIQ Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - FIQ Source"]
    #[inline(always)]
    pub fn source(&self) -> SourceR {
        SourceR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - FIQ Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIQ_CONTROL")
            .field("enable", &self.enable())
            .field("source", &self.source())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - FIQ Source"]
    #[inline(always)]
    #[must_use]
    pub fn source(&mut self) -> SourceW<FiqControlSpec> {
        SourceW::new(self, 0)
    }
    #[doc = "Bit 7 - FIQ Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<FiqControlSpec> {
        EnableW::new(self, 7)
    }
}
#[doc = "FIQ control\n\nYou can [`read`](crate::Reg::read) this register and get [`fiq_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiq_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FiqControlSpec;
impl crate::RegisterSpec for FiqControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fiq_control::R`](R) reader structure"]
impl crate::Readable for FiqControlSpec {}
#[doc = "`write(|w| ..)` method takes [`fiq_control::W`](W) writer structure"]
impl crate::Writable for FiqControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIQ_CONTROL to value 0"]
impl crate::Resettable for FiqControlSpec {
    const RESET_VALUE: u32 = 0;
}

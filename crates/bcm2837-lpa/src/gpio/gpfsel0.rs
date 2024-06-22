#[doc = "Register `GPFSEL0` reader"]
pub type R = crate::R<Gpfsel0Spec>;
#[doc = "Register `GPFSEL0` writer"]
pub type W = crate::W<Gpfsel0Spec>;
#[doc = "Function Select 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel0 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Pin is connected to SDA0"]
    Sda0 = 4,
    #[doc = "5: Pin is connected to SA5"]
    Sa5 = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    Reserved3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    Reserved4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    Reserved5 = 2,
}
impl From<Fsel0> for u8 {
    #[inline(always)]
    fn from(variant: Fsel0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel0 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel0 {}
#[doc = "Field `FSEL0` reader - Function Select 0"]
pub type Fsel0R = crate::FieldReader<Fsel0>;
impl Fsel0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel0 {
        match self.bits {
            0 => Fsel0::Input,
            1 => Fsel0::Output,
            4 => Fsel0::Sda0,
            5 => Fsel0::Sa5,
            6 => Fsel0::Reserved2,
            7 => Fsel0::Reserved3,
            3 => Fsel0::Reserved4,
            2 => Fsel0::Reserved5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel0::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel0::Output
    }
    #[doc = "Pin is connected to SDA0"]
    #[inline(always)]
    pub fn is_sda0(&self) -> bool {
        *self == Fsel0::Sda0
    }
    #[doc = "Pin is connected to SA5"]
    #[inline(always)]
    pub fn is_sa5(&self) -> bool {
        *self == Fsel0::Sa5
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel0::Reserved2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == Fsel0::Reserved3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fsel0::Reserved4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fsel0::Reserved5
    }
}
#[doc = "Field `FSEL0` writer - Function Select 0"]
pub type Fsel0W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel0, crate::Safe>;
impl<'a, REG> Fsel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel0::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel0::Output)
    }
    #[doc = "Pin is connected to SDA0"]
    #[inline(always)]
    pub fn sda0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel0::Sda0)
    }
    #[doc = "Pin is connected to SA5"]
    #[inline(always)]
    pub fn sa5(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel0::Sa5)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel0::Reserved2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel0::Reserved3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel0::Reserved4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel0::Reserved5)
    }
}
#[doc = "Function Select 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel1 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Pin is connected to SCL0"]
    Scl0 = 4,
    #[doc = "5: Pin is connected to SA4"]
    Sa4 = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    Reserved3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    Reserved4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    Reserved5 = 2,
}
impl From<Fsel1> for u8 {
    #[inline(always)]
    fn from(variant: Fsel1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel1 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel1 {}
#[doc = "Field `FSEL1` reader - Function Select 1"]
pub type Fsel1R = crate::FieldReader<Fsel1>;
impl Fsel1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel1 {
        match self.bits {
            0 => Fsel1::Input,
            1 => Fsel1::Output,
            4 => Fsel1::Scl0,
            5 => Fsel1::Sa4,
            6 => Fsel1::Reserved2,
            7 => Fsel1::Reserved3,
            3 => Fsel1::Reserved4,
            2 => Fsel1::Reserved5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel1::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel1::Output
    }
    #[doc = "Pin is connected to SCL0"]
    #[inline(always)]
    pub fn is_scl0(&self) -> bool {
        *self == Fsel1::Scl0
    }
    #[doc = "Pin is connected to SA4"]
    #[inline(always)]
    pub fn is_sa4(&self) -> bool {
        *self == Fsel1::Sa4
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel1::Reserved2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == Fsel1::Reserved3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fsel1::Reserved4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fsel1::Reserved5
    }
}
#[doc = "Field `FSEL1` writer - Function Select 1"]
pub type Fsel1W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel1, crate::Safe>;
impl<'a, REG> Fsel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel1::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel1::Output)
    }
    #[doc = "Pin is connected to SCL0"]
    #[inline(always)]
    pub fn scl0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel1::Scl0)
    }
    #[doc = "Pin is connected to SA4"]
    #[inline(always)]
    pub fn sa4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel1::Sa4)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel1::Reserved2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel1::Reserved3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel1::Reserved4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel1::Reserved5)
    }
}
#[doc = "Function Select 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel2 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Pin is connected to SDA1"]
    Sda1 = 4,
    #[doc = "5: Pin is connected to SA3"]
    Sa3 = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    Reserved3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    Reserved4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    Reserved5 = 2,
}
impl From<Fsel2> for u8 {
    #[inline(always)]
    fn from(variant: Fsel2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel2 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel2 {}
#[doc = "Field `FSEL2` reader - Function Select 2"]
pub type Fsel2R = crate::FieldReader<Fsel2>;
impl Fsel2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel2 {
        match self.bits {
            0 => Fsel2::Input,
            1 => Fsel2::Output,
            4 => Fsel2::Sda1,
            5 => Fsel2::Sa3,
            6 => Fsel2::Reserved2,
            7 => Fsel2::Reserved3,
            3 => Fsel2::Reserved4,
            2 => Fsel2::Reserved5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel2::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel2::Output
    }
    #[doc = "Pin is connected to SDA1"]
    #[inline(always)]
    pub fn is_sda1(&self) -> bool {
        *self == Fsel2::Sda1
    }
    #[doc = "Pin is connected to SA3"]
    #[inline(always)]
    pub fn is_sa3(&self) -> bool {
        *self == Fsel2::Sa3
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel2::Reserved2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == Fsel2::Reserved3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fsel2::Reserved4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fsel2::Reserved5
    }
}
#[doc = "Field `FSEL2` writer - Function Select 2"]
pub type Fsel2W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel2, crate::Safe>;
impl<'a, REG> Fsel2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel2::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel2::Output)
    }
    #[doc = "Pin is connected to SDA1"]
    #[inline(always)]
    pub fn sda1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel2::Sda1)
    }
    #[doc = "Pin is connected to SA3"]
    #[inline(always)]
    pub fn sa3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel2::Sa3)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel2::Reserved2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel2::Reserved3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel2::Reserved4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel2::Reserved5)
    }
}
#[doc = "Function Select 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel3 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Pin is connected to SCL1"]
    Scl1 = 4,
    #[doc = "5: Pin is connected to SA2"]
    Sa2 = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    Reserved3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    Reserved4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    Reserved5 = 2,
}
impl From<Fsel3> for u8 {
    #[inline(always)]
    fn from(variant: Fsel3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel3 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel3 {}
#[doc = "Field `FSEL3` reader - Function Select 3"]
pub type Fsel3R = crate::FieldReader<Fsel3>;
impl Fsel3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel3 {
        match self.bits {
            0 => Fsel3::Input,
            1 => Fsel3::Output,
            4 => Fsel3::Scl1,
            5 => Fsel3::Sa2,
            6 => Fsel3::Reserved2,
            7 => Fsel3::Reserved3,
            3 => Fsel3::Reserved4,
            2 => Fsel3::Reserved5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel3::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel3::Output
    }
    #[doc = "Pin is connected to SCL1"]
    #[inline(always)]
    pub fn is_scl1(&self) -> bool {
        *self == Fsel3::Scl1
    }
    #[doc = "Pin is connected to SA2"]
    #[inline(always)]
    pub fn is_sa2(&self) -> bool {
        *self == Fsel3::Sa2
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel3::Reserved2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == Fsel3::Reserved3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fsel3::Reserved4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fsel3::Reserved5
    }
}
#[doc = "Field `FSEL3` writer - Function Select 3"]
pub type Fsel3W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel3, crate::Safe>;
impl<'a, REG> Fsel3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel3::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel3::Output)
    }
    #[doc = "Pin is connected to SCL1"]
    #[inline(always)]
    pub fn scl1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel3::Scl1)
    }
    #[doc = "Pin is connected to SA2"]
    #[inline(always)]
    pub fn sa2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel3::Sa2)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel3::Reserved2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel3::Reserved3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel3::Reserved4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel3::Reserved5)
    }
}
#[doc = "Function Select 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel4 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Pin is connected to GPCLK0"]
    Gpclk0 = 4,
    #[doc = "5: Pin is connected to SA1"]
    Sa1 = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    Reserved3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    Reserved4 = 3,
    #[doc = "2: Pin is connected to ARM_TDI"]
    ArmTdi = 2,
}
impl From<Fsel4> for u8 {
    #[inline(always)]
    fn from(variant: Fsel4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel4 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel4 {}
#[doc = "Field `FSEL4` reader - Function Select 4"]
pub type Fsel4R = crate::FieldReader<Fsel4>;
impl Fsel4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel4 {
        match self.bits {
            0 => Fsel4::Input,
            1 => Fsel4::Output,
            4 => Fsel4::Gpclk0,
            5 => Fsel4::Sa1,
            6 => Fsel4::Reserved2,
            7 => Fsel4::Reserved3,
            3 => Fsel4::Reserved4,
            2 => Fsel4::ArmTdi,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel4::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel4::Output
    }
    #[doc = "Pin is connected to GPCLK0"]
    #[inline(always)]
    pub fn is_gpclk0(&self) -> bool {
        *self == Fsel4::Gpclk0
    }
    #[doc = "Pin is connected to SA1"]
    #[inline(always)]
    pub fn is_sa1(&self) -> bool {
        *self == Fsel4::Sa1
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel4::Reserved2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == Fsel4::Reserved3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fsel4::Reserved4
    }
    #[doc = "Pin is connected to ARM_TDI"]
    #[inline(always)]
    pub fn is_arm_tdi(&self) -> bool {
        *self == Fsel4::ArmTdi
    }
}
#[doc = "Field `FSEL4` writer - Function Select 4"]
pub type Fsel4W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel4, crate::Safe>;
impl<'a, REG> Fsel4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel4::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel4::Output)
    }
    #[doc = "Pin is connected to GPCLK0"]
    #[inline(always)]
    pub fn gpclk0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel4::Gpclk0)
    }
    #[doc = "Pin is connected to SA1"]
    #[inline(always)]
    pub fn sa1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel4::Sa1)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel4::Reserved2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel4::Reserved3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel4::Reserved4)
    }
    #[doc = "Pin is connected to ARM_TDI"]
    #[inline(always)]
    pub fn arm_tdi(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel4::ArmTdi)
    }
}
#[doc = "Function Select 5"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel5 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Pin is connected to GPCLK1"]
    Gpclk1 = 4,
    #[doc = "5: Pin is connected to SA0"]
    Sa0 = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    Reserved3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    Reserved4 = 3,
    #[doc = "2: Pin is connected to ARM_TDO"]
    ArmTdo = 2,
}
impl From<Fsel5> for u8 {
    #[inline(always)]
    fn from(variant: Fsel5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel5 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel5 {}
#[doc = "Field `FSEL5` reader - Function Select 5"]
pub type Fsel5R = crate::FieldReader<Fsel5>;
impl Fsel5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel5 {
        match self.bits {
            0 => Fsel5::Input,
            1 => Fsel5::Output,
            4 => Fsel5::Gpclk1,
            5 => Fsel5::Sa0,
            6 => Fsel5::Reserved2,
            7 => Fsel5::Reserved3,
            3 => Fsel5::Reserved4,
            2 => Fsel5::ArmTdo,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel5::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel5::Output
    }
    #[doc = "Pin is connected to GPCLK1"]
    #[inline(always)]
    pub fn is_gpclk1(&self) -> bool {
        *self == Fsel5::Gpclk1
    }
    #[doc = "Pin is connected to SA0"]
    #[inline(always)]
    pub fn is_sa0(&self) -> bool {
        *self == Fsel5::Sa0
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel5::Reserved2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == Fsel5::Reserved3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fsel5::Reserved4
    }
    #[doc = "Pin is connected to ARM_TDO"]
    #[inline(always)]
    pub fn is_arm_tdo(&self) -> bool {
        *self == Fsel5::ArmTdo
    }
}
#[doc = "Field `FSEL5` writer - Function Select 5"]
pub type Fsel5W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel5, crate::Safe>;
impl<'a, REG> Fsel5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel5::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel5::Output)
    }
    #[doc = "Pin is connected to GPCLK1"]
    #[inline(always)]
    pub fn gpclk1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel5::Gpclk1)
    }
    #[doc = "Pin is connected to SA0"]
    #[inline(always)]
    pub fn sa0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel5::Sa0)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel5::Reserved2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel5::Reserved3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel5::Reserved4)
    }
    #[doc = "Pin is connected to ARM_TDO"]
    #[inline(always)]
    pub fn arm_tdo(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel5::ArmTdo)
    }
}
#[doc = "Function Select 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel6 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Pin is connected to GPCLK2"]
    Gpclk2 = 4,
    #[doc = "5: Pin is connected to SOE_N"]
    SoeN = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    Reserved3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    Reserved4 = 3,
    #[doc = "2: Pin is connected to ARM_RTCK"]
    ArmRtck = 2,
}
impl From<Fsel6> for u8 {
    #[inline(always)]
    fn from(variant: Fsel6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel6 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel6 {}
#[doc = "Field `FSEL6` reader - Function Select 6"]
pub type Fsel6R = crate::FieldReader<Fsel6>;
impl Fsel6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel6 {
        match self.bits {
            0 => Fsel6::Input,
            1 => Fsel6::Output,
            4 => Fsel6::Gpclk2,
            5 => Fsel6::SoeN,
            6 => Fsel6::Reserved2,
            7 => Fsel6::Reserved3,
            3 => Fsel6::Reserved4,
            2 => Fsel6::ArmRtck,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel6::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel6::Output
    }
    #[doc = "Pin is connected to GPCLK2"]
    #[inline(always)]
    pub fn is_gpclk2(&self) -> bool {
        *self == Fsel6::Gpclk2
    }
    #[doc = "Pin is connected to SOE_N"]
    #[inline(always)]
    pub fn is_soe_n(&self) -> bool {
        *self == Fsel6::SoeN
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel6::Reserved2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == Fsel6::Reserved3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fsel6::Reserved4
    }
    #[doc = "Pin is connected to ARM_RTCK"]
    #[inline(always)]
    pub fn is_arm_rtck(&self) -> bool {
        *self == Fsel6::ArmRtck
    }
}
#[doc = "Field `FSEL6` writer - Function Select 6"]
pub type Fsel6W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel6, crate::Safe>;
impl<'a, REG> Fsel6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel6::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel6::Output)
    }
    #[doc = "Pin is connected to GPCLK2"]
    #[inline(always)]
    pub fn gpclk2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel6::Gpclk2)
    }
    #[doc = "Pin is connected to SOE_N"]
    #[inline(always)]
    pub fn soe_n(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel6::SoeN)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel6::Reserved2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel6::Reserved3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel6::Reserved4)
    }
    #[doc = "Pin is connected to ARM_RTCK"]
    #[inline(always)]
    pub fn arm_rtck(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel6::ArmRtck)
    }
}
#[doc = "Function Select 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel7 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Pin is connected to SPI0_CE1_N"]
    Spi0Ce1N = 4,
    #[doc = "5: Pin is connected to SWE_N"]
    SweN = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    Reserved3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    Reserved4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    Reserved5 = 2,
}
impl From<Fsel7> for u8 {
    #[inline(always)]
    fn from(variant: Fsel7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel7 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel7 {}
#[doc = "Field `FSEL7` reader - Function Select 7"]
pub type Fsel7R = crate::FieldReader<Fsel7>;
impl Fsel7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel7 {
        match self.bits {
            0 => Fsel7::Input,
            1 => Fsel7::Output,
            4 => Fsel7::Spi0Ce1N,
            5 => Fsel7::SweN,
            6 => Fsel7::Reserved2,
            7 => Fsel7::Reserved3,
            3 => Fsel7::Reserved4,
            2 => Fsel7::Reserved5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel7::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel7::Output
    }
    #[doc = "Pin is connected to SPI0_CE1_N"]
    #[inline(always)]
    pub fn is_spi0_ce1_n(&self) -> bool {
        *self == Fsel7::Spi0Ce1N
    }
    #[doc = "Pin is connected to SWE_N"]
    #[inline(always)]
    pub fn is_swe_n(&self) -> bool {
        *self == Fsel7::SweN
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel7::Reserved2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == Fsel7::Reserved3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fsel7::Reserved4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fsel7::Reserved5
    }
}
#[doc = "Field `FSEL7` writer - Function Select 7"]
pub type Fsel7W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel7, crate::Safe>;
impl<'a, REG> Fsel7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel7::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel7::Output)
    }
    #[doc = "Pin is connected to SPI0_CE1_N"]
    #[inline(always)]
    pub fn spi0_ce1_n(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel7::Spi0Ce1N)
    }
    #[doc = "Pin is connected to SWE_N"]
    #[inline(always)]
    pub fn swe_n(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel7::SweN)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel7::Reserved2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel7::Reserved3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel7::Reserved4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel7::Reserved5)
    }
}
#[doc = "Function Select 8"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel8 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Pin is connected to SPI0_CE0_N"]
    Spi0Ce0N = 4,
    #[doc = "5: Pin is connected to SD0"]
    Sd0 = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    Reserved3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    Reserved4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    Reserved5 = 2,
}
impl From<Fsel8> for u8 {
    #[inline(always)]
    fn from(variant: Fsel8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel8 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel8 {}
#[doc = "Field `FSEL8` reader - Function Select 8"]
pub type Fsel8R = crate::FieldReader<Fsel8>;
impl Fsel8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel8 {
        match self.bits {
            0 => Fsel8::Input,
            1 => Fsel8::Output,
            4 => Fsel8::Spi0Ce0N,
            5 => Fsel8::Sd0,
            6 => Fsel8::Reserved2,
            7 => Fsel8::Reserved3,
            3 => Fsel8::Reserved4,
            2 => Fsel8::Reserved5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel8::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel8::Output
    }
    #[doc = "Pin is connected to SPI0_CE0_N"]
    #[inline(always)]
    pub fn is_spi0_ce0_n(&self) -> bool {
        *self == Fsel8::Spi0Ce0N
    }
    #[doc = "Pin is connected to SD0"]
    #[inline(always)]
    pub fn is_sd0(&self) -> bool {
        *self == Fsel8::Sd0
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel8::Reserved2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == Fsel8::Reserved3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fsel8::Reserved4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fsel8::Reserved5
    }
}
#[doc = "Field `FSEL8` writer - Function Select 8"]
pub type Fsel8W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel8, crate::Safe>;
impl<'a, REG> Fsel8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel8::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel8::Output)
    }
    #[doc = "Pin is connected to SPI0_CE0_N"]
    #[inline(always)]
    pub fn spi0_ce0_n(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel8::Spi0Ce0N)
    }
    #[doc = "Pin is connected to SD0"]
    #[inline(always)]
    pub fn sd0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel8::Sd0)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel8::Reserved2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel8::Reserved3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel8::Reserved4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel8::Reserved5)
    }
}
#[doc = "Function Select 9"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel9 {
    #[doc = "0: Pin is an input"]
    Input = 0,
    #[doc = "1: Pin is an output"]
    Output = 1,
    #[doc = "4: Pin is connected to SPI0_MISO"]
    Spi0Miso = 4,
    #[doc = "5: Pin is connected to SD1"]
    Sd1 = 5,
    #[doc = "6: Alt function 2 reserved"]
    Reserved2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    Reserved3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    Reserved4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    Reserved5 = 2,
}
impl From<Fsel9> for u8 {
    #[inline(always)]
    fn from(variant: Fsel9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel9 {
    type Ux = u8;
}
impl crate::IsEnum for Fsel9 {}
#[doc = "Field `FSEL9` reader - Function Select 9"]
pub type Fsel9R = crate::FieldReader<Fsel9>;
impl Fsel9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel9 {
        match self.bits {
            0 => Fsel9::Input,
            1 => Fsel9::Output,
            4 => Fsel9::Spi0Miso,
            5 => Fsel9::Sd1,
            6 => Fsel9::Reserved2,
            7 => Fsel9::Reserved3,
            3 => Fsel9::Reserved4,
            2 => Fsel9::Reserved5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Fsel9::Input
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Fsel9::Output
    }
    #[doc = "Pin is connected to SPI0_MISO"]
    #[inline(always)]
    pub fn is_spi0_miso(&self) -> bool {
        *self == Fsel9::Spi0Miso
    }
    #[doc = "Pin is connected to SD1"]
    #[inline(always)]
    pub fn is_sd1(&self) -> bool {
        *self == Fsel9::Sd1
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == Fsel9::Reserved2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == Fsel9::Reserved3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == Fsel9::Reserved4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == Fsel9::Reserved5
    }
}
#[doc = "Field `FSEL9` writer - Function Select 9"]
pub type Fsel9W<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsel9, crate::Safe>;
impl<'a, REG> Fsel9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel9::Input)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel9::Output)
    }
    #[doc = "Pin is connected to SPI0_MISO"]
    #[inline(always)]
    pub fn spi0_miso(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel9::Spi0Miso)
    }
    #[doc = "Pin is connected to SD1"]
    #[inline(always)]
    pub fn sd1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel9::Sd1)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel9::Reserved2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel9::Reserved3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel9::Reserved4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel9::Reserved5)
    }
}
impl R {
    #[doc = "Bits 0:2 - Function Select 0"]
    #[inline(always)]
    pub fn fsel0(&self) -> Fsel0R {
        Fsel0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Function Select 1"]
    #[inline(always)]
    pub fn fsel1(&self) -> Fsel1R {
        Fsel1R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Function Select 2"]
    #[inline(always)]
    pub fn fsel2(&self) -> Fsel2R {
        Fsel2R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Function Select 3"]
    #[inline(always)]
    pub fn fsel3(&self) -> Fsel3R {
        Fsel3R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Function Select 4"]
    #[inline(always)]
    pub fn fsel4(&self) -> Fsel4R {
        Fsel4R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Function Select 5"]
    #[inline(always)]
    pub fn fsel5(&self) -> Fsel5R {
        Fsel5R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Function Select 6"]
    #[inline(always)]
    pub fn fsel6(&self) -> Fsel6R {
        Fsel6R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Function Select 7"]
    #[inline(always)]
    pub fn fsel7(&self) -> Fsel7R {
        Fsel7R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Function Select 8"]
    #[inline(always)]
    pub fn fsel8(&self) -> Fsel8R {
        Fsel8R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - Function Select 9"]
    #[inline(always)]
    pub fn fsel9(&self) -> Fsel9R {
        Fsel9R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPFSEL0")
            .field("fsel0", &self.fsel0())
            .field("fsel1", &self.fsel1())
            .field("fsel2", &self.fsel2())
            .field("fsel3", &self.fsel3())
            .field("fsel4", &self.fsel4())
            .field("fsel5", &self.fsel5())
            .field("fsel6", &self.fsel6())
            .field("fsel7", &self.fsel7())
            .field("fsel8", &self.fsel8())
            .field("fsel9", &self.fsel9())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Function Select 0"]
    #[inline(always)]
    #[must_use]
    pub fn fsel0(&mut self) -> Fsel0W<Gpfsel0Spec> {
        Fsel0W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Function Select 1"]
    #[inline(always)]
    #[must_use]
    pub fn fsel1(&mut self) -> Fsel1W<Gpfsel0Spec> {
        Fsel1W::new(self, 3)
    }
    #[doc = "Bits 6:8 - Function Select 2"]
    #[inline(always)]
    #[must_use]
    pub fn fsel2(&mut self) -> Fsel2W<Gpfsel0Spec> {
        Fsel2W::new(self, 6)
    }
    #[doc = "Bits 9:11 - Function Select 3"]
    #[inline(always)]
    #[must_use]
    pub fn fsel3(&mut self) -> Fsel3W<Gpfsel0Spec> {
        Fsel3W::new(self, 9)
    }
    #[doc = "Bits 12:14 - Function Select 4"]
    #[inline(always)]
    #[must_use]
    pub fn fsel4(&mut self) -> Fsel4W<Gpfsel0Spec> {
        Fsel4W::new(self, 12)
    }
    #[doc = "Bits 15:17 - Function Select 5"]
    #[inline(always)]
    #[must_use]
    pub fn fsel5(&mut self) -> Fsel5W<Gpfsel0Spec> {
        Fsel5W::new(self, 15)
    }
    #[doc = "Bits 18:20 - Function Select 6"]
    #[inline(always)]
    #[must_use]
    pub fn fsel6(&mut self) -> Fsel6W<Gpfsel0Spec> {
        Fsel6W::new(self, 18)
    }
    #[doc = "Bits 21:23 - Function Select 7"]
    #[inline(always)]
    #[must_use]
    pub fn fsel7(&mut self) -> Fsel7W<Gpfsel0Spec> {
        Fsel7W::new(self, 21)
    }
    #[doc = "Bits 24:26 - Function Select 8"]
    #[inline(always)]
    #[must_use]
    pub fn fsel8(&mut self) -> Fsel8W<Gpfsel0Spec> {
        Fsel8W::new(self, 24)
    }
    #[doc = "Bits 27:29 - Function Select 9"]
    #[inline(always)]
    #[must_use]
    pub fn fsel9(&mut self) -> Fsel9W<Gpfsel0Spec> {
        Fsel9W::new(self, 27)
    }
}
#[doc = "GPIO Function Select 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpfsel0::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpfsel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpfsel0Spec;
impl crate::RegisterSpec for Gpfsel0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpfsel0::R`](R) reader structure"]
impl crate::Readable for Gpfsel0Spec {}
#[doc = "`write(|w| ..)` method takes [`gpfsel0::W`](W) writer structure"]
impl crate::Writable for Gpfsel0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}

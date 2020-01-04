#[doc = "Reader of register SRK_REVOKE"]
pub type R = crate::R<u32, super::SRK_REVOKE>;
#[doc = "Writer for register SRK_REVOKE"]
pub type W = crate::W<u32, super::SRK_REVOKE>;
#[doc = "Register SRK_REVOKE `reset()`'s with value 0"]
impl crate::ResetValue for super::SRK_REVOKE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BITS`"]
pub type BITS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BITS`"]
pub struct BITS_W<'a> {
    w: &'a mut W,
}
impl<'a> BITS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - BITS"]
    #[inline(always)]
    pub fn bits_(&self) -> BITS_R {
        BITS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - BITS"]
    #[inline(always)]
    pub fn bits_(&mut self) -> BITS_W {
        BITS_W { w: self }
    }
}

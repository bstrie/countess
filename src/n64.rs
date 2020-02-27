#![allow(dead_code)]

#[allow(non_camel_case_types)]
#[repr(transparent)]
#[derive(Copy, Clone)]
struct n64(u64);

impl n64 {
	const MAX: Self = n64(99);
	const MIN: Self = n64(10);
	fn from_str_radix(src: &str, radix: u32) -> Result<Self, std::num::ParseIntError> { Ok(n64(u64::from_str_radix(src, radix)?)) }
	const fn count_ones(self) -> u32 { self.0.count_ones() }
	const fn count_zeros(self) -> u32 { self.0.count_zeros() }
	const fn leading_zeros(self) -> u32 { self.0.leading_zeros() }
	const fn trailing_zeros(self) -> u32 { self.0.trailing_zeros() }
	const fn rotate_left(self, n: u32) -> Self { n64(self.0.rotate_left(n)) }
	const fn rotate_right(self, n: u32) -> Self { n64(self.0.rotate_right(n)) }
	const fn swap_bytes(self) -> Self { n64(self.0.swap_bytes()) }
	const fn from_be(x: Self) -> Self { n64(u64::from_be(x.0)) }
	const fn from_le(x: Self) -> Self { n64(u64::from_le(x.0)) }
	const fn to_be(self) -> Self { n64(self.0.to_be()) }
	const fn to_le(self) -> Self { n64(self.0.to_be()) }
	fn checked_add(self, rhs: Self) -> Option<Self> { Some(n64(self.0.checked_add(rhs.0)?)) }
	fn checked_sub(self, rhs: Self) -> Option<Self> { Some(n64(self.0.checked_sub(rhs.0)?)) }
	fn checked_mul(self, rhs: Self) -> Option<Self> { Some(n64(self.0.checked_mul(rhs.0)?)) }
	fn checked_div(self, rhs: Self) -> Option<Self> { Some(n64(self.0.checked_div(rhs.0)?)) }
	fn checked_rem(self, rhs: Self) -> Option<Self> { Some(n64(self.0.checked_rem(rhs.0)?)) }
	fn checked_neg(self) -> Option<Self> { Some(n64(self.0.checked_neg()?)) }
	fn checked_shl(self, rhs: u32) -> Option<Self> { Some(n64(self.0.checked_shl(rhs)?)) }
	fn checked_shr(self, rhs: u32) -> Option<Self> { Some(n64(self.0.checked_shr(rhs)?)) }
	fn checked_pow(self, exp: u32) -> Option<Self> { Some(n64(self.0.checked_pow(exp)?)) }
	fn saturating_add(self, rhs: Self) -> Self { n64(self.0.saturating_add(rhs.0)) }
	fn saturating_sub(self, rhs: Self) -> Self { n64(self.0.saturating_sub(rhs.0)) }
	fn saturating_mul(self, rhs: Self) -> Self { n64(self.0.saturating_mul(rhs.0)) }
	fn saturating_pow(self, exp: u32) -> Self { n64(self.0.saturating_pow(exp)) }
	const fn wrapping_add(self, rhs: Self) -> Self { n64(self.0.wrapping_add(rhs.0)) }
	const fn wrapping_sub(self, rhs: Self) -> Self { n64(self.0.wrapping_sub(rhs.0)) }
	const fn wrapping_mul(self, rhs: Self) -> Self { n64(self.0.wrapping_mul(rhs.0)) }
	fn wrapping_div(self, rhs: Self) -> Self { n64(self.0.wrapping_div(rhs.0)) }
	fn wrapping_rem(self, rhs: Self) -> Self { n64(self.0.wrapping_rem(rhs.0)) }
	const fn wrapping_neg(self) -> Self { n64(self.0.wrapping_neg()) }
	const fn wrapping_shl(self, rhs: u32) -> Self { n64(self.0.wrapping_shl(rhs)) }
	const fn wrapping_shr(self, rhs: u32) -> Self { n64(self.0.wrapping_shr(rhs)) }
	fn wrapping_pow(self, exp: u32) -> Self { n64(self.0.wrapping_pow(exp)) }
	const fn overflowing_add(self, rhs: Self) -> (Self, bool) { let (n, b) = self.0.overflowing_add(rhs.0); (n64(n), b) }
	const fn overflowing_sub(self, rhs: Self) -> (Self, bool) { let (n, b) = self.0.overflowing_sub(rhs.0); (n64(n), b) }
	const fn overflowing_mul(self, rhs: Self) -> (Self, bool) { let (n, b) = self.0.overflowing_mul(rhs.0); (n64(n), b) }
	fn overflowing_div(self, rhs: Self) -> (Self, bool) { let (n, b) = self.0.overflowing_div(rhs.0); (n64(n), b) }
	fn overflowing_rem(self, rhs: Self) -> (Self, bool) { let (n, b) = self.0.overflowing_rem(rhs.0); (n64(n), b) }
	const fn overflowing_neg(self) -> (Self, bool) { let (n, b) = self.0.overflowing_neg(); (n64(n), b) }
	const fn overflowing_shl(self, rhs: u32) -> (Self, bool) { let (n, b) = self.0.overflowing_shl(rhs); (n64(n), b) }
	const fn overflowing_shr(self, rhs: u32) -> (Self, bool) { let (n, b) = self.0.overflowing_shr(rhs); (n64(n), b) }
	fn overflowing_pow(self, exp: u32) -> (Self, bool) { let (n, b) = self.0.overflowing_pow(exp); (n64(n), b) }
	fn pow(self, exp: u32) -> Self { n64(self.0.pow(exp)) }
	fn is_power_of_two(self) -> bool { self.0.is_power_of_two() }
	fn next_power_of_two(self) -> Self { n64(self.0.next_power_of_two()) }
	fn checked_next_power_of_two(self) -> Option<Self> { Some(n64(self.0.checked_next_power_of_two()?)) }
	fn to_be_bytes(self) -> [u8; std::mem::size_of::<Self>()] { self.0.to_be_bytes() }
	fn to_le_bytes(self) -> [u8; std::mem::size_of::<Self>()] { self.0.to_le_bytes() }
	fn to_ne_bytes(self) -> [u8; std::mem::size_of::<Self>()] { self.0.to_ne_bytes() }
	fn from_be_bytes(bytes: [u8; std::mem::size_of::<Self>()]) -> Self { n64(u64::from_be_bytes(bytes)) }
	fn from_le_bytes(bytes: [u8; std::mem::size_of::<Self>()]) -> Self { n64(u64::from_le_bytes(bytes)) }
	fn from_ne_bytes(bytes: [u8; std::mem::size_of::<Self>()]) -> Self { n64(u64::from_ne_bytes(bytes)) }
}

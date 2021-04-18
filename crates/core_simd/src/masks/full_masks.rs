//! Masks that take up full SIMD vector registers.

macro_rules! define_mask {
    {
        $(#[$attr:meta])*
        struct $name:ident<const $lanes:ident: usize>(
            crate::$type:ident<$lanes2:ident>
        );
    } => {
        $(#[$attr])*
        #[derive(Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
        #[repr(transparent)]
        pub struct $name<const $lanes: usize>(crate::$type<$lanes2>)
        where
            crate::$type<LANES>: crate::LanesAtMost32;

        impl_full_mask_reductions! { $name, $type }

        impl<const LANES: usize> Copy for $name<LANES>
        where
            crate::$type<LANES>: crate::LanesAtMost32,
        {}

        impl<const LANES: usize> Clone for $name<LANES>
        where
            crate::$type<LANES>: crate::LanesAtMost32,
        {
            #[inline]
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<const LANES: usize> $name<LANES>
        where
            crate::$type<LANES>: crate::LanesAtMost32,
        {
            pub fn splat(value: bool) -> Self {
                Self(<crate::$type<LANES>>::splat(
                    if value {
                        -1
                    } else {
                        0
                    }
                ))
            }

            #[inline]
            pub fn test(&self, lane: usize) -> bool {
                assert!(lane < LANES, "lane index out of range");
                self.0[lane] == -1
            }

            #[inline]
            pub fn set(&mut self, lane: usize, value: bool) {
                assert!(lane < LANES, "lane index out of range");
                self.0[lane] = if value {
                    -1
                } else {
                    0
                }
            }

            #[inline]
            pub fn to_int(self) -> crate::$type<LANES> {
                self.0
            }

            #[inline]
            pub unsafe fn from_int_unchecked(value: crate::$type<LANES>) -> Self {
                Self(value)
            }
        }

        impl<const LANES: usize> core::convert::From<$name<LANES>> for crate::$type<LANES>
        where
            crate::$type<LANES>: crate::LanesAtMost32,
        {
            fn from(value: $name<LANES>) -> Self {
                value.0
            }
        }

        impl<const LANES: usize> core::fmt::Debug for $name<LANES>
        where
            crate::$type<LANES>: crate::LanesAtMost32,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_list()
                    .entries((0..LANES).map(|lane| self.test(lane)))
                    .finish()
            }
        }

        impl<const LANES: usize> core::fmt::Binary for $name<LANES>
        where
            crate::$type<LANES>: crate::LanesAtMost32,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                core::fmt::Binary::fmt(&self.0, f)
            }
        }

        impl<const LANES: usize> core::fmt::Octal for $name<LANES>
        where
            crate::$type<LANES>: crate::LanesAtMost32,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                core::fmt::Octal::fmt(&self.0, f)
            }
        }

        impl<const LANES: usize> core::fmt::LowerHex for $name<LANES>
        where
            crate::$type<LANES>: crate::LanesAtMost32,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                core::fmt::LowerHex::fmt(&self.0, f)
            }
        }

        impl<const LANES: usize> core::fmt::UpperHex for $name<LANES>
        where
            crate::$type<LANES>: crate::LanesAtMost32,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                core::fmt::UpperHex::fmt(&self.0, f)
            }
        }

        impl<const LANES: usize> core::ops::BitAnd for $name<LANES>
        where
            crate::$type<LANES>: crate::LanesAtMost32,
        {
            type Output = Self;
            #[inline]
            fn bitand(self, rhs: Self) -> Self {
                Self(self.0 & rhs.0)
            }
        }

        impl<const LANES: usize> core::ops::BitOr for $name<LANES>
        where
            crate::$type<LANES>: crate::LanesAtMost32,
        {
            type Output = Self;
            #[inline]
            fn bitor(self, rhs: Self) -> Self {
                Self(self.0 | rhs.0)
            }
        }

        impl<const LANES: usize> core::ops::BitXor for $name<LANES>
        where
            crate::$type<LANES>: crate::LanesAtMost32,
        {
            type Output = Self;
            #[inline]
            fn bitxor(self, rhs: Self) -> Self::Output {
                Self(self.0 ^ rhs.0)
            }
        }

        impl<const LANES: usize> core::ops::Not for $name<LANES>
        where
            crate::$type<LANES>: crate::LanesAtMost32,
        {
            type Output = $name<LANES>;
            #[inline]
            fn not(self) -> Self::Output {
                Self(!self.0)
            }
        }

        impl<const LANES: usize> core::ops::BitAndAssign for $name<LANES>
        where
            crate::$type<LANES>: crate::LanesAtMost32,
        {
            #[inline]
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0;
            }
        }

        impl<const LANES: usize> core::ops::BitOrAssign for $name<LANES>
        where
            crate::$type<LANES>: crate::LanesAtMost32,
        {
            #[inline]
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0;
            }
        }

        impl<const LANES: usize> core::ops::BitXorAssign for $name<LANES>
        where
            crate::$type<LANES>: crate::LanesAtMost32,
        {
            #[inline]
            fn bitxor_assign(&mut self, rhs: Self) {
                self.0 ^= rhs.0;
            }
        }
    }
}

define_mask! {
    /// A mask equivalent to [SimdI8](crate::SimdI8), where all bits in the lane must be either set
    /// or unset.
    struct Mask8<const LANES: usize>(crate::SimdI8<LANES>);
}

define_mask! {
    /// A mask equivalent to [SimdI16](crate::SimdI16), where all bits in the lane must be either set
    /// or unset.
    struct Mask16<const LANES: usize>(crate::SimdI16<LANES>);
}

define_mask! {
    /// A mask equivalent to [SimdI32](crate::SimdI32), where all bits in the lane must be either set
    /// or unset.
    struct Mask32<const LANES: usize>(crate::SimdI32<LANES>);
}

define_mask! {
    /// A mask equivalent to [SimdI64](crate::SimdI64), where all bits in the lane must be either set
    /// or unset.
    struct Mask64<const LANES: usize>(crate::SimdI64<LANES>);
}

define_mask! {
    /// A mask equivalent to [SimdIsize](crate::SimdIsize), where all bits in the lane must be either set
    /// or unset.
    struct MaskSize<const LANES: usize>(crate::SimdIsize<LANES>);
}

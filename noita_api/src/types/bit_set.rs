use crate::TagManager;
use noita_api_macros::assert_size_with;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
#[repr(transparent)]
#[assert_size_with(0x20, u8)]
#[assert_size_with(0x40, u16)]
#[derive(Default)]
pub struct BitSet<T>(pub [T; 32]);
macro_rules! define_bitset {
    ($ty:ty) => {
        impl Debug for BitSet<$ty> {
            #[inline]
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.debug_map().entries(self.iter_tags_indices()).finish()
            }
        }
        impl BitSet<$ty> {
            #[must_use]
            #[inline]
            pub fn get(&self, n: $ty) -> bool {
                let out_index = n / <$ty>::BITS.strict_cast::<$ty>();
                let in_index = n % <$ty>::BITS.strict_cast::<$ty>();
                self[out_index.strict_cast::<usize>()] & (1 << in_index) != 0
            }
            #[inline]
            pub fn set(&mut self, n: $ty, value: bool) {
                let out_index = n / <$ty>::BITS.strict_cast::<$ty>();
                let in_index = n % <$ty>::BITS.strict_cast::<$ty>();
                if value {
                    self[out_index.strict_cast::<usize>()] |= 1 << in_index;
                } else {
                    self[out_index.strict_cast::<usize>()] &= !(1 << in_index);
                }
            }
            #[must_use]
            #[inline]
            pub fn len(&self) -> usize {
                let n: u32 = self.iter().map(|s| s.count_ones()).sum();
                n.strict_cast::<usize>()
            }
            #[must_use]
            #[inline]
            pub fn is_empty(&self) -> bool {
                self.iter().all(|a| *a == 0)
            }
            #[must_use]
            #[inline]
            pub fn has_tag(&self, tag: &str) -> bool {
                let tag_manager = TagManager::<$ty>::global();
                if let Some(n) = tag_manager.tag_indices.get(tag) {
                    self.get(*n)
                } else {
                    false
                }
            }
            #[inline]
            pub fn set_tag(&mut self, tag: &str, value: bool) {
                let tag_manager = TagManager::<$ty>::global();
                if let Some(n) = tag_manager.tag_indices.get(tag) {
                    self.set(*n, value)
                } else {
                    todo!()
                }
            }
            #[inline]
            pub fn iter_tags(&self) -> impl Iterator<Item = &str> {
                let tag_manager = TagManager::<$ty>::global().as_ref();
                self.iter_indices()
                    .map(|i| tag_manager.tags[i.strict_cast::<usize>()].as_str())
            }
            #[inline]
            pub fn iter_tags_indices(&self) -> impl Iterator<Item = ($ty, &str)> {
                let tag_manager = TagManager::<$ty>::global().as_ref();
                self.iter_indices()
                    .map(|i| (i, tag_manager.tags[i.strict_cast::<usize>()].as_str()))
            }
        }
    };
}
impl BitSet<u8> {
    #[inline]
    pub fn iter_indices(&self) -> impl Iterator<Item = u8> {
        (0..=255).filter(|i| self.get(*i))
    }
}
impl BitSet<u16> {
    #[inline]
    pub fn iter_indices(&self) -> impl Iterator<Item = u16> {
        (0..512).filter(|i| self.get(*i))
    }
}
define_bitset!(u8);
define_bitset!(u16);
impl<T> Deref for BitSet<T> {
    type Target = [T; 32];
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for BitSet<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

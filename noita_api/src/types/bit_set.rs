use crate::TagManager;
use noita_api_macros::assert_size_with;
use std::ops::{Deref, DerefMut};
#[repr(transparent)]
#[assert_size_with(0x20, u8)]
#[assert_size_with(0x40, u16)]
#[derive(Debug)]
pub struct BitSet<T>(pub [T; 32]);
macro_rules! define_bitset {
    ($ty:ty) => {
        impl BitSet<$ty> {
            #[must_use]
            #[inline]
            pub fn get(&self, n: $ty) -> bool {
                let out_index = n / <$ty>::try_from(<$ty>::BITS).unwrap();
                let in_index = n % <$ty>::try_from(<$ty>::BITS).unwrap();
                self[usize::from(out_index)] & (1 << in_index) != 0
            }
            #[inline]
            pub fn set(&mut self, n: $ty, value: bool) {
                let out_index = n / <$ty>::try_from(<$ty>::BITS).unwrap();
                let in_index = n % <$ty>::try_from(<$ty>::BITS).unwrap();
                if value {
                    self[usize::from(out_index)] |= 1 << in_index;
                } else {
                    self[usize::from(out_index)] &= !(1 << in_index);
                }
            }
            #[must_use]
            #[inline]
            pub fn count(&self) -> usize {
                let n: u32 = self.iter().map(|s| s.count_ones()).sum();
                usize::try_from(n).unwrap()
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
            pub fn get_tags(&self) -> impl Iterator<Item = &str> {
                let tag_manager = TagManager::<$ty>::global().as_ref();
                tag_manager
                    .tag_indices
                    .iter()
                    .filter_map(|(a, b)| if self.get(*b) { Some(a.as_str()) } else { None })
            }
        }
    };
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

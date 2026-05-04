use region::{Region, query_range};
use std::ptr::null;
use std::slice;
pub enum Token {
    Any,
    Byte(u8),
}
#[allow(clippy::needless_pass_by_value)]
pub fn search<const N: usize>(list: [Token; N]) -> *const u8 {
    for region in query_range(null::<u8>(), usize::MAX)
        .unwrap()
        .flatten()
        .filter(Region::is_readable)
    {
        let slice = unsafe { slice::from_raw_parts(region.as_ptr::<u8>(), region.len()) };
        if let Some(n) = slice.array_windows().position(|w: &[u8; N]| {
            list.iter()
                .zip(w.iter())
                .all(|(m, b)| if let Token::Byte(l) = m { l == b } else { true })
        }) {
            return unsafe { region.as_ptr::<u8>().add(n) };
        }
    }
    unreachable!()
}
pub fn get_function(mut ptr: *const u8) -> *const u8 {
    unsafe {
        ptr = ptr.sub(ptr.addr() % 0x10);
        loop {
            let n3 = ptr.sub(3).read();
            let n1 = ptr.sub(1).read();
            let p0 = ptr.read();
            let p1 = ptr.add(1).read();
            let p2 = ptr.add(2).read();
            if (n1 == 0xcc || n1 == 0xc3 || n3 == 0xc2)
                && (0x50..0x58).contains(&p0)
                && ((0x50..0x58).contains(&p1) || (p1 == 0x8b && p2 == 0xec))
            {
                return ptr;
            }
            ptr = ptr.sub(0x10);
        }
    }
}

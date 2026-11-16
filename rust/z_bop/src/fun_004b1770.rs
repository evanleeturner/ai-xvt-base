// SPDX-License-Identifier: MIT

unsafe extern "C" {
    fn malloc(size: u32) -> *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b1770(param_1: *mut i32) -> *mut core::ffi::c_void {
    // SAFETY: malloc is the game's allocator; param_1 points at a caller-owned
    // block of at least 0x13 dwords, and the new 0xd8-byte block is written
    // within its bounds, as the original does.
    unsafe {
        let p = malloc(0xd8) as *mut i32;
        let mut i = 0usize;
        while i < 0x36 {
            p.add(i).write_unaligned(0);
            i += 1;
        }
        p.write_unaligned(0);
        let mut j = 0usize;
        while j < 0x13 {
            let v = param_1.add(j).read_unaligned();
            p.add(3 + j).write_unaligned(v);
            j += 1;
        }
        let bytes_pp = (param_1.add(6).read_unaligned() as u32) >> 3;
        let size = bytes_pp
            .wrapping_mul(param_1.add(1).read_unaligned() as u32)
            .wrapping_mul(param_1.read_unaligned() as u32);
        let buf = malloc(size);
        p.add(0x17).write_unaligned(buf as usize as i32);
        let stride = bytes_pp.wrapping_mul(param_1.read_unaligned() as u32);
        p.add(6).write_unaligned(stride as i32);
        p as *mut core::ffi::c_void
    }
}

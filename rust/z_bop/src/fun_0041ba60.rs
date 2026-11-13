// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: i32;
    static mut DAT_0051c583: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004269e0(param_1: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0041ba60(param_1: u32) -> i32 {
    // SAFETY: the globals and callee are the game's own; the pointers read
    // are the game's table entries, read where the original reads them.
    unsafe {
        let i_var2 = ((param_1 & 0xffff) as i32).wrapping_mul(0x23);
        let i_var3 = DAT_009a1fe8.wrapping_add(i_var2);
        let pc = ((i_var3.wrapping_add(0x1f)) as usize as *const u32).read_unaligned() as usize
            as *const u8;
        if pc.is_null() {
            return 0;
        }
        let u_var4: u32;
        if pc.read() == 0 {
            let b = ((i_var3.wrapping_add(3)) as usize as *const u8).read() as u32;
            let arg = ((((i_var3 as u32) >> 8) << 8) | b) & 0xffff00ff;
            let r = FUN_004269e0(arg);
            let base = (&raw mut DAT_0051c583) as usize;
            let addr = base.wrapping_add(((r & 0xffff) as usize).wrapping_mul(0x101));
            u_var4 = (addr as *const u16).read_unaligned() as u32;
            let u_var5 = pc.byte_add(0x79).cast::<u16>().read_unaligned() as u32;
            if u_var4 < u_var5 {
                return u_var5.wrapping_shl(0xc) as i32;
            }
        } else {
            if pc.read() != 1 {
                return 0;
            }
            let i_var4 = pc.byte_add(0xa5).cast::<i32>().read_unaligned();
            if i_var4 != 0 {
                let s = ((i_var4.wrapping_add(8)) as usize as *const u16).read_unaligned();
                return (s as u32).wrapping_shl(0xc) as i32;
            }
            u_var4 = pc.byte_add(0x79).cast::<u16>().read_unaligned() as u32;
        }
        u_var4.wrapping_shl(0xc) as i32
    }
}

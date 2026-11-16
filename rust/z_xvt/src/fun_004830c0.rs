// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071b730: i32;
    static mut DAT_0071ecf4: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_00523b70: [u16; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004830c0() -> u32 {
    let mut au_stack: [u32; 8] = [0; 8];
    // SAFETY: the globals are the game's own memory; every pointer is
    // formed from the same addresses the original dereferences.
    unsafe {
        let base = (&raw mut DAT_0071b730) as *const u8;
        let mut i_var1: i32 = base
            .byte_add(DAT_0071ecf4.wrapping_mul(0x5bd) as u32 as usize)
            .cast::<i32>()
            .read_unaligned();
        if i_var1 == -1 {
            return 0xffff0000;
        }
        let p = i_var1
            .wrapping_mul(0x23)
            .wrapping_add(0x1f)
            .wrapping_add(DAT_006d4988) as u32 as usize as *const u8;
        i_var1 = p.byte_add(0xa7).cast::<i32>().read_unaligned();
        if i_var1 == 0 {
            return (DAT_006d4988 as u32) & 0xffff0000;
        }
        let buf = au_stack.as_mut_ptr() as *mut u8;
        let mut s_var3: i16 = 0;
        loop {
            let addr = (s_var3 as i32).wrapping_add(0x293).wrapping_add(i_var1) as u32 as usize
                as *const u8;
            let b = addr.read() as usize;
            buf.byte_add(b.wrapping_mul(2))
                .cast::<i16>()
                .write_unaligned(s_var3);
            s_var3 = s_var3.wrapping_add(1);
            if !(s_var3 < 10) {
                break;
            }
        }
        let mut b_var2: i32 = 1;
        s_var3 = 0;
        loop {
            let local_uint = buf
                .byte_add(((s_var3 as i32).wrapping_mul(2)) as u32 as usize)
                .cast::<u32>()
                .read_unaligned();
            let u_var4 = local_uint & 0xffff;
            let local_ushort =
                (i_var1.wrapping_add(0x1dd) as u32 as usize as *const u16).read_unaligned();
            let local_short = ((u_var4.wrapping_mul(2))
                .wrapping_add(0x29e)
                .wrapping_add(i_var1 as u32) as usize as *const i16)
                .read_unaligned();
            let tbl = (&raw const DAT_00523b70)
                .cast::<u16>()
                .add(u_var4 as usize)
                .read();
            if local_short == 0 && (local_ushort & tbl) != 0 {
                b_var2 = 0;
            }
            s_var3 = s_var3.wrapping_add(1);
            if !(s_var3 < 10) {
                break;
            }
        }
        (b_var2 == 0) as u32
    }
}

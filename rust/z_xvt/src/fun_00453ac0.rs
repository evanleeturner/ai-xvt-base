// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005f0928: u32;
    static mut DAT_005f0f00: u32;
    static mut DAT_005f09c4: u32;
    static mut DAT_005f0e24: u32;
    static mut DAT_005f09bc: u32;
    static mut DAT_005f0e28: u32;
    static mut DAT_005f09c0: u32;
    static mut DAT_005f0e2c: u32;
    static mut DAT_005f0e10: u32;
    static mut _DAT_005254e0: u32;
    static mut DAT_005f0920: u32;
    static mut DAT_005f0e32: u32;
    static mut DAT_005f0924: u32;
    static mut DAT_005f0e30: u32;
    static mut DAT_005f0e48: u32;
    static mut DAT_005f0e34: u32;
    static mut DAT_005f09d4: u32;
    static mut DAT_00703c6c: u32;
    static mut DAT_005f09d0: u32;
    static mut DAT_00703c70: u32;
    static mut DAT_005f09b8: u32;
    static mut DAT_00703c84: u32;
    static mut DAT_005f0e44: u32;
    static mut DAT_00525e08: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00453ac0() {
    // SAFETY: all globals are the game's own memory; the string copy reads
    // and writes the same bytes the original does.
    unsafe {
        let mut u_var2: u32 = 0xffff_ffff;
        let mut pc4: *const u8 = (&raw mut DAT_005f0928) as *const u8;
        let mut pc5: *const u8;
        loop {
            pc5 = pc4;
            if u_var2 == 0 {
                break;
            }
            u_var2 = u_var2.wrapping_sub(1);
            pc5 = pc4.wrapping_add(1);
            let c = pc4.read();
            pc4 = pc5;
            if c == 0 {
                break;
            }
        }
        u_var2 = !u_var2;
        pc4 = pc5.wrapping_sub(u_var2 as usize);
        let mut dst: *mut u8 = (&raw mut DAT_005f0f00) as *mut u8;
        let mut u_var3 = u_var2 >> 2;
        while u_var3 != 0 {
            dst.cast::<u32>()
                .write_unaligned(pc4.cast::<u32>().read_unaligned());
            pc4 = pc4.wrapping_add(4);
            dst = dst.wrapping_add(4);
            u_var3 = u_var3.wrapping_sub(1);
        }
        u_var2 &= 3;
        while u_var2 != 0 {
            dst.write(pc4.read());
            pc4 = pc4.wrapping_add(1);
            dst = dst.wrapping_add(1);
            u_var2 = u_var2.wrapping_sub(1);
        }
        DAT_005f09c4 = DAT_005f0e24;
        DAT_005f09bc = DAT_005f0e28;
        DAT_005f09c0 = DAT_005f0e2c;
        DAT_005f0e10 = _DAT_005254e0;
        DAT_005f0920 = DAT_005f0e32;
        DAT_005f0924 = DAT_005f0e30;
        DAT_005f0e48 = DAT_005f0e34;
        DAT_005f09d4 = (DAT_00703c6c as u16) as u32;
        DAT_005f09d0 = (DAT_00703c70 as u16) as u32;
        DAT_005f09b8 = (DAT_00703c84 as u16) as u32;
        DAT_005f0e44 = DAT_00525e08;
    }
}

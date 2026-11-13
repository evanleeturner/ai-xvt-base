// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005f261c: i32;
    static mut DAT_005f1d44: i32;
    static mut DAT_0052ae50: i32;
    static mut DAT_005f1d7c: i32;
    static mut DAT_005f1d80: i32;
    static mut DAT_005f1d38: i32;
    static mut DAT_005f2630: i32;
    static mut DAT_005f1d58: i32;
    static mut DAT_005f2614: i32;
    static mut DAT_005f1d5c: i32;
    static mut DAT_005f2624: i32;
    static mut DAT_005f2620: i32;
    static mut DAT_005f1d74: i32;
    static mut DAT_005f2638: i32;
    static mut DAT_005f2634: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004773f0() {
    // SAFETY: the globals are the game's own memory; the addresses computed
    // from them point at the game's buffers, read and written as the original does.
    unsafe {
        let i_var5: i32 = DAT_005f261c.wrapping_add(0x1000);
        let i_var3: i32 = DAT_005f1d44.wrapping_add(DAT_0052ae50);
        let i_var1: i32 = DAT_005f1d7c.wrapping_add(DAT_005f1d80);
        let mut p: u32 = i_var3.wrapping_add(DAT_005f1d80.wrapping_mul(2)) as u32;
        while p < i_var3.wrapping_add(i_var1.wrapping_mul(2)) as u32 {
            let u_var2: u32 = DAT_005f1d38.wrapping_add(DAT_005f2630) as u32;
            DAT_005f1d38 = (u_var2 & 0xff) as i32;
            let a: i32 = (DAT_005f1d58 >> 8)
                .wrapping_shl((DAT_005f2614 & 0x1f) as u32)
                .wrapping_add(DAT_005f1d5c >> 8)
                & DAT_005f2624;
            let byte_addr: i32 = a.wrapping_add(DAT_005f2620);
            let b: u32 = (byte_addr as u32 as *const u8).read() as u32;
            let idx: u32 = b.wrapping_add(((u_var2 >> 8) & 0xf).wrapping_mul(0x100));
            let src: u32 = (i_var5 as u32).wrapping_add(idx.wrapping_mul(2));
            let val: u16 = (src as *const u16).read_unaligned();
            (p as *mut u16).write_unaligned(val);
            DAT_005f2630 = DAT_005f2630.wrapping_add(DAT_005f1d74);
            DAT_005f1d5c = DAT_005f1d5c.wrapping_add(DAT_005f2638);
            DAT_005f1d58 = DAT_005f1d58.wrapping_add(DAT_005f2634);
            p = p.wrapping_add(2);
        }
    }
}

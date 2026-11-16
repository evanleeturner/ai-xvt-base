// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00527028: u32;
    static mut DAT_00527024: u32;
    static mut DAT_0052702c: u32;
    static mut DAT_0099f6e0: u32;
    static mut DAT_00999bec: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn timeGetTime() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0046ffc0() {
    // SAFETY: the globals are the game's own memory; the pointer walks the
    // game's record array starting at DAT_00999bec, as the original does.
    unsafe {
        let mut u_var2: u32 = 0;
        DAT_00527028 = DAT_00527024;
        DAT_0052702c = 0;
        if DAT_0099f6e0 != 0 {
            let mut p: *mut u32 = &raw mut DAT_00999bec;
            loop {
                u_var2 = u_var2.wrapping_add(1);
                p.write_unaligned(p.add(1).read_unaligned());
                p.add(2).write_unaligned(p.add(3).read_unaligned());
                p.add(4).write_unaligned(p.add(5).read_unaligned());
                let d_var1 = timeGetTime();
                p.add(0x88).write_unaligned(d_var1);
                p = p.add(0x8e);
                if u_var2 >= DAT_0099f6e0 {
                    break;
                }
            }
        }
    }
}

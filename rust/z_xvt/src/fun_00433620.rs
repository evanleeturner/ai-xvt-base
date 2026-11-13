// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005251b0: u32;
    static mut DAT_005251ac: u32;
    static mut DAT_005251b4: u32;
    static mut DAT_00739290: u32;
    static mut DAT_0073379c: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn timeGetTime() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00433620() {
    // SAFETY: the globals are the game's own memory; the pointer walks the
    // game's record array in 0x8e-dword strides exactly as the original does.
    unsafe {
        let mut u_var3: u32 = 0;
        DAT_005251b0 = DAT_005251ac;
        DAT_005251b4 = 0;
        if DAT_00739290 != 0 {
            let mut p: *mut u32 = &raw mut DAT_0073379c;
            loop {
                u_var3 = u_var3.wrapping_add(1);
                p.write_unaligned(p.add(1).read_unaligned());
                p.add(2).write_unaligned(p.add(3).read_unaligned());
                p.add(4).write_unaligned(p.add(5).read_unaligned());
                let d_var1 = timeGetTime();
                p.add(0x88).write_unaligned(d_var1);
                p = p.add(0x8e);
                if u_var3 >= DAT_00739290 {
                    break;
                }
            }
        }
    }
}

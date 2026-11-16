// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006cca80: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0045bcb0(param_1: u32) -> *mut i32;
    fn FUN_004b696b(param_1: *mut i32);
    fn FUN_004b88a0(param_1: i32, param_2: i32, param_3: i32) -> *mut i32;
    fn FUN_0045bbe0(param_1: *const i8, ...);
    fn FUN_004b63ea(
        param_1: i32,
        param_2: i32,
        param_3: *mut i32,
        param_4: *mut core::ffi::c_void,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0045c050(
    param_1: i32,
    param_2: i32,
    param_3: u32,
    param_4: *mut u8,
) -> *mut i32 {
    let _ = param_3;
    let mut param_2 = param_2;
    let mut param_4 = param_4;
    let unaff_ebp: *mut core::ffi::c_void = core::ptr::null_mut();
    let unaff_edi: i32 = 0;
    let piv_ar7: *mut i32 = core::ptr::null_mut();
    // Frame image: local_6c at index 0, puStack_5c at index 4,
    // iStack_10 at index 23, iStack_8 at index 25.
    let mut local_6c: [u32; 27] = [1; 27];

    // SAFETY: the callees and the global are the game's own; the buffer is
    // handed to the game's object as the original does, and the pointers
    // read and written are the game's surface and table memory.
    unsafe {
        let mut pi_var1 = FUN_0045bcb0(param_4 as usize as u32);
        let i_var2 = DAT_006cca80;
        if pi_var1.add(2).read() != 0 {
            FUN_004b696b(pi_var1);
            return pi_var1;
        }
        pi_var1 = FUN_004b88a0(param_1, param_2, DAT_006cca80);
        let base = local_6c.as_mut_ptr();
        for i in 0..27 {
            base.add(i).write_volatile(0);
        }
        base.write_volatile(0x6c);
        let obj = pi_var1.add(3).read() as usize as *const i32;
        let vtbl = obj.read();
        let f: unsafe extern "C" fn(*mut i32, i32, *mut u32, i32) -> i32 =
            core::mem::transmute(vtbl.wrapping_add(100) as usize as *const core::ffi::c_void);
        let i_var3 = f(obj as *mut i32, 0, base, 1);
        if i_var3 != 0 {
            FUN_0045bbe0(0x00527acc as usize as *const i8, i_var3);
        }
        while param_2 != 0 {
            param_2 = param_2.wrapping_sub(1);
            let mut i_var3 = (base.add(23).read_volatile()) as i32;
            let mut pu_var4 = base.add(4).read_volatile() as usize as *mut u16;
            let mut pb_var5 = param_4;
            if 0 < i_var3 {
                loop {
                    param_4 = pb_var5.add(1);
                    i_var3 = i_var3.wrapping_sub(1);
                    let tbl = base.add(25).read_volatile() as i32;
                    let idx = (pb_var5.read() as u32).wrapping_mul(2) as i32;
                    let v = (tbl.wrapping_add(idx) as usize as *const u16).read_unaligned();
                    pu_var4.write_unaligned(v);
                    pu_var4 = pu_var4.add(1);
                    pb_var5 = param_4;
                    if i_var3 == 0 {
                        break;
                    }
                }
            }
            let cur = base.add(4).read_volatile();
            base.add(4).write_volatile(cur.wrapping_add(i_var2 as u32));
        }
        let i_var2 = FUN_004b63ea(unaff_edi, 0, piv_ar7, unaff_ebp);
        if i_var2 != 0 {
            return piv_ar7;
        }
        FUN_0045bbe0(0x00527aa8 as usize as *const i8);
        core::ptr::null_mut()
    }
}

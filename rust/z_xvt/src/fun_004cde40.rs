// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005324ec: [i8; 11];
    static mut DAT_007806fa: [u8; 4];
    static mut DAT_007806fe: [u8; 4];

    static mut DAT_005202a0: u32;
    static mut DAT_005202a4: u32;
    static mut DAT_005202a8: u32;
    static mut DAT_005202ac: u32;
    static mut DAT_005202b0: u32;
    static mut DAT_005202b4: u32;
    static mut DAT_005202b8: u32;
    static mut DAT_005202bc: u32;
    static mut DAT_005202c0: u32;
    static mut DAT_005202c4: u32;
    static mut DAT_005202c8: u32;
    static mut DAT_005202cc: u32;
    static mut DAT_005202d0: u32;
    static mut DAT_005202d4: u32;
    static mut DAT_005202d8: u32;
    static mut DAT_005202dc: u32;
    static mut DAT_00521b90: u32;
    static mut DAT_00521b94: u32;
    static mut DAT_00521b98: u32;
    static mut DAT_00521b9c: u32;
    static mut DAT_00521ba0: u32;
    static mut DAT_00521ba4: u32;
    static mut DAT_00521ba8: u32;
    static mut DAT_00521bac: u32;
    static mut DAT_00521bb0: u32;
    static mut DAT_00521bb4: u32;
    static mut DAT_00521bb8: u32;
    static mut DAT_00521bbc: u32;
    static mut DAT_00520290: u32;
    static mut DAT_00520294: u32;
    static mut DAT_00520298: u32;
    static mut DAT_0052029c: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn Ordinal_4(a: i32, b: *mut core::ffi::c_void, c: i32, d: i32, e: i32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ce2a0(
        param_1: *mut i32,
        param_2: *mut u32,
        param_3: *mut i8,
        param_4: *mut i8,
    ) -> i32;
}

unsafe fn get_dpl() -> *mut i32 {
    // SAFETY: reads the game's packed global holding the interface pointer.
    unsafe { (&raw const DAT_007806fe).cast::<i32>().read_unaligned() as usize as *mut i32 }
}

unsafe fn clr_dpl() {
    // SAFETY: writes the game's packed global.
    unsafe { (&raw mut DAT_007806fe).cast::<i32>().write_unaligned(0) }
}

unsafe fn vslot(lp: *mut i32, off: usize) -> usize {
    // SAFETY: lp is a COM interface pointer; its first word is the vtable.
    unsafe {
        let vt = lp.read_unaligned() as u32 as usize;
        ((vt + off) as *const usize).read_unaligned()
    }
}

unsafe fn release(lp: *mut i32) {
    // SAFETY: calls Release (+8) on the COM object.
    unsafe {
        let f: unsafe extern "stdcall" fn(*mut i32) -> u32 =
            core::mem::transmute(vslot(lp, 8) as *const core::ffi::c_void);
        f(lp);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004cde40(
    param_1: u32,
    param_2: i32,
    param_3: u32,
    _param_4: u32,
    _param_5: u32,
    _param_6: u32,
    _param_7: u32,
    _param_8: u32,
    in_stack_00000024: u32,
    in_stack_00000028: *mut i8,
) -> u8 {
    let mut in_stack_00000028 = in_stack_00000028;
    let mut local_104: [u32; 5] = [0; 5];
    let mut local_dc: [u32; 10] = [0; 10];
    let mut ac_stack_b4: [i8; 16] = [0; 16];
    let mut au_stack_a4: [u32; 20] = [0; 20];
    let mut buf_w: [u32; 16] = [0; 16];
    let mut local_90: *mut u32 = local_104.as_mut_ptr();

    // SAFETY: the globals and callees are the game's own; all pointers
    // passed point at live locals or the game's globals.
    unsafe {
        core::ptr::copy_nonoverlapping(
            (&raw const DAT_005324ec).cast::<u8>(),
            buf_w.as_mut_ptr().cast::<u8>(),
            11,
        );

        match in_stack_00000024 {
            0 => {
                local_104[0] = DAT_005202a0;
                local_104[1] = DAT_005202a4;
                local_104[2] = DAT_005202a8;
                local_104[3] = DAT_005202ac;
                local_90 = local_104.as_mut_ptr();
            }
            1 => {
                local_104[0] = DAT_005202c0;
                local_104[1] = DAT_005202c4;
                local_104[2] = DAT_005202c8;
                local_104[3] = DAT_005202cc;
                au_stack_a4[1] = DAT_00521bb0;
                au_stack_a4[2] = DAT_00521bb4;
                au_stack_a4[3] = DAT_00521bb8;
                au_stack_a4[4] = DAT_00521bbc;
                local_90 = local_104.as_mut_ptr();
            }
            2 => {
                local_104[0] = DAT_005202b0;
                local_104[1] = DAT_005202b4;
                local_104[2] = DAT_005202b8;
                local_104[3] = DAT_005202bc;
                au_stack_a4[1] = DAT_00521ba0;
                au_stack_a4[2] = DAT_00521ba4;
                au_stack_a4[3] = DAT_00521ba8;
                au_stack_a4[4] = DAT_00521bac;
                local_90 = local_104.as_mut_ptr();
            }
            3 => {
                local_104[0] = DAT_005202d0;
                local_104[1] = DAT_005202d4;
                local_104[2] = DAT_005202d8;
                local_104[3] = DAT_005202dc;
                au_stack_a4[1] = DAT_00521b90;
                au_stack_a4[2] = DAT_00521b94;
                au_stack_a4[3] = DAT_00521b98;
                au_stack_a4[4] = DAT_00521b9c;
                in_stack_00000028 = local_dc.as_mut_ptr().cast::<i8>();
                local_dc[2] = 0;
                local_dc[3] = 0;
                local_dc[0] = 2;
                local_dc[1] = 0x2580;
                local_dc[4] = 1;
                local_90 = local_104.as_mut_ptr();
            }
            _ => {}
        }

        let u_var4: u32 = 0;
        let mut i_var1 = Ordinal_4(
            0,
            (&raw mut DAT_007806fe).cast::<core::ffi::c_void>(),
            0,
            0,
            0,
        );
        if i_var1 != 0 {
            return 0;
        }

        let mut lp = get_dpl();
        i_var1 = FUN_004ce2a0(lp, local_90, ac_stack_b4.as_mut_ptr(), in_stack_00000028);
        if i_var1 != 0 {
            lp = get_dpl();
            release(lp);
            clr_dpl();
            return 0;
        }

        au_stack_a4 = [0; 20];
        au_stack_a4[0] = 0x50;
        au_stack_a4[1] = 0;
        au_stack_a4[2] = DAT_00520290;
        au_stack_a4[3] = DAT_00520294;
        au_stack_a4[4] = DAT_00520298;
        au_stack_a4[5] = DAT_0052029c;
        au_stack_a4[6] = buf_w[12];
        au_stack_a4[7] = buf_w[13];
        au_stack_a4[8] = buf_w[14];
        au_stack_a4[9] = buf_w[15];
        au_stack_a4[10] = 0x10;
        au_stack_a4[11] = 0;
        au_stack_a4[12] = param_3;

        local_104[1] = 0x10;
        local_104[2] = 0;
        local_104[3] = param_1;

        local_dc = [0; 10];
        local_dc[0] = 0x28;
        local_dc[1] = 2;
        if param_2 == 0 {
            local_dc[1] = 1;
        }
        local_dc[2] = au_stack_a4.as_mut_ptr() as usize as u32;
        local_dc[3] = local_104.as_mut_ptr().add(1) as usize as u32;
        local_dc[9] = local_104[0];
        local_dc[4] = u_var4;

        lp = get_dpl();
        let set: unsafe extern "stdcall" fn(*mut i32, i32, i32, *mut u32) -> i32 =
            core::mem::transmute(vslot(lp, 0x30) as *const core::ffi::c_void);
        i_var1 = set(lp, 0, 0, local_dc.as_mut_ptr());
        if i_var1 == 0 {
            lp = get_dpl();
            let conn: unsafe extern "stdcall" fn(
                *mut i32,
                i32,
                *mut core::ffi::c_void,
                i32,
            ) -> i32 = core::mem::transmute(vslot(lp, 0xc) as *const core::ffi::c_void);
            i_var1 = conn(
                lp,
                0,
                (&raw mut DAT_007806fa).cast::<core::ffi::c_void>(),
                0,
            );
            lp = get_dpl();
            release(lp);
            clr_dpl();
            return (i_var1 == 0) as u8;
        }
        lp = get_dpl();
        release(lp);
        clr_dpl();
        0
    }
}

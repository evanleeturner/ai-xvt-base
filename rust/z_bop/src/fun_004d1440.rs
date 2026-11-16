// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005182a0: u32;
    static mut DAT_005182a4: u32;
    static mut DAT_005182a8: u32;
    static mut DAT_005182ac: u32;
    static mut DAT_005182b0: u32;
    static mut DAT_005182b4: u32;
    static mut DAT_005182b8: u32;
    static mut DAT_005182bc: u32;
    static mut DAT_005182c0: u32;
    static mut DAT_005182c4: u32;
    static mut DAT_005182c8: u32;
    static mut DAT_005182cc: u32;
    static mut DAT_005182d0: u32;
    static mut DAT_005182d4: u32;
    static mut DAT_005182d8: u32;
    static mut DAT_005182dc: u32;
    static mut DAT_00518f00: u32;
    static mut DAT_00518f04: u32;
    static mut DAT_00518f08: u32;
    static mut DAT_00518f0c: u32;
    static mut DAT_00518f10: u32;
    static mut DAT_00518f14: u32;
    static mut DAT_00518f18: u32;
    static mut DAT_00518f1c: u32;
    static mut DAT_00518f20: u32;
    static mut DAT_00518f24: u32;
    static mut DAT_00518f28: u32;
    static mut DAT_00518f2c: u32;
    static mut DAT_00518290: u32;
    static mut DAT_00518294: u32;
    static mut DAT_00518298: u32;
    static mut DAT_0051829c: u32;
    static mut DAT_00acebc9: [u8; 4];
    static mut DAT_00acebc5: [u8; 4];
    static mut s_modem_game_0052b9e4: [i8; 12];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn Ordinal_4(
        param_1: i32,
        param_2: *mut core::ffi::c_void,
        param_3: i32,
        param_4: i32,
        param_5: i32,
    ) -> i32;
    fn FUN_004d1840(
        param_1: *mut i32,
        param_2: *mut u32,
        param_3: *mut i8,
        param_4: *mut i8,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_variables, unused_assignments)]
pub unsafe extern "C" fn FUN_004d1440(
    param_1: u32,
    param_2: i32,
    param_3: u32,
    param_4: u32,
    param_5: u32,
    param_6: u32,
    param_7: u32,
    param_8: u32,
    in_stack_00000024: u32,
    in_stack_00000028: *mut i8,
) -> u8 {
    let mut in_stack_00000028 = in_stack_00000028;
    let mut a104: [u32; 4] = [0; 4];
    let mut adc: [u32; 10] = [0; 10];
    let mut ab4: [u8; 16] = [0; 16];
    let mut aa4: [u32; 20] = [0; 20];
    let mut a40: [u32; 16] = [0; 16];
    let mut zero_ptr: u32;

    // SAFETY: globals are the game's own memory; locals are accessed in bounds;
    // the COM-like object and vtable pointers come from the game's own state.
    unsafe {
        let s = &raw const s_modem_game_0052b9e4 as *const u8;
        a40[0] = s.cast::<u32>().read_unaligned();
        a40[1] = s.byte_add(4).cast::<u32>().read_unaligned();
        let w = s.byte_add(8).cast::<u16>().read_unaligned() as u32;
        let b = s.byte_add(10).read() as u32;
        a40[2] = w | (b << 16);
        // bytes 11..64 zero: byte 11 is top byte of a40[2]
        a40[2] &= 0x00ff_ffff;
        for i in 3..16 {
            a40[i] = 0;
        }

        match in_stack_00000024 {
            0 => {
                a104[0] = DAT_005182a0;
                a104[1] = DAT_005182a4;
                a104[2] = DAT_005182a8;
                a104[3] = DAT_005182ac;
                aa4[5] = a104.as_mut_ptr() as u32;
            }
            1 => {
                a104[0] = DAT_005182c0;
                a104[1] = DAT_005182c4;
                a104[2] = DAT_005182c8;
                a104[3] = DAT_005182cc;
                aa4[1] = DAT_00518f20;
                aa4[2] = DAT_00518f24;
                aa4[3] = DAT_00518f28;
                aa4[4] = DAT_00518f2c;
                aa4[5] = a104.as_mut_ptr() as u32;
            }
            2 => {
                a104[0] = DAT_005182b0;
                a104[1] = DAT_005182b4;
                a104[2] = DAT_005182b8;
                a104[3] = DAT_005182bc;
                aa4[1] = DAT_00518f10;
                aa4[2] = DAT_00518f14;
                aa4[3] = DAT_00518f18;
                aa4[4] = DAT_00518f1c;
                aa4[5] = a104.as_mut_ptr() as u32;
            }
            3 => {
                a104[0] = DAT_005182d0;
                a104[1] = DAT_005182d4;
                a104[2] = DAT_005182d8;
                a104[3] = DAT_005182dc;
                adc[0] = 2;
                adc[1] = 0x2580;
                aa4[1] = DAT_00518f00;
                adc[4] = 1;
                aa4[2] = DAT_00518f04;
                aa4[3] = DAT_00518f08;
                in_stack_00000028 = adc.as_mut_ptr() as *mut i8;
                aa4[4] = DAT_00518f0c;
                adc[2] = 0;
                adc[3] = 0;
                aa4[5] = a104.as_mut_ptr() as u32;
            }
            _ => {}
        }

        let u_var6: u32 = 0;
        let u_var5: u32 = 0;
        let u_var4: u32 = 0;
        zero_ptr = 0;
        let obj_slot = &raw mut DAT_00acebc9;
        let i_var1 = Ordinal_4(0, obj_slot.cast::<core::ffi::c_void>(), 0, 0, 0);
        if i_var1 != 0 {
            return 0;
        }
        let mut pi_var7: *mut i32 = obj_slot.cast::<*mut i32>().read_unaligned();
        let i_var1 = FUN_004d1840(
            pi_var7,
            aa4[5] as *mut u32,
            ab4.as_mut_ptr().cast::<i8>(),
            in_stack_00000028,
        );
        if i_var1 != 0 {
            pi_var7 = obj_slot.cast::<*mut i32>().read_unaligned();
            let vt = pi_var7.cast::<*mut i32>().read_unaligned();
            let f = core::mem::transmute::<i32, unsafe extern "stdcall" fn(*mut i32) -> i32>(
                vt.add(2).read(),
            );
            f(pi_var7);
            obj_slot.cast::<u32>().write_unaligned(zero_ptr);
            return 0;
        }

        for i in 0..20 {
            aa4[i] = 0;
        }
        aa4[0] = 0x50;
        aa4[1] = 0;
        aa4[2] = DAT_00518290;
        aa4[3] = DAT_00518294;
        aa4[4] = DAT_00518298;
        aa4[5] = DAT_0051829c;
        aa4[6] = a40[12];
        aa4[7] = a40[13];
        aa4[8] = a40[14];
        aa4[9] = a40[15];
        aa4[10] = 0x10;
        aa4[11] = 0;
        aa4[12] = param_3;
        aa4[13] = 0;
        aa4[14] = 0;
        aa4[15] = 0;
        aa4[16] = 0;
        aa4[17] = 0;
        aa4[18] = 0;
        aa4[19] = 0;
        a104[1] = 0x10;
        a104[2] = 0;
        a104[3] = param_1;
        for i in 0..10 {
            adc[i] = 0;
        }
        adc[0] = 0x28;
        adc[1] = 2;
        if param_2 == 0 {
            adc[1] = 1;
        }
        adc[2] = aa4.as_mut_ptr() as u32;
        adc[3] = (&raw mut a104[1]) as u32;
        adc[9] = a104[0];
        adc[4] = u_var4;
        adc[5] = u_var5;
        adc[6] = u_var6;

        pi_var7 = obj_slot.cast::<*mut i32>().read_unaligned();
        let vt = pi_var7.cast::<*mut i32>().read_unaligned();
        let f12 = core::mem::transmute::<
            i32,
            unsafe extern "stdcall" fn(*mut i32, i32, i32, *mut i8) -> i32,
        >(vt.add(12).read());
        let i_var1 = f12(pi_var7, 0, 0, adc.as_mut_ptr().cast::<i8>());
        if i_var1 == 0 {
            pi_var7 = obj_slot.cast::<*mut i32>().read_unaligned();
            let vt = pi_var7.cast::<*mut i32>().read_unaligned();
            let f3 = core::mem::transmute::<
                i32,
                unsafe extern "stdcall" fn(*mut i32, i32, *mut core::ffi::c_void, i32) -> i32,
            >(vt.add(3).read());
            let i_var1 = f3(
                pi_var7,
                0,
                (&raw mut DAT_00acebc5).cast::<core::ffi::c_void>(),
                0,
            );
            pi_var7 = obj_slot.cast::<*mut i32>().read_unaligned();
            let vt = pi_var7.cast::<*mut i32>().read_unaligned();
            let f2 = core::mem::transmute::<i32, unsafe extern "stdcall" fn(*mut i32) -> i32>(
                vt.add(2).read(),
            );
            f2(pi_var7);
            obj_slot.cast::<u32>().write_unaligned(zero_ptr);
            return (i_var1 == 0) as u8;
        }
        pi_var7 = obj_slot.cast::<*mut i32>().read_unaligned();
        let vt = pi_var7.cast::<*mut i32>().read_unaligned();
        let f2 = core::mem::transmute::<i32, unsafe extern "stdcall" fn(*mut i32) -> i32>(
            vt.add(2).read(),
        );
        f2(pi_var7);
        obj_slot.cast::<u32>().write_unaligned(zero_ptr);
        0
    }
}

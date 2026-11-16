// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005272b8: u32;
    static mut DAT_005272b4: u16;
    static mut DAT_005272a8: u16;
    static mut DAT_005272ac: u32;
    static mut DAT_0060f1c0: u32;
    static mut DAT_0060f1e8: u32;
    static mut _DAT_0060f208: u32;
    static mut _DAT_0060f1f0: u32;
    static mut DAT_0060f1d8: u32;
    static mut DAT_0060f204: u32;
    static mut DAT_0060f200: u32;
    static mut DAT_0060f1f4: u32;
    static mut DAT_0060f1cc: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ac6e0(a: u32) -> u32;
    fn FUN_004ac5f0(a: u32, b: i32) -> u32;
    fn FUN_0049eb60(a: i32) -> u32;
    fn FUN_004ac720(h: u32) -> *mut i32;
    fn FUN_00471ff0(p: *mut i32);
    fn FUN_004ac740();
    fn FUN_00474960(a: *mut i32, b: *mut i32, c: i32, d: i32, e: *mut u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00474610(param_1: u32) -> i32 {
    // SAFETY: all globals and callees are the game's own; pointers returned by
    // the game's allocator are read and written at the offsets the original uses.
    unsafe {
        let mut u_var1: u32 = DAT_005272b8;
        let mut u_var2: u32;
        let mut u_var8: u16;
        if (DAT_005272b8 as i32) < (param_1 as i32) && DAT_005272b4 != 0 {
            u_var1 = FUN_004ac6e0(DAT_005272b4 as u32);
            DAT_005272b4 = 0;
            DAT_005272b8 = 0;
        }
        u_var8 = (u_var1 >> 16) as u16;
        if DAT_005272b4 == 0 {
            u_var2 = FUN_004ac5f0(param_1, 0);
            DAT_005272b4 = u_var2 as u16;
            if DAT_005272b4 == 0 {
                u_var2 = FUN_0049eb60(0);
            }
            u_var8 = (u_var2 >> 16) as u16;
            DAT_005272b8 = param_1;
        }
        let pi3: *mut i32 = FUN_004ac720(((u_var8 as u32) << 16) | DAT_005272b4 as u32);
        let mut pi4: *mut i32 =
            FUN_004ac720((((pi3 as usize as u32) >> 16) << 16) | DAT_005272a8 as u32);
        let mut pi6: *mut i32 = pi3;
        let mut c = param_1 >> 2;
        while c != 0 {
            pi6.write_unaligned(pi4.read_unaligned());
            pi4 = pi4.add(1);
            pi6 = pi6.add(1);
            c -= 1;
        }
        c = param_1 & 3;
        while c != 0 {
            (pi6 as *mut i8).write((pi4 as *const i8).read());
            pi4 = pi4.byte_add(1);
            pi6 = pi6.byte_add(1);
            c -= 1;
        }
        if (pi3.read_unaligned() as u32) != (pi3 as usize as u32) {
            FUN_00471ff0(pi3);
        }
        u_var1 = param_1.wrapping_mul(2);
        FUN_004ac740();
        let mut u_var5: u32 = DAT_005272ac;
        if u_var1.wrapping_sub(DAT_005272ac) != 0
            && (DAT_005272ac as i32) <= (u_var1 as i32)
            && DAT_005272a8 != 0
        {
            u_var5 = FUN_004ac6e0(DAT_005272a8 as u32);
            DAT_005272a8 = 0;
            DAT_005272ac = 0;
        }
        u_var8 = (u_var5 >> 16) as u16;
        if DAT_005272a8 == 0 {
            u_var2 = FUN_004ac5f0(u_var1, 0);
            DAT_005272a8 = u_var2 as u16;
            if DAT_005272a8 == 0 {
                u_var2 = FUN_0049eb60(0);
            }
            u_var8 = (u_var2 >> 16) as u16;
            DAT_005272ac = u_var1;
        }
        pi6 = FUN_004ac720(((u_var8 as u32) << 16) | DAT_005272a8 as u32);
        pi6.write_unaligned(pi3.read_unaligned());
        pi6.add(1).write_unaligned(pi3.add(1).read_unaligned());
        pi6.add(2).write_unaligned(pi3.add(2).read_unaligned());
        pi6.add(3)
            .cast::<i16>()
            .write_unaligned(pi3.add(3).read_unaligned() as i16);
        pi6.write_unaligned(pi6 as usize as i32);
        let ptrval: i32 = (pi6 as usize as u32).wrapping_add(0xe) as i32;
        pi6.byte_add(10).cast::<i32>().write_unaligned(ptrval);
        let mut cnt: i32 = pi3.byte_add(6).cast::<i32>().read_unaligned();
        pi4 = ((pi6 as usize as u32)
            .wrapping_add(0xe)
            .wrapping_add((cnt as u32).wrapping_mul(4))) as usize as *mut i32;
        let mut local_d4: i32 = (cnt as u32).wrapping_mul(4).wrapping_add(0xe) as i32;
        let mut local_d0: [u32; 52] = [0; 52];
        DAT_0060f1c0 = 0;
        DAT_0060f1e8 = 0;
        _DAT_0060f208 = 0;
        _DAT_0060f1f0 = 0;
        DAT_0060f1d8 = 0;
        DAT_0060f204 = 0;
        pi6.byte_add(6).cast::<i32>().write_unaligned(0);
        let mut local_d8: i32 = 0;
        cnt = pi3.byte_add(6).cast::<i32>().read_unaligned();
        if 0 < cnt {
            let mut i_var9: i32 = 0;
            loop {
                i_var9 = i_var9.wrapping_add(4);
                let tmp6: i32 = pi6.byte_add(6).cast::<i32>().read_unaligned();
                let newcnt: i32 = tmp6.wrapping_add(1);
                pi6.byte_add(6).cast::<i32>().write_unaligned(newcnt);
                let tmp10: i32 = pi6.byte_add(10).cast::<i32>().read_unaligned();
                let pv: i32 = pi4 as usize as i32;
                ((tmp10 as u32).wrapping_sub(4).wrapping_add(i_var9 as u32) as usize as *mut i32)
                    .write_unaligned(pv);
                DAT_0060f200 = 0;
                DAT_0060f1f4 = 0;
                DAT_0060f1cc = 0;
                let tmp10b: i32 = pi3.byte_add(10).cast::<i32>().read_unaligned();
                let srcptr: i32 = ((tmp10b as u32).wrapping_sub(4).wrapping_add(i_var9 as u32)
                    as usize as *const i32)
                    .read_unaligned();
                let i_var7 = FUN_00474960(
                    pi4,
                    srcptr as u32 as usize as *mut i32,
                    pi3 as usize as i32,
                    pi6 as usize as i32,
                    local_d0.as_mut_ptr(),
                );
                local_d4 = local_d4.wrapping_add(i_var7);
                pi4 = ((pi4 as usize as u32).wrapping_add(i_var7 as u32)) as usize as *mut i32;
                local_d8 = local_d8.wrapping_add(1);
                cnt = pi3.byte_add(6).cast::<i32>().read_unaligned();
                if !(local_d8 < cnt) {
                    break;
                }
            }
        }
        local_d4
    }
}

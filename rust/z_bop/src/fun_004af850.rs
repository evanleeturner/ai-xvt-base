// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00521f19: [u8; 1];
    static mut DAT_009a7ed0: [i16; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ac720(param_1: u32) -> *mut core::ffi::c_void;
    fn FUN_00471ff0(param_1: *mut i32);
    fn FUN_004adcc0(param_1: i32) -> i32;
    fn FUN_004ac740();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004af850(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
) -> i32 {
    let mut param_5 = param_5;
    let mut param_6 = param_6;
    let mut local_800: [i32; 256] = [0; 256];
    let mut af: [f32; 256] = [0.0; 256];

    // SAFETY: the globals and callees are the game's own; the pointers come
    // from the game's structures and are read where the original reads them.
    unsafe {
        let flags = (&raw const DAT_00521f19)
            .cast::<u8>()
            .offset(param_1.wrapping_mul(0x1a) as isize)
            .read();
        if flags & 1 == 0 {
            return 0;
        }

        let hi = (((param_1.wrapping_mul(5) as u32) >> 16) as i16) as u32;
        let lo = ((&raw const DAT_009a7ed0)
            .cast::<i16>()
            .offset(param_1 as isize)
            .read()) as i32 as u32;
        let arg = (hi << 16) | lo;
        let pv4 = FUN_004ac720(arg) as *mut i32;

        if (pv4.read_unaligned() as usize as *mut i32) != pv4 {
            FUN_00471ff0(pv4);
        }

        let arr = pv4.byte_add(10).cast::<*mut i32>().read_unaligned();
        let first = arr.read_unaligned() as usize as *const i32;
        // arr points to an array of ints; **(int**) is the first element deref'd as pointer
        let _ = first;
        let first_ptr = arr.read_unaligned() as usize as *const i32;
        if first_ptr.byte_add(4).read_unaligned() == 0x14 {
            param_5 = param_5.wrapping_add(1);
        }

        let cnt = pv4.byte_add(6).cast::<i32>().read_unaligned();
        if cnt <= param_5 {
            param_5 = cnt.wrapping_add(-1);
        }

        let elem = arr.offset(param_5 as isize).read_unaligned();
        let r = FUN_004adcc0(elem);
        let mut pf8 = ((r as usize + 0x14) as *const *const f32).read_unaligned();
        let mut i5 = ((r as usize + 0x10) as *const i32).read_unaligned();

        if 2 < i5 {
            i5 = i5.wrapping_add(-2);
        }
        if 0x100 < i5 {
            i5 = 0x100;
        }
        if i5 <= param_6 {
            param_6 = i5.wrapping_add(-1);
        }

        let mut i6: i32 = 0;
        if 0 < i5 {
            let mut i9: i32 = 0;
            let mut f1: f32;
            loop {
                let x = pf8.read_unaligned();
                f1 = pf8.add(1).read_unaligned();
                let f2 = pf8.add(2).read_unaligned();
                let idx = (i9 / 4) as usize;
                af[idx] = (x - param_2 as f32) * (x - param_2 as f32);
                local_800[idx] = i6;
                pf8 = pf8.add(3);
                af[idx] = (f1 - param_3 as f32) * (f1 - param_3 as f32) + af[idx];
                f1 = (f2 - param_4 as f32) * (f2 - param_4 as f32) + af[idx];
                i6 = i6.wrapping_add(1);
                if i5 <= i6 {
                    break;
                }
                af[idx] = f1;
                i9 = i9.wrapping_add(4);
            }
            af[(i9 / 4) as usize] = f1;
        }

        let mut i7: i32 = 0;
        let mut i9: i32 = 0;
        let mut local_810: f32 = 4.611686e18_f32;
        let mut i6: i32 = local_800[0];
        let mut i10: i32 = i9;
        if 0 < param_6.wrapping_add(1) {
            loop {
                while i9 < i5 {
                    if af[i9 as usize] < local_810 {
                        local_810 = af[i9 as usize];
                        i6 = i9;
                    }
                    i9 = i9.wrapping_add(1);
                }
                let k = (i7 / 4) as usize;
                af[i6 as usize] = af[k];
                let i3 = local_800[i6 as usize];
                i9 = i10.wrapping_add(1);
                af[k] = local_810;
                local_800[i6 as usize] = local_800[k];
                local_800[k] = i3;
                i10 = i9;
                i7 = i7.wrapping_add(4);
                if !(i9 < param_6.wrapping_add(1)) {
                    break;
                }
            }
        }

        FUN_004ac740();
        local_800[param_6 as usize]
    }
}

// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009d8a54: i32;
    static mut DAT_009d8a58: i32;
    static mut DAT_009d8b60: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00427bd0(param_1: i16, param_2: i16, param_3: i32);
    fn FUN_00427d30(param_1: i16, param_2: i16, param_3: i32);
}

#[inline(always)]
fn mulshift(a: i32, s: i16) -> u32 {
    let l = (a as i64).wrapping_mul(s as i64);
    ((l as u32) >> 15) | ((((l as u64) >> 32) as i32).wrapping_shl(17) as u32)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004037b0(param_1: i32, param_2: i32, param_3: i32, param_4: i32) {
    // SAFETY: the game's own memory: param_1 points at an object whose field at
    // +0x1f holds a pointer to a record; reads and writes follow the original.
    unsafe {
        let p1 = param_1 as usize as *const u8;
        let local_base: i32 = p1.byte_add(0x1f).cast::<i32>().read_unaligned();
        let b = local_base as usize as *const u8;

        let c_92: u8 = b.byte_add(0x92).read();
        if c_92 != 0 {
            FUN_00427bd0(
                p1.byte_add(0x12).cast::<i16>().read_unaligned(),
                p1.byte_add(0x10).cast::<i16>().read_unaligned(),
                param_1,
            );
            FUN_00427d30(p1.byte_add(0x14).cast::<i16>().read_unaligned(), 0, param_1);
        }

        let s_99 = b.byte_add(0x99).cast::<i16>().read_unaligned();
        let s_9f = b.byte_add(0x9f).cast::<i16>().read_unaligned();
        let s_93 = b.byte_add(0x93).cast::<i16>().read_unaligned();
        DAT_009d8a58 = mulshift(param_3, s_9f)
            .wrapping_add(mulshift(param_2, s_99))
            .wrapping_add(mulshift(param_4, s_93)) as i32;

        let s_9b = b.byte_add(0x9b).cast::<i16>().read_unaligned();
        let s_a1 = b.byte_add(0xa1).cast::<i16>().read_unaligned();
        let s_95 = b.byte_add(0x95).cast::<i16>().read_unaligned();
        DAT_009d8a54 = mulshift(param_3, s_a1)
            .wrapping_add(mulshift(param_2, s_9b))
            .wrapping_add(mulshift(param_4, s_95)) as i32;

        let s_9d = b.byte_add(0x9d).cast::<i16>().read_unaligned();
        let s_a3 = b.byte_add(0xa3).cast::<i16>().read_unaligned();
        let s_97 = b.byte_add(0x97).cast::<i16>().read_unaligned();
        DAT_009d8b60 = mulshift(param_3, s_a3)
            .wrapping_add(mulshift(param_2, s_9d))
            .wrapping_add(mulshift(param_4, s_97)) as i32;
    }
}

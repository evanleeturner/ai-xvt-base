// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004aa910(
        a: *mut i32,
        b: *mut i32,
        c: *mut i32,
        d: *mut i32,
        e: *mut i32,
        f: *mut i32,
        g: *mut i32,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004ac8f0(
    param_1: i16,
    param_2: *mut i16,
    param_3: *mut i16,
    param_4: *mut i16,
    param_5: *mut i16,
) {
    let mut local_34: [i16; 2] = [0; 2];
    let mut local_30: [i16; 2] = [0; 2];
    let mut local_2c: i32 = 0;
    let mut local_28: i32 = 0;
    // The C passes local_24 + 6, which reaches past the declared six
    // elements, so the array is sized to hold what the callee writes there.
    let mut local_24: [i32; 9] = [0; 9];

    // SAFETY: the callee is the game's own; every pointer passed refers to
    // a live local, and the output pointers are the caller's.
    unsafe {
        let p = local_24.as_mut_ptr();
        FUN_004aa910(
            local_34.as_mut_ptr().cast::<i32>(),
            local_30.as_mut_ptr().cast::<i32>(),
            &raw mut local_2c,
            &raw mut local_28,
            p,
            p.add(3),
            p.add(6),
        );
        let idx = param_1 as isize;
        param_2.write_unaligned(p.offset(idx).read() as i16);
        param_3.write_unaligned(p.offset(idx + 3).read() as i16);
        param_4.write_unaligned(local_34[0]);
        param_5.write_unaligned(local_30[0]);
    }
}

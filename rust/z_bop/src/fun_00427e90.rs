// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a0049c: i32;
    static mut DAT_00a004a8: i32;
    static mut DAT_00a0048c: i32;
    static mut DAT_00a004cc: i32;
    static mut DAT_00a00494: i32;
    static mut DAT_00a00488: i32;
    static mut DAT_00a004a4: i32;
    static mut DAT_00a004b8: i32;
    static mut DAT_00a00480: i32;
    static mut DAT_00a004b0: i32;
    static mut DAT_009fe7f4: i32;
    static mut DAT_00a00490: i32;
    static mut DAT_00a07c68: i32;
    static mut DAT_00a07c6c: i32;
    static mut DAT_00a07cc4: i32;
    static mut DAT_00a004d0: i32;
    static mut DAT_00a004ac: i32;
    static mut DAT_00a004b4: i32;
    static mut DAT_009d77ac: i32;
    static mut DAT_009d77b8: i32;
    static mut DAT_009d77b4: i32;
    static mut DAT_009d77b0: i32;
    static mut DAT_009d7690: i32;
    static mut DAT_009d768c: i32;
    static mut DAT_009d7688: i32;
    static mut DAT_009d77a4: i32;
    static mut DAT_009d77a0: i32;
    static mut DAT_009d12ec: i32;
    static mut DAT_009d12f0: i32;
    static mut DAT_009d1304: i32;
    static mut DAT_00a081e0: i32;
    static mut DAT_00a081e4: i32;
    static mut DAT_00a08150: i32;
    static mut DAT_009ec476: i8;
}

#[inline(always)]
fn dot(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32 {
    let mut v = (a as u32)
        .wrapping_mul(b as u32)
        .wrapping_add((c as u32).wrapping_mul(d as u32))
        .wrapping_add((e as u32).wrapping_mul(f as u32)) as i32;
    if 0x3fffffff < v {
        v = 0x3fffffff;
    }
    if v < -0x3fffffff {
        v = -0x3fff0000;
    }
    v >> 0xf
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00427e90() {
    // SAFETY: all globals are the game's own memory, read and written
    // by value exactly as the original does.
    unsafe {
        DAT_009d77ac = dot(
            DAT_00a0049c,
            DAT_00a004a8,
            DAT_00a0048c,
            DAT_00a004cc,
            DAT_00a00494,
            DAT_00a00488,
        );
        DAT_009d77b8 = dot(
            DAT_00a0049c,
            DAT_00a004a4,
            DAT_00a0048c,
            DAT_00a004b8,
            DAT_00a00494,
            DAT_00a00480,
        );
        DAT_009d77b4 = dot(
            DAT_00a0049c,
            DAT_00a004b0,
            DAT_00a0048c,
            DAT_009fe7f4,
            DAT_00a00494,
            DAT_00a00490,
        );
        DAT_009d77b0 = dot(
            DAT_00a07c68,
            DAT_00a004a8,
            DAT_00a07c6c,
            DAT_00a004cc,
            DAT_00a07cc4,
            DAT_00a00488,
        );
        DAT_009d7690 = dot(
            DAT_00a07c68,
            DAT_00a004a4,
            DAT_00a07c6c,
            DAT_00a004b8,
            DAT_00a07cc4,
            DAT_00a00480,
        );
        DAT_009d768c = dot(
            DAT_00a07c68,
            DAT_00a004b0,
            DAT_00a07c6c,
            DAT_009fe7f4,
            DAT_00a07cc4,
            DAT_00a00490,
        );
        DAT_009d7688 = dot(
            DAT_00a004d0,
            DAT_00a004a8,
            DAT_00a004ac,
            DAT_00a004cc,
            DAT_00a004b4,
            DAT_00a00488,
        );
        DAT_009d77a4 = dot(
            DAT_00a004d0,
            DAT_00a004a4,
            DAT_00a004ac,
            DAT_00a004b8,
            DAT_00a004b4,
            DAT_00a00480,
        );
        DAT_009d77a0 = dot(
            DAT_00a004d0,
            DAT_00a004b0,
            DAT_00a004ac,
            DAT_009fe7f4,
            DAT_00a004b4,
            DAT_00a00490,
        );

        if DAT_009ec476 != 0 {
            DAT_00a081e0 = dot(
                DAT_00a0049c,
                DAT_009d12ec,
                DAT_00a0048c,
                DAT_009d12f0,
                DAT_00a00494,
                DAT_009d1304,
            );
            DAT_00a081e4 = dot(
                DAT_00a07c68,
                DAT_009d12ec,
                DAT_00a07c6c,
                DAT_009d12f0,
                DAT_00a07cc4,
                DAT_009d1304,
            );
            DAT_00a08150 = dot(
                DAT_00a004d0,
                DAT_009d12ec,
                DAT_00a004ac,
                DAT_009d12f0,
                DAT_00a004b4,
                DAT_009d1304,
            );
            return;
        }
        DAT_00a081e0 = DAT_009d12ec;
        DAT_00a081e4 = DAT_009d12f0;
        DAT_00a08150 = DAT_009d1304;
    }
}

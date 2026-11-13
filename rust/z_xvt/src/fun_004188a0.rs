// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_007091a4: u32;
    static mut DAT_00703c30: u16;
    static mut DAT_007329b8: u16;
    static mut DAT_00523c60: [u16; 512];
    static mut DAT_006db794: u32;
    static mut DAT_0071b70c: u32;
    static mut DAT_007325d0: u32;
}

unsafe fn step(val: u32, ang: u16) -> u32 {
    let mut sign: u8 = 0;
    let mut v = val;
    if (v as i32) < 0 {
        v = 0u32.wrapping_sub(v);
        sign = 0x80;
    }
    let idx = (((ang >> 5) as u32 & 0x3fe) >> 1) as usize;
    // SAFETY: the table is the game's own memory; the index is at most 0x1ff.
    let c = unsafe { (&raw const DAT_00523c60).cast::<u16>().add(idx).read() } as u32;
    let mut r = (v >> 16)
        .wrapping_mul(c)
        .wrapping_add((v & 0xffff).wrapping_mul(c).wrapping_add(0x8000) >> 16);
    if ((sign ^ (ang >> 8) as u8) & 0x80) != 0 {
        r = 0u32.wrapping_sub(r);
    }
    r
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004188a0() {
    // SAFETY: all globals are the game's own memory.
    unsafe {
        let u3 = step(DAT_007091a4, DAT_00703c30);
        DAT_006db794 = step(u3, DAT_007329b8.wrapping_add(0x4000));
        DAT_0071b70c = step(u3, DAT_007329b8);
        DAT_007325d0 = step(DAT_007091a4, DAT_00703c30.wrapping_add(0x4000));
    }
}

// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732948: *mut c_void;
    static mut DAT_0052a53c: u16;
    static mut DAT_0052a540: u32;
    static mut DAT_00523570: i8;
    static mut DAT_0052a324: i8;
    static mut DAT_00602678: i32;
    static mut DAT_006026a0: i32;
    static mut DAT_006026c0: i32;
    static mut DAT_006026a8: i32;
    static mut DAT_00602690: i32;
    static mut DAT_006026bc: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn fread(dst: *mut c_void, size: u32, count: u32, file: *mut c_void) -> u32;
    fn fwrite(src: *const c_void, size: u32, count: u32, file: *mut c_void) -> u32;
    fn fclose(file: *mut c_void) -> i32;
    fn FUN_0047ed40(param_1: *mut i8, param_2: *mut i8, param_3: i32, param_4: i32);
    fn FUN_0048f4b0(param_1: u32);
    fn FUN_0048f3c0(param_1: u32, param_2: i32) -> u16;
    fn FUN_0047f010(param_1: i32);
    fn FUN_0048f4f0(param_1: u32) -> *mut i32;
    fn FUN_00486000(param_1: *mut i32, param_2: i32);
    fn FUN_0048f510();
    fn FUN_00488830(param_1: u32) -> i32;
    fn FUN_0048ac10(param_1: *mut u32, param_2: *mut u32) -> i32;
}

unsafe fn reloc(buf: *mut i32) {
    // SAFETY: buf is the game's block returned by FUN_0048f4f0; fields are
    // read and written at the original's offsets.
    unsafe {
        let p5 = buf.read_unaligned();
        if p5 != buf as usize as i32 {
            let delta = (buf as usize as u32).wrapping_sub(p5 as u32) as i32;
            buf.write_unaligned((p5 as u32).wrapping_add(delta as u32) as i32);
            let f10 = buf.byte_add(10).read_unaligned();
            if f10 != 0 {
                buf.byte_add(10)
                    .write_unaligned((f10 as u32).wrapping_add(delta as u32) as i32);
                let mut f6 = buf.byte_add(6).read_unaligned();
                if 0 < f6 {
                    let mut i = 0i32;
                    let mut off = 0i32;
                    loop {
                        let f10 = buf.byte_add(10).read_unaligned();
                        let p = f10.wrapping_add(off) as usize as *mut i32;
                        let v = p.read_unaligned();
                        if v != 0 {
                            p.write_unaligned((v as u32).wrapping_add(delta as u32) as i32);
                            let f10 = buf.byte_add(10).read_unaligned();
                            let q = (f10.wrapping_add(off) as usize as *const i32).read_unaligned();
                            FUN_00486000(q as usize as *mut i32, delta);
                        }
                        off = off.wrapping_add(4);
                        i = i.wrapping_add(1);
                        f6 = buf.byte_add(6).read_unaligned();
                        if !(i < f6) {
                            break;
                        }
                    }
                }
            }
        }
    }
}

unsafe fn last_idx(s: *const i8) -> isize {
    // SAFETY: s is a NUL-terminated string.
    unsafe {
        let mut n: u32 = 0;
        while *s.add(n as usize) != 0 {
            n += 1;
        }
        n.wrapping_sub(1) as i32 as isize
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00488490(param_1: *mut i8) -> u16 {
    // SAFETY: the callees and globals are the game's own; param_1 is the
    // caller's NUL-terminated path string.
    unsafe {
        let mut local_d4: i32 = 0;
        let mut u_stack_d8: u32 = 0;
        let mut local_d0: [u32; 52] = [0; 52];

        FUN_0047ed40(param_1, &raw mut DAT_00523570, 1, 0);
        let mut p_file = DAT_00732948;
        if p_file.is_null() {
            return 0;
        }
        fread((&raw mut local_d4).cast(), 1, 4, p_file);
        let i_var7 = local_d4;
        if local_d4 < 1 {
            local_d4 = 0u32.wrapping_sub(local_d4 as u32) as i32;
            fread((&raw mut u_stack_d8).cast(), 1, 4, p_file);
        } else {
            local_d4 = 0;
            u_stack_d8 = i_var7 as u32;
        }
        if DAT_0052a540 < u_stack_d8 && DAT_0052a53c != 0 {
            FUN_0048f4b0(DAT_0052a53c as u32);
            DAT_0052a53c = 0;
            DAT_0052a540 = 0;
        }
        if DAT_0052a53c == 0 {
            DAT_0052a53c = FUN_0048f3c0(u_stack_d8, 0);
            if DAT_0052a53c == 0 {
                FUN_0047f010(0);
            }
            DAT_0052a540 = u_stack_d8;
        }
        let mut local_da = DAT_0052a53c;
        let mut dst = FUN_0048f4f0(((u_stack_d8 & 0xffff) << 16) | DAT_0052a53c as u32);
        fread(dst.cast(), 1, u_stack_d8, p_file);
        fclose(p_file);
        reloc(dst);

        if local_d4 == 0 {
            let idx = last_idx(param_1);
            let c1 = *param_1.offset(idx);
            let idx = last_idx(param_1);
            *param_1.offset(idx) = b'0' as i8;
            FUN_0047ed40(param_1, &raw mut DAT_0052a324, 0, 1);
            p_file = DAT_00732948;
            if !p_file.is_null() {
                fwrite((&raw const u_stack_d8).cast(), 1, 4, p_file);
                fwrite(dst.cast(), 1, u_stack_d8, p_file);
                fclose(p_file);
            }
            let idx = last_idx(param_1);
            *param_1.offset(idx) = c1;
            FUN_0048f510();
            u_stack_d8 = FUN_00488830(u_stack_d8) as u32;
            local_da = DAT_0052a53c;
            dst = FUN_0048f4f0(((u_stack_d8 & 0xffff) << 16) | DAT_0052a53c as u32);
            reloc(dst);
            local_d4 = -1;
            FUN_0047ed40(param_1, &raw mut DAT_0052a324, 0, 1);
            p_file = DAT_00732948;
            if !p_file.is_null() {
                fwrite((&raw const local_d4).cast(), 1, 4, p_file);
                fwrite((&raw const u_stack_d8).cast(), 1, 4, p_file);
                fwrite(dst.cast(), 1, u_stack_d8, p_file);
                fclose(p_file);
            }
            local_d4 = 0u32.wrapping_sub(local_d4 as u32) as i32;
        }
        local_d0 = [0; 52];
        let mut i_var9: i32 = 0;
        DAT_00602678 = 0;
        DAT_006026a0 = 0;
        DAT_006026c0 = 0;
        DAT_006026a8 = 0;
        let mut i_var7: i32 = 0;
        DAT_00602690 = 0;
        DAT_006026bc = 0;
        let mut f6 = dst.byte_add(6).read_unaligned();
        u_stack_d8 = (f6 as u32).wrapping_mul(4).wrapping_add(0xe);
        f6 = dst.byte_add(6).read_unaligned();
        if 0 < f6 {
            loop {
                i_var9 = i_var9.wrapping_add(4);
                i_var7 = i_var7.wrapping_add(1);
                let f10 = dst.byte_add(10).read_unaligned();
                let a = f10.wrapping_sub(4).wrapping_add(i_var9) as usize as *const i32;
                let r = FUN_0048ac10(
                    a.read_unaligned() as usize as *mut u32,
                    local_d0.as_mut_ptr(),
                );
                u_stack_d8 = u_stack_d8.wrapping_add(r as u32);
                f6 = dst.byte_add(6).read_unaligned();
                if !(i_var7 < f6) {
                    break;
                }
            }
        }
        FUN_0048f510();
        local_da
    }
}

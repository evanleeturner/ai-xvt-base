// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005320d0: [i8; 1];
    static mut DAT_005320f4: [i8; 1];
    static mut DAT_005320ec: [i8; 1];
    static mut DAT_00523570: [i8; 1];
    static mut DAT_0081b740: [i8; 1];
    static mut DAT_0081bcf0: [i8; 1];
    static mut DAT_0052a324: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dfc80(param_1: *mut i8) -> *mut *mut c_void;
    fn FUN_004be160(param_1: *mut i8, param_2: *mut i8) -> *mut c_void;
    fn FUN_004be320(param_1: *mut c_void, param_2: *mut c_void, param_3: i32) -> i32;
    fn FUN_004be1f0(param_1: *mut c_void) -> u32;
    fn __strcmpi(param_1: *mut i8, param_2: *mut i8) -> i32;
    fn FUN_004dfea0(param_1: *mut c_void);
    fn _sprintf(param_1: *mut i8, param_2: *mut i8, ...) -> i32;
    fn FUN_004be3e0(param_1: *mut c_void, param_2: *mut c_void, param_3: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004c04d0(param_1: i32) -> i32 {
    let mut local_20: [i8; 32] = [0; 32];
    let buf = local_20.as_mut_ptr();
    // SAFETY: the callees and globals are the game's own; all pointers come
    // from the game's lists and buffers, used where the original uses them.
    unsafe {
        'lab_05c0: {
            if param_1 != 0 {
                let src = (&raw mut DAT_005320f4).cast::<i32>();
                let d = buf.cast::<i32>();
                d.write_unaligned(src.read_unaligned());
                d.add(1).write_unaligned(src.add(1).read_unaligned());
                d.add(2).write_unaligned(src.add(2).read_unaligned());
                d.add(3).write_unaligned(src.add(3).read_unaligned());
                break 'lab_05c0;
            }
            let puVar2 = FUN_004dfc80((&raw mut DAT_005320d0).cast::<i8>());
            if puVar2.is_null() {
                break 'lab_05c0;
            }
            let mut puVar8 = (*puVar2) as *mut *mut c_void;
            let count = puVar2.add(1).read() as i32;
            let mut iVar5: i32 = 0;
            if 0 < count {
                loop {
                    let p3 =
                        FUN_004be160((*puVar8) as *mut i8, (&raw mut DAT_00523570).cast::<i8>());
                    if !p3.is_null() {
                        FUN_004be320(p3, (&raw mut DAT_0081b740).cast::<c_void>(), 0xe);
                        FUN_004be1f0(p3);
                        let r = __strcmpi(
                            (&raw mut DAT_0081b740).cast::<i8>(),
                            (&raw mut DAT_0081bcf0).cast::<i8>(),
                        );
                        if r == 0 {
                            let start = (*puVar8) as *const i8;
                            let mut len: usize = 0;
                            loop {
                                let c = *start.add(len);
                                len += 1;
                                if c == 0 {
                                    break;
                                }
                            }
                            let mut i = 0usize;
                            while i < len {
                                *buf.add(i) = *start.add(i);
                                i += 1;
                            }
                            break;
                        }
                    }
                    puVar8 = puVar8.add(1).read() as *mut *mut c_void;
                    iVar5 = iVar5.wrapping_add(1);
                    if !(iVar5 < puVar2.add(1).read() as i32) {
                        break;
                    }
                }
            }
            FUN_004dfea0(puVar2.cast::<c_void>());
        }
        if (&raw mut DAT_0081bcf0).cast::<u8>().read() == 0 {
            return 0;
        }
        if *buf == 0 {
            _sprintf(
                buf,
                (&raw mut DAT_005320ec).cast::<i8>(),
                (&raw mut DAT_0081bcf0).cast::<i8>(),
            );
        }
        let p3 = FUN_004be160(buf, (&raw mut DAT_0052a324).cast::<i8>());
        if !p3.is_null() {
            FUN_004be3e0(p3, (&raw mut DAT_0081bcf0).cast::<c_void>(), 0x3df3a);
            FUN_004be1f0(p3);
            return 1;
        }
        0
    }
}

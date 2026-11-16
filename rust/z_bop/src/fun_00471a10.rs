// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0060f1e0: *mut f32;
    static mut DAT_0060f1d0: *mut f32;
    static mut DAT_0060f1c4: *mut f32;
    static mut DAT_0099945c: i32;
    static mut DAT_009a1fe4: u16;
    static mut DAT_009d682c: u16;
    static mut DAT_009d7680: i32;
    static mut DAT_00a081ec: i32;
    static mut _DAT_005181c0: f32;
    static mut DAT_00999462: [u8; 4];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00471a10(
    param_1: i32,
    param_2: *mut u32,
    param_3: *mut f32,
    param_4: *mut f32,
) -> u32 {
    let mut param_3 = param_3;
    let mut pf10 = param_4;
    // SAFETY: all pointers are the caller's vertex/edge buffers and the
    // globals are the game's own memory; accessed as the original does.
    unsafe {
        if 0x80000000u32 < param_4.add(2).read().to_bits() {
            let pf11 = param_3.add(2);
            pf10 = param_3;
            param_3 = param_4;
            if 0x80000000u32 < pf11.read().to_bits() {
                return 0xffffffff;
            }
        }
        if 0x80000000u32 < param_3.add(2).read().to_bits() {
            DAT_0060f1e0 = DAT_0060f1d0;
            let a = (param_1.wrapping_add(0xbc) as usize as *const i32).read_unaligned();
            let b = (param_1.wrapping_add(200) as usize as *const i32).read_unaligned();
            DAT_0060f1c4 = (DAT_0099945c as u32)
                .wrapping_add((a.wrapping_add(b) as u32).wrapping_mul(0x18))
                as usize as *mut f32;
            DAT_0060f1d0 = DAT_0060f1c4;
            let p200 = param_1.wrapping_add(200) as usize as *mut i32;
            p200.write_unaligned(p200.read_unaligned().wrapping_add(1));
            let pc = &raw mut DAT_00999462 as *mut u8;
            let cnt = (pc as *const i32).read_unaligned().wrapping_add(1);
            (pc as *mut i32).write_unaligned(cnt);

            let f4 = _DAT_005181c0 / pf10.add(2).read();
            let i6 = ((DAT_009a1fe4 >> 1) as u32).wrapping_add(DAT_009d7680 as u32) as i32;
            let f1 = pf10.add(1).read();
            let f2 = param_3.add(1).read();
            let f5 = param_3.add(2).read()
                / ((param_3.add(2).read() - f4 * (DAT_00a081ec as f32)) + _DAT_005181c0);
            let f3 = param_3.add(1).read();
            let v0 = (((pf10.read() - (DAT_009d682c >> 1) as i32 as f32) * f4 - param_3.read())
                * f5
                + param_3.read())
                * (DAT_00a081ec as f32);
            DAT_0060f1d0.write(v0);
            let v1 = (DAT_00a081ec as f32) * (((f1 - i6 as f32) * f4 - f2) * f5 + f3);
            DAT_0060f1d0.add(1).write(v1);
            let w0 = ((DAT_009d682c >> 1) as i32 as f32) + DAT_0060f1d0.read();
            DAT_0060f1d0.write(w0);
            let w1 = (((DAT_009a1fe4 >> 1) as u32).wrapping_add(DAT_009d7680 as u32) as i32 as f32)
                + DAT_0060f1d0.add(1).read();
            DAT_0060f1d0.add(1).write(w1);
            DAT_0060f1d0.add(2).write(DAT_00a081ec as f32);
            let w3 = (pf10.add(3).read() - param_3.add(3).read()) * f5 + param_3.add(3).read();
            DAT_0060f1d0.add(3).write(w3);
            param_3 = DAT_0060f1d0;
        }
        let mut u9: u32;
        if param_3.add(1).read().to_bits() < 0x80000001 {
            let t = param_3.add(1).read();
            u9 = (t as i64 as i32) as u32;
            if (u9 as i32 as f32) != t {
                u9 = u9.wrapping_add(1);
            }
        } else {
            u9 = 0;
        }
        let mut u7: u32;
        if pf10.add(1).read().to_bits() < 0x80000001 {
            let t = pf10.add(1).read();
            u7 = (t as i64 as i32) as u32;
            if (u7 as i32 as f32) != t {
                u7 = u7.wrapping_add(1);
            }
        } else {
            u7 = 0;
        }
        if u7 != u9 {
            let mut u8 = u7;
            let mut pf11 = pf10;
            if (u7 as i32) < (u9 as i32) {
                u8 = u9;
                u9 = u7;
                pf11 = param_3;
                param_3 = pf10;
            }
            if 0 < (u8 as i32) {
                u7 = DAT_009a1fe4 as u32;
                if (u9 as i32) < (u7 as i32) {
                    if (u7 as i32) < (u8 as i32) {
                        u8 = u7;
                    }
                    let pf2 = param_2 as *mut f32;
                    param_2.write(u8);
                    let f3 = _DAT_005181c0 / (pf11.add(1).read() - param_3.add(1).read());
                    pf2.add(4).write((pf11.read() - param_3.read()) * f3);
                    let f1 = pf11.add(3).read();
                    let f2 = param_3.add(3).read();
                    param_2.add(6).write(0);
                    pf2.add(5).write((f1 - f2) * f3);
                    let f1 = param_3.add(1).read();
                    pf2.add(2)
                        .write(pf2.add(4).read() * ((u9 as i32 as f32) - f1) + param_3.read());
                    let f2 = param_3.add(3).read();
                    param_2.add(1).write(u9);
                    pf2.add(3)
                        .write(((u9 as i32 as f32) - f1) * pf2.add(5).read() + f2);
                    return u9;
                }
                return 0xffffffff;
            }
            return 0xffffffff;
        }
        0xffffffff
    }
}

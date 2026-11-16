/* SPDX-License-Identifier: MIT */
/* z_xvt 0044c2b0 */
typedef unsigned char  u8;
typedef unsigned short u16;
typedef unsigned int   u32;

extern u16  DAT_00732a3c;
extern int  DAT_006d4980;
extern u8  *DAT_006d4988;
extern u8   DAT_006d9d7a;
extern u16  DAT_006d9d50;
extern u32  DAT_006d499c;
extern u8  *DAT_00732e54;
/* DAT_006d9d7b / 7f / 83 are unaligned ints: declared as byte arrays, read via memcpy */
extern u8   DAT_006d9d7b[4];
extern u8   DAT_006d9d7f[4];
extern u8   DAT_006d9d83[4];

u16  __cdecl FUN_0049f010(u16 a, u16 b, u16 c);
u8  *__cdecl FUN_0042b150(u8 *a);
u32  __cdecl FUN_00429a20(u32 a);
u16  __cdecl FUN_00429c40(u32 a);
u16  __cdecl FUN_0044c560(u16 a, u32 b);
u32  __cdecl FUN_00412810(int a, int b, int c);
void __cdecl FUN_0042a1a0(u32 a, u32 b);
u32  __cdecl FUN_00444040(u32 a, u32 b);

u16 __cdecl FUN_0044c2b0(u16 param_1, u16 param_2, short param_3, u16 param_4, u16 param_5)
{
    u16 local_a;
    u32 local_4;
    u16 uVar8;
    u16 uVar9;
    u16 uVar3;
    u16 uVar10;
    u16 k;
    u8 *iVar4;
    u8 *iVar5;
    u8 *p;
    u8 *q;
    int bAny;
    short sVar7;
    u8 cVar1;
    int a, b, c;
    int d, e, f;

    local_a = 0xffff;
    local_4 = 0xffffffff;
    uVar8 = DAT_00732a3c;
    if ((int)(u32)DAT_00732a3c < DAT_006d4980) {
        do {
            sVar7 = (short)uVar8;
            p = DAT_006d4988 + (u32)uVar8 * 0x23;
            if (p[3] != 0) {
                __builtin_memcpy(&q, p + 0x1f, 4);
                __builtin_memcpy(&iVar4, q + 0xa7, 4);
                bAny = 0;
                k = 0;
                do {
                    cVar1 = iVar4[(u32)k + 0x1e6];
                    if (cVar1 != 0) {
                        bAny = 1;
                    }
                    k = (u16)(k + 1);
                } while (k < 10);
                if (bAny) {
                    uVar9 = FUN_0049f010(uVar8, param_1, param_2);
                    uVar3 = FUN_0049f010(uVar8, param_4, param_5);
                    if (param_3 == 1) {
                        uVar9 = (u16)(uVar9 | uVar3);
                    } else {
                        uVar9 = (u16)(uVar9 & uVar3);
                    }
                    if ((uVar9 != 0) && ((int)(u32)DAT_00732a3c < DAT_006d4980)) {
                        uVar10 = DAT_00732a3c;
                        do {
                            p = DAT_006d4988 + (u32)uVar10 * 0x23;
                            if (p[3] != 0) {
                                __builtin_memcpy(&q, p + 0x1f, 4);
                                __builtin_memcpy(&q, q + 0xa7, 4);
                                iVar5 = FUN_0042b150(q);
                                cVar1 = iVar5[0x34];
                                if ((((((cVar1 == 0xb) || (cVar1 == 0xc)) || (cVar1 == 0x17)) &&
                                      (*(short *)(iVar5 + 0x18) == sVar7)) ||
                                     (*(u16 *)(iVar4 + 0x6e) == uVar10)) &&
                                    (*(short *)(DAT_00732e54 + 0x342) != sVar7)) {
                                    if (FUN_00429a20((u32)uVar10) != 0) {
                                        if ((DAT_006d9d7a & 4) != 0) {
                                            if ((short)FUN_00429c40((u32)uVar10) == 0) goto LAB_0044c51d;
                                        }
                                        if ((DAT_006d9d7a & 1) != 0) {
                                            if ((short)FUN_0044c560(uVar10, 0xff) == 0) goto LAB_0044c51d;
                                        }
                                        if ((DAT_006d9d7a & 0x20) == 0) {
                                            FUN_0042a1a0((u32)DAT_006d9d50, (u32)uVar10);
                                        } else {
                                            p = DAT_006d4988 + (u32)uVar10 * 0x23;
                                            __builtin_memcpy(&a, p + 4, 4);
                                            __builtin_memcpy(&b, p + 8, 4);
                                            __builtin_memcpy(&c, p + 0xc, 4);
                                            __builtin_memcpy(&d, DAT_006d9d7b, 4);
                                            __builtin_memcpy(&e, DAT_006d9d7f, 4);
                                            __builtin_memcpy(&f, DAT_006d9d83, 4);
                                            DAT_006d499c = FUN_00412810(
                                                (int)((u32)a - (u32)d),
                                                (int)((u32)b - (u32)e),
                                                (int)((u32)c - (u32)f));
                                        }
                                        if ((DAT_006d9d7a & 0x10) == 0) {
                                            goto LAB_0044c507;
                                        } else if ((int)DAT_006d499c < 0x10001) {
                                            u8 *r;
                                            __builtin_memcpy(&r, DAT_006d4988 + (u32)DAT_006d9d50 * 0x23 + 0x1f, 4);
                                            if (FUN_00444040((u32)uVar10, (u32)r[0x87]) != 0) {
LAB_0044c507:
                                                if (DAT_006d499c < local_4) {
                                                    local_4 = DAT_006d499c;
                                                    local_a = uVar10;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
LAB_0044c51d:
                            uVar10 = (u16)(uVar10 + 1);
                        } while ((int)(u32)uVar10 < DAT_006d4980);
                    }
                }
            }
            uVar8 = (u16)(sVar7 + 1U);
        } while ((int)(u32)uVar8 < DAT_006d4980);
    }
    return local_a;
}

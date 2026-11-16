/* SPDX-License-Identifier: MIT */
/* z_xvt 0044d7a0 */
extern unsigned int   DAT_00732e54;
extern unsigned char  DAT_0052b0b4[];
extern unsigned char  DAT_0052b0b6[];
extern unsigned char  DAT_0052b0bc[];
extern unsigned char  DAT_0052b0be[];
extern unsigned char  DAT_0052b0c0[];
extern unsigned char  DAT_0052b0c3[];
extern int            DAT_00732e94;
extern int            DAT_00732e90;
extern int            DAT_00732e98;
extern int            DAT_006d9d64;
extern int            DAT_006d9d68;
extern int            DAT_006d9d6c;
extern unsigned short DAT_006d9d50;
extern int            DAT_006d4988;
extern int            DAT_006d4980;
extern unsigned int   DAT_006d499c;
extern unsigned char  DAT_006d9d56[4]; /* packed 4-byte pointer value at unaligned 0x6d9d56 */
extern int            DAT_006db798;
extern int            DAT_0070ab18;
extern int            DAT_0070ab14;
extern int            DAT_0070ac20;
extern unsigned short DAT_0071b704;
extern unsigned short DAT_0071b74a;
extern int            DAT_00732904;
extern unsigned short DAT_00732a3c;

unsigned short *FUN_0042b1e0(unsigned int a, unsigned int b);
/* two args in registers, four on the stack, caller cleans 16 bytes (callee ends RET 0x0) */
__attribute__((regparm(2), cdecl)) void FUN_0042a230(unsigned int a, unsigned int b, unsigned int c, short d, unsigned short e, short f);
unsigned int FUN_00412810(int a, int b, int c);
int FUN_0042b150(int a);
unsigned char FUN_00444040(unsigned int a, unsigned int b);
unsigned char FUN_00429a20(unsigned int a);
int FUN_004490f0(unsigned int a);
unsigned int FUN_00449790(unsigned int a, unsigned int b);
unsigned int FUN_00422ab0(unsigned int a);
unsigned int FUN_004479a0(unsigned int a, unsigned int b, unsigned int c, unsigned int d);
unsigned char FUN_00478470(void);

unsigned int __stdcall FUN_0044d7a0(void)
{
    int iVar1;
    unsigned short uVar2;
    unsigned char bVar4;
    int iVar5;
    unsigned short *puVar6;
    int iVar7;
    unsigned int uVar8;
    unsigned int uVar9;
    unsigned int uVar10;
    int iVar11;
    unsigned short uVar12;
    unsigned short uVar13;
    unsigned short uVar14;
    short sVar15;
    unsigned short local_42;
    unsigned short uStack_40;
    unsigned short local_3e;
    unsigned int local_38;
    unsigned char local_28[40];
    unsigned int local_4c;
    int tmp_a;
    int tmp_b;
    int tmp_c;
    unsigned short tmp_us;
    short tmp_s1;
    short tmp_s2;
    int cond;

    if (*(char *)(DAT_00732e54 + 7) == '\x03') {
        return DAT_00732e54 & 0xffff0000;
    }
    __builtin_memcpy(&tmp_us, (void *)(DAT_00732e54 + 0x1df), 2);
    if (tmp_us != 0) {
        __builtin_memcpy(&tmp_us, (void *)(DAT_00732e54 + 0x1e1), 2);
        if (tmp_us == 0) {
            local_42 = 0;
            uVar10 = DAT_00732e54;
            if (*(char *)(DAT_00732e54 + 0x228) != '\0') {
                do {
                    iVar5 = (unsigned int)local_42 + (unsigned int)*(unsigned char *)(DAT_00732e54 + 4) * 0xef;
                    uStack_40 = (unsigned short)DAT_0052b0b4[iVar5];
                    if (uStack_40 < (unsigned short)(DAT_0052b0b6[iVar5] + 1)) {
                        do {
                            uVar10 = (unsigned int)uStack_40;
                            if (*(char *)(DAT_0052b0c3[uVar10 * 8 + (unsigned int)*(unsigned char *)(DAT_00732e54 + 4) * 0xef] + 0x2ca + DAT_00732e54) == '\0') {
                                iVar5 = DAT_00732e54 + uVar10 * 4;
                                if (((*(signed char *)(iVar5 + 0x344) == -0x70) || (*(signed char *)(iVar5 + 0x344) == -0x6b)) &&
                                    (*(char *)(iVar5 + 0x346) != '\0')) {
                                    if (*(char *)(iVar5 + 0x347) == '\0') {
                                        puVar6 = FUN_0042b1e0(DAT_00732e54, uVar10);
                                        *puVar6 = 0xffff;
                                        DAT_00732e94 = DAT_006d9d64;
                                        DAT_00732e90 = DAT_006d9d68;
                                        DAT_00732e98 = DAT_006d9d6c;
                                        iVar7 = (unsigned int)*(unsigned char *)(DAT_00732e54 + 4) * 0xef;
                                        iVar5 = iVar7 + uVar10 * 8;
                                        __builtin_memcpy(&tmp_us, &DAT_0052b0be[iVar5], 2);
                                        __builtin_memcpy(&tmp_s1, &DAT_0052b0bc[iVar5], 2);
                                        __builtin_memcpy(&tmp_s2, &DAT_0052b0c0[uVar10 * 8 + iVar7], 2);
                                        FUN_0042a230((unsigned int)DAT_006d9d50, (unsigned int)tmp_us,
                                                     (unsigned int)DAT_006d9d50 * 0x23 + DAT_006d4988,
                                                     tmp_s1, tmp_us, tmp_s2);
                                        if (*(char *)((unsigned int)DAT_006d9d50 * 0x23 + 3 + DAT_006d4988) == '5') {
                                            DAT_0070ab18 = (int)((unsigned int)DAT_0070ab18 * 2);
                                            DAT_0070ab14 = (int)((unsigned int)DAT_0070ab14 * 2);
                                            DAT_0070ac20 = (int)((unsigned int)DAT_0070ac20 * 2);
                                        }
                                        DAT_00732e94 = (int)((unsigned int)DAT_00732e94 + (unsigned int)DAT_0070ab18);
                                        DAT_00732e90 = (int)((unsigned int)DAT_00732e90 + (unsigned int)DAT_0070ab14);
                                        DAT_00732e98 = (int)((unsigned int)DAT_00732e98 + (unsigned int)DAT_0070ac20);
                                        local_38 = 0x40000;
                                        local_3e = 0xffff;
                                        uVar14 = DAT_0071b704;
                                        if ((int)(unsigned int)DAT_0071b704 < DAT_00732904) {
                                            do {
                                                iVar11 = (unsigned int)uVar14 * 0x23 + DAT_006d4988;
                                                if (*(char *)(iVar11 + 3) != '\0') {
                                                    iVar1 = DAT_006db798 + (unsigned int)(unsigned short)(uVar14 - DAT_0071b704) * 8;
                                                    if ((*(char *)(iVar1 + 1) != '\0') &&
                                                        (*(unsigned short *)(iVar1 + 4) == DAT_006d9d50)) {
                                                        sVar15 = 0;
                                                        for (uVar12 = DAT_0071b704; (int)(unsigned int)uVar12 < DAT_00732904;
                                                             uVar12 = uVar12 + 1) {
                                                            if ((*(char *)((unsigned int)uVar12 * 0x23 + 3 + DAT_006d4988) != '\0') &&
                                                                (uVar14 != uVar12)) {
                                                                iVar5 = DAT_006db798 + (unsigned int)(unsigned short)(uVar12 - DAT_0071b704) * 8;
                                                                if ((4 < *(unsigned char *)(iVar5 + 1)) &&
                                                                    (*(unsigned short *)(iVar5 + 4) == uVar14)) {
                                                                    sVar15 = sVar15 + 1;
                                                                }
                                                            }
                                                        }
                                                        if (sVar15 == 0) {
                                                            __builtin_memcpy(&tmp_a, (void *)(iVar11 + 4), 4);
                                                            __builtin_memcpy(&tmp_b, (void *)(iVar11 + 8), 4);
                                                            __builtin_memcpy(&tmp_c, (void *)(iVar11 + 0xc), 4);
                                                            DAT_006d499c = FUN_00412810((int)((unsigned int)tmp_a - (unsigned int)DAT_00732e94),
                                                                                        (int)((unsigned int)tmp_b - (unsigned int)DAT_00732e90),
                                                                                        (int)((unsigned int)tmp_c - (unsigned int)DAT_00732e98));
                                                            if ((0x4000 < (int)DAT_006d499c) && (DAT_006d499c < local_38)) {
                                                                local_3e = uVar14;
                                                                local_38 = DAT_006d499c;
                                                            }
                                                        }
                                                    }
                                                }
                                                uVar14 = uVar14 + 1;
                                            } while ((int)(unsigned int)uVar14 < DAT_00732904);
                                        }
                                        if (local_3e == 0xffff) {
                                            uVar9 = (unsigned int)DAT_00732a3c;
                                            local_38 = 0x40000;
                                            local_3e = 0xffff;
                                            if ((int)(unsigned int)DAT_00732a3c < DAT_006d4980) {
                                                do {
                                                    iVar5 = (uVar9 & 0xffff) * 0x23;
                                                    uVar14 = (unsigned short)uVar9;
                                                    if (*(char *)(DAT_006d4988 + iVar5 + 3) != '\0') {
                                                        __builtin_memcpy(&tmp_a, (void *)(DAT_006d4988 + iVar5 + 0x1f), 4);
                                                        __builtin_memcpy(&iVar7, (void *)(tmp_a + 0xa7), 4);
                                                        iVar11 = FUN_0042b150(iVar7);
                                                        cond = 0;
                                                        if ((*(unsigned short *)(iVar11 + 0x18) == DAT_006d9d50) &&
                                                            ((*(char *)(iVar11 + 0x34) == '\f') ||
                                                             (*(char *)(iVar11 + 0x34) == '\x17'))) {
                                                            cond = 1;
                                                        } else if (*(unsigned short *)(iVar7 + 0x6a) == uVar14) {
                                                            cond = 1;
                                                        } else if (*(unsigned short *)(iVar7 + 0x6e) == uVar14) {
                                                            cond = 1;
                                                        } else {
                                                            __builtin_memcpy(&tmp_b, (void *)(DAT_006d4988 + 0x1b + iVar5), 4);
                                                            if (tmp_b != -1) {
                                                                __builtin_memcpy(&tmp_c, (void *)((unsigned int)DAT_006d9d50 * 0x23 + 0x1f + DAT_006d4988), 4);
                                                                if (FUN_00444040(uVar9, (unsigned int)*(unsigned char *)(tmp_c + 0x87)) == 1) {
                                                                    __builtin_memcpy(&tmp_b, (void *)(DAT_006d4988 + 0x1b + iVar5), 4);
                                                                    __builtin_memcpy(&tmp_us, (char *)&DAT_0071b74a + (unsigned int)tmp_b * 0x5bd, 2);
                                                                    if (tmp_us == DAT_006d9d50) {
                                                                        cond = 1;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        if (cond != 0) {
                                                            if (FUN_00429a20(uVar9 & 0xffff) != 0) {
                                                                uVar13 = 0;
                                                                for (uVar12 = DAT_0071b704; (int)(unsigned int)uVar12 < DAT_00732904;
                                                                     uVar12 = uVar12 + 1) {
                                                                    if ((*(char *)((unsigned int)uVar12 * 0x23 + 3 + DAT_006d4988) != '\0') &&
                                                                        (uVar14 != uVar12)) {
                                                                        iVar7 = DAT_006db798 + (unsigned int)(unsigned short)(uVar12 - DAT_0071b704) * 8;
                                                                        if ((*(char *)(iVar7 + 1) != '\0') &&
                                                                            (*(unsigned short *)(iVar7 + 4) == uVar14)) {
                                                                            uVar13 = uVar13 + 1;
                                                                        }
                                                                    }
                                                                }
                                                                if (uVar13 < 2) {
                                                                    iVar5 = DAT_006d4988 + iVar5;
                                                                    __builtin_memcpy(&tmp_a, (void *)(iVar5 + 4), 4);
                                                                    __builtin_memcpy(&tmp_b, (void *)(iVar5 + 8), 4);
                                                                    __builtin_memcpy(&tmp_c, (void *)(iVar5 + 0xc), 4);
                                                                    DAT_006d499c = FUN_00412810((int)((unsigned int)tmp_a - (unsigned int)DAT_00732e94),
                                                                                                (int)((unsigned int)tmp_b - (unsigned int)DAT_00732e90),
                                                                                                (int)((unsigned int)tmp_c - (unsigned int)DAT_00732e98));
                                                                    if (DAT_006d499c < local_38) {
                                                                        local_3e = uVar14;
                                                                        local_38 = DAT_006d499c;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    uVar9 = (unsigned int)(unsigned short)(uVar14 + 1);
                                                } while ((int)(unsigned int)(unsigned short)(uVar14 + 1) < DAT_006d4980);
                                            }
                                        } else {
                                            *puVar6 = local_3e;
                                        }
                                        if (local_3e != 0xffff) {
                                            *puVar6 = local_3e;
                                        }
                                        if (*puVar6 != 0xffff) {
                                            uVar14 = 0;
                                            __builtin_memcpy(&local_4c, DAT_006d9d56, 4);
                                            uVar2 = *(unsigned short *)(local_4c + 0x18);
                                            local_28[0] = 0;
                                            __builtin_memcpy(&local_4c, DAT_006d9d56, 4);
                                            *(unsigned short *)(local_4c + 0x18) = *puVar6;
                                            __builtin_memcpy(&local_4c, DAT_006d9d56, 4);
                                            if ((int)(unsigned int)*(unsigned short *)(local_4c + 0x18) < DAT_006d4980) {
                                                uVar12 = 0;
                                                __builtin_memcpy(&local_4c, DAT_006d9d56, 4);
                                                iVar5 = (unsigned int)*(unsigned short *)(local_4c + 0x18) * 0x23;
                                                iVar7 = FUN_004490f0((unsigned int)*(unsigned char *)(DAT_006d4988 + 3 + iVar5));
                                                if (0 < iVar7) {
                                                    do {
                                                        uVar8 = FUN_00449790((unsigned int)*(unsigned char *)(DAT_006d4988 + 3 + iVar5),
                                                                             (unsigned int)uVar12);
                                                        if (((short)uVar8 == 1) || ((short)uVar8 == 3)) {
                                                            local_28[uVar14] = (unsigned char)uVar12;
                                                            uVar14 = uVar14 + 1;
                                                        }
                                                        uVar12 = uVar12 + 1;
                                                    } while ((int)(unsigned int)uVar12 < iVar7);
                                                }
                                                uVar14 = (unsigned short)FUN_00422ab0((unsigned int)uVar14);
                                                __builtin_memcpy(&local_4c, DAT_006d9d56, 4);
                                                *(unsigned short *)(local_4c + 0x1c) = (unsigned short)local_28[uVar14];
                                            } else {
                                                __builtin_memcpy(&local_4c, DAT_006d9d56, 4);
                                                *(unsigned short *)(local_4c + 0x1c) = 0;
                                            }
                                            uVar9 = FUN_004479a0((unsigned int)DAT_006d9d50, uVar10,
                                                                 (unsigned int)*(unsigned char *)(DAT_00732e54 + 0x344 + uVar10 * 4), 0xffff);
                                            if (uVar9 != 0xffffffff) {
                                                bVar4 = FUN_00478470();
                                                *(unsigned char *)(DAT_006db798 + 1 + uVar9 * 8) = (unsigned char)((bVar4 & 3) + 3);
                                                *(unsigned char *)(DAT_00732e54 + 0x347 + uVar10 * 4) = 0x14;
                                            }
                                            __builtin_memcpy(&local_4c, DAT_006d9d56, 4);
                                            *(unsigned short *)(local_4c + 0x18) = uVar2;
                                        }
                                    } else {
                                        *(char *)(iVar5 + 0x347) = *(char *)(iVar5 + 0x347) + -1;
                                    }
                                }
                            }
                            uStack_40 = uStack_40 + 1;
                        } while (uStack_40 <
                                 (unsigned short)(DAT_0052b0b6[(unsigned int)local_42 + (unsigned int)*(unsigned char *)(DAT_00732e54 + 4) * 0xef] + 1));
                    }
                    local_42 = local_42 + 1;
                    uVar10 = (unsigned int)local_42;
                } while (uVar10 < *(unsigned char *)(DAT_00732e54 + 0x228));
            }
            return uVar10 & 0xffff0000;
        }
        return DAT_00732e54 & 0xffff0000;
    }
    return DAT_00732e54 & 0xffff0000;
}

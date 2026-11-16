/* SPDX-License-Identifier: MIT */
/* z_bop 00461690 */
void __cdecl FUN_00403540(unsigned int a, unsigned int b);
unsigned int __cdecl FUN_0041f410(int a, int b, int c);
unsigned char __cdecl FUN_00433390(unsigned int a, unsigned short b, unsigned short c);
void __cdecl FUN_00458e20(unsigned int a, int b);
int __cdecl FUN_004a5490(unsigned short a, unsigned short b);

extern unsigned short DAT_00a07cec;
extern int DAT_009a1fe0;
extern int DAT_009a1fe8;
extern char DAT_009a73d8;
extern unsigned short DAT_009a73b0;
extern unsigned int DAT_009a1ff8;
extern int DAT_00a08144;
extern int DAT_00a08140;
extern int DAT_00a08148;
extern int DAT_00a07cdc;
extern int DAT_00a07cd8;
extern int DAT_00a07cd4;
extern int DAT_009a1fd4;
extern int DAT_009a1fd8;
extern int DAT_009a1fd0;
extern unsigned int DAT_00a07ce8;
extern int DAT_009d1308;
extern unsigned char DAT_00521f2e[];

short __cdecl FUN_00461690(unsigned short param_1, unsigned short param_2, short param_3,
                           unsigned short param_4, unsigned short param_5, short param_6)
{
    unsigned char bVar1;
    char cVar2;
    unsigned char bVar3;
    unsigned short uVar4;
    unsigned short uVar8;
    unsigned int uVar9;
    unsigned int uVar12;
    short sVar11;
    short local_a;
    unsigned int local_8;
    int iVar5;
    unsigned char *rec;
    unsigned char *rec2;
    unsigned char *p;
    unsigned char *q;
    unsigned char *q2;
    unsigned char *p2;
    short s;
    int cond;
    int i32;
    unsigned int u32a;
    unsigned int u32b;
    int x, y, z;

    local_a = -1;
    local_8 = 0xffffffff;
    uVar9 = (unsigned int)DAT_00a07cec;
    if ((int)(unsigned int)DAT_00a07cec < DAT_009a1fe0) {
        do {
            uVar12 = uVar9 & 0xffff;
            sVar11 = (short)uVar9;
            rec = (unsigned char *)DAT_009a1fe8 + uVar12 * 0x23;
            if (rec[3] != 0) {
                bVar3 = FUN_00433390(uVar9, param_1, param_2);
                uVar8 = (unsigned short)bVar3;
                bVar3 = FUN_00433390(uVar9, param_4, param_5);
                uVar4 = (unsigned short)bVar3;
                if (param_3 == 1) {
                    uVar8 = (unsigned short)(uVar8 | uVar4);
                } else {
                    uVar8 = (unsigned short)(uVar8 & uVar4);
                }
                if (uVar8 != 0) {
                    cond = (DAT_009a73d8 == '\0');
                    if (!cond) {
                        __builtin_memcpy(&p, rec + 0x1f, 4);
                        __builtin_memcpy(&q, p + 0xa9, 4);
                        __builtin_memcpy(&s, q + 0x1e5, 2);
                        cond = (s != 0);
                    }
                    if (cond) {
                        if (sVar11 == -1) {
                            iVar5 = 0;
                        } else {
                            rec = (unsigned char *)DAT_009a1fe8 + uVar12 * 0x23;
                            __builtin_memcpy(&p, rec + 0x1f, 4);
                            if (p == 0) {
                                bVar3 = (rec[3] == 0);
LAB_a:
                                iVar5 = 1 - (unsigned int)bVar3;
                            } else if (rec[3] == 0) {
                                iVar5 = 0;
                            } else {
                                __builtin_memcpy(&q, p + 0xa9, 4);
                                if (q == 0) {
                                    iVar5 = 1;
                                } else if (q[0x5e] == 0x15 && q[0x5f] != 0) {
                                    iVar5 = 0;
                                } else {
                                    cVar2 = (char)q[7];
                                    if (cVar2 == 1 || cVar2 == 3 || cVar2 == 4) {
                                        iVar5 = 0;
                                    } else if (q[0x23d] == 3 && q[0x241] != 0) {
                                        FUN_00403540(uVar12, (unsigned int)DAT_009a73b0);
                                        cVar2 = *((char *)DAT_009a1fe8 + (unsigned int)DAT_009a73b0 * 0x23 + 2);
                                        if (cVar2 == 0 || cVar2 == 1 || cVar2 == 2) {
                                            if ((int)DAT_009a1ff8 < 0x8001) goto LAB_b;
                                            iVar5 = 0;
                                        } else {
                                            if ((int)DAT_009a1ff8 < 0x10001) goto LAB_b;
                                            iVar5 = 0;
                                        }
                                    } else {
LAB_b:
                                        rec2 = (unsigned char *)DAT_009a1fe8 + uVar12 * 0x23;
                                        __builtin_memcpy(&i32, rec2 + 0x1b, 4);
                                        if (i32 == -1) {
                                            __builtin_memcpy(&p2, rec2 + 0x1f, 4);
                                            __builtin_memcpy(&q2, p2 + 0xa9, 4);
                                            __builtin_memcpy(&u32a, q2 + 0xcb, 4);
                                            __builtin_memcpy(&u32b, q + 0xc3, 4);
                                            bVar3 = (u32a < u32b);
                                            goto LAB_a;
                                        }
                                        iVar5 = 1;
                                    }
                                }
                            }
                        }
                        if (iVar5 != 0) {
                            rec = (unsigned char *)DAT_009a1fe8 + uVar12 * 0x23;
                            __builtin_memcpy(&x, rec + 4, 4);
                            __builtin_memcpy(&y, rec + 8, 4);
                            __builtin_memcpy(&z, rec + 0xc, 4);
                            DAT_009a1ff8 = FUN_0041f410((int)((unsigned int)x - (unsigned int)DAT_00a08144),
                                                        (int)((unsigned int)y - (unsigned int)DAT_00a08140),
                                                        (int)((unsigned int)z - (unsigned int)DAT_00a08148));
                            if (DAT_009a1ff8 < local_8) {
                                if (param_6 != 0) {
                                    FUN_00458e20(uVar12, 0);
                                    DAT_009a1fd4 = DAT_00a07cdc;
                                    DAT_009a1fd8 = DAT_00a07cd8;
                                    DAT_009a1fd0 = DAT_00a07cd4;
                                    iVar5 = FUN_004a5490(DAT_009a73b0, DAT_009a73b0);
                                    if (iVar5 != 0) goto LAB_next1;
                                }
                                local_a = sVar11;
                                local_8 = DAT_009a1ff8;
                            }
                        }
                    }
                }
            }
LAB_next1:
            uVar9 = (uVar9 & 0xffff0000u) | (unsigned short)(sVar11 + 1U);
        } while ((int)(unsigned int)(unsigned short)(sVar11 + 1U) < DAT_009a1fe0);
    }
    uVar9 = (uVar9 & 0xffff0000u) | (unsigned short)DAT_00a07ce8;
    if ((int)(DAT_00a07ce8 & 0xffff) < (int)((unsigned int)DAT_009d1308 + DAT_00a07ce8)) {
        do {
            uVar12 = uVar9 & 0xffff;
            bVar1 = *((unsigned char *)DAT_009a1fe8 + uVar12 * 0x23 + 3);
            sVar11 = (short)uVar9;
            if (bVar1 != 0 && (DAT_00521f2e[(unsigned int)bVar1 * 0x1a] & 2) != 0) {
                bVar3 = FUN_00433390(uVar9, param_1, param_2);
                uVar8 = (unsigned short)bVar3;
                bVar3 = FUN_00433390(uVar9, param_4, param_5);
                uVar4 = (unsigned short)bVar3;
                if (param_3 == 1) {
                    uVar8 = (unsigned short)(uVar8 | uVar4);
                } else {
                    uVar8 = (unsigned short)(uVar8 & uVar4);
                }
                if (uVar8 != 0) {
                    if (sVar11 == -1) {
                        iVar5 = 0;
                    } else {
                        rec = (unsigned char *)DAT_009a1fe8 + uVar12 * 0x23;
                        __builtin_memcpy(&p, rec + 0x1f, 4);
                        if (p == 0) {
                            bVar3 = (rec[3] == 0);
LAB_c:
                            iVar5 = 1 - (unsigned int)bVar3;
                        } else if (rec[3] == 0) {
                            iVar5 = 0;
                        } else {
                            __builtin_memcpy(&q, p + 0xa9, 4);
                            if (q == 0) {
                                iVar5 = 1;
                            } else if (q[0x5e] == 0x15 && q[0x5f] != 0) {
                                iVar5 = 0;
                            } else {
                                cVar2 = (char)q[7];
                                if (cVar2 == 1 || cVar2 == 3 || cVar2 == 4) {
                                    iVar5 = 0;
                                } else if (q[0x23d] == 3 && q[0x241] != 0) {
                                    FUN_00403540(uVar12, (unsigned int)DAT_009a73b0);
                                    cVar2 = *((char *)DAT_009a1fe8 + (unsigned int)DAT_009a73b0 * 0x23 + 2);
                                    if (cVar2 == 0 || cVar2 == 1 || cVar2 == 2) {
                                        if ((int)DAT_009a1ff8 < 0x8001) goto LAB_d;
                                        iVar5 = 0;
                                    } else {
                                        if ((int)DAT_009a1ff8 < 0x10001) goto LAB_d;
                                        iVar5 = 0;
                                    }
                                } else {
LAB_d:
                                    rec2 = (unsigned char *)DAT_009a1fe8 + uVar12 * 0x23;
                                    __builtin_memcpy(&i32, rec2 + 0x1b, 4);
                                    if (i32 == -1) {
                                        __builtin_memcpy(&p2, rec2 + 0x1f, 4);
                                        __builtin_memcpy(&q2, p2 + 0xa9, 4);
                                        __builtin_memcpy(&u32a, q2 + 0xcb, 4);
                                        __builtin_memcpy(&u32b, q + 0xc3, 4);
                                        bVar3 = (u32a < u32b);
                                        goto LAB_c;
                                    }
                                    iVar5 = 1;
                                }
                            }
                        }
                    }
                    if (iVar5 != 0) {
                        FUN_00458e20(uVar12, 0);
                        DAT_009a1ff8 = FUN_0041f410((int)((unsigned int)DAT_00a07cdc - (unsigned int)DAT_00a08144),
                                                    (int)((unsigned int)DAT_00a07cd8 - (unsigned int)DAT_00a08140),
                                                    (int)((unsigned int)DAT_00a07cd4 - (unsigned int)DAT_00a08148));
                        if (DAT_009a1ff8 < local_8) {
                            if (param_6 != 0) {
                                DAT_009a1fd4 = DAT_00a07cdc;
                                DAT_009a1fd8 = DAT_00a07cd8;
                                DAT_009a1fd0 = DAT_00a07cd4;
                                iVar5 = FUN_004a5490(DAT_009a73b0, DAT_009a73b0);
                                if (iVar5 != 0) goto LAB_next2;
                            }
                            local_a = sVar11;
                            local_8 = DAT_009a1ff8;
                        }
                    }
                }
            }
LAB_next2:
            uVar9 = (uVar9 & 0xffff0000u) | (unsigned short)(sVar11 + 1U);
        } while ((int)(unsigned int)(unsigned short)(sVar11 + 1U) <
                 (int)((unsigned int)DAT_009d1308 + DAT_00a07ce8));
    }
    if (0x10000 < local_8) {
        local_a = -1;
    }
    return local_a;
}

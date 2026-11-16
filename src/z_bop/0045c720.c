/* SPDX-License-Identifier: MIT */
/* z_bop 0045c720 */
extern unsigned short DAT_00a07cec;
extern int DAT_009a1fe0;
extern int DAT_009a1fe8;
extern unsigned char DAT_009a73c0;
extern int DAT_00a08104;
extern unsigned short DAT_009a73b0;
extern unsigned int DAT_009a1ff8;
extern unsigned char DAT_009a73d8;
extern unsigned char DAT_009a73da;
extern unsigned char DAT_009a73db[4];
extern unsigned char DAT_009a73df[4];
extern unsigned char DAT_009a73e3[4];
extern int DAT_00a07ce8;
extern int DAT_009d1308;
extern unsigned char DAT_00521f2e[];

_Bool FUN_00433390(unsigned int param_1, unsigned short param_2, unsigned short param_3);
void FUN_00403540(unsigned int param_1, unsigned int param_2);
_Bool FUN_00403070(unsigned int param_1);
_Bool FUN_0045dba0(unsigned short param_1, short param_2);
unsigned int FUN_0041f410(int param_1, int param_2, int param_3);
unsigned int FUN_00484f80(unsigned short param_1);

unsigned int __cdecl FUN_0045c720(unsigned short param_1, unsigned short param_2, short param_3,
                                  unsigned short param_4, unsigned short param_5)
{
    unsigned char bVar1;
    char cVar2;
    _Bool bVar3;
    unsigned short uVar4;
    int iVar5;
    int iVar6;
    int iVar7;
    short sVar8;
    unsigned short uVar9;
    unsigned int uVar10;
    unsigned int uVar11;
    unsigned short uVar13;
    unsigned int unaff_EDI;
    unsigned short local_e;
    int local_c;
    unsigned int local_4;
    int tA;
    int tB;
    unsigned int tC;
    unsigned int tD;
    unsigned int tE;
    short sA;

    local_c = 0;
    uVar11 = DAT_00a07cec;
    if ((int)(unsigned int)DAT_00a07cec < DAT_009a1fe0) {
        do {
            uVar10 = uVar11 & 0xffff;
            iVar5 = DAT_009a1fe8 + uVar10 * 0x23;
            sVar8 = (short)uVar11;
            if ((*(char *)(iVar5 + 3) != '\0') && (*(unsigned char *)(iVar5 + 0x16) != DAT_009a73c0)) {
                bVar3 = FUN_00433390(uVar11, param_1, param_2);
                uVar13 = (unsigned short)(unsigned int)bVar3;
                bVar3 = FUN_00433390(uVar11, param_4, param_5);
                uVar9 = (unsigned short)(unsigned int)bVar3;
                if (param_3 == 1) {
                    uVar13 = uVar13 | uVar9;
                } else {
                    uVar13 = uVar13 & uVar9;
                }
                unaff_EDI = (unaff_EDI & 0xffff0000U) | uVar13;
                if ((uVar13 != 0) && (*(short *)(DAT_00a08104 + 0x36e) != sVar8)) {
                    if (sVar8 == -1) {
                        iVar5 = 0;
                    } else {
                        iVar5 = DAT_009a1fe8 + uVar10 * 0x23;
                        __builtin_memcpy(&tA, (const void *)(iVar5 + 0x1f), 4);
                        if (tA == 0) {
                            bVar3 = *(char *)(iVar5 + 3) == '\0';
LAB_0045c90f:
                            iVar5 = 1 - (unsigned int)bVar3;
                        } else if (*(char *)(iVar5 + 3) == '\0') {
                            iVar5 = 0;
                        } else {
                            __builtin_memcpy(&unaff_EDI, (const void *)(tA + 0xa9), 4);
                            if (unaff_EDI == 0) {
                                iVar5 = 1;
                            } else if ((*(char *)(unaff_EDI + 0x5e) == '\x15') &&
                                       (*(char *)(unaff_EDI + 0x5f) != '\0')) {
                                iVar5 = 0;
                            } else {
                                cVar2 = *(char *)(unaff_EDI + 7);
                                if ((cVar2 == '\x01') || ((cVar2 == '\x03' || (cVar2 == '\x04')))) {
                                    iVar5 = 0;
                                } else if ((*(char *)(unaff_EDI + 0x23d) == '\x03') &&
                                           (*(char *)(unaff_EDI + 0x241) != '\0')) {
                                    FUN_00403540(uVar10, (unsigned int)DAT_009a73b0);
                                    cVar2 = *(char *)((unsigned int)DAT_009a73b0 * 0x23 + 2 + DAT_009a1fe8);
                                    if ((cVar2 == '\0') || ((cVar2 == '\x01' || (cVar2 == '\x02')))) {
                                        if ((int)DAT_009a1ff8 < 0x8001) goto LAB_0045c8d3;
                                        iVar5 = 0;
                                    } else {
                                        if ((int)DAT_009a1ff8 < 0x10001) goto LAB_0045c8d3;
                                        iVar5 = 0;
                                    }
                                } else {
LAB_0045c8d3:
                                    iVar5 = DAT_009a1fe8 + uVar10 * 0x23;
                                    __builtin_memcpy(&tA, (const void *)(iVar5 + 0x1b), 4);
                                    if (tA == -1) {
                                        __builtin_memcpy(&tA, (const void *)(iVar5 + 0x1f), 4);
                                        __builtin_memcpy(&tB, (const void *)(tA + 0xa9), 4);
                                        __builtin_memcpy(&tC, (const void *)(tB + 0xcb), 4);
                                        __builtin_memcpy(&tD, (const void *)(unaff_EDI + 0xc3), 4);
                                        bVar3 = tC < tD;
                                        goto LAB_0045c90f;
                                    }
                                    iVar5 = 1;
                                }
                            }
                        }
                    }
                    if (iVar5 != 0) {
                        __builtin_memcpy(&iVar5, (const void *)(DAT_009a1fe8 + 0x1f + uVar10 * 0x23), 4);
                        __builtin_memcpy(&iVar6, (const void *)(iVar5 + 0xa9), 4);
                        if ((DAT_009a73d8 == '\0') ||
                            ((__builtin_memcpy(&sA, (const void *)(iVar6 + 0x1e5), 2), sA != 0) &&
                             (((DAT_009a73d8 == '\0' || (*(char *)(iVar6 + 0x1eb) == '\0')) ||
                               (__builtin_memcpy(&uVar10,
                                                 (const void *)((unsigned int)DAT_009a73b0 * 0x23 + 0x1f +
                                                                DAT_009a1fe8),
                                                 4),
                                *(char *)(uVar10 + 0x89) != *(char *)(iVar5 + 0x89)))))) {
                            if ((DAT_009a73da & 4) != 0) {
                                bVar3 = FUN_00403070(uVar11);
                                if ((short)(unsigned int)bVar3 == 0) goto LAB_0045c99a;
                            }
                            local_c = local_c + 1;
                        }
                    }
                }
            }
LAB_0045c99a:
            uVar11 = (uVar11 & 0xffff0000U) | (unsigned short)((unsigned int)(int)sVar8 + 1U);
        } while ((int)(unsigned int)(unsigned short)((unsigned int)(int)sVar8 + 1U) < DAT_009a1fe0);
    }
    uVar11 = (unsigned short)DAT_00a07ce8;
    if ((int)((unsigned int)DAT_00a07ce8 & 0xffff) <
        (int)((unsigned int)DAT_009d1308 + (unsigned int)DAT_00a07ce8)) {
        do {
            unaff_EDI = uVar11 & 0xffff;
            bVar1 = *(unsigned char *)(unaff_EDI * 0x23 + 3 + DAT_009a1fe8);
            sVar8 = (short)uVar11;
            if ((bVar1 != 0) && ((DAT_00521f2e[(unsigned int)bVar1 * 0x1a] & 2) != 0)) {
                bVar3 = FUN_00433390(uVar11, param_1, param_2);
                uVar13 = (unsigned short)(unsigned int)bVar3;
                bVar3 = FUN_00433390(uVar11, param_4, param_5);
                uVar9 = (unsigned short)(unsigned int)bVar3;
                if (param_3 == 1) {
                    uVar13 = uVar13 | uVar9;
                } else {
                    uVar13 = uVar13 & uVar9;
                }
                if ((uVar13 != 0) && (*(short *)(DAT_00a08104 + 0x36e) != sVar8)) {
                    if (sVar8 == -1) {
                        iVar5 = 0;
                    } else {
                        iVar5 = DAT_009a1fe8 + unaff_EDI * 0x23;
                        __builtin_memcpy(&tA, (const void *)(iVar5 + 0x1f), 4);
                        if (tA == 0) {
                            bVar3 = *(char *)(iVar5 + 3) == '\0';
LAB_0045cba3:
                            iVar5 = 1 - (unsigned int)bVar3;
                        } else if (*(char *)(iVar5 + 3) == '\0') {
                            iVar5 = 0;
                        } else {
                            __builtin_memcpy(&iVar5, (const void *)(tA + 0xa9), 4);
                            if (iVar5 == 0) {
                                iVar5 = 1;
                            } else if ((*(char *)(iVar5 + 0x5e) == '\x15') &&
                                       (*(char *)(iVar5 + 0x5f) != '\0')) {
                                iVar5 = 0;
                            } else {
                                cVar2 = *(char *)(iVar5 + 7);
                                if ((cVar2 == '\x01') || ((cVar2 == '\x03' || (cVar2 == '\x04')))) {
                                    iVar5 = 0;
                                } else if ((*(char *)(iVar5 + 0x23d) == '\x03') &&
                                           (*(char *)(iVar5 + 0x241) != '\0')) {
                                    FUN_00403540(unaff_EDI, (unsigned int)DAT_009a73b0);
                                    cVar2 = *(char *)((unsigned int)DAT_009a73b0 * 0x23 + 2 + DAT_009a1fe8);
                                    if ((cVar2 == '\0') || ((cVar2 == '\x01' || (cVar2 == '\x02')))) {
                                        if ((int)DAT_009a1ff8 < 0x8001) goto LAB_0045cb67;
                                        iVar5 = 0;
                                    } else {
                                        if ((int)DAT_009a1ff8 < 0x10001) goto LAB_0045cb67;
                                        iVar5 = 0;
                                    }
                                } else {
LAB_0045cb67:
                                    iVar6 = DAT_009a1fe8 + unaff_EDI * 0x23;
                                    __builtin_memcpy(&tA, (const void *)(iVar6 + 0x1b), 4);
                                    if (tA == -1) {
                                        __builtin_memcpy(&tA, (const void *)(iVar6 + 0x1f), 4);
                                        __builtin_memcpy(&tB, (const void *)(tA + 0xa9), 4);
                                        __builtin_memcpy(&tC, (const void *)(tB + 0xcb), 4);
                                        __builtin_memcpy(&tD, (const void *)(iVar5 + 0xc3), 4);
                                        bVar3 = tC < tD;
                                        goto LAB_0045cba3;
                                    }
                                    iVar5 = 1;
                                }
                            }
                        }
                    }
                    if (iVar5 != 0) {
                        if ((DAT_009a73da & 4) != 0) {
                            bVar3 = FUN_00403070(uVar11);
                            if ((short)(unsigned int)bVar3 == 0) goto LAB_0045cbc5;
                        }
                        local_c = local_c + 1;
                    }
                }
            }
LAB_0045cbc5:
            uVar11 = (uVar11 & 0xffff0000U) | (unsigned short)((unsigned int)(int)sVar8 + 1U);
        } while ((int)(unsigned int)(unsigned short)((unsigned int)(int)sVar8 + 1U) <
                 (int)((unsigned int)DAT_009d1308 + (unsigned int)DAT_00a07ce8));
    }
    if (local_c == 0) {
        return 0xffff;
    }
    uVar11 = (unaff_EDI & 0xffff0000U) | DAT_00a07cec;
    local_4 = 0xffffffff;
    local_e = 0xffff;
    if ((int)(unsigned int)DAT_00a07cec < DAT_009a1fe0) {
        do {
            uVar10 = uVar11 & 0xffff;
            iVar5 = uVar10 * 0x23;
            uVar13 = (unsigned short)uVar11;
            if ((*(char *)(DAT_009a1fe8 + iVar5 + 3) != '\0') &&
                (*(unsigned char *)(DAT_009a1fe8 + iVar5 + 0x16) != DAT_009a73c0)) {
                bVar3 = FUN_00433390(uVar11, param_1, param_2);
                uVar9 = (unsigned short)(unsigned int)bVar3;
                bVar3 = FUN_00433390(uVar11, param_4, param_5);
                uVar4 = (unsigned short)(unsigned int)bVar3;
                if (param_3 == 1) {
                    uVar9 = uVar9 | uVar4;
                } else {
                    uVar9 = uVar9 & uVar4;
                }
                if ((uVar9 != 0) && (*(unsigned short *)(DAT_00a08104 + 0x36e) != uVar13)) {
                    if (uVar13 == 0xffff) {
                        iVar6 = 0;
                    } else {
                        iVar6 = DAT_009a1fe8 + uVar10 * 0x23;
                        __builtin_memcpy(&tA, (const void *)(iVar6 + 0x1f), 4);
                        if (tA == 0) {
                            bVar3 = *(char *)(iVar6 + 3) == '\0';
LAB_0045cde6:
                            iVar6 = 1 - (unsigned int)bVar3;
                        } else if (*(char *)(iVar6 + 3) == '\0') {
                            iVar6 = 0;
                        } else {
                            __builtin_memcpy(&iVar6, (const void *)(tA + 0xa9), 4);
                            if (iVar6 == 0) {
                                iVar6 = 1;
                            } else if ((*(char *)(iVar6 + 0x5e) == '\x15') &&
                                       (*(char *)(iVar6 + 0x5f) != '\0')) {
                                iVar6 = 0;
                            } else {
                                cVar2 = *(char *)(iVar6 + 7);
                                if ((cVar2 == '\x01') || ((cVar2 == '\x03' || (cVar2 == '\x04')))) {
                                    iVar6 = 0;
                                } else if ((*(char *)(iVar6 + 0x23d) == '\x03') &&
                                           (*(char *)(iVar6 + 0x241) != '\0')) {
                                    FUN_00403540(uVar10, (unsigned int)DAT_009a73b0);
                                    cVar2 = *(char *)((unsigned int)DAT_009a73b0 * 0x23 + 2 + DAT_009a1fe8);
                                    if ((cVar2 == '\0') || ((cVar2 == '\x01' || (cVar2 == '\x02')))) {
                                        if ((int)DAT_009a1ff8 < 0x8001) goto LAB_0045cdaa;
                                        iVar6 = 0;
                                    } else {
                                        if ((int)DAT_009a1ff8 < 0x10001) goto LAB_0045cdaa;
                                        iVar6 = 0;
                                    }
                                } else {
LAB_0045cdaa:
                                    iVar7 = DAT_009a1fe8 + uVar10 * 0x23;
                                    __builtin_memcpy(&tA, (const void *)(iVar7 + 0x1b), 4);
                                    if (tA == -1) {
                                        __builtin_memcpy(&tA, (const void *)(iVar7 + 0x1f), 4);
                                        __builtin_memcpy(&tB, (const void *)(tA + 0xa9), 4);
                                        __builtin_memcpy(&tC, (const void *)(tB + 0xcb), 4);
                                        __builtin_memcpy(&tD, (const void *)(iVar6 + 0xc3), 4);
                                        bVar3 = tC < tD;
                                        goto LAB_0045cde6;
                                    }
                                    iVar6 = 1;
                                }
                            }
                        }
                    }
                    if (iVar6 != 0) {
                        __builtin_memcpy(&iVar6, (const void *)(DAT_009a1fe8 + 0x1f + iVar5), 4);
                        __builtin_memcpy(&iVar7, (const void *)(iVar6 + 0xa9), 4);
                        if ((DAT_009a73d8 == '\0') ||
                            ((__builtin_memcpy(&sA, (const void *)(iVar7 + 0x1e5), 2), sA != 0) &&
                             (((DAT_009a73d8 == '\0' || (*(char *)(iVar7 + 0x1eb) == '\0')) ||
                               (__builtin_memcpy(&tE,
                                                 (const void *)((unsigned int)DAT_009a73b0 * 0x23 + 0x1f +
                                                                DAT_009a1fe8),
                                                 4),
                                *(char *)(tE + 0x89) != *(char *)(iVar6 + 0x89)))))) {
                            if ((DAT_009a73da & 4) != 0) {
                                bVar3 = FUN_00403070(uVar11);
                                if ((short)(unsigned int)bVar3 == 0) goto LAB_0045cf20;
                            }
                            if ((DAT_009a73da & 1) != 0) {
                                bVar3 = FUN_0045dba0(uVar13, (short)local_c);
                                if ((short)(unsigned int)bVar3 == 0) goto LAB_0045cf20;
                            }
                            if ((DAT_009a73da & 0x20) == 0) {
                                FUN_00403540((unsigned int)DAT_009a73b0, uVar10);
                            } else {
                                iVar5 = iVar5 + DAT_009a1fe8;
                                __builtin_memcpy(&tC, DAT_009a73db, 4);
                                __builtin_memcpy(&tD, DAT_009a73df, 4);
                                __builtin_memcpy(&tE, DAT_009a73e3, 4);
                                DAT_009a1ff8 = FUN_0041f410(*(int *)(iVar5 + 4) - tC,
                                                            *(int *)(iVar5 + 8) - tD,
                                                            *(int *)(iVar5 + 0xc) - tE);
                            }
                            if (0x4000 < (int)DAT_009a1ff8) {
                                uVar10 = FUN_00484f80(uVar13);
                                if ((char)uVar10 == '\x01') goto LAB_0045cf20;
                            }
                            if (DAT_009a1ff8 < local_4) {
                                local_4 = DAT_009a1ff8;
                                local_e = uVar13;
                            }
                        }
                    }
                }
            }
LAB_0045cf20:
            uVar11 = (uVar11 & 0xffff0000U) | (unsigned short)(uVar13 + 1);
        } while ((int)(unsigned int)(unsigned short)(uVar13 + 1) < DAT_009a1fe0);
    }
    uVar10 = (uVar11 & 0xffff0000U) | (unsigned short)DAT_00a07ce8;
    uVar11 = (unsigned int)DAT_00a07ce8 & 0xffff;
    if ((int)uVar11 < (int)((unsigned int)DAT_009d1308 + (unsigned int)DAT_00a07ce8)) {
        do {
            uVar11 = uVar10 & 0xffff;
            bVar1 = *(unsigned char *)(uVar11 * 0x23 + 3 + DAT_009a1fe8);
            uVar13 = (unsigned short)uVar10;
            if ((bVar1 != 0) && ((DAT_00521f2e[(unsigned int)bVar1 * 0x1a] & 2) != 0)) {
                bVar3 = FUN_00433390(uVar10, param_1, param_2);
                uVar9 = (unsigned short)(unsigned int)bVar3;
                bVar3 = FUN_00433390(uVar10, param_4, param_5);
                uVar4 = (unsigned short)(unsigned int)bVar3;
                if (param_3 == 1) {
                    uVar9 = uVar9 | uVar4;
                } else {
                    uVar9 = uVar9 & uVar4;
                }
                if ((uVar9 != 0) && (*(unsigned short *)(DAT_00a08104 + 0x36e) != uVar13)) {
                    if (uVar13 == 0xffff) {
                        iVar5 = 0;
                    } else {
                        iVar5 = DAT_009a1fe8 + uVar11 * 0x23;
                        __builtin_memcpy(&tA, (const void *)(iVar5 + 0x1f), 4);
                        if (tA == 0) {
                            bVar3 = *(char *)(iVar5 + 3) == '\0';
LAB_0045d12a:
                            iVar5 = 1 - (unsigned int)bVar3;
                        } else if (*(char *)(iVar5 + 3) == '\0') {
                            iVar5 = 0;
                        } else {
                            __builtin_memcpy(&iVar5, (const void *)(tA + 0xa9), 4);
                            if (iVar5 == 0) {
                                iVar5 = 1;
                            } else if ((*(char *)(iVar5 + 0x5e) == '\x15') &&
                                       (*(char *)(iVar5 + 0x5f) != '\0')) {
                                iVar5 = 0;
                            } else {
                                cVar2 = *(char *)(iVar5 + 7);
                                if ((cVar2 == '\x01') || ((cVar2 == '\x03' || (cVar2 == '\x04')))) {
                                    iVar5 = 0;
                                } else if ((*(char *)(iVar5 + 0x23d) == '\x03') &&
                                           (*(char *)(iVar5 + 0x241) != '\0')) {
                                    FUN_00403540(uVar11, (unsigned int)DAT_009a73b0);
                                    cVar2 = *(char *)((unsigned int)DAT_009a73b0 * 0x23 + 2 + DAT_009a1fe8);
                                    if ((cVar2 == '\0') || ((cVar2 == '\x01' || (cVar2 == '\x02')))) {
                                        if ((int)DAT_009a1ff8 < 0x8001) goto LAB_0045d0ee;
                                        iVar5 = 0;
                                    } else {
                                        if ((int)DAT_009a1ff8 < 0x10001) goto LAB_0045d0ee;
                                        iVar5 = 0;
                                    }
                                } else {
LAB_0045d0ee:
                                    iVar6 = DAT_009a1fe8 + uVar11 * 0x23;
                                    __builtin_memcpy(&tA, (const void *)(iVar6 + 0x1b), 4);
                                    if (tA == -1) {
                                        __builtin_memcpy(&tA, (const void *)(iVar6 + 0x1f), 4);
                                        __builtin_memcpy(&tB, (const void *)(tA + 0xa9), 4);
                                        __builtin_memcpy(&tC, (const void *)(tB + 0xcb), 4);
                                        __builtin_memcpy(&tD, (const void *)(iVar5 + 0xc3), 4);
                                        bVar3 = tC < tD;
                                        goto LAB_0045d12a;
                                    }
                                    iVar5 = 1;
                                }
                            }
                        }
                    }
                    if (iVar5 != 0) {
                        if ((DAT_009a73da & 4) != 0) {
                            bVar3 = FUN_00403070(uVar10);
                            if ((short)(unsigned int)bVar3 == 0) goto LAB_0045d18c;
                        }
                        if ((DAT_009a73da & 1) != 0) {
                            bVar3 = FUN_0045dba0(uVar13, (short)local_c);
                            if ((short)(unsigned int)bVar3 == 0) goto LAB_0045d18c;
                        }
                        FUN_00403540((unsigned int)DAT_009a73b0, uVar11);
                        if (DAT_009a1ff8 < local_4) {
                            local_4 = DAT_009a1ff8;
                            local_e = uVar13;
                        }
                    }
                }
            }
LAB_0045d18c:
            uVar10 = (uVar10 & 0xffff0000U) | (unsigned short)(uVar13 + 1);
            uVar11 = (unsigned int)DAT_009d1308 + (unsigned int)DAT_00a07ce8;
        } while ((int)(unsigned int)(unsigned short)(uVar13 + 1) < (int)uVar11);
    }
    return (uVar11 & 0xffff0000U) | (unsigned int)local_e;
}

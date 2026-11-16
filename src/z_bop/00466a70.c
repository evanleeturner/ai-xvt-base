/* SPDX-License-Identifier: MIT */
/* z_bop 00466a70 */
/* Globals (types inferred from decompile usage) */
extern unsigned char *DAT_00a08104;
extern unsigned char DAT_009a73b6[4]; /* packed 4-byte pointer global at an unaligned address */
extern unsigned char *DAT_009a1fe8;
extern unsigned short DAT_009a73c0;
extern unsigned short DAT_009a73b0;
extern unsigned short DAT_00a07cec;
extern int DAT_009a1fe0;
extern int DAT_009a8c00;
extern int DAT_009d8a58;
extern int DAT_009d8a54;
extern int DAT_009d8b60;

extern unsigned char DAT_009d8ccc[];
extern unsigned char DAT_009d8cce[];
extern unsigned char DAT_009d8ccf[];
extern unsigned char DAT_009d8cd0[];
extern char DAT_009d8c80[];
extern unsigned char DAT_009d8c88[];
extern unsigned char DAT_0051c641[];
extern unsigned char DAT_0051c645[];
extern unsigned char DAT_0051c649[];
extern unsigned char DAT_009a20ea[];
extern unsigned char DAT_009a20f8[];
extern unsigned char DAT_009a20fa[];
extern unsigned char DAT_009a2100[];
extern unsigned char DAT_009a2112[];
extern unsigned char DAT_009a2119[];
extern unsigned char DAT_009a211a[];
extern unsigned char DAT_009a211d[];
extern unsigned char DAT_009a2146[];
extern unsigned char DAT_009a2150[];
extern unsigned char DAT_009a215a[];
extern unsigned char DAT_009a2164[];

/* Prototypes (signatures/conventions not confirmed by evidence) */
unsigned long long FUN_00402e20(unsigned short a);
void FUN_004037b0(unsigned char *a, int b, int c, int d);
void FUN_00403990(void);
void FUN_004a4d90(unsigned int a, unsigned int b);
unsigned int FUN_00435850(int a, unsigned int b, unsigned int c);
void FUN_00451c20(unsigned int a, void *b, int c);
void FUN_00433d30(unsigned short a, unsigned short b, unsigned short c);
unsigned int FUN_00458780(int a);
void FUN_00435930(int a, unsigned int b, unsigned int c, unsigned int d);

#define CONCAT22(h, l) ((((unsigned int)(unsigned short)(h)) << 16) | (unsigned int)(unsigned short)(l))
#define INC16(p) (*(short *)(p) = (short)(*(short *)(p) + 1))

unsigned int __stdcall FUN_00466a70(void)
{
    unsigned char *e;
    unsigned char *obj;
    unsigned char *puVar6;
    unsigned char *pcVar15;
    unsigned char *p73b6;
    unsigned char uVar2;
    unsigned char bVar5;
    unsigned int uVar8;
    unsigned int uVar9;
    unsigned int uVar11;
    unsigned int uVar12;
    unsigned int extraout_var_00;
    unsigned int uVar17;
    unsigned int iVar1;
    unsigned int iVar10;
    unsigned int iVar14;
    unsigned int unaff_EDI;
    unsigned short uVar13;
    unsigned short uVar16;
    unsigned short uStack_8;
    unsigned int local_4;
    unsigned int t32;
    unsigned int tv;
    unsigned short t16;
    unsigned char *tp;
    unsigned char *iVar7;
    char cVar3;
    char cVar4;
    _Bool bVar18;
    int cond;

    unaff_EDI = 0;
    uVar13 = 0;
    uVar16 = 0;
    uStack_8 = 0;

    DAT_00a08104[0xaf] = 1;
    tv = 0x1d;
    __builtin_memcpy(&p73b6, DAT_009a73b6, 4);
    __builtin_memcpy(p73b6 + 0xe, &tv, 4);

    if (DAT_00a08104[0x1eb] == 0) {
        uVar16 = (unsigned short)FUN_00402e20((unsigned short)DAT_009d8ccc[(unsigned int)DAT_009a73c0 * 0x566]);
        uStack_8 = 0x12;
        if (uVar16 == 0xffff) {
            if (DAT_009d8ccf[(unsigned int)DAT_009a73c0 * 0x566] != 0) {
                uVar16 = (unsigned short)FUN_00402e20((unsigned short)DAT_009d8cce[(unsigned int)DAT_009a73c0 * 0x566]);
                uStack_8 = 0x13;
            }
        }
    } else {
        uVar16 = (unsigned short)FUN_00402e20((unsigned short)DAT_009d8cd0[(unsigned int)DAT_009a73c0 * 0x566]);
        uStack_8 = 0x14;
    }

    if (uVar16 != 0xffff) {
        __builtin_memcpy(&p73b6, DAT_009a73b6, 4);
        t16 = uVar16;
        __builtin_memcpy(p73b6 + 0x18, &t16, 2);
        unaff_EDI = (unsigned int)uVar16 * 0x23;
        __builtin_memcpy(&t16, DAT_009a1fe8 + unaff_EDI, 2);
        __builtin_memcpy(&p73b6, DAT_009a73b6, 4);
        __builtin_memcpy(p73b6 + 0x1a, &t16, 2);
        __builtin_memcpy(&p73b6, DAT_009a73b6, 4);
        p73b6[0x1e] = 0;
        __builtin_memcpy(&obj, DAT_009a1fe8 + unaff_EDI + 0x1f, 4);
        __builtin_memcpy(&tp, obj + 0xa9, 4);
        uVar2 = tp[4];
        uVar8 = (unsigned int)(unsigned short)(((unsigned int)uVar2 << 8) | uVar2);
        {
            int a1, a2, a3;
            __builtin_memcpy(&a1, DAT_0051c641 + uVar8, 4);
            __builtin_memcpy(&a2, DAT_0051c645 + uVar8, 4);
            __builtin_memcpy(&a3, DAT_0051c649 + uVar8, 4);
            FUN_004037b0(DAT_009a1fe8 + unaff_EDI, a1, a2, a3);
        }
        __builtin_memcpy(&t32, DAT_009a1fe8 + 4 + unaff_EDI, 4);
        t32 = t32 + (unsigned int)DAT_009d8a58;
        __builtin_memcpy(&p73b6, DAT_009a73b6, 4);
        __builtin_memcpy(p73b6 + 0x1f, &t32, 4);
        __builtin_memcpy(&t32, DAT_009a1fe8 + 8 + unaff_EDI, 4);
        t32 = t32 + (unsigned int)DAT_009d8a54;
        __builtin_memcpy(&p73b6, DAT_009a73b6, 4);
        __builtin_memcpy(p73b6 + 0x23, &t32, 4);
        __builtin_memcpy(&t32, DAT_009a1fe8 + 0xc + unaff_EDI, 4);
        t32 = t32 + (unsigned int)DAT_009d8b60;
        __builtin_memcpy(&p73b6, DAT_009a73b6, 4);
        __builtin_memcpy(p73b6 + 0x27, &t32, 4);
    }

    FUN_00403990();

    if (uVar16 == 0xffff) {
        FUN_004a4d90((unsigned int)DAT_009a73b0, 0x23);
        return 1;
    }

    __builtin_memcpy(&tp, DAT_009a1fe8 + (unsigned int)uVar16 * 0x23 + 0x1f, 4);
    __builtin_memcpy(&uVar16, tp + 0x79, 2);
    if (uVar16 < 0x19) {
        uVar9 = 0x28;
    } else {
        uVar9 = (unsigned int)uVar16 + 0x19;
    }
    FUN_004a4d90((unsigned int)DAT_009a73b0, uVar9);

    uVar9 = ((0u - (unsigned int)(DAT_009a1fe8[(unsigned int)DAT_009a73b0 * 0x23 + 2] == 4)) & 0x200) + 0x200;
    if ((int)uVar9 <= DAT_009a8c00) goto LAB_004672ed;

    uVar9 = ((unaff_EDI >> 16) << 16) | (unsigned int)DAT_00a07cec;
    if ((int)(unsigned int)DAT_00a07cec < DAT_009a1fe0) {
        do {
            iVar10 = (uVar9 & 0xffff) * 0x23;
            e = DAT_009a1fe8 + iVar10;
            if (e[3] != 0) {
                uVar11 = (unsigned int)DAT_009a73c0;
                if (e[0x16] == uVar11) {
                    __builtin_memcpy(&obj, e + 0x1f, 4);
                    __builtin_memcpy(&puVar6, obj + 0xa9, 4);
                    if (puVar6[5] != 0xff) {
                        if (puVar6[0x5e] == 10) {
                            __builtin_memcpy(&t32, e + 0x1b, 4);
                            if ((int)t32 == -1) {
                                if (puVar6[0x1eb] == 0) {
                                    if ((puVar6[0x33] == 0) &&
                                        ((puVar6[0x72] != 0) ||
                                         ((puVar6[0x73] == 0) && (puVar6[0x74] == 0)))) {
                                        iVar14 = uVar11 * 0x126;
                                        INC16(DAT_009a20ea + iVar14);
                                        cVar3 = (char)puVar6[0xb0];
                                        cVar4 = DAT_009d8c80[uVar11 * 0x566];
                                        if (cVar4 == cVar3) {
                                            DAT_009a2112[iVar14] = 1;
                                        }
                                        local_4 = (unsigned int)(cVar4 == cVar3);
                                        FUN_00435850(0xc, CONCAT22(iVar14 >> 16, DAT_009a73c0), local_4);
                                    }
                                    if ((puVar6[0x1eb] == 0) && (puVar6[0x33] == 1) &&
                                        (puVar6[0x73] == 0) && (puVar6[0x74] == 0)) {
                                        uVar11 = (unsigned int)DAT_009a73c0;
                                        INC16(DAT_009a2100 + uVar11 * 0x126);
                                        if (DAT_009d8c80[uVar11 * 0x566] == (char)puVar6[0xb0]) {
                                            DAT_009a211d[uVar11 * 0x126] = 1;
                                        }
                                    }
                                }
                                FUN_00451c20(uVar9, puVar6, 0x8d);
                                FUN_00433d30((unsigned short)uVar9, DAT_009a73c0, uStack_8);
                                DAT_009a1fe8[3 + iVar10] = 0;
                                FUN_00458780((int)puVar6);
                            }
                        }
                    }
                }
            }
            uVar16 = (unsigned short)((unsigned short)uVar9 + 1);
            uVar9 = ((uVar9 >> 16) << 16) | (unsigned int)uVar16;
        } while ((int)(unsigned int)uVar16 < DAT_009a1fe0);
    }

    uVar12 = (unsigned int)DAT_009a1fe0 >> 8;
    cond = 0;
    if (DAT_00a08104[0x1eb] == 0) {
        __builtin_memcpy(&p73b6, DAT_009a73b6, 4);
        if (p73b6[9] == 0) {
            if ((DAT_00a08104[0x72] != 0) ||
                ((DAT_00a08104[0x73] == 0) && (DAT_00a08104[0x74] == 0))) {
                cond = 1;
            }
        }
    }
    if (cond) {
        uVar9 = (unsigned int)DAT_009a73c0;
        iVar10 = uVar9 * 0x126;
        INC16(DAT_009a20ea + iVar10);
        cVar3 = DAT_009d8c80[uVar9 * 0x566];
        cVar4 = (char)DAT_00a08104[0xb0];
        if (cVar3 == cVar4) {
            DAT_009a2112[iVar10] = 1;
        }
        /* extraout_var_00: upper 3 bytes of EAX left by the call (FUN_00435850's return value) */
        extraout_var_00 = FUN_00435850(0xc, CONCAT22(iVar10 >> 16, DAT_009a73c0), (unsigned int)(cVar3 == cVar4)) >> 8;
        uVar12 = extraout_var_00;
    }

    pcVar15 = DAT_00a08104 + 0x1eb;
    if (*pcVar15 == 0) {
        __builtin_memcpy(&p73b6, DAT_009a73b6, 4);
        iVar10 = (uVar12 << 8) | (unsigned int)p73b6[9];
        __builtin_memcpy(&p73b6, DAT_009a73b6, 4);
        if (p73b6[9] == 1) {
            iVar10 = (uVar12 << 8) | (unsigned int)DAT_00a08104[0x73];
            if ((DAT_00a08104[0x73] == 0) &&
                (iVar10 = (uVar12 << 8) | (unsigned int)DAT_00a08104[0x74],
                 DAT_00a08104[0x74] == 0)) {
                uVar9 = (unsigned int)DAT_009a73c0;
                iVar10 = uVar9 * 0x126;
                INC16(DAT_009a2100 + iVar10);
                if (DAT_009d8c80[uVar9 * 0x566] == (char)DAT_00a08104[0xb0]) {
                    DAT_009a211d[iVar10] = 1;
                }
            }
        }
        uVar13 = (unsigned short)(iVar10 >> 16);
        if (*pcVar15 != 0) goto LAB_00466fa8;
    } else {
LAB_00466fa8:
        uVar9 = (unsigned int)DAT_009a73c0;
        __builtin_memcpy(&obj, DAT_009a1fe8 + (unsigned int)DAT_009a73b0 * 0x23 + 0x1f, 4);
        bVar5 = obj[0x89];
        uVar11 = (unsigned int)bVar5;
        iVar10 = uVar11 + uVar9 * 0x126;
        DAT_009a2146[iVar10] = (unsigned char)(DAT_009a2146[uVar9 * 0x126 + uVar11] + 1);
        cVar3 = (char)DAT_00a08104[0xb0];
        cVar4 = DAT_009d8c80[uVar9 * 0x566];
        if (cVar4 == cVar3) {
            DAT_009a2150[iVar10] = (unsigned char)(DAT_009a2150[iVar10] + 1);
        }
        FUN_00435930(0x2c, CONCAT22(iVar10 >> 16, DAT_009a73c0), (unsigned int)(cVar4 == cVar3), (unsigned int)bVar5);
        uVar9 = 0;
        uVar13 = 0;
        do {
            puVar6 = DAT_00a08104;
            if (uVar9 != uVar11) {
                uVar17 = (unsigned int)DAT_009a73c0;
                if ((unsigned int)DAT_009d8c88[uVar17 * 0x566] != uVar9) {
                    iVar10 = uVar9 + uVar17 * 0x126;
                    DAT_009a215a[iVar10] = (unsigned char)(DAT_009a215a[uVar17 * 0x126 + uVar9] + 1);
                    if (DAT_009d8c80[uVar17 * 0x566] == (char)puVar6[0xb0]) {
                        DAT_009a2164[iVar10] = 1;
                    }
                }
            }
            uVar9 = uVar9 + 1;
        } while (uVar9 < 10);
    }

    FUN_00451c20(CONCAT22(uVar13, DAT_009a73b0), DAT_00a08104, 0x8d);
    FUN_00433d30(DAT_009a73b0, DAT_009a73c0, uStack_8);
    DAT_009a1fe8[(unsigned int)DAT_009a73b0 * 0x23 + 3] = 0;
    uVar9 = FUN_00458780((int)DAT_00a08104);

    __builtin_memcpy(&uVar16, DAT_00a08104 + 0x66, 2);
    if (uVar16 != 0xffff) {
        iVar10 = (unsigned int)uVar16 * 0x23;
        e = DAT_009a1fe8 + iVar10;
        uVar9 = (unsigned int)e;
        __builtin_memcpy(&t32, e + 0x1f, 4);
        if (t32 != 0) {
            uVar8 = (unsigned int)e[0x16];
            FUN_00433d30(uVar16, (unsigned short)uVar8, uStack_8);
            __builtin_memcpy(&obj, DAT_009a1fe8 + 0x1f + iVar10, 4);
            __builtin_memcpy(&iVar7, obj + 0xa9, 4);
            if (iVar7[0x1eb] == 0) {
                iVar14 = uVar8 * 0x126;
                INC16(DAT_009a20f8 + iVar14);
                bVar18 = (DAT_009d8c80[uVar8 * 0x566] == (char)iVar7[0xb0]);
                if (bVar18) {
                    DAT_009a2119[iVar14] = 1;
                }
                FUN_00435850(0x2e, CONCAT22(uStack_8, uVar8), (unsigned int)bVar18);
            } else {
                bVar5 = obj[0x89];
                uVar9 = uVar8;
                uStack_8 = (unsigned short)bVar5;
                iVar14 = uVar9 * 0x126;
                DAT_009a2146[(unsigned int)uStack_8 + iVar14] =
                    (unsigned char)(DAT_009a2146[uVar9 * 0x126 + (unsigned int)uStack_8] + 1);
                iVar1 = uVar9 * 0x566;
                bVar18 = (DAT_009d8c80[iVar1] == (char)iVar7[0xb0]);
                if (bVar18) {
                    DAT_009a2150[(unsigned int)uStack_8 + iVar14] =
                        (unsigned char)(DAT_009a2150[(unsigned int)uStack_8 + iVar14] + 1);
                }
                FUN_00435930(0x2c, (((unsigned int)bVar5) << 16) | (unsigned int)(unsigned short)uVar8,
                             (unsigned int)bVar18, (unsigned int)bVar5);
                uVar11 = 0;
                do {
                    if ((uVar11 != (unsigned int)uStack_8) && ((unsigned int)DAT_009d8c88[iVar1] != uVar11)) {
                        DAT_009a215a[uVar11 + iVar14] = (unsigned char)(DAT_009a215a[uVar11 + iVar14] + 1);
                        if (DAT_009d8c80[iVar1] == (char)iVar7[0xb0]) {
                            DAT_009a2164[uVar11 + iVar14] = 1;
                        }
                    }
                    uVar11 = uVar11 + 1;
                } while (uVar11 < 10);
                cVar3 = DAT_009d8c80[iVar1];
                INC16(DAT_009a20fa + uVar9 * 0x126);
                if (cVar3 == (char)iVar7[0xb0]) {
                    DAT_009a211a[uVar9 * 0x126] = 1;
                }
            }
            DAT_009a1fe8[3 + iVar10] = 0;
            uVar9 = FUN_00458780((int)iVar7);
        }
    }

LAB_004672ed:
    return uVar9 & 0xffff0000;
}

/* SPDX-License-Identifier: MIT */
/* z_bop 004015b0 */
/* UNRESOLVED: __ftol x87 operands (FPU stack inputs) were not recoverable.
 * The calls keep Ghidra's operand-less form. Callee signatures are taken
 * from Ghidra; calling conventions are fixed from the stack-check findings
 * (callees pop 0 bytes, so __cdecl). */

extern unsigned char  DAT_009d6941;
extern unsigned short DAT_009ec5e4;
extern unsigned short DAT_00a07cec;
extern int            DAT_009d1308;
extern int            DAT_00a07ce8;
extern unsigned char *DAT_009a1fe8;
extern unsigned char *DAT_00a08104;
extern unsigned char *DAT_00a90a60;
extern unsigned short _DAT_00a90a5e;
extern int            DAT_00a07cdc;
extern int            DAT_00a07cd8;
extern int            DAT_00a07cd4;
extern unsigned char  DAT_00521f26[];
extern unsigned int   DAT_00a00870[];
extern int            DAT_00a00874[];
extern unsigned char  DAT_009d1320[];
extern char           DAT_0051a1d0[];
extern char           s_stationaryldrpln_0051a1bc[];
extern char           s_stationaryflwpln_0051a1a8[];
extern unsigned char  DAT_0051a0b0[];
extern unsigned char  DAT_0051c5b9[];

void __stdcall FUN_0042c0a0(void);
int  __cdecl FUN_004adc40(unsigned int a);
int  __cdecl FUN_004ae2a0(unsigned int a, unsigned int b);
void __cdecl FUN_004af000(unsigned int a, unsigned int b);
void __cdecl FUN_00458e20(unsigned int a, int b);
void __cdecl FUN_00427bd0(short a, short b, unsigned char *c);
void __cdecl FUN_00427d30(short a, int b, unsigned char *c);
long long __ftol(void);
unsigned int __cdecl FUN_0046abf0(unsigned int a, unsigned int b);
unsigned short __stdcall FUN_00425cb0(void);
void __cdecl FUN_004021a0(unsigned int a);
int  __cdecl FUN_004af5b0(unsigned int a);
void __cdecl FUN_004a7480(unsigned int a, int b);
void __cdecl FUN_00458fa0(unsigned int a, int b);
void __cdecl FUN_004592d0(unsigned int a);
int  __cdecl FUN_004ae6e0(int a, unsigned int b);
void __cdecl FUN_00450650(int a, int b);
void __cdecl FUN_00459480(unsigned int a);

void __stdcall FUN_004015b0(void)
{
    unsigned short *puVar2;
    unsigned char bVar3;
    unsigned char bVar4;
    int iVar5;
    long long lVar6;
    long long lVar7;
    long long lVar8;
    unsigned int uVar9;
    unsigned int uVar10;
    _Bool bVar11;
    char cVar12;
    unsigned short uVar13;
    unsigned short uVar14;
    unsigned short uVar15;
    unsigned int uVar16;
    int iVar17;
    unsigned char *pbVar18;
    unsigned char bVar19;
    int iVar20;
    unsigned int uVar21;
    unsigned int uVar22;
    unsigned short uVar23;
    char *pcVar24;
    char *pcVar25;
    char *pcVar26;
    _Bool bVar27;
    long long lVar28;
    long long lVar29;
    long long lVar30;
    long long lVar31;
    long long lVar32;
    long long lVar33;
    long long lVar34;
    int iVar35;
    unsigned short uStack_bc;
    int iStack_a8;
    int iStack_a0;
    int iStack_9c;
    /* scratch locals for packed (unaligned) accesses; stand in for
     * Ghidra's inline pointer expressions on iVar20/iVar35 */
    unsigned char *pbVar50;
    unsigned char *pbVar51;
    unsigned char *pbVar52;
    short sVar53;
    int iVar54;
    short sVar55[9];
    short sVar56;
    short sVar57;
    short sVar58;

    if (DAT_009d6941 != 0) {
        FUN_0042c0a0();
    }
    if (DAT_009ec5e4 == 0) {
        DAT_009ec5e4 = 0x1d;
        uStack_bc = DAT_00a07cec;
        if ((int)(unsigned int)DAT_00a07cec < DAT_009d1308 + DAT_00a07ce8) {
            do {
                uVar9 = (unsigned int)uStack_bc;
                iVar35 = (int)(uVar9 * 0x23);
                bVar3 = *(DAT_009a1fe8 + iVar35 + 3);
                if ((99 < bVar3) && (bVar3 < 0x6a)) {
                    iVar20 = (int)((unsigned int)((int)(uVar9 - DAT_009d1308) >> 4) * 0x1d);
                    __builtin_memcpy(&sVar53, DAT_009a1fe8 + iVar35 + 0x14, 2);
                    sVar53 = (short)((unsigned short)sVar53 +
                                     (unsigned short)(short)((int)((unsigned int)iVar20 +
                                                                   ((unsigned int)(iVar20 >> 0x1f) & 0xfU)) >> 4));
                    __builtin_memcpy(DAT_009a1fe8 + iVar35 + 0x14, &sVar53, 2);

                    iVar20 = (int)((unsigned int)((int)(uVar9 - DAT_009d1308) >> 3) * 0x1d);
                    __builtin_memcpy(&sVar53, DAT_009a1fe8 + 0x12 + iVar35, 2);
                    sVar53 = (short)((unsigned short)sVar53 +
                                     (unsigned short)(short)((int)((unsigned int)iVar20 +
                                                                   ((unsigned int)(iVar20 >> 0x1f) & 0x1fU)) >> 5));
                    __builtin_memcpy(DAT_009a1fe8 + 0x12 + iVar35, &sVar53, 2);

                    iVar20 = (int)((unsigned int)(4 - ((int)(uVar9 - DAT_009d1308) >> 4)) * 0x1d);
                    __builtin_memcpy(&sVar53, DAT_009a1fe8 + 0x10 + iVar35, 2);
                    sVar53 = (short)((unsigned short)sVar53 +
                                     (unsigned short)(short)((int)((unsigned int)iVar20 +
                                                                   ((unsigned int)(iVar20 >> 0x1f) & 0xfU)) >> 4));
                    __builtin_memcpy(DAT_009a1fe8 + 0x10 + iVar35, &sVar53, 2);
                }
                pbVar50 = DAT_009a1fe8 + iVar35;
                __builtin_memcpy(&pbVar51, pbVar50 + 0x1f, 4);
                if (pbVar51 == 0) {
                    if (pbVar50[3] != 0) {
                        __builtin_memcpy(&DAT_00a90a60, &DAT_00521f26[(unsigned int)pbVar50[3] * 0x1a], 4);
                        if (DAT_00a90a60 != 0) {
LAB_00402123:
                            _DAT_00a90a5e = (unsigned short)pbVar50[0x19];
                            FUN_004021a0(uVar9);
                            *(DAT_009a1fe8 + 0x19 + iVar35) = *(unsigned char *)&_DAT_00a90a5e;
                        }
                    }
                } else {
                    bVar3 = pbVar50[3];
                    uVar15 = (unsigned short)bVar3;
                    if (uVar15 != 0) {
                        uVar10 = (unsigned int)uVar15;
                        __builtin_memcpy(&DAT_00a90a60, &DAT_00521f26[uVar10 * 0x1a], 4);
                        __builtin_memcpy(&iVar5, pbVar51 + 0xa9, 4);
                        switch (pbVar50[2]) {
                        case 0:
                        case 1:
                        case 2:
                        case 3:
                        case 4:
                        case 5:
                            if (uVar15 < 0x49) {
                                uVar13 = *(unsigned short *)&DAT_00a00870[uVar10 * 0x65];
                            } else {
                                iVar20 = FUN_004adc40(uVar10);
                                uVar13 = (unsigned short)iVar20;
                            }
                            bVar11 = 0;
                            __builtin_memcpy(&pbVar52, DAT_009a1fe8 + 0x1f + iVar35, 4);
                            __builtin_memcpy(&DAT_00a08104, pbVar52 + 0xa9, 4);
                            __builtin_memcpy(&sVar53, DAT_00a08104 + 0x1e5, 2);
                            if (sVar53 == 0) {
LAB_004017dd:
                                bVar27 = 0;
                            } else {
                                pcVar25 = (char *)(DAT_009d1320 + (unsigned int)*(DAT_00a08104 + 0x34) * 0x55);
                                iVar20 = 8;
                                bVar27 = 0;
                                pcVar24 = pcVar25;
                                pcVar26 = DAT_0051a1d0;
                                do {
                                    if (iVar20 == 0) break;
                                    iVar20 = iVar20 + -1;
                                    bVar27 = (*pcVar24 == *pcVar26);
                                    pcVar24 = pcVar24 + 1;
                                    pcVar26 = pcVar26 + 1;
                                } while (bVar27);
                                if (bVar27) goto LAB_004017dd;
                                iVar20 = 0x11;
                                bVar27 = 0;
                                pcVar24 = pcVar25;
                                pcVar26 = s_stationaryldrpln_0051a1bc;
                                do {
                                    if (iVar20 == 0) break;
                                    iVar20 = iVar20 + -1;
                                    bVar27 = (*pcVar24 == *pcVar26);
                                    pcVar24 = pcVar24 + 1;
                                    pcVar26 = pcVar26 + 1;
                                } while (bVar27);
                                if (bVar27) goto LAB_004017dd;
                                iVar20 = 0x11;
                                bVar27 = 0;
                                pcVar24 = s_stationaryflwpln_0051a1a8;
                                do {
                                    if (iVar20 == 0) break;
                                    iVar20 = iVar20 + -1;
                                    bVar27 = (*pcVar25 == *pcVar24);
                                    pcVar25 = pcVar25 + 1;
                                    pcVar24 = pcVar24 + 1;
                                } while (bVar27);
                                if (bVar27) goto LAB_004017dd;
                                bVar27 = 1;
                            }
                            if (bVar27 && (iStack_a0 = 0, *(DAT_00a08104 + 0x21d) != 0)) {
                                iStack_9c = 0;
                                iStack_a8 = 0;
                                do {
                                    if ((*(DAT_00a08104 + 0x370 + iStack_a8) == 2) &&
                                        (uVar22 = (unsigned int)DAT_0051c5b9[iStack_9c + (unsigned int)*(DAT_00a08104 + 4) * 0x101],
                                         *(DAT_00a08104 + uVar22 + 0x33c) != 0)) {
                                        if (uVar15 < 0x49) {
                                            uVar21 = uVar22;
                                            if ((int)DAT_00a00870[uVar10 * 0x65] <= (int)uVar22) {
                                                uVar21 = DAT_00a00870[uVar10 * 0x65] - 1;
                                            }
                                            iVar20 = DAT_00a00874[uVar21 + uVar10 * 0x65];
                                        } else {
                                            iVar20 = FUN_004ae2a0(uVar10, uVar22);
                                        }
                                        if (iVar20 == 0x15) {
                                            puVar2 = (unsigned short *)(DAT_00a08104 + 0x3b2 + iStack_a8);
                                            if (*puVar2 == 0xffff) {
                                                bVar19 = *(DAT_00a08104 + uVar22 + 0x30a);
                                                if ((bVar19 & 1) == 0) {
                                                    cVar12 = (char)(bVar19 - 4);
                                                } else {
                                                    cVar12 = (char)(bVar19 + 4);
                                                }
                                                *(DAT_00a08104 + uVar22 + 0x30a) = (unsigned char)cVar12;
                                                uVar23 = FUN_00425cb0();
                                                if (uVar23 < 0x600) {
                                                    pbVar18 = DAT_00a08104 + uVar22 + 0x30a;
                                                    *pbVar18 = *pbVar18 ^ 1;
                                                }
                                            } else {
                                                FUN_004af000(uVar10, uVar22);
                                                pbVar50 = DAT_009a1fe8 + iVar35;
                                                FUN_00458e20((unsigned int)*puVar2, 0);
                                                __builtin_memcpy(&iVar54, pbVar50 + 4, 4);
                                                DAT_00a07cdc = (int)((unsigned int)DAT_00a07cdc - (unsigned int)iVar54);
                                                __builtin_memcpy(&iVar54, pbVar50 + 8, 4);
                                                DAT_00a07cd8 = (int)((unsigned int)DAT_00a07cd8 - (unsigned int)iVar54);
                                                __builtin_memcpy(&iVar54, pbVar50 + 0xc, 4);
                                                DAT_00a07cd4 = (int)((unsigned int)DAT_00a07cd4 - (unsigned int)iVar54);
                                                __builtin_memcpy(&pbVar51, pbVar50 + 0x1f, 4);
                                                if (*(pbVar51 + 0x92) != 0) {
                                                    __builtin_memcpy(&sVar56, pbVar50 + 0x12, 2);
                                                    __builtin_memcpy(&sVar57, pbVar50 + 0x10, 2);
                                                    FUN_00427bd0(sVar56, sVar57, pbVar50);
                                                    __builtin_memcpy(&sVar58, pbVar50 + 0x14, 2);
                                                    FUN_00427d30(sVar58, 0, pbVar50);
                                                }
                                                __builtin_memcpy(&pbVar51, pbVar50 + 0x1f, 4);
                                                __builtin_memcpy(sVar55, pbVar51 + 0x93, 18);
                                                lVar29 = (long long)DAT_00a07cdc * (long long)(int)sVar55[3];
                                                lVar31 = (long long)DAT_00a07cd8 * (long long)(int)sVar55[4];
                                                lVar30 = (long long)DAT_00a07cd4 * (long long)(int)sVar55[5];
                                                lVar34 = (long long)DAT_00a07cdc * (long long)(int)sVar55[0];
                                                lVar33 = (long long)DAT_00a07cd8 * (long long)(int)sVar55[1];
                                                lVar32 = (long long)DAT_00a07cd4 * (long long)(int)sVar55[2];
                                                lVar6 = (long long)DAT_00a07cdc * (long long)(int)sVar55[6];
                                                lVar7 = (long long)DAT_00a07cd8 * (long long)(int)sVar55[7];
                                                lVar8 = (long long)DAT_00a07cd4 * (long long)(int)sVar55[8];
                                                lVar28 = __ftol();
                                                DAT_00a07cdc = (int)((((unsigned int)lVar30 >> 0xf |
                                                                       (unsigned int)((unsigned long long)lVar30 >> 0x20) << 0x11) +
                                                                      ((unsigned int)lVar29 >> 0xf |
                                                                       (unsigned int)((unsigned long long)lVar29 >> 0x20) << 0x11) +
                                                                      ((unsigned int)lVar31 >> 0xf |
                                                                       (unsigned int)((unsigned long long)lVar31 >> 0x20) << 0x11)) -
                                                                     (unsigned int)(int)lVar28);
                                                lVar29 = __ftol();
                                                DAT_00a07cd8 = (int)((0U - (unsigned int)(int)lVar29) -
                                                                     (((unsigned int)lVar32 >> 0xf |
                                                                       (unsigned int)((unsigned long long)lVar32 >> 0x20) << 0x11) +
                                                                      ((unsigned int)lVar34 >> 0xf |
                                                                       (unsigned int)((unsigned long long)lVar34 >> 0x20) << 0x11) +
                                                                      ((unsigned int)lVar33 >> 0xf |
                                                                       (unsigned int)((unsigned long long)lVar33 >> 0x20) << 0x11)));
                                                lVar29 = __ftol();
                                                DAT_00a07cd4 = (int)((((unsigned int)lVar8 >> 0xf |
                                                                       (unsigned int)((unsigned long long)lVar8 >> 0x20) << 0x11) +
                                                                      ((unsigned int)lVar6 >> 0xf |
                                                                       (unsigned int)((unsigned long long)lVar6 >> 0x20) << 0x11) +
                                                                      ((unsigned int)lVar7 >> 0xf |
                                                                       (unsigned int)((unsigned long long)lVar7 >> 0x20) << 0x11)) -
                                                                     (unsigned int)(int)lVar29);
                                                __ftol();
                                                __ftol();
                                                __ftol();
                                                lVar30 = __ftol();
                                                lVar31 = __ftol();
                                                lVar29 = __ftol();
                                                lVar29 = (long long)DAT_00a07cdc * (long long)(int)lVar29;
                                                lVar31 = (long long)DAT_00a07cd8 * (long long)(int)lVar31;
                                                lVar30 = (long long)DAT_00a07cd4 * (long long)(int)lVar30;
                                                lVar32 = __ftol();
                                                lVar33 = __ftol();
                                                lVar34 = __ftol();
                                                lVar34 = (long long)DAT_00a07cdc * (long long)(int)lVar34;
                                                lVar33 = (long long)DAT_00a07cd8 * (long long)(int)lVar33;
                                                lVar32 = (long long)DAT_00a07cd4 * (long long)(int)lVar32;
                                                uVar16 = FUN_0046abf0(((unsigned int)lVar32 >> 0xf |
                                                                       (unsigned int)((unsigned long long)lVar32 >> 0x20) << 0x11) +
                                                                      ((unsigned int)lVar34 >> 0xf |
                                                                       (unsigned int)((unsigned long long)lVar34 >> 0x20) << 0x11) +
                                                                      ((unsigned int)lVar33 >> 0xf |
                                                                       (unsigned int)((unsigned long long)lVar33 >> 0x20) << 0x11),
                                                                      ((unsigned int)lVar30 >> 0xf |
                                                                       (unsigned int)((unsigned long long)lVar30 >> 0x20) << 0x11) +
                                                                      ((unsigned int)lVar29 >> 0xf |
                                                                       (unsigned int)((unsigned long long)lVar29 >> 0x20) << 0x11) +
                                                                      ((unsigned int)lVar31 >> 0xf |
                                                                       (unsigned int)((unsigned long long)lVar31 >> 0x20) << 0x11));
                                                *(DAT_00a08104 + uVar22 + 0x30a) = (unsigned char)(uVar16 >> 8);
                                            }
                                        }
                                    }
                                    iStack_9c = iStack_9c + 8;
                                    iStack_a8 = iStack_a8 + 4;
                                    iStack_a0 = iStack_a0 + 1;
                                } while (iStack_a0 < (int)(unsigned int)*(DAT_00a08104 + 0x21d));
                            }
                            uVar23 = 0;
                            if (uVar13 != 0) {
                                do {
                                    uVar22 = (unsigned int)uVar23;
                                    if (uVar15 < 0x49) {
                                        if ((int)DAT_00a00870[uVar10 * 0x65] <= (int)uVar22) {
                                            uVar22 = DAT_00a00870[uVar10 * 0x65] - 1;
                                        }
                                        iVar20 = DAT_00a00874[uVar22 + uVar10 * 0x65];
                                    } else {
                                        iVar20 = FUN_004ae2a0(uVar10, uVar22);
                                    }
                                    if (iVar20 == 3) {
                                        pbVar18 = (unsigned char *)(unsigned int)((unsigned int)uVar13 + 0x2d8 + (unsigned int)iVar5);
                                        DAT_00a90a60 = DAT_0051a0b0;
                                        _DAT_00a90a5e = (unsigned short)*pbVar18;
                                        FUN_004021a0(uVar9);
                                        *pbVar18 = *(unsigned char *)&_DAT_00a90a5e;
                                    }
                                    if (*(DAT_00a08104 + 7) == 3) {
                                        iVar17 = FUN_004af5b0(uVar10);
                                        if (iVar17 == 0) {
                                            uVar23 = (unsigned short)(uVar23 + 3);
                                            FUN_004a7480(((unsigned int)bVar3 << 16) | uStack_bc, 0);
                                        } else {
                                            FUN_00458fa0(((unsigned int)bVar3 << 16) | uStack_bc, 0);
                                            uVar14 = FUN_00425cb0();
                                            if (uVar14 < 0x1800) {
                                                FUN_004592d0(((unsigned int)bVar3 << 16) | uStack_bc);
                                            }
                                        }
                                    }
                                    if (bVar27 &&
                                        ((((iVar20 == 0xb || iVar20 == 0x17) || iVar20 == 0xc) ||
                                          (((iVar20 == 0x18 || iVar20 == 0xd) || iVar20 == 0x19))))) {
                                        uVar22 = (unsigned int)uVar23;
                                        bVar19 = *(DAT_00a08104 + uVar22 + 0x30a);
                                        if ((bVar19 & 1) == 0) {
                                            cVar12 = (char)(bVar19 - 4);
                                        } else {
                                            cVar12 = (char)(bVar19 + 4);
                                        }
                                        *(DAT_00a08104 + uVar22 + 0x30a) = (unsigned char)cVar12;
                                        uVar14 = FUN_00425cb0();
                                        if (uVar14 < 0x200) {
                                            pbVar18 = DAT_00a08104 + uVar22 + 0x30a;
                                            *pbVar18 = *pbVar18 ^ 1;
                                        }
                                    }
                                    if (((iVar20 == 7) && (uVar15 == 4)) &&
                                        ((*(DAT_00a08104 + 0x29) & 1) != 0)) {
                                        if ((*(DAT_00a08104 + 0x29) & 2) == 0) {
                                            uVar22 = (unsigned int)uVar23;
                                            cVar12 = (char)*(DAT_00a08104 + uVar22 + 0x30a);
                                            if (cVar12 != 0) {
                                                *(DAT_00a08104 + uVar22 + 0x30a) = (unsigned char)(cVar12 + -4);
                                                if (0x80 < *(DAT_00a08104 + uVar22 + 0x30a)) {
                                                    *(DAT_00a08104 + uVar22 + 0x30a) = 0;
                                                }
                                            }
                                        } else {
                                            pbVar18 = DAT_00a08104 + uVar23 + 0x30a;
                                            bVar19 = *pbVar18;
                                            if (bVar19 < 0x40) {
                                                *pbVar18 = (unsigned char)(bVar19 + 4);
                                            }
                                        }
                                    }
                                    if ((iVar20 == 0x14) && ((*(DAT_00a08104 + 0x29) & 1) != 0)) {
                                        if ((*(DAT_00a08104 + 0x29) & 2) == 0) {
                                            if (uVar15 == 1) {
                                                pbVar18 = DAT_00a08104 + uVar23 + 0x30a;
                                                if (*pbVar18 != 0) {
                                                    bVar19 = (unsigned char)(*pbVar18 - 1);
LAB_0040200c:
                                                    *pbVar18 = bVar19;
                                                    goto LAB_0040200e;
                                                }
                                            } else if (uVar15 == 4) {
                                                pbVar18 = DAT_00a08104 + uVar23 + 0x30a;
                                                if (*pbVar18 != 0) {
                                                    bVar19 = (unsigned char)(*pbVar18 - 4);
                                                    goto LAB_0040200c;
                                                }
                                            }
                                        } else if (uVar15 == 1) {
                                            iVar20 = FUN_004ae6e0(1, (unsigned int)uVar23);
                                            bVar19 = 8;
                                            if (-1 < iVar20) {
                                                bVar19 = 0xc;
                                            }
                                            pbVar18 = DAT_00a08104 + uVar23 + 0x30a;
                                            bVar4 = *pbVar18;
                                            if (bVar4 < bVar19) {
                                                *pbVar18 = (unsigned char)(bVar4 + 1);
LAB_0040200e:
                                                bVar11 = 1;
                                            }
                                        } else if (uVar15 == 4) {
                                            pbVar18 = DAT_00a08104 + uVar23 + 0x30a;
                                            if (*pbVar18 < 0x40) {
                                                bVar19 = (unsigned char)(*pbVar18 + 4);
                                                goto LAB_0040200c;
                                            }
                                        }
                                    }
                                    uVar23 = (unsigned short)(uVar23 + 1);
                                } while (uVar23 < uVar13);
                            }
                            if ((*(DAT_00a08104 + 0x29) & 1) != 0) {
                                if ((*(DAT_00a08104 + 0x29) & 2) == 0) {
                                    if (!bVar11) {
                                        *(DAT_00a08104 + 0x29) = 0;
                                        __builtin_memcpy(&iVar35, DAT_009a1fe8 + 0x1b + iVar35, 4);
                                        iVar20 = 0x80;
                                        goto LAB_00402075;
                                    }
                                } else if (!bVar11) {
                                    *(DAT_00a08104 + 0x29) = 2;
                                    __builtin_memcpy(&iVar35, DAT_009a1fe8 + 0x1b + iVar35, 4);
                                    iVar20 = 0x81;
LAB_00402075:
                                    FUN_00450650(iVar20, iVar35);
                                }
                            }
                            if ((*(DAT_00a08104 + 7) == 0) &&
                                (*(DAT_00a08104 + 0x246) == 1)) {
                                __builtin_memcpy(&sVar53, DAT_00a08104 + 0x248, 2);
                                if (sVar53 != 0) {
                                    FUN_00459480(((unsigned int)bVar3 << 16) | uStack_bc);
                                    FUN_00459480(((unsigned int)bVar3 << 16) | uStack_bc);
                                    FUN_00459480(((unsigned int)bVar3 << 16) | uStack_bc);
                                }
                            }
                            break;
                        case 0xb:
                        case 0xd:
                            if (pbVar50[3] != 'Y') goto LAB_00402123;
                            pbVar50[0x1a] = 0;
                            uVar15 = FUN_00425cb0();
                            if (uVar15 < 0x800) {
                                FUN_004592d0(((unsigned int)bVar3 << 16) | uStack_bc);
                            }
                            break;
                        }
                    }
                }
                uStack_bc = (unsigned short)(uStack_bc + 1);
            } while ((int)(unsigned int)uStack_bc < DAT_009d1308 + DAT_00a07ce8);
        }
    }
    return;
}

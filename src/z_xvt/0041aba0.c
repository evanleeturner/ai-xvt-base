/* SPDX-License-Identifier: MIT */
/* z_xvt 0041aba0 */
extern unsigned char *DAT_00732e54;
extern unsigned char *DAT_006d4988;
extern unsigned char *DAT_006db798;
extern unsigned short DAT_0071e6be;
extern unsigned short _DAT_0071f2de;
extern unsigned short DAT_0070a1d2;
extern short DAT_00732e40;
extern short DAT_006da9f0;
extern short DAT_0071b704;
extern int DAT_0070ab18;
extern int DAT_0070ab14;
extern int DAT_0070ac20;
extern int DAT_00732a2c;
extern int DAT_00732a28;
extern int DAT_00732a24;
extern int DAT_006d4974;
extern int DAT_006d4978;
extern int DAT_006d4970;
extern int DAT_0071e6c4;
extern int DAT_00732f38;
extern int DAT_00732fe8;
extern int DAT_006db580;
extern int DAT_006db720;
extern int DAT_006db704;
extern int DAT_006db708;
extern int DAT_0071ecf4;
extern unsigned char DAT_0052b0a8[];
extern unsigned char DAT_0052b0aa[];
extern unsigned char DAT_0052b0ac[];
extern unsigned char DAT_0052b0bc[];
extern unsigned char DAT_0052b0be[];
extern unsigned char DAT_0052b0c0[];
extern unsigned char DAT_0052b0c2[];
extern unsigned char DAT_0052b0c3[];
extern unsigned char DAT_00525aae[];
extern unsigned char DAT_00525a7e[];
extern unsigned char s_starshipintohyperpln_00525b38[];
/* Register leftovers (Ghidra extraout_*) */
extern int extraout_ECX;
extern int extraout_EDX;
extern int extraout_EDX_00;

unsigned int __cdecl FUN_0042b250(unsigned char *param_1);
int __cdecl FUN_0044aa70(unsigned int param_1, unsigned int param_2, unsigned int param_3);
int __cdecl FUN_0044ab00(unsigned int param_1, unsigned int param_2, unsigned int param_3);
int __cdecl FUN_0044ab90(unsigned int param_1, unsigned int param_2, unsigned int param_3);
int __cdecl FUN_00449790(unsigned int param_1, unsigned int param_2);
void __cdecl FUN_00483950(unsigned short param_1, unsigned int param_2, unsigned int param_3, int param_4, int param_5, int param_6);
/* Original: param_1/param_2 arrive in ECX/EDX; callee ends RET 0, caller cleans 16 stack bytes. */
void __cdecl FUN_0042a230(unsigned char *param_3, short param_4, short param_5, short param_6);
void __cdecl FUN_00421c70(unsigned int param_1, int param_2);
unsigned int __cdecl FUN_00412810(unsigned int param_1, unsigned int param_2, unsigned int param_3);
int __cdecl FUN_00419190(unsigned short param_1, unsigned short param_2);
void __cdecl FUN_00418cd0(unsigned int param_1, unsigned int param_2, unsigned int param_3);
unsigned short __cdecl FUN_00478470(void);
unsigned int __cdecl FUN_00478320(unsigned int param_1, unsigned short param_2);
unsigned short __cdecl FUN_00422710(int param_1);
short __cdecl FUN_00446fe0(unsigned int param_1);
void __cdecl FUN_00483410(short param_1, short param_2, unsigned char *param_3);
/* Original: param_1/param_2 arrive in ECX/EDX; callee ends RET 0, caller cleans 8 stack bytes (param_3, param_4). */
long long __cdecl FUN_00413ca0(int param_3, int param_4);
void __cdecl FUN_0040a3c0(unsigned int param_1, int param_2);

void __cdecl FUN_0041aba0(unsigned short param_1, unsigned int param_2, unsigned short param_3)
{
    unsigned char *pbVar1;
    unsigned char bVar2;
    unsigned char bVar3;
    char cVar4;
    int iVar5;
    int iVar6;
    int bVar7;
    short sVar8;
    unsigned short uVar9;
    unsigned short uVar10;
    short sVar11;
    unsigned int uVar12;
    unsigned char *iVar13;
    unsigned int uVar14;
    int iVar15;
    int iVar17;
    int iVar18;
    unsigned int uVar19;
    unsigned int uVar20;
    unsigned int uVar21;
    unsigned int iVar22;
    unsigned char bVar23;
    unsigned char bVar24;
    int iVar25;
    int iVar26;
    short sVar27;
    int iVar28;
    long long uVar29;
    short s_1df;
    unsigned char *pNew;
    unsigned char *pSub;
    unsigned char *pSrcSub;
    unsigned char *pSlot;
    int iTmp;
    short sTmp;

    __builtin_memcpy(&s_1df, DAT_00732e54 + 0x1df, 2);
    if (s_1df != 0) {
        uVar12 = param_2 & 0xffff;
        iVar5 = (unsigned int)DAT_00732e54[4] * 0xef;
        bVar24 = DAT_0052b0c3[uVar12 * 8 + iVar5];
        bVar2 = DAT_0052b0c2[uVar12 * 8 + iVar5];
        if (DAT_00732e54[0x31a + (unsigned int)bVar24] != 0) {
            iVar13 = (unsigned char *)((unsigned int)param_1 * 0x23 + (unsigned int)DAT_006d4988);
            uVar14 = FUN_0042b250(DAT_00732e54);
            uVar10 = (unsigned short)uVar14;
            bVar3 = DAT_00732e54[0x345 + uVar12 * 4];
            pbVar1 = DAT_00732e54 + 0x345 + uVar12 * 4;
            bVar23 = bVar3 & 0x7f;
            if (bVar23 != 0) {
                __builtin_memcpy(&uVar14, DAT_00732e54 + 0x1d, 4);
                if (uVar14 == 0) {
                    if (uVar10 < 0xaaaa) {
                        if (uVar10 < 0x5555) {
                            if (uVar10 < 0x4000) {
                                bVar24 = (unsigned char)(DAT_0071e6be >> 3);
                            } else {
                                bVar24 = (unsigned char)(DAT_0071e6be / 6);
                            }
                        } else {
                            bVar24 = (unsigned char)(DAT_0071e6be >> 2);
                        }
                    } else {
                        bVar24 = (unsigned char)(DAT_0071e6be >> 1);
                    }
                } else {
                    if (0x27fff < uVar14) {
                        return;
                    }
                    if (uVar14 < 0x18000) {
                        if (0x7fff < uVar14) {
                            if (_DAT_0071f2de < 0x76) {
                                return;
                            }
                            bVar24 = 2;
                        }
                    } else {
                        if (_DAT_0071f2de < 0x76) {
                            return;
                        }
                        bVar24 = 1;
                    }
                }
                if (bVar24 == 0) {
                    bVar24 = 1;
                }
                bVar24 = (unsigned char)(bVar23 - bVar24);
                if (bVar23 < bVar24) {
                    bVar24 = 0;
                }
                *pbVar1 = bVar3 & 0x80;
                DAT_00732e54[0x345 + uVar12 * 4] = DAT_00732e54[0x345 + uVar12 * 4] | bVar24;
                return;
            }
            *pbVar1 = bVar3 & 0x80;
            DAT_00732e54[0x345 + uVar12 * 4] = DAT_00732e54[0x345 + uVar12 * 4] | 0x3b;
            __builtin_memcpy(&iVar18, iVar13 + 4, 4);
            __builtin_memcpy(&iVar26, iVar13 + 8, 4);
            __builtin_memcpy(&iVar28, iVar13 + 0xc, 4);
            if ((bVar2 == 0xff) || (0x75 < _DAT_0071f2de)) {
                __builtin_memcpy(&sVar27, &DAT_0052b0bc[uVar12 * 8 + iVar5], 2);
                __builtin_memcpy(&sVar8, &DAT_0052b0be[uVar12 * 8 + iVar5], 2);
                __builtin_memcpy(&sVar11, &DAT_0052b0c0[uVar12 * 8 + iVar5], 2);
            } else if (iVar13[3] == 0x35) {
                uVar14 = (unsigned int)bVar2;
                iVar15 = FUN_0044aa70(0x35, (unsigned int)bVar24, uVar14);
                sVar27 = (short)(iVar15 >> 1);
                iVar15 = FUN_0044ab00((unsigned int)iVar13[3], (unsigned int)bVar24, uVar14);
                sVar11 = (short)(iVar15 >> 1);
                iVar15 = FUN_0044ab90((unsigned int)iVar13[3], (unsigned int)bVar24, uVar14);
                sVar8 = (short)(iVar15 >> 1);
            } else {
                uVar14 = (unsigned int)bVar2;
                iVar15 = FUN_0044aa70((unsigned int)iVar13[3], (unsigned int)bVar24, uVar14);
                sVar27 = (short)iVar15;
                iVar15 = FUN_0044ab00((unsigned int)iVar13[3], (unsigned int)bVar24, uVar14);
                sVar11 = (short)iVar15;
                iVar15 = FUN_0044ab90((unsigned int)iVar13[3], (unsigned int)bVar24, uVar14);
                sVar8 = (short)iVar15;
            }
            iVar17 = FUN_00449790((unsigned int)iVar13[3], (unsigned int)bVar24);
            iVar15 = extraout_ECX;
            iVar25 = extraout_EDX;
            if (iVar17 == 0x15) {
                DAT_0070ab18 = (int)sVar27;
                DAT_0070ab14 = (int)sVar11;
                DAT_0070ac20 = (int)sVar8;
                if (iVar13[3] == '5') {
                    DAT_0070ab14 = (int)((unsigned int)DAT_0070ab14 * 2);
                    DAT_0070ab18 = (int)((unsigned int)DAT_0070ab18 * 2);
                    DAT_0070ac20 = (int)((unsigned int)DAT_0070ac20 * 2);
                }
                FUN_00483950((unsigned short)((unsigned short)DAT_00732e54[0x2f2 + (unsigned int)bVar24] << 8),
                             (unsigned int)iVar13[3], (unsigned int)bVar24,
                             DAT_0070ab18, DAT_0070ab14, DAT_0070ac20);
                iVar15 = (int)iVar13;
                iVar25 = extraout_EDX_00;
                if (iVar13[3] == '5') {
                    DAT_0070ab18 = DAT_0070ab18 >> 1;
                    iVar15 = DAT_0070ab14 >> 1;
                    iVar25 = DAT_0070ac20 >> 1;
                    DAT_0070ab14 = iVar15;
                    DAT_0070ac20 = iVar25;
                }
                sVar27 = (short)DAT_0070ab18;
                sVar11 = (short)DAT_0070ab14;
                sVar8 = (short)DAT_0070ac20;
            }
            /* ECX = iVar15, EDX = iVar25 in the original (register args, not expressible here) */
            FUN_0042a230(iVar13, sVar27, sVar8, sVar11);
            if (iVar13[3] == '5') {
                DAT_0070ab18 = (int)((unsigned int)DAT_0070ab18 * 2);
                DAT_0070ab14 = (int)((unsigned int)DAT_0070ab14 * 2);
                DAT_0070ac20 = (int)((unsigned int)DAT_0070ac20 * 2);
            }
            iVar18 = (int)((unsigned int)iVar18 + (unsigned int)DAT_0070ab18);
            iVar26 = (int)((unsigned int)iVar26 + (unsigned int)DAT_0070ab14);
            iVar28 = (int)((unsigned int)iVar28 + (unsigned int)DAT_0070ac20);
            FUN_00421c70((unsigned int)param_3, 0);
            iVar17 = DAT_00732a2c;
            iVar25 = DAT_00732a28;
            iVar15 = DAT_00732a24;
            uVar14 = (unsigned int)DAT_00732a2c - (unsigned int)iVar18;
            uVar19 = (unsigned int)DAT_00732a28 - (unsigned int)iVar26;
            uVar20 = (unsigned int)DAT_00732a24 - (unsigned int)iVar28;
            uVar21 = FUN_00412810(uVar14, uVar19, uVar20);
            if (uVar21 < 0x14001) {
                DAT_006d4974 = iVar17;
                DAT_006d4978 = iVar25;
                DAT_006d4970 = iVar15;
                DAT_0071e6c4 = iVar18;
                DAT_00732f38 = iVar26;
                DAT_00732fe8 = iVar28;
                if (FUN_00419190(param_1, param_1) == 0) {
                    cVar4 = (char)DAT_00732e54[0x346 + uVar12 * 4];
                    iVar6 = (int)((unsigned int)param_3 * 0x23);
                    __builtin_memcpy(&iVar22, DAT_006d4988 + 0x1f + iVar6, 4);
                    if (iVar22 != 0) {
                        FUN_00418cd0(uVar14, uVar19, uVar20);
                        if (cVar4 == '\0') {
                            DAT_006db580 = (int)((unsigned int)DAT_0070a1d2 * (unsigned int)DAT_006db580) >> 0xe;
                        } else {
                            DAT_006db580 = (int)((unsigned int)DAT_0070a1d2 * (unsigned int)DAT_006db580) >> 0xf;
                        }
                        sVar27 = (short)DAT_006db580;
                        uVar9 = FUN_00478470();
                        sVar27 = (short)(sVar27 + (uVar9 & 3) + -1);
                        __builtin_memcpy(&pSub, DAT_006d4988 + 0x1f + iVar6, 4);
                        __builtin_memcpy(&sTmp, pSub + 0x79, 2);
                        if (sTmp == 0) {
                            sVar27 = 0;
                        }
                        uVar14 = FUN_00478320((((unsigned int)(unsigned short)(iVar22 >> 0x10)) << 16)
                                              | (unsigned int)(unsigned short)sVar27, uVar10);
                        uVar14 = uVar14 & 0xffff;
                        __builtin_memcpy(&pSub, DAT_006d4988 + 0x1f + iVar6, 4);
                        __builtin_memcpy(&iTmp, pSub + 6, 4);
                        iVar17 = (int)((unsigned int)iVar17 + ((unsigned int)iVar17 - (unsigned int)iTmp) * uVar14);
                        __builtin_memcpy(&iTmp, pSub + 10, 4);
                        iVar25 = (int)((unsigned int)iVar25 + ((unsigned int)iVar25 - (unsigned int)iTmp) * uVar14);
                        __builtin_memcpy(&iTmp, pSub + 0xe, 4);
                        iVar15 = (int)((unsigned int)iVar15 + ((unsigned int)iVar15 - (unsigned int)iTmp) * uVar14);
                    }
                    FUN_00418cd0((unsigned int)iVar17 - (unsigned int)iVar18,
                                 (unsigned int)iVar25 - (unsigned int)iVar26,
                                 (unsigned int)iVar15 - (unsigned int)iVar28);
                    sVar8 = DAT_00732e40;
                    sVar27 = DAT_006da9f0;
                    FUN_00478470();
                    uVar10 = FUN_00422710(7);
                    if (uVar10 != 0xffff) {
                        iVar15 = (int)(((unsigned int)uVar10 * 8 - (unsigned int)uVar10) * 5);
                        pNew = DAT_006d4988 + iVar15;
                        __builtin_memcpy(&pSub, pNew + 0x1f, 4);
                        pSub[0] = 1;
                        pNew[2] = 7;
                        __builtin_memcpy(&pSrcSub, iVar13 + 0x1f, 4);
                        pSub[0x86] = pSrcSub[0x86];
                        uVar9 = 0;
                        bVar7 = 0;
                        do {
                            iVar25 = (int)((unsigned int)uVar9 + (unsigned int)iVar5);
                            if (((DAT_0052b0aa[iVar25] <= uVar12) &&
                                 (uVar12 <= DAT_0052b0ac[iVar25])) &&
                                (((signed char)DAT_0052b0a8[iVar25] == -0x6f) ||
                                 ((signed char)DAT_0052b0a8[iVar25] == -0x6e))) {
                                bVar7 = 1;
                            }
                            uVar9 = (unsigned short)(uVar9 + 1);
                        } while (uVar9 < 2);
                        if (cVar4 == '\0') {
                            __builtin_memcpy(&pSrcSub, iVar13 + 0x1f, 4);
                            cVar4 = (char)pSrcSub[0x86];
                            if ((cVar4 == '\0') || (cVar4 == '\x02')) {
                                uVar9 = (unsigned short)((((unsigned short)(-(unsigned short)(!bVar7))) & 0xfff9) + 0x91);
                            } else {
                                uVar9 = (unsigned short)((((unsigned short)(-(unsigned short)(!bVar7))) & 0xfffa) + 0x92);
                            }
                        } else {
                            uVar9 = (unsigned short)(0x8e - (!bVar7));
                        }
                        pNew[3] = (unsigned char)uVar9;
                        __builtin_memcpy(&pSub, pNew + 0x1f, 4);
                        sTmp = 1;
                        __builtin_memcpy(pSub + 0x81, &sTmp, 2);
                        __builtin_memcpy(pSub + 0x83, &param_1, 2);
                        pSub[0x85] = iVar13[3];
                        __builtin_memcpy(pNew + 0x12, &sVar8, 2);
                        sTmp = 0;
                        __builtin_memcpy(pNew + 0x14, &sTmp, 2);
                        __builtin_memcpy(pNew + 0x10, &sVar27, 2);
                        uVar12 = (unsigned int)uVar9;
                        pSub[0x90] = 1;
                        __builtin_memcpy(&pSub, pNew + 0x1f, 4);
                        pSub[0x89] = pSub[0x90];
                        __builtin_memcpy(&pSub, pNew + 0x1f, 4);
                        __builtin_memcpy(pSub + 0x79, &DAT_00525aae[uVar12 * 2], 2);
                        __builtin_memcpy(pSub + 0x7d, &DAT_00525a7e[uVar12 * 2], 2);
                        sVar11 = FUN_00446fe0(uVar12);
                        sTmp = (short)(sVar11 * 3);
                        __builtin_memcpy(&pSub, pNew + 0x1f, 4);
                        __builtin_memcpy(pSub + 0x7f, &sTmp, 2);
                        FUN_00483410(sVar8, sVar27, pNew);
                        __builtin_memcpy(&pSub, pNew + 0x1f, 4);
                        __builtin_memcpy(pSub + 6, &iVar18, 4);
                        __builtin_memcpy(&pSub, pNew + 0x1f, 4);
                        __builtin_memcpy(pSub + 10, &iVar26, 4);
                        __builtin_memcpy(&pSub, pNew + 0x1f, 4);
                        __builtin_memcpy(pSub + 0xe, &iVar28, 4);
                        __builtin_memcpy(&sTmp, &s_starshipintohyperpln_00525b38[uVar12 * 2 + 6], 2);
                        /* ECX = DAT_006db720, EDX = (int)sTmp in the original */
                        uVar29 = FUN_00413ca0(DAT_006db720, (int)sTmp);
                        iVar18 = (int)((unsigned int)iVar18 + (unsigned int)(int)uVar29);
                        __builtin_memcpy(&sTmp, &s_starshipintohyperpln_00525b38[uVar12 * 2 + 6], 2);
                        /* ECX = iVar18, EDX = (int)sTmp in the original */
                        uVar29 = FUN_00413ca0(DAT_006db704, (int)sTmp);
                        iVar26 = (int)((unsigned int)iVar26 + (unsigned int)(int)uVar29);
                        __builtin_memcpy(&sTmp, &s_starshipintohyperpln_00525b38[uVar12 * 2 + 6], 2);
                        /* ECX = iVar26, EDX = (int)sTmp in the original */
                        uVar29 = FUN_00413ca0(DAT_006db708, (int)sTmp);
                        __builtin_memcpy(pNew + 4, &iVar18, 4);
                        __builtin_memcpy(pNew + 8, &iVar26, 4);
                        iTmp = (int)((unsigned int)iVar28 + (unsigned int)(int)uVar29);
                        __builtin_memcpy(pNew + 0xc, &iTmp, 4);
                        iVar5 = (int)((unsigned int)(unsigned short)(uVar10 - (unsigned short)DAT_0071b704) * 8);
                        pSlot = DAT_006db798 + iVar5;
                        pSlot[1] = 0;
                        __builtin_memcpy(pSlot + 4, &param_3, 2);
                        __builtin_memcpy(&pSub, pNew + 0x1f, 4);
                        __builtin_memcpy(pSlot + 6, pSub + 0x79, 2);
                        pSlot[0] = 0xff;
                        __builtin_memcpy(&pSub, pNew + 0x1f, 4);
                        __builtin_memcpy(pSub + 0xa3, &pSlot, 4);
                        FUN_0040a3c0((unsigned int)uVar10, DAT_0071ecf4);
                    }
                }
            }
        }
    }
    return;
}

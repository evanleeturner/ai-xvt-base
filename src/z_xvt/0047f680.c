/* SPDX-License-Identifier: MIT */
/* z_xvt 0047f680 */
extern unsigned char DAT_007092c1;
extern unsigned short DAT_0071e6a4;
extern unsigned short DAT_00732a3c;
extern int DAT_00703c88;
extern int DAT_00732a38;
extern int DAT_006d4988;
extern unsigned short DAT_006d4980;
extern unsigned short DAT_006a89f4;
extern unsigned char *DAT_006a89f8;
extern unsigned char DAT_00526872[];
extern unsigned char DAT_00528230[];
extern unsigned char DAT_0052b0c3[];
extern char DAT_00524948[];
extern char DAT_00703ca0[];
extern char s_stationaryldrpln_00524934[];
extern char s_stationaryflwpln_00524920[];
extern int DAT_00732e54;
extern int DAT_00732a2c;
extern int DAT_00732a28;
extern int DAT_00732a24;

void FUN_00401c20(void);
unsigned int FUN_004490f0(unsigned int a);
int FUN_0042b150(int a);
int FUN_00449790(unsigned int a, unsigned int b);
unsigned short *FUN_0042b1e0(int a, int b);
void FUN_0044a780(unsigned int a, unsigned int b);
void FUN_00421c70(unsigned int a, int b);
void FUN_00483410(short a, short b, int c);
void FUN_00483540(short a, int b, int c);
int FUN_00413cc0(int a, int b, int c, int d, int e, int f);
long long __ftol(void);
unsigned int FUN_00419100(unsigned int a, unsigned int b);
unsigned short FUN_00478470(void);
void FUN_00421580(int a);
int FUN_0044ad00(unsigned int a);
void FUN_0041a750(unsigned int a, int b);
void FUN_00421df0(unsigned int a, int b);
void FUN_00422190(unsigned int a);
int FUN_00449cb0(int a, unsigned int b);
void FUN_004a1d70(int a, int b);
void FUN_004223d0(unsigned int a);

#define E(o) ((unsigned char *)((unsigned int)DAT_006d4988 + (unsigned int)iVar26 + (unsigned int)(o)))
#define S(o) ((unsigned char *)((unsigned int)DAT_00732e54 + (unsigned int)(o)))

void __stdcall FUN_0047f680(void)
{
    short sTmp;
    short sA;
    short sB;
    short sC;
    int iTmp;
    unsigned int uVar4;
    int iVar26;
    int iVar18;
    int iVar3;
    int iVar11;
    int iVar12;
    unsigned int uVar9;
    unsigned int uVar13;
    unsigned int uVar14;
    unsigned int uVar15;
    unsigned int uVar21;
    unsigned short uVar7;
    unsigned short uVar8;
    unsigned short uVar22;
    unsigned short uStack_2a;
    unsigned short *puVar10;
    unsigned char bVar17;
    unsigned char *pbVar16;
    unsigned char *pEnt;
    char cVar6;
    int bVar5;
    int bVar24;
    char *pcVar19;
    char *pcVar20;
    char *pcVar23;
    long long lVar25;
    int iStack_1c;
    int iStack_18;
    int iStack_14;

    if (DAT_007092c1 != 0) {
        FUN_00401c20();
    }
    if (DAT_0071e6a4 == 0) {
        DAT_0071e6a4 = 0x1d;
        uStack_2a = DAT_00732a3c;
        if ((int)(unsigned int)DAT_00732a3c < (int)((unsigned int)DAT_00703c88 + (unsigned int)DAT_00732a38)) {
            do {
                uVar4 = (unsigned int)uStack_2a;
                iVar26 = (int)(uVar4 * 0x23u);
                bVar17 = *E(3);
                if ((99 < bVar17) && (bVar17 < 0x6a)) {
                    iVar18 = (int)((unsigned int)((int)(uVar4 - (unsigned int)DAT_00703c88) >> 4) * 0x1du);
                    __builtin_memcpy(&sTmp, E(0x14), 2);
                    sTmp = (short)((unsigned short)sTmp +
                           (unsigned short)((int)((unsigned int)iVar18 + (unsigned int)((iVar18 >> 0x1f) & 0xf)) >> 4));
                    __builtin_memcpy(E(0x14), &sTmp, 2);

                    iVar18 = (int)((unsigned int)((int)(uVar4 - (unsigned int)DAT_00703c88) >> 3) * 0x1du);
                    __builtin_memcpy(&sTmp, E(0x12), 2);
                    sTmp = (short)((unsigned short)sTmp +
                           (unsigned short)((int)((unsigned int)iVar18 + (unsigned int)((iVar18 >> 0x1f) & 0x1f)) >> 5));
                    __builtin_memcpy(E(0x12), &sTmp, 2);

                    iVar18 = (int)((unsigned int)(int)(4u - (unsigned int)((int)(uVar4 - (unsigned int)DAT_00703c88) >> 4)) * 0x1du);
                    __builtin_memcpy(&sTmp, E(0x10), 2);
                    sTmp = (short)((unsigned short)sTmp +
                           (unsigned short)((int)((unsigned int)iVar18 + (unsigned int)((iVar18 >> 0x1f) & 0xf)) >> 4));
                    __builtin_memcpy(E(0x10), &sTmp, 2);
                }
                pEnt = E(0);
                __builtin_memcpy(&iTmp, pEnt + 0x1f, 4);
                if (iTmp == 0) {
                    if (pEnt[3] != 0) {
                        __builtin_memcpy(&DAT_006a89f8, &DAT_00526872[(unsigned int)pEnt[3] * 0x16], 4);
                        if (DAT_006a89f8 != 0) {
                            DAT_006a89f4 = (unsigned short)pEnt[0x19];
                            if (DAT_006a89f8 != 0) {
                                DAT_006a89f4 = (unsigned short)(DAT_006a89f4 + 1);
                                uVar8 = *(unsigned short *)(DAT_006a89f8 + (unsigned int)DAT_006a89f4 * 2);
                                if (uVar8 == 0xffff) {
                                    iVar18 = (int)((((unsigned int)uStack_2a * 8u) - (unsigned int)uStack_2a) * 5u);
                                    *(unsigned char *)((unsigned int)DAT_006d4988 + 3u + (unsigned int)iVar18) = 0;
                                    if (uStack_2a < DAT_006d4980) {
                                        __builtin_memcpy(&iTmp, (unsigned char *)((unsigned int)DAT_006d4988 + 0x1fu + (unsigned int)iVar18), 4);
                                        __builtin_memcpy(&iVar18, (unsigned char *)iTmp + 0xa7, 4);
                                        if (iVar18 != 0) {
                                            FUN_00421580(iVar18);
                                        }
                                    }
                                } else if (uVar8 == 0xfffd) {
                                    DAT_006a89f4 = 0;
                                } else {
joined_r0x004800fb:
                                    if ((0xfeff < uVar8) && (uVar8 != 0xfffe)) {
                                        DAT_006a89f4 = (unsigned short)(uVar8 + 0x100);
                                    }
                                }
                            }
LAB_0048010d:
                            *E(0x19) = (unsigned char)DAT_006a89f4;
                        }
                    }
                } else {
                    uVar8 = (unsigned short)pEnt[3];
                    if (uVar8 != 0) {
                        uVar4 = (unsigned int)uVar8;
                        __builtin_memcpy(&DAT_006a89f8, &DAT_00526872[uVar4 * 0x16], 4);
                        __builtin_memcpy(&iTmp, pEnt + 0x1f, 4);
                        __builtin_memcpy(&iVar3, (unsigned char *)iTmp + 0xa7, 4);
                        switch (pEnt[2]) {
                        case 0:
                        case 1:
                        case 2:
                        case 3:
                        case 4:
                        case 5:
                            uVar9 = FUN_004490f0(uVar4);
                            bVar5 = 0;
                            __builtin_memcpy(&iTmp, E(0x1f), 4);
                            __builtin_memcpy(&DAT_00732e54, (unsigned char *)iTmp + 0xa7, 4);
                            iVar18 = FUN_0042b150(DAT_00732e54);
                            __builtin_memcpy(&sTmp, S(0x1df), 2);
                            if (sTmp == 0) {
LAB_0047f8a1:
                                bVar24 = 0;
                            } else {
                                pcVar20 = DAT_00703ca0 + (unsigned int)*(unsigned char *)((unsigned int)iVar18 + 10u) * 0x55u;
                                iVar18 = 8;
                                bVar24 = 0;
                                pcVar19 = pcVar20;
                                pcVar23 = DAT_00524948;
                                do {
                                    if (iVar18 == 0) break;
                                    iVar18 = (int)((unsigned int)iVar18 - 1u);
                                    bVar24 = (*pcVar19 == *pcVar23);
                                    pcVar19 = pcVar19 + 1;
                                    pcVar23 = pcVar23 + 1;
                                } while (bVar24);
                                if (bVar24) goto LAB_0047f8a1;
                                iVar18 = 0x11;
                                bVar24 = 0;
                                pcVar19 = pcVar20;
                                pcVar23 = s_stationaryldrpln_00524934;
                                do {
                                    if (iVar18 == 0) break;
                                    iVar18 = (int)((unsigned int)iVar18 - 1u);
                                    bVar24 = (*pcVar19 == *pcVar23);
                                    pcVar19 = pcVar19 + 1;
                                    pcVar23 = pcVar23 + 1;
                                } while (bVar24);
                                if (bVar24) goto LAB_0047f8a1;
                                iVar18 = 0x11;
                                bVar24 = 0;
                                pcVar19 = s_stationaryflwpln_00524920;
                                do {
                                    if (iVar18 == 0) break;
                                    iVar18 = (int)((unsigned int)iVar18 - 1u);
                                    bVar24 = (*pcVar20 == *pcVar19);
                                    pcVar20 = pcVar20 + 1;
                                    pcVar19 = pcVar19 + 1;
                                } while (bVar24);
                                if (bVar24) goto LAB_0047f8a1;
                                bVar24 = 1;
                            }
                            if ((bVar24) && (iStack_1c = 0, *S(0x213) != 0)) {
                                iStack_18 = 0;
                                iStack_14 = 0;
                                do {
                                    if (((*(char *)S(0x344u + (unsigned int)iStack_14) == 2) &&
                                         (uVar21 = (unsigned int)DAT_0052b0c3[(unsigned int)iStack_18 + (unsigned int)*S(4) * 0xefu],
                                          *S(0x31au + uVar21) != 0)) &&
                                        (iVar18 = FUN_00449790(uVar4, uVar21), iVar18 == 0x15)) {
                                        puVar10 = FUN_0042b1e0(DAT_00732e54, iStack_1c);
                                        if (*puVar10 == 0xffff) {
                                            bVar17 = *S(0x2f2u + uVar21);
                                            if ((bVar17 & 1) == 0) {
                                                cVar6 = (char)(bVar17 - 4);
                                            } else {
                                                cVar6 = (char)(bVar17 + 4);
                                            }
                                            *S(0x2f2u + uVar21) = (unsigned char)cVar6;
                                            uVar22 = FUN_00478470();
                                            if (uVar22 < 0x600) {
                                                pbVar16 = S(0x2f2u + uVar21);
                                                *pbVar16 = (unsigned char)(*pbVar16 ^ 1);
                                            }
                                        } else {
                                            FUN_0044a780(uVar4, uVar21);
                                            iVar18 = (int)((unsigned int)DAT_006d4988 + (unsigned int)iVar26);
                                            FUN_00421c70((unsigned int)*puVar10, 0);
                                            __builtin_memcpy(&iTmp, (unsigned char *)((unsigned int)iVar18 + 4u), 4);
                                            DAT_00732a2c = (int)((unsigned int)DAT_00732a2c - (unsigned int)iTmp);
                                            __builtin_memcpy(&iTmp, (unsigned char *)((unsigned int)iVar18 + 8u), 4);
                                            DAT_00732a28 = (int)((unsigned int)DAT_00732a28 - (unsigned int)iTmp);
                                            __builtin_memcpy(&iTmp, (unsigned char *)((unsigned int)iVar18 + 0xcu), 4);
                                            DAT_00732a24 = (int)((unsigned int)DAT_00732a24 - (unsigned int)iTmp);
                                            __builtin_memcpy(&iTmp, (unsigned char *)((unsigned int)iVar18 + 0x1fu), 4);
                                            if (*(char *)((unsigned int)iTmp + 0x90u) != 0) {
                                                short s12, s10, s14;
                                                __builtin_memcpy(&s12, (unsigned char *)((unsigned int)iVar18 + 0x12u), 2);
                                                __builtin_memcpy(&s10, (unsigned char *)((unsigned int)iVar18 + 0x10u), 2);
                                                FUN_00483410(s12, s10, iVar18);
                                                __builtin_memcpy(&s14, (unsigned char *)((unsigned int)iVar18 + 0x14u), 2);
                                                FUN_00483540(s14, 0, iVar18);
                                            }
                                            __builtin_memcpy(&iVar12, (unsigned char *)((unsigned int)iVar18 + 0x1fu), 4);
                                            __builtin_memcpy(&sA, (unsigned char *)iVar12 + 0x97, 2);
                                            __builtin_memcpy(&sB, (unsigned char *)iVar12 + 0x99, 2);
                                            __builtin_memcpy(&sC, (unsigned char *)iVar12 + 0x9b, 2);
                                            iVar11 = FUN_00413cc0(DAT_00732a2c, DAT_00732a28, DAT_00732a24,
                                                                  (int)sA, (int)sB, (int)sC);
                                            __builtin_memcpy(&iVar12, (unsigned char *)((unsigned int)iVar18 + 0x1fu), 4);
                                            __builtin_memcpy(&sA, (unsigned char *)iVar12 + 0x91, 2);
                                            __builtin_memcpy(&sB, (unsigned char *)iVar12 + 0x93, 2);
                                            __builtin_memcpy(&sC, (unsigned char *)iVar12 + 0x95, 2);
                                            iVar12 = FUN_00413cc0(DAT_00732a2c, DAT_00732a28, DAT_00732a24,
                                                                  (int)sA, (int)sB, (int)sC);
                                            __builtin_memcpy(&iVar18, (unsigned char *)((unsigned int)iVar18 + 0x1fu), 4);
                                            __builtin_memcpy(&sA, (unsigned char *)iVar18 + 0x9d, 2);
                                            __builtin_memcpy(&sB, (unsigned char *)iVar18 + 0x9f, 2);
                                            __builtin_memcpy(&sC, (unsigned char *)iVar18 + 0xa1, 2);
                                            iVar18 = FUN_00413cc0(DAT_00732a2c, DAT_00732a28, DAT_00732a24,
                                                                  (int)sA, (int)sB, (int)sC);
                                            lVar25 = __ftol();
                                            DAT_00732a2c = (int)((unsigned int)iVar11 - (unsigned int)(int)lVar25);
                                            lVar25 = __ftol();
                                            DAT_00732a28 = (int)((0u - (unsigned int)(int)lVar25) - (unsigned int)iVar12);
                                            lVar25 = __ftol();
                                            DAT_00732a24 = (int)((unsigned int)iVar18 - (unsigned int)(int)lVar25);
                                            lVar25 = __ftol();
                                            iVar18 = (int)lVar25;
                                            lVar25 = __ftol();
                                            iVar12 = (int)lVar25;
                                            lVar25 = __ftol();
                                            FUN_00413cc0(DAT_00732a2c, DAT_00732a28, DAT_00732a24, (int)lVar25, iVar12, iVar18);
                                            lVar25 = __ftol();
                                            iVar18 = (int)lVar25;
                                            lVar25 = __ftol();
                                            iVar12 = (int)lVar25;
                                            lVar25 = __ftol();
                                            uVar13 = (unsigned int)FUN_00413cc0(DAT_00732a2c, DAT_00732a28, DAT_00732a24,
                                                                                (int)lVar25, iVar12, iVar18);
                                            lVar25 = __ftol();
                                            iVar18 = (int)lVar25;
                                            lVar25 = __ftol();
                                            iVar12 = (int)lVar25;
                                            lVar25 = __ftol();
                                            uVar14 = (unsigned int)FUN_00413cc0(DAT_00732a2c, DAT_00732a28, DAT_00732a24,
                                                                                (int)lVar25, iVar12, iVar18);
                                            uVar15 = FUN_00419100(uVar14, uVar13);
                                            *S(0x2f2u + uVar21) = (unsigned char)(uVar15 >> 8);
                                        }
                                    }
                                    iStack_18 = (int)((unsigned int)iStack_18 + 8u);
                                    iStack_14 = (int)((unsigned int)iStack_14 + 4u);
                                    iStack_1c = (int)((unsigned int)iStack_1c + 1u);
                                } while (iStack_1c < (int)(unsigned int)*S(0x213));
                            }
                            uVar22 = 0;
                            if ((unsigned short)uVar9 != 0) {
                                do {
                                    iVar18 = FUN_00449790(uVar4, (unsigned int)uVar22);
                                    if (iVar18 == 3) {
                                        DAT_006a89f8 = DAT_00528230;
                                        DAT_006a89f4 = (unsigned short)(*(unsigned char *)((uVar9 & 0xffffu) + 0x2cau + (unsigned int)iVar3) + 1);
                                        uVar7 = *(unsigned short *)(DAT_00528230 + (unsigned int)DAT_006a89f4 * 2);
                                        if (uVar7 == 0xffff) {
                                            iVar12 = (int)((((unsigned int)uStack_2a * 8u) - (unsigned int)uStack_2a) * 5u);
                                            *(unsigned char *)((unsigned int)DAT_006d4988 + 3u + (unsigned int)iVar12) = 0;
                                            if (uStack_2a < DAT_006d4980) {
                                                __builtin_memcpy(&iTmp, (unsigned char *)((unsigned int)DAT_006d4988 + 0x1fu + (unsigned int)iVar12), 4);
                                                __builtin_memcpy(&iVar12, (unsigned char *)iTmp + 0xa7, 4);
                                                if (iVar12 != 0) {
                                                    FUN_00421580(iVar12);
                                                }
                                            }
                                        } else if (uVar7 == 0xfffd) {
                                            DAT_006a89f4 = 0;
                                        } else if ((0xfeff < uVar7) && (uVar7 != 0xfffe)) {
                                            DAT_006a89f4 = (unsigned short)(uVar7 + 0x100);
                                        }
                                        *(unsigned char *)((uVar9 & 0xffffu) + 0x2cau + (unsigned int)iVar3) = (unsigned char)DAT_006a89f4;
                                    }
                                    if (*S(7) == 3) {
                                        iVar12 = FUN_0044ad00(uVar4);
                                        if (iVar12 == 0) {
                                            uVar22 = (unsigned short)(uVar22 + 3);
                                            FUN_0041a750(((unsigned int)uStack_2a << 16) | (unsigned int)uStack_2a, 0);
                                        } else {
                                            FUN_00421df0(((unsigned int)uStack_2a << 16) | (unsigned int)uStack_2a, 0);
                                            uVar7 = FUN_00478470();
                                            if (uVar7 < 0x1800) {
                                                FUN_00422190(((unsigned int)uStack_2a << 16) | (unsigned int)uStack_2a);
                                            }
                                        }
                                    }
                                    if ((bVar24) &&
                                        ((((iVar18 == 0xb || (iVar18 == 0x17)) || (iVar18 == 0xc)) ||
                                          (((iVar18 == 0x18 || (iVar18 == 0xd)) || (iVar18 == 0x19)))))) {
                                        uVar21 = (unsigned int)uVar22;
                                        bVar17 = *S(0x2f2u + uVar21);
                                        if ((bVar17 & 1) == 0) {
                                            cVar6 = (char)(bVar17 - 4);
                                        } else {
                                            cVar6 = (char)(bVar17 + 4);
                                        }
                                        *S(0x2f2u + uVar21) = (unsigned char)cVar6;
                                        uVar7 = FUN_00478470();
                                        if (uVar7 < 0x200) {
                                            pbVar16 = S(0x2f2u + uVar21);
                                            *pbVar16 = (unsigned char)(*pbVar16 ^ 1);
                                        }
                                    }
                                    if ((iVar18 == 0x14) && ((*S(0x29) & 1) != 0)) {
                                        if ((*S(0x29) & 2) == 0) {
                                            if (uVar8 == 1) {
                                                pbVar16 = S((unsigned int)uVar22 + 0x2f2u);
                                                if (*pbVar16 != 0) {
                                                    bVar17 = (unsigned char)(*pbVar16 - 1);
LAB_0047fe92:
                                                    *pbVar16 = bVar17;
                                                    goto LAB_0047fe94;
                                                }
                                            } else if (uVar8 == 4) {
                                                pbVar16 = S((unsigned int)uVar22 + 0x2f2u);
                                                if (*pbVar16 != 0) {
                                                    bVar17 = (unsigned char)(*pbVar16 - 3);
                                                    goto LAB_0047fe92;
                                                }
                                            }
                                        } else if (uVar8 == 1) {
                                            iVar18 = FUN_00449cb0(1, (unsigned int)uVar22);
                                            bVar17 = 8;
                                            if (-1 < iVar18) {
                                                bVar17 = 0xc;
                                            }
                                            pbVar16 = S(0x2f2u + (unsigned int)uVar22);
                                            if (*pbVar16 < bVar17) {
                                                *pbVar16 = (unsigned char)(*pbVar16 + 1);
LAB_0047fe94:
                                                bVar5 = 1;
                                            }
                                        } else if (uVar8 == 4) {
                                            pbVar16 = S((unsigned int)uVar22 + 0x2f2u);
                                            if (*pbVar16 < 0x40) {
                                                bVar17 = (unsigned char)(*pbVar16 + 3);
                                                goto LAB_0047fe92;
                                            }
                                        }
                                    }
                                    uVar22 = (unsigned short)(uVar22 + 1);
                                } while (uVar22 < (unsigned short)uVar9);
                            }
                            if ((*S(0x29) & 1) != 0) {
                                if ((*S(0x29) & 2) == 0) {
                                    if (!bVar5) {
                                        *S(0x29) = 0;
                                        __builtin_memcpy(&iVar26, (unsigned char *)((unsigned int)DAT_006d4988 + 0x1bu + (unsigned int)iVar26), 4);
                                        iVar18 = 0x7f;
                                        goto LAB_0047fef8;
                                    }
                                } else if (!bVar5) {
                                    *S(0x29) = 2;
                                    __builtin_memcpy(&iVar26, (unsigned char *)((unsigned int)DAT_006d4988 + 0x1bu + (unsigned int)iVar26), 4);
                                    iVar18 = 0x80;
LAB_0047fef8:
                                    FUN_004a1d70(iVar18, iVar26);
                                }
                            }
                            if (((*S(7) == 0) && (*S(0x23c) == 1)) && (*(short *)S(0x23e) != 0)) {
                                FUN_004223d0(((unsigned int)uStack_2a << 16) | (unsigned int)uStack_2a);
                                FUN_004223d0(((unsigned int)uStack_2a << 16) | (unsigned int)uStack_2a);
                                FUN_004223d0(((unsigned int)uStack_2a << 16) | (unsigned int)uStack_2a);
                            }
                            break;
                        case 0xb:
                        case 0xd:
                            if (pEnt[3] != 'Y') {
                                DAT_006a89f4 = (unsigned short)pEnt[0x19];
                                if (DAT_006a89f8 != 0) {
                                    DAT_006a89f4 = (unsigned short)(DAT_006a89f4 + 1);
                                    uVar8 = *(unsigned short *)(DAT_006a89f8 + (unsigned int)DAT_006a89f4 * 2);
                                    if (uVar8 == 0xffff) {
                                        iVar18 = (int)((((unsigned int)uStack_2a * 8u) - (unsigned int)uStack_2a) * 5u);
                                        *(unsigned char *)((unsigned int)DAT_006d4988 + 3u + (unsigned int)iVar18) = 0;
                                        if (uStack_2a < DAT_006d4980) {
                                            __builtin_memcpy(&iTmp, (unsigned char *)((unsigned int)DAT_006d4988 + 0x1fu + (unsigned int)iVar18), 4);
                                            __builtin_memcpy(&iVar18, (unsigned char *)iTmp + 0xa7, 4);
                                            if (iVar18 != 0) {
                                                FUN_00421580(iVar18);
                                            }
                                        }
                                    } else {
                                        if (uVar8 != 0xfffd) goto joined_r0x004800fb;
                                        DAT_006a89f4 = 0;
                                    }
                                }
                                goto LAB_0048010d;
                            }
                            pEnt[0x1a] = 0;
                            uVar8 = FUN_00478470();
                            if (uVar8 < 0x800) {
                                FUN_00422190(((unsigned int)uStack_2a << 16) | (unsigned int)uStack_2a);
                            }
                        }
                    }
                }
                uStack_2a = (unsigned short)(uStack_2a + 1);
            } while ((int)(unsigned int)uStack_2a < (int)((unsigned int)DAT_00703c88 + (unsigned int)DAT_00732a38));
        }
    }
    return;
}

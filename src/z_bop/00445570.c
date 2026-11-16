/* SPDX-License-Identifier: MIT */
/* z_bop 00445570 */
/* Unresolved: FUN_* prototypes below are inferred from call sites only; the
 * calling conventions are assumed to be cdecl and are not confirmed.
 * Unresolved: unaff_BP is the caller's BP register at entry in the decompile;
 * its value is unknown, so it is initialised to 0 to satisfy the build.
 * Entity records (stride 0x23) and player records (stride 0x5bd) are accessed
 * by byte offset, because the evidence gives no full layout. Many of those
 * fields are unaligned, so they go through memcpy (LD/ST); the memcpy
 * expansions are what inflate the raw call count against the vanilla binary. */

extern unsigned char DAT_009e9680[];
extern unsigned char DAT_009e9670[];
extern unsigned char DAT_009e96c5[];
extern unsigned char DAT_009e96c9[];
extern unsigned char DAT_009e96cd[];
extern unsigned char DAT_009e96d1[];
extern unsigned char DAT_009e96d5[];
extern unsigned char DAT_009e96d9[];
extern unsigned int DAT_009ecc34;
extern int DAT_009ec5d0;
extern int DAT_009a1fe8;
extern int DAT_009a1fe0;
extern unsigned int DAT_009d8a58;
extern unsigned int DAT_009d8a54;
extern unsigned int DAT_009d8b60;
extern unsigned short DAT_009ec5fe;
extern short DAT_009d8112;
extern int DAT_00a07ce8;
extern int DAT_009e9658;
extern unsigned char DAT_009d6940[2];
extern unsigned int DAT_009a20b0;
extern unsigned int DAT_009cd26c;
extern unsigned int DAT_009fd438;
extern unsigned int DAT_00a07cdc;
extern unsigned int DAT_00a07cd8;
extern unsigned int DAT_00a07cd4;
extern short DAT_009a8070;
extern short DAT_00a080f0;
extern int DAT_009a8da4;
extern int DAT_009a8d84;
extern int DAT_009a8d88;
extern unsigned char DAT_0051c635[];
extern unsigned char DAT_0051c633[];
extern unsigned char DAT_0051c593[];
extern unsigned char DAT_0051c63d[];
extern unsigned char DAT_0051c63f[];
extern unsigned char DAT_0051c639[];
extern unsigned char DAT_0051c63b[];
extern unsigned char DAT_0051c637[];
extern unsigned char DAT_00521f1c[];
extern unsigned char DAT_0051a44f[];
extern char s_Unable_to_quantize_image_00521c20[];
extern unsigned char DAT_00521cc8[];
extern unsigned char DAT_00521d20[];

unsigned int FUN_004269e0(unsigned int a);
void FUN_004035d0(int a, int b, short c, short d);
void FUN_0042b290(void);
void FUN_00458fa0(unsigned int a, int b);
unsigned char FUN_00425cb0(void);
void FUN_0041f300(unsigned int a, char b);
void FUN_00433d30(unsigned short a, unsigned short b, int c);
int FUN_004af5b0(unsigned int a);
void FUN_004a7480(unsigned int a, int b);
void FUN_00427bd0(short a, short b, int c);
void FUN_004464c0(int a);
void FUN_00458e20(unsigned int a, int b);
int FUN_004ae640(unsigned int a, unsigned int b);
int FUN_004ae6e0(unsigned int a, unsigned int b);
int FUN_004ae5a0(unsigned int a, unsigned int b);
void FUN_004037b0(int a, int b, int c, int d);
void FUN_0046a8d0(unsigned int a, unsigned int b, unsigned int c);

#define LD(T, a) ({ T t_; __builtin_memcpy(&t_, (const void *)(a), sizeof(T)); t_; })
#define ST(T, a, v) do { T st_ = (T)(v); __builtin_memcpy((void *)(a), &st_, sizeof(T)); } while (0)
#define XD LD(int, iVar6 + 0x1f)
#define MUL15(sv, u) \
    ({ long long l_ = (long long)(int)(sv) * (long long)(int)(u); \
       (((unsigned int)l_ >> 0xf) | ((unsigned int)((unsigned long long)l_ >> 0x20) << 0x11)); })

void __stdcall FUN_00445570(void)
{
    int iVar5;
    int iVar6;
    unsigned char uVar2;
    unsigned char bVar3;
    unsigned short uVar4;
    short sVar9;
    unsigned char bVar10;
    char cVar11;
    short sVar12;
    unsigned int uVar13;
    int iVar14;
    int iVar15;
    int iVar16;
    int iVar19;
    unsigned int uVar18;
    unsigned int uVar22;
    unsigned int uVar25;
    unsigned short uVar21;
    unsigned short uVar23;
    short sVar24;
    int psVar20;
    int puVar26;
    short sStack_8;
    unsigned short local_6;
    unsigned short unaff_BP = 0;
    int bVar8;
    int bVar17;

    uVar23 = 0;
    do {
        uVar18 = (unsigned int)uVar23;
        iVar6 = uVar18 * 0x5bd;
        if (((((DAT_009e9680)[uVar18 * 0x5bd] != '\0') && (uVar18 != DAT_009ecc34)) &&
             (iVar5 = LD(int, (int)DAT_009e9670 + iVar6), iVar5 != -1)) &&
            ((iVar19 = iVar5 * 0x23 + DAT_009a1fe8, *(char *)(iVar19 + 2) == '\0' &&
              ((DAT_009ec5d0 == -1 || (DAT_009ec5d0 == iVar5)))))) {
            uVar18 = FUN_004269e0(((unsigned int)iVar5 & 0xffff0000u) | (unsigned int)*(unsigned char *)(iVar19 + 3));
            iVar5 = (uVar18 & 0xffff) * 0x101;
            FUN_004035d0(LD(int, (int)DAT_009e9670 + iVar6) * 0x23 + DAT_009a1fe8, 0,
                         LD(short, (int)DAT_0051c635 + iVar5), LD(short, (int)DAT_0051c633 + iVar5));
            ST(unsigned int, (int)DAT_009e96d1 + iVar6, LD(unsigned int, (int)DAT_009e96c5 + iVar6));
            uVar22 = DAT_009d8a58;
            ST(unsigned int, (int)DAT_009e96d5 + iVar6, LD(unsigned int, (int)DAT_009e96c9 + iVar6));
            uVar18 = DAT_009d8a54;
            ST(unsigned int, (int)DAT_009e96d9 + iVar6, LD(unsigned int, (int)DAT_009e96cd + iVar6));
            uVar25 = DAT_009d8b60;
            ST(unsigned int, (int)DAT_009e96c5 + iVar6, uVar22);
            ST(unsigned int, (int)DAT_009e96c9 + iVar6, uVar18);
            ST(unsigned int, (int)DAT_009e96cd + iVar6, uVar25);
        }
        uVar23 = uVar23 + 1;
    } while (uVar23 < 8);

    iVar5 = (DAT_009ecc34 & 0xffff) * 0x5bd;
    iVar6 = LD(int, (int)DAT_009e9670 + (DAT_009ecc34 & 0xffff) * 0x5bd);
    if (((iVar6 != -1) && (iVar19 = iVar6 * 0x23 + DAT_009a1fe8, *(char *)(iVar19 + 2) == '\0')) &&
        ((DAT_009ec5d0 == -1 || (DAT_009ec5d0 == iVar6)))) {
        uVar18 = FUN_004269e0(((unsigned int)iVar6 & 0xffff0000u) | (unsigned int)*(unsigned char *)(iVar19 + 3));
        iVar6 = (uVar18 & 0xffff) * 0x101;
        FUN_004035d0(LD(int, (int)DAT_009e9670 + iVar5) * 0x23 + DAT_009a1fe8, 0,
                     LD(short, (int)DAT_0051c635 + iVar6), LD(short, (int)DAT_0051c633 + iVar6));
        ST(unsigned int, (int)DAT_009e96d1 + iVar5, LD(unsigned int, (int)DAT_009e96c5 + iVar5));
        uVar22 = DAT_009d8a58;
        ST(unsigned int, (int)DAT_009e96d5 + iVar5, LD(unsigned int, (int)DAT_009e96c9 + iVar5));
        uVar18 = DAT_009d8a54;
        ST(unsigned int, (int)DAT_009e96d9 + iVar5, LD(unsigned int, (int)DAT_009e96cd + iVar5));
        uVar25 = DAT_009d8b60;
        ST(unsigned int, (int)DAT_009e96c5 + iVar5, uVar22);
        ST(unsigned int, (int)DAT_009e96c9 + iVar5, uVar18);
        ST(unsigned int, (int)DAT_009e96cd + iVar5, uVar25);
    }

    uVar23 = DAT_009ec5fe;
    sVar9 = DAT_009d8112;
    local_6 = 0;
    bVar8 = 0;
    if (0 < DAT_00a07ce8) {
        do {
            if (DAT_009ec5d0 != -1) {
                if (bVar8) {
                    DAT_009d8112 = sVar9;
                    DAT_009ec5fe = uVar23;
                    return;
                }
                bVar8 = 1;
                local_6 = (unsigned short)DAT_009ec5d0;
            }
            uVar18 = (unsigned int)local_6;
            iVar5 = uVar18 * 0x23;
            iVar6 = DAT_009a1fe8 + iVar5;
            DAT_009d8112 = sVar9;
            DAT_009ec5fe = uVar23;
            if ((XD != 0) && (iVar19 = LD(int, XD + 2), iVar19 != 0)) {
                DAT_009ec5fe = (unsigned short)(uVar23 + ((short)DAT_009e9658 - (short)iVar19));
                if (DAT_009ec5fe != 0) {
                    DAT_009d8112 = (short)(0xecu / (unsigned int)DAT_009ec5fe);
                    if (DAT_009d8112 == 0) {
                        DAT_009d8112 = 1;
                    }
                    ST(int, XD + 2, (unsigned int)LD(int, XD + 2) + (unsigned int)DAT_009ec5fe);
                    goto LAB_004458bd;
                }
                if ((DAT_009ec5d0 == -1) && (LD(unsigned int, iVar6 + 0x1b) != 0xffffffffu)) {
                    if ((DAT_009d6940[1] != '\0') && (LD(unsigned int, iVar6 + 0x1b) == DAT_009ecc34)) {
                        FUN_0042b290();
                    }
                    ST(unsigned int, XD + 6, LD(unsigned int, iVar6 + 4));
                    ST(unsigned int, XD + 10, LD(unsigned int, iVar6 + 8));
                    ST(unsigned int, XD + 0xe, LD(unsigned int, iVar6 + 0xc));
                }
                goto switchD_00445bc5_caseD_5;
            }
LAB_004458bd:
            if (*(char *)(iVar6 + 3) == '\0') goto switchD_00445bc5_caseD_5;
            uVar2 = *(unsigned char *)(iVar6 + 2);
            uVar4 = LD(unsigned short, XD + 0x81);
            if (uVar4 == 0) goto LAB_00445a45;
            uVar21 = (unsigned short)(uVar4 - DAT_009ec5fe);
            if (uVar4 < uVar21) {
                uVar21 = 0;
            }
            ST(unsigned short, XD + 0x81, uVar21);
            if (uVar21 != 0) goto LAB_00445a45;
            switch (uVar2) {
            case 0:
                FUN_00458fa0(((unsigned int)unaff_BP << 16) | (unsigned int)local_6, 1);
                bVar10 = FUN_00425cb0();
                FUN_0041f300(uVar18, (char)((bVar10 & 1) + 0x7f));
                FUN_00433d30(local_6, (unsigned short)*(unsigned char *)(DAT_009a1fe8 + 0x16 + iVar5), 2);
                break;
            case 1:
            case 2:
            case 3:
            case 4:
            case 5:
                iVar19 = FUN_004af5b0((unsigned int)*(unsigned char *)(iVar6 + 3));
                if (iVar19 == 0) {
                    FUN_004a7480(((unsigned int)unaff_BP << 16) | (unsigned int)local_6, 1);
                    FUN_0041f300(uVar18, (char)0x82);
                    *(char *)(LD(int, DAT_009a1fe8 + 0x1f + iVar5) + 1) =
                        (char)(LD(int, (int)DAT_00521f1c + (unsigned int)*(unsigned char *)(iVar6 + 3) * 0x1a) >> 9);
                } else {
                    FUN_0041f300(uVar18, (char)0x82);
                }
                FUN_00433d30(local_6, (unsigned short)*(unsigned char *)(DAT_009a1fe8 + 0x16 + iVar5), 2);
                break;
            case 6:
            case 7:
                if (DAT_0051a44f[*(unsigned char *)(iVar6 + 3)] != '\0') {
                    cVar11 = -0x7f;
                    goto LAB_00445a3d;
                }
                *(unsigned char *)(iVar6 + 3) = 0;
                goto switchD_00445bc5_caseD_5;
            default:
                *(unsigned char *)(iVar6 + 3) = 0;
                goto switchD_00445bc5_caseD_5;
            case 0xb:
                bVar10 = FUN_00425cb0();
                cVar11 = (char)((bVar10 & 1) + 0x7f);
LAB_00445a3d:
                FUN_0041f300(uVar18, cVar11);
            }
LAB_00445a45:
            if (DAT_009ec5d0 == -1) {
                if ((DAT_009d6940[1] != '\0') && (LD(unsigned int, DAT_009a1fe8 + 0x1b + iVar5) == DAT_009ecc34)) {
                    FUN_0042b290();
                }
                ST(unsigned int, XD + 6, LD(unsigned int, iVar6 + 4));
                ST(unsigned int, XD + 10, LD(unsigned int, iVar6 + 8));
                ST(unsigned int, XD + 0xe, LD(unsigned int, iVar6 + 0xc));
            }
            sVar12 = LD(short, XD + 0x77);
            psVar20 = XD + 0x77;
            if (sVar12 != 0) {
                if (((int)uVar18 < DAT_009a1fe0) &&
                    (iVar19 = LD(int, LD(int, DAT_009a1fe8 + 0x1f + iVar5) + 0xa9),
                     LD(short, iVar19 + 0x70) != -1)) {
                    if (sVar12 < 0) {
                        ST(short, psVar20, (short)((((unsigned int)DAT_009ec5fe << 0xc) / 0xec) + (unsigned int)(int)sVar12));
                        psVar20 = XD + 0x77;
                        if (-1 < LD(short, psVar20)) {
LAB_00445b29:
                            ST(short, psVar20, 0);
                            sVar12 = LD(short, XD + 0x77);
                            ST(unsigned short, iVar19 + 0x70, 0xffff);
                        }
                    } else {
                        ST(short, psVar20, (short)((int)((unsigned int)DAT_009ec5fe << 0xc) / -0xec) + sVar12);
                        psVar20 = XD + 0x77;
                        if (LD(short, psVar20) < 1) goto LAB_00445b29;
                    }
                }
                ST(short, iVar6 + 0x14,
                   (short)((unsigned int)LD(short, iVar6 + 0x14) +
                           (unsigned int)((int)((int)((unsigned int)(int)sVar12 * (unsigned int)DAT_009ec5fe) / 0xec) * 4)));
                *(unsigned char *)(XD + 0x92) = 1;
            }
            uVar22 = 0;
            iVar19 = XD;
            if (LD(unsigned short, iVar19 + 0x79) != 0) {
                uVar22 = (unsigned int)((int)((unsigned int)((int)((unsigned int)LD(unsigned short, iVar19 + 0x79) * 0x1234u + 0x80u) >> 8) *
                                              (unsigned int)DAT_009ec5fe) / 0xec) & 0xffff;
            }
            switch (uVar2) {
            case 0:
            case 1:
            case 2:
            case 3:
            case 4:
                iVar14 = LD(int, iVar19 + 0xa9);
                if (*(char *)(iVar19 + 0x8b) != '\0') {
                    FUN_00427bd0(LD(short, iVar6 + 0x12), LD(short, iVar6 + 0x10), iVar6);
                }
                DAT_009a20b0 = MUL15(LD(short, XD + 0x8c), uVar22);
                DAT_009cd26c = MUL15(LD(short, XD + 0x8e), uVar22);
                DAT_009fd438 = MUL15(LD(short, XD + 0x90), uVar22);
                if (LD(short, iVar14 + 0x1e5) != 0) {
                    if (*(char *)(iVar14 + 0x5e) == '\x12') {
                        uVar18 = 0xfa;
                    } else if (*(char *)(iVar14 + 0x5e) == '\x1e') {
                        uVar18 = 0x2ee;
                    } else {
                        uVar18 = (unsigned int)LD(unsigned short, (int)DAT_0051c593 + (unsigned int)*(unsigned char *)(iVar14 + 4) * 0x101);
                    }
                    iVar19 = LD(int, iVar14 + 0xb1);
                    sVar12 = (short)uVar18;
                    if (iVar19 != 0) {
                        if ((0u - (unsigned int)iVar19) == uVar18 || (int)(0u - uVar18) < iVar19) {
                            sVar24 = sVar12;
                            if (iVar19 <= (int)uVar18) {
                                sVar24 = (short)iVar19;
                            }
                        } else {
                            sVar24 = (short)(0u - (unsigned int)(int)sVar12);
                        }
                        iVar15 = (int)((unsigned int)(int)sVar24 * (unsigned int)DAT_009ec5fe) / 0xec;
                        if (iVar15 == 0) {
                            iVar15 = iVar19;
                        }
                        ST(int, iVar14 + 0xb1, (unsigned int)iVar19 - (unsigned int)iVar15);
                        DAT_009a20b0 = DAT_009a20b0 + (unsigned int)iVar15;
                    }
                    iVar19 = LD(int, iVar14 + 0xb5);
                    if (iVar19 != 0) {
                        if ((0u - (unsigned int)iVar19) == uVar18 || (int)(0u - uVar18) < iVar19) {
                            sVar24 = sVar12;
                            if (iVar19 <= (int)uVar18) {
                                sVar24 = (short)iVar19;
                            }
                        } else {
                            sVar24 = (short)(0u - (unsigned int)(int)sVar12);
                        }
                        iVar15 = (int)((unsigned int)(int)sVar24 * (unsigned int)DAT_009ec5fe) / 0xec;
                        if (iVar15 == 0) {
                            iVar15 = iVar19;
                        }
                        ST(int, iVar14 + 0xb5, (unsigned int)iVar19 - (unsigned int)iVar15);
                        DAT_009cd26c = DAT_009cd26c + (unsigned int)iVar15;
                    }
                    uVar22 = LD(unsigned int, iVar14 + 0xb9);
                    if (uVar22 != 0) {
                        if ((0u - uVar22) == uVar18 || (int)(0u - uVar18) < (int)uVar22) {
                            if ((int)uVar22 <= (int)uVar18) {
                                uVar18 = uVar22 & 0xffff;
                            }
                        } else {
                            uVar18 = (unsigned int)(unsigned short)(0u - (unsigned int)(int)sVar12);
                        }
                        uVar18 = (unsigned int)((int)((unsigned int)(int)(short)uVar18 * (unsigned int)DAT_009ec5fe) / 0xec);
                        if (uVar18 == 0) {
                            uVar18 = uVar22;
                        }
                        ST(unsigned int, iVar14 + 0xb9, uVar22 - uVar18);
                        DAT_009fd438 = DAT_009fd438 + uVar18;
                    }
                }
                FUN_004464c0(iVar6);
                if (LD(unsigned short, iVar14 + 0x66) != 0xffff) {
                    iVar15 = (unsigned int)LD(unsigned short, iVar14 + 0x66) * 0x23;
                    iVar16 = DAT_009a1fe8 + iVar15;
                    iVar19 = LD(int, iVar16 + 0x1f);
                    if (iVar19 != 0) {
                        uVar2 = *(unsigned char *)(LD(int, iVar19 + 0xa9) + 4);
                        uVar4 = (unsigned short)(((unsigned int)uVar2 << 8) | uVar2);
                        if (*(unsigned char *)(iVar16 + 2) < 3) {
                            sVar12 = (short)((unsigned int)LD(short, (int)DAT_0051c63d + (unsigned int)*(unsigned char *)(iVar14 + 4) * 0x101) -
                                             (unsigned int)LD(short, (int)DAT_0051c639 + uVar4));
                        } else if (*(unsigned char *)(DAT_009a1fe8 + 2 + iVar5) < 3) {
                            sVar12 = (short)((unsigned int)LD(short, (int)DAT_0051c63f + (unsigned int)*(unsigned char *)(iVar14 + 4) * 0x101) -
                                             (unsigned int)LD(short, (int)DAT_0051c639 + uVar4));
                        } else {
                            sVar12 = (short)((unsigned int)LD(short, (int)DAT_0051c63f + (unsigned int)*(unsigned char *)(iVar14 + 4) * 0x101) -
                                             (unsigned int)LD(short, (int)DAT_0051c63b + uVar4));
                        }
                        FUN_004035d0(iVar6, 0, sVar12, LD(short, (int)DAT_0051c637 + uVar4));
                        iVar15 = DAT_009a1fe8 + iVar15;
                        ST(unsigned int, LD(int, iVar15 + 0x1f) + 6, LD(unsigned int, iVar15 + 4));
                        ST(unsigned int, LD(int, iVar15 + 0x1f) + 10, LD(unsigned int, iVar15 + 8));
                        ST(unsigned int, LD(int, iVar15 + 0x1f) + 0xe, LD(unsigned int, iVar15 + 0xc));
                        ST(unsigned int, iVar15 + 4, LD(unsigned int, iVar6 + 4) + DAT_009d8a58);
                        ST(unsigned int, iVar15 + 8, LD(unsigned int, iVar6 + 8) + DAT_009d8a54);
                        ST(unsigned int, iVar15 + 0xc, LD(unsigned int, iVar6 + 0xc) + DAT_009d8b60);
                    }
                }
                break;
            case 6:
            case 7:
                iVar5 = LD(int, iVar19 + 0xa5);
                bVar10 = *(unsigned char *)(iVar5 + 1);
                if ((bVar10 != 0) && (LD(unsigned short, iVar5 + 4) != 0xffff)) {
                    uVar25 = (unsigned int)LD(unsigned short, iVar5 + 4);
                    if (((int)uVar25 < DAT_00a07ce8) &&
                        ((psVar20 = uVar25 * 0x23 + DAT_009a1fe8,
                          *(char *)(psVar20 + 3) == '\0' || (LD(short, iVar5 + 6) != LD(short, psVar20))))) {
                        FUN_0041f300(uVar18, (char)0x81);
                        break;
                    }
                    bVar17 = 0;
                    uVar4 = LD(unsigned short, iVar5 + 2);
                    bVar3 = (unsigned char)s_Unable_to_quantize_image_00521c20[*(unsigned char *)(iVar6 + 3) + 7];
                    if ((((int)uVar25 < DAT_009a1fe0) &&
                         (iVar19 = LD(int, LD(int, uVar25 * 0x23 + 0x1f + DAT_009a1fe8) + 0xa9),
                          *(char *)(iVar19 + 0x23d) == '\x03')) && (*(char *)(iVar19 + 0x241) != '\0')) {
                        bVar17 = 1;
                    }
                    if (!bVar17) {
                        FUN_00458e20(uVar25, 0);
                        if (((int)uVar25 < DAT_009a1fe0) &&
                            (iVar19 = uVar25 * 0x23 + DAT_009a1fe8, *(char *)LD(int, iVar19 + 0x1f) == '\0')) {
                            if (uVar4 == 0xffff) {
                                uVar13 = (unsigned int)*(unsigned char *)(iVar19 + 3);
                                iVar14 = FUN_004ae640(uVar13, 0);
                                iVar14 = (int)(0u - (unsigned int)iVar14);
                                iVar15 = FUN_004ae6e0(uVar13, 0);
                                uVar25 = 0;
                            } else {
                                uVar25 = (unsigned int)uVar4;
                                uVar13 = (unsigned int)*(unsigned char *)(iVar19 + 3);
                                iVar14 = FUN_004ae640(uVar13, uVar25);
                                iVar14 = (int)(0u - (unsigned int)iVar14);
                                iVar15 = FUN_004ae6e0(uVar13, uVar25);
                            }
                            iVar16 = FUN_004ae5a0(uVar13, uVar25);
                            FUN_004037b0(iVar19, iVar16, iVar15, iVar14);
                            DAT_009d8a58 = DAT_009d8a58 + DAT_00a07cdc;
                            DAT_009d8a54 = DAT_009d8a54 + DAT_00a07cd8;
                            DAT_009d8b60 = DAT_009d8b60 + DAT_00a07cd4;
                        } else {
                            DAT_009d8a58 = DAT_00a07cdc;
                            DAT_009d8a54 = DAT_00a07cd8;
                            DAT_009d8b60 = DAT_00a07cd4;
                        }
                        FUN_00458e20(uVar18, 0);
                        DAT_009d8a58 = DAT_009d8a58 - DAT_00a07cdc;
                        DAT_009d8a54 = DAT_009d8a54 - DAT_00a07cd8;
                        DAT_009d8b60 = DAT_009d8b60 - DAT_00a07cd4;
                        FUN_0046a8d0(DAT_009d8a58, DAT_009d8a54, DAT_009d8b60);
                        sVar24 = (short)((unsigned int)DAT_009a8070 - (unsigned int)LD(short, iVar6 + 0x10));
                        iVar19 = (unsigned int)(unsigned short)((unsigned short)bVar10 + (unsigned short)bVar3) * 2;
                        uVar18 = (unsigned int)((int)((unsigned int)LD(unsigned short, (int)DAT_00521cc8 + iVar19) * (unsigned int)DAT_009ec5fe) / 0xec);
                        sVar12 = (short)uVar18;
                        sStack_8 = sVar24;
                        if (sVar24 < 0) {
                            sStack_8 = (short)(0u - (unsigned int)(int)sVar24);
                        }
                        if ((int)(uVar18 & 0xffff) < (int)sStack_8) {
                            if (sVar24 < 0) {
                                sVar12 = (short)(0u - (unsigned int)(int)sVar12);
                            }
                            ST(short, iVar6 + 0x10, (unsigned int)LD(short, iVar6 + 0x10) + (unsigned int)(int)sVar12);
                            puVar26 = XD + 0x79;
                            if (200 < LD(unsigned short, puVar26)) {
                                ST(unsigned short, puVar26,
                                   (unsigned int)LD(unsigned short, puVar26) +
                                   (unsigned int)(int)(short)((int)((unsigned int)LD(unsigned short, (int)DAT_00521d20 + iVar19) *
                                                                    (unsigned int)DAT_009ec5fe) / -0xec));
                                puVar26 = XD + 0x79;
                                if (LD(unsigned short, puVar26) < 200) {
                                    ST(unsigned short, puVar26, 200);
                                }
                            }
                        } else {
                            ST(short, iVar6 + 0x10, DAT_009a8070);
                            psVar20 = XD + 0x79;
                            if (LD(unsigned short, XD + 0x79) < LD(unsigned short, iVar5 + 8)) {
                                ST(short, psVar20,
                                   (unsigned int)LD(short, psVar20) +
                                   (unsigned int)(int)(short)((int)((unsigned int)LD(unsigned short, (int)DAT_00521d20 + iVar19) *
                                                                    (unsigned int)DAT_009ec5fe) / 0xec));
                            }
                        }
                        sVar24 = (short)((unsigned int)DAT_00a080f0 - (unsigned int)LD(short, iVar6 + 0x12));
                        uVar18 = (unsigned int)((int)((unsigned int)LD(unsigned short, (int)DAT_00521cc8 + iVar19) * (unsigned int)DAT_009ec5fe) / 0xec);
                        sVar12 = (short)uVar18;
                        sStack_8 = sVar24;
                        if (sVar24 < 0) {
                            sStack_8 = (short)(0u - (unsigned int)(int)sVar24);
                        }
                        if ((int)(uVar18 & 0xffff) < (int)sStack_8) {
                            if (sVar24 < 0) {
                                sVar12 = (short)(0u - (unsigned int)(int)sVar12);
                            }
                            ST(short, iVar6 + 0x12, (unsigned int)LD(short, iVar6 + 0x12) + (unsigned int)(int)sVar12);
                        } else {
                            ST(short, iVar6 + 0x12, DAT_00a080f0);
                        }
                        *(unsigned char *)(XD + 0x92) = 1;
                        *(unsigned char *)(XD + 0x8b) = *(unsigned char *)(XD + 0x92);
                        FUN_00427bd0(LD(short, iVar6 + 0x12), LD(short, iVar6 + 0x10), iVar6);
                        ST(short, XD + 0x8c, DAT_009a8da4);
                        ST(short, XD + 0x8e, DAT_009a8d84);
                        ST(short, XD + 0x90, DAT_009a8d88);
                    }
                }
                if (*(char *)(XD + 0x8b) != '\0') {
                    FUN_00427bd0(LD(short, iVar6 + 0x12), LD(short, iVar6 + 0x10), iVar6);
                }
                DAT_009a20b0 = MUL15(LD(short, XD + 0x8c), uVar22);
                DAT_009cd26c = MUL15(LD(short, XD + 0x8e), uVar22);
                DAT_009fd438 = MUL15(LD(short, XD + 0x90), uVar22);
                goto LAB_00446437;
            case 0xb:
            case 0xd:
                if (*(char *)(iVar19 + 0x8b) != '\0') {
                    FUN_00427bd0(LD(short, iVar6 + 0x12), LD(short, iVar6 + 0x10), iVar6);
                }
                DAT_009a20b0 = MUL15(LD(short, XD + 0x8c), uVar22);
                DAT_009cd26c = MUL15(LD(short, XD + 0x8e), uVar22);
                DAT_009fd438 = MUL15(LD(short, XD + 0x90), uVar22);
LAB_00446437:
                FUN_004464c0(iVar6);
            }
switchD_00445bc5_caseD_5:
            local_6 = local_6 + 1;
        } while ((int)(unsigned int)local_6 < DAT_00a07ce8);
    }
    DAT_009d8112 = sVar9;
    DAT_009ec5fe = uVar23;
    return;
}

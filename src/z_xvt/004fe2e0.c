/* SPDX-License-Identifier: MIT */
/* z_xvt 004fe2e0 */
/* Globals at addresses that are not multiples of 4 are declared as byte
   arrays and read through __builtin_memcpy into a local of the right width. */
#define LD(dst, p) __builtin_memcpy(&(dst), (p), sizeof(dst))

extern unsigned int DAT_00654c60[10];
extern unsigned int DAT_00654b98[10];
extern unsigned int DAT_00654bd0[10];
extern unsigned int DAT_00654bb8[];
extern unsigned int DAT_00654c18[10];
extern unsigned int DAT_00654c40[];
extern unsigned int DAT_00654c88[8];
extern unsigned int DAT_00654ca8[];
extern unsigned int DAT_00654cb8[8];
extern unsigned int DAT_00654cd8[];
extern unsigned int DAT_00654cf0[8];
extern unsigned int DAT_00654d10[10];
extern unsigned int DAT_00654d38[];
extern int DAT_00654bfc;
extern int DAT_00654bf8;
extern int DAT_00654d44;

extern unsigned char DAT_0081f7da[];
extern unsigned char DAT_0081f7de[];
extern unsigned char DAT_0081f7e6[];
extern unsigned char DAT_0081fa9a[];
extern unsigned char DAT_0081fa9e[];
extern unsigned char DAT_0081fa7e[];
extern unsigned char DAT_0081fa82[];
extern unsigned char DAT_0081d32e[];
extern unsigned char DAT_0081d332[];
extern unsigned char DAT_0081e0a6[];
extern unsigned char DAT_0081e0c6[];
extern unsigned char DAT_0081e186[];
extern unsigned char DAT_0081e266[];
extern unsigned char DAT_0081e286[];
extern unsigned char DAT_0081e346[];
extern unsigned char DAT_0081bfba[4];
extern unsigned char DAT_0081bfb6[4];
extern unsigned char DAT_007438d8[];
extern unsigned char DAT_007438e4[];
extern int DAT_0081b850;
extern char DAT_0085e7c4;
extern short DAT_0075766c;
extern unsigned int DAT_00757a28;
extern int DAT_007579d0[];

unsigned int __stdcall FUN_004fe2e0(void)
{
    unsigned char bVar1;
    unsigned int uVar2;
    int iVar3;
    unsigned int *puVar4;
    unsigned int uVar5;
    int iVar6;
    int iVar7;
    unsigned int uVar8;
    unsigned int *piVar9;
    int iVar10;
    unsigned int uVar11;
    unsigned int uVar12;
    unsigned char *pbVar13;
    unsigned int uVar14;
    unsigned int *piVar15;
    unsigned int *puVar16;
    unsigned int uVar17;
    unsigned int uStack_10;
    unsigned char *pbStack_8;
    int iStack_4;
    unsigned char *pb;
    unsigned char *pb2;
    int bfba;
    int bfb6;
    int a, b, c, d;
    unsigned int ua, ub;
    unsigned int slot;
    unsigned int cls;

    puVar16 = DAT_00654c60;
    for (iVar6 = 10; iVar6 != 0; iVar6 = iVar6 - 1) {
        *puVar16 = 0;
        puVar16 = puVar16 + 1;
    }
    puVar16 = DAT_00654b98;
    for (iVar6 = 10; iVar6 != 0; iVar6 = iVar6 - 1) {
        *puVar16 = 0;
        puVar16 = puVar16 + 1;
    }
    puVar16 = DAT_00654bd0;
    for (iVar6 = 10; iVar6 != 0; iVar6 = iVar6 - 1) {
        *puVar16 = 0;
        puVar16 = puVar16 + 1;
    }
    DAT_00654bfc = 0;
    iVar6 = 0;
    pb = DAT_0081f7da;
    DAT_00654bf8 = 0;
    do {
        LD(a, pb + 4);
        if (a != 0) {
            iVar6 = iVar6 + 1;
            LD(slot, pb);
            if (DAT_00654c60[DAT_007438d8[slot * 0x562u]] == 0) {
                DAT_00654bfc = DAT_00654bfc + 1;
            }
            LD(slot, pb);
            DAT_00654c60[DAT_007438d8[slot * 0x562u]] = 1;
        }
        pb = pb + 0x58;
    } while (pb < DAT_0081fa9a);

    LD(bfba, DAT_0081bfba);
    if ((bfba == 1) || (bfba == 2)) {
        if (((DAT_0085e7c4 != '\0') || ((DAT_0081b850 == 2 || (iVar6 == 1)))) &&
            (iVar6 = (int)DAT_0075766c, iVar6 != 0)) {
            pbVar13 = DAT_007438d8;
            do {
                if (pbVar13[0xc] != 0) {
                    bVar1 = *pbVar13;
                    if (DAT_00654c60[bVar1] == 0) {
                        DAT_00654bfc = DAT_00654bfc + 1;
                        DAT_00654bd0[bVar1] = 1;
                    }
                    DAT_00654c60[bVar1] = 1;
                }
                pbVar13 = pbVar13 + 0x562;
                iVar6 = iVar6 + -1;
            } while (iVar6 != 0);
        }
        pb = DAT_0081d32e;
        puVar4 = DAT_00654b98;
        do {
            LD(a, pb);
            *puVar4 = (unsigned int)(a != -1);
            pb = pb + 20;
            puVar4 = puVar4 + 1;
        } while (puVar4 < DAT_00654bb8);
    }
    puVar16 = DAT_00654d10;
    for (iVar6 = 10; iVar6 != 0; iVar6 = iVar6 - 1) {
        *puVar16 = 0xffffffff;
        puVar16 = puVar16 + 1;
    }
    puVar16 = DAT_00654c18;
    for (iVar6 = 10; iVar6 != 0; iVar6 = iVar6 - 1) {
        *puVar16 = 0xffffffff;
        puVar16 = puVar16 + 1;
    }
    puVar16 = DAT_00654cf0;
    for (iVar6 = 8; iVar6 != 0; iVar6 = iVar6 - 1) {
        *puVar16 = 0xffffffff;
        puVar16 = puVar16 + 1;
    }
    puVar16 = DAT_00654cb8;
    for (iVar6 = 8; iVar6 != 0; iVar6 = iVar6 - 1) {
        *puVar16 = 0xffffffff;
        puVar16 = puVar16 + 1;
    }
    puVar16 = DAT_00654c88;
    for (iVar6 = 8; iVar6 != 0; iVar6 = iVar6 - 1) {
        *puVar16 = 0xffffffff;
        puVar16 = puVar16 + 1;
    }

    uStack_10 = 0;
    do {
        if (DAT_00654c60[uStack_10] != 0) {
            iVar6 = 0;
            puVar4 = DAT_00654c18;
            uVar17 = uStack_10;
            do {
                uVar11 = *puVar4;
                if (uVar11 == 0xffffffff) {
                    DAT_00654c18[iVar6] = uVar17;
                    break;
                }
                LD(a, DAT_0081fa7e + uVar11 * 28u);
                LD(b, DAT_0081fa7e + uVar17 * 28u);
                if ((bfba == 1) || (bfba == 2)) {
                    if (a < b)
                        goto LAB_004fe4f1;
                }
                else {
                    LD(ua, DAT_0081fa82 + uVar11 * 28u);
                    LD(ub, DAT_0081fa82 + uVar17 * 28u);
                    if (((ua == 0) && (ub == 1)) || (a < b)) {
LAB_004fe4f1:
                        *puVar4 = uVar17;
                        uVar17 = uVar11;
                    }
                }
                puVar4 = puVar4 + 1;
                iVar6 = iVar6 + 1;
            } while (puVar4 < DAT_00654c40);
        }
        uStack_10 = uStack_10 + 1;
    } while (uStack_10 < 10);

    iVar6 = 0;
    piVar15 = DAT_00654c18;
    piVar9 = DAT_00654c60;
    LD(bfb6, DAT_0081bfb6);
    do {
        if ((*piVar9 != 0) && (iVar10 = iVar6, (int)*piVar15 == bfb6)) break;
        piVar15 = piVar15 + 1;
        piVar9 = piVar9 + 1;
        iVar6 = iVar6 + 1;
        iVar10 = DAT_00654bf8;
    } while (piVar9 < DAT_00654c88);
    DAT_00654bf8 = iVar10;

    if ((bfba == 1) || (bfba == 2)) {
        uVar17 = 0;
        do {
            if (DAT_00654b98[uVar17] != 0) {
                iVar6 = 0;
                puVar4 = DAT_00654d10;
                uVar11 = uVar17;
                do {
                    uVar2 = *puVar4;
                    if (uVar2 == 0xffffffff) {
                        DAT_00654d10[iVar6] = uVar11;
                        break;
                    }
                    LD(a, DAT_0081d332 + uVar2 * 20u);
                    LD(b, DAT_0081d332 + uVar11 * 20u);
                    if (a < b) {
                        *puVar4 = uVar11;
                        uVar11 = uVar2;
                    }
                    puVar4 = puVar4 + 1;
                    iVar6 = iVar6 + 1;
                } while (puVar4 < DAT_00654d38);
            }
            uVar17 = uVar17 + 1;
        } while (uVar17 < 10);
    }

    uStack_10 = 0;
    pbStack_8 = DAT_0081f7de;
    do {
        LD(a, pbStack_8);
        if (a != 0) {
            iStack_4 = 0;
            piVar15 = DAT_00654c88;
            uVar17 = uStack_10;
            do {
                iVar6 = (int)*piVar15;
                if (iVar6 == -1) {
                    DAT_00654c88[iStack_4] = uVar17;
                    break;
                }
                LD(slot, DAT_0081f7da + uVar17 * 0x58u);
                cls = DAT_007438d8[slot * 0x562u];
                LD(ua, DAT_0081fa82 + cls * 28u);
                LD(slot, DAT_0081f7da + (unsigned int)iVar6 * 0x58u);
                cls = DAT_007438d8[slot * 0x562u];
                LD(ub, DAT_0081fa82 + cls * 28u);
                LD(c, DAT_0081f7e6 + (unsigned int)iVar6 * 0x58u);
                LD(d, DAT_0081f7e6 + uVar17 * 0x58u);
                if (((ua == 1) && (ub == 0)) ||
                    (((ua != 0 || (ub != 1)) && (c < d)))) {
                    *piVar15 = uVar17;
                    uVar17 = (unsigned int)iVar6;
                }
                piVar15 = piVar15 + 1;
                iStack_4 = iStack_4 + 1;
            } while (piVar15 < DAT_00654ca8);
        }
        pbStack_8 = pbStack_8 + 0x58;
        uStack_10 = uStack_10 + 1;
    } while (pbStack_8 < DAT_0081fa9e);

    iVar6 = 0;
    pb = DAT_0081f7de;
    do {
        LD(a, pb);
        if (a != 0) {
            iVar7 = 0;
            piVar9 = DAT_00654cb8;
            iVar10 = iVar6;
            do {
                iVar3 = (int)*piVar9;
                if (iVar3 == -1) {
                    DAT_00654cb8[iVar7] = (unsigned int)iVar10;
                    break;
                }
                LD(ua, DAT_0081e0a6 + (unsigned int)iVar3 * 4u);
                LD(ub, DAT_0081e0a6 + (unsigned int)iVar10 * 4u);
                if (ua < ub) {
                    *piVar9 = (unsigned int)iVar10;
                    iVar10 = iVar3;
                }
                piVar9 = piVar9 + 1;
                iVar7 = iVar7 + 1;
            } while (piVar9 < DAT_00654cd8);
        }
        pb = pb + 0x58;
        iVar6 = iVar6 + 1;
    } while (pb < DAT_0081fa9e);

    if ((bfba == 1) || (bfba == 2)) {
        uStack_10 = 0;
        uVar17 = (unsigned int)(int)DAT_0075766c;
        if (uVar17 != 0) {
            pbStack_8 = DAT_007438e4;
            do {
                uVar11 = uStack_10 + 8;
                if (*pbStack_8 != '\0') {
                    iVar6 = 0;
                    pb2 = DAT_0081f7de;
                    do {
                        LD(a, pb2);
                        LD(b, pb2 - 4);
                        if ((a != 0) && ((unsigned int)b == uStack_10)) break;
                        pb2 = pb2 + 0x58;
                        iVar6 = iVar6 + 1;
                    } while (pb2 < DAT_0081fa9e);
                    if (iVar6 == 8) {
                        iStack_4 = 0;
                        puVar4 = DAT_00654cb8;
                        do {
                            uVar2 = *puVar4;
                            if (uVar2 == 0xffffffff) {
                                DAT_00654cb8[iStack_4] = uVar11;
                                break;
                            }
                            if (uVar11 < 8) {
                                LD(uVar5, DAT_0081e0a6 + uVar11 * 4u);
                                LD(uVar14, DAT_0081e0c6 + uVar11 * 4u);
                            }
                            else {
                                LD(uVar5, DAT_0081e0c6 + uVar11 * 4u);
                                LD(uVar14, DAT_0081e186 + uVar11 * 4u);
                            }
                            if ((int)uVar2 < 8) {
                                LD(uVar8, DAT_0081e0a6 + uVar2 * 4u);
                                LD(uVar12, DAT_0081e0c6 + uVar2 * 4u);
                            }
                            else {
                                LD(uVar8, DAT_0081e0c6 + uVar2 * 4u);
                                LD(uVar12, DAT_0081e186 + uVar2 * 4u);
                            }
                            if ((uVar8 < uVar5) || ((uVar8 == uVar5 && (uVar12 < uVar14)))) {
                                *puVar4 = uVar11;
                                uVar11 = uVar2;
                            }
                            puVar4 = puVar4 + 1;
                            iStack_4 = iStack_4 + 1;
                        } while (puVar4 < DAT_00654cd8);
                    }
                }
                pbStack_8 = pbStack_8 + 0x562;
                uStack_10 = uStack_10 + 1;
            } while (uStack_10 < uVar17);
        }
    }

    iVar6 = 0;
    pb = DAT_0081f7de;
    do {
        LD(a, pb);
        if (a != 0) {
            iVar7 = 0;
            piVar9 = DAT_00654cf0;
            iVar10 = iVar6;
            do {
                iVar3 = (int)*piVar9;
                if (iVar3 == -1) {
                    DAT_00654cf0[iVar7] = (unsigned int)iVar10;
                    break;
                }
                LD(ua, DAT_0081e266 + (unsigned int)iVar3 * 4u);
                LD(ub, DAT_0081e266 + (unsigned int)iVar10 * 4u);
                if (ua < ub) {
                    *piVar9 = (unsigned int)iVar10;
                    iVar10 = iVar3;
                }
                piVar9 = piVar9 + 1;
                iVar7 = iVar7 + 1;
            } while (piVar9 < DAT_00654d10);
        }
        pb = pb + 0x58;
        iVar6 = iVar6 + 1;
    } while (pb < DAT_0081fa9e);

    if ((bfba == 1) || (bfba == 2)) {
        uStack_10 = 0;
        uVar17 = (unsigned int)(int)DAT_0075766c;
        if (uVar17 != 0) {
            pbStack_8 = DAT_007438e4;
            do {
                uVar11 = uStack_10 + 8;
                if (*pbStack_8 != '\0') {
                    iVar6 = 0;
                    pb2 = DAT_0081f7de;
                    do {
                        LD(a, pb2);
                        LD(b, pb2 - 4);
                        if ((a != 0) && ((unsigned int)b == uStack_10)) break;
                        pb2 = pb2 + 0x58;
                        iVar6 = iVar6 + 1;
                    } while (pb2 < DAT_0081fa9e);
                    if (iVar6 == 8) {
                        iStack_4 = 0;
                        puVar4 = DAT_00654cf0;
                        do {
                            uVar2 = *puVar4;
                            if (uVar2 == 0xffffffff) {
                                DAT_00654cf0[iStack_4] = uVar11;
                                break;
                            }
                            if (uVar11 < 8) {
                                LD(uVar5, DAT_0081e266 + uVar11 * 4u);
                                LD(uVar14, DAT_0081e286 + uVar11 * 4u);
                            }
                            else {
                                LD(uVar5, DAT_0081e286 + uVar11 * 4u);
                                LD(uVar14, DAT_0081e346 + uVar11 * 4u);
                            }
                            if ((int)uVar2 < 8) {
                                LD(uVar8, DAT_0081e266 + uVar2 * 4u);
                                LD(uVar12, DAT_0081e286 + uVar2 * 4u);
                            }
                            else {
                                LD(uVar8, DAT_0081e286 + uVar2 * 4u);
                                LD(uVar12, DAT_0081e346 + uVar2 * 4u);
                            }
                            if ((uVar8 < uVar5) || ((uVar8 == uVar5 && (uVar12 < uVar14)))) {
                                *puVar4 = uVar11;
                                uVar11 = uVar2;
                            }
                            puVar4 = puVar4 + 1;
                            iStack_4 = iStack_4 + 1;
                        } while (puVar4 < DAT_00654d10);
                    }
                }
                pbStack_8 = pbStack_8 + 0x562;
                uStack_10 = uStack_10 + 1;
            } while (uStack_10 < uVar17);
        }
    }

    uVar17 = 0;
    DAT_00654d44 = 0;
    if (bfba == 1) {
        if (DAT_00757a28 != 0) {
            int *pi = DAT_007579d0;
            do {
                if (1 < *pi) break;
                pi = pi + 1;
                uVar17 = uVar17 + 1;
            } while (uVar17 < DAT_00757a28);
            if (uVar17 != DAT_00757a28) {
                DAT_00654d44 = 0;
                return 1;
            }
        }
        DAT_00654d44 = 1;
    }
    return 1;
}

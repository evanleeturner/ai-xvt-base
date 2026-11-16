/* SPDX-License-Identifier: MIT */
/* z_xvt 00431d70 */
/* Globals keep the decompiler's placeholders. Array globals are indexed as
   Ghidra's (&DAT_xxx)[i] expressions were. Types below are inferred from use. */
extern unsigned int DAT_005251b0;
extern int          DAT_005251b4;
extern int          DAT_00525558;
extern unsigned int DAT_005ef21c;
extern int          DAT_007330d4;
extern unsigned int DAT_00739290;
extern unsigned int DAT_00739298[];   /* 0x84-dword copy buffer; [0] and [1] are read after the copy */
extern unsigned char DAT_007392a8;
extern char         DAT_0085e78f;

extern unsigned int DAT_00539a08[];
extern unsigned int DAT_00539a0c[];
extern unsigned int DAT_00539a10[];
extern unsigned char DAT_00539a14[];
extern char         DAT_00539a15[];
extern unsigned char DAT_00539a16[];
extern unsigned char DAT_00539a17[];
extern int          DAT_00539a18[];
extern unsigned int DAT_00539a1c[];

extern int          DAT_0073379c[];
extern int          DAT_007337a4[];
extern int          DAT_007337ac[];
extern DWORD        DAT_007339bc[];
extern int          DAT_007339c0[];
extern unsigned char DAT_007339c8[];

DWORD __stdcall timeGetTime(void);
void FUN_0042ff10(void);
void FUN_00433160(void);
unsigned int FUN_00433540(unsigned int param_1);
unsigned int FUN_00433340(unsigned int param_1, unsigned int param_2, unsigned int param_3,
                          unsigned int param_4, unsigned int param_5);
void FUN_00432a30(int param_1, unsigned int *param_2, int param_3);
int FUN_00433450(unsigned int param_1);

unsigned char *FUN_00431d70(unsigned int *param_1, unsigned int *param_2)
{
    unsigned char bVar1;
    char cVar2;
    int bVar3;
    unsigned int uVar4;
    unsigned int uVar5;
    unsigned int uVar6;
    int iVar7;
    unsigned int uVar8;
    DWORD DVar9;
    unsigned int uVar10;
    unsigned char bVar11;
    int iVar12;
    unsigned char *pbVar13;
    int iVar14;
    unsigned char *pbVar15;
    unsigned int uVar16;
    unsigned int uVar17;
    int iVar18;
    unsigned int *puVar19;
    unsigned int *piVar20;
    unsigned int *puVar21;
    unsigned int *piVar22;
    int iVar23;
    unsigned int uStack_32c;
    unsigned int uStack_328;
    unsigned int uStack_314;
    int iStack_310;
    unsigned char abStack_2f0[40];
    unsigned char abStack_2c8[40];
    unsigned char abStack_2a0[120];
    unsigned char abStack_228[40];
    unsigned int uStack_200[3]; /* [0]=200, [1]=1fc, [2]=1f8 */

    FUN_0042ff10();
    FUN_00433160();
    if (DAT_005251b4 == 0) {
        return (unsigned char *)0;
    }
    pbVar13 = abStack_2c8;
    for (iVar12 = 10; iVar12 != 0; iVar12 = iVar12 + -1) {
        pbVar13[0] = 0;
        pbVar13[1] = 0;
        pbVar13[2] = 0;
        pbVar13[3] = 0;
        pbVar13 = pbVar13 + 4;
    }
    pbVar13 = abStack_2f0;
    for (iVar12 = 10; iVar12 != 0; iVar12 = iVar12 + -1) {
        pbVar13[0] = 0;
        pbVar13[1] = 0;
        pbVar13[2] = 0;
        pbVar13[3] = 0;
        pbVar13 = pbVar13 + 4;
    }
    pbVar13 = abStack_228;
    for (iVar12 = 10; iVar12 != 0; iVar12 = iVar12 + -1) {
        pbVar13[0] = 0x5a;
        pbVar13[1] = 0x5a;
        pbVar13[2] = 0x5a;
        pbVar13[3] = 0x5a;
        pbVar13 = pbVar13 + 4;
    }
    pbVar13 = abStack_2a0;
    for (iVar12 = 0x1e; iVar12 != 0; iVar12 = iVar12 + -1) {
        pbVar13[0] = 0;
        pbVar13[1] = 0;
        pbVar13[2] = 0;
        pbVar13[3] = 0;
        pbVar13 = pbVar13 + 4;
    }
    uStack_32c = DAT_005251b0;
    uVar17 = DAT_00739290;
    iVar12 = DAT_005251b4;
    if (0 < (int)DAT_00739290) {
        uVar4 = DAT_00739290;
        pbVar13 = (unsigned char *)DAT_007337a4;
        pbVar15 = abStack_2a0;
        do {
            *pbVar15 = *pbVar13;
            uVar4 = uVar4 - 1;
            pbVar15[1] = pbVar13[-8];
            pbVar15[2] = pbVar13[8];
            pbVar13 = pbVar13 + 0x238;
            pbVar15 = pbVar15 + 3;
        } while (uVar4 != 0);
    }
    do {
        DAT_00739290 = uVar17;
        if (iVar12 < 1) {
            uVar17 = DAT_005251b0;
            iVar12 = DAT_005251b4;
            if (0x3fe < DAT_005251b4) {
                for (; 0 < iVar12; iVar12 = iVar12 + -1) {
                    iVar18 = uVar17 * 0x210;
                    if (DAT_00539a08[uVar17 * 0x84] == 0) {
                        uVar17 = uVar17 + 1;
                    }
                    else {
                        cVar2 = DAT_00539a15[iVar18];
                        uVar4 = (unsigned int)DAT_00539a16[iVar18];
                        uVar10 = FUN_00433540(DAT_00539a08[uVar17 * 0x84]);
                        if ((uVar10 < DAT_00739290) && (uVar10 < 0x28)) {
                            if (cVar2 != '\0') {
                                if (cVar2 != '\x02') {
                                    iVar14 = DAT_0073379c[uVar10 * 0x8e];
                                }
                                else {
                                    iVar14 = DAT_007337ac[uVar10 * 0x8e];
                                }
                            }
                            else {
                                iVar14 = DAT_007337a4[uVar10 * 0x8e];
                            }
                            uStack_328 = (unsigned int)iVar14 + 1U;
                            if (0x7f < (int)uStack_328) {
                                uStack_328 = 0;
                            }
                        }
                        iVar14 = uVar4 - uStack_328;
                        if ((uVar10 < DAT_00739290) && ((iVar14 < -0x1b || ((-1 < iVar14 && (iVar14 < 100)))))) {
                            if (DAT_00539a14[iVar18] != '\0') {
                                if (cVar2 != '\0') {
                                    if (cVar2 != '\x02') {
                                        DAT_0073379c[uVar10 * 0x8e] = uVar4;
                                    }
                                    else {
                                        DAT_007337ac[uVar10 * 0x8e] = uVar4;
                                    }
                                }
                                else {
                                    DAT_007337a4[uVar10 * 0x8e] = uVar4;
                                }
                                DAT_005ef21c = uVar4;
                                DAT_007339c0[uVar10 * 0x8e] = (int)((unsigned int)DAT_007339c0[uVar10 * 0x8e] + 1U);
                                puVar19 = &DAT_00539a08[uVar17 * 0x84];
                                puVar21 = DAT_00739298;
                                for (iVar12 = 0x84; iVar12 != 0; iVar12 = iVar12 + -1) {
                                    *puVar21 = *puVar19;
                                    puVar19 = puVar19 + 1;
                                    puVar21 = puVar21 + 1;
                                }
                                FUN_00433450(uVar17);
                                *param_1 = DAT_00739298[0];
                                *param_2 = DAT_00739298[1];
                                return &DAT_007392a8;
                            }
                            uVar17 = uVar17 + 1;
                        }
                        else {
                            iVar18 = FUN_00433450(uVar17);
                            if (iVar18 != 0) {
                                uVar17 = uVar17 + 1;
                            }
                        }
                    }
                    if (0x3ff < (int)uVar17) {
                        uVar17 = 0;
                    }
                }
            }
            return (unsigned char *)0;
        }
        iVar18 = uStack_32c * 0x210;
        piVar20 = &DAT_00539a08[uStack_32c * 0x84];
        if (DAT_00539a08[uStack_32c * 0x84] == 0) {
            if (uStack_32c == DAT_005251b0) {
                puVar19 = &DAT_00539a08[uStack_32c * 0x84];
                puVar21 = DAT_00739298;
                for (iVar12 = 0x84; iVar12 != 0; iVar12 = iVar12 + -1) {
                    *puVar21 = *puVar19;
                    puVar19 = puVar19 + 1;
                    puVar21 = puVar21 + 1;
                }
                *param_1 = DAT_00539a08[uStack_32c * 0x84];
                *param_2 = DAT_00539a0c[uStack_32c * 0x84];
                DAT_005251b0 = DAT_005251b0 + 1U;
                DAT_005251b4 = (int)((unsigned int)DAT_005251b4 - 1U);
                if (0x3ff < (int)DAT_005251b0) {
                    DAT_005251b0 = 0;
                }
                return &DAT_007392a8;
            }
joined_r0x00431ef6:
            uStack_32c = uStack_32c + 1;
            if (0x3ff < (int)uStack_32c) {
                uStack_32c = 0;
            }
        }
        else {
            bVar11 = DAT_00539a16[iVar18];
            uVar10 = (unsigned int)bVar11;
            uVar4 = (unsigned int)(DAT_00539a15[iVar18] == '\0');
            uVar5 = (unsigned int)(DAT_00539a15[iVar18] == '\x02');
            uVar6 = FUN_00433540(DAT_00539a08[uStack_32c * 0x84]);
            if ((DAT_00739290 != uVar17) && (uVar6 < 0x28)) {
                iVar14 = uVar6 * 3;
                abStack_2a0[iVar14] = 0x7f;
                abStack_2a0[iVar14 + 1] = 0x7f;
                abStack_2a0[iVar14 + 2] = 0x7f;
            }
            if (uVar6 < DAT_00739290) {
                if (abStack_228[uVar6] < abStack_2f0[uVar6]) goto joined_r0x00431ef6;
                if (DAT_00539a17[iVar18] == '\0') {
                    abStack_2f0[uVar6] = abStack_2f0[uVar6] + 1;
                }
                if (uVar4 == 0) {
                    if (uVar5 == 0) {
                        uStack_328 = (unsigned int)DAT_0073379c[uVar6 * 0x8e] + 1U;
                        if (0x7f < (int)uStack_328) {
                            uStack_328 = 0;
                        }
                        bVar1 = abStack_2a0[uVar6 * 3 + 1];
                    }
                    else {
                        uStack_328 = (unsigned int)DAT_007337ac[uVar6 * 0x8e] + 1U;
                        if (0x7f < (int)uStack_328) {
                            uStack_328 = 0;
                        }
                        bVar1 = abStack_2a0[uVar6 * 3 + 2];
                    }
                }
                else {
                    uStack_328 = (unsigned int)DAT_007337a4[uVar6 * 0x8e] + 1U;
                    if (0x7f < (int)uStack_328) {
                        uStack_328 = 0;
                    }
                    bVar1 = abStack_2a0[uVar6 * 3];
                }
                uStack_314 = bVar1 + 1;
                if (0x7f < uStack_314) {
                    uStack_314 = 0;
                }
            }
            iVar14 = DAT_00539a18[uStack_32c * 0x84];
            iVar7 = uVar10 - uStack_328;
            if ((uVar6 < DAT_00739290) && ((iVar7 < -0x1c || ((-1 < iVar7 && (iVar7 < 100)))))) {
                if ((uStack_328 == uVar10) && (uStack_314 == uStack_328)) {
                    DVar9 = timeGetTime();
                    DAT_007339bc[uVar6 * 0x8e] = DVar9;
                    if (uVar4 == 0) {
                        if (uVar5 == 0) {
                            DAT_0073379c[uVar6 * 0x8e] = uVar10;
                        }
                        else {
                            DAT_007337ac[uVar6 * 0x8e] = uVar10;
                        }
                    }
                    else {
                        DAT_007337a4[uVar6 * 0x8e] = uVar10;
                    }
                    DAT_005ef21c = uVar10;
                    if ((iVar14 != 1) || (DAT_0085e78f != '\x01')) {
                        DAT_007339c0[uVar6 * 0x8e] = (int)((unsigned int)DAT_007339c0[uVar6 * 0x8e] + 1U);
                    }
                    puVar19 = &DAT_00539a08[uStack_32c * 0x84];
                    puVar21 = DAT_00739298;
                    for (iVar12 = 0x84; iVar12 != 0; iVar12 = iVar12 + -1) {
                        *puVar21 = *puVar19;
                        puVar19 = puVar19 + 1;
                        puVar21 = puVar21 + 1;
                    }
                    FUN_00433450(uStack_32c);
                    *param_1 = DAT_00739298[0];
                    *param_2 = DAT_00739298[1];
                    return &DAT_007392a8;
                }
                if ((iVar14 == 1) && (DAT_0085e78f == '\x01')) {
                    DVar9 = timeGetTime();
                    DAT_007339bc[uVar6 * 0x8e] = DVar9;
                    if (uVar4 == 0) {
                        if (uVar5 == 0) {
                            DAT_0073379c[uVar6 * 0x8e] = uVar10;
                        }
                        else {
                            DAT_007337ac[uVar6 * 0x8e] = uVar10;
                        }
                    }
                    else {
                        DAT_007337a4[uVar6 * 0x8e] = uVar10;
                    }
                    puVar19 = &DAT_00539a08[uStack_32c * 0x84];
                    puVar21 = DAT_00739298;
                    DAT_005ef21c = uVar10;
                    for (iVar12 = 0x84; iVar12 != 0; iVar12 = iVar12 + -1) {
                        *puVar21 = *puVar19;
                        puVar19 = puVar19 + 1;
                        puVar21 = puVar21 + 1;
                    }
                    FUN_00433450(uStack_32c);
                    *param_1 = DAT_00739298[0];
                    *param_2 = DAT_00739298[1];
                    return &DAT_007392a8;
                }
                if (DAT_00539a17[iVar18] == '\0') {
                    iVar7 = 0;
                    if (uVar4 == 0) {
                        iVar7 = 2 - (uVar5 == 0);
                    }
                    iVar7 = uVar6 * 3 + iVar7;
                    uVar17 = abStack_2a0[iVar7] + 1;
                    abStack_2a0[iVar7] = bVar11;
                    if (0x7f < uVar17) {
                        uVar17 = 0;
                    }
                    iStack_310 = uVar10 - uVar17;
                    if (iStack_310 < 0) {
                        iStack_310 = (int)((unsigned int)iStack_310 + 0x80U);
                    }
                    bVar3 = 0;
                    iVar7 = (int)((unsigned int)iStack_310 * (unsigned int)DAT_00525558);
                    if (abStack_2f0[uVar6] < 0x7f) {
                        abStack_2f0[uVar6] = abStack_2f0[uVar6] + (char)iStack_310;
                    }
                    uVar16 = uVar17;
                    if (abStack_2c8[uVar6] < 0x1a) {
                        while (uVar16 != uVar10) {
                            uVar8 = FUN_00433340(uStack_32c, uVar16, uVar4, uVar5, uVar6);
                            if (((int)uVar8 < 0x400) && (-1 < (int)uVar8)) {
                                if (uVar16 == uStack_328) {
                                    DVar9 = timeGetTime();
                                    DAT_007339bc[uVar6 * 0x8e] = DVar9;
                                    puVar19 = &DAT_00539a08[uVar8 * 0x84];
                                    puVar21 = DAT_00739298;
                                    for (iVar12 = 0x84; iVar12 != 0; iVar12 = iVar12 + -1) {
                                        *puVar21 = *puVar19;
                                        puVar19 = puVar19 + 1;
                                        puVar21 = puVar21 + 1;
                                    }
                                    FUN_00433450(uVar8);
                                    if (uVar4 == 0) {
                                        if (uVar5 == 0) {
                                            DAT_0073379c[uVar6 * 0x8e] = uStack_328;
                                        }
                                        else {
                                            DAT_007337ac[uVar6 * 0x8e] = uStack_328;
                                        }
                                    }
                                    else {
                                        DAT_007337a4[uVar6 * 0x8e] = uStack_328;
                                    }
                                    DAT_005ef21c = uStack_328;
                                    DAT_007339c0[uVar6 * 0x8e] = (int)((unsigned int)DAT_007339c0[uVar6 * 0x8e] + 1U);
                                    *param_1 = DAT_00739298[0];
                                    *param_2 = DAT_00739298[1];
                                    if (iStack_310 < 2) {
                                        DAT_00539a14[iVar18] = 0;
                                        DAT_00539a10[uStack_32c * 0x84] = 0;
                                    }
                                    return &DAT_007392a8;
                                }
                                iStack_310 = (int)((unsigned int)iStack_310 - 1U);
                            }
                            else {
                                cVar2 = DAT_00539a14[iVar18];
                                abStack_2c8[uVar6] = abStack_2c8[uVar6] + 1;
                                if (cVar2 == '\0') {
                                    uStack_200[2] = 0;
                                    if (uVar4 == 0) {
                                        uStack_200[2] = 2 - (uVar5 == 0);
                                    }
                                    if (iVar14 == 2) {
                                        uStack_200[0] = 0x33;
                                        iVar23 = *piVar20;
                                        uStack_200[1] = (DAT_00539a1c[uStack_32c * 0x84] & 0x7fffffff) - iVar7;
                                        uStack_200[2] = uVar16;
                                    }
                                    else {
                                        iVar23 = *piVar20;
                                        uStack_200[0] = 0x38;
                                        uStack_200[1] = uVar16;
                                    }
                                    FUN_00432a30(iVar23, uStack_200, 0xc);
                                    *(unsigned int *)(DAT_007339c8 + uVar6 * 0x238) =
                                        *(unsigned int *)(DAT_007339c8 + uVar6 * 0x238) + 1U;
                                }
                                else {
                                    DVar9 = timeGetTime();
                                    if (DAT_007330d4 == 0) {
                                        bVar11 = (-(iVar14 == 2) & 2U) + 3;
                                        uVar8 = 1000U << (DAT_00539a14[iVar18] & 0x1f);
                                    }
                                    else {
                                        bVar11 = 0;
                                        uVar8 = (-(unsigned int)(iVar14 == 2) & 37000) + 3000;
                                    }
                                    if (DVar9 - DAT_00539a10[uStack_32c * 0x84] <= uVar8) goto LAB_00432309;
                                    if (bVar11 < (unsigned char)DAT_00539a14[iVar18]) {
                                        DVar9 = timeGetTime();
                                        DAT_007339bc[uVar6 * 0x8e] = DVar9;
                                        while ((uVar16 = FUN_00433340(uStack_32c, uVar17, uVar4, uVar5, uVar6),
                                                (int)uVar16 < 0 || (0x400 < (int)uVar16))) {
                                            if (uVar17 == uVar10) {
                                                if (uVar4 == 0) {
                                                    if (uVar5 == 0) {
                                                        DAT_0073379c[uVar6 * 0x8e] = uVar10;
                                                    }
                                                    else {
                                                        DAT_007337ac[uVar6 * 0x8e] = uVar10;
                                                    }
                                                }
                                                else {
                                                    DAT_007337a4[uVar6 * 0x8e] = uVar10;
                                                }
                                                DAT_005ef21c = uVar10;
                                                DAT_007339c0[uVar6 * 0x8e] = (int)((unsigned int)DAT_007339c0[uVar6 * 0x8e] + 1U);
                                                piVar22 = DAT_00739298;
                                                for (iVar12 = 0x84; iVar12 != 0; iVar12 = iVar12 + -1) {
                                                    *piVar22 = *piVar20;
                                                    piVar20 = piVar20 + 1;
                                                    piVar22 = piVar22 + 1;
                                                }
                                                FUN_00433450(uStack_32c);
                                                *param_1 = DAT_00739298[0];
                                                *param_2 = DAT_00739298[1];
                                                return &DAT_007392a8;
                                            }
                                            uVar17 = uVar17 + 1;
                                            if (0x7f < (int)uVar17) {
                                                uVar17 = 0;
                                            }
                                        }
                                        puVar19 = &DAT_00539a08[uVar16 * 0x84];
                                        puVar21 = DAT_00739298;
                                        for (iVar12 = 0x84; iVar12 != 0; iVar12 = iVar12 + -1) {
                                            *puVar21 = *puVar19;
                                            puVar19 = puVar19 + 1;
                                            puVar21 = puVar21 + 1;
                                        }
                                        FUN_00433450(uVar16);
                                        if (uVar4 == 0) {
                                            if (uVar5 == 0) {
                                                DAT_0073379c[uVar6 * 0x8e] = uVar17;
                                            }
                                            else {
                                                DAT_007337ac[uVar6 * 0x8e] = uVar17;
                                            }
                                        }
                                        else {
                                            DAT_007337a4[uVar6 * 0x8e] = uVar17;
                                        }
                                        DAT_005ef21c = uVar17;
                                        DAT_007339c0[uVar6 * 0x8e] = (int)((unsigned int)DAT_007339c0[uVar6 * 0x8e] + 1U);
                                        *param_1 = DAT_00739298[0];
                                        *param_2 = DAT_00739298[1];
                                        return &DAT_007392a8;
                                    }
                                    uStack_200[2] = 0;
                                    if (uVar4 == 0) {
                                        uStack_200[2] = 2 - (uVar5 == 0);
                                    }
                                    if (iVar14 == 2) {
                                        uStack_200[0] = 0x33;
                                        iVar23 = *piVar20;
                                        uStack_200[1] = (DAT_00539a1c[uStack_32c * 0x84] & 0x7fffffff) - iVar7;
                                        uStack_200[2] = uVar16;
                                    }
                                    else {
                                        iVar23 = *piVar20;
                                        uStack_200[0] = 0x38;
                                        uStack_200[1] = uVar16;
                                    }
                                    FUN_00432a30(iVar23, uStack_200, 0xc);
                                }
                                bVar3 = 1;
                            }
LAB_00432309:
                            iVar7 = (int)((unsigned int)iVar7 - (unsigned int)DAT_00525558);
                            uVar16 = uVar16 + 1;
                            if (0x7f < (int)uVar16) {
                                uVar16 = 0;
                            }
                        }
                        if (iStack_310 < 1) {
                            DAT_00539a14[iVar18] = 0;
                            DAT_00539a10[uStack_32c * 0x84] = 0;
                        }
                        else if (bVar3) {
                            DAT_00539a14[iVar18] = DAT_00539a14[iVar18] + '\x01';
                            DVar9 = timeGetTime();
                            DAT_00539a10[uStack_32c * 0x84] = DVar9;
                        }
                    }
                }
                else if ((uStack_32c == DAT_005251b0) && (iVar18 = FUN_00433450(uStack_32c), iVar18 == 0))
                    goto LAB_004323c9;
                goto joined_r0x00431ef6;
            }
            iVar18 = FUN_00433450(uStack_32c);
            if (iVar18 != 0) goto joined_r0x00431ef6;
        }
LAB_004323c9:
        uVar17 = DAT_00739290;
        iVar12 = iVar12 + -1;
    } while (1);
}

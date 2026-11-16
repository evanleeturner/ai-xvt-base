/* SPDX-License-Identifier: MIT */
/* z_bop 0046e780 */
/* Callee prototypes */
void FUN_0046c830(void);
void FUN_0046fae0(void);
unsigned int FUN_0046fee0(unsigned int param_1);
unsigned int FUN_0046fce0(unsigned int param_1, unsigned int param_2, unsigned int param_3, unsigned int param_4, unsigned int param_5);
void FUN_0046f470(int param_1, unsigned int *param_2, int param_3);
/* Body is a bare ret; the caller pushes a string (cdecl), so the string is passed. */
void __cdecl FUN_0046c220(const char *param_1);
int FUN_0046fdf0(unsigned int param_1);
DWORD __stdcall timeGetTime(void);

/* Globals, as separate non-overlapping symbols (types inferred from usage) */
extern int DAT_0052702c;
extern unsigned int DAT_00527028;
extern unsigned int DAT_0099f6e0;
extern unsigned int DAT_0099f6e8;
extern unsigned int DAT_0099f6ec;
extern unsigned char DAT_0099f6f8;
extern unsigned int DAT_005599a8;
extern unsigned int DAT_005599ac;
extern unsigned int DAT_005599b0;
extern char DAT_005599b4;
extern char DAT_005599b5;
extern unsigned char DAT_005599b6;
extern char DAT_005599b7;
extern int DAT_005599b8;
extern unsigned int DAT_005599bc;
extern unsigned char DAT_00999bec;
extern unsigned char DAT_00999bf4;
extern unsigned char DAT_00999bfc;
extern unsigned int DAT_00999e0c;
extern unsigned int DAT_00999e10;
extern unsigned char DAT_00999e18;
extern unsigned int _DAT_0060f1bc;
extern int DAT_00523418;
extern int DAT_00999524;
extern char DAT_00bb736f;

unsigned char *FUN_0046e780(unsigned int *param_1, unsigned int *param_2)
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
    int iVar11;
    unsigned char bVar12;
    int iVar13;
    unsigned char *pbVar14;
    unsigned char *pbVar15;
    unsigned int uVar16;
    unsigned int uVar17;
    int iVar18;
    unsigned int *puVar19;
    int *piVar20;
    unsigned int *puVar21;
    int *piVar22;
    int iVar23;
    unsigned int uStack_32c;
    unsigned int uStack_328;
    unsigned int uStack_314;
    int iStack_310;
    unsigned char abStack_2f0[40];
    unsigned char abStack_2c8[40];
    unsigned char abStack_2a0[120];
    unsigned char abStack_228[40];
    unsigned int uStack_200[3]; /* [0]=uStack_200, [1]=uStack_1fc, [2]=uStack_1f8 (contiguous, passed by address) */

    FUN_0046c830();
    FUN_0046fae0();
    if (DAT_0052702c == 0) {
        return (unsigned char *)0;
    }
    pbVar14 = abStack_2c8;
    for (iVar13 = 10; iVar13 != 0; iVar13 = iVar13 + -1) {
        pbVar14[0] = 0;
        pbVar14[1] = 0;
        pbVar14[2] = 0;
        pbVar14[3] = 0;
        pbVar14 = pbVar14 + 4;
    }
    pbVar14 = abStack_2f0;
    for (iVar13 = 10; iVar13 != 0; iVar13 = iVar13 + -1) {
        pbVar14[0] = 0;
        pbVar14[1] = 0;
        pbVar14[2] = 0;
        pbVar14[3] = 0;
        pbVar14 = pbVar14 + 4;
    }
    pbVar14 = abStack_228;
    for (iVar13 = 10; iVar13 != 0; iVar13 = iVar13 + -1) {
        pbVar14[0] = 0x5a;
        pbVar14[1] = 0x5a;
        pbVar14[2] = 0x5a;
        pbVar14[3] = 0x5a;
        pbVar14 = pbVar14 + 4;
    }
    pbVar14 = abStack_2a0;
    for (iVar13 = 0x1e; iVar13 != 0; iVar13 = iVar13 + -1) {
        pbVar14[0] = 0;
        pbVar14[1] = 0;
        pbVar14[2] = 0;
        pbVar14[3] = 0;
        pbVar14 = pbVar14 + 4;
    }
    uStack_32c = DAT_00527028;
    uVar17 = DAT_0099f6e0;
    iVar13 = DAT_0052702c;
    if (0 < (int)DAT_0099f6e0) {
        uVar4 = DAT_0099f6e0;
        pbVar14 = &DAT_00999bf4;
        pbVar15 = abStack_2a0;
        do {
            *pbVar15 = *pbVar14;
            uVar4 = uVar4 - 1;
            pbVar15[1] = pbVar14[-8];
            pbVar15[2] = pbVar14[8];
            pbVar14 = pbVar14 + 0x238;
            pbVar15 = pbVar15 + 3;
        } while (uVar4 != 0);
    }
    do {
        DAT_0099f6e0 = uVar17;
        if (iVar13 < 1) {
            uVar17 = DAT_00527028;
            iVar13 = DAT_0052702c;
            if (0x3fe < DAT_0052702c) {
                for (; 0 < iVar13; iVar13 = iVar13 + -1) {
                    iVar18 = (int)(uVar17 * 0x210);
                    if ((&DAT_005599a8)[uVar17 * 0x84] == 0) {
                        uVar17 = uVar17 + 1;
                    }
                    else {
                        cVar2 = (&DAT_005599b5)[iVar18];
                        uVar4 = (unsigned int)(unsigned char)(&DAT_005599b6)[iVar18];
                        uVar10 = FUN_0046fee0((&DAT_005599a8)[uVar17 * 0x84]);
                        if ((uVar10 < DAT_0099f6e0) && (uVar10 < 0x28)) {
                            if (cVar2 != '\0') {
                                if (cVar2 != '\x02') {
                                    iVar11 = *(int *)(&DAT_00999bec + uVar10 * 0x238);
                                }
                                else {
                                    iVar11 = *(int *)(&DAT_00999bfc + uVar10 * 0x238);
                                }
                            }
                            else {
                                iVar11 = *(int *)(&DAT_00999bf4 + uVar10 * 0x238);
                            }
                            uStack_328 = (unsigned int)iVar11 + 1u;
                            if (0x7f < (int)uStack_328) {
                                uStack_328 = 0;
                            }
                        }
                        iVar11 = (int)(uVar4 - uStack_328);
                        if ((uVar10 < DAT_0099f6e0) && ((iVar11 < -0x1b || ((-1 < iVar11 && (iVar11 < 100)))))) {
                            if ((&DAT_005599b4)[iVar18] != '\0') {
                                FUN_0046c220("(NMR) ");
                                if (cVar2 != '\0') {
                                    if (cVar2 != '\x02') {
                                        *(unsigned int *)(&DAT_00999bec + uVar10 * 0x238) = uVar4;
                                    }
                                    else {
                                        *(unsigned int *)(&DAT_00999bfc + uVar10 * 0x238) = uVar4;
                                    }
                                }
                                else {
                                    *(unsigned int *)(&DAT_00999bf4 + uVar10 * 0x238) = uVar4;
                                }
                                _DAT_0060f1bc = uVar4;
                                (&DAT_00999e10)[uVar10 * 0x8e] = (&DAT_00999e10)[uVar10 * 0x8e] + 1;
                                puVar19 = &DAT_005599a8 + uVar17 * 0x84;
                                puVar21 = &DAT_0099f6e8;
                                for (iVar13 = 0x84; iVar13 != 0; iVar13 = iVar13 + -1) {
                                    *puVar21 = *puVar19;
                                    puVar19 = puVar19 + 1;
                                    puVar21 = puVar21 + 1;
                                }
                                FUN_0046fdf0(uVar17);
                                *param_1 = DAT_0099f6e8;
                                *param_2 = DAT_0099f6ec;
                                return &DAT_0099f6f8;
                            }
                            uVar17 = uVar17 + 1;
                        }
                        else {
                            iVar18 = FUN_0046fdf0(uVar17);
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
        iVar18 = (int)(uStack_32c * 0x210);
        piVar20 = (int *)(&DAT_005599a8 + uStack_32c * 0x84);
        if ((&DAT_005599a8)[uStack_32c * 0x84] == 0) {
            if (DAT_00527028 == uStack_32c) {
                puVar19 = &DAT_005599a8 + uStack_32c * 0x84;
                puVar21 = &DAT_0099f6e8;
                for (iVar13 = 0x84; iVar13 != 0; iVar13 = iVar13 + -1) {
                    *puVar21 = *puVar19;
                    puVar19 = puVar19 + 1;
                    puVar21 = puVar21 + 1;
                }
                *param_1 = (&DAT_005599a8)[uStack_32c * 0x84];
                *param_2 = (&DAT_005599ac)[uStack_32c * 0x84];
                DAT_00527028 = DAT_00527028 + 1;
                DAT_0052702c = (int)((unsigned int)DAT_0052702c - 1u);
                if (0x3ff < (int)DAT_00527028) {
                    DAT_00527028 = 0;
                }
                return &DAT_0099f6f8;
            }
joined_r0x0046e906:
            uStack_32c = uStack_32c + 1;
            if (0x3ff < (int)uStack_32c) {
                uStack_32c = 0;
            }
        }
        else {
            bVar12 = (&DAT_005599b6)[iVar18];
            uVar10 = (unsigned int)bVar12;
            uVar4 = (unsigned int)((&DAT_005599b5)[iVar18] == '\0');
            uVar5 = (unsigned int)((&DAT_005599b5)[iVar18] == '\x02');
            uVar6 = FUN_0046fee0((&DAT_005599a8)[uStack_32c * 0x84]);
            if ((uVar17 != DAT_0099f6e0) && (uVar6 < 0x28)) {
                iVar11 = (int)(uVar6 * 3);
                abStack_2a0[iVar11] = 0x7f;
                abStack_2a0[iVar11 + 1] = 0x7f;
                abStack_2a0[iVar11 + 2] = 0x7f;
            }
            if (uVar6 < DAT_0099f6e0) {
                if (abStack_228[uVar6] < abStack_2f0[uVar6]) goto joined_r0x0046e906;
                if ((&DAT_005599b7)[iVar18] == '\0') {
                    abStack_2f0[uVar6] = (unsigned char)(abStack_2f0[uVar6] + 1);
                }
                if (uVar4 == 0) {
                    if (uVar5 == 0) {
                        uStack_328 = (unsigned int)*(int *)(&DAT_00999bec + uVar6 * 0x238) + 1u;
                        if (0x7f < (int)uStack_328) {
                            uStack_328 = 0;
                        }
                        bVar1 = abStack_2a0[uVar6 * 3 + 1];
                    }
                    else {
                        uStack_328 = (unsigned int)*(int *)(&DAT_00999bfc + uVar6 * 0x238) + 1u;
                        if (0x7f < (int)uStack_328) {
                            uStack_328 = 0;
                        }
                        bVar1 = abStack_2a0[uVar6 * 3 + 2];
                    }
                }
                else {
                    uStack_328 = (unsigned int)*(int *)(&DAT_00999bf4 + uVar6 * 0x238) + 1u;
                    if (0x7f < (int)uStack_328) {
                        uStack_328 = 0;
                    }
                    bVar1 = abStack_2a0[uVar6 * 3];
                }
                uStack_314 = (unsigned int)bVar1 + 1u;
                if (0x7f < uStack_314) {
                    uStack_314 = 0;
                }
            }
            iVar11 = (&DAT_005599b8)[uStack_32c * 0x84];
            iVar7 = (int)(uVar10 - uStack_328);
            if ((uVar6 < DAT_0099f6e0) && ((iVar7 < -0x1c || ((-1 < iVar7 && (iVar7 < 100)))))) {
                if ((uStack_328 == uVar10) && (uStack_314 == uStack_328)) {
                    iVar13 = (int)(uVar6 * 0x238);
                    DVar9 = timeGetTime();
                    (&DAT_00999e0c)[uVar6 * 0x8e] = DVar9;
                    if (uVar4 == 0) {
                        if (uVar5 == 0) {
                            *(unsigned int *)(&DAT_00999bec + iVar13) = uVar10;
                        }
                        else {
                            *(unsigned int *)(&DAT_00999bfc + iVar13) = uVar10;
                        }
                    }
                    else {
                        *(unsigned int *)(&DAT_00999bf4 + iVar13) = uVar10;
                    }
                    if ((iVar11 != 1) || (_DAT_0060f1bc = uVar10, DAT_00bb736f != '\x01')) {
                        _DAT_0060f1bc = uVar10;
                        (&DAT_00999e10)[uVar6 * 0x8e] = (&DAT_00999e10)[uVar6 * 0x8e] + 1;
                    }
                    puVar19 = &DAT_005599a8 + uStack_32c * 0x84;
                    puVar21 = &DAT_0099f6e8;
                    for (iVar13 = 0x84; iVar13 != 0; iVar13 = iVar13 + -1) {
                        *puVar21 = *puVar19;
                        puVar19 = puVar19 + 1;
                        puVar21 = puVar21 + 1;
                    }
                    FUN_0046fdf0(uStack_32c);
                    *param_1 = DAT_0099f6e8;
                    *param_2 = DAT_0099f6ec;
                    return &DAT_0099f6f8;
                }
                if ((iVar11 == 1) && (DAT_00bb736f == '\x01')) {
                    iVar13 = (int)(uVar6 * 0x238);
                    DVar9 = timeGetTime();
                    (&DAT_00999e0c)[uVar6 * 0x8e] = DVar9;
                    if (uVar4 == 0) {
                        if (uVar5 == 0) {
                            *(unsigned int *)(&DAT_00999bec + iVar13) = uVar10;
                        }
                        else {
                            *(unsigned int *)(&DAT_00999bfc + iVar13) = uVar10;
                        }
                    }
                    else {
                        *(unsigned int *)(&DAT_00999bf4 + iVar13) = uVar10;
                    }
                    puVar19 = &DAT_005599a8 + uStack_32c * 0x84;
                    puVar21 = &DAT_0099f6e8;
                    _DAT_0060f1bc = uVar10;
                    for (iVar13 = 0x84; iVar13 != 0; iVar13 = iVar13 + -1) {
                        *puVar21 = *puVar19;
                        puVar19 = puVar19 + 1;
                        puVar21 = puVar21 + 1;
                    }
                    FUN_0046fdf0(uStack_32c);
                    *param_1 = DAT_0099f6e8;
                    *param_2 = DAT_0099f6ec;
                    return &DAT_0099f6f8;
                }
                if ((&DAT_005599b7)[iVar18] == '\0') {
                    iVar7 = 0;
                    if (uVar4 == 0) {
                        iVar7 = 2 - (int)(unsigned int)(uVar5 == 0);
                    }
                    iVar7 = (int)(uVar6 * 3) + iVar7;
                    uVar17 = (unsigned int)abStack_2a0[iVar7] + 1u;
                    abStack_2a0[iVar7] = bVar12;
                    if (0x7f < uVar17) {
                        uVar17 = 0;
                    }
                    iStack_310 = (int)(uVar10 - uVar17);
                    if (iStack_310 < 0) {
                        iStack_310 = iStack_310 + 0x80;
                    }
                    bVar3 = 0;
                    iVar7 = (int)((unsigned int)iStack_310 * (unsigned int)DAT_00523418);
                    if (abStack_2f0[uVar6] < 0x7f) {
                        abStack_2f0[uVar6] = (unsigned char)(abStack_2f0[uVar6] + (char)iStack_310);
                    }
                    uVar16 = uVar17;
                    if (abStack_2c8[uVar6] < 0x1a) {
                        while (uVar16 != uVar10) {
                            uVar8 = FUN_0046fce0(uStack_32c, uVar16, uVar4, uVar5, uVar6);
                            if (((int)uVar8 < 0x400) && (-1 < (int)uVar8)) {
                                if (uVar16 == uStack_328) {
                                    iVar13 = (int)(uVar6 * 0x238);
                                    DVar9 = timeGetTime();
                                    (&DAT_00999e0c)[uVar6 * 0x8e] = DVar9;
                                    puVar19 = &DAT_005599a8 + uVar8 * 0x84;
                                    puVar21 = &DAT_0099f6e8;
                                    for (iVar11 = 0x84; iVar11 != 0; iVar11 = iVar11 + -1) {
                                        *puVar21 = *puVar19;
                                        puVar19 = puVar19 + 1;
                                        puVar21 = puVar21 + 1;
                                    }
                                    FUN_0046fdf0(uVar8);
                                    if (uVar4 == 0) {
                                        if (uVar5 == 0) {
                                            *(unsigned int *)(&DAT_00999bec + iVar13) = uStack_328;
                                        }
                                        else {
                                            *(unsigned int *)(&DAT_00999bfc + iVar13) = uStack_328;
                                        }
                                    }
                                    else {
                                        *(unsigned int *)(&DAT_00999bf4 + iVar13) = uStack_328;
                                    }
                                    _DAT_0060f1bc = uStack_328;
                                    (&DAT_00999e10)[uVar6 * 0x8e] = (&DAT_00999e10)[uVar6 * 0x8e] + 1;
                                    *param_1 = DAT_0099f6e8;
                                    *param_2 = DAT_0099f6ec;
                                    if (iStack_310 < 2) {
                                        (&DAT_005599b4)[iVar18] = 0;
                                        (&DAT_005599b0)[uStack_32c * 0x84] = 0;
                                    }
                                    return &DAT_0099f6f8;
                                }
                                iStack_310 = iStack_310 + -1;
                            }
                            else {
                                cVar2 = (&DAT_005599b4)[iVar18];
                                abStack_2c8[uVar6] = (unsigned char)(abStack_2c8[uVar6] + 1);
                                if (cVar2 == '\0') {
                                    uStack_200[2] = 0;
                                    if (uVar4 == 0) {
                                        uStack_200[2] = 2u - (unsigned int)(uVar5 == 0);
                                    }
                                    if (iVar11 == 2) {
                                        uStack_200[0] = 0x33;
                                        iVar23 = *piVar20;
                                        uStack_200[1] = ((&DAT_005599bc)[uStack_32c * 0x84] & 0x7fffffffu) - (unsigned int)iVar7;
                                        uStack_200[2] = uVar16;
                                    }
                                    else {
                                        iVar23 = *piVar20;
                                        uStack_200[0] = 0x38;
                                        uStack_200[1] = uVar16;
                                    }
                                    FUN_0046f470(iVar23, &uStack_200[0], 0xc);
                                    *(int *)(&DAT_00999e18 + uVar6 * 0x238) =
                                        (int)((unsigned int)*(int *)(&DAT_00999e18 + uVar6 * 0x238) + 1u);
                                }
                                else {
                                    DVar9 = timeGetTime();
                                    if (DAT_00999524 == 0) {
                                        bVar12 = (unsigned char)((((unsigned int)(-(int)(iVar11 == 2))) & 2U) + 3u);
                                        uVar8 = 1000u << (((unsigned int)(&DAT_005599b4)[iVar18]) & 0x1fu);
                                    }
                                    else {
                                        bVar12 = 0;
                                        uVar8 = ((((unsigned int)(iVar11 == 2)) * 0xffffffffu) & 37000u) + 3000u;
                                    }
                                    if (DVar9 - (&DAT_005599b0)[uStack_32c * 0x84] <= uVar8) goto LAB_0046ed26;
                                    FUN_0046c220("(RTO) ");
                                    if (bVar12 < (unsigned char)(&DAT_005599b4)[iVar18]) {
                                        FUN_0046c220("(TMR) ");
                                        iVar13 = (int)(uVar6 * 0x238);
                                        DVar9 = timeGetTime();
                                        (&DAT_00999e0c)[uVar6 * 0x8e] = DVar9;
                                        while ((uVar16 = FUN_0046fce0(uStack_32c, uVar17, uVar4, uVar5, uVar6),
                                                (int)uVar16 < 0 || (0x400 < (int)uVar16))) {
                                            if (uVar17 == uVar10) {
                                                if (uVar4 == 0) {
                                                    if (uVar5 == 0) {
                                                        *(unsigned int *)(&DAT_00999bec + iVar13) = uVar10;
                                                    }
                                                    else {
                                                        *(unsigned int *)(&DAT_00999bfc + iVar13) = uVar10;
                                                    }
                                                }
                                                else {
                                                    *(unsigned int *)(&DAT_00999bf4 + iVar13) = uVar10;
                                                }
                                                _DAT_0060f1bc = uVar10;
                                                (&DAT_00999e10)[uVar6 * 0x8e] = (&DAT_00999e10)[uVar6 * 0x8e] + 1;
                                                piVar22 = (int *)&DAT_0099f6e8;
                                                for (iVar13 = 0x84; iVar13 != 0; iVar13 = iVar13 + -1) {
                                                    *piVar22 = *piVar20;
                                                    piVar20 = piVar20 + 1;
                                                    piVar22 = piVar22 + 1;
                                                }
                                                FUN_0046fdf0(uStack_32c);
                                                *param_1 = DAT_0099f6e8;
                                                *param_2 = DAT_0099f6ec;
                                                return &DAT_0099f6f8;
                                            }
                                            uVar17 = uVar17 + 1;
                                            if (0x7f < (int)uVar17) {
                                                uVar17 = 0;
                                            }
                                        }
                                        puVar19 = &DAT_005599a8 + uVar16 * 0x84;
                                        puVar21 = &DAT_0099f6e8;
                                        for (iVar18 = 0x84; iVar18 != 0; iVar18 = iVar18 + -1) {
                                            *puVar21 = *puVar19;
                                            puVar19 = puVar19 + 1;
                                            puVar21 = puVar21 + 1;
                                        }
                                        FUN_0046fdf0(uVar16);
                                        if (uVar4 == 0) {
                                            if (uVar5 == 0) {
                                                *(unsigned int *)(&DAT_00999bec + iVar13) = uVar17;
                                            }
                                            else {
                                                *(unsigned int *)(&DAT_00999bfc + iVar13) = uVar17;
                                            }
                                        }
                                        else {
                                            *(unsigned int *)(&DAT_00999bf4 + iVar13) = uVar17;
                                        }
                                        _DAT_0060f1bc = uVar17;
                                        (&DAT_00999e10)[uVar6 * 0x8e] = (&DAT_00999e10)[uVar6 * 0x8e] + 1;
                                        *param_1 = DAT_0099f6e8;
                                        *param_2 = DAT_0099f6ec;
                                        return &DAT_0099f6f8;
                                    }
                                    uStack_200[2] = 0;
                                    if (uVar4 == 0) {
                                        uStack_200[2] = 2u - (unsigned int)(uVar5 == 0);
                                    }
                                    if (iVar11 == 2) {
                                        uStack_200[0] = 0x33;
                                        iVar23 = *piVar20;
                                        uStack_200[1] = ((&DAT_005599bc)[uStack_32c * 0x84] & 0x7fffffffu) - (unsigned int)iVar7;
                                        uStack_200[2] = uVar16;
                                    }
                                    else {
                                        iVar23 = *piVar20;
                                        uStack_200[0] = 0x38;
                                        uStack_200[1] = uVar16;
                                    }
                                    FUN_0046f470(iVar23, &uStack_200[0], 0xc);
                                }
                                bVar3 = 1;
                            }
LAB_0046ed26:
                            iVar7 = (int)((unsigned int)iVar7 - (unsigned int)DAT_00523418);
                            uVar16 = uVar16 + 1;
                            if (0x7f < (int)uVar16) {
                                uVar16 = 0;
                            }
                        }
                        if (iStack_310 < 1) {
                            (&DAT_005599b4)[iVar18] = 0;
                            (&DAT_005599b0)[uStack_32c * 0x84] = 0;
                        }
                        else if (bVar3) {
                            (&DAT_005599b4)[iVar18] = (char)((&DAT_005599b4)[iVar18] + '\x01');
                            DVar9 = timeGetTime();
                            (&DAT_005599b0)[uStack_32c * 0x84] = DVar9;
                        }
                    }
                }
                else if ((DAT_00527028 == uStack_32c) && (iVar18 = FUN_0046fdf0(uStack_32c), iVar18 == 0))
                    goto LAB_0046ede6;
                goto joined_r0x0046e906;
            }
            iVar18 = FUN_0046fdf0(uStack_32c);
            if (iVar18 != 0) goto joined_r0x0046e906;
        }
LAB_0046ede6:
        uVar17 = DAT_0099f6e0;
        iVar13 = iVar13 + -1;
    } while (1);
}

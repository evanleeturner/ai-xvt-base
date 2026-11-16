/* SPDX-License-Identifier: MIT */
/* z_xvt 004663b0 */
extern long long __cdecl __ftol(void);

extern int             DAT_00525544;
extern unsigned short  DAT_007091ae;
/* packed (unaligned) globals: memcpy access only */
extern int             DAT_006a888e;
extern unsigned int   *DAT_006a882a;
extern unsigned int   *DAT_006a882e;

#define IADD(a, b) ((int)((unsigned int)(a) + (unsigned int)(b)))
#define ISUB(a, b) ((int)((unsigned int)(a) - (unsigned int)(b)))
#define PF(b, o) (*(float *)((b) + (o)))
#define CLR(s) (*(unsigned int *)(*(int *)((s) + 0x68) + (param_3 - *(unsigned int *)((s) + 0x40)) * 4) = 0)
#define LOADTBL() __builtin_memcpy(&iTbl, &DAT_006a888e, 4)
#define TBLP() (*(unsigned int **)(iTbl + param_3 * 4))
#define SF(p, i) (*(float *)((p) + (i)))

void __cdecl FUN_004663b0(float param_1, float param_2, unsigned int param_3, int param_4)
{
    int *piVar1;
    int *piVar2;
    int *piVar3;
    float *pfVar4;
    float *pfVar5;
    float fVar6;
    float fVar7;
    float fVar8;
    int iVar9;
    int iVar10;
    int iVar11;
    int iVar12;
    unsigned int uVar13;
    int iVar14;
    unsigned int *puVar15;
    int iVar16;
    unsigned int *puVar17;
    int iVar18;
    unsigned int *puVar19;
    unsigned int *puVar20;
    unsigned int *puVar21;
    unsigned int *puVar22;
    unsigned int *local_28;
    int local_24;
    unsigned int local_c;
    unsigned int bits;
    int iTbl;

    *(unsigned int *)(*(int *)(param_4 + 0x68) + (param_3 - *(int *)(param_4 + 0x40)) * 4) = 0;
    __builtin_memcpy(&bits, &param_1, 4);
    if (bits < 0x80000001u) {
        local_24 = (int)param_1;
        if ((float)local_24 != param_1) {
            local_24 = IADD(local_24, 1);
        }
    }
    else {
        local_24 = 0;
    }
    __builtin_memcpy(&bits, &param_2, 4);
    if (bits < 0x80000001u) {
        local_c = (unsigned int)(int)param_2;
        if ((float)(int)local_c != param_2) {
            local_c = (unsigned int)IADD(local_c, 1);
        }
    }
    else {
        local_c = 0;
    }
    uVar13 = (unsigned int)DAT_007091ae;
    if ((int)uVar13 <= (int)local_c) {
        local_c = uVar13;
    }
    if ((int)local_c <= local_24) {
        return;
    }
    if ((int)uVar13 <= local_24) {
        return;
    }
    if ((DAT_00525544 != 0) && ((param_3 & 1) != 0)) {
        return;
    }
    local_28 = (unsigned int *)0;
    LOADTBL();
    puVar21 = TBLP();
    puVar20 = local_28;
    iVar9 = local_24;
    while (puVar21 != (unsigned int *)0) {
        iVar16 = (int)puVar21[2];
        piVar1 = (int *)(puVar21 + 2);
        local_28 = puVar21;
        local_24 = iVar9;
        if (iVar9 < iVar16) {
            iVar12 = (int)puVar21[1];
            piVar2 = (int *)(puVar21 + 1);
            if (iVar9 < iVar12) break;
            iVar10 = (int)puVar21[5];
            piVar3 = (int *)(puVar21 + 5);
            if (PF(param_4, 0x48) <= PF(iVar10, 0x4c)) {
joined_r0x004669aa:
                local_24 = iVar16;
                if ((int)local_c <= iVar16) {
                    return;
                }
                goto LAB_00466a5a;
            }
            if (PF(iVar10, 0x48) <= PF(param_4, 0x4c)) {
                if ((iVar16 <= (int)local_c) && (*piVar1 = iVar9, *piVar2 == iVar9)) {
                    CLR(*piVar3);
                    if (puVar20 == (unsigned int *)0) {
                        LOADTBL();
                        TBLP() = (unsigned int *)*puVar21;
                        local_28 = puVar20;
                    }
                    else {
                        *puVar20 = *puVar21;
                        local_28 = puVar20;
                    }
                }
                goto LAB_00466a5a;
            }
            fVar6 = PF(param_4, 0x28) * (float)iVar9 +
                    PF(param_4, 0x2c) * (float)(int)param_3 + PF(param_4, 0x30);
            pfVar4 = (float *)(iVar10 + 0x28);
            fVar7 = PF(iVar10, 0x28) * (float)iVar9 +
                    (float)(int)param_3 * PF(iVar10, 0x2c) + PF(iVar10, 0x30);
            if (fVar6 <= fVar7) {
                if (*pfVar4 < PF(param_4, 0x28)) {
                    if (iVar16 < (int)local_c) {
                        local_24 = iVar16;
                        if ((float)ISUB(iVar16, iVar9) * PF(param_4, 0x28) + fVar6 <=
                            *pfVar4 * (float)ISUB(iVar16, iVar9) + fVar7) goto LAB_00466a5a;
                    }
                    else if ((float)(int)(local_c - iVar9) * PF(param_4, 0x28) + fVar6 <=
                             *pfVar4 * (float)(int)(local_c - iVar9) + fVar7) {
                        return;
                    }
                    iVar16 = (int)((fVar7 - fVar6) / (PF(param_4, 0x28) - *pfVar4));
                    if (iVar16 < 0) {
                        iVar16 = 0;
                    }
                    iVar16 = IADD(iVar9, iVar16);
                }
                goto joined_r0x004669aa;
            }
            if (*pfVar4 <= PF(param_4, 0x28)) {
                if ((iVar16 <= (int)local_c) && (*piVar1 = iVar9, *piVar2 == iVar9)) {
                    CLR(*piVar3);
                    if (puVar20 == (unsigned int *)0) {
                        LOADTBL();
                        TBLP() = (unsigned int *)*puVar21;
                        local_28 = puVar20;
                    }
                    else {
                        *puVar20 = *puVar21;
                        local_28 = puVar20;
                    }
                }
                goto LAB_00466a5a;
            }
            if ((int)local_c < iVar16) {
                if ((float)(int)(local_c - iVar9) * PF(param_4, 0x28) + fVar6 <
                    *pfVar4 * (float)(int)(local_c - iVar9) + fVar7) {
                    iVar16 = (int)((fVar6 - fVar7) / (*pfVar4 - PF(param_4, 0x28)));
                    if (iVar16 < 0) {
                        iVar16 = 0;
                    }
                    uVar13 = (unsigned int)IADD(iVar16, iVar9);
                    if ((int)local_c < IADD(iVar16, iVar9)) {
                        uVar13 = local_c;
                    }
joined_r0x00466a50:
                    local_c = uVar13;
                    if ((int)local_c <= iVar9) {
                        return;
                    }
                }
                goto LAB_00466a5a;
            }
            iVar10 = ISUB(iVar16, iVar9);
            if ((float)iVar10 * *pfVar4 + fVar7 <= PF(param_4, 0x28) * (float)iVar10 + fVar6) {
                *piVar1 = iVar9;
                if (*piVar2 == iVar9) {
                    CLR(*piVar3);
                    if (puVar20 == (unsigned int *)0) {
                        LOADTBL();
                        TBLP() = (unsigned int *)*puVar21;
                        local_28 = puVar20;
                    }
                    else {
                        *puVar20 = *puVar21;
                        local_28 = puVar20;
                    }
                }
                goto LAB_00466a5a;
            }
            iVar11 = (int)(((*pfVar4 * (float)iVar10 + fVar7) -
                            (PF(param_4, 0x28) * (float)iVar10 + fVar6)) /
                           (*pfVar4 - PF(param_4, 0x28)));
            if (iVar11 < 0) {
                iVar11 = 0;
            }
            if (iVar10 < iVar11) {
                iVar11 = iVar10;
            }
            uVar13 = (unsigned int)ISUB(iVar16, iVar11);
            iVar10 = ISUB((int)uVar13, iVar9);
            iVar14 = ISUB(iVar9, iVar12);
            iVar18 = (int)(local_c - iVar16);
            if (((iVar11 < iVar14) || (iVar10 < iVar14)) || (iVar18 < iVar14)) {
                if (((iVar14 < iVar11) || (iVar10 < iVar11)) || (iVar18 < iVar11)) {
                    if (((iVar10 <= iVar14) && (iVar10 <= iVar11)) && (iVar10 <= iVar18))
                        goto joined_r0x004669aa;
                    goto joined_r0x00466a50;
                }
                *piVar1 = iVar9;
                if (*piVar2 == iVar9) {
                    CLR(*piVar3);
                    if (puVar20 == (unsigned int *)0) {
                        LOADTBL();
                        TBLP() = (unsigned int *)*puVar21;
                        local_28 = puVar20;
                    }
                    else {
                        *puVar20 = *puVar21;
                        local_28 = puVar20;
                    }
                }
                goto LAB_00466a5a;
            }
            iVar11 = ISUB(ISUB(iVar16, iVar12), iVar11);
            *piVar2 = IADD(iVar11, iVar12);
            SF(puVar21, 3) = (float)iVar11 * SF(puVar21, 4) + SF(puVar21, 3);
            puVar15 = (unsigned int *)*puVar21;
            if ((puVar15 == (unsigned int *)0) || (*piVar2 < (int)puVar15[1])) break;
            if (puVar20 == (unsigned int *)0) {
                LOADTBL();
                TBLP() = puVar15;
            }
            else {
                *puVar20 = (unsigned int)puVar15;
            }
            iVar16 = *piVar2;
            if (iVar16 < (int)puVar15[2]) {
                iVar12 = ISUB(puVar15[2], iVar16);
                *piVar2 = IADD(iVar16, iVar12);
                SF(puVar21, 3) = (float)iVar12 * SF(puVar21, 4) + SF(puVar21, 3);
            }
            if (*piVar2 < *piVar1) {
                puVar17 = (unsigned int *)*puVar15;
                puVar19 = puVar17;
                if (puVar17 != (unsigned int *)0) {
                    while (iVar16 = *piVar2, puVar19 = puVar17, (int)puVar17[1] < iVar16) {
                        if (iVar16 < (int)puVar17[2]) {
                            iVar12 = ISUB(puVar17[2], iVar16);
                            *piVar2 = IADD(iVar16, iVar12);
                            SF(puVar21, 3) = (float)iVar12 * SF(puVar21, 4) + SF(puVar21, 3);
                        }
                        if ((*piVar1 <= *piVar2) ||
                            (puVar19 = (unsigned int *)*puVar17, puVar15 = puVar17, puVar17 = puVar19,
                             puVar19 == (unsigned int *)0)) break;
                    }
                }
                if (*piVar1 <= *piVar2) goto LAB_004668e5;
                *puVar15 = (unsigned int)puVar21;
                *puVar21 = (unsigned int)puVar19;
            }
            else {
LAB_004668e5:
                CLR(*piVar3);
            }
            if (puVar20 == (unsigned int *)0) {
                LOADTBL();
                puVar21 = TBLP();
            }
            else {
                puVar21 = (unsigned int *)*puVar20;
            }
        }
        else {
LAB_00466a5a:
            puVar21 = (unsigned int *)*puVar21;
            puVar20 = local_28;
            iVar9 = local_24;
        }
    }
    __builtin_memcpy(&puVar15, &DAT_006a882a, 4);
    puVar17 = puVar15 + 6;
    __builtin_memcpy(&puVar22, &DAT_006a882e, 4);
    if (puVar15 + 6 == puVar22) {
        puVar17 = puVar15;
    }
    __builtin_memcpy(&DAT_006a882a, &puVar17, 4);
    puVar15[1] = (unsigned int)iVar9;
    puVar15[2] = local_c;
    puVar15[5] = (unsigned int)param_4;
    if (*(int *)(param_4 + 0x38) == 0) {
        puVar15[3] = 0;
    }
    else {
        SF(puVar15, 3) = (float)iVar9;
        SF(puVar15, 3) = SF(puVar15, 3) - PF(*(int *)(param_4 + 0x38), 8);
        SF(puVar15, 3) = SF(puVar15, 3) * PF(param_4, 0x34);
        SF(puVar15, 3) = PF(*(int *)(param_4 + 0x38), 0xc) + SF(puVar15, 3);
    }
    puVar15[4] = *(unsigned int *)(param_4 + 0x34);
    *(unsigned int **)(*(int *)(param_4 + 0x68) + (param_3 - *(int *)(param_4 + 0x40)) * 4) = puVar15;
    if (puVar20 == (unsigned int *)0) {
        LOADTBL();
        TBLP() = puVar15;
    }
    else {
        *puVar20 = (unsigned int)puVar15;
    }
    local_28 = puVar15;
    *puVar15 = (unsigned int)puVar21;
    if (puVar21 != (unsigned int *)0) {
        while (1) {
            iVar9 = (int)puVar21[1];
            piVar1 = (int *)(puVar21 + 1);
            iVar16 = (int)puVar15[2];
            if (iVar16 <= iVar9) break;
            iVar12 = (int)puVar21[5];
            piVar2 = (int *)(puVar21 + 5);
            if (PF(iVar12, 0x4c) < PF(param_4, 0x48)) {
                if (PF(param_4, 0x4c) < PF(iVar12, 0x48)) {
                    fVar6 = PF(param_4, 0x28) * (float)iVar9 +
                            PF(param_4, 0x2c) * (float)(int)param_3 + PF(param_4, 0x30);
                    pfVar4 = (float *)(iVar12 + 0x28);
                    fVar7 = PF(iVar12, 0x28) * (float)iVar9 +
                            (float)(int)param_3 * PF(iVar12, 0x2c) + PF(iVar12, 0x30);
                    if (fVar7 < fVar6) {
                        piVar3 = (int *)(puVar21 + 2);
                        if (PF(param_4, 0x28) < *pfVar4) {
                            if (iVar16 < *piVar3) {
                                iVar16 = ISUB(iVar16, iVar9);
                                fVar8 = (float)iVar16;
                                if (fVar8 * PF(param_4, 0x28) + fVar6 < *pfVar4 * fVar8 + fVar7) {
                                    iVar12 = (int)((fVar6 - fVar7) / (*pfVar4 - PF(param_4, 0x28)));
                                    if (iVar12 < 0) {
                                        iVar12 = 0;
                                    }
                                    if (iVar16 < iVar12) {
                                        iVar12 = iVar16;
                                    }
                                    pfVar4 = (float *)(puVar21 + 3);
                                    pfVar5 = (float *)(puVar21 + 4);
                                    *piVar1 = IADD(iVar9, iVar12);
                                    *pfVar4 = (float)iVar12 * *pfVar5 + *pfVar4;
                                    puVar20 = (unsigned int *)*puVar21;
                                    if ((puVar20 != (unsigned int *)0) && ((int)puVar20[1] < *piVar1)) {
                                        *local_28 = (unsigned int)puVar20;
                                        iVar9 = *piVar1;
                                        if (iVar9 < (int)puVar20[2]) {
                                            iVar16 = ISUB(puVar20[2], iVar9);
                                            *piVar1 = IADD(iVar9, iVar16);
                                            *pfVar4 = (float)iVar16 * *pfVar5 + *pfVar4;
                                        }
                                        if (*piVar1 < *piVar3) {
                                            puVar17 = (unsigned int *)*puVar20;
                                            if (puVar17 != (unsigned int *)0) {
                                                while (puVar19 = puVar17, iVar9 = *piVar1, puVar17 = puVar19,
                                                       (int)puVar19[1] < iVar9) {
                                                    if (iVar9 < (int)puVar19[2]) {
                                                        iVar16 = ISUB(puVar19[2], iVar9);
                                                        *piVar1 = IADD(iVar9, iVar16);
                                                        *pfVar4 = (float)iVar16 * *pfVar5 + *pfVar4;
                                                    }
                                                    if ((*piVar3 <= *piVar1) ||
                                                        (puVar17 = (unsigned int *)*puVar19, puVar20 = puVar19,
                                                         puVar17 == (unsigned int *)0)) break;
                                                }
                                            }
                                            if (*piVar3 <= *piVar1) goto LAB_004674cd;
                                            goto LAB_004674c1;
                                        }
                                        goto LAB_004674ee;
                                    }
                                }
                                else {
                                    pfVar4 = (float *)(puVar21 + 3);
                                    pfVar5 = (float *)(puVar21 + 4);
                                    *piVar1 = IADD(iVar9, iVar16);
                                    *pfVar4 = *pfVar5 * fVar8 + *pfVar4;
                                    puVar20 = (unsigned int *)*puVar21;
                                    if ((puVar20 != (unsigned int *)0) && ((int)puVar20[1] < *piVar1)) {
                                        *local_28 = (unsigned int)puVar20;
                                        iVar9 = *piVar1;
                                        if (iVar9 < (int)puVar20[2]) {
                                            iVar16 = ISUB(puVar20[2], iVar9);
                                            *piVar1 = IADD(iVar9, iVar16);
                                            *pfVar4 = (float)iVar16 * *pfVar5 + *pfVar4;
                                        }
                                        if (*piVar1 < *piVar3) {
                                            puVar17 = (unsigned int *)*puVar20;
                                            if (puVar17 != (unsigned int *)0) {
                                                while (puVar19 = puVar17, iVar9 = *piVar1, puVar17 = puVar19,
                                                       (int)puVar19[1] < iVar9) {
                                                    if (iVar9 < (int)puVar19[2]) {
                                                        iVar16 = ISUB(puVar19[2], iVar9);
                                                        *piVar1 = IADD(iVar9, iVar16);
                                                        *pfVar4 = (float)iVar16 * *pfVar5 + *pfVar4;
                                                    }
                                                    if ((*piVar3 <= *piVar1) ||
                                                        (puVar17 = (unsigned int *)*puVar19, puVar20 = puVar19,
                                                         puVar17 == (unsigned int *)0)) break;
                                                }
                                            }
                                            if (*piVar1 < *piVar3) {
                                                *puVar20 = (unsigned int)puVar21;
                                                *puVar21 = (unsigned int)puVar17;
                                                puVar20 = (unsigned int *)*local_28;
                                                puVar21 = local_28;
                                            }
                                            else {
                                                CLR(*piVar2);
                                                puVar20 = (unsigned int *)*local_28;
                                                puVar21 = local_28;
                                            }
                                        }
                                        else {
                                            CLR(*piVar2);
                                            puVar20 = (unsigned int *)*local_28;
                                            puVar21 = local_28;
                                        }
                                    }
                                }
                            }
                            else {
                                iVar16 = ISUB(*piVar3, iVar9);
                                fVar8 = (float)iVar16;
                                if (PF(param_4, 0x28) * fVar8 + fVar6 < fVar8 * *pfVar4 + fVar7) {
                                    iVar12 = (int)(((*pfVar4 * fVar8 + fVar7) -
                                                    (PF(param_4, 0x28) * fVar8 + fVar6)) /
                                                   (*pfVar4 - PF(param_4, 0x28)));
                                    if (iVar12 < 0) {
                                        iVar12 = 0;
                                    }
                                    iVar16 = ISUB(iVar16, iVar12);
                                    if (iVar16 < 0) {
                                        iVar16 = 0;
                                    }
                                    pfVar4 = (float *)(puVar21 + 3);
                                    pfVar5 = (float *)(puVar21 + 4);
                                    *piVar1 = IADD(iVar9, iVar16);
                                    *pfVar4 = (float)iVar16 * *pfVar5 + *pfVar4;
                                    if (*piVar1 < *piVar3) {
                                        puVar20 = (unsigned int *)*puVar21;
                                        if ((puVar20 != (unsigned int *)0) && ((int)puVar20[1] < *piVar1)) {
                                            *local_28 = (unsigned int)puVar20;
                                            iVar9 = *piVar1;
                                            if (iVar9 < (int)puVar20[2]) {
                                                iVar16 = ISUB(puVar20[2], iVar9);
                                                *piVar1 = IADD(iVar9, iVar16);
                                                *pfVar4 = (float)iVar16 * *pfVar5 + *pfVar4;
                                            }
                                            if (*piVar1 < *piVar3) {
                                                puVar17 = (unsigned int *)*puVar20;
                                                if (puVar17 != (unsigned int *)0) {
                                                    while (puVar19 = puVar17, iVar9 = *piVar1, puVar17 = puVar19,
                                                           (int)puVar19[1] < iVar9) {
                                                        if (iVar9 < (int)puVar19[2]) {
                                                            iVar16 = ISUB(puVar19[2], iVar9);
                                                            *piVar1 = IADD(iVar9, iVar16);
                                                            *pfVar4 = (float)iVar16 * *pfVar5 + *pfVar4;
                                                        }
                                                        if ((*piVar3 <= *piVar1) ||
                                                            (puVar17 = (unsigned int *)*puVar19, puVar20 = puVar19,
                                                             puVar17 == (unsigned int *)0)) break;
                                                    }
                                                }
                                                if (*piVar1 < *piVar3) {
                                                    *puVar20 = (unsigned int)puVar21;
                                                    *puVar21 = (unsigned int)puVar17;
                                                    puVar20 = (unsigned int *)*local_28;
                                                    puVar21 = local_28;
                                                }
                                                else {
                                                    CLR(*piVar2);
                                                    puVar20 = (unsigned int *)*local_28;
                                                    puVar21 = local_28;
                                                }
                                            }
                                            else {
                                                CLR(*piVar2);
                                                puVar20 = (unsigned int *)*local_28;
                                                puVar21 = local_28;
                                            }
                                        }
                                    }
                                    else {
                                        CLR(*piVar2);
                                        *local_28 = *puVar21;
                                        puVar20 = (unsigned int *)*puVar21;
                                        puVar21 = local_28;
                                    }
                                }
                                else {
                                    CLR(iVar12);
                                    *local_28 = *puVar21;
                                    puVar20 = (unsigned int *)*puVar21;
                                    puVar21 = local_28;
                                }
                            }
                        }
                        else if (iVar16 < *piVar3) {
                            pfVar4 = (float *)(puVar21 + 3);
                            pfVar5 = (float *)(puVar21 + 4);
                            *piVar1 = IADD(iVar9, ISUB(iVar16, iVar9));
                            *pfVar4 = (float)ISUB(iVar16, iVar9) * *pfVar5 + *pfVar4;
                            puVar20 = (unsigned int *)*puVar21;
                            if ((puVar20 != (unsigned int *)0) && ((int)puVar20[1] < *piVar1)) {
                                *local_28 = (unsigned int)puVar20;
                                iVar9 = *piVar1;
                                if (iVar9 < (int)puVar20[2]) {
                                    iVar16 = ISUB(puVar20[2], iVar9);
                                    *piVar1 = IADD(iVar9, iVar16);
                                    *pfVar4 = (float)iVar16 * *pfVar5 + *pfVar4;
                                }
                                if (*piVar1 < *piVar3) {
                                    puVar17 = (unsigned int *)*puVar20;
                                    if (puVar17 != (unsigned int *)0) {
                                        while (puVar19 = puVar17, iVar9 = *piVar1, puVar17 = puVar19,
                                               (int)puVar19[1] < iVar9) {
                                            if (iVar9 < (int)puVar19[2]) {
                                                iVar16 = ISUB(puVar19[2], iVar9);
                                                *piVar1 = IADD(iVar9, iVar16);
                                                *pfVar4 = (float)iVar16 * *pfVar5 + *pfVar4;
                                            }
                                            if ((*piVar3 <= *piVar1) ||
                                                (puVar17 = (unsigned int *)*puVar19, puVar20 = puVar19,
                                                 puVar17 == (unsigned int *)0)) break;
                                        }
                                    }
                                    if (*piVar1 < *piVar3) {
                                        *puVar20 = (unsigned int)puVar21;
                                        *puVar21 = (unsigned int)puVar17;
                                        puVar20 = (unsigned int *)*local_28;
                                        puVar21 = local_28;
                                    }
                                    else {
                                        CLR(*piVar2);
                                        puVar20 = (unsigned int *)*local_28;
                                        puVar21 = local_28;
                                    }
                                }
                                else {
                                    CLR(*piVar2);
                                    puVar20 = (unsigned int *)*local_28;
                                    puVar21 = local_28;
                                }
                            }
                        }
                        else {
                            CLR(iVar12);
                            *local_28 = *puVar21;
                            puVar20 = (unsigned int *)*puVar21;
                            puVar21 = local_28;
                        }
                    }
                    else if (*pfVar4 < PF(param_4, 0x28)) {
                        iVar12 = (int)puVar21[2];
                        piVar3 = (int *)(puVar21 + 2);
                        if (iVar12 < iVar16) {
                            if ((float)ISUB(iVar12, iVar9) * *pfVar4 + fVar7 <
                                PF(param_4, 0x28) * (float)ISUB(iVar12, iVar9) + fVar6) {
                                iVar9 = (int)(((PF(param_4, 0x28) * (float)ISUB(iVar12, iVar9) + fVar6) -
                                               (*pfVar4 * (float)ISUB(iVar12, iVar9) + fVar7)) /
                                              (PF(param_4, 0x28) - *pfVar4));
                                if (iVar9 < 0) {
                                    iVar9 = 0;
                                }
                                *piVar3 = ISUB(iVar12, iVar9);
                                if (*piVar1 < ISUB(iVar12, iVar9)) {
                                    puVar20 = (unsigned int *)*puVar21;
                                }
                                else {
                                    CLR(*piVar2);
                                    *local_28 = *puVar21;
                                    puVar20 = (unsigned int *)*puVar21;
                                    puVar21 = local_28;
                                }
                            }
                            else {
                                puVar20 = (unsigned int *)*puVar21;
                            }
                        }
                        else {
                            iVar10 = ISUB(iVar16, iVar9);
                            fVar8 = (float)iVar10;
                            if (fVar8 * PF(param_4, 0x28) + fVar6 <= *pfVar4 * fVar8 + fVar7) {
                                puVar15[2] = puVar21[1];
                                return;
                            }
                            iVar11 = (int)(((PF(param_4, 0x28) * fVar8 + fVar6) -
                                            (*pfVar4 * fVar8 + fVar7)) /
                                           (PF(param_4, 0x28) - *pfVar4));
                            if (iVar11 < 0) {
                                iVar11 = 0;
                            }
                            if (iVar10 < iVar11) {
                                iVar11 = iVar10;
                            }
                            if ((iVar11 < ISUB(iVar10, iVar11)) && (iVar11 < ISUB(iVar12, iVar16))) {
                                puVar15[2] = puVar21[1];
                                return;
                            }
                            if (ISUB(iVar10, iVar11) < ISUB(iVar12, iVar16)) {
                                pfVar4 = (float *)(puVar21 + 3);
                                *piVar1 = IADD(iVar10, iVar9);
                                pfVar5 = (float *)(puVar21 + 4);
                                *pfVar4 = *pfVar5 * fVar8 + *pfVar4;
                                puVar20 = (unsigned int *)*puVar21;
                                if ((puVar20 != (unsigned int *)0) && ((int)puVar20[1] < *piVar1)) {
                                    *local_28 = (unsigned int)puVar20;
                                    iVar9 = *piVar1;
                                    if (iVar9 < (int)puVar20[2]) {
                                        iVar16 = ISUB(puVar20[2], iVar9);
                                        *piVar1 = IADD(iVar9, iVar16);
                                        *pfVar4 = (float)iVar16 * *pfVar5 + *pfVar4;
                                    }
                                    if (*piVar1 < *piVar3) {
                                        puVar17 = (unsigned int *)*puVar20;
                                        if (puVar17 != (unsigned int *)0) {
                                            while (puVar19 = puVar17, iVar9 = *piVar1, puVar17 = puVar19,
                                                   (int)puVar19[1] < iVar9) {
                                                if (iVar9 < (int)puVar19[2]) {
                                                    iVar16 = ISUB(puVar19[2], iVar9);
                                                    *piVar1 = IADD(iVar9, iVar16);
                                                    *pfVar4 = (float)iVar16 * *pfVar5 + *pfVar4;
                                                }
                                                if ((*piVar3 <= *piVar1) ||
                                                    (puVar17 = (unsigned int *)*puVar19, puVar20 = puVar19,
                                                     puVar17 == (unsigned int *)0)) break;
                                            }
                                        }
                                        if (*piVar1 < *piVar3) {
                                            *puVar20 = (unsigned int)puVar21;
                                            *puVar21 = (unsigned int)puVar17;
                                            puVar20 = (unsigned int *)*local_28;
                                            puVar21 = local_28;
                                        }
                                        else {
                                            CLR(*piVar2);
                                            puVar20 = (unsigned int *)*local_28;
                                            puVar21 = local_28;
                                        }
                                    }
                                    else {
                                        CLR(*piVar2);
                                        puVar20 = (unsigned int *)*local_28;
                                        puVar21 = local_28;
                                    }
                                }
                            }
                            else {
                                *piVar3 = ISUB(iVar16, iVar11);
                                puVar20 = (unsigned int *)*puVar21;
                            }
                        }
                    }
                    else {
                        if (iVar16 <= (int)puVar21[2]) {
                            puVar15[2] = puVar21[1];
                            return;
                        }
                        puVar20 = (unsigned int *)*puVar21;
                    }
                }
                else {
                    piVar3 = (int *)(puVar21 + 2);
                    if (iVar16 < (int)puVar21[2]) {
                        *piVar1 = IADD(iVar9, ISUB(iVar16, iVar9));
                        SF(puVar21, 3) = (float)ISUB(iVar16, iVar9) * SF(puVar21, 4) + SF(puVar21, 3);
                        puVar20 = (unsigned int *)*puVar21;
                        if ((puVar20 != (unsigned int *)0) && ((int)puVar20[1] < *piVar1)) {
                            *local_28 = (unsigned int)puVar20;
                            iVar9 = *piVar1;
                            if (iVar9 < (int)puVar20[2]) {
                                iVar16 = ISUB(puVar20[2], iVar9);
                                *piVar1 = IADD(iVar9, iVar16);
                                SF(puVar21, 3) = (float)iVar16 * SF(puVar21, 4) + SF(puVar21, 3);
                            }
                            if (*piVar1 < *piVar3) {
                                puVar17 = (unsigned int *)*puVar20;
                                if (puVar17 != (unsigned int *)0) {
                                    while (puVar19 = puVar17, iVar9 = *piVar1, puVar17 = puVar19,
                                           (int)puVar19[1] < iVar9) {
                                        if (iVar9 < (int)puVar19[2]) {
                                            iVar16 = ISUB(puVar19[2], iVar9);
                                            *piVar1 = IADD(iVar9, iVar16);
                                            SF(puVar21, 3) = (float)iVar16 * SF(puVar21, 4) + SF(puVar21, 3);
                                        }
                                        if ((*piVar3 <= *piVar1) ||
                                            (puVar17 = (unsigned int *)*puVar19, puVar20 = puVar19,
                                             puVar17 == (unsigned int *)0)) break;
                                    }
                                }
                                if (*piVar1 < *piVar3) {
LAB_004674c1:
                                    *puVar20 = (unsigned int)puVar21;
                                    *puVar21 = (unsigned int)puVar17;
                                    puVar20 = (unsigned int *)*local_28;
                                    puVar21 = local_28;
                                }
                                else {
LAB_004674cd:
                                    CLR(*piVar2);
                                    puVar20 = (unsigned int *)*local_28;
                                    puVar21 = local_28;
                                }
                            }
                            else {
LAB_004674ee:
                                CLR(*piVar2);
                                puVar20 = (unsigned int *)*local_28;
                                puVar21 = local_28;
                            }
                        }
                    }
                    else {
                        CLR(iVar12);
                        *local_28 = *puVar21;
                        puVar20 = (unsigned int *)*puVar21;
                        puVar21 = local_28;
                    }
                }
            }
            else {
                if (iVar16 <= (int)puVar21[2]) {
                    puVar15[2] = puVar21[1];
                    return;
                }
                puVar20 = (unsigned int *)*puVar21;
            }
            local_28 = puVar21;
            puVar21 = puVar20;
            if (puVar20 == (unsigned int *)0) {
                return;
            }
        }
    }
    return;
}

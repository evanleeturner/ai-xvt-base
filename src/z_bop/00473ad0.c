/* SPDX-License-Identifier: MIT */
/* z_bop 00473ad0 */
#define F_AT(i) (*(float *)((int)((uint)iVar1 + (uint)(i) * 4u)))
#define I_ADD(a, b) ((int)((uint)(a) + (uint)(b)))
#define I_MUL(a, b) ((int)((uint)(a) * (uint)(b)))

undefined4 __cdecl FUN_00473ad0(int param_1, float *param_2, float *param_3)
{
    int iVar1;
    float fVar2;
    float fVar3;
    float fVar4;
    float fVar5;
    float fVar6;
    float fVar7;
    int iVar8;
    float fVar9;
    float fVar10;
    int iVar11;
    int iVar12;
    int iVar13;
    int iVar14;
    int iVar15;
    int iVar16;
    int iVar17;
    int iVar18;
    int iVar19;
    int local_3c;
    float local_1c;
    float local_18;
    int *local_14;
    int local_10;
    float *local_8;
    int local_4;
    uint u9;
    uint u10;

    iVar1 = *(int *)(param_1 + 0x7c);
    fVar2 = *param_2;
    fVar3 = param_2[1];
    fVar4 = param_2[2];
    local_4 = 0;
    fVar5 = *param_3;
    fVar6 = param_3[1];
    fVar7 = param_3[2];
    local_14 = *(int **)(param_1 + 0xa0);
    local_8 = *(float **)(param_1 + 0x94);
    if (0 < *(int *)(param_1 + 0x8c)) {
        do {
            iVar12 = local_14[3];
            iVar8 = *local_14;
            iVar13 = I_MUL(iVar8, 3);
            iVar14 = local_14[1];
            iVar11 = I_MUL(iVar14, 3);
            iVar16 = local_14[2];
            iVar19 = I_MUL(iVar16, 3);
            iVar15 = I_MUL(iVar12, 3);

            if ((fVar2 < F_AT(iVar13)) || (fVar5 < F_AT(iVar13))) {
                if ((F_AT(iVar13) < fVar2) ||
                    (F_AT(iVar13) < fVar5) ||
                    (F_AT(iVar11) < fVar2) ||
                    (F_AT(iVar11) < fVar5) ||
                    (F_AT(iVar19) < fVar2) ||
                    (F_AT(iVar19) < fVar5) ||
                    ((iVar15 != -3) && ((F_AT(iVar15) < fVar2) || (F_AT(iVar15) < fVar5)))) {
                    goto LAB_00473ca5;
                }
            }
            else if ((fVar2 < F_AT(iVar11)) ||
                     (fVar5 < F_AT(iVar11)) ||
                     (fVar2 < F_AT(iVar19)) ||
                     (fVar5 < F_AT(iVar19)) ||
                     ((iVar15 != -3) && ((fVar2 < F_AT(iVar15)) || (fVar5 < F_AT(iVar15))))) {
LAB_00473ca5:
                iVar12 = I_ADD(iVar13, 1);
                iVar16 = I_ADD(iVar11, 1);
                iVar18 = I_ADD(iVar19, 1);
                iVar14 = I_ADD(iVar15, 1);
                if ((fVar3 < F_AT(iVar12)) || (fVar6 < F_AT(iVar12))) {
                    if ((F_AT(iVar12) < fVar3) ||
                        (F_AT(iVar12) < fVar6) ||
                        (F_AT(iVar16) < fVar3) ||
                        (F_AT(iVar16) < fVar6) ||
                        (F_AT(iVar18) < fVar3) ||
                        (F_AT(iVar18) < fVar6) ||
                        ((iVar14 != -2) && ((F_AT(iVar14) < fVar3) || (F_AT(iVar14) < fVar6)))) {
                        goto LAB_00473de3;
                    }
                }
                else if ((fVar3 < F_AT(iVar16)) ||
                         (fVar6 < F_AT(iVar16)) ||
                         (fVar3 < F_AT(iVar18)) ||
                         (fVar6 < F_AT(iVar18)) ||
                         ((iVar14 != -2) && ((fVar3 < F_AT(iVar14)) || (fVar6 < F_AT(iVar14))))) {
LAB_00473de3:
                    iVar18 = I_ADD(iVar13, 2);
                    iVar17 = I_ADD(iVar11, 2);
                    iVar14 = I_ADD(iVar19, 2);
                    iVar16 = I_ADD(iVar15, 2);
                    if ((fVar4 < F_AT(iVar18)) || (fVar7 < F_AT(iVar18))) {
                        if ((F_AT(iVar18) < fVar4) ||
                            (F_AT(iVar18) < fVar7) ||
                            (F_AT(iVar17) < fVar4) ||
                            (F_AT(iVar17) < fVar7) ||
                            (F_AT(iVar14) < fVar4) ||
                            (F_AT(iVar14) < fVar7) ||
                            ((iVar16 != -1) && ((F_AT(iVar16) < fVar4) || (F_AT(iVar16) < fVar7)))) {
                            goto LAB_00473f21;
                        }
                    }
                    else if ((fVar4 < F_AT(iVar17)) ||
                             (fVar7 < F_AT(iVar17)) ||
                             (fVar4 < F_AT(iVar14)) ||
                             (fVar7 < F_AT(iVar14)) ||
                             ((iVar16 != -1) && ((fVar4 < F_AT(iVar16)) || (fVar7 < F_AT(iVar16))))) {
LAB_00473f21:
                        fVar9 = local_8[1] * (fVar3 - F_AT(I_ADD(iVar18, -1))) +
                                local_8[2] * (fVar4 - F_AT(iVar18)) +
                                (fVar2 - F_AT(iVar13)) * local_8[0];
                        fVar10 = (fVar5 - F_AT(iVar13)) * local_8[0] +
                                 local_8[2] * (fVar7 - F_AT(iVar18)) +
                                 local_8[1] * (fVar6 - F_AT(I_ADD(iVar18, -1)));
                        __builtin_memcpy(&u9, &fVar9, 4);
                        __builtin_memcpy(&u10, &fVar10, 4);
                        if (u9 < 0x80000001u) {
                            if ((0x421fffff < (int)u9) && (0x80000000u < u10)) {
                                goto LAB_00473fd0;
                            }
                        }
                        else if ((0xc21fffffu < u9) && (0 < (int)u10)) {
LAB_00473fd0:
                            fVar10 = -fVar9 / fVar10;
                            if ((local_8[2] <= local_8[0]) || (local_8[2] <= local_8[1])) {
                                local_3c = iVar17;
                                if ((local_8[1] <= local_8[0]) || (local_8[1] <= local_8[2])) {
                                    local_1c = (fVar6 - fVar3) * fVar10 + fVar3;
                                    local_18 = (fVar7 - fVar4) * fVar10 + fVar4;
                                    local_10 = I_ADD(iVar13, 2);
                                    iVar12 = -1;
                                    iVar19 = I_ADD(iVar19, 1);
                                    iVar15 = -1;
                                    iVar13 = I_ADD(iVar13, 1);
                                    goto LAB_004740f9;
                                }
                                local_1c = (fVar5 - fVar2) * fVar10 + fVar2;
                                local_18 = (fVar7 - fVar4) * fVar10 + fVar4;
                                local_10 = I_ADD(iVar13, 2);
                                iVar12 = -2;
                            }
                            else {
                                local_1c = (fVar5 - fVar2) * fVar10 + fVar2;
                                local_18 = (fVar6 - fVar3) * fVar10 + fVar3;
                                iVar12 = -2;
                                iVar15 = -2;
                                iVar14 = I_ADD(iVar19, 1);
                                local_3c = I_ADD(iVar11, 1);
                                local_10 = I_ADD(iVar13, 1);
LAB_004740f9:
                                iVar15 = I_ADD(iVar16, iVar15);
                                iVar16 = I_ADD(iVar15, 1);
                            }
                            iVar17 = I_ADD(iVar17, iVar12);
                            fVar9 = (F_AT(local_3c) - F_AT(local_10)) * (local_1c - F_AT(iVar13)) -
                                    (local_18 - F_AT(local_10)) * (F_AT(iVar17) - F_AT(iVar13));
                            fVar10 = (F_AT(iVar14) - F_AT(local_3c)) * (local_1c - F_AT(iVar17)) -
                                     (local_18 - F_AT(local_3c)) * (F_AT(iVar19) - F_AT(iVar17));
                            __builtin_memcpy(&u9, &fVar9, 4);
                            __builtin_memcpy(&u10, &fVar10, 4);
                            if (u9 < 0x80000001u) {
                                if (u10 < 0x80000001u) goto LAB_00474181;
                            }
                            else if (0x80000000u < u10) {
LAB_00474181:
                                fVar10 = local_1c - F_AT(iVar19);
                                if (iVar15 < 0) {
                                    fVar10 = (F_AT(local_10) - F_AT(iVar14)) * fVar10 -
                                             (local_18 - F_AT(iVar14)) *
                                             (F_AT(iVar13) - F_AT(iVar19));
                                    __builtin_memcpy(&u9, &fVar9, 4);
                                    __builtin_memcpy(&u10, &fVar10, 4);
                                    if (u9 < 0x80000001u) {
                                        if (u10 < 0x80000001u) {
                                            return 1;
                                        }
                                    }
                                    else if (0x80000000u < u10) {
                                        return 1;
                                    }
                                }
                                else {
                                    fVar10 = (F_AT(iVar16) - F_AT(iVar14)) * fVar10 -
                                             (local_18 - F_AT(iVar14)) *
                                             (F_AT(iVar15) - F_AT(iVar19));
                                    __builtin_memcpy(&u9, &fVar9, 4);
                                    __builtin_memcpy(&u10, &fVar10, 4);
                                    if (u9 < 0x80000001u) {
                                        if (u10 < 0x80000001u) goto LAB_0047421e;
                                    }
                                    else if (0x80000000u < u10) {
LAB_0047421e:
                                        fVar10 = (F_AT(local_10) - F_AT(iVar16)) *
                                                 (local_1c - F_AT(iVar15)) -
                                                 (local_18 - F_AT(iVar16)) *
                                                 (F_AT(iVar13) - F_AT(iVar15));
                                        __builtin_memcpy(&u9, &fVar9, 4);
                                        __builtin_memcpy(&u10, &fVar10, 4);
                                        if (u9 < 0x80000001u) {
                                            if (u10 < 0x80000001u) {
                                                return 1;
                                            }
                                        }
                                        else if (0x80000000u < u10) {
                                            return 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            local_4 = local_4 + 1;
            local_14 = local_14 + 0x10;
            local_8 = local_8 + 3;
        } while (local_4 < *(int *)(param_1 + 0x8c));
    }
    return 0;
}

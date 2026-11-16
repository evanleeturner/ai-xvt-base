/* SPDX-License-Identifier: MIT */
/* z_xvt 00487cb0 */
undefined4 __cdecl FUN_00487cb0(int param_1, float *param_2, float *param_3)
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

#define FL(a) (*(float *)((unsigned int)iVar1 + (unsigned int)(a)))
#define FI(i) FL((unsigned int)(i) * 4u)
#define BITS(x) (*(unsigned int *)&(x))

    iVar1 = *(int *)((unsigned int)param_1 + 0x7cu);
    fVar2 = *param_2;
    fVar3 = param_2[1];
    fVar4 = param_2[2];
    local_4 = 0;
    fVar5 = *param_3;
    fVar6 = param_3[1];
    fVar7 = param_3[2];
    local_14 = *(int **)((unsigned int)param_1 + 0xa0u);
    local_8 = *(float **)((unsigned int)param_1 + 0x94u);
    if (0 < *(int *)((unsigned int)param_1 + 0x8cu)) {
        do {
            iVar12 = local_14[3];
            iVar8 = *local_14;
            iVar13 = (int)((unsigned int)iVar8 * 3u);
            iVar14 = local_14[1];
            iVar11 = (int)((unsigned int)iVar14 * 3u);
            iVar16 = local_14[2];
            iVar19 = (int)((unsigned int)iVar16 * 3u);
            iVar15 = (int)((unsigned int)iVar12 * 3u);
            if ((fVar2 < FL((unsigned int)iVar8 * 0xcu)) || (fVar5 < FL((unsigned int)iVar8 * 0xcu))) {
                if ((FL((unsigned int)iVar8 * 0xcu) < fVar2) ||
                    (FL((unsigned int)iVar8 * 0xcu) < fVar5) ||
                    (FL((unsigned int)iVar14 * 0xcu) < fVar2) ||
                    (FL((unsigned int)iVar14 * 0xcu) < fVar5) ||
                    (FL((unsigned int)iVar16 * 0xcu) < fVar2) ||
                    (FL((unsigned int)iVar16 * 0xcu) < fVar5) ||
                    ((iVar15 != -3) &&
                     ((FL((unsigned int)iVar12 * 0xcu) < fVar2) ||
                      (FL((unsigned int)iVar12 * 0xcu) < fVar5)))) {
                    goto LAB_00487e85;
                }
            }
            else if ((fVar2 < FL((unsigned int)iVar14 * 0xcu)) ||
                     (fVar5 < FL((unsigned int)iVar14 * 0xcu)) ||
                     (fVar2 < FL((unsigned int)iVar16 * 0xcu)) ||
                     (fVar5 < FL((unsigned int)iVar16 * 0xcu)) ||
                     ((iVar15 != -3) &&
                      ((fVar2 < FL((unsigned int)iVar12 * 0xcu)) ||
                       (fVar5 < FL((unsigned int)iVar12 * 0xcu))))) {
LAB_00487e85:
                iVar12 = (int)((unsigned int)iVar13 + 1u);
                iVar16 = (int)((unsigned int)iVar11 + 1u);
                iVar18 = (int)((unsigned int)iVar19 + 1u);
                iVar14 = (int)((unsigned int)iVar15 + 1u);
                if ((fVar3 < FL(4u + (unsigned int)iVar8 * 0xcu)) ||
                    (fVar6 < FI(iVar12))) {
                    if ((FI(iVar12) < fVar3) ||
                        (FI(iVar12) < fVar6) ||
                        (FI(iVar16) < fVar3) ||
                        (FI(iVar16) < fVar6) ||
                        (FI(iVar18) < fVar3) ||
                        (FI(iVar18) < fVar6) ||
                        ((iVar14 != -2) &&
                         ((FI(iVar14) < fVar3) ||
                          (FI(iVar14) < fVar6)))) {
                        goto LAB_00487fc3;
                    }
                }
                else if ((fVar3 < FI(iVar16)) ||
                         (fVar6 < FI(iVar16)) ||
                         (fVar3 < FI(iVar18)) ||
                         (fVar6 < FI(iVar18)) ||
                         ((iVar14 != -2) &&
                          ((fVar3 < FI(iVar14)) ||
                           (fVar6 < FI(iVar14))))) {
LAB_00487fc3:
                    iVar18 = (int)((unsigned int)iVar13 + 2u);
                    iVar17 = (int)((unsigned int)iVar11 + 2u);
                    iVar14 = (int)((unsigned int)iVar19 + 2u);
                    iVar16 = (int)((unsigned int)iVar15 + 2u);
                    if ((fVar4 < FL(4u + (unsigned int)iVar12 * 4u)) ||
                        (fVar7 < FI(iVar18))) {
                        if ((FI(iVar18) < fVar4) ||
                            (FI(iVar18) < fVar7) ||
                            (FI(iVar17) < fVar4) ||
                            (FI(iVar17) < fVar7) ||
                            (FI(iVar14) < fVar4) ||
                            (FI(iVar14) < fVar7) ||
                            ((iVar16 != -1) &&
                             ((FI(iVar16) < fVar4) ||
                              (FI(iVar16) < fVar7)))) {
                            goto LAB_00488101;
                        }
                    }
                    else if ((fVar4 < FI(iVar17)) ||
                             (fVar7 < FI(iVar17)) ||
                             (fVar4 < FI(iVar14)) ||
                             (fVar7 < FI(iVar14)) ||
                             ((iVar16 != -1) &&
                              ((fVar4 < FI(iVar16)) ||
                               (fVar7 < FI(iVar16))))) {
LAB_00488101:
                        fVar9 = local_8[1] * (fVar3 - FL(0xfffffffcu + (unsigned int)iVar18 * 4u)) +
                                local_8[2] * (fVar4 - FI(iVar18)) +
                                (fVar2 - FL((unsigned int)iVar8 * 0xcu)) * *local_8;
                        fVar10 = (fVar5 - FL((unsigned int)iVar8 * 0xcu)) * *local_8 +
                                 local_8[2] * (fVar7 - FI(iVar18)) +
                                 local_8[1] * (fVar6 - FL(0xfffffffcu + (unsigned int)iVar18 * 4u));
                        if (BITS(fVar9) < 0x80000001u) {
                            if ((0x421fffff < (int)BITS(fVar9)) && (0x80000000u < BITS(fVar10))) {
LAB_004881b0:
                                fVar10 = -fVar9 / fVar10;
                                if ((local_8[2] <= *local_8) || (local_8[2] <= local_8[1])) {
                                    local_3c = iVar17;
                                    if ((local_8[1] <= *local_8) || (local_8[1] <= local_8[2])) {
                                        local_1c = (fVar6 - fVar3) * fVar10 + fVar3;
                                        local_18 = (fVar7 - fVar4) * fVar10 + fVar4;
                                        local_10 = (int)((unsigned int)iVar13 + 2u);
                                        iVar12 = -1;
                                        iVar19 = (int)((unsigned int)iVar19 + 1u);
                                        iVar15 = -1;
                                        iVar13 = (int)((unsigned int)iVar13 + 1u);
                                        goto LAB_004882d9;
                                    }
                                    local_1c = (fVar5 - fVar2) * fVar10 + fVar2;
                                    local_18 = (fVar7 - fVar4) * fVar10 + fVar4;
                                    local_10 = (int)((unsigned int)iVar13 + 2u);
                                    iVar12 = -2;
                                }
                                else {
                                    local_1c = (fVar5 - fVar2) * fVar10 + fVar2;
                                    local_18 = (fVar6 - fVar3) * fVar10 + fVar3;
                                    iVar12 = -2;
                                    iVar15 = -2;
                                    iVar14 = (int)((unsigned int)iVar19 + 1u);
                                    local_3c = (int)((unsigned int)iVar11 + 1u);
                                    local_10 = (int)((unsigned int)iVar13 + 1u);
LAB_004882d9:
                                    iVar15 = (int)((unsigned int)iVar16 + (unsigned int)iVar15);
                                    iVar16 = (int)((unsigned int)iVar15 + 1u);
                                }
                                iVar17 = (int)((unsigned int)iVar17 + (unsigned int)iVar12);
                                fVar9 = (FI(local_3c) - FI(local_10)) *
                                        (local_1c - FI(iVar13)) -
                                        (local_18 - FI(local_10)) *
                                        (FI(iVar17) - FI(iVar13));
                                fVar10 = (local_1c - FI(iVar17)) *
                                         (FI(iVar14) - FI(local_3c)) -
                                         (local_18 - FI(local_3c)) *
                                         (FI(iVar19) - FI(iVar17));
                                if (BITS(fVar9) < 0x80000001u) {
                                    if (BITS(fVar10) < 0x80000001u) goto LAB_00488361;
                                }
                                else if (0x80000000u < BITS(fVar10)) {
LAB_00488361:
                                    if (iVar15 < 0) {
                                        fVar10 = (local_1c - FI(iVar19)) *
                                                 (FI(local_10) - FI(iVar14)) -
                                                 (local_18 - FI(iVar14)) *
                                                 (FI(iVar13) - FI(iVar19));
joined_r0x0048843f:
                                        if (BITS(fVar9) < 0x80000001u) {
                                            if (BITS(fVar10) < 0x80000001u) {
                                                return 1;
                                            }
                                        }
                                        else if (0x80000000u < BITS(fVar10)) {
                                            return 1;
                                        }
                                    }
                                    else {
                                        fVar10 = (local_1c - FI(iVar19)) *
                                                 (FI(iVar16) - FI(iVar14)) -
                                                 (local_18 - FI(iVar14)) *
                                                 (FI(iVar15) - FI(iVar19));
                                        if (BITS(fVar9) < 0x80000001u) {
                                            if (BITS(fVar10) < 0x80000001u) goto LAB_00488405;
                                        }
                                        else if (0x80000000u < BITS(fVar10)) {
LAB_00488405:
                                            fVar10 = (local_1c - FI(iVar15)) *
                                                     (FI(local_10) - FI(iVar16)) -
                                                     (local_18 - FI(iVar16)) *
                                                     (FI(iVar13) - FI(iVar15));
                                            goto joined_r0x0048843f;
                                        }
                                    }
                                }
                            }
                        }
                        else if ((0xc21fffffu < BITS(fVar9)) && (0 < (int)BITS(fVar10))) goto LAB_004881b0;
                    }
                }
            }
            local_4 = (int)((unsigned int)local_4 + 1u);
            local_14 = local_14 + 0x10;
            local_8 = local_8 + 3;
        } while (local_4 < *(int *)((unsigned int)param_1 + 0x8cu));
    }
#undef FL
#undef FI
#undef BITS
    return 0;
}

/* SPDX-License-Identifier: MIT */
/* z_bop 004210a0 */
void __cdecl FUN_004210a0(float *param_1, float *param_2)
{
    float fVar1 = param_2[2];
    float fVar2 = param_2[5];
    float fVar3 = param_1[1];
    float fVar4 = param_2[8];
    float fVar5 = *param_2;
    float fVar6 = param_2[6];
    float fVar7 = param_2[3];
    float fVar8 = param_1[5];
    float fVar9 = param_2[7];
    float fVar10 = param_2[1];
    float fVar11 = param_1[3];
    float fVar12 = param_2[4];
    float fVar13 = param_2[8];
    float fVar14 = param_2[5];
    float fVar15 = param_2[2];
    float fVar16 = param_1[3];
    float fVar17 = *param_2;
    float fVar18 = param_2[6];
    float fVar19 = param_2[3];
    float fVar20 = param_2[1];
    float fVar21 = param_1[6];
    float fVar22 = param_2[7];
    float fVar23 = param_2[4];
    float fVar24 = param_2[5];
    float fVar25 = param_1[7];
    float fVar26 = param_1[6];
    float fVar27 = param_2[2];
    float fVar28 = param_2[8];
    float fVar29 = *param_2;
    float fVar30 = param_2[6];
    float fVar31 = param_1[2];
    float fVar32 = param_1[1];
    float fVar33 = param_2[3];

    param_1[1] = param_1[1] * param_2[4] + param_1[2] * param_2[7] + *param_1 * param_2[1];
    param_1[2] = param_1[2] * fVar4 + fVar2 * fVar3 + *param_1 * fVar1;
    param_1[3] = fVar7 * param_1[4] + param_1[5] * fVar6 + fVar5 * param_1[3];
    *param_1 = fVar30 * fVar31 + fVar29 * *param_1 + fVar32 * fVar33;
    param_1[5] = fVar15 * fVar16 + fVar14 * param_1[4] + param_1[5] * fVar13;
    param_1[4] = fVar12 * param_1[4] + fVar10 * fVar11 + fVar8 * fVar9;
    param_1[6] = param_1[7] * fVar19 + fVar18 * param_1[8] + fVar17 * param_1[6];
    param_1[7] = param_1[7] * fVar23 + fVar22 * param_1[8] + fVar20 * fVar21;
    param_1[8] = fVar28 * param_1[8] + fVar26 * fVar27 + fVar24 * fVar25;
}

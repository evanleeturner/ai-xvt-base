/* SPDX-License-Identifier: MIT */
/* z_bop 004a66d0 */
extern float _DAT_005181fc;

unsigned int __cdecl FUN_004a66d0(float *param_1, int param_2, int *param_3, unsigned int *param_4)
{
    int iVar1;
    float *pfVar2;
    float fVar3;
    float fVar4;
    float fVar5;
    float fVar6;
    int bVar7;
    int iVar8;
    int iVar9;
    int iVar10;
    float local_14;
    float local_10;
    float local_c;
    float local_8;
    float local_4;

    local_4 = *param_1;
    local_14 = param_1[1];
    local_10 = param_1[2];

    if (0x80000000u < *(unsigned int *)&local_4) {
        local_4 = -local_4;
    }
    if (0x80000000u < *(unsigned int *)&local_14) {
        local_14 = -local_14;
    }
    if (0x80000000u < *(unsigned int *)&local_10) {
        local_10 = -local_10;
    }

    if ((local_10 < local_14) || (local_10 < local_4)) {
        if ((local_14 < local_4) || (local_14 < local_10)) {
            iVar9 = 1;
            iVar10 = 2;
        } else {
            iVar9 = 0;
            iVar10 = 2;
            param_4[1] = *param_4;
        }
    } else {
        iVar9 = 0;
        iVar10 = 1;
        param_4[2] = param_4[1];
        param_4[1] = *param_4;
    }

    iVar1 = (int)((unsigned int)iVar9 + (unsigned int)*param_3 * 3u);
    fVar3 = *(float *)((unsigned int)param_2 + (unsigned int)iVar1 * 4u);
    pfVar2 = (float *)((unsigned int)param_2 +
                       ((unsigned int)*param_3 * 3u + (unsigned int)iVar10) * 4u);
    fVar4 = *pfVar2;
    fVar5 = *(float *)((unsigned int)param_2 +
                       ((unsigned int)iVar9 + (unsigned int)param_3[1] * 3u) * 4u);
    fVar6 = *(float *)((unsigned int)param_2 +
                       ((unsigned int)param_3[1] * 3u + (unsigned int)iVar10) * 4u);
    bVar7 = ((fVar6 - fVar4) * (*(float *)&param_4[1] - fVar3) -
             (fVar5 - fVar3) * (*(float *)&param_4[2] - fVar4)) < _DAT_005181fc;

    fVar3 = *(float *)((unsigned int)param_2 +
                       ((unsigned int)iVar9 + (unsigned int)param_3[2] * 3u) * 4u);
    fVar4 = *(float *)((unsigned int)param_2 +
                       ((unsigned int)param_3[2] * 3u + (unsigned int)iVar10) * 4u);
    fVar5 = (fVar4 - fVar6) * (*(float *)&param_4[1] - fVar5) -
            (fVar3 - fVar5) * (*(float *)&param_4[2] - fVar6);

    if ((fVar5 < _DAT_005181fc) && (!bVar7)) {
        return 0;
    }
    if ((*(unsigned int *)&fVar5 < 0x80000001u) && (bVar7)) {
        return 0;
    }

    local_c = fVar4;
    local_8 = fVar3;
    if (param_3[3] != -1) {
        iVar8 = (int)((unsigned int)param_3[3] * 3u);
        local_8 = *(float *)((unsigned int)param_2 +
                             ((unsigned int)iVar9 + (unsigned int)iVar8) * 4u);
        local_c = *(float *)((unsigned int)param_2 +
                             ((unsigned int)iVar10 + (unsigned int)iVar8) * 4u);
        fVar3 = (local_c - fVar4) * (*(float *)&param_4[1] - fVar3) -
                (local_8 - fVar3) * (*(float *)&param_4[2] - fVar4);
        if ((fVar3 < _DAT_005181fc) && (!bVar7)) {
            return 0;
        }
        if ((*(unsigned int *)&fVar3 < 0x80000001u) && (bVar7)) {
            return 0;
        }
    }

    fVar3 = (*(float *)&param_4[1] - local_8) * (*pfVar2 - local_c) -
            (*(float *)((unsigned int)param_2 + (unsigned int)iVar1 * 4u) - local_8) *
            (*(float *)&param_4[2] - local_c);
    if ((fVar3 < _DAT_005181fc) && (!bVar7)) {
        return 0;
    }
    if ((*(unsigned int *)&fVar3 < 0x80000001u) && (bVar7)) {
        return 0;
    }
    return 1;
}

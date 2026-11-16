/* SPDX-License-Identifier: MIT */
/* z_xvt 0045ed70 */
extern unsigned int DAT_00732f3c;
extern unsigned short DAT_007091ae;
extern unsigned short DAT_006d4984;
extern int DAT_00709ffc;
extern float _DAT_006a8a08;
extern float _DAT_00520194;
extern int DAT_005f10dc;
extern int DAT_005f10b8;
extern int DAT_005f102c[];

void __cdecl FUN_0045ed70(int param_1, int param_2, int param_3)
{
    float *pfVar1;
    float *pfVar2;
    float *pfVar3;
    float fVar4, fVar5, fVar6, fVar7, fVar8, fVar9, fVar10, fVar11, fVar12;
    int iVar13;
    unsigned int uVar9;
    unsigned int uVar10;

    iVar13 = DAT_005f10dc;
    fVar9 = *(float *)(int)((unsigned int)param_3 + 8u + (unsigned int)param_1 * 0x18u);
    pfVar1 = (float *)(int)((unsigned int)param_3 + (unsigned int)param_1 * 0x18u);
    fVar10 = *(float *)(int)((unsigned int)param_3 + 8u + (unsigned int)param_2 * 0x18u);
    pfVar2 = (float *)(int)((unsigned int)param_3 + (unsigned int)param_2 * 0x18u);
    __builtin_memcpy(&uVar9, &fVar9, 4);
    __builtin_memcpy(&uVar10, &fVar10, 4);
    if (uVar9 < 0x80000001u) {
        if (0x80000000u < uVar10) {
            fVar4 = *pfVar2;
            fVar9 = (float)DAT_00732f3c / fVar9;
            fVar5 = pfVar2[1];
            fVar12 = (*pfVar1 - (float)(DAT_007091ae >> 1)) * fVar9 * _DAT_006a8a08;
            fVar6 = pfVar2[4];
            fVar7 = pfVar2[5];
            fVar11 = (pfVar1[1] - (float)(int)((unsigned int)(DAT_006d4984 >> 1) + (unsigned int)DAT_00709ffc)) * fVar9 * _DAT_006a8a08;
            fVar8 = pfVar1[4];
            fVar10 = fVar10 / ((fVar10 - fVar9) + _DAT_00520194);
            pfVar3 = (float *)(int)((unsigned int)param_3 + (unsigned int)DAT_005f10dc * 0x18u);
            fVar9 = pfVar1[5];
            DAT_005f10dc = (int)((unsigned int)DAT_005f10dc + 1u);
            pfVar3[3] = pfVar2[3] - (pfVar2[3] - pfVar1[3]) * fVar10;
            *pfVar3 = fVar4 - (fVar4 - fVar12) * fVar10;
            pfVar3[1] = fVar5 - (fVar5 - fVar11) * fVar10;
            pfVar3[4] = fVar6 - (fVar6 - fVar8) * fVar10;
            pfVar3[5] = fVar7 - fVar10 * (fVar7 - fVar9);
            fVar9 = (float)DAT_00732f3c;
            pfVar3[2] = fVar9;
            *pfVar3 = *pfVar3 * fVar9;
            pfVar3[1] = fVar9 * pfVar3[1];
            *pfVar3 = (float)(DAT_007091ae >> 1) + *pfVar3;
            pfVar3[1] = (float)(int)((unsigned int)(DAT_006d4984 >> 1) + (unsigned int)DAT_00709ffc) + pfVar3[1];
            DAT_005f10b8 = (int)((unsigned int)DAT_005f10b8 + 1u);
            DAT_005f102c[DAT_005f10b8] = iVar13;
            return;
        }
        DAT_005f10b8 = (int)((unsigned int)DAT_005f10b8 + 1u);
        DAT_005f102c[DAT_005f10b8] = param_2;
    }
    else if (uVar10 < 0x80000001u) {
        fVar4 = *pfVar1;
        fVar5 = pfVar1[1];
        fVar6 = pfVar1[4];
        fVar7 = pfVar1[5];
        fVar10 = (float)DAT_00732f3c / fVar10;
        fVar12 = (*pfVar2 - (float)(DAT_007091ae >> 1)) * fVar10 * _DAT_006a8a08;
        fVar11 = (pfVar2[1] - (float)(int)((unsigned int)(DAT_006d4984 >> 1) + (unsigned int)DAT_00709ffc)) * fVar10 * _DAT_006a8a08;
        fVar8 = pfVar2[4];
        fVar9 = fVar9 / ((fVar10 - fVar9) - _DAT_00520194);
        pfVar3 = (float *)(int)((unsigned int)param_3 + (unsigned int)DAT_005f10dc * 0x18u);
        fVar10 = pfVar2[5];
        DAT_005f10dc = (int)((unsigned int)DAT_005f10dc + 1u);
        pfVar3[3] = pfVar1[3] - (pfVar2[3] - pfVar1[3]) * fVar9;
        *pfVar3 = fVar4 - (fVar12 - fVar4) * fVar9;
        pfVar3[1] = fVar5 - (fVar11 - fVar5) * fVar9;
        pfVar3[4] = fVar6 - (fVar8 - fVar6) * fVar9;
        pfVar3[5] = fVar7 - fVar9 * (fVar10 - fVar7);
        fVar9 = (float)DAT_00732f3c;
        pfVar3[2] = fVar9;
        *pfVar3 = *pfVar3 * fVar9;
        pfVar3[1] = fVar9 * pfVar3[1];
        *pfVar3 = (float)(DAT_007091ae >> 1) + *pfVar3;
        pfVar3[1] = (float)(int)((unsigned int)(DAT_006d4984 >> 1) + (unsigned int)DAT_00709ffc) + pfVar3[1];
        (&DAT_005f102c[1])[DAT_005f10b8] = iVar13;
        DAT_005f10b8 = (int)((unsigned int)DAT_005f10b8 + 2u);
        DAT_005f102c[DAT_005f10b8] = param_2;
        return;
    }
    return;
}

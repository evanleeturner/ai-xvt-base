/* SPDX-License-Identifier: MIT */
/* z_bop 004af850 */
extern unsigned char DAT_00521f19[];
extern short DAT_009a7ed0[];
extern void *FUN_004ac720(unsigned int);
extern void FUN_00471ff0(int *);
extern int FUN_004adcc0(int);
extern void FUN_004ac740(void);

int __cdecl FUN_004af850(int param_1, int param_2, int param_3, int param_4, int param_5, int param_6)
{
    float fVar1;
    float fVar2;
    int iVar3;
    int *piVar4;
    int iVar5;
    int iVar6;
    int iVar7;
    float *pfVar8;
    int iVar9;
    int iVar10;
    float local_810;
    int local_800[256];
    float afStack_400[256];

    if ((DAT_00521f19[param_1 * 0x1a] & 1) == 0) {
        return 0;
    }

    piVar4 = (int *)FUN_004ac720(((unsigned int)(short)((unsigned int)(param_1 * 5) >> 16) << 16) | (unsigned int)(short)DAT_009a7ed0[param_1]);

    if ((int *)*piVar4 != piVar4) {
        FUN_00471ff0(piVar4);
    }

    if (*(int *)(**(int **)((int)piVar4 + 10) + 4) == 0x14) {
        param_5 = param_5 + 1;
    }

    if (*(int *)((int)piVar4 + 6) <= param_5) {
        param_5 = *(int *)((int)piVar4 + 6) + -1;
    }

    iVar5 = FUN_004adcc0((*(int **)((int)piVar4 + 10))[param_5]);
    pfVar8 = *(float **)(iVar5 + 0x14);
    iVar5 = *(int *)(iVar5 + 0x10);

    if (2 < iVar5) {
        iVar5 = iVar5 + -2;
    }
    if (0x100 < iVar5) {
        iVar5 = 0x100;
    }
    if (iVar5 <= param_6) {
        param_6 = iVar5 + -1;
    }

    iVar6 = 0;
    if (0 < iVar5) {
        iVar9 = 0;
        while (1) {
            fVar1 = pfVar8[1];
            fVar2 = pfVar8[2];
            afStack_400[iVar9 / 4] = (*pfVar8 - (float)param_2) * (*pfVar8 - (float)param_2);
            local_800[iVar9 / 4] = iVar6;
            pfVar8 = pfVar8 + 3;
            afStack_400[iVar9 / 4] = (fVar1 - (float)param_3) * (fVar1 - (float)param_3) + afStack_400[iVar9 / 4];
            fVar1 = (fVar2 - (float)param_4) * (fVar2 - (float)param_4) + afStack_400[iVar9 / 4];
            iVar6 = iVar6 + 1;
            if (iVar5 <= iVar6) break;
            afStack_400[iVar9 / 4] = fVar1;
            iVar9 = iVar9 + 4;
        }
        afStack_400[iVar9 / 4] = fVar1;
    }

    iVar7 = 0;
    iVar9 = 0;
    local_810 = 4.611686e+18F;
    iVar6 = local_800[0];
    iVar10 = iVar9;
    if (0 < param_6 + 1) {
        do {
            for (; iVar9 < iVar5; iVar9 = iVar9 + 1) {
                if (afStack_400[iVar9] < local_810) {
                    local_810 = afStack_400[iVar9];
                    iVar6 = iVar9;
                }
            }
            afStack_400[iVar6] = afStack_400[iVar7 / 4];
            iVar3 = local_800[iVar6];
            iVar9 = iVar10 + 1;
            afStack_400[iVar7 / 4] = local_810;
            local_800[iVar6] = local_800[iVar7 / 4];
            local_800[iVar7 / 4] = iVar3;
            iVar10 = iVar9;
            iVar7 = iVar7 + 4;
        } while (iVar9 < param_6 + 1);
    }

    FUN_004ac740();
    return local_800[param_6];
}

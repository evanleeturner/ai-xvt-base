/* SPDX-License-Identifier: MIT */
/* z_xvt 0051e850 */
void __cdecl FUN_0051e850(int param_1, int param_2, unsigned int *param_3, unsigned int param_4,
                         int param_5, int param_6, int param_7, int param_8)
{
    unsigned char uVar1;
    unsigned short uVar2;
    unsigned int uVar3;
    int iVar4;
    int iVar5;
    int iVar6;
    int iVar7;
    unsigned int uVar8;
    unsigned int uVar9;
    int iVar10;
    unsigned int *puVar11;

    uVar9 = (unsigned int)(-param_7 & 7);
    if ((int)param_4 < (int)uVar9) {
        uVar9 = param_4;
    }
    iVar4 = (int)(param_4 - uVar9) >> 3;
    uVar8 = (param_4 + iVar4 * -8) - uVar9;
    iVar5 = param_6 * 2 - param_4;
    iVar6 = param_2 * 2 - param_4;
    puVar11 = (unsigned int *)(param_1 + param_8 * param_2 + param_7);
    iVar7 = param_5 >> 1;

    if ((uVar9 == 0) && (uVar8 == 0)) {
        if (iVar7 != 0) {
            do {
                iVar7 = iVar7 - 1;
                for (iVar10 = iVar4; iVar10 != 0; iVar10 = iVar10 - 1) {
                    puVar11[1] = param_3[1];
                    *puVar11 = *param_3;
                    param_3 = param_3 + 2;
                    puVar11 = puVar11 + 2;
                }
                param_3 = (unsigned int *)((int)param_3 + iVar5);
                puVar11 = (unsigned int *)((int)puVar11 + iVar6);
            } while (iVar7 != 0);
            return;
        }
    }
    else if (iVar7 != 0) {
        do {
            iVar7 = iVar7 - 1;
            if ((uVar9 & 1) != 0) {
                uVar1 = *(unsigned char *)param_3;
                param_3 = (unsigned int *)((int)param_3 + 1);
                *(unsigned char *)puVar11 = uVar1;
                puVar11 = (unsigned int *)((int)puVar11 + 1);
            }
            if ((uVar9 & 2) != 0) {
                uVar2 = *(unsigned short *)param_3;
                param_3 = (unsigned int *)((int)param_3 + 2);
                *(unsigned short *)puVar11 = uVar2;
                puVar11 = (unsigned int *)((int)puVar11 + 2);
            }
            iVar10 = iVar4;
            if ((uVar9 & 4) != 0) {
                uVar3 = *param_3;
                param_3 = param_3 + 1;
                *puVar11 = uVar3;
                puVar11 = puVar11 + 1;
            }
            for (; iVar10 != 0; iVar10 = iVar10 - 1) {
                puVar11[1] = param_3[1];
                *puVar11 = *param_3;
                param_3 = param_3 + 2;
                puVar11 = puVar11 + 2;
            }
            if ((uVar8 & 4) != 0) {
                uVar3 = *param_3;
                param_3 = param_3 + 1;
                *puVar11 = uVar3;
                puVar11 = puVar11 + 1;
            }
            if ((uVar8 & 2) != 0) {
                uVar2 = *(unsigned short *)param_3;
                param_3 = (unsigned int *)((int)param_3 + 2);
                *(unsigned short *)puVar11 = uVar2;
                puVar11 = (unsigned int *)((int)puVar11 + 2);
            }
            if ((uVar8 & 1) != 0) {
                uVar1 = *(unsigned char *)param_3;
                param_3 = (unsigned int *)((int)param_3 + 1);
                *(unsigned char *)puVar11 = uVar1;
                puVar11 = (unsigned int *)((int)puVar11 + 1);
            }
            param_3 = (unsigned int *)((int)param_3 + iVar5);
            puVar11 = (unsigned int *)((int)puVar11 + iVar6);
        } while (iVar7 != 0);
    }
    return;
}

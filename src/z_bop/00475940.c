/* SPDX-License-Identifier: MIT */
/* z_bop 00475940 */
extern int DAT_005272b0;
extern int DAT_0060f204;

unsigned int * __cdecl FUN_00479de0(int param_1, char *param_2);
void __cdecl FUN_00475940(int param_1, unsigned int *param_2, int param_3, int param_4);

void __cdecl FUN_00475940(int param_1, unsigned int *param_2, int param_3, int param_4)
{
    float *pfVar1;
    float *pfVar2;
    int iVar3;
    int iVar4;
    int bVar5;
    int local_4;

    if (param_2 == (unsigned int *)0) {
        return;
    }
    while (param_2[1] == 7) {
        param_2 = FUN_00479de0(param_3, (char *)param_2[5]);
        if (param_2 == (unsigned int *)0) {
            return;
        }
    }
    switch (param_2[1]) {
    case 1:
    case 0xf:
    case 0x10:
    case 0x11:
        if (*(int *)(param_4 + 0x84) == 0) {
            if (DAT_005272b0 == 0) {
                iVar3 = (int)(param_2[4] * 100u);
            } else {
                iVar3 = (int)(param_2[4] * 0x54u);
            }
            local_4 = 0;
            pfVar1 = (float *)(param_2[5] + (unsigned int)iVar3 + 4u);
            if (0 < DAT_0060f204) {
                do {
                    pfVar2 = *(float **)(param_1 + 0x14);
                    iVar4 = 0;
                    iVar3 = *(int *)(param_1 + 0x10);
                    bVar5 = (iVar3 == 0);
                    if (0 < iVar3) {
                        do {
                            if (((*pfVar1 == *pfVar2) && (pfVar2[1] == pfVar1[1])) && (pfVar2[2] == pfVar1[2]))
                                break;
                            pfVar2 = pfVar2 + 3;
                            iVar4 = iVar4 + 1;
                        } while (iVar4 < iVar3);
                        bVar5 = (iVar4 == iVar3);
                    }
                    if (bVar5) {
                        *pfVar2 = *pfVar1;
                        pfVar2[1] = pfVar1[1];
                        pfVar2[2] = pfVar1[2];
                        *(int *)(param_1 + 0x10) = *(int *)(param_1 + 0x10) + 1;
                    }
                    pfVar1 = pfVar1 + 3;
                    local_4 = local_4 + 1;
                } while (local_4 < DAT_0060f204);
            }
            goto LAB_00475af9;
        }
        break;
    case 3:
        DAT_0060f204 = (int)param_2[4];
        break;
    case 0xb:
        local_4 = 0;
        *(unsigned int *)(param_4 + 0x84) = param_2[5];
        pfVar1 = (float *)param_2[5];
        if (0 < (int)param_2[4]) {
            do {
                pfVar2 = *(float **)(param_1 + 0x14);
                iVar4 = 0;
                iVar3 = *(int *)(param_1 + 0x10);
                bVar5 = (iVar3 == 0);
                if (0 < iVar3) {
                    do {
                        if (((*pfVar1 == *pfVar2) && (pfVar2[1] == pfVar1[1])) && (pfVar2[2] == pfVar1[2]))
                            break;
                        pfVar2 = pfVar2 + 3;
                        iVar4 = iVar4 + 1;
                    } while (iVar4 < iVar3);
                    bVar5 = (iVar4 == iVar3);
                }
                if (bVar5) {
                    *pfVar2 = *pfVar1;
                    pfVar2[1] = pfVar1[1];
                    pfVar2[2] = pfVar1[2];
                    *(int *)(param_1 + 0x10) = *(int *)(param_1 + 0x10) + 1;
                }
                pfVar1 = pfVar1 + 3;
                local_4 = local_4 + 1;
            } while (local_4 < (int)param_2[4]);
        }
        goto LAB_00475af9;
    }
LAB_00475af9:
    if (DAT_005272b0 != 0) {
        *(unsigned int *)(param_4 + 0x84) = 0;
    }
    iVar4 = 0;
    iVar3 = 0;
    if (0 < (int)param_2[2]) {
        do {
            iVar4 = iVar4 + 4;
            iVar3 = iVar3 + 1;
            FUN_00475940(param_1, *(unsigned int **)(param_2[3] + (unsigned int)iVar4 - 4u), param_3, param_4);
        } while (iVar3 < (int)param_2[2]);
    }
    return;
}

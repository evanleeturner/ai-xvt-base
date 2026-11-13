/* SPDX-License-Identifier: MIT */
/* z_bop 00475850 */
unsigned int * __cdecl FUN_00479de0(int param_1, char *param_2);
void __cdecl FUN_00475850(int param_1, unsigned int *param_2, int param_3, unsigned int param_4);

void __cdecl FUN_00475850(int param_1, unsigned int *param_2, int param_3, unsigned int param_4)
{
    if (param_2 != 0) {
        while (param_2[1] == 7) {
            param_2 = FUN_00479de0(param_3, (char *)param_2[5]);
            if (param_2 == 0) {
                return;
            }
        }
        if (param_2[1] == 0xd) {
            float *pfVar1 = (float *)param_2[5];
            int local_4 = 0;
            if (0 < (int)param_2[4]) {
                do {
                    float *pfVar2 = *(float **)(param_1 + 0x14);
                    int iVar3 = 0;
                    int iVar4 = *(int *)(param_1 + 0x10);
                    int bVar5 = (iVar4 == 0);
                    if (0 < iVar4) {
                        do {
                            if ((*pfVar2 == *pfVar1) && (pfVar2[1] == pfVar1[1])) {
                                break;
                            }
                            pfVar2 += 2;
                            iVar3 += 1;
                        } while (iVar3 < iVar4);
                        bVar5 = (iVar4 == iVar3);
                    }
                    if (bVar5) {
                        *pfVar2 = *pfVar1;
                        pfVar2[1] = pfVar1[1];
                        *(int *)(param_1 + 0x10) = *(int *)(param_1 + 0x10) + 1;
                    }
                    pfVar1 += 2;
                    local_4 += 1;
                } while (local_4 < (int)param_2[4]);
            }
        }
        int iVar4 = 0;
        if (0 < (int)param_2[2]) {
            int iVar3 = 0;
            do {
                iVar3 += 4;
                iVar4 += 1;
                FUN_00475850(param_1, *(unsigned int **)(param_2[3] - 4 + iVar3), param_3, param_4);
            } while (iVar4 < (int)param_2[2]);
        }
    }
    return;
}

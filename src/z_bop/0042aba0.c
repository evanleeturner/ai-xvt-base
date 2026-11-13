/* SPDX-License-Identifier: MIT */
/* z_bop 0042aba0 */
extern undefined4 * __cdecl FUN_00479de0(int param_1, char *param_2);
extern void __cdecl FUN_0042aba0(undefined4 *param_1, int param_2, float *param_3);

void __cdecl FUN_0042aba0(undefined4 *param_1, int param_2, float *param_3)
{
    if (param_1 != NULL) {
        while (param_1[1] == 7) {
            param_1 = FUN_00479de0(param_2, (char *)param_1[5]);
            if (param_1 == NULL) {
                return;
            }
        }
        if (param_1[1] == 3) {
            int iVar2 = param_1[4];
            float *pfVar3 = (float *)param_1[5];
            if (iVar2 > 0) {
                while (iVar2 > 0) {
                    *pfVar3 += *param_3;
                    pfVar3[1] += param_3[1];
                    pfVar3[2] += param_3[2];
                    pfVar3 += 3;
                    iVar2--;
                }
            }
        }
        int iVar2 = 0;
        int iVar4 = 0;
        if ((int)param_1[2] > 0) {
            do {
                undefined4 *puVar1 = (undefined4 *)(param_1[3] + iVar2);
                iVar2 += 4;
                iVar4++;
                FUN_0042aba0((undefined4 *)*puVar1, param_2, param_3);
            } while (iVar4 < (int)param_1[2]);
        }
    }
    return;
}

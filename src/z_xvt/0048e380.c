/* SPDX-License-Identifier: MIT */
/* z_xvt 0048e380 */
extern int DAT_006026bc;
extern float DAT_0052a350[];
extern float _DAT_005201dc;

void __cdecl FUN_0048e380(float *param_1, int param_2, int param_3)
{
    int iVar3 = 0;
    if (0 < DAT_006026bc) {
        do {
            int iVar5 = 0;
            int *piVar4 = (int *)(param_2 + 4);
            float *pfVar2 = (float *)(param_2 + 4 + param_3 * 0x40);
            int iVar1 = param_3;
            if (0 < param_3) {
                do {
                    if ((((*piVar4 == iVar3) || (piVar4[1] == iVar3)) || (piVar4[2] == iVar3)) || (piVar4[3] == iVar3)) {
                        iVar5 = iVar5 + 1;
                        if (iVar5 == 1) {
                            *param_1 = *pfVar2;
                            param_1[1] = pfVar2[1];
                            param_1[2] = pfVar2[2];
                        }
                        else {
                            *param_1 = *param_1 + *pfVar2;
                            param_1[1] = pfVar2[1] + param_1[1];
                            param_1[2] = pfVar2[2] + param_1[2];
                        }
                    }
                    piVar4 = piVar4 + 0x10;
                    pfVar2 = pfVar2 + 3;
                    iVar1 = iVar1 + -1;
                } while (iVar1 != 0);
            }
            if (1 < iVar5) {
                float local_8;
                if (iVar5 < 0x41) {
                    local_8 = DAT_0052a350[iVar5];
                }
                else {
                    local_8 = _DAT_005201dc / (float)iVar5;
                }
                *param_1 = *param_1 * local_8;
                param_1[1] = param_1[1] * local_8;
                param_1[2] = param_1[2] * local_8;
            }
            param_1 = param_1 + 3;
            iVar3 = iVar3 + 1;
        } while (iVar3 < DAT_006026bc);
    }
    return;
}

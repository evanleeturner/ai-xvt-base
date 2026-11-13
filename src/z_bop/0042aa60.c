/* SPDX-License-Identifier: MIT */
/* z_bop 0042aa60 */
typedef unsigned int undefined4;

undefined4 * __cdecl FUN_00479de0(int param_1, char * param_2);

void __cdecl FUN_0042aa60(undefined4 *param_1, int param_2, undefined4 param_3, undefined4 param_4)
{
    float fVar1;
    float *pfVar2;
    int iVar3;
    int iVar4;

    if (param_1 != (undefined4 *)0x0) {
        while (param_1[1] == 7) {
            param_1 = FUN_00479de0(param_2, (char *)param_1[5]);
            if (param_1 == (undefined4 *)0x0) {
                return;
            }
        }
        iVar3 = param_1[1];
        if (iVar3 == 1) {
            iVar3 = param_1[4];
            if (0 < iVar3) {
                union {
                    undefined4 parts[2];
                    double d;
                } u;
                u.parts[0] = param_3;
                u.parts[1] = param_4;
                fVar1 = (float)(double)u.d;
                pfVar2 = (float *)(param_1[5] + 4 + iVar3 * 0x4c);
                while (1) {
                    iVar3 = iVar3 - 1;
                    *pfVar2 = *pfVar2 / fVar1;
                    pfVar2[1] = pfVar2[1] / fVar1;
                    pfVar2[2] = pfVar2[2] / fVar1;
                    pfVar2[3] = pfVar2[3] / fVar1;
                    pfVar2[4] = pfVar2[4] / fVar1;
                    if (iVar3 == 0) break;
                    pfVar2[5] = pfVar2[5] / fVar1;
                    pfVar2 = pfVar2 + 6;
                }
                pfVar2[5] = pfVar2[5] / fVar1;
            }
        }
        else if (iVar3 == 3) {
            iVar3 = param_1[4];
            if (0 < iVar3) {
                union {
                    undefined4 parts[2];
                    double d;
                } u;
                u.parts[0] = param_3;
                u.parts[1] = param_4;
                fVar1 = (float)(double)u.d;
                pfVar2 = (float *)param_1[5];
                while (1) {
                    iVar3 = iVar3 - 1;
                    *pfVar2 = *pfVar2 / fVar1;
                    pfVar2[1] = pfVar2[1] / fVar1;
                    if (iVar3 == 0) break;
                    pfVar2[2] = pfVar2[2] / fVar1;
                    pfVar2 = pfVar2 + 3;
                }
                pfVar2[2] = pfVar2[2] / fVar1;
            }
        }
        else if (iVar3 == 0x15) {
            iVar3 = param_1[2];
            if (0 < iVar3) {
                pfVar2 = (float *)param_1[5];
                while (1) {
                    union {
                        undefined4 parts[2];
                        double d;
                    } u;
                    u.parts[0] = param_3;
                    u.parts[1] = param_4;
                    fVar1 = *pfVar2 * (float)(double)u.d;
                    iVar3 = iVar3 - 1;
                    if (iVar3 == 0) break;
                    *pfVar2 = fVar1;
                    pfVar2 = pfVar2 + 1;
                }
                *pfVar2 = fVar1;
            }
        }
        iVar4 = 0;
        iVar3 = 0;
        if (0 < (int)param_1[2]) {
            do {
                iVar4 = iVar4 + 4;
                iVar3 = iVar3 + 1;
                FUN_0042aa60(*(undefined4 **)(param_1[3] + -4 + iVar4), param_2, param_3, param_4);
            } while (iVar3 < (int)param_1[2]);
        }
    }
    return;
}

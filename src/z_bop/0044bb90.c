/* SPDX-License-Identifier: MIT */
/* z_bop 0044bb90 */
int __cdecl FUN_0044bb90(int *param_1, int *param_2, int *param_3, unsigned int *param_4)
{
    int iVar1;
    unsigned int uVar2;
    int *piVar3;
    int *piVar4;
    
    if (param_2 != (int *)0x0) {
        *param_2 = 0;
    }
    if (param_3 != (int *)0x0) {
        *param_3 = 0;
    }
    if (param_4 != (unsigned int *)0x0) {
        *param_4 = 0;
    }
    iVar1 = param_1[1];
    piVar4 = param_1 + 3;
    if ((*param_1 == 0x46464952) && (param_1[2] == 0x45564157)) {
        while (piVar4 < (int *)((int)param_1 + iVar1 + 8)) {
            uVar2 = piVar4[1];
            piVar3 = piVar4 + 2;
            if (*piVar4 == 0x20746d66) {
                if ((param_2 != (int *)0x0) && (*param_2 == 0)) {
                    if (uVar2 < 0xe) {
                        return 0;
                    }
                    *param_2 = (int)piVar3;
                    if (((param_3 == (int *)0x0) || (*param_3 != 0)) &&
                       ((param_4 == (unsigned int *)0x0 || (*param_4 != 0)))) {
                        return 1;
                    }
                }
            }
            else if ((*piVar4 == 0x61746164) &&
                    (((param_3 != (int *)0x0 && (*param_3 == 0)) ||
                     ((param_4 != (unsigned int *)0x0 && (*param_4 == 0)))))) {
                if (param_3 != (int *)0x0) {
                    *param_3 = (int)piVar3;
                }
                if (param_4 != (unsigned int *)0x0) {
                    *param_4 = uVar2;
                }
                if ((param_2 == (int *)0x0) || (*param_2 != 0)) {
                    return 1;
                }
            }
            piVar4 = (int *)((int)piVar3 + (uVar2 + 1 & 0xfffffffe));
        }
    }
    return 0;
}

/* SPDX-License-Identifier: MIT */
/* z_xvt 00431d40 */
void * __cdecl FUN_00431d70(int *param_1, undefined4 *param_2);
void __cdecl FUN_00430980(int param_1, int param_2);

void __cdecl FUN_00431d40(int *param_1, undefined4 *param_2)
{
    int *piVar1;
    
    while (1) {
        piVar1 = (int *)FUN_00431d70(param_1, param_2);
        if (piVar1 == 0 || *param_1 == 0) {
            break;
        }
        FUN_00430980(*piVar1, (int)piVar1);
    }
    return;
}

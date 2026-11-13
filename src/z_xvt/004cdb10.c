/* SPDX-License-Identifier: MIT */
/* z_xvt 004cdb10 */
DWORD __stdcall GetTickCount(void);
int * __cdecl FUN_004cc9a0(int *param_1, undefined4 *param_2);

int * __cdecl FUN_004cdb10(int *param_1, undefined4 *param_2, int param_3)
{
    DWORD DVar1;
    DWORD DVar2;
    int *piVar3;
    int local_8;
    undefined4 local_4;

    DVar1 = GetTickCount();
    do {
        DVar2 = GetTickCount();
        if ((unsigned int)(param_3 * 1000) < DVar2 - DVar1) {
            return (int *)0x0;
        }
        piVar3 = FUN_004cc9a0(&local_8, &local_4);
    } while (piVar3 == (int *)0x0);
    *param_1 = local_8;
    *param_2 = local_4;
    return piVar3;
}

/* SPDX-License-Identifier: MIT */
/* z_xvt 00485fa0 */
void __cdecl FUN_00486000(int *param_1, int param_2);

void __cdecl FUN_00485fa0(int *param_1)
{
    int iVar3;
    int iVar4;
    int iVar5;
    int *piVar2;
    int iVar1;

    iVar3 = 0;
    iVar5 = (int)param_1 - *param_1;
    *param_1 = *param_1 + iVar5;
    if (*(int *)((int)param_1 + 10) != 0) {
        iVar4 = 0;
        *(int *)((int)param_1 + 10) = *(int *)((int)param_1 + 10) + iVar5;
        if (0 < *(int *)((int)param_1 + 6)) {
            do {
                piVar2 = (int *)(*(int *)((int)param_1 + 10) + iVar3);
                iVar1 = *piVar2;
                if (iVar1 != 0) {
                    *piVar2 = iVar1 + iVar5;
                    FUN_00486000(piVar2, iVar5);
                }
                iVar3 = iVar3 + 4;
                iVar4 = iVar4 + 1;
            } while (iVar4 < *(int *)((int)param_1 + 6));
        }
    }
    return;
}

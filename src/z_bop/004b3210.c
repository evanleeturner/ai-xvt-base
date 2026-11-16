/* SPDX-License-Identifier: MIT */
/* z_bop 004b3210 */
extern int *DAT_00528cb4;
extern int DAT_00518ee0;

int __stdcall FUN_004b3210(void)
{
    int *piVar2;
    int *local_10;
    int local_8;
    int local_4;
    int iVar1;
    int **vtable;

    piVar2 = DAT_00528cb4;
    local_8 = 0;
    local_10 = &DAT_00518ee0;
    vtable = (int **) *piVar2;
    iVar1 = ((int (__stdcall *)(void))vtable[0])();
    if (iVar1 != 0) {
        return 0;
    }
    local_10 = (int *)0x1000;
    local_4 = 0;
    iVar1 = ((int (__stdcall *)(int *, int **, int, int))vtable[0x17])(piVar2, &local_10, local_8, local_4);
    if (iVar1 != 0) {
        return 0;
    }
    ((void (__stdcall *)(int *))vtable[2])(piVar2);
    return 1;
}

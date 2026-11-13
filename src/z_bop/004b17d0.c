/* SPDX-License-Identifier: MIT */
/* z_bop 004b17d0 */
void __cdecl _free(void *);

void __cdecl FUN_004b17d0(int *param_1)
{
    int iVar1;
    int *piVar2;

    if (*param_1 == 1) {
        piVar2 = (int *)param_1[0x19];
        if (piVar2 != (int *)0x0) {
            (*(void (__stdcall **)(int *))(*piVar2 + 8))(piVar2);
        }
    }
    else {
        _free((void *)param_1[0x17]);
    }
    piVar2 = param_1;
    for (iVar1 = 0x36; iVar1 != 0; iVar1 = iVar1 + -1) {
        *piVar2 = 0;
        piVar2 = piVar2 + 1;
    }
    _free(param_1);
    return;
}

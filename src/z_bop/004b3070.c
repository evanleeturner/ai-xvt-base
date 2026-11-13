/* SPDX-License-Identifier: MIT */
/* z_bop 004b3070 */
extern int DAT_00528c8c;
extern int DAT_00528c90;
extern int _DAT_00528c88;
extern int DAT_00528c5c;
extern int DAT_00528c84;

void __stdcall FUN_004b3070(void)
{
    int *piVar1;
    int *piVar2;
    
    piVar2 = (int *)DAT_00528c8c;
    while (piVar2 != (int *)0x0) {
        piVar1 = (int *)piVar2[1];
        if (piVar1 != (int *)0x0) {
            (*(void (**)(int *))(*(int **)piVar1 + 8))(piVar1);
            piVar2[1] = 0;
        }
        piVar1 = (int *)*piVar2;
        if (piVar1 != (int *)0x0) {
            (*(void (**)(int *))(*(int **)piVar1 + 8))(piVar1);
            *piVar2 = 0;
        }
        piVar2[0x1e] = 0;
        piVar2[0x23] = 0;
        piVar1 = (int *)piVar2[0x25];
        piVar2[0x25] = 0;
        piVar2[0x24] = 0;
        piVar2 = piVar1;
    }
    DAT_00528c8c = 0;
    DAT_00528c90 = 0;
    _DAT_00528c88 = 0;
    *(int *)(DAT_00528c5c + 0x14c) = *(int *)(DAT_00528c5c + 0x148);
    DAT_00528c84 = 1;
    return;
}

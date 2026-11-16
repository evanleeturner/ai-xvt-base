/* SPDX-License-Identifier: MIT */
/* z_xvt 004dd810 */
extern int DAT_00758e8e;
extern int DAT_00758e92;
extern int *DAT_00758e86;
extern int *DAT_00758e8a;
extern int DAT_00758d5a;
extern int DAT_00758dbe;
extern int DAT_00532bd8;
extern int DAT_00758e9e;

void __stdcall FUN_004dd810(void)
{
    int iVar1;
    int *puVar2;
    int *puVar3;

    DAT_00758e8e = 10;
    DAT_00758e92 = 10;
    DAT_00758e86 = &DAT_00758d5a;
    DAT_00758e8a = &DAT_00758dbe;
    puVar2 = &DAT_00532bd8;
    puVar3 = &DAT_00758d5a;
    for (iVar1 = 0x19; iVar1 != 0; iVar1 = iVar1 - 1) {
        *puVar3 = *puVar2;
        puVar2 = puVar2 + 1;
        puVar3 = puVar3 + 1;
    }
    puVar2 = &DAT_00758e9e;
    for (iVar1 = 0x10; iVar1 != 0; iVar1 = iVar1 - 1) {
        *puVar2 = 0;
        puVar2 = puVar2 + 1;
    }
    return;
}

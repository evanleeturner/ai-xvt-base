/* SPDX-License-Identifier: MIT */
/* z_bop 004cddc0 */
extern unsigned int DAT_00acebc5;
extern unsigned int DAT_00acebd1;
extern unsigned int DAT_00acebd5;
extern unsigned int DAT_00acebd9;
extern unsigned int DAT_00acebdd;

bool __cdecl FUN_004cddc0(unsigned int param_1)
{
    unsigned int local_50[0x14];
    unsigned int *puVar1;
    int iVar2;
    int i;

    puVar1 = local_50;
    for (i = 0x14; i != 0; i = i + -1) {
        *puVar1 = 0;
        puVar1 = puVar1 + 1;
    }
    local_50[0] = 0x50;
    local_50[1] = 0x40;
    local_50[6] = DAT_00acebd1;
    local_50[10] = 0x20;
    local_50[7] = DAT_00acebd5;
    local_50[8] = DAT_00acebd9;
    local_50[9] = DAT_00acebdd;
    local_50[12] = param_1;

    int (__stdcall *func)(unsigned int, unsigned int *, unsigned int) =
        (int (__stdcall *)(unsigned int, unsigned int *, unsigned int))
        (*(unsigned int *)(*(unsigned int *)DAT_00acebc5 + 0x60));
    iVar2 = func(DAT_00acebc5, local_50, 2);
    return iVar2 == 0;
}

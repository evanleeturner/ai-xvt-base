/* SPDX-License-Identifier: MIT */
/* z_bop 004ddc40 */
extern unsigned int DAT_00aa734e;
extern unsigned int DAT_00aa7352;
extern unsigned int *DAT_00aa7346;
extern unsigned int *DAT_00aa734a;
extern unsigned int DAT_00aa721a;
extern unsigned int DAT_00aa727e;
extern unsigned int DAT_0052c100;
extern unsigned int DAT_00aa735e;

void __stdcall FUN_004ddc40(void)
{
    unsigned int iVar1;
    unsigned int *puVar2;
    unsigned int *puVar3;
    
    DAT_00aa734e = 10;
    DAT_00aa7352 = 10;
    DAT_00aa7346 = &DAT_00aa721a;
    DAT_00aa734a = &DAT_00aa727e;
    puVar2 = &DAT_0052c100;
    puVar3 = &DAT_00aa721a;
    for (iVar1 = 0x19; iVar1 != 0; iVar1 = iVar1 - 1) {
        *puVar3 = *puVar2;
        puVar2 = puVar2 + 1;
        puVar3 = puVar3 + 1;
    }
    puVar2 = &DAT_00aa735e;
    for (iVar1 = 0x10; iVar1 != 0; iVar1 = iVar1 - 1) {
        *puVar2 = 0;
        puVar2 = puVar2 + 1;
    }
    return;
}

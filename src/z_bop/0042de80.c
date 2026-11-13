/* SPDX-License-Identifier: MIT */
/* z_bop 0042de80 */
extern int DAT_00a0fe80;
extern int DAT_00a0aba0;
extern int DAT_009a1fe0;
extern int DAT_00a0a7f0;
extern int DAT_009a20a4;
extern int DAT_00a0aa10;

void __stdcall FUN_0042de80(void)
{
    int iVar1;
    unsigned int uVar2;
    unsigned int *puVar3;

    puVar3 = (unsigned int *)&DAT_00a0fe80;
    for (iVar1 = 0x1a3; iVar1 != 0; iVar1 = iVar1 - 1) {
        *puVar3 = 0;
        puVar3 = puVar3 + 1;
    }
    uVar2 = 0;
    do {
        puVar3 = (unsigned int *)((int)&DAT_00a0aba0 + uVar2);
        uVar2 = uVar2 + 0x61;
        for (iVar1 = 0x18; iVar1 != 0; iVar1 = iVar1 - 1) {
            *puVar3 = 0;
            puVar3 = puVar3 + 1;
        }
        *(unsigned char *)puVar3 = 0;
    } while (uVar2 < 0x246);
    if (DAT_009a1fe0 != 0) {
        puVar3 = (unsigned int *)&DAT_00a0a7f0;
        for (iVar1 = DAT_009a1fe0; iVar1 != 0; iVar1 = iVar1 - 1) {
            *puVar3 = 0;
            puVar3 = puVar3 + 1;
        }
    }
    DAT_009a20a4 = 0;
    DAT_00a0aa10 = 0;
    return;
}

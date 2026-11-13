/* SPDX-License-Identifier: MIT */
/* z_bop 004f36d0 */
extern int DAT_00aa5e10;
extern int DAT_00aa5e68;
extern short DAT_00aa5aac;
extern unsigned char DAT_00a91d18;
extern int DAT_00aa5e38;

void __stdcall FUN_004f36d0(void)
{
    int iVar2;
    int *piVar1;
    unsigned char *pbVar3;
    int *puVar4;

    puVar4 = (int *)&DAT_00aa5e10;
    for (iVar2 = 10; iVar2 != 0; iVar2 = iVar2 + -1) {
        *puVar4 = 0;
        puVar4 = puVar4 + 1;
    }
    DAT_00aa5e68 = 0;
    if (0 < DAT_00aa5aac) {
        pbVar3 = (unsigned char *)&DAT_00a91d18;
        iVar2 = (int)DAT_00aa5aac;
        do {
            if (pbVar3[0xc] != 0) {
                ((int *)&DAT_00aa5e10)[*pbVar3] = ((int *)&DAT_00aa5e10)[*pbVar3] + 1;
            }
            pbVar3 = pbVar3 + 0x562;
            iVar2 = iVar2 + -1;
        } while (iVar2 != 0);
    }
    piVar1 = (int *)&DAT_00aa5e10;
    do {
        if (*piVar1 != 0) {
            DAT_00aa5e68 = DAT_00aa5e68 + 1;
        }
        piVar1 = piVar1 + 1;
    } while (piVar1 < (int *)&DAT_00aa5e38);
    return;
}

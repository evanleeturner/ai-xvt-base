/* SPDX-License-Identifier: MIT */
/* z_xvt 004f0100 */
extern int DAT_007579d0[];
extern int DAT_00757a28;
extern short DAT_0075766c;
extern unsigned char DAT_007438d8[];
extern int DAT_007579f8;

void __stdcall FUN_004f0100(void)
{
    int *piVar1;
    int iVar2;
    unsigned char *pbVar3;
    int *puVar4;
    
    puVar4 = DAT_007579d0;
    for (iVar2 = 10; iVar2 != 0; iVar2 = iVar2 + -1) {
        *puVar4 = 0;
        puVar4 = puVar4 + 1;
    }
    DAT_00757a28 = 0;
    if (0 < DAT_0075766c) {
        pbVar3 = DAT_007438d8;
        iVar2 = (int)DAT_0075766c;
        do {
            if (pbVar3[0xc] != 0) {
                DAT_007579d0[*pbVar3] = DAT_007579d0[*pbVar3] + 1;
            }
            pbVar3 = pbVar3 + 0x562;
            iVar2 = iVar2 + -1;
        } while (iVar2 != 0);
    }
    piVar1 = DAT_007579d0;
    do {
        if (*piVar1 != 0) {
            DAT_00757a28 = DAT_00757a28 + 1;
        }
        piVar1 = piVar1 + 1;
    } while (piVar1 < &DAT_007579f8);
    return;
}

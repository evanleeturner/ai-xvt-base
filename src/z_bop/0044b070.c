/* SPDX-License-Identifier: MIT */
/* z_bop 0044b070 */
extern int __cdecl FUN_0040e190(int param_1);
extern void __cdecl FUN_00410ff0(unsigned int param_1, unsigned int param_2);

extern char DAT_00523928[];
extern char DAT_00523929[];
extern unsigned int DAT_005233cc;
extern unsigned int DAT_005233ec;
extern unsigned int DAT_005233f4;
extern unsigned short DAT_005569c0[];

void __cdecl FUN_0044b070(unsigned int param_1, unsigned int param_2)
{
    int iVar1;
    unsigned int uVar2;
    unsigned short uVar3;
    short sVar4;
    unsigned int uVar5;

    sVar4 = 7;
    uVar5 = 0;
    uVar3 = 0;
    do {
        iVar1 = FUN_0040e190((int)(signed char)DAT_00523929[uVar5] + (param_2 & 0xffff));
        uVar2 = iVar1 + ((int)(signed char)DAT_00523928[uVar5] + (param_1 & 0xffff)) * 2;
        if (DAT_005233ec != 0x13 && DAT_005233f4 == 0xa0000) {
            uVar2 = uVar2 % DAT_005233cc;
            FUN_00410ff0(uVar2, DAT_005233f4 & 0xfffffffe);
        }
        uVar5 = (unsigned int)(unsigned short)((short)uVar5 + 2);
        sVar4 = sVar4 - 1;
        *(unsigned short *)(uVar2 + (DAT_005233f4 & 0xfffffffe)) = DAT_005569c0[uVar3];
        uVar3 = uVar3 + 1;
    } while (sVar4 != 0);
    return;
}

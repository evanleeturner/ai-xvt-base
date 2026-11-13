/* SPDX-License-Identifier: MIT */
/* z_bop 004106c0 */
extern short DAT_0051a81c;
extern char *PTR_DAT_0051a818;
extern unsigned short DAT_00a08c86;
extern unsigned short DAT_00a08c88;
extern int DAT_005233ec;
extern int DAT_005233f4;
extern unsigned int DAT_005233cc;
extern char DAT_0054f9c0;

int __cdecl FUN_0040e190(int param_1);
void __stdcall FUN_00410ff0(void);

void __stdcall FUN_004106c0(void)
{
    unsigned short uVar1 = 0;
    unsigned int uVar6 = 0;
    short sVar4;
    char *pcVar5;
    int iVar2;
    unsigned int uVar3;

    for (sVar4 = DAT_0051a81c; sVar4 != 0; sVar4 = sVar4 - 1) {
        pcVar5 = PTR_DAT_0051a818 + uVar6;
        iVar2 = FUN_0040e190((int)pcVar5[1] + (unsigned int)DAT_00a08c86);
        uVar3 = (unsigned int)iVar2 + (unsigned int)*pcVar5 + (unsigned int)DAT_00a08c88;
        if ((DAT_005233ec != 0x13) && (DAT_005233f4 == 0xa0000)) {
            uVar3 = uVar3 % DAT_005233cc;
            FUN_00410ff0();
        }
        uVar6 = (unsigned int)(unsigned short)((short)uVar6 + 2);
        *(unsigned char *)(uVar3 + DAT_005233f4) = (&DAT_0054f9c0)[uVar1];
        uVar1 = uVar1 + 1;
    }
    return;
}

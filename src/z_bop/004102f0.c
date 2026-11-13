/* SPDX-License-Identifier: MIT */
/* z_bop 004102f0 */
extern int DAT_005233ec;
extern int DAT_005233f4;
extern unsigned int DAT_005233cc;
extern short DAT_009ed238;

extern int __cdecl FUN_0040e190(int param_1);
extern void __stdcall FUN_00410ff0(void);

void __cdecl FUN_004102f0(unsigned char *param_1, int param_2, int param_3, short param_4, int param_5)
{
    unsigned char uVar1;
    unsigned char *puVar2;
    int iVar3;
    unsigned int uVar4;

    for (; param_5 != 0; param_5--) {
        iVar3 = FUN_0040e190(param_3);
        uVar4 = (unsigned int)(param_2 + iVar3);
        if ((DAT_005233ec != 0x13) && (DAT_005233f4 == 0xa0000)) {
            uVar4 = uVar4 % DAT_005233cc;
            FUN_00410ff0();
        }
        puVar2 = (unsigned char *)(uVar4 + DAT_005233f4);
        for (DAT_009ed238 = param_4; DAT_009ed238 != 0; DAT_009ed238--) {
            uVar1 = *param_1;
            param_1++;
            *puVar2 = uVar1;
            puVar2++;
        }
        param_3++;
        DAT_009ed238 = 0;
    }
    return;
}

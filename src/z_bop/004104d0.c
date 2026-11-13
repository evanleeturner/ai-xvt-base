/* SPDX-License-Identifier: MIT */
/* z_bop 004104d0 */
extern unsigned int DAT_005233cc;
extern unsigned int DAT_005233ec;
extern unsigned int DAT_005233f4;
extern int __cdecl FUN_0040e190(int param_1);
extern void __stdcall FUN_00410ff0(void);

void __cdecl FUN_004104d0(unsigned short *param_1, short param_2)
{
    unsigned short uVar1;
    int iVar2;
    unsigned int uVar3;

    while (param_2 != 0) {
        uVar1 = *param_1;
        iVar2 = FUN_0040e190((unsigned int)param_1[1]);
        uVar3 = (unsigned int)uVar1 + (unsigned int)iVar2;
        if ((DAT_005233ec != 0x13) && (DAT_005233f4 != 0xa0000)) {
            uVar3 = uVar3 % DAT_005233cc;
            FUN_00410ff0();
        }
        if ((param_1[2] & 1) != 0) {
            *(unsigned char *)(uVar3 + DAT_005233f4) = 0x2c;
        }
        if (DAT_005233ec == 0x101) {
            uVar1 = *param_1;
            iVar2 = FUN_0040e190((unsigned int)(unsigned short)(param_1[1] + 1));
            uVar3 = (unsigned int)uVar1 + (unsigned int)iVar2;
            if ((DAT_005233ec != 0x13) && (DAT_005233f4 != 0xa0000)) {
                uVar3 = uVar3 % DAT_005233cc;
                FUN_00410ff0();
            }
            if ((param_1[2] & 2) != 0) {
                *(unsigned char *)(uVar3 + DAT_005233f4) = 0x2c;
            }
        }
        param_1 += 3;
        param_2 -= 1;
    }
}

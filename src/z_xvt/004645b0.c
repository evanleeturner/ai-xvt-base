/* SPDX-License-Identifier: MIT */
/* z_xvt 004645b0 */
extern int DAT_005f1d20;
extern int DAT_0052552c;
extern int DAT_00525534;
extern unsigned int DAT_0052550c;
extern short DAT_0071f2e8;

void __cdecl FUN_004645b0(unsigned char *param_1, int param_2, int param_3, short param_4, int param_5)
{
    int iVar4;
    unsigned int uVar2;
    unsigned char *puVar3;
    unsigned char uVar1;

    if (param_5 != 0) {
        iVar4 = param_3 << 2;
        do {
            uVar2 = param_2 + *(int *)(DAT_005f1d20 + iVar4);
            if (DAT_0052552c != 0x13 && DAT_00525534 != 0xa0000) {
                uVar2 = uVar2 % DAT_0052550c;
            }
            puVar3 = (unsigned char *)(uVar2 + DAT_00525534);
            for (DAT_0071f2e8 = param_4; DAT_0071f2e8 != 0; DAT_0071f2e8--) {
                uVar1 = *puVar3;
                puVar3++;
                *param_1 = uVar1;
                param_1++;
            }
            param_5--;
            iVar4 += 4;
            DAT_0071f2e8 = 0;
        } while (param_5 != 0);
    }
}

/* SPDX-License-Identifier: MIT */
/* z_bop 004277f0 */
void __cdecl FUN_004a9590(unsigned int param_1);
extern void (__cdecl *DAT_009a6fe4)(int);
extern unsigned char DAT_009ec464;
extern unsigned char DAT_009a807a;
extern unsigned char DAT_00520eb0[];

void __cdecl FUN_004277f0(unsigned short param_1, unsigned int param_2, unsigned int param_3)
{
    unsigned char uVar3;
    unsigned char uVar2;
    unsigned short uVar4;
    short sVar5;
    int bVar1;

    uVar3 = DAT_009ec464;
    uVar2 = DAT_009a807a;
    if (param_1 != 0xffff) {
        bVar1 = 0;
        for (; param_2 != 0; param_2 = param_2 - 1) {
            uVar4 = param_1 / *(unsigned short *)(DAT_00520eb0 + param_2 * 2);
            param_1 = param_1 - *(unsigned short *)(DAT_00520eb0 + param_2 * 2) * uVar4;
            if (((bVar1) || (param_2 <= param_3)) || (uVar4 != 0)) {
                bVar1 = 1;
                if (9 < uVar4) {
                    uVar4 = 9;
                }
                sVar5 = uVar4 + 0x30;
            }
            else {
                sVar5 = 0x20;
                bVar1 = 0;
            }
            (*DAT_009a6fe4)(sVar5);
        }
        return;
    }
    DAT_009ec464 = 0;
    FUN_004a9590(0x40);
    for (; param_2 != 0; param_2 = param_2 - 1) {
        (*DAT_009a6fe4)(0x30);
    }
    DAT_009ec464 = uVar3;
    DAT_009a807a = uVar2;
    return;
}

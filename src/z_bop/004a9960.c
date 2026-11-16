/* SPDX-License-Identifier: MIT */
/* z_bop 004a9960 */
extern unsigned short DAT_009cd270;
extern short DAT_009d8c00;
extern short DAT_00a08108;
extern unsigned char DAT_00523d40;
extern void (__cdecl *DAT_009a6fe4)(unsigned char);

void __cdecl FUN_004a9590(unsigned int param_1);
unsigned long long __cdecl FUN_004278c0(unsigned char *param_1);

void __cdecl FUN_004a9960(unsigned char *param_1)
{
    unsigned short uVar1;
    unsigned char *pbVar2;
    unsigned long long uVar3;
    unsigned long long uVar4;
    unsigned char bVar5;
    unsigned char local_50[80];

    bVar5 = *param_1;
    while (bVar5 != 0) {
        bVar5 = *param_1;
        if (bVar5 == 0xfe) {
            param_1 = param_1 + 1;
            FUN_004a9590((unsigned int)*param_1);
        }
        else if (bVar5 < 0x10) {
            FUN_004a9590((unsigned int)bVar5);
        }
        else {
            if ((bVar5 == 0x20) && (DAT_009cd270 != 0)) {
                bVar5 = param_1[1];
                pbVar2 = param_1 + 1;
                uVar1 = 0;
                while ((bVar5 != 0x20) && (bVar5 = *pbVar2, bVar5 != 0)) {
                    pbVar2 = pbVar2 + 1;
                    local_50[uVar1] = bVar5;
                    bVar5 = *pbVar2;
                    uVar1 = (unsigned short)(uVar1 + 1);
                }
                local_50[uVar1] = 0;
                uVar3 = FUN_004278c0(local_50);
                uVar4 = FUN_004278c0(&DAT_00523d40);
                if ((int)((unsigned int)(int)DAT_009d8c00 - 2u) <
                    (int)(((unsigned int)uVar3 & 0xffffu) + ((unsigned int)uVar4 & 0xffffu) + (unsigned int)(int)DAT_00a08108)) {
                    bVar5 = 10;
                }
                else {
                    bVar5 = *param_1;
                }
            }
            (*DAT_009a6fe4)(bVar5);
        }
        pbVar2 = param_1 + 1;
        param_1 = param_1 + 1;
        bVar5 = *pbVar2;
    }
    return;
}

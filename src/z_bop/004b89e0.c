/* SPDX-License-Identifier: MIT */
/* z_bop 004b89e0 */
extern unsigned char DAT_00bb72bc[];

void FUN_004d6460(int *param_1, int param_2, int param_3, int param_4, int param_5);
char *FUN_004ddc10(unsigned int param_1);
void FUN_004db7d0(int param_1, char *param_2, int *param_3, int param_4, int param_5, unsigned int param_6);
void FUN_004d64a0(int *param_1, int param_2, int param_3);
void FUN_004b8ea0(unsigned char *param_1, int *param_2, int param_3);

void FUN_004b89e0(int param_1)
{
    int local_10[4];
    char *pcVar1;
    int *piVar2;
    int iVar3;
    int iVar4;
    unsigned int uVar5;

    FUN_004d6460(local_10, 0x164, 199, 600, 0xd5);
    piVar2 = local_10;
    uVar5 = 0xffff;
    iVar4 = 1;
    iVar3 = 0;
    pcVar1 = FUN_004ddc10(0xf2);
    FUN_004db7d0(0xc, pcVar1, piVar2, iVar3, iVar4, uVar5);
    FUN_004d64a0(local_10, 0, 0xf);
    FUN_004b8ea0(DAT_00bb72bc + param_1, local_10, 0xec);
    return;
}

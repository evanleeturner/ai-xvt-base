/* SPDX-License-Identifier: MIT */
/* z_bop 004b8c80 */
void __cdecl FUN_004d6460(int *param_1, int param_2, int param_3, int param_4, int param_5);
char * __cdecl FUN_004ddc10(unsigned int param_1);
void __cdecl FUN_004db7d0(int param_1, char *param_2, int *param_3, int param_4, int param_5, unsigned int param_6);
void __cdecl FUN_004d64a0(int *param_1, int param_2, int param_3);
void __cdecl FUN_004b8ea0(unsigned char *param_1, int *param_2, int param_3);

extern unsigned char DAT_00bb72ba;

void __cdecl FUN_004b8c80(int param_1)
{
    int local_10[4];
    int *piVar2;
    unsigned int uVar5;
    int iVar4;
    int iVar3;
    char *pcVar1;

    FUN_004d6460(local_10, 0x164, 0x14a, 600, 0x158);
    piVar2 = local_10;
    uVar5 = 0xffff;
    iVar4 = 1;
    iVar3 = 0;
    pcVar1 = FUN_004ddc10(0xf1);
    FUN_004db7d0(0xc, pcVar1, piVar2, iVar3, iVar4, uVar5);
    FUN_004d64a0(local_10, 0, 0xf);
    FUN_004b8ea0(&DAT_00bb72ba + param_1, local_10, 0xec);
    return;
}

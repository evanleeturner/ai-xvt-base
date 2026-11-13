/* SPDX-License-Identifier: MIT */
/* z_bop 004b8570 */
extern unsigned char DAT_00bb72c6[];
extern unsigned char DAT_00bb72c4[];

void __cdecl FUN_004d6460(int *param_1, int param_2, int param_3, int param_4, int param_5);
char * __cdecl FUN_004ddc10(unsigned int param_1);
void __cdecl FUN_004db7d0(int param_1, char *param_2, int *param_3, int param_4, int param_5, unsigned int param_6);
void __cdecl FUN_004d64a0(int *param_1, int param_2, int param_3);
void __cdecl FUN_004b9090(char *param_1, int *param_2, int param_3);

void __cdecl FUN_004b8570(int param_1)
{
    unsigned char *pbVar1;
    char *pcVar2;
    int *piVar3;
    int iVar4;
    int iVar5;
    unsigned int uVar6;
    int local_10[4];

    FUN_004d6460(local_10, 0x58, 0xa0, 0x14c, 0xae);
    piVar3 = local_10;
    uVar6 = 0xffff;
    iVar5 = 1;
    iVar4 = 0;
    pcVar2 = FUN_004ddc10(0xe0);
    FUN_004db7d0(0xc, pcVar2, piVar3, iVar4, iVar5, uVar6);
    FUN_004d64a0(local_10, 0, 0xf);
    pbVar1 = DAT_00bb72c6 + param_1;
    FUN_004b9090((char *)pbVar1, local_10, 0xe1);
    if (DAT_00bb72c4[param_1] < *pbVar1) {
        *pbVar1 = DAT_00bb72c4[param_1];
    }
    return;
}

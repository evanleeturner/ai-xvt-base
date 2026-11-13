/* SPDX-License-Identifier: MIT */
/* z_bop 004f4f80 */
extern int DAT_00b6a5aa;
extern int DAT_00aa6118;
extern int DAT_0052c208;
extern int DAT_00bb2810;

void __cdecl FUN_004d6460(int *param_1, int param_2, int param_3, int param_4, int param_5);
char * __cdecl FUN_004ddc10(unsigned int param_1);
void __cdecl FUN_004db7d0(int param_1, char *param_2, int *param_3, int param_4, int param_5, unsigned int param_6);
int __cdecl FUN_004dbbd0(int param_1, int param_2, int *param_3, unsigned int param_4, int param_5, int param_6);
int __cdecl FUN_004d9d70(int *param_1, int param_2, int param_3, int param_4, int param_5, int param_6, int param_7);

int __stdcall FUN_004f4f80(void)
{
    int local_10[4];
    unsigned int uVar3;
    int *piVar4;
    int iVar2;
    int iVar5;
    unsigned int uVar6;
    unsigned int uVar7;
    char *pcVar1;

    FUN_004d6460(local_10, 0x58, 0xcf, 0x1ae, 0xe0);
    if (DAT_00b6a5aa == 2) {
        uVar3 = 0x1d7;
    }
    else if (DAT_00b6a5aa == 4) {
        uVar3 = 0x1d8;
    }
    else if (DAT_00b6a5aa == 5) {
        uVar3 = 0x309;
    }
    else {
        uVar3 = 0xc0;
    }
    piVar4 = local_10;
    uVar7 = 0xffff;
    iVar5 = 1;
    iVar2 = 0;
    pcVar1 = FUN_004ddc10(uVar3);
    FUN_004db7d0(0xf, pcVar1, piVar4, iVar2, iVar5, uVar7);
    FUN_004d6460(local_10, 0x58, 0xe1, 0x1a4, 0x1b1);
    iVar2 = FUN_004dbbd0(0xc, DAT_00aa6118, local_10, 0xffff, 4, 0x1000);
    uVar6 = 0x1ae;
    if (0xd < iVar2 + 1) {
        FUN_004d6460(local_10, 0x1a5, 0xe1, 0x1ae, 0x1b1);
        DAT_0052c208 = FUN_004d9d70(local_10, DAT_0052c208, iVar2 + 1, 0, 5, DAT_00bb2810, 9);
        uVar6 = 0x1a4;
    }
    FUN_004d6460(local_10, 0x58, 0xe1, uVar6, 0x1b1);
    FUN_004dbbd0(0xc, DAT_00aa6118, local_10, 0xffff, 4, DAT_0052c208);
    return 1;
}

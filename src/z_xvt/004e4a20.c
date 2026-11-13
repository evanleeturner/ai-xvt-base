/* SPDX-License-Identifier: MIT */
/* z_xvt 004e4a20 */
extern int DAT_0081bfba;
extern int DAT_00757cc4;
extern int DAT_005330fc;
extern int DAT_00859c2c;

extern void FUN_004dcfd0(int *param_1, int param_2, int param_3, int param_4, int param_5);
extern char *FUN_004dcfa0(unsigned int param_1);
extern void FUN_004daac0(int param_1, char *param_2, int *param_3, int param_4, int param_5, unsigned int param_6);
extern int FUN_004daee0(int param_1, int param_2, int *param_3, unsigned int param_4, int param_5, int param_6);
extern int FUN_004d9010(int *param_1, int param_2, int param_3, int param_4, int param_5, int param_6, int param_7);

int __stdcall FUN_004e4a20(void)
{
    char *pcVar1;
    int iVar2;
    unsigned int uVar3;
    int *piVar4;
    int iVar5;
    unsigned int uVar6;
    unsigned int uVar7;
    int local_10[4];

    FUN_004dcfd0(local_10, 0x58, 0x8a, 0x1ae, 0x99);
    if (DAT_0081bfba == 2) {
        uVar3 = 0x1c9;
    }
    else if (DAT_0081bfba == 4) {
        uVar3 = 0x1ca;
    }
    else {
        uVar3 = 0xb4;
    }
    piVar4 = local_10;
    uVar7 = 0xffff;
    iVar5 = 1;
    iVar2 = 0;
    pcVar1 = FUN_004dcfa0(uVar3);
    FUN_004daac0(0xf, pcVar1, piVar4, iVar2, iVar5, uVar7);
    FUN_004dcfd0(local_10, 0x58, 0x9d, 0x1a4, 0x12d);
    iVar2 = FUN_004daee0(0xc, DAT_00757cc4, local_10, 0xffff, 4, 0x400);
    uVar6 = 0x1ae;
    if (9 < iVar2 + 1) {
        FUN_004dcfd0(local_10, 0x1a5, 0x9d, 0x1ae, 0x12d);
        DAT_005330fc = FUN_004d9010(local_10, DAT_005330fc, iVar2 + 1, 0, 5, DAT_00859c2c, 9);
        uVar6 = 0x1a4;
    }
    FUN_004dcfd0(local_10, 0x58, 0x9d, uVar6, 0x12d);
    FUN_004daee0(0xc, DAT_00757cc4, local_10, 0xffff, 4, DAT_005330fc);
    return 1;
}

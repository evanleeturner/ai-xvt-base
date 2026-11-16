/* SPDX-License-Identifier: MIT */
/* z_bop 004dd220 */
int FUN_004dad90(void);
void FUN_004dad60(void);
void FUN_004d6460(int *param_1, int param_2, int param_3, int param_4, int param_5);
void FUN_004de090(int *param_1, int *param_2);
void FUN_004dc3b0(int param_1, int *param_2);
void FUN_004dc170(void);
void FUN_004dad40(void);
int FUN_004dd2c0(void);

extern int DAT_00665808;
extern int DAT_0066580c;
extern int DAT_00665890;
extern int DAT_00665894;
extern int DAT_00665898;

int __cdecl FUN_004dd220(int *param_1)
{
    int iVar1;
    int iVar2;
    int *puVar3;
    int local_10[4];

    iVar1 = FUN_004dad90();
    FUN_004dad60();
    FUN_004d6460(local_10, 0, 0, 0x280, 0x1e0);
    FUN_004de090(&DAT_00665808, &DAT_0066580c);
    puVar3 = &DAT_00665890;
    for (iVar2 = 0x40; iVar2 != 0; iVar2 = iVar2 - 1) {
        *puVar3 = 0;
        puVar3 = puVar3 + 1;
    }
    FUN_004dc3b0((int)FUN_004dd2c0, local_10);
    FUN_004dc170();
    if (iVar1 != 0) {
        FUN_004dad40();
    }
    *param_1 = DAT_00665890;
    param_1[1] = DAT_00665894;
    param_1[2] = DAT_00665898;
    *(unsigned char *)(param_1 + 3) = 0;
    return 1;
}

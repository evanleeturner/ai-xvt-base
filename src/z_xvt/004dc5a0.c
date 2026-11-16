/* SPDX-License-Identifier: MIT */
/* z_xvt 004dc5a0 */
extern unsigned int __cdecl FUN_004da080(void);
extern void __cdecl FUN_004da050(void);
extern void __cdecl FUN_004dcfd0(int *param_1, int param_2, int param_3, int param_4, int param_5);
extern void __cdecl FUN_004ddc70(void *param_1, void *param_2);
extern void __cdecl FUN_004db6f0(unsigned int *param_1, int *param_2);
extern void __cdecl FUN_004db4c0(void);
extern void __cdecl FUN_004da030(void);

extern unsigned int DAT_0064fa20;
extern unsigned int DAT_0064fa24;
extern unsigned int DAT_0064faa8;
extern unsigned int DAT_0064faac;
extern unsigned int DAT_0064fab0;
extern unsigned int LAB_004dc640;

unsigned int __cdecl FUN_004dc5a0(unsigned int *param_1)
{
    int iVar1;
    int iVar2;
    unsigned int *puVar3;
    int local_10[4];

    iVar1 = FUN_004da080();
    FUN_004da050();
    FUN_004dcfd0(local_10, 0, 0, 0x280, 0x1e0);
    FUN_004ddc70(&DAT_0064fa20, &DAT_0064fa24);
    puVar3 = (unsigned int *)&DAT_0064faa8;
    for (iVar2 = 0x40; iVar2 != 0; iVar2--) {
        *puVar3 = 0;
        puVar3 = puVar3 + 1;
    }
    FUN_004db6f0(&LAB_004dc640, local_10);
    FUN_004db4c0();
    if (iVar1 != 0) {
        FUN_004da030();
    }
    *param_1 = DAT_0064faa8;
    param_1[1] = DAT_0064faac;
    param_1[2] = DAT_0064fab0;
    *(unsigned char *)(param_1 + 3) = 0;
    return 1;
}

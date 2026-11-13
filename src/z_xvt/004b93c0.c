/* SPDX-License-Identifier: MIT */
/* z_xvt 004b93c0 */
void __cdecl FUN_004dcfd0(void *param_1, int param_2, int param_3, int param_4, int param_5);
char * __cdecl FUN_004dcfa0(unsigned int param_1);
void __cdecl FUN_004daac0(int param_1, char *param_2, int *param_3, int param_4, int param_5, unsigned int param_6);
void __cdecl FUN_004dd030(int *param_1, int param_2, int param_3);
void __cdecl FUN_004b9eb0(void *param_1, int *param_2, int param_3, int param_4, int param_5, int param_6, char param_7);
extern char DAT_0085e6e2;

void __cdecl FUN_004b93c0(int param_1)
{
    char local_buf[0x164];
    int *piVar2;
    int iVar3;
    int iVar4;
    unsigned int uVar5;
    char *pcVar1;

    FUN_004dcfd0(local_buf, 0x164, 0x6f, 0x258, 0x7d);
    piVar2 = (int *)local_buf;
    uVar5 = 0xffff;
    iVar4 = 1;
    iVar3 = 0;
    pcVar1 = FUN_004dcfa0(0xee);
    FUN_004daac0(0xc, pcVar1, piVar2, iVar3, iVar4, uVar5);
    FUN_004dd030(piVar2, 0, 0xf);
    FUN_004b9eb0(&DAT_0085e6e2 + param_1, piVar2, 0x14, 0xef, 1, *(int *)piVar2, *(char *)((char *)piVar2 + 4));
    return;
}

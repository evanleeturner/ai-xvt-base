/* SPDX-License-Identifier: MIT */
/* z_bop 004b86e0 */
void __cdecl FUN_004d6460(int *param_1, unsigned int param_2, unsigned int param_3, unsigned int param_4, unsigned int param_5);
char * __cdecl FUN_004ddc10(unsigned int param_1);
void __cdecl FUN_004db7d0(unsigned int param_1, char *param_2, int *param_3, int param_4, int param_5, unsigned int param_6);
void __cdecl FUN_004d64a0(int *param_1, int param_2, int param_3);
void __cdecl FUN_004b93f0(int *param_1, int *param_2, int param_3, int param_4, int param_5);
extern int DAT_00bb72ca;

void __cdecl FUN_004b86e0(int param_1)
{
  char *pcVar1;
  int *piVar2;
  int iVar3;
  int iVar4;
  unsigned int uVar5;
  int local_10[4];
  
  FUN_004d6460(local_10, 0x58, 0xf3, 0x14c, 0x101);
  piVar2 = local_10;
  uVar5 = 0xffff;
  iVar4 = 1;
  iVar3 = 0;
  pcVar1 = FUN_004ddc10(0xff);
  FUN_004db7d0(0xc, pcVar1, piVar2, iVar3, iVar4, uVar5);
  FUN_004d64a0(local_10, 0, 0xf);
  FUN_004b93f0(&DAT_00bb72ca + param_1, local_10, 8, 0x100, 1);
  return;
}

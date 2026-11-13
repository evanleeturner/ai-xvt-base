/* SPDX-License-Identifier: MIT */
/* z_bop 004b8a60 */
extern void __cdecl FUN_004d6460(int *param_1, int param_2, int param_3, int param_4, int param_5);
extern char * __cdecl FUN_004ddc10(unsigned int param_1);
extern void __cdecl FUN_004db7d0(int param_1, char *param_2, int *param_3, int param_4, int param_5, unsigned int param_6);
extern void __cdecl FUN_004d64a0(int *param_1, int param_2, int param_3);
extern void __cdecl FUN_004b93f0(int *param_1, int *param_2, int param_3, int param_4, int param_5);
extern void __cdecl FUN_004db640(int param_1, char *param_2, int *param_3, unsigned int param_4);
extern int DAT_00bb72c0;
extern int DAT_00b6a2c0;

void __cdecl FUN_004b8a60(int param_1)
{
  char *pcVar1;
  int *piVar2;
  int iVar3;
  int iVar4;
  unsigned int uVar5;
  int local_10[4];
  
  FUN_004d6460(local_10, 0x164, 0xe4, 600, 0xf2);
  piVar2 = local_10;
  uVar5 = 0xffff;
  iVar4 = 1;
  iVar3 = 0;
  pcVar1 = FUN_004ddc10(0xf7);
  FUN_004db7d0(0xc, pcVar1, piVar2, iVar3, iVar4, uVar5);
  FUN_004d64a0(local_10, 0, 0xf);
  FUN_004b93f0(&DAT_00bb72c0 + param_1, local_10, 0x14, 0xf8, 1);
  FUN_004d64a0(local_10, 0, 0xf);
  piVar2 = local_10;
  uVar5 = DAT_00b6a2c0;
  pcVar1 = FUN_004ddc10(0xfa);
  FUN_004db640(0xc, pcVar1, piVar2, uVar5);
  return;
}

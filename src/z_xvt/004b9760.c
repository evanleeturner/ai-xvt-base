/* SPDX-License-Identifier: MIT */
/* z_xvt 004b9760 */
void __cdecl FUN_004dcfd0(int *param_1, int param_2, int param_3, int param_4, int param_5);
void * __cdecl FUN_004dcfa0(int param_1);
void __cdecl FUN_004daac0(int param_1, void *param_2, int *param_3, int param_4, int param_5, unsigned int param_6);
void __cdecl FUN_004dd030(int *param_1, int param_2, int param_3);
void __cdecl FUN_004b9970(void *param_1, int *param_2, int param_3);
extern char DAT_0085e6da;

void __cdecl FUN_004b9760(int param_1)
{
  void *pcVar1;
  int *piVar2;
  int iVar3;
  int iVar4;
  unsigned int uVar5;
  int local_10[4];
  
  FUN_004dcfd0(local_10, 0x164, 0x14a, 600, 0x158);
  piVar2 = local_10;
  uVar5 = 0xffff;
  iVar4 = 1;
  iVar3 = 0;
  pcVar1 = FUN_004dcfa0(0xe4);
  FUN_004daac0(0xc, pcVar1, piVar2, iVar3, iVar4, uVar5);
  FUN_004dd030(local_10, 0, 0xf);
  FUN_004b9970(&DAT_0085e6da + param_1, local_10, 0xdf);
  return;
}

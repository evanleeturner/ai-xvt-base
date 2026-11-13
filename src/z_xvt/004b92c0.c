/* SPDX-License-Identifier: MIT */
/* z_xvt 004b92c0 */
extern int DAT_0085e6d0;

void __cdecl FUN_004dcfd0(int *param_1, int param_2, int param_3, int param_4, int param_5);
char * __cdecl FUN_004dcfa0(unsigned int param_1);
void __cdecl FUN_004daac0(int param_1, char *param_2, int *param_3, int param_4, int param_5, unsigned int param_6);
void __cdecl FUN_004dd030(int *param_1, int param_2, int param_3);
void __cdecl FUN_004b9970(int *param_1, int *param_2, int param_3);

void __cdecl FUN_004b92c0(int param_1)
{
  char *pcVar1;
  int *piVar2;
  int iVar3;
  int iVar4;
  unsigned int uVar5;
  int local_10[4];
  
  FUN_004dcfd0(local_10, 0x58, 0x146, 0x14c, 0x154);
  piVar2 = local_10;
  uVar5 = 0xffff;
  iVar4 = 1;
  iVar3 = 0;
  pcVar1 = FUN_004dcfa0(0xde);
  FUN_004daac0(0xc, pcVar1, piVar2, iVar3, iVar4, uVar5);
  FUN_004dd030(local_10, 0, 0xf);
  FUN_004b9970(&DAT_0085e6d0 + param_1, local_10, 0xdf);
  return;
}

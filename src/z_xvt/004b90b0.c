/* SPDX-License-Identifier: MIT */
/* z_xvt 004b90b0 */
void __cdecl FUN_004dcfd0(int *param_1, int param_2, int param_3, int param_4, int param_5);
char *__cdecl FUN_004dcfa0(unsigned int param_1);
void __cdecl FUN_004daac0(int param_1, char *param_2, int *param_3, int param_4, int param_5, unsigned int param_6);
void __cdecl FUN_004dd030(int *param_1, int param_2, int param_3);
void __cdecl FUN_004b9b40(char *param_1, int *param_2, int param_3);
extern unsigned char DAT_0085e6e4[];
extern unsigned char DAT_0085e6e6[];

void __cdecl FUN_004b90b0(int param_1)
{
  unsigned char *pbVar1;
  char *pcVar2;
  int *piVar3;
  int iVar4;
  int iVar5;
  unsigned int uVar6;
  int local_10[4];
  
  FUN_004dcfd0(local_10, 0x58, 0xa0, 0x14c, 0xae);
  piVar3 = local_10;
  uVar6 = 0xffff;
  iVar5 = 1;
  iVar4 = 0;
  pcVar2 = FUN_004dcfa0(0xd3);
  FUN_004daac0(0xc, pcVar2, piVar3, iVar4, iVar5, uVar6);
  FUN_004dd030(local_10, 0, 0xf);
  pbVar1 = DAT_0085e6e6 + param_1;
  FUN_004b9b40((char *)pbVar1, local_10, 0xd4);
  if (DAT_0085e6e4[param_1] < *pbVar1) {
    *pbVar1 = DAT_0085e6e4[param_1];
  }
  return;
}

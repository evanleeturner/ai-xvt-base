/* SPDX-License-Identifier: MIT */
/* z_xvt 004b9010 */
extern char DAT_0085e6e4;
extern char DAT_0085e6e6;

void __cdecl FUN_004dcfd0(int *param_1, int param_2, int param_3, int param_4, int param_5);
char * __cdecl FUN_004dcfa0(unsigned int param_1);
void __cdecl FUN_004daac0(int param_1, char *param_2, int *param_3, int param_4, int param_5, unsigned int param_6);
void __cdecl FUN_004dd030(int *param_1, int param_2, int param_3);
void __cdecl FUN_004b9b40(char *param_1, int *param_2, int param_3);

void __cdecl FUN_004b9010(int param_1)
{
  char cVar1;
  char cVar2;
  char *pcVar3;
  int *piVar4;
  int iVar5;
  int iVar6;
  unsigned int uVar7;
  int local_10[4];
  
  FUN_004dcfd0(local_10, 0x58, 0x6f, 0x14c, 0x7d);
  piVar4 = local_10;
  uVar7 = 0xffff;
  iVar6 = 1;
  iVar5 = 0;
  pcVar3 = FUN_004dcfa0(0xcf);
  FUN_004daac0(0xc, pcVar3, piVar4, iVar5, iVar6, uVar7);
  FUN_004dd030(local_10, 0, 0xf);
  cVar1 = (&DAT_0085e6e4)[param_1];
  FUN_004b9b40(&DAT_0085e6e4 + param_1, local_10, 0xd0);
  cVar2 = (&DAT_0085e6e4)[param_1];
  if (cVar2 != cVar1) {
    (&DAT_0085e6e6)[param_1] = cVar2;
  }
  return;
}

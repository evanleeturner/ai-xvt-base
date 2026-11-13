/* SPDX-License-Identifier: MIT */
/* z_xvt 004b9870 */
extern char DAT_0085e6ec[];
extern char DAT_0085e6ee;
extern unsigned int _DAT_0081b6fc;

void __cdecl FUN_004dcfd0(int *param_1, int param_2, int param_3, int param_4, int param_5);
char * __cdecl FUN_004dcfa0(unsigned int param_1);
void __cdecl FUN_004daac0(int param_1, char *param_2, int *param_3, int param_4, int param_5, unsigned int param_6);
void __cdecl FUN_004dd030(int *param_1, int param_2, int param_3);
void __cdecl FUN_004b9970(void *param_1, int *param_2, int param_3);
void __cdecl FUN_004b9950(void *param_1, int *param_2, int param_3);

void __cdecl FUN_004b9870(int param_1)
{
  char *pcVar1;
  int *piVar2;
  int iVar3;
  int iVar4;
  unsigned int uVar5;
  int local_10[4];
  
  FUN_004dcfd0(local_10, 0x164, 0x184, 600, 0x192);
  if (DAT_0085e6ec[param_1] != '\0') {
    piVar2 = local_10;
    uVar5 = 0xffff;
    iVar4 = 1;
    iVar3 = 0;
    pcVar1 = FUN_004dcfa0(0x2f3);
    FUN_004daac0(0xc, pcVar1, piVar2, iVar3, iVar4, uVar5);
    FUN_004dd030(local_10, 0, 0xf);
    FUN_004b9970(&DAT_0085e6ee + param_1, local_10, 0xdf);
    return;
  }
  piVar2 = local_10;
  iVar4 = 1;
  iVar3 = 0;
  uVar5 = _DAT_0081b6fc;
  pcVar1 = FUN_004dcfa0(0x2f3);
  FUN_004daac0(0xc, pcVar1, piVar2, iVar3, iVar4, uVar5);
  FUN_004dd030(local_10, 0, 0xf);
  FUN_004b9950(&DAT_0085e6ee + param_1, local_10, 0xdf);
  return;
}

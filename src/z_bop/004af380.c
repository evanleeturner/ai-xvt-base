/* SPDX-License-Identifier: MIT */
/* z_bop 004af380 */
extern unsigned char DAT_00521f19[];
extern unsigned short DAT_009a7ed0[];

extern int __cdecl FUN_004ac720(int param_1);
extern void __cdecl FUN_00471ff0(int *param_1);
extern int __cdecl FUN_004af150(int *param_1, int param_2, int param_3);
extern long long __cdecl __ftol(void);
extern void __cdecl FUN_004ac740(void);

int __cdecl FUN_004af380(int param_1, int param_2, int param_3)
{
  int *piVar1;
  int iVar2;
  int iVar3;
  long long lVar4;

  if (DAT_00521f19[param_1 * 0x1a] & 1) {
    return 0;
  }
  piVar1 = (int *)FUN_004ac720((((unsigned int)(param_1 * 5) >> 16) << 16) | DAT_009a7ed0[param_1]);
  if (*piVar1 != (int)piVar1) {
    FUN_00471ff0(piVar1);
  }
  if (*(int *)(**(int **)((int)piVar1 + 10) + 4) == 0x14) {
    param_2 = param_2 + 1;
  }
  if (*(int *)((int)piVar1 + 6) <= param_2) {
    param_2 = *(int *)((int)piVar1 + 6) - 1;
  }
  iVar3 = 0;
  iVar2 = FUN_004af150((int *)(*(int **)((int)piVar1 + 10))[param_2], (int)piVar1, param_3);
  if (iVar2 != 0) {
    lVar4 = __ftol();
    iVar3 = (int)lVar4;
  }
  FUN_004ac740();
  return -iVar3;
}

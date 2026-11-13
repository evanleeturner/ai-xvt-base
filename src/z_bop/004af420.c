/* SPDX-License-Identifier: MIT */
/* z_bop 004af420 */
extern char DAT_00521f19[];
extern short DAT_009a7ed0[];

extern int *FUN_004ac720(int);
extern void FUN_00471ff0(int *);
extern int FUN_004af150(int *, int, int);
extern long long __ftol(void);
extern void FUN_004ac740(void);

int __cdecl FUN_004af420(int param_1, int param_2, int param_3)
{
  int *piVar1;
  int iVar2;
  int uVar3;
  long long lVar4;

  if ((DAT_00521f19[param_1 * 0x1a] & 1) == 0) {
    return 0;
  }
  piVar1 = FUN_004ac720(((param_1 * 5) & 0xFFFF0000) | (unsigned short)DAT_009a7ed0[param_1]);
  if (*piVar1 != (int)piVar1) {
    FUN_00471ff0(piVar1);
  }
  if (*(int *)(**(int **)(piVar1 + 0xA) + 4) == 0x14) {
    param_2 = param_2 + 1;
  }
  if (*(int *)(piVar1 + 0x6) <= param_2) {
    param_2 = *(int *)(piVar1 + 0x6) - 1;
  }
  uVar3 = 0;
  iVar2 = FUN_004af150((int *)(*(int **)(piVar1 + 0xA))[param_2], (int)piVar1, param_3);
  if (iVar2 != 0) {
    lVar4 = __ftol();
    uVar3 = (int)lVar4;
  }
  FUN_004ac740();
  return uVar3;
}

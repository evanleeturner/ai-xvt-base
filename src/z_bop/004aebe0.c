/* SPDX-License-Identifier: MIT */
/* z_bop 004aebe0 */
extern unsigned char DAT_00521f19[];
extern unsigned short DAT_009a7ed0[];

void * __cdecl FUN_004ac720(int param_1);
void __cdecl FUN_00471ff0(int *param_1);
int __cdecl FUN_004ae1a0(int param_1, int *param_2);
long long __cdecl __ftol(void);
void __stdcall FUN_004ac740(void);

int __cdecl FUN_004aebe0(int param_1, int param_2)
{
  int *piVar1;
  int iVar2;
  int uVar3;
  long long lVar4;

  if (param_2 < 0) {
    return 0;
  }
  if ((DAT_00521f19[param_1 * 0x1a] & 1) == 0) {
    return 0;
  }
  piVar1 = (int *)FUN_004ac720(((unsigned int)(param_1 * 5) & 0xFFFF0000) | (unsigned short)DAT_009a7ed0[param_1]);
  if (*piVar1 != (int)piVar1) {
    FUN_00471ff0(piVar1);
  }
  if (*(int *)(**(int **)((int)piVar1 + 10) + 4) == 0x14) {
    param_2 = param_2 + 1;
  }
  if (*(int *)((int)piVar1 + 6) <= param_2) {
    param_2 = *(int *)((int)piVar1 + 6) - 1;
  }
  iVar2 = FUN_004ae1a0((*(int **)((int)piVar1 + 10))[param_2], piVar1);
  if (iVar2 == 0) {
    uVar3 = 0;
  }
  else if (*(int *)(iVar2 + 0x38) == 0) {
    lVar4 = __ftol();
    uVar3 = (int)lVar4;
  }
  else {
    lVar4 = __ftol();
    uVar3 = (int)lVar4;
  }
  FUN_004ac740();
  return uVar3;
}

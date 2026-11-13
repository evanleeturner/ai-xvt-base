/* SPDX-License-Identifier: MIT */
/* z_bop 004aeaa0 */
extern void *FUN_004ac720(int param);
extern void FUN_00471ff0(void *param);
extern int FUN_004ae1a0(int param1, void *param2);
extern long long __ftol(void);
extern void __stdcall FUN_004ac740(void);
extern unsigned char DAT_00521f19[];
extern unsigned short DAT_009a7ed0[];

unsigned int __cdecl FUN_004aeaa0(int param_1, int param_2)
{
  int *piVar1;
  int iVar2;
  unsigned int uVar3;
  long long lVar4;
  
  if (param_2 < 0) {
    return 0;
  }
  if ((DAT_00521f19[param_1 * 0x1a] & 1) == 0) {
    return 0;
  }
  piVar1 = (int *)FUN_004ac720(((unsigned int)(param_1 * 5) >> 16) | (unsigned int)(unsigned short)DAT_009a7ed0[param_1]);
  if (*piVar1 != (int)piVar1) {
    FUN_00471ff0(piVar1);
  }
  if (*(int *)((char *)* (int **)((char *)piVar1 + 10) + 4) == 0x14) {
    param_2 = param_2 + 1;
  }
  if (*(int *)((char *)piVar1 + 6) <= param_2) {
    param_2 = *(int *)((char *)piVar1 + 6) - 1;
  }
  uVar3 = 0;
  iVar2 = FUN_004ae1a0((*(int **)((char *)piVar1 + 10))[param_2], piVar1);
  if (iVar2 != 0) {
    lVar4 = __ftol();
    uVar3 = (unsigned int)lVar4;
  }
  FUN_004ac740();
  return uVar3;
}

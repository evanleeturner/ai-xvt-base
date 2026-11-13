/* SPDX-License-Identifier: MIT */
/* z_bop 004aea00 */
extern unsigned char DAT_00521f19[];
extern short DAT_009a7ed0[];
extern int * __cdecl FUN_004ac720(int param_1);
extern void __cdecl FUN_00471ff0(int *param_1);
extern int __cdecl FUN_004ae1a0(int param_1, int *param_2);
extern long long __cdecl __ftol(void);
extern void __stdcall FUN_004ac740(void);

int __cdecl FUN_004aea00(int param_1, int param_2)
{
  int *piVar1;
  int iVar2;
  int uVar3;
  long long lVar4;
  int *p10;
  int val6;

  if (param_2 < 0) {
    return 0;
  }
  if ((DAT_00521f19[param_1 * 0x1a] & 1) == 0) {
    return 0;
  }
  piVar1 = (int *)FUN_004ac720((((param_1 * 5) >> 16) & 0xFFFF) << 16 | (unsigned short)DAT_009a7ed0[param_1]);
  if (*piVar1 != (int)piVar1) {
    FUN_00471ff0(piVar1);
  }
  __builtin_memcpy(&p10, (char *)piVar1 + 10, 4);
  if (*(int *)(*p10 + 4) == 0x14) {
    param_2 = param_2 + 1;
  }
  __builtin_memcpy(&val6, (char *)piVar1 + 6, 4);
  if (val6 <= param_2) {
    param_2 = val6 - 1;
  }
  uVar3 = 0;
  iVar2 = FUN_004ae1a0(p10[param_2], piVar1);
  if (iVar2 != 0) {
    lVar4 = __ftol();
    uVar3 = (int)lVar4;
  }
  FUN_004ac740();
  return uVar3;
}

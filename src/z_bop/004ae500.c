/* SPDX-License-Identifier: MIT */
/* z_bop 004ae500 */
extern unsigned char DAT_00521f19[];
extern short DAT_009a7ed0[];

extern int __cdecl FUN_004ac720(int param_1);
extern void __cdecl FUN_00471ff0(int *param_1);
extern void __cdecl FUN_004adcc0(int param_1);
extern long long __cdecl __ftol(void);
extern void __stdcall FUN_004ac740(void);

int __cdecl FUN_004ae500(int param_1, int param_2)
{
  int *piVar1;
  long long lVar2;
  int *ptr_at_10;
  int val_at_6;

  if ((DAT_00521f19[param_1 * 0x1a] & 1) == 0) {
    return 0;
  }
  piVar1 = (int *)FUN_004ac720(((unsigned int)(param_1 * 5) & 0xFFFF0000) | (unsigned short)DAT_009a7ed0[param_1]);
  if (*piVar1 != (int)piVar1) {
    FUN_00471ff0(piVar1);
  }
  __builtin_memcpy(&ptr_at_10, (char *)piVar1 + 10, 4);
  __builtin_memcpy(&val_at_6, (char *)piVar1 + 6, 4);
  if (*(int *)((int)ptr_at_10 + 4) != 0x14) {
    param_2 = param_2 + 1;
  }
  if (val_at_6 <= param_2) {
    param_2 = val_at_6 - 1;
  }
  FUN_004adcc0(ptr_at_10[param_2]);
  lVar2 = __ftol();
  FUN_004ac740();
  return (int)lVar2;
}

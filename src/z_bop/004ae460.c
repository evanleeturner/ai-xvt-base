/* SPDX-License-Identifier: MIT */
/* z_bop 004ae460 */
extern char DAT_00521f19[];
extern short DAT_009a7ed0[];
extern int * __cdecl FUN_004ac720(int);
extern void __cdecl FUN_00471ff0(int *);
extern void __cdecl FUN_004adcc0(int);
extern long __cdecl __ftol(void);
extern void __cdecl FUN_004ac740(int);

int __cdecl FUN_004ae460(int param_1, int param_2)
{
  int *piVar1;
  long lVar2;
  int field_6;
  int field_10;

  if ((DAT_00521f19[param_1 * 0x1a] & 1) == 0) {
    return 0;
  }
  piVar1 = FUN_004ac720(((param_1 * 5) & 0xFFFF0000) | (DAT_009a7ed0[param_1] & 0xFFFF));
  if ((int *)*piVar1 != piVar1) {
    FUN_00471ff0(piVar1);
  }
  __builtin_memcpy(&field_6, (char *)piVar1 + 6, 4);
  __builtin_memcpy(&field_10, (char *)piVar1 + 10, 4);
  if (*(int *)(field_10 + 4) == 0x14) {
    param_2 = param_2 + 1;
  }
  if (field_6 <= param_2) {
    param_2 = field_6 - 1;
  }
  FUN_004adcc0(((int *)field_10)[param_2]);
  lVar2 = __ftol();
  FUN_004ac740((int)lVar2);
  return (int)lVar2;
}

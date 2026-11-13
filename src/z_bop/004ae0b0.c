/* SPDX-License-Identifier: MIT */
/* z_bop 004ae0b0 */
extern long long __cdecl FUN_004ae0b0(int param_1);
extern void __cdecl FUN_004add60(int param_1);
extern long long __cdecl __ftol(void);
extern int DAT_00663658[];

long long __cdecl FUN_004ae0b0(int param_1)
{
  long long lVar1;
  
  if (DAT_00663658[param_1] == 0) {
    FUN_004add60(param_1);
  }
  lVar1 = __ftol();
  return lVar1;
}

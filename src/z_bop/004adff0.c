/* SPDX-License-Identifier: MIT */
/* z_bop 004adff0 */
extern int DAT_00663658;
long long __cdecl FUN_004add60(int param_1);
long long __cdecl __ftol(void);

long long __cdecl FUN_004adff0(int param_1)
{
  long long lVar1;
  
  if (*(int *)(&DAT_00663658 + param_1 * 4) == 0) {
    FUN_004add60(param_1);
  }
  lVar1 = __ftol();
  return lVar1;
}

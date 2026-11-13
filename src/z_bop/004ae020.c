/* SPDX-License-Identifier: MIT */
/* z_bop 004ae020 */
extern int DAT_00663658[];

void FUN_004add60(int param_1);
long long __ftol(void);

long long __cdecl FUN_004ae020(int param_1)
{
  long long lVar1;
  
  if (DAT_00663658[param_1] == 0) {
    FUN_004add60(param_1);
  }
  lVar1 = __ftol();
  return lVar1;
}

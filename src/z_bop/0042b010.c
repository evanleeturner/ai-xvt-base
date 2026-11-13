/* SPDX-License-Identifier: MIT */
/* z_bop 0042b010 */
long long __ftol(void);

extern int _DAT_009d12ec;
extern int _DAT_009d12f0;
extern int _DAT_009d1304;

void __stdcall FUN_0042b010(void)
{
  long long lVar1;
  
  lVar1 = __ftol();
  _DAT_009d12ec = (int)(short)lVar1;
  lVar1 = __ftol();
  _DAT_009d12f0 = (int)(short)lVar1;
  lVar1 = __ftol();
  _DAT_009d1304 = (int)(short)lVar1;
  return;
}

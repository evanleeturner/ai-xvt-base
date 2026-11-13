/* SPDX-License-Identifier: MIT */
/* z_xvt 004cffc0 */
extern DWORD GetTickCount(void);
extern DWORD DAT_00759b2c;
extern DWORD DAT_00759b28;

DWORD __stdcall FUN_004cffc0(void)
{
  DWORD DVar1;
  
  if (DAT_00759b2c == 1) {
    DAT_00759b2c = 2;
    DVar1 = GetTickCount();
    DAT_00759b28 = DVar1 + 1000;
  }
  return 1;
}

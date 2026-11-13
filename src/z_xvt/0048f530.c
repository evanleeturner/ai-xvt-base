/* SPDX-License-Identifier: MIT */
/* z_xvt 0048f530 */
extern DWORD DAT_0052a564;
extern DWORD __stdcall timeGetTime(void);

void __stdcall FUN_0048f530(void)
{
  DWORD DVar1;
  
  DVar1 = timeGetTime();
  if (DAT_0052a564 == 0) {
    DAT_0052a564 = DVar1;
  }
  DAT_0052a564 = DAT_0052a564 + ((DVar1 - DAT_0052a564) & 0xfffffffc);
  return;
}

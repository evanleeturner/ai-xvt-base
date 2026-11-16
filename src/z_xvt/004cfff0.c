/* SPDX-License-Identifier: MIT */
/* z_xvt 004cfff0 */
extern DWORD DAT_00759b0c;
extern DWORD DAT_00759b18;
extern DWORD DAT_00759b20;
extern DWORD DAT_00759b24;
extern DWORD DAT_00759b2c;

undefined4 __cdecl FUN_004cfbc0(undefined4, undefined4, byte);
DWORD __stdcall GetTickCount(void);

undefined4 __stdcall FUN_004cfff0(void)
{
  DWORD DVar1;
  
  FUN_004cfbc0(DAT_00759b0c, DAT_00759b24 / 60000, (byte)((DAT_00759b24 % 60000) / 1000));
  DVar1 = GetTickCount();
  DAT_00759b18 = DAT_00759b20 + DVar1 + 2000;
  DAT_00759b2c = 0;
  return 1;
}

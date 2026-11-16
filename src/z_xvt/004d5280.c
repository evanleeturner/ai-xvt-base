/* SPDX-License-Identifier: MIT */
/* z_xvt 004d5280 */
extern DWORD (__stdcall *PTR_GetTickCount_008604f8)(void);

DWORD __stdcall GetTickCount(void)
{
  return (*PTR_GetTickCount_008604f8)();
}

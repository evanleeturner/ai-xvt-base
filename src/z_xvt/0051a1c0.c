/* SPDX-License-Identifier: MIT */
/* z_xvt 0051a1c0 */
DWORD __stdcall timeGetTime(void);
extern DWORD DAT_00536700;

void __stdcall FUN_0051a1c0(void)
{
    DAT_00536700 = timeGetTime();
    return;
}

/* SPDX-License-Identifier: MIT */
/* z_xvt 004a7220 */
extern DWORD (__stdcall *PTR_timeGetTime_0086070c)(void);

DWORD __stdcall timeGetTime(void)
{
    return PTR_timeGetTime_0086070c();
}

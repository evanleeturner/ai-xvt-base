/* SPDX-License-Identifier: MIT */
/* z_xvt 0051a1b0 */
extern DWORD *PTR_timeGetTime_0086070c;

DWORD __stdcall timeGetTime(void)
{
    return ((DWORD (__stdcall *)(void))PTR_timeGetTime_0086070c)();
}

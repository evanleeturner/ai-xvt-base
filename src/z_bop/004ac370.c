/* SPDX-License-Identifier: MIT */
/* z_bop 004ac370 */
extern DWORD (__stdcall *PTR_timeGetTime_00bb96b8)(void);

DWORD __stdcall timeGetTime(void)
{
    return PTR_timeGetTime_00bb96b8();
}

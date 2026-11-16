/* SPDX-License-Identifier: MIT */
/* z_bop 004d4df0 */
extern DWORD PTR_GetTickCount_00bb94d0;

DWORD __stdcall GetTickCount(void)
{
    return ((DWORD (__stdcall *)(void))PTR_GetTickCount_00bb94d0)();
}

/* SPDX-License-Identifier: MIT */
/* z_xvt 0051c0c0 */
extern DWORD _DAT_0065510c;

DWORD __stdcall timeGetTime(void);

void __stdcall FUN_0051c0c0(void)
{
    _DAT_0065510c = timeGetTime();
    return;
}

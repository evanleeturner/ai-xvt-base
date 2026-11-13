/* SPDX-License-Identifier: MIT */
/* z_xvt 004db550 */
extern DWORD DAT_00758ee2;
extern BYTE DAT_00758edf;

BYTE __stdcall FUN_004db550(void)
{
    if (DAT_00758ee2 != 0) {
        return 0;
    }
    return DAT_00758edf;
}

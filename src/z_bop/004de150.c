/* SPDX-License-Identifier: MIT */
/* z_bop 004de150 */
int __stdcall ShowCursor(int bShow);

int __stdcall FUN_004de150(void)
{
    int iVar1;
    do {
        iVar1 = ShowCursor(0);
    } while (iVar1 < 0);
    return 1;
}

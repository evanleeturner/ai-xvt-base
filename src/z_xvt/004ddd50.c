/* SPDX-License-Identifier: MIT */
/* z_xvt 004ddd50 */
int __stdcall ShowCursor(int);

undefined4 __stdcall FUN_004ddd50(void)
{
    int iVar1;
    
    do {
        iVar1 = ShowCursor(1);
    } while (iVar1 < 0);
    return 1;
}

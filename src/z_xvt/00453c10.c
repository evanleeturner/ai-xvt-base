/* SPDX-License-Identifier: MIT */
/* z_xvt 00453c10 */
extern short DAT_00525e08;
extern long long __ftol(void);

void __stdcall FUN_00453c10(void)
{
    long long lVar1;
    
    lVar1 = __ftol();
    DAT_00525e08 = (short)lVar1;
    return;
}

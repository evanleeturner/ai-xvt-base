/* SPDX-License-Identifier: MIT */
/* z_xvt 004d04b0 */
extern int DAT_00758d41;

void __cdecl FUN_004d0430(char *param_1);

void __stdcall FUN_004d04b0(void)
{
    int iVar1;
    
    if (DAT_00758d41 != 0) {
        iVar1 = 0x87bc;
        do {
            FUN_004d0430((char *)(DAT_00758d41 + iVar1));
            iVar1 -= 0x44;
        } while (iVar1 >= 0);
    }
    return;
}

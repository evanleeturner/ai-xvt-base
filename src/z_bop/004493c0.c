/* SPDX-License-Identifier: MIT */
/* z_bop 004493c0 */
extern int DAT_005236a8;
void FUN_00449130(void);

void __stdcall FUN_004493c0(void)
{
    unsigned char bVar1;
    
    bVar1 = (unsigned char)DAT_005236a8;
    while ((bVar1 & 0x7f) != 0x7f) {
        FUN_00449130();
        bVar1 = (unsigned char)DAT_005236a8;
    }
    FUN_00449130();
    return;
}

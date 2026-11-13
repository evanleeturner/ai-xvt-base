/* SPDX-License-Identifier: MIT */
/* z_bop 00463cc0 */
extern int DAT_00b6d5de[];
int __cdecl FUN_00463c80(int param_1);

void __cdecl FUN_00463cc0(int param_1)
{
    int iVar1;
    
    iVar1 = FUN_00463c80(param_1);
    DAT_00b6d5de[iVar1 * 0x16] = 1;
    return;
}

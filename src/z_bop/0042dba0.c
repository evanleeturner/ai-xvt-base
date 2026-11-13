/* SPDX-License-Identifier: MIT */
/* z_bop 0042dba0 */
extern int __cdecl FUN_0042dda0(char *param_1);
extern unsigned char DAT_00a10660[];

unsigned char __cdecl FUN_0042dba0(char *param_1)
{
    int iVar1;
    
    iVar1 = FUN_0042dda0(param_1);
    if (iVar1 == -1) {
        return 0;
    }
    return DAT_00a10660[iVar1 * 0x145];
}

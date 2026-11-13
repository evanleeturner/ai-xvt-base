/* SPDX-License-Identifier: MIT */
/* z_bop 004b15e0 */
int __cdecl FUN_004b15e0(int param_1)
{
    int iVar1 = 0;
    for (; 1 < param_1; param_1 = param_1 >> 1) {
        iVar1 = iVar1 + 1;
    }
    return iVar1;
}

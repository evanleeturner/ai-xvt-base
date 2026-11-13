/* SPDX-License-Identifier: MIT */
/* z_bop 004b3280 */
void __cdecl FUN_004b3160(int);
void __cdecl FUN_004b30f0(int);
extern int DAT_00528c84;

void __cdecl FUN_004b3280(int param_1)
{
    *(int *)(param_1 + 0x8c) = DAT_00528c84;
    FUN_004b3160(param_1);
    FUN_004b30f0(param_1);
    return;
}

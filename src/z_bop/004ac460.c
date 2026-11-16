/* SPDX-License-Identifier: MIT */
/* z_bop 004ac460 */
extern int DAT_0066e700;

int __stdcall FUN_004ac460(void)
{
    int vtable = DAT_0066e700;
    int func = *(int *)(vtable + 0x6c);
    int result = ((int (__stdcall *)(int))func)(DAT_0066e700);
    return result == 0;
}

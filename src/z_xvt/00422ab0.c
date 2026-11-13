/* SPDX-License-Identifier: MIT */
/* z_xvt 00422ab0 */
unsigned short __cdecl FUN_00478470(void);

short __cdecl FUN_00422ab0(unsigned short param_1)
{
    unsigned short uVar1;

    if (param_1 == 0) {
        return 0;
    }
    uVar1 = FUN_00478470();
    return (short)(unsigned short)((unsigned int)uVar1 - (unsigned int)(unsigned short)(uVar1 / param_1) * (unsigned int)param_1);
}

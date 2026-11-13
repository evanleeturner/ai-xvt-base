/* SPDX-License-Identifier: MIT */
/* z_xvt 004a3490 */
extern unsigned short DAT_00732950[];
extern unsigned int DAT_00732e70[];

void __cdecl FUN_004a3490(unsigned short param_1, unsigned int param_2)
{
    DAT_00732950[(unsigned int)param_1] = (unsigned short)(param_1 + 0x8000);
    DAT_00732e70[(unsigned int)param_1] = param_2;
    return;
}

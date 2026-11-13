/* SPDX-License-Identifier: MIT */
/* z_bop 00421980 */
extern unsigned short DAT_0051c018[];
extern unsigned short DAT_0051c248[];
extern unsigned short DAT_0051c130[];

unsigned short __cdecl FUN_00421980(unsigned short param_1, short param_2)
{
    unsigned int uVar1;
    
    uVar1 = (unsigned int)(param_1 >> 6);
    if (param_2 == 0x5b) {
        return DAT_0051c018[uVar1];
    }
    if (param_2 == 0x6e) {
        return DAT_0051c248[uVar1];
    }
    return DAT_0051c130[uVar1];
}

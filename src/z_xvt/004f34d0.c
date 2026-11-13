/* SPDX-License-Identifier: MIT */
/* z_xvt 004f34d0 */
short __cdecl FUN_004f34d0(short param_1, short param_2, short param_3)
{
    short sVar1;

    if ((((param_1 <= param_2) ||
          (sVar1 = (short)(unsigned short)((unsigned short)param_1 - (unsigned short)param_3),
           param_1 = param_2,
           sVar1 < param_2)) &&
         (sVar1 = param_1, param_1 < param_2)) &&
        (sVar1 = (short)(unsigned short)((unsigned short)param_1 + (unsigned short)param_3),
         param_2 < sVar1)) {
        sVar1 = param_2;
    }
    return sVar1;
}

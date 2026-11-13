/* SPDX-License-Identifier: MIT */
/* z_xvt 00478350 */
uint __cdecl FUN_00478350(uint param_1, ushort param_2)
{
    if (param_2 == 0xffff) {
        return param_1;
    }
    return ((param_1 & 0xffff) * (uint)param_2 >> 0x10) + (param_1 >> 0x10) * (uint)param_2;
}

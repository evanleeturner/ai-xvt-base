/* SPDX-License-Identifier: MIT */
/* z_xvt 00478320 */
uint __cdecl FUN_00478320(uint param_1, ushort param_2)
{
    uint uVar1;

    uVar1 = param_1 & 0xffff;
    if (param_2 != 0xffff) {
        uVar1 = ((uint)param_2 * uVar1 + 0x8000) >> 0x10;
    }
    return uVar1;
}

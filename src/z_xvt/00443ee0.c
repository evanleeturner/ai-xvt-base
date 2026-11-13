/* SPDX-License-Identifier: MIT */
/* z_xvt 00443ee0 */
int __cdecl FUN_00443ee0(uint param_1)
{
    int iVar1;
    unsigned short uVar2;
    unsigned short local_2;

    iVar1 = (param_1 & 0xffff) - 0x8f;
    uVar2 = (unsigned short)((uint)iVar1 >> 0x10);
    local_2 = (unsigned short)iVar1;
    switch (iVar1) {
        case 0:
            return (uint)uVar2 << 0x10;
        case 1:
            return ((uint)uVar2 << 0x10) | 1;
        case 2:
            return ((uint)uVar2 << 0x10) | 2;
        case 3:
            return ((uint)uVar2 << 0x10) | 3;
        case 4:
            return ((uint)uVar2 << 0x10) | 4;
        case 5:
            return ((uint)uVar2 << 0x10) | 5;
        case 6:
            return ((uint)uVar2 << 0x10) | 6;
        default:
            return ((uint)uVar2 << 0x10) | local_2;
    }
}

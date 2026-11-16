/* SPDX-License-Identifier: MIT */
/* z_xvt 004830c0 */
extern int DAT_0071b730;
extern int DAT_0071ecf4;
extern int DAT_006d4988;
extern unsigned short DAT_00523b70[];

uint __stdcall FUN_004830c0(void)
{
    int iVar1;
    int bVar2;
    short sVar3;
    uint uVar4;
    uint auStack_14[5];

    iVar1 = *(int *)((int)&DAT_0071b730 + DAT_0071ecf4 * 0x5bd);
    if (iVar1 == -1) {
        return 0xffff0000;
    }
    iVar1 = *(int *)(*(int *)(iVar1 * 0x23 + 0x1f + DAT_006d4988) + 0xa7);
    if (iVar1 == 0) {
        return DAT_006d4988 & 0xffff0000;
    }
    sVar3 = 0;
    do {
        *(short *)((int)auStack_14 + (uint)*(unsigned char *)(sVar3 + 0x293 + iVar1) * 2) = sVar3;
        sVar3 = sVar3 + 1;
    } while (sVar3 < 10);
    bVar2 = 1;
    sVar3 = 0;
    do {
        uint local_uint;
        __builtin_memcpy(&local_uint, (void *)((int)auStack_14 + sVar3 * 2), 4);
        uVar4 = local_uint & 0xffff;
        unsigned short local_ushort;
        __builtin_memcpy(&local_ushort, (void *)(iVar1 + 0x1dd), 2);
        short local_short;
        __builtin_memcpy(&local_short, (void *)(uVar4 * 2 + 0x29e + iVar1), 2);
        if ((local_short == 0) &&
            ((local_ushort & DAT_00523b70[uVar4]) != 0)) {
            bVar2 = 0;
        }
        sVar3 = sVar3 + 1;
    } while (sVar3 < 10);
    return (uint)!bVar2;
}

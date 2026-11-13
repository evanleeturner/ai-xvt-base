/* SPDX-License-Identifier: MIT */
/* z_bop 0040e9e0 */
uint FUN_0040e940(byte *, uint, uint, uint);

void __cdecl FUN_0040e9e0(int param_1, uint param_2, uint param_3)
{
    uint uVar1;
    int iVar2;
    int iVar3;
    byte local_4;
    byte local_3;
    char local_2;

    iVar2 = 0;
    do {
        local_3 = (byte)(iVar2 >> 5) & 0x3f;
        local_4 = ((byte)((uint)(iVar2 >> 5) >> 6) & 0x1f) * 2;
        iVar3 = iVar2 + 1;
        local_2 = ((byte)iVar3 & 0x1f) * 2;
        uVar1 = FUN_0040e940(&local_4, 0x9a7bc0, param_2, param_3);
        *(char *)(param_1 + -1 + iVar3) = (char)uVar1;
        iVar2 = iVar3;
    } while (iVar3 < 0x10000);
    (void)local_3;
    (void)local_2;
    return;
}

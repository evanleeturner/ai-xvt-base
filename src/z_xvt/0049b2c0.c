/* SPDX-License-Identifier: MIT */
/* z_xvt 0049b2c0 */
extern unsigned char DAT_00645780[];
extern unsigned char DAT_00645900[];

void __stdcall FUN_0049b2c0(void)
{
    int iVar1;

    iVar1 = 0;
    do {
        *(unsigned int *)((int)&DAT_00645780 + iVar1) = 0;
        *(unsigned int *)((int)&DAT_00645900 + iVar1) = 0;
        iVar1 = iVar1 + 0x30;
    } while (iVar1 < 0x180);
    return;
}

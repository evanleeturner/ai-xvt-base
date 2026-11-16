/* SPDX-License-Identifier: MIT */
/* z_xvt 004f0d60 */
extern undefined4 DAT_00757870;
extern undefined4 DAT_007579b0;
extern undefined4 DAT_0081b850;
extern undefined4 _DAT_00758620;
extern undefined4 DAT_00758620;
void FUN_004cb830(int, unsigned int *, int);

undefined4 __stdcall FUN_004f0d60(void)
{
    int iVar1;
    undefined4 *puVar2;
    
    puVar2 = &DAT_00757870;
    for (iVar1 = 0x50; iVar1 != 0; iVar1 = iVar1 - 1) {
        *puVar2 = 0;
        puVar2 = puVar2 + 1;
    }
    puVar2 = &DAT_007579b0;
    for (iVar1 = 8; iVar1 != 0; iVar1 = iVar1 - 1) {
        *puVar2 = 0;
        puVar2 = puVar2 + 1;
    }
    if (DAT_0081b850 != 2) {
        _DAT_00758620 = 0x54;
        FUN_004cb830(0, (unsigned int *)&DAT_00758620, 4);
    }
    return 1;
}

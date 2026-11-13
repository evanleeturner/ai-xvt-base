/* SPDX-License-Identifier: MIT */
/* z_xvt 004fe2b0 */
extern undefined4 DAT_0081f7de;
extern undefined4 DAT_0081fa9e;
extern undefined4 __cdecl FUN_004cdc20(undefined4);

undefined4 __stdcall FUN_004fe2b0(void)
{
    undefined4 *piVar1;
    
    piVar1 = &DAT_0081f7de;
    do {
        if (*piVar1 != 0) {
            FUN_004cdc20(*piVar1);
        }
        piVar1 = piVar1 + 0x16;
    } while (piVar1 < &DAT_0081fa9e);
    return 1;
}

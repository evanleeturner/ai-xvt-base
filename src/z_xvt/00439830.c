/* SPDX-License-Identifier: MIT */
/* z_xvt 00439830 */
extern char DAT_0073ba30;
uint __cdecl FUN_00491960(char *param_1);
uint __cdecl FUN_004918c0(char *param_1);

uint __cdecl FUN_00439830(int param_1, int param_2)
{
    if (param_1 < 4 || 0x342 < param_1) {
        return 0;
    }
    if (param_2 == 0x100) {
        return FUN_00491960((char *)&DAT_0073ba30 + param_1 * 0x18);
    }
    if (param_2 != 0x500) {
        return 0;
    }
    return FUN_004918c0((char *)&DAT_0073ba30 + param_1 * 0x18);
}

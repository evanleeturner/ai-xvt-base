/* SPDX-License-Identifier: MIT */
/* z_bop 004a9300 */
extern int DAT_00a0adf0;
uint __cdecl FUN_0042dbd0(char *param_1);
uint __cdecl FUN_0042dba0(char *param_1);

uint __cdecl FUN_004a9300(int param_1, int param_2)
{
    if (param_1 < 4 || param_1 > 0x345) {
        return 0;
    }
    if (param_2 == 0x100) {
        return FUN_0042dbd0((char *)(DAT_00a0adf0 + param_1 * 24));
    }
    if (param_2 != 0x500) {
        return 0;
    }
    return FUN_0042dba0((char *)(DAT_00a0adf0 + param_1 * 24));
}

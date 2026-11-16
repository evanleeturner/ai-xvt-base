/* SPDX-License-Identifier: MIT */
/* z_bop 004a9180 */
extern unsigned char __cdecl FUN_0042d7c0(char *param_1, int param_2);
extern unsigned char __cdecl FUN_0042d940(char *param_1, int param_2);
extern unsigned char __cdecl FUN_0042dac0(char *param_1, int param_2);
extern unsigned int __cdecl FUN_0042db50(char *param_1, int param_2);
extern char DAT_00a0adf0;

unsigned int __cdecl FUN_004a9180(int param_1, int param_2, int param_3)
{
    if (param_1 < 4 || 0x345 < param_1) {
        return 0;
    }
    if (param_2 < 0x601) {
        if (param_2 == 0x600) {
            return FUN_0042d7c0(&DAT_00a0adf0 + param_1 * 0x18, param_3);
        }
        if (param_2 != 0x500) {
            return 0;
        }
        return FUN_0042db50(&DAT_00a0adf0 + param_1 * 0x18, param_3);
    }
    if (param_2 == 0x700) {
        return FUN_0042d940(&DAT_00a0adf0 + param_1 * 0x18, param_3);
    }
    if (param_2 != 0x777) {
        return 0;
    }
    return FUN_0042dac0(&DAT_00a0adf0 + param_1 * 0x18, param_3);
}

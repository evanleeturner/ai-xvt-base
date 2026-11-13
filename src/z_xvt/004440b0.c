/* SPDX-License-Identifier: MIT */
/* z_xvt 004440b0 */
extern char DAT_0072f508[];

bool __cdecl FUN_004440b0(int param_1, int param_2)
{
    if (param_1 == param_2) {
        return 0;
    }
    return DAT_0072f508[param_1 + param_2 * 0x1e5] == '\0';
}

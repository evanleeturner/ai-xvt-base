/* SPDX-License-Identifier: MIT */
/* z_xvt 004d04e0 */
int __cdecl FUN_004d3570(char *param_1);

int __cdecl FUN_004d04e0(char *param_1)
{
    if (*param_1 == '\0') {
        return 0;
    }
    return FUN_004d3570(param_1) != -1;
}

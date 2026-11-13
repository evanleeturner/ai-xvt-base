/* SPDX-License-Identifier: MIT */
/* z_xvt 004d3570 */
extern int DAT_00758d41;
extern int DAT_00758d45;
int __cdecl FUN_004d35a0(int param_1, int param_2, char *param_3);

int __cdecl FUN_004d3570(char *param_1)
{
    if (param_1 == (char *)0x0) {
        return -1;
    }
    return FUN_004d35a0(DAT_00758d41, DAT_00758d45 + -1, param_1);
}

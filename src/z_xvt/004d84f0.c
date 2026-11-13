/* SPDX-License-Identifier: MIT */
/* z_xvt 004d84f0 */
int __cdecl FUN_004d84f0(int param_1, unsigned int param_2, int param_3, int param_4)
{
    unsigned int result;

    result = param_2 * 0x10000u;
    result = result + (param_2 >> 0x10);
    result = result + (unsigned int)param_4;
    result = result + (unsigned int)param_3;
    result = result + (unsigned int)param_1;
    return (int)result;
}

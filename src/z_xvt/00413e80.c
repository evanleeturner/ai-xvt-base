/* SPDX-License-Identifier: MIT */
/* z_xvt 00413e80 */
unsigned short __cdecl FUN_00413e80(unsigned char param_1, int param_2)
{
    unsigned char *pbVar1;
    unsigned char hi;
    unsigned char lo;

    pbVar1 = (unsigned char *)((unsigned int)param_2 + (unsigned int)param_1 * 3u);
    hi = (unsigned char)((unsigned int)(*pbVar1 >> 1) << 3);
    lo = (unsigned char)(pbVar1[2] >> 1);
    return (unsigned short)(((unsigned short)((unsigned short)pbVar1[1] << 5)) |
                            (unsigned short)(((unsigned short)hi << 8) | (unsigned short)lo));
}

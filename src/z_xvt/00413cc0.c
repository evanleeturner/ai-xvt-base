/* SPDX-License-Identifier: MIT */
/* z_xvt 00413cc0 */
int __cdecl FUN_00413cc0(int param_1, int param_2, int param_3, int param_4, int param_5, int param_6)
{
    long long local_a = (long long)param_3 * (long long)param_6;
    long long local_b = (long long)param_1 * (long long)param_4;
    long long local_c = (long long)param_2 * (long long)param_5;

    unsigned int ua = ((unsigned int)local_a >> 0xf) |
                      ((unsigned int)((unsigned long long)local_a >> 0x20) << 0x11);
    unsigned int ub = ((unsigned int)local_b >> 0xf) |
                      ((unsigned int)((unsigned long long)local_b >> 0x20) << 0x11);
    unsigned int uc = ((unsigned int)local_c >> 0xf) |
                      ((unsigned int)((unsigned long long)local_c >> 0x20) << 0x11);

    return (int)(ua + (ub + uc));
}

/* SPDX-License-Identifier: MIT */
/* z_bop 0043c740 */
extern unsigned char DAT_009d9150[];
extern unsigned char DAT_009d9156[];
extern unsigned char DAT_009d8c83[];
extern unsigned char DAT_009d8c91[];

int __cdecl FUN_0043c740(int param_1, int *param_2)
{
    int idx = param_1 * 0x566;

    if (DAT_009d9150[idx] == 1) {
        return 0;
    }
    if (DAT_009d9156[idx] == 0) {
        return *param_2;
    }
    if (DAT_009d8c83[idx] == 1) {
        return *param_2;
    }
    if (DAT_009d8c91[idx] == 0) {
        return 0;
    }
    return *param_2;
}

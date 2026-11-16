/* SPDX-License-Identifier: MIT */
/* z_xvt 004ac6f0 */
extern unsigned char DAT_0070b210[];
extern unsigned char DAT_0070b216[];
extern unsigned char DAT_0070ad43[];
extern unsigned char DAT_0070ad51[];

int FUN_004ac6f0(int param_1, int *param_2) {
    int idx = param_1 * 0x566;
    if (DAT_0070b210[idx] == 1) {
        return 0;
    }
    if (DAT_0070b216[idx] == 0) {
        return *param_2;
    }
    if (DAT_0070ad43[idx] == 1) {
        return *param_2;
    }
    if (DAT_0070ad51[idx] == 0) {
        return 0;
    }
    return *param_2;
}

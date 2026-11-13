/* SPDX-License-Identifier: MIT */
/* z_xvt 004f2cb0 */
extern short DAT_0065047a;

void FUN_004f2c10(void);
void FUN_004f2e40(short param_2);

unsigned int FUN_004f2cb0(short param_1, short param_2)
{
    if ((int)DAT_0065047a - (int)param_1 == 1) {
        return 0;
    }
    if (DAT_0065047a > param_1) {
        FUN_004f2c10();
    }
    while (DAT_0065047a <= param_1) {
        DAT_0065047a++;
        FUN_004f2e40(param_2);
    }
    return (unsigned int)DAT_0065047a << 16 | 1;
}

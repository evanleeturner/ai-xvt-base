/* SPDX-License-Identifier: MIT */
/* z_xvt 004d4d00 */
extern int DAT_00759663;
extern int *DAT_0075966f;
extern int DAT_0075964b;
extern int DAT_0075964f;
extern int DAT_00759645;
extern int DAT_007595fd;
extern int _DAT_007595d9;

int __stdcall FUN_004d4d00(void)
{
    int iVar1;

    if (DAT_00759663 == 0) {
        return 0;
    }
    if (DAT_0075966f == 0) {
        return 0;
    }
    DAT_0075964b = DAT_0075964f;
    if (DAT_00759645 != 0) {
        return DAT_007595fd;
    }
    _DAT_007595d9 = 0x6c;
    do {
        while (1) {
            iVar1 = ((int (__stdcall *)(void *, int, int *, int, int))(*(void **)DAT_0075966f + 25))(DAT_0075966f, 0, &_DAT_007595d9, 0, 0);
            if (iVar1 != -0x7789fe3e) {
                break;
            }
            ((void (__stdcall *)(void *))(*(void **)DAT_0075966f + 27))(DAT_0075966f);
        }
    } while (iVar1 == -0x7789fde4 || iVar1 == -0x7789fe52 || iVar1 == -0x7789fe48);
    DAT_00759645 = 1;
    return DAT_007595fd;
}

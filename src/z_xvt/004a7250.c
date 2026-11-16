/* SPDX-License-Identifier: MIT */
/* z_xvt 004a7250 */
extern void **DAT_00658020;
extern void **DAT_00657c00;

void __stdcall FUN_004a7250(void)
{
    int local_64[20];
    int iVar1;
    int iVar2;

    local_64[0] = 100;
    while (1) {
        iVar1 = (*(int (__stdcall **)(void *, int, int, int, int, void *))((char *)*DAT_00658020 + 0x14))(DAT_00658020, 0, 0, 0, 0x400, local_64);
        if (iVar1 == 0) {
            return;
        }
        if (iVar1 == -0x7789FE3E) {
            iVar2 = (*(int (__stdcall **)(void *))((char *)*DAT_00657c00 + 0x6C))(DAT_00657c00);
            if (iVar2 != 0) {
                break;
            }
        }
        if (iVar1 != -0x7789FDE4) {
            return;
        }
    }
    return;
}

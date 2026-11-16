/* SPDX-License-Identifier: MIT */
/* z_xvt 004a73a0 */
extern void **DAT_00657c00;

unsigned int __stdcall FUN_004a73a0(void)
{
    unsigned int local_6c[27];
    unsigned int uStack_48 = 0;
    unsigned int *puVar1;
    int iVar2;

    puVar1 = local_6c;
    for (iVar2 = 0x1b; iVar2 != 0; iVar2 = iVar2 - 1) {
        *puVar1 = 0;
        puVar1 = puVar1 + 1;
    }
    local_6c[0] = 0x6c;
    do {
        iVar2 = ((int (__stdcall *)(void *, int, unsigned int *, int, int))(*DAT_00657c00 + 100))(DAT_00657c00, 0, local_6c, 0, 0);
        if (iVar2 == 0) {
            return uStack_48;
        }
    } while (iVar2 == -0x7789fde4);
    return 0;
}

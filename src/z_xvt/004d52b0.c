/* SPDX-License-Identifier: MIT */
/* z_xvt 004d52b0 */
extern int DAT_00759663;
extern int *DAT_00759667;
extern int DAT_0075964b;
extern int DAT_00759653;
extern int DAT_00758830;

int __stdcall FUN_004d52b0(void)
{
    int iVar1;
    int *puVar2;
    int local_6c[27];
    int uStack_48 = 0;
    
    if (DAT_00759663 == 0) {
        return 0;
    }
    if (DAT_00759667 == 0) {
        return 0;
    }
    puVar2 = local_6c;
    for (iVar1 = 0x1b; iVar1 != 0; iVar1 = iVar1 + -1) {
        *puVar2 = 0;
        puVar2 = puVar2 + 1;
    }
    local_6c[0] = 0x6c;
    do {
        while (1) {
            iVar1 = ((int (__stdcall *)(int *, int, int *, int, int))((void *)*DAT_00759667 + 0x64))(DAT_00759667, 0, local_6c, 0, 0);
            if (iVar1 == 0) {
                DAT_0075964b = DAT_00759653;
                DAT_00758830 = uStack_48;
                return 1;
            }
            if (iVar1 != -0x7789fe3e) break;
            ((void (__stdcall *)(int *))((void *)*DAT_00759667 + 0x6c))(DAT_00759667);
        }
    } while (iVar1 == -0x7789fde4);
    return 0;
}

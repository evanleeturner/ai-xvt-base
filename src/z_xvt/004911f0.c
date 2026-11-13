/* SPDX-License-Identifier: MIT */
/* z_xvt 004911f0 */
extern int DAT_00658654;
extern int *DAT_00658658;

int __stdcall FUN_004911f0(void)
{
    int iVar1;
    int *piVar2;
    char local_4[4];

    if (DAT_00658654 == 0) {
        return 0;
    }
    piVar2 = DAT_00658658;
    iVar1 = ((int (__stdcall **)(void *, void *))(*(void **)DAT_00658658))[0x18 / 4](DAT_00658658, local_4);
    if (iVar1 != 0) {
        return 0;
    }
    return ((int)piVar2 * 0x7f) / 2000 + 0x7f;
}

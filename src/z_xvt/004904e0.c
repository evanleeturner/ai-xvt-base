/* SPDX-License-Identifier: MIT */
/* z_xvt 004904e0 */
extern int *DAT_0052a59c;

int * __stdcall FUN_004904e0(void)
{
    int *piVar1;
    char local_8[4];
    char local_4[4];
    
    if (DAT_0052a59c != (int *)0x0) {
        piVar1 = DAT_0052a59c;
        (*(void (__stdcall **)(int *, void *, void *))(*DAT_0052a59c + 0x10))(DAT_0052a59c, local_8, local_4);
        return piVar1;
    }
    return (int *)0x0;
}

/* SPDX-License-Identifier: MIT */
/* z_xvt 004b5410 */
extern void *DAT_0052f1f4;
extern void *DAT_0052f1cc;
extern void __stdcall FUN_0045bbe0(void);

void __stdcall FUN_004b5410(void)
{
    if (DAT_0052f1f4 != NULL) {
        void **vftable = *(void ***)DAT_0052f1f4;
        ((void (__stdcall *)(void *))vftable[2])(DAT_0052f1f4);
    }
    DAT_0052f1f4 = NULL;
    FUN_0045bbe0();
    DAT_0052f1cc = 0;
    return;
}

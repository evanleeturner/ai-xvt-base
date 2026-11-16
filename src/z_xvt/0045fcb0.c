/* SPDX-License-Identifier: MIT */
/* z_xvt 0045fcb0 */
extern void **DAT_006a8a5c;
extern void **DAT_00658020;

void __stdcall FUN_0045fcb0(void)
{
    if (DAT_006a8a5c != (void **)0x0) {
        (*(void (__stdcall **)(void *, int, void *))((void **)((char *)*DAT_00658020 + 0x20)))(DAT_00658020, 0, DAT_006a8a5c);
        (*(void (__stdcall **)(void *))((void **)((char *)*DAT_006a8a5c + 0x8)))(DAT_006a8a5c);
        DAT_006a8a5c = (void **)0x0;
    }
    return;
}

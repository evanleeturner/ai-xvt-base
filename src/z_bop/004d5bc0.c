/* SPDX-License-Identifier: MIT */
/* z_bop 004d5bc0 */
extern void **DAT_00aa7b2e;

void __stdcall FUN_004d5bc0(void)
{
    if (DAT_00aa7b2e != (void *)0x0) {
        (**(void (__stdcall **)(void *))(*DAT_00aa7b2e + 0x28))(DAT_00aa7b2e);
    }
    return;
}

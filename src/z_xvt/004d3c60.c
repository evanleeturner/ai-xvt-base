/* SPDX-License-Identifier: MIT */
/* z_xvt 004d3c60 */
extern void **DAT_0075966b;
extern void **DAT_00759667;
extern void **DAT_0075966f;
extern int DAT_0081b6e4;
extern int DAT_0081b6f0;

void __stdcall FUN_004d3c60(void)
{
    int iVar1;
    
    iVar1 = ((int (__stdcall *)(void *))(*(void **)(*DAT_0075966b + 0x6c)))((void *)DAT_0075966b);
    if (iVar1 == 0) {
        if ((DAT_0081b6e4 == 0) && (DAT_0081b6f0 == 0)) {
            ((void (__stdcall *)(void *))(*(void **)(*DAT_00759667 + 0x6c)))((void *)DAT_00759667);
            return;
        }
        ((void (__stdcall *)(void *))(*(void **)(*DAT_0075966f + 0x6c)))((void *)DAT_0075966f);
    }
    return;
}

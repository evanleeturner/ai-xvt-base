/* SPDX-License-Identifier: MIT */
/* z_xvt 004b5bc2 */
extern int DAT_0052f204;
extern int _DAT_0052f208;
extern int DAT_0052f20c;
extern void *DAT_0052f1f8;
extern int DAT_0064ee2c;
extern int DAT_0064ee30;
extern int DAT_0064ee40;
extern int DAT_0064ee60;

extern int __cdecl FUN_004b51f0(int param_1, int param_2, int param_3);
extern void __stdcall FUN_0045bbe0(void);

bool __stdcall FUN_004b5bc2(void)
{
    int iVar1;

    DAT_0052f204 = 0;
    _DAT_0052f208 = 0;
    DAT_0052f20c = 1;
    iVar1 = ((int (__stdcall *)(void *, int *))((void **)(*(void **)DAT_0052f1f8))[4])(DAT_0052f1f8, &DAT_0064ee30);

    if (iVar1 == 0) {
        DAT_0064ee2c = DAT_0064ee40;
        DAT_0064ee60 = DAT_0064ee40;
    }
    else {
        FUN_004b51f0(iVar1, 0x52f210, 0x79);
        FUN_0045bbe0();
    }
    return iVar1 == 0;
}

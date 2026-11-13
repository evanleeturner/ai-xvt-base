/* SPDX-License-Identifier: MIT */
/* z_bop 004b1c90 */
extern int DAT_00528c84;
extern int DAT_00528ca0;
extern int DAT_00528ca4;
extern int DAT_00528ca8;
extern void *DAT_00528c98;
extern int DAT_00528cc0;
extern int DAT_00664754;
extern int DAT_00664758;
extern int DAT_00664768;
extern int DAT_00664c80;

extern char * __cdecl FUN_004b1600(int param_1, int *param_2, int param_3);
extern void __stdcall FUN_004079e0(void);

int __stdcall FUN_004b1c90(void)
{
    int iVar1;
    
    DAT_00528c84 = DAT_00528c84 + 1;
    DAT_00528ca0 = 0;
    DAT_00528ca4 = 0;
    DAT_00528ca8 = 1;
    iVar1 = ((int (__stdcall *)(void *, void *))(*(void **)(*(void **)DAT_00528c98 + 0x10)))(DAT_00528c98, &DAT_00664758);
    if (iVar1 != 0) {
        FUN_004b1600(iVar1, &DAT_00528cc0, 0x79);
        FUN_004079e0();
        return 0;
    }
    DAT_00664754 = DAT_00664768;
    DAT_00664c80 = DAT_00664768;
    return 1;
}

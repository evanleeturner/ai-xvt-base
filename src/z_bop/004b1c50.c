/* SPDX-License-Identifier: MIT */
/* z_bop 004b1c50 */
extern void **DAT_00664548;
extern int DAT_00528cc0;

extern char * __cdecl FUN_004b1600(int param_1, int *param_2, int param_3);
extern void __stdcall FUN_004079e0(void);

void __stdcall FUN_004b1c50(void)
{
    int iVar1;
    
    iVar1 = (*(int (__stdcall **)(void *))((char *)*DAT_00664548 + 0x50))(DAT_00664548);
    if (iVar1 != 0) {
        FUN_004b1600(iVar1, &DAT_00528cc0, 0x79);
        FUN_004079e0();
    }
    return;
}

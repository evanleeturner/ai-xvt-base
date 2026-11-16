/* SPDX-License-Identifier: MIT */
/* z_xvt 004d8a00 */
extern int DAT_00532a34;
extern int _DAT_00758820;
extern int DAT_00758618;
extern int DAT_0064f658;
extern int *DAT_00757cc4;
extern int DAT_00758824;

void __stdcall FUN_004ca200(void);
void __stdcall FUN_004d8520(void);
void __cdecl FUN_004d8a70(int);
void __stdcall FUN_004d8900(void);

int __stdcall FUN_004d8a00(void)
{
    int iVar1;
    int iVar2;
    int *puVar3;
    
    iVar2 = 0;
    DAT_00532a34 = 0xFFFFFFFF;
    _DAT_00758820 = 0;
    DAT_00758618 = 0xFFFFFFFF;
    puVar3 = &DAT_0064f658;
    for (iVar1 = 8; iVar1 != 0; iVar1--) {
        *puVar3 = 0;
        puVar3++;
    }
    puVar3 = DAT_00757cc4;
    for (iVar1 = 0x100; iVar1 != 0; iVar1--) {
        *puVar3 = 0;
        puVar3++;
    }
    FUN_004ca200();
    FUN_004d8520();
    if (DAT_00758824 > 0) {
        do {
            FUN_004d8a70(iVar2);
            iVar2++;
        } while (iVar2 < DAT_00758824);
    }
    FUN_004ca200();
    FUN_004d8900();
    return DAT_00758824;
}

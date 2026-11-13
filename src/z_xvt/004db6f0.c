/* SPDX-License-Identifier: MIT */
/* z_xvt 004db6f0 */
void __stdcall FUN_004d4d90(void);
void __cdecl FUN_004db790(unsigned int param_1, int *param_2);
int __cdecl FUN_004d58d0(void);
void __cdecl FUN_004dbea0(void);
void __cdecl FUN_004dbb40(void);
unsigned int __stdcall FUN_004d4d00(void);
void __cdecl _exit(int code);

extern unsigned int DAT_007806f2;
extern unsigned int DAT_00758f2e;
extern unsigned int DAT_00758f4e;
extern unsigned int DAT_0075dc46;
extern unsigned char DAT_00758ee0;
extern unsigned char DAT_00758ee1;
extern unsigned int DAT_00758830;

unsigned int __cdecl FUN_004db6f0(unsigned int param_1, int *param_2)
{
    int iVar1;
    unsigned int *puVar2;

    FUN_004d4d90();
    FUN_004db790(param_1, param_2);
    DAT_007806f2 = 0;
    puVar2 = &DAT_00758f2e;
    for (iVar1 = 8; iVar1 != 0; iVar1 = iVar1 - 1) {
        *puVar2 = 0;
        puVar2 = puVar2 + 1;
    }
    puVar2 = &DAT_00758f4e;
    for (iVar1 = 8; iVar1 != 0; iVar1 = iVar1 - 1) {
        *puVar2 = 0;
        puVar2 = puVar2 + 1;
    }
    if (DAT_0075dc46 != 0) {
        DAT_0075dc46 = DAT_0075dc46 - 1;
    }
    DAT_00758ee0 = 0;
    DAT_00758ee1 = 0;
    do {
        iVar1 = FUN_004d58d0();
        if (iVar1 == 1) {
            FUN_004dbea0();
            FUN_004dbb40();
            DAT_00758830 = FUN_004d4d00();
            DAT_00758ee0 = 0;
            DAT_00758ee1 = 0;
            return 1;
        }
    } while (iVar1 != 2);
    _exit(0);
    return 0;
}

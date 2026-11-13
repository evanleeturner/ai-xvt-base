/* SPDX-License-Identifier: MIT */
/* z_xvt 004d0e60 */
extern int DAT_00758d41;
int __cdecl FUN_004d3570(char *param_1);
unsigned int __cdecl FUN_004d0ea0(void *param_1, int param_2, int param_3);

unsigned int __cdecl FUN_004d0e60(char *param_1, int param_2, int param_3)
{
    int iVar1;
    unsigned int uVar2;
    
    iVar1 = FUN_004d3570(param_1);
    if (iVar1 == -1) {
        return 0;
    }
    uVar2 = FUN_004d0ea0(*(void **)(DAT_00758d41 + 0x40 + iVar1 * 0x44), param_2, param_3);
    return uVar2;
}

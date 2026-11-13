/* SPDX-License-Identifier: MIT */
/* z_bop 0040e5e0 */
void __cdecl FUN_00449410(unsigned int, unsigned int, unsigned int, unsigned int);

extern unsigned char DAT_009a7bc0[];
extern unsigned char DAT_009a7bc1[];
extern unsigned char DAT_009a7bc2[];
extern unsigned char DAT_009a7a30;

void __cdecl FUN_0040e5e0(unsigned char *param_1, unsigned short param_2, unsigned int param_3)
{
    unsigned int uVar2;
    unsigned int uVar3;
    
    uVar2 = (unsigned int)param_2;
    uVar3 = uVar2;
    while (uVar3 < uVar2 + (param_3 & 0xffff)) {
        DAT_009a7bc0[(unsigned int)param_2 * 3] = *param_1;
        DAT_009a7bc1[(unsigned int)param_2 * 3] = param_1[1];
        DAT_009a7bc2[(unsigned int)param_2 * 3] = param_1[2];
        uVar3 = (unsigned int)(unsigned short)(param_2 + 1);
        param_1 = param_1 + 3;
        param_2 = param_2 + 1;
    }
    if (DAT_009a7a30 == '\x02') {
        FUN_00449410(0x9a7bc0, 0xa00530, uVar2, param_3 & 0xffff);
    }
    return;
}

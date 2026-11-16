/* SPDX-License-Identifier: MIT */
/* z_bop 004a0030 */
extern void FUN_004a44d0(short param_1, int param_2);
extern unsigned int FUN_004a4d70(unsigned int param_1, unsigned short param_2);
extern unsigned short DAT_009a73b0;

unsigned int __stdcall caseD_8(void)
{
    unsigned int uVar1;
    
    FUN_004a44d0(0x8000, 1);
    uVar1 = FUN_004a4d70((unsigned int)DAT_009a73b0, 0xffff);
    return uVar1 & 0xffff0000;
}

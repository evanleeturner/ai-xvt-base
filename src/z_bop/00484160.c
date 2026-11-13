/* SPDX-License-Identifier: MIT */
/* z_bop 00484160 */
extern unsigned short _DAT_009e8f50;
extern unsigned char DAT_009ec476;
extern unsigned short DAT_005233fc;
extern unsigned char DAT_00a080fc;
extern unsigned char DAT_009a73f4;

extern unsigned short DAT_00527348[];
extern unsigned short DAT_00527358[];
extern unsigned char DAT_00527360[];
extern unsigned char DAT_00527368[];

void __cdecl FUN_00484160(unsigned int param_1)
{
    unsigned int iVar1;
    
    iVar1 = (param_1 & 0xffff) * 2;
    _DAT_009e8f50 = DAT_00527348[iVar1 / 2];
    DAT_009ec476 = 1;
    DAT_005233fc = DAT_00527358[iVar1 / 2];
    DAT_00a080fc = DAT_00527360[iVar1];
    DAT_009a73f4 = DAT_00527368[iVar1];
    return;
}

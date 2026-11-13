/* SPDX-License-Identifier: MIT */
/* z_xvt 004f33c0 */
extern short DAT_00650424;
extern short DAT_00650426;
extern short DAT_0065042c;
extern short DAT_0065042e;

void __cdecl FUN_004f33c0(int *param_1, short param_2, short param_3, short *param_4, short *param_5)
{
    short sVar1;
    int iVar2;

    iVar2 = (int)((unsigned int)((int)param_2 - (int)DAT_00650424) * (unsigned int)(int)DAT_0065042c);
    sVar1 = (short)((int)((unsigned int)iVar2 + (unsigned int)((iVar2 >> 0x1f) & 0xff)) >> 8);
    *param_4 = sVar1;
    *param_4 = (short)((unsigned int)(unsigned short)(short)((int)((unsigned int)param_1[2] - (unsigned int)param_1[0]) >> 1)
                       + (unsigned int)(unsigned short)sVar1
                       + (unsigned int)(unsigned short)(short)param_1[0]);

    iVar2 = (int)((unsigned int)((int)param_3 - (int)DAT_00650426) * (unsigned int)(int)DAT_0065042e);
    sVar1 = (short)((int)((unsigned int)iVar2 + (unsigned int)((iVar2 >> 0x1f) & 0xff)) >> 8);
    *param_5 = sVar1;
    *param_5 = (short)((unsigned int)(unsigned short)(short)((int)((unsigned int)param_1[3] - (unsigned int)param_1[1]) >> 1)
                       + (unsigned int)(unsigned short)(short)param_1[1]
                       + (unsigned int)(unsigned short)sVar1);
    return;
}

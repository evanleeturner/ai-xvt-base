/* SPDX-License-Identifier: MIT */
/* z_xvt 004a17b0 */
extern unsigned char DAT_0070aee5[];
extern unsigned char DAT_0070aee6[];
extern unsigned char DAT_0070aee7[];
extern unsigned char DAT_0070aee8[];
extern unsigned char DAT_0070aee9[];
extern unsigned char DAT_0070aef3[];
extern unsigned char DAT_0071f2d8;
extern unsigned char DAT_0071f2dc;
extern unsigned char DAT_0071f2dd;
extern int DAT_007092e8[];

void __cdecl FUN_004a17b0(unsigned short param_1, unsigned int param_2, int param_3)
{
    int iVar1;
    int iVar2;
    int iVar3;
    int local_4;

    iVar2 = (param_2 & 0xFFFF) * 0x566;
    iVar1 = 8;
    do {
        if (DAT_0070aee5[iVar2] == 0x02 &&
            DAT_0070aee6[iVar2] == param_1 &&
            (DAT_0070aee7[iVar2] == 0x12 || (DAT_0070aee7[iVar2] == 0x13 && param_3 == 1))) {
            
            if ((unsigned int)(unsigned char)DAT_0070aef3[iVar2] * 5 == 0 ||
                ((unsigned int)(unsigned char)DAT_0071f2d8 * 0x3C + (unsigned int)(unsigned char)DAT_0071f2dc) * 0x3C + (unsigned int)(unsigned char)DAT_0071f2dd <=
                (unsigned int)(unsigned char)DAT_0070aef3[iVar2] * 5) {
                local_4 = (char)DAT_0070aee8[iVar2];
            }
            
            iVar3 = 0;
            local_4 = local_4 * 0xFA;
            do {
                if (DAT_0070aee9[iVar3 + iVar2] != 0) {
                    DAT_007092e8[iVar3] = DAT_007092e8[iVar3] + local_4;
                }
                iVar3++;
            } while (iVar3 < 10);
        }
        iVar2 += 0x4E;
        iVar1--;
    } while (iVar1 != 0);
}

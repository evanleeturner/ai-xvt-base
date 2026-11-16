/* SPDX-License-Identifier: MIT */
/* z_xvt 00401960 */
extern short DAT_006da9f8;
extern short DAT_007092ca;
extern short DAT_007092ce;
extern unsigned char DAT_0070ac34;
extern unsigned char DAT_0070ac35;
extern int DAT_006d4980;
extern int DAT_006d4988;
extern int DAT_00732e54;
extern short DAT_00732910;
extern int DAT_0071ecf4;

int __cdecl FUN_004490f0(unsigned int param_1);
int __cdecl FUN_00449790(unsigned int param_1, unsigned int param_2);
void __cdecl FUN_004a7d20(int param_1);

void __cdecl FUN_00401960(unsigned short param_1)
{
    int iVar1;
    char cVar2;
    int iVar3;
    unsigned char bVar4;
    int iVar5;
    unsigned int uVar6;
    unsigned short local_6;
    int iTmp;

    DAT_006da9f8 = 1;
    DAT_007092ca = 0;
    DAT_007092ce = 0xc;
    bVar4 = (unsigned char)param_1;
    if (param_1 < 9) {
        DAT_0070ac34 = (unsigned char)((int)(10u - (unsigned int)param_1) / 2);
        DAT_0070ac35 = (unsigned char)((bVar4 & 1) * 0x1e);
    }
    else {
        DAT_0070ac34 = 0;
        DAT_0070ac35 = (unsigned char)((0x14 - bVar4) * 5);
    }
    local_6 = 0;
    if (0 < DAT_006d4980) {
        do {
            iVar3 = (int)((unsigned int)DAT_006d4988 + (unsigned int)local_6 * 0x23u);
            if ((*(char *)(iVar3 + 3) != '\0') && (*(char *)(iVar3 + 2) == '\x0e')) {
                /* unaligned reads at +0x1f and +0xa7 */
                __builtin_memcpy(&iTmp, (unsigned char *)iVar3 + 0x1f, 4);
                __builtin_memcpy(&DAT_00732e54, (unsigned char *)iTmp + 0xa7, 4);
                iVar5 = 0;
                uVar6 = 0;
                iVar3 = FUN_004490f0((unsigned int)*(unsigned char *)(iVar3 + 3));
                if (0 < iVar3) {
                    do {
                        iVar5 = iVar5 + 1;
                        iVar1 = FUN_00449790(
                            (unsigned int)*(unsigned char *)((unsigned int)DAT_006d4988 + 3u + (unsigned int)local_6 * 0x23u),
                            uVar6);
                        if (local_6 == 1) {
                            if (iVar1 == 1) {
                                ((unsigned char *)DAT_00732e54)[0x319 + iVar5] = 0xff;
                            }
                            else {
                                ((unsigned char *)DAT_00732e54)[0x319 + iVar5] = 0;
                            }
                            ((unsigned char *)DAT_00732e54)[0x2c9 + iVar5] = 2;
                        }
                        else {
                            switch (iVar1) {
                            case 1:
                                ((unsigned char *)DAT_00732e54)[0x2c9 + iVar5] = 2;
                                break;
                            case 5:
                                ((unsigned char *)DAT_00732e54)[0x319 + iVar5] = (unsigned char)(bVar4 * 2);
                                ((unsigned char *)DAT_00732e54)[0x2c9 + iVar5] = 0;
                                break;
                            case 0x11:
                                if (param_1 < 2) {
                                    ((unsigned char *)DAT_00732e54)[0x2c9 + iVar5] = 2;
                                    ((unsigned char *)DAT_00732e54)[0x319 + iVar5] = 0;
                                }
                                else {
                                    cVar2 = '\x03';
LAB_00401b71:
                                    ((unsigned char *)DAT_00732e54)[0x319 + iVar5] = (unsigned char)(bVar4 * cVar2);
                                    ((unsigned char *)DAT_00732e54)[0x2c9 + iVar5] = 0;
                                    ((unsigned char *)DAT_00732e54)[0x2f1 + iVar5] = 0;
                                }
                                break;
                            case 0x12:
                                if (param_1 < 5) {
                                    ((unsigned char *)DAT_00732e54)[0x2c9 + iVar5] = 2;
                                    ((unsigned char *)DAT_00732e54)[0x319 + iVar5] = 0;
                                }
                                else {
                                    ((unsigned char *)DAT_00732e54)[0x2c9 + iVar5] = 0;
                                    ((unsigned char *)DAT_00732e54)[0x2f1 + iVar5] = 0;
                                    ((unsigned char *)DAT_00732e54)[0x319 + iVar5] = 0xff;
                                }
                                break;
                            case 0x13:
                                if (2 < param_1) {
                                    cVar2 = '\x18';
                                    goto LAB_00401b71;
                                }
                                ((unsigned char *)DAT_00732e54)[0x2c9 + iVar5] = 2;
                                ((unsigned char *)DAT_00732e54)[0x319 + iVar5] = 0;
                            }
                        }
                        uVar6 = (unsigned int)(unsigned short)((short)uVar6 + 1);
                    } while ((int)uVar6 < iVar3);
                }
            }
            local_6 = local_6 + 1;
        } while ((int)(unsigned int)local_6 < DAT_006d4980);
    }
    if (DAT_00732910 == 0) {
        FUN_004a7d20(DAT_0071ecf4);
    }
    return;
}

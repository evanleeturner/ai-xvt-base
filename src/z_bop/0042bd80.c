/* SPDX-License-Identifier: MIT */
/* z_bop 0042bd80 */
extern unsigned short DAT_009a8078;
extern unsigned short DAT_009d694a;
extern unsigned short DAT_009d694e;
extern unsigned char DAT_009d8b74[2];
extern int DAT_009a1fe0;
extern int DAT_009a1fe8;
extern int DAT_00a08104;
extern unsigned int DAT_00a00870[];
extern unsigned int DAT_00a00874[];
extern unsigned short DAT_00a00860;
extern int DAT_009ecc34;

int __cdecl FUN_004adc40(unsigned int param_1);
int __cdecl FUN_004ae2a0(unsigned int param_1, unsigned int param_2);
void __cdecl FUN_00438a30(int param_1);

void __cdecl FUN_0042bd80(unsigned short param_1)
{
    unsigned char bVar1;
    int iVar2;
    int iVar3;
    int iVar4;
    unsigned int uVar4;
    int iVar5;
    char cVar6;
    unsigned int uVar7;
    unsigned char bVar8;
    unsigned int uVar9;
    unsigned short local_6;
    int iCount;
    int iTmp;

    DAT_009a8078 = 1;
    DAT_009d694a = 0;
    DAT_009d694e = 0xc;
    bVar8 = (unsigned char)param_1;
    if (param_1 < 9) {
        DAT_009d8b74[0] = (unsigned char)((int)(10u - (unsigned int)param_1) / 2);
        DAT_009d8b74[1] = (unsigned char)((bVar8 & 1) * 0x1e);
    }
    else {
        DAT_009d8b74[0] = 0;
        DAT_009d8b74[1] = (unsigned char)((unsigned char)(0x14 - bVar8) * 5);
    }
    local_6 = 0;
    if (0 < DAT_009a1fe0) {
        do {
            iVar2 = (unsigned int)local_6 * 0x23;
            iVar4 = DAT_009a1fe8 + iVar2;
            if ((*(char *)(DAT_009a1fe8 + 3 + iVar2) != '\0') && (*(char *)(iVar4 + 2) == '\x0e')) {
                __builtin_memcpy(&iTmp, (void *)(iVar4 + 0x1f), 4);
                __builtin_memcpy(&DAT_00a08104, (void *)(iTmp + 0xa9), 4);
                bVar1 = *(unsigned char *)(iVar4 + 3);
                if (bVar1 < 0x49) {
                    iCount = (int)DAT_00a00870[(unsigned int)bVar1 * 0x65];
                }
                else {
                    iCount = FUN_004adc40((unsigned int)bVar1);
                }
                uVar9 = 0;
                iVar3 = 0;
                if (0 < iCount) {
                    do {
                        bVar1 = *(unsigned char *)(DAT_009a1fe8 + 3 + iVar2);
                        uVar4 = (unsigned int)bVar1;
                        if (bVar1 < 0x49) {
                            uVar7 = uVar9;
                            if ((int)DAT_00a00870[uVar4 * 0x65] <= (int)uVar9) {
                                uVar7 = DAT_00a00870[uVar4 * 0x65] - 1;
                            }
                            iVar5 = (int)DAT_00a00874[uVar4 * 0x65 + uVar7];
                        }
                        else {
                            iVar5 = FUN_004ae2a0(uVar4, uVar9);
                        }
                        if (local_6 == 1) {
                            if (iVar5 == 1) {
                                *(unsigned char *)(iVar3 + 0x33c + DAT_00a08104) = 0xff;
                            }
                            else {
                                *(unsigned char *)(iVar3 + 0x33c + DAT_00a08104) = 0;
                            }
                            *(unsigned char *)(iVar3 + 0x2d8 + DAT_00a08104) = 2;
                        }
                        else {
                            switch (iVar5) {
                            case 1:
                                *(unsigned char *)(iVar3 + 0x2d8 + DAT_00a08104) = 2;
                                break;
                            case 5:
                                *(unsigned char *)(iVar3 + 0x33c + DAT_00a08104) = (unsigned char)(bVar8 * 2);
                                *(unsigned char *)(iVar3 + 0x2d8 + DAT_00a08104) = 0;
                                break;
                            case 0x11:
                                if (param_1 < 2) {
                                    *(unsigned char *)(iVar3 + 0x2d8 + DAT_00a08104) = 2;
                                    *(unsigned char *)(iVar3 + 0x33c + DAT_00a08104) = 0;
                                }
                                else {
                                    cVar6 = '\x03';
                                    goto LAB_0042bfea;
                                }
                                break;
                            case 0x12:
                                if (param_1 < 5) {
                                    *(unsigned char *)(iVar3 + 0x2d8 + DAT_00a08104) = 2;
                                    *(unsigned char *)(iVar3 + 0x33c + DAT_00a08104) = 0;
                                }
                                else {
                                    *(unsigned char *)(iVar3 + 0x2d8 + DAT_00a08104) = 0;
                                    *(unsigned char *)(iVar3 + 0x30a + DAT_00a08104) = 0;
                                    *(unsigned char *)(iVar3 + 0x33c + DAT_00a08104) = 0xff;
                                }
                                break;
                            case 0x13:
                                if (2 < param_1) {
                                    cVar6 = '\x18';
                                    goto LAB_0042bfea;
                                }
                                *(unsigned char *)(iVar3 + 0x2d8 + DAT_00a08104) = 2;
                                *(unsigned char *)(iVar3 + 0x33c + DAT_00a08104) = 0;
                                break;
                            default:
                                break;
                            LAB_0042bfea:
                                *(unsigned char *)(iVar3 + 0x33c + DAT_00a08104) = (unsigned char)(bVar8 * cVar6);
                                *(unsigned char *)(iVar3 + 0x2d8 + DAT_00a08104) = 0;
                                *(unsigned char *)(iVar3 + 0x30a + DAT_00a08104) = 0;
                                break;
                            }
                        }
                        uVar9 = (unsigned int)(unsigned short)((short)uVar9 + 1);
                        iVar3 = iVar3 + 1;
                    } while ((int)uVar9 < iCount);
                }
            }
            local_6 = local_6 + 1;
        } while ((int)(unsigned int)local_6 < DAT_009a1fe0);
    }
    if (DAT_00a00860 == 0) {
        FUN_00438a30(DAT_009ecc34);
    }
    return;
}

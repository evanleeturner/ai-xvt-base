/* SPDX-License-Identifier: MIT */
/* z_bop 004f7590 */
extern short DAT_0066925c;
extern short DAT_0066925a;
extern short DAT_00669262[];
extern short DAT_0052cf10[];
extern short DAT_006696e2;
extern short _DAT_00669714;
extern short _DAT_00669776;
extern short _DAT_006696d6;
extern short _DAT_006696d8;
extern short DAT_00669778;
extern short DAT_006696da[];
extern short DAT_006696de[];
extern short DAT_00669204;
extern short DAT_00669206;
extern short DAT_00669208;
extern short DAT_0066920a;
extern short DAT_0066920c;
extern short DAT_0066920e;
extern short DAT_00669210;
extern short DAT_00669212;
extern short DAT_006696e4[];
extern short DAT_006696f4[];
extern short DAT_00669704[];
extern short DAT_00669716[];
extern short DAT_00669726[];
extern short DAT_00669736[];
extern short DAT_00669746[];
extern short DAT_00669756[];
extern short DAT_00669766[];
extern unsigned char DAT_00a91d17[];
extern char DAT_00bb73a7;
extern char *DAT_00669582[];
extern char s_sfxText_0052cfe4[];
extern char s_sfxTarget1_0052cfec[];
extern char s_sfxTarget2_0052cff8[];

void __cdecl FUN_004de5a0(char *param_1, int param_2, int param_3, int param_4);

void __cdecl FUN_004f7590(short param_1)
{
    char cVar1;
    short sVar2;
    short sVar3;
    unsigned char bVar4;
    short sVar5;
    int iVar6;
    unsigned int uVar7;
    unsigned int uVar8;
    short sVar9;
    short sVar10;
    int iVar11;
    char *pcVar12;
    char *pcVar13;
    short local_3e;
    short local_38[8];
    char local_28[40];
    unsigned int uTmp;

    local_3e = DAT_0066925c;
    DAT_006696e2 = 0;
    _DAT_00669714 = 0;
    _DAT_00669776 = 0;
    _DAT_006696d6 = 0;
    _DAT_006696d8 = 0;
    DAT_00669778 = 0;
    sVar9 = DAT_0066925c;
    if (DAT_00669262[DAT_0066925c] <= DAT_0066925a) {
        do {
            local_3e = sVar9;
            sVar2 = DAT_00669262[local_3e];
            sVar9 = (short)(local_3e + 2);
            sVar10 = 0;
            sVar5 = DAT_00669262[(short)(local_3e + 1)];
            sVar3 = DAT_0052cf10[sVar5];
            if (0 < sVar3) {
                do {
                    iVar6 = (int)sVar9;
                    iVar11 = (int)sVar10;
                    sVar9 = (short)(sVar9 + 1);
                    sVar10 = (short)(sVar10 + 1);
                    local_38[iVar11] = DAT_00669262[iVar6];
                } while (sVar10 < sVar3);
            }
            if (DAT_0066925a == sVar2) {
                switch ((int)sVar5) {
                case 1:
                    DAT_00669778 = 1;
                    break;
                case 3:
                    sVar5 = 0;
                    do {
                        iVar6 = (int)sVar5;
                        sVar5 = (short)(sVar5 + 1);
                        DAT_006696da[iVar6] = 0;
                    } while (sVar5 < 2);
                    DAT_006696e2 = 1;
                    break;
                case 4:
                case 5:
                    DAT_006696da[(short)(sVar5 + -4)] = 1;
                    DAT_006696de[(short)(sVar5 + -4)] = local_38[0];
                    break;
                case 6:
                    if ((sVar2 == 0) || (param_1 != 0)) {
                        DAT_00669204 = local_38[0];
                        DAT_00669206 = local_38[1];
                    }
                    DAT_0066920a = local_38[1];
                    DAT_00669208 = local_38[0];
                    _DAT_006696d6 = 1;
                    break;
                case 7:
                    if ((sVar2 == 0) || (param_1 != 0)) {
                        DAT_0066920c = local_38[0];
                        DAT_0066920e = local_38[1];
                    }
                    DAT_00669212 = local_38[1];
                    DAT_00669210 = local_38[0];
                    _DAT_006696d8 = 1;
                    break;
                case 8:
                    sVar5 = 0;
                    do {
                        iVar6 = (int)sVar5;
                        sVar5 = (short)(sVar5 + 1);
                        DAT_006696e4[iVar6] = 0;
                    } while (sVar5 < 8);
                    _DAT_00669714 = 1;
                    break;
                case 9:
                case 10:
                case 0xb:
                case 0xc:
                case 0xd:
                case 0xe:
                case 0xf:
                case 0x10:
                    if (param_1 == 0) {
                        bVar4 = DAT_00a91d17[(int)local_38[0] * 0x562];
                        if (2 < bVar4) {
                            bVar4 = 2;
                        }
                        if (bVar4 == 1) {
                            if (DAT_00bb73a7 != '\0') {
                                pcVar13 = s_sfxTarget2_0052cff8;
LAB_004f7813:
                                FUN_004de5a0(pcVar13, 1, 0, 0x7f);
                            }
                        } else if (DAT_00bb73a7 != '\0') {
                            pcVar13 = s_sfxTarget1_0052cfec;
                            goto LAB_004f7813;
                        }
                    }
                    sVar5 = (short)(sVar5 + -9);
                    DAT_006696e4[sVar5] = 1;
                    DAT_00669704[sVar5] = (short)((((param_1 == 0) - 1)) & 0x50);
                    DAT_006696f4[sVar5] = local_38[0];
                    break;
                case 0x11:
                    sVar5 = 0;
                    do {
                        iVar6 = (int)sVar5;
                        sVar5 = (short)(sVar5 + 1);
                        DAT_00669716[iVar6] = 0;
                    } while (sVar5 < 8);
                    _DAT_00669776 = 1;
                    break;
                case 0x12:
                case 0x13:
                case 0x14:
                case 0x15:
                case 0x16:
                case 0x17:
                case 0x18:
                case 0x19:
                    if (param_1 == 0) {
                        uVar7 = 0xffffffff;
                        pcVar13 = DAT_00669582[local_38[0]];
                        do {
                            pcVar12 = pcVar13;
                            if (uVar7 == 0) break;
                            uVar7 = uVar7 - 1;
                            pcVar12 = pcVar13 + 1;
                            cVar1 = *pcVar13;
                            pcVar13 = pcVar12;
                        } while (cVar1 != '\0');
                        uVar7 = ~uVar7;
                        pcVar13 = pcVar12 + -uVar7;
                        pcVar12 = local_28;
                        for (uVar8 = uVar7 >> 2; uVar8 != 0; uVar8 = uVar8 - 1) {
                            __builtin_memcpy(&uTmp, pcVar13, 4);
                            __builtin_memcpy(pcVar12, &uTmp, 4);
                            pcVar13 = pcVar13 + 4;
                            pcVar12 = pcVar12 + 4;
                        }
                        for (uVar7 = uVar7 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
                            *pcVar12 = *pcVar13;
                            pcVar13 = pcVar13 + 1;
                            pcVar12 = pcVar12 + 1;
                        }
                        iVar6 = -1;
                        pcVar13 = local_28;
                        do {
                            if (iVar6 == 0) break;
                            iVar6 = iVar6 + -1;
                            cVar1 = *pcVar13;
                            pcVar13 = pcVar13 + 1;
                        } while (cVar1 != '\0');
                        if (((short)iVar6 != -2) && (DAT_00bb73a7 != '\0')) {
                            FUN_004de5a0(s_sfxText_0052cfe4, 1, 0, 0x7f);
                        }
                    }
                    iVar11 = (int)(short)(sVar5 + -0x12);
                    DAT_00669716[iVar11] = 1;
                    DAT_00669756[iVar11] = (short)((((param_1 == 0) - 1)) & 0x50);
                    DAT_00669726[iVar11] = local_38[0];
                    DAT_00669736[iVar11] = local_38[1];
                    DAT_00669746[iVar11] = local_38[2];
                    DAT_00669766[iVar11] = local_38[3];
                }
            }
        } while (sVar2 <= DAT_0066925a);
    }
    DAT_0066925a = (short)(DAT_0066925a + 1);
    DAT_0066925c = local_3e;
    return;
}

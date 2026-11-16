/* SPDX-License-Identifier: MIT */
/* z_xvt 004f2e40 */
extern short DAT_0065047a;
extern short DAT_0065047c;
extern short DAT_00650482[];
extern short DAT_00533da8[];
extern short DAT_00650902;
extern short DAT_00650934;
extern short DAT_00650996;
extern short DAT_006508f6;
extern short DAT_006508f8;
extern short DAT_00650998;
extern short DAT_006508fa[];
extern short DAT_006508fe[];
extern short DAT_00650424;
extern short DAT_00650426;
extern short DAT_00650428;
extern short DAT_0065042a;
extern short DAT_0065042c;
extern short DAT_0065042e;
extern short DAT_00650430;
extern short DAT_00650432;
extern short DAT_00650904[];
extern short DAT_00650914[];
extern short DAT_00650924[];
extern short DAT_00650936[];
extern short DAT_00650946[];
extern short DAT_00650956[];
extern short DAT_00650966[];
extern short DAT_00650976[];
extern short DAT_00650986[];
extern unsigned char DAT_006507a2[];
extern unsigned char DAT_007438d7[];
extern char DAT_0085e7c5;
extern char s_sfxText_00533e68[];
extern char s_sfxTarget1_00533e70[];
extern char s_sfxTarget2_00533e7c[];

void __cdecl FUN_004de190(char *param_1, int param_2, int param_3, int param_4);

void __cdecl FUN_004f2e40(short param_1)
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

    local_3e = DAT_0065047c;
    DAT_00650902 = 0;
    DAT_00650934 = 0;
    DAT_00650996 = 0;
    DAT_006508f6 = 0;
    DAT_006508f8 = 0;
    DAT_00650998 = 0;
    sVar9 = DAT_0065047c;
    if (DAT_00650482[DAT_0065047c] <= DAT_0065047a) {
        do {
            local_3e = sVar9;
            sVar2 = DAT_00650482[local_3e];
            sVar9 = local_3e + 2;
            sVar10 = 0;
            sVar5 = DAT_00650482[(short)(local_3e + 1)];
            sVar3 = DAT_00533da8[sVar5];
            if (0 < sVar3) {
                do {
                    iVar6 = (int)sVar9;
                    iVar11 = (int)sVar10;
                    sVar9 = sVar9 + 1;
                    sVar10 = sVar10 + 1;
                    local_38[iVar11] = DAT_00650482[iVar6];
                } while (sVar10 < sVar3);
            }
            if (DAT_0065047a == sVar2) {
                switch ((int)sVar5) {
                case 1:
                    DAT_00650998 = 1;
                    break;
                case 3:
                    sVar5 = 0;
                    do {
                        iVar6 = (int)sVar5;
                        sVar5 = sVar5 + 1;
                        DAT_006508fa[iVar6] = 0;
                    } while (sVar5 < 2);
                    DAT_00650902 = 1;
                    break;
                case 4:
                case 5:
                    DAT_006508fa[(short)(sVar5 + -4)] = 1;
                    DAT_006508fe[(short)(sVar5 + -4)] = local_38[0];
                    break;
                case 6:
                    if ((sVar2 == 0) || (param_1 != 0)) {
                        DAT_00650424 = local_38[0];
                        DAT_00650426 = local_38[1];
                    }
                    DAT_0065042a = local_38[1];
                    DAT_00650428 = local_38[0];
                    DAT_006508f6 = 1;
                    break;
                case 7:
                    if ((sVar2 == 0) || (param_1 != 0)) {
                        DAT_0065042c = local_38[0];
                        DAT_0065042e = local_38[1];
                    }
                    DAT_00650432 = local_38[1];
                    DAT_00650430 = local_38[0];
                    DAT_006508f8 = 1;
                    break;
                case 8:
                    sVar5 = 0;
                    do {
                        iVar6 = (int)sVar5;
                        sVar5 = sVar5 + 1;
                        DAT_00650904[iVar6] = 0;
                    } while (sVar5 < 8);
                    DAT_00650934 = 1;
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
                        bVar4 = DAT_007438d7[local_38[0] * 0x562];
                        if (2 < bVar4) {
                            bVar4 = 2;
                        }
                        if (bVar4 == 1) {
                            if (DAT_0085e7c5 != '\0') {
                                pcVar13 = s_sfxTarget2_00533e7c;
                                FUN_004de190(pcVar13, 1, 0, 0x7f);
                            }
                        } else if (DAT_0085e7c5 != '\0') {
                            pcVar13 = s_sfxTarget1_00533e70;
                            FUN_004de190(pcVar13, 1, 0, 0x7f);
                        }
                    }
                    sVar5 = sVar5 + -9;
                    DAT_00650904[sVar5] = 1;
                    DAT_00650924[sVar5] = ((param_1 == 0) - 1) & 0x50;
                    DAT_00650914[sVar5] = local_38[0];
                    break;
                case 0x11:
                    sVar5 = 0;
                    do {
                        iVar6 = (int)sVar5;
                        sVar5 = sVar5 + 1;
                        DAT_00650936[iVar6] = 0;
                    } while (sVar5 < 8);
                    DAT_00650996 = 1;
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
                        __builtin_memcpy(&pcVar13, DAT_006507a2 + (int)local_38[0] * 4, sizeof(pcVar13));
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
                            __builtin_memcpy(pcVar12, pcVar13, 4);
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
                        if (((short)iVar6 != -2) && (DAT_0085e7c5 != '\0')) {
                            FUN_004de190(s_sfxText_00533e68, 1, 0, 0x7f);
                        }
                    }
                    iVar11 = (int)(short)(sVar5 + -0x12);
                    DAT_00650936[iVar11] = 1;
                    DAT_00650976[iVar11] = ((param_1 == 0) - 1) & 0x50;
                    DAT_00650946[iVar11] = local_38[0];
                    DAT_00650956[iVar11] = local_38[1];
                    DAT_00650966[iVar11] = local_38[2];
                    DAT_00650986[iVar11] = local_38[3];
                    break;
                }
            }
        } while (DAT_0065047a >= sVar2);
    }
    DAT_0065047a = DAT_0065047a + 1;
    DAT_0065047c = local_3e;
    return;
}

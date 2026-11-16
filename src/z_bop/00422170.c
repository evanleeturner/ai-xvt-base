/* SPDX-License-Identifier: MIT */
/* z_bop 00422170 */
extern int DAT_0052361c;
extern int DAT_005233d8;
extern int DAT_009a8c18;
extern int DAT_009a8d50;
extern short DAT_009cd266;
extern short DAT_009d114c;
extern short DAT_009d12e0;
extern short DAT_009e95f2;
extern short DAT_009e9646;
extern short DAT_009e9650;
extern short DAT_009e9652;
extern short DAT_009e9654;
extern short DAT_009ec458;
extern short DAT_009ec462;
extern unsigned char DAT_009ec616;
extern unsigned char DAT_009ec617;
extern short DAT_009ed230;
extern int DAT_009fe7d4;
extern short DAT_00a00850;
extern short DAT_00a00852;
extern int DAT_00a60a50;
extern int DAT_009a57e0[];
extern unsigned short DAT_009ec818[256];
extern short DAT_009ec618[256];
extern unsigned char DAT_00523610[];
extern unsigned char DAT_00523600[];

int FUN_004226a0(void);
void FUN_004214a0(int *, int, int);
void FUN_004213e0(int *, int, int);
void FUN_004214f0(int *, int, int);
void FUN_00421430(int *, int, int);
int FUN_00422710(void);
void FUN_00422560(void);

void __cdecl FUN_00422170(unsigned char *param_1, int param_2)
{
    unsigned char *pbVar1;
    unsigned char bVar2;
    int *piVar3;
    unsigned short uVar4;
    short sVar5;
    int iVar6;
    int iVar7;
    unsigned char bVar8;
    unsigned short uVar9;
    short sVar10;
    int *piVar11;
    int iVar12;
    unsigned char local_12;
    unsigned char local_11;
    unsigned char local_10;
    unsigned char local_f;
    short local_e;

    DAT_0052361c = 1;
    iVar6 = FUN_004226a0();
    if (iVar6 != 0) {
        DAT_00a00852 = DAT_009e9654;
        DAT_00a00850 = DAT_009e9652;
        DAT_009e9646 = DAT_009ec458;
        DAT_009e9650 = DAT_009d114c;
        if (DAT_009e95f2 < 1) {
            *(int *)(DAT_009fe7d4 + 0x25b4) =
                (int)((unsigned int)((int)DAT_009ed230 - (int)DAT_009e9652) * (unsigned int)DAT_009a8d50
                      + (unsigned int)(int)DAT_009e9654 * (unsigned int)DAT_005233d8
                      + (unsigned int)DAT_009a8c18);
            *(int *)(DAT_009fe7d4 + 0x3eb8) = DAT_009a8d50;
        }
        else {
            *(int *)(DAT_009fe7d4 + 0x25b4) =
                (int)((unsigned int)(int)DAT_009e9654 * (unsigned int)DAT_005233d8
                      + (unsigned int)(int)DAT_009e9652 * (unsigned int)DAT_009a8d50
                      + (unsigned int)DAT_009a8c18);
            *(int *)(DAT_009fe7d4 + 0x3eb8) = (int)(0u - (unsigned int)DAT_009a8d50);
        }
        local_10 = 0;
        local_f = 0;
        local_11 = 0;
        bVar2 = 0;
        DAT_009a8c18 = *(int *)(DAT_009fe7d4 + 0x25b4);
        DAT_009cd266 = 1;
        DAT_009d12e0 = 0;
        bVar8 = *param_1;
        while (bVar8 != 0xff) {
            uVar9 = (unsigned short)local_11;
            bVar8 = local_f;
            if (((unsigned char)(DAT_009ec616 + local_10) < local_10) &&
                (bVar8 = (unsigned char)(local_f + 1), bVar8 == 0)) {
                uVar9 = (unsigned short)(unsigned char)(local_11 + 1);
            }
            if ((unsigned char)(bVar8 + DAT_009ec617) < bVar8) {
                uVar9 = (unsigned short)(unsigned char)((char)uVar9 + 1);
            }
            iVar6 = 0;
            iVar12 = 0;
            sVar10 = (short)((uVar9 * 0x100 - ((unsigned short)local_11 * 0x100 + (unsigned short)local_f)) +
                             (unsigned short)(unsigned char)(bVar8 + DAT_009ec617));
            local_e = sVar10;
            if (DAT_0052361c == 1) {
                bVar8 = *param_1;
                piVar3 = DAT_009a57e0;
                uVar9 = 0;
                while (bVar8 != 0xfe) {
                    bVar8 = *param_1;
                    piVar11 = piVar3;
                    if (bVar8 == 0xfb) {
                        bVar2 = param_1[1];
                        param_1 = param_1 + 3;
                        iVar7 = iVar12;
                        uVar4 = uVar9;
                    }
                    else if (bVar8 == 0xfc) {
                        pbVar1 = param_1 + 1;
                        param_1 = param_1 + 2;
                        iVar7 = (int)((unsigned int)iVar12 + (unsigned int)DAT_009ec818[*pbVar1]);
                        uVar4 = (unsigned short)(uVar9 + DAT_009ec618[*pbVar1]);
                        if (uVar4 < uVar9) {
                            iVar7 = (int)((unsigned int)iVar7 + 1u);
                        }
                    }
                    else {
                        if (bVar8 == 0xfd) {
                            bVar8 = param_1[1];
                            local_12 = param_1[2];
                            param_1 = param_1 + 3;
                        }
                        else {
                            param_1 = param_1 + 1;
                            local_12 = (unsigned char)((bVar8 >> (DAT_00523610[param_2] & 0x1f)) + bVar2);
                            bVar8 = (unsigned char)(bVar8 & DAT_00523600[param_2]);
                        }
                        iVar7 = (int)((unsigned int)iVar12 + (unsigned int)DAT_009ec818[bVar8]);
                        uVar4 = (unsigned short)(uVar9 + DAT_009ec618[bVar8]);
                        if (uVar4 < uVar9) {
                            iVar7 = (int)((unsigned int)iVar7 + 1u);
                        }
                        *piVar3 = iVar12;
                        piVar11 = piVar3 + 3;
                        iVar6 = iVar6 + 1;
                        piVar3[1] = (int)(unsigned int)local_12;
                        piVar3[2] = (int)((unsigned int)(iVar7 - iVar12) + 1u);
                    }
                    iVar12 = iVar7;
                    piVar3 = piVar11;
                    uVar9 = uVar4;
                    bVar8 = *param_1;
                }
                param_1 = param_1 + 1;
            }
            do {
                if ((iVar6 != 0) && (-1 < DAT_009d114c)) {
                    sVar5 = (short)(DAT_009ec462 + (short)iVar12);
                    DAT_00a60a50 = iVar6;
                    if ((sVar5 < 0) ||
                        ((DAT_009d114c <= sVar5 || (DAT_009ec462 < 0)) || (DAT_009ec462 < DAT_009ec458))) {
                        if (DAT_005233d8 == 2) {
                            FUN_004214f0(DAT_009a57e0, DAT_009a8c18, DAT_009fe7d4 + 0x25b8);
                        }
                        else {
                            FUN_00421430(DAT_009a57e0, DAT_009a8c18, DAT_009fe7d4 + 0x25b8);
                        }
                    }
                    else if (DAT_005233d8 == 2) {
                        FUN_004214a0(DAT_009a57e0, DAT_009a8c18, DAT_009fe7d4 + 0x25b8);
                    }
                    else {
                        FUN_004213e0(DAT_009a57e0, DAT_009a8c18, DAT_009fe7d4 + 0x25b8);
                    }
                }
                if (sVar10 != 0) {
                    iVar7 = FUN_00422710();
                    if (iVar7 == 0) {
                        return;
                    }
                    FUN_00422560();
                }
            } while (((short)(local_e + -1) != 0) && (local_e = (short)(local_e + -1), sVar10 != 0));
            if ((unsigned char)(local_10 + DAT_009ec616) < local_10) {
                local_f = (unsigned char)(local_f + 1);
            }
            if ((unsigned char)(local_f + DAT_009ec617) < local_f) {
                local_11 = (unsigned char)(local_11 + 1);
            }
            local_10 = (unsigned char)(local_10 + DAT_009ec616);
            local_f = (unsigned char)(local_f + DAT_009ec617);
            bVar8 = *param_1;
        }
    }
    return;
}

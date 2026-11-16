/* SPDX-License-Identifier: MIT */
/* z_xvt 0049c2e0 */
extern short DAT_006fedd0;
extern short DAT_0071f714;
extern short DAT_0070a1d0;
extern unsigned short DAT_006d4a46;
extern unsigned short DAT_0071e6ca;
extern unsigned short DAT_006d498c;
extern unsigned short DAT_007091ac;
extern unsigned int DAT_006d4980;
extern int DAT_006d4988;
extern unsigned short DAT_006d4984;
extern int DAT_0070a134;
extern int DAT_0070a00c;
extern unsigned int DAT_0070a130;
extern unsigned int DAT_0070a010;
extern unsigned int DAT_0070a12c;
extern unsigned int DAT_0070a138;
extern unsigned int DAT_006db7ac;
extern unsigned int DAT_006db7b0;
extern unsigned int DAT_006db7b4;
extern unsigned short DAT_00528230[];

int __cdecl FUN_004490f0(int param_1);
unsigned int __cdecl FUN_00449790(int param_1, unsigned int param_2);
unsigned int __cdecl FUN_00419100(unsigned int param_1, unsigned int param_2);
unsigned int __cdecl FUN_0040ca90(unsigned int param_1, unsigned int param_2);
unsigned int __cdecl FUN_0040cad0(unsigned int param_1, unsigned int param_2);
void __cdecl FUN_0047f360(short param_1, unsigned short param_2, int param_3, short param_4,
                          short param_5, unsigned int param_6, short param_7);

void __cdecl FUN_0049c2e0(unsigned int param_1, int param_2)
{
    unsigned short uVar1;
    short sVar2;
    int iVar3;
    unsigned int uVar4;
    unsigned int uVar5;
    unsigned int uVar6;
    char cVar7;
    int iVar8;
    unsigned short uVar9;
    short sVar10;
    short local_4 = 0; /* original is uninitialised; zeroed to satisfy the build gate */
    short stmp;

    sVar2 = DAT_006fedd0;
    DAT_0071f714 = (short)param_1;
    if ((short)param_1 == DAT_0070a1d0) {
        DAT_006fedd0 = (short)param_1;
    }
    DAT_006d4a46 = (unsigned short)((short)param_1 == DAT_0070a1d0);
    uVar9 = 0;
    iVar3 = FUN_004490f0(param_2);
    DAT_0071e6ca = (unsigned short)iVar3;
    sVar10 = local_4;
    if (DAT_0071e6ca != 0) {
        do {
            DAT_006d498c = uVar9;
            uVar4 = FUN_00449790(param_2, (unsigned int)uVar9);
            if (DAT_006d4a46 == 2) {
                DAT_006d4a46 = 0;
            }
            if ((uVar9 == DAT_007091ac) && (DAT_006d4a46 == 0)) {
                DAT_006d4a46 = 2;
            }
            cVar7 = '\0';
            if (param_1 < DAT_006d4980) {
                __builtin_memcpy(&iVar3, (unsigned char *)(DAT_006d4988 + 0x1f + param_1 * 0x23), 4);
                if (iVar3 != 0) {
                    __builtin_memcpy(&iVar3, (unsigned char *)(iVar3 + 0xa7), 4);
                    if (iVar3 != 0) {
                        cVar7 = *(char *)(uVar9 + 0x2ca + iVar3);
                    }
                }
            }
            if (cVar7 == '\0') {
                if (((short)uVar4 == 3) && (param_1 < DAT_006d4980)) {
                    uVar5 = 0;
                    __builtin_memcpy(&iVar3, (unsigned char *)(DAT_006d4988 + 0x1f + param_1 * 0x23), 4);
                    if (iVar3 != 0) {
                        __builtin_memcpy(&iVar3, (unsigned char *)(iVar3 + 0xa7), 4);
                        if (iVar3 != 0) {
                            uVar5 = (unsigned int)*(unsigned char *)(DAT_0071e6ca + 0x2ca + iVar3);
                        }
                    }
                    uVar1 = DAT_00528230[uVar5];
                    if ((0x7fff < uVar1) && (uVar1 < 0xff00)) {
                        if (local_4 == 0) {
                            iVar3 = DAT_0070a134;
                            if (DAT_0070a134 < 0) {
                                iVar3 = (int)(0u - (unsigned int)DAT_0070a134);
                            }
                            iVar8 = DAT_0070a00c;
                            if (DAT_0070a00c < 0) {
                                iVar8 = (int)(0u - (unsigned int)DAT_0070a00c);
                            }
                            uVar5 = DAT_0070a130;
                            uVar6 = DAT_0070a010;
                            if (iVar3 < iVar8) {
                                uVar5 = DAT_0070a12c;
                                uVar6 = DAT_0070a138;
                            }
                            if ((int)uVar5 < 0) {
                                uVar4 = FUN_00419100(uVar6, 0u - uVar5);
                                sVar10 = (short)uVar4;
                            } else {
                                uVar4 = FUN_00419100(uVar6, uVar5);
                                sVar10 = (short)(0u - (unsigned int)(unsigned short)(short)uVar4);
                            }
                            local_4 = 1;
                        }
                        __builtin_memcpy(&stmp, (unsigned char *)(DAT_006d4988 + 0x14 + param_1 * 0x23), 2);
                        sVar10 = (short)(sVar10 + stmp);
                        uVar5 = FUN_0040ca90(DAT_006db7ac, DAT_006db7b4);
                        if (((int)(uVar5 & 0xffff0000) < 1) && (-0x10001 < (int)(uVar5 & 0xffff0000))) {
                            uVar6 = FUN_0040cad0(DAT_006db7b0, DAT_006db7b4);
                            if (((int)(uVar6 & 0xffff0000) < 1) && (-0x10001 < (int)(uVar6 & 0xffff0000))) {
                                FUN_0047f360(DAT_0071f714, uVar1, 0x100, (short)uVar5,
                                             (short)((DAT_006d4984 & 0xfffe) - (short)uVar6),
                                             DAT_006db7b4, sVar10);
                            }
                        }
                    }
                }
            }
            uVar9 = (unsigned short)(uVar9 + 1);
        } while (uVar9 < DAT_0071e6ca);
    }
    DAT_006fedd0 = sVar2;
    return;
}

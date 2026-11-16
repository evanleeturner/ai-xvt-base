/* SPDX-License-Identifier: MIT */
/* z_bop 0041fc50 */
extern short DAT_009cc450;
extern short DAT_009ed664;
extern short DAT_009d8110;
extern unsigned short DAT_009a20a6;
extern unsigned int DAT_00a00870[];
extern unsigned int DAT_00a00874[];
extern unsigned short DAT_009ec60a;
extern unsigned short DAT_009a1fe6;
extern unsigned short DAT_009d682e;
extern unsigned int DAT_009a1fe0;
extern unsigned int DAT_009a1fe8;
extern unsigned short DAT_009a1fe4;
extern unsigned short DAT_0051a0b0[];
extern int DAT_009d77b4;
extern int DAT_009d768c;
extern unsigned int DAT_009d77b0;
extern unsigned int DAT_009d7690;
extern unsigned int DAT_009d77ac;
extern unsigned int DAT_009d77b8;
extern int DAT_009a8e2c;
extern int DAT_009a8e30;
extern int DAT_009a8e34;

int __cdecl FUN_004adc40(int param_1);
unsigned int __cdecl FUN_004ae2a0(int param_1, unsigned int param_2);
unsigned int __cdecl FUN_0046abf0(unsigned int param_1, unsigned int param_2);
unsigned int __cdecl FUN_00427570(int param_1, int param_2);
unsigned int __cdecl FUN_00427590(int param_1, int param_2);
void __cdecl FUN_00401250(short param_1, unsigned short param_2, int param_3,
                          short param_4, short param_5, int param_6, short param_7);

void __cdecl FUN_0041fc50(unsigned int param_1, int param_2)
{
    unsigned short uVar1;
    short sVar2;
    short sVar3;
    int iVar4;
    unsigned int uVar5;
    unsigned int uVar6;
    char cVar7;
    int iVar8;
    short sVar9;
    unsigned int uVar10;
    short local_6;
    short *pLocal_6;
    unsigned short uStack_2;
    int tmp;
    short stmp;

    sVar2 = DAT_009cc450;
    DAT_009ed664 = (short)param_1;
    if ((short)param_1 == DAT_009d8110) {
        DAT_009cc450 = (short)param_1;
    }
    DAT_009a20a6 = (unsigned short)((short)param_1 == DAT_009d8110);
    if (param_2 < 0x49) {
        DAT_009ec60a = (unsigned short)DAT_00a00870[(unsigned int)param_2 * 0x65u];
    }
    else {
        iVar4 = FUN_004adc40(param_2);
        DAT_009ec60a = (unsigned short)iVar4;
    }
    uStack_2 = 0;
    pLocal_6 = &local_6;
    sVar9 = *pLocal_6;
    if (DAT_009ec60a != 0) {
        do {
            DAT_009a1fe6 = uStack_2;
            uVar10 = (unsigned int)uStack_2;
            if (param_2 < 0x49) {
                uVar6 = uVar10;
                if ((int)DAT_00a00870[(unsigned int)param_2 * 0x65u] <= (int)uVar10) {
                    uVar6 = DAT_00a00870[(unsigned int)param_2 * 0x65u] - 1;
                }
                sVar3 = (short)DAT_00a00874[(unsigned int)param_2 * 0x65u + uVar6];
            }
            else {
                uVar5 = FUN_004ae2a0(param_2, uVar10);
                sVar3 = (short)uVar5;
            }
            if (DAT_009a20a6 == 2) {
                DAT_009a20a6 = 0;
            }
            if ((uStack_2 == DAT_009d682e) && (DAT_009a20a6 == 0)) {
                DAT_009a20a6 = 2;
            }
            if (param_1 < DAT_009a1fe0) {
                cVar7 = '\0';
                __builtin_memcpy(&iVar4, (const char *)(DAT_009a1fe8 + 0x1f + param_1 * 0x23), 4);
                if (iVar4 != 0) {
                    __builtin_memcpy(&tmp, (const char *)((unsigned int)iVar4 + 0xa9), 4);
                    iVar4 = tmp;
                    if (iVar4 != 0) {
                        cVar7 = *(char *)(uVar10 + 0x2d8 + (unsigned int)iVar4);
                    }
                }
                if (cVar7 == '\0') goto LAB_0041fdb5;
            }
            else {
LAB_0041fdb5:
                if ((sVar3 == 3) && (param_1 < DAT_009a1fe0)) {
                    uVar10 = 0;
                    __builtin_memcpy(&iVar4, (const char *)(DAT_009a1fe8 + 0x1f + param_1 * 0x23), 4);
                    if (iVar4 != 0) {
                        __builtin_memcpy(&tmp, (const char *)((unsigned int)iVar4 + 0xa9), 4);
                        iVar4 = tmp;
                        if (iVar4 != 0) {
                            uVar10 = (unsigned int)*(unsigned char *)((unsigned int)DAT_009ec60a + 0x2d8 + (unsigned int)iVar4);
                        }
                    }
                    uVar1 = DAT_0051a0b0[uVar10];
                    if ((0x7fff < uVar1) && (uVar1 < 0xff00)) {
                        if (local_6 == 0) {
                            iVar4 = DAT_009d77b4;
                            if (DAT_009d77b4 < 0) {
                                iVar4 = (int)(0u - (unsigned int)DAT_009d77b4);
                            }
                            iVar8 = DAT_009d768c;
                            if (DAT_009d768c < 0) {
                                iVar8 = (int)(0u - (unsigned int)DAT_009d768c);
                            }
                            uVar10 = DAT_009d77b0;
                            uVar6 = DAT_009d7690;
                            if (iVar4 < iVar8) {
                                uVar10 = DAT_009d77ac;
                                uVar6 = DAT_009d77b8;
                            }
                            if ((int)uVar10 < 0) {
                                uVar5 = FUN_0046abf0(uVar6, 0u - uVar10);
                                sVar9 = (short)uVar5;
                            }
                            else {
                                uVar5 = FUN_0046abf0(uVar6, uVar10);
                                sVar9 = (short)(0u - (unsigned int)(unsigned short)uVar5);
                            }
                            local_6 = 1;
                        }
                        __builtin_memcpy(&stmp, (const char *)(DAT_009a1fe8 + 0x14 + param_1 * 0x23), 2);
                        sVar9 = (short)((int)sVar9 + (int)stmp);
                        uVar10 = FUN_00427570(DAT_009a8e2c, DAT_009a8e34);
                        if (((int)(uVar10 & 0xffff0000) < 1) && (-0x10001 < (int)(uVar10 & 0xffff0000))) {
                            uVar6 = FUN_00427590(DAT_009a8e30, DAT_009a8e34);
                            if (((int)(uVar6 & 0xffff0000) < 1) && (-0x10001 < (int)(uVar6 & 0xffff0000))) {
                                FUN_00401250(DAT_009ed664, uVar1, 0x100, (short)uVar10,
                                             (short)((int)(DAT_009a1fe4 & 0xfffe) - (int)(short)uVar6),
                                             DAT_009a8e34, sVar9);
                            }
                        }
                    }
                }
            }
            uStack_2 = (unsigned short)(uStack_2 + 1);
        } while (uStack_2 < DAT_009ec60a);
    }
    DAT_009cc450 = sVar2;
    return;
}

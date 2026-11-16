/* SPDX-License-Identifier: MIT */
/* z_bop 00401000 */
extern int DAT_009a1fe8;
extern short DAT_009ed664;
extern int DAT_00a90a60;
extern unsigned short DAT_00a90a5e;
extern unsigned short DAT_009a1fe6;
extern short DAT_009a1fe4;
extern unsigned char DAT_00521f26[];
extern unsigned short DAT_0051a060[];
extern unsigned int DAT_009a8e34;
extern int DAT_009a8e2c;
extern int DAT_009a8e30;
extern int DAT_009d77b4;
extern int DAT_009d768c;
extern unsigned int DAT_009d77b0;
extern unsigned int DAT_009d7690;
extern unsigned int DAT_009d77ac;
extern unsigned int DAT_009d77b8;

void __cdecl FUN_004728d0(int param_1, unsigned int param_2);
unsigned int __cdecl FUN_0046abf0(unsigned int param_1, unsigned int param_2);
unsigned int __cdecl FUN_00427570(int param_1, unsigned int param_2);
unsigned int __cdecl FUN_00427590(int param_1, unsigned int param_2);
void __cdecl FUN_00401250(short param_1, unsigned short param_2, unsigned short param_3,
                          short param_4, short param_5, unsigned int param_6, short param_7);

void __cdecl FUN_00401000(int param_1)
{
    unsigned char bVar1;
    int iVar2;
    short sVar3;
    unsigned short uVar4;
    unsigned int uVar5;
    unsigned int uVar6;
    unsigned int uVar7;
    int iVar8;
    int iVar9;
    unsigned short uVar10;
    unsigned short local_2;
    int tmp;

    iVar2 = (int)((unsigned int)param_1 * 0x23u);
    iVar8 = (int)((unsigned int)DAT_009a1fe8 + (unsigned int)iVar2);
    DAT_009ed664 = (short)param_1;
    local_2 = (unsigned short)*(unsigned char *)(iVar8 + 3);
    __builtin_memcpy(&DAT_00a90a60, &DAT_00521f26[(unsigned int)local_2 * 0x1a], 4);
    if (local_2 == 0x59) {
        uVar10 = (unsigned short)(*(unsigned char *)(iVar8 + 0x19) >> 1);
    }
    else {
        if (DAT_00a90a60 == 0) {
            return;
        }
        DAT_00a90a5e = (unsigned short)*(unsigned char *)(iVar8 + 0x19);
        uVar10 = *(unsigned short *)(DAT_00a90a60 + (unsigned int)DAT_00a90a5e * 2);
    }
    if (uVar10 < 0xff00) {
        if (uVar10 < 0x8000) {
            if (local_2 == 0x59) {
                __builtin_memcpy(&tmp, (unsigned char *)(iVar8 + 0x1f), 4);
                local_2 = (unsigned short)*(unsigned char *)(tmp + 0x87);
            }
            DAT_009a1fe6 = uVar10;
            FUN_004728d0(iVar8, (unsigned int)uVar10);
            if (local_2 == 0x59) {
                DAT_00a90a5e = (unsigned short)*(unsigned char *)(DAT_009a1fe8 + 0x1a + iVar2);
                uVar10 = DAT_0051a060[(unsigned int)DAT_00a90a5e];
            }
        }
        if (((uVar10 < 0xff00) && (0x7fff < uVar10)) && (-1 < (int)DAT_009a8e34)) {
            iVar8 = DAT_009d77b4;
            if (DAT_009d77b4 < 0) {
                iVar8 = (int)(0u - (unsigned int)DAT_009d77b4);
            }
            iVar9 = DAT_009d768c;
            if (DAT_009d768c < 0) {
                iVar9 = (int)(0u - (unsigned int)DAT_009d768c);
            }
            uVar6 = DAT_009d77b0;
            uVar7 = DAT_009d7690;
            if (iVar8 < iVar9) {
                uVar6 = DAT_009d77ac;
                uVar7 = DAT_009d77b8;
            }
            if ((int)uVar6 < 0) {
                uVar5 = FUN_0046abf0(uVar7, 0u - uVar6);
                sVar3 = (short)uVar5;
            }
            else {
                uVar5 = FUN_0046abf0(uVar7, uVar6);
                sVar3 = (short)(0u - (unsigned int)(unsigned short)uVar5);
            }
            uVar6 = FUN_00427570(DAT_009a8e2c, DAT_009a8e34);
            if (((int)(uVar6 & 0xffff0000u) < 1) && (-0x10001 < (int)(uVar6 & 0xffff0000u))) {
                uVar7 = FUN_00427590(DAT_009a8e30, DAT_009a8e34);
                if (((int)(uVar7 & 0xffff0000u) < 1) && (-0x10001 < (int)(uVar7 & 0xffff0000u))) {
                    __builtin_memcpy(&tmp, (unsigned char *)(DAT_009a1fe8 + 0x1f + iVar2), 4);
                    bVar1 = *(unsigned char *)(tmp + 1);
                    if (bVar1 == 0) {
                        uVar4 = 0x100;
                    }
                    else {
                        uVar4 = (unsigned short)((unsigned short)bVar1 * 0x40);
                        if (0xff < uVar4) {
                            uVar4 = (unsigned short)(uVar4 + 0x100);
                        }
                    }
                    FUN_00401250(DAT_009ed664, uVar10, uVar4, (short)uVar6,
                                 (short)(DAT_009a1fe4 - (short)uVar7),
                                 DAT_009a8e34, sVar3);
                }
            }
        }
    }
    return;
}

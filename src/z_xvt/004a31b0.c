/* SPDX-License-Identifier: MIT */
/* z_xvt 004a31b0 */
extern unsigned short DAT_00732a3c;
extern int DAT_006d4980;
extern int DAT_006d4988;
extern unsigned char DAT_0070ad8b[];
extern unsigned char DAT_0070ad43[];
extern unsigned char DAT_0070ad47[];
extern unsigned char DAT_0070acf0[];
extern int DAT_0071ecf4;
extern unsigned char DAT_0071b743[];
extern unsigned char DAT_0071b730[];
extern unsigned char DAT_0071b73a[];
extern unsigned char DAT_0071bb23[];
extern unsigned char DAT_0071bb1f[];
extern unsigned char DAT_0071bb1b[];
extern unsigned char DAT_0052b07a[];
extern int DAT_00732a2c;
extern int DAT_00732a28;
extern int DAT_00732a24;
extern unsigned int DAT_006db580;
extern unsigned short _DAT_00732950;
extern unsigned short _DAT_00732952;
extern unsigned short _DAT_00732954;
extern short DAT_00732956;
extern unsigned short DAT_00732fe2;
extern int _DAT_00732e78;
extern int _DAT_00732e74;
extern int _DAT_00732e70;

void __cdecl FUN_00421c70(uint param_1, uint param_2);
void __cdecl FUN_00418cd0(int param_1, int param_2, int param_3);
void __cdecl FUN_004a1d70(int param_1, int param_2);

void __cdecl FUN_004a31b0(uint param_1, uint param_2)
{
    int iVar1;
    int iVar2;
    int iVar3;
    unsigned short uVar4;
    short sVar5;
    uint uVar6;
    int iVar7;
    uint uVar8;
    int iTmp;
    int iTmp2;
    unsigned int uTmp;
    unsigned short usTmp;

    uVar6 = param_1 & 0xffff;
    iVar1 = (int)(uVar6 * 0x566);
    uVar4 = DAT_00732a3c;
    if (DAT_0070ad8b[uVar6 * 0x566] == 0) {
        uVar8 = 0x8000;
LAB_004a3258:
        FUN_00421c70(uVar8, uVar6);
    }
    else {
        for (; (int)(uint)uVar4 < DAT_006d4980; uVar4 = (unsigned short)(uVar4 + 1)) {
            iVar7 = (int)((uint)uVar4 * 0x23 + (uint)DAT_006d4988);
            if (((*(char *)(iVar7 + 3) != 0) && (*(unsigned char *)(iVar7 + 0x16) == uVar6))) {
                __builtin_memcpy(&iTmp, (unsigned char *)(iVar7 + 0x1f), 4);
                __builtin_memcpy(&iTmp2, (unsigned char *)(iTmp + 0xa7), 4);
                if (*(char *)(iTmp2 + 5) == -1) {
                    uVar8 = (uint)uVar4;
                    goto LAB_004a3258;
                }
            }
        }
    }
    iVar7 = (int)((uint)DAT_0071ecf4 * 0x5bd);
    if (DAT_0071b743[iVar7] == 0) {
        __builtin_memcpy(&iTmp, DAT_0071b730 + iVar7, 4);
        iVar7 = (int)((uint)iTmp * 0x23 + (uint)DAT_006d4988);
        __builtin_memcpy(&iVar2, (unsigned char *)(iVar7 + 0xc), 4);
        __builtin_memcpy(&iVar3, (unsigned char *)(iVar7 + 8), 4);
        __builtin_memcpy(&iVar7, (unsigned char *)(iVar7 + 4), 4);
    }
    else {
        __builtin_memcpy(&iVar2, DAT_0071bb23 + iVar7, 4);
        __builtin_memcpy(&iVar3, DAT_0071bb1f + iVar7, 4);
        __builtin_memcpy(&iVar7, DAT_0071bb1b + iVar7, 4);
    }
    FUN_00418cd0((int)((uint)DAT_00732a2c - (uint)iVar7),
                 (int)((uint)DAT_00732a28 - (uint)iVar3),
                 (int)((uint)DAT_00732a24 - (uint)iVar2));
    DAT_006db580 = DAT_006db580 * 0xa1;
    uTmp = (DAT_006db580 >> 0x10) + 0x32;
    sVar5 = (short)(uTmp / 100);
    if (sVar5 == 0) {
        sVar5 = 1;
    }
    _DAT_00732950 = (unsigned short)DAT_0070ad43[iVar1];
    DAT_00732fe2 = (unsigned short)DAT_0070ad47[iVar1];
    _DAT_00732954 = (unsigned short)sVar5;
    __builtin_memcpy(&usTmp, DAT_0071b73a + (uint)DAT_0071ecf4 * 0x5bd, 2);
    if (usTmp == DAT_00732fe2) {
        if (_DAT_00732950 != 1) {
            _DAT_00732952 = 0x8001;
            _DAT_00732e78 = (int)DAT_0070acf0 + iVar1;
            _DAT_00732954 = 0x8002;
            __builtin_memcpy(&_DAT_00732e74, DAT_0052b07a + (param_2 & 0xffff) * 0xef, 4);
            DAT_00732956 = sVar5;
            FUN_004a1d70(0xeb, DAT_0071ecf4);
            return;
        }
        _DAT_00732950 = 0x8000;
        _DAT_00732e74 = (int)DAT_0070acf0 + iVar1;
        _DAT_00732952 = 0x8001;
        __builtin_memcpy(&_DAT_00732e70, DAT_0052b07a + (param_2 & 0xffff) * 0xef, 4);
        FUN_004a1d70(0xea, DAT_0071ecf4);
        return;
    }
    _DAT_00732952 = 0x8001;
    __builtin_memcpy(&_DAT_00732e74, DAT_0052b07a + (param_2 & 0xffff) * 0xef, 4);
    if (DAT_0070ad43[iVar1] != 1) {
        FUN_004a1d70(0x72, DAT_0071ecf4);
        return;
    }
    FUN_004a1d70(0x71, DAT_0071ecf4);
    return;
}

/* SPDX-License-Identifier: MIT */
/* z_bop 00421c50 */
/* Globals: widths taken from the P-code and decompile; names kept as placeholders. */
extern int DAT_005233d8;
extern int DAT_005235f8;
extern int DAT_009a8d50;
extern short DAT_009d8c2c;
extern short DAT_009e95f2;
extern unsigned short DAT_009ecc44;
extern unsigned short DAT_009ed230;
extern short DAT_00a07cd0;
extern unsigned short DAT_00a080f2;

void __stdcall RaiseException(DWORD, DWORD, DWORD, const DWORD *);
unsigned short __cdecl FUN_0046a3b0(unsigned short a);
unsigned short __cdecl FUN_00421980(unsigned short a, short b);

void __cdecl FUN_00421c50(unsigned short param_1, unsigned short *param_2)
{
    unsigned short uVar1;
    unsigned short uVar2;
    unsigned int uVar3;
    int iVar4;
    unsigned short uVar5;
    unsigned short uVar6;
    short sVar7;
    unsigned short uVar8;
    unsigned short *puVar9;
    int bVar10;
    short sVar11;
    short local_8;
    unsigned int div_lo;

    *param_2 = param_1;
    uVar1 = 0;
    param_2[2] = param_1 & 0x8000;
    param_2[4] = (unsigned short)(param_1 + 0x4000) & 0x8000;
    uVar2 = 0;
    if (param_1 < 0x8000) {
        uVar5 = 0;
        if (param_1 < 0x4000) goto LAB_00421cad;
    }
    else {
        param_1 = param_1 & 0x7fff;
        uVar1 = 1;
        uVar5 = 1;
        if (0x3fff < param_1) goto LAB_00421cad;
    }
    uVar1 = uVar5;
    uVar2 = 2;
LAB_00421cad:
    param_2[8] = uVar1;
    param_2[9] = uVar2;
    uVar2 = param_1;
    if (0x3fff < param_1) {
        uVar2 = (unsigned short)(0x8000 - param_1);
    }
    uVar1 = FUN_0046a3b0(uVar2);
    param_2[1] = uVar1;
    uVar1 = FUN_0046a3b0((unsigned short)(uVar2 + 0x4000));
    param_2[3] = uVar1;
    if (uVar2 < DAT_00a080f2) {
        local_8 = 0;
        param_2[0xc] = 0;
        if (DAT_005235f8 == 1) {
            sVar7 = 100;
        }
        else {
            sVar7 = 0x5b;
        }
    }
    else {
        local_8 = 4;
        param_2[0xc] = 4;
        uVar2 = (unsigned short)(0x4000 - uVar2);
        if (DAT_005235f8 == 1) {
            sVar7 = 100;
        }
        else {
            sVar7 = 0x6e;
        }
    }
    uVar1 = FUN_00421980(uVar2, sVar7);
    uVar2 = FUN_0046a3b0((unsigned short)(uVar2 + 0x4000));
    param_2[5] = uVar2;
    if (uVar2 == 0) {
        RaiseException(0xC0000094UL, 0, 0, NULL);
    }
    uVar5 = (unsigned short)(0x80000000ULL / (unsigned long long)uVar2);
    if ((local_8 != 0) && (DAT_005235f8 == 0)) {
        uVar5 = (unsigned short)(((((unsigned int)(0x80000000ULL / (unsigned long long)uVar2)) & 0xffff) * (unsigned int)DAT_009ecc44 + 0x8000) >> 0x10);
    }
    param_2[6] = uVar5;
    uVar2 = 0;
    param_2[0x18] = 0;
    param_2[0x19] = 0;
    if (param_2[0xc] == 0) {
        uVar8 = 0;
        param_2[7] = DAT_009d8c2c;
        uVar5 = 0x8000;
        sVar7 = (short)(DAT_009d8c2c - 2);
        uVar6 = 2;
        if (DAT_009d8c2c != 1) {
            do {
                uVar2 = (unsigned short)(uVar2 + 1);
                if ((unsigned short)(uVar5 + uVar1) < uVar5) {
                    uVar8 = (unsigned short)(uVar8 + 1);
                }
                param_2[uVar6 + 0x18] = uVar2;
                param_2[uVar6 + 0x19] = uVar8;
                bVar10 = (sVar7 != 0);
                uVar5 = (unsigned short)(uVar5 + uVar1);
                sVar7 = (short)(sVar7 - 1);
                uVar6 = (unsigned short)(uVar6 + 2);
            } while (bVar10);
        }
    }
    else {
        uVar8 = 0;
        uVar6 = 0;
        param_2[7] = DAT_00a07cd0;
        uVar2 = 0x8000;
        sVar7 = (short)(DAT_00a07cd0 - 2);
        uVar5 = 2;
        if (DAT_00a07cd0 != 1) {
            do {
                uVar6 = (unsigned short)(uVar6 + 1);
                if ((unsigned short)(uVar2 + uVar1) < uVar2) {
                    uVar8 = (unsigned short)(uVar8 + 1);
                }
                param_2[uVar5 + 0x18] = uVar8;
                param_2[uVar5 + 0x19] = uVar6;
                bVar10 = (sVar7 != 0);
                uVar2 = (unsigned short)(uVar2 + uVar1);
                sVar7 = (short)(sVar7 - 1);
                uVar5 = (unsigned short)(uVar5 + 2);
            } while (bVar10);
        }
    }
    uVar2 = param_2[7];
    uVar5 = (unsigned short)(param_2[0xc] == 0);
    uVar6 = 0;
    while (uVar2 != 0) {
        uVar8 = 1;
        uVar3 = (unsigned int)uVar5;
        while ((uVar2 = (unsigned short)(uVar2 - 1), uVar2 != 0 && (param_2[uVar5 + 0x1a] == param_2[uVar3 + 0x18]))) {
            uVar8 = (unsigned short)(uVar8 + 1);
            uVar5 = (unsigned short)(uVar5 + 2);
        }
        uVar5 = (unsigned short)(uVar5 + 2);
        param_2[uVar6 + 0xc9a] = uVar8;
        uVar6 = (unsigned short)(uVar6 + 1);
    }
    param_2[0xc99] = uVar6;
    uVar2 = (unsigned short)(param_1 + 0x4000) & 0x7fff;
    if (0x3fff < uVar2) {
        uVar2 = (unsigned short)(0x8000 - uVar2);
    }
    if (uVar2 < DAT_00a080f2) {
        sVar7 = 0;
        param_2[0xd] = 0;
        if (DAT_005235f8 == 1) {
            sVar11 = 100;
        }
        else {
            sVar11 = 0x5b;
        }
    }
    else {
        sVar7 = 4;
        uVar2 = (unsigned short)(0x4000 - uVar2);
        param_2[0xd] = 4;
        if (DAT_005235f8 == 1) {
            sVar11 = 100;
        }
        else {
            sVar11 = 0x6e;
        }
    }
    puVar9 = param_2 + 0xe;
    uVar2 = FUN_00421980(uVar2, sVar11);
    *puVar9 = uVar2;
    param_2[0xf] = 0;
    if ((sVar7 == local_8) && (DAT_005235f8 == 0)) {
        uVar2 = *param_2;
        if (((uVar2 < 0xe000) && (0x9fff < uVar2)) || ((uVar2 < 0x6000) && (0x1fff < uVar2))) {
            if ((int)(unsigned int)*puVar9 == 0) {
                RaiseException(0xC0000094UL, 0, 0, NULL);
            }
            div_lo = (unsigned int)(0x1000000LL / (long long)(int)(unsigned int)*puVar9);
            uVar3 = div_lo & 0xffff;
            *puVar9 = (unsigned short)(uVar3 << 8);
            param_2[0xf] = (unsigned short)(uVar3 >> 8);
            param_2[0xc98] = uVar1 >> 8;
        }
        else {
            *puVar9 = 0x100;
            param_2[0xf] = 0x100;
            param_2[0xc98] = (unsigned short)(unsigned char)(((unsigned int)uVar1 * 0xd800) >> 0x18);
        }
    }
    else {
        param_2[0xc98] = (unsigned short)(unsigned char)(((unsigned int)*puVar9 * (unsigned int)uVar1 + 0x800000) >> 0x18);
    }
    param_2[0xb] = param_2[0xc] | param_2[8] | param_2[9];
    param_2[10] = (unsigned short)((param_2[9] >> 1) + param_2[8]);
    uVar2 = param_2[7];
    param_2[0x10] = param_2[0x18];
    param_2[0x14] = param_2[0x19];
    uVar6 = (unsigned short)(DAT_009ed230 - param_2[0x19]);
    param_2[0x12] = uVar6;
    uVar1 = param_2[(unsigned int)uVar2 * 2 + 0x16];
    param_2[0x11] = uVar1;
    uVar5 = param_2[(unsigned int)uVar2 * 2 + 0x17];
    param_2[0x15] = uVar5;
    uVar5 = (unsigned short)(DAT_009ed230 - uVar5);
    uVar1 = (unsigned short)(uVar1 - param_2[0x18]);
    param_2[0x13] = uVar5;
    param_2[0x16] = uVar1;
    if ((short)uVar1 < 0) {
        param_2[0x16] = (unsigned short)(-uVar1);
    }
    uVar5 = (unsigned short)(uVar5 - uVar6);
    param_2[0x17] = uVar5;
    if ((short)uVar5 < 0) {
        param_2[0x17] = (unsigned short)(-uVar5);
    }
    if (DAT_005233d8 == 2) {
        uVar1 = 0;
        if (uVar2 != 0) {
            do {
                uVar3 = (unsigned int)uVar1;
                uVar2 = param_2[uVar3 * 2 + 0x18];
                if (param_2[9] != 0) {
                    uVar2 = (unsigned short)(-uVar2);
                }
                uVar5 = param_2[uVar3 * 2 + 0x19];
                if (param_2[8] != 0) {
                    uVar5 = (unsigned short)(-uVar5);
                }
                if (DAT_009e95f2 < 1) {
                    iVar4 = (int)((unsigned int)(int)((short)uVar2 * 2) - (unsigned int)(int)(short)uVar5 * (unsigned int)DAT_009a8d50);
                }
                else {
                    iVar4 = (int)((unsigned int)(int)(short)uVar5 * (unsigned int)DAT_009a8d50 + (unsigned int)(int)((short)uVar2 * 2));
                }
                uVar1 = (unsigned short)(uVar1 + 1);
                *(int *)(param_2 + uVar3 * 2 + 0x12dc) = iVar4;
            } while (uVar1 < param_2[7]);
            return;
        }
    }
    else {
        uVar1 = 0;
        if (uVar2 != 0) {
            do {
                uVar3 = (unsigned int)uVar1;
                uVar2 = param_2[uVar3 * 2 + 0x18];
                if (param_2[9] != 0) {
                    uVar2 = (unsigned short)(-uVar2);
                }
                uVar5 = param_2[uVar3 * 2 + 0x19];
                if (param_2[8] != 0) {
                    uVar5 = (unsigned short)(-uVar5);
                }
                if (DAT_009e95f2 < 1) {
                    iVar4 = (int)((unsigned int)(int)(short)uVar2 - (unsigned int)(int)(short)uVar5 * (unsigned int)DAT_009a8d50);
                }
                else {
                    iVar4 = (int)((unsigned int)(int)(short)uVar2 + (unsigned int)(int)(short)uVar5 * (unsigned int)DAT_009a8d50);
                }
                uVar1 = (unsigned short)(uVar1 + 1);
                *(int *)(param_2 + uVar3 * 2 + 0x12dc) = iVar4;
            } while (uVar1 < param_2[7]);
        }
    }
    return;
}

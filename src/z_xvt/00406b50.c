/* SPDX-License-Identifier: MIT */
/* z_xvt 00406b50 */
extern unsigned short DAT_00732e42;
extern int DAT_00525738;
extern int DAT_00525518;
extern unsigned short DAT_00523738[];
extern unsigned short DAT_00523620[];
extern unsigned short DAT_00523850[];
extern unsigned short DAT_0071ed00;
extern unsigned short DAT_0070acec;
extern unsigned short DAT_00732a20;
extern unsigned short DAT_0071f2e0;
extern short DAT_0071b6b2;
extern int DAT_006db6d0;

unsigned short __cdecl FUN_004183f0(unsigned short a);
void __stdcall RaiseException(DWORD, DWORD, DWORD, const DWORD *);

void __cdecl FUN_00406b50(unsigned short param_1, unsigned short *param_2)
{
    unsigned short uVar1;
    unsigned short uVar2;
    unsigned int uVar3;
    int iVar4;
    unsigned short uVar5;
    unsigned short *puVar6;
    unsigned short uVar7;
    short sVar8;
    unsigned short uVar9;
    unsigned short uVar10;
    int bVar11;
    short local_8;
    unsigned int q;

    *param_2 = param_1;
    uVar1 = 0;
    param_2[2] = param_1 & 0x8000;
    param_2[4] = (unsigned short)(param_1 + 0x4000) & 0x8000;
    uVar9 = 0;
    if (param_1 < 0x8000) {
        uVar2 = 0;
        if (param_1 < 0x4000) goto LAB_00406bad;
    } else {
        param_1 = param_1 & 0x7fff;
        uVar1 = 1;
        uVar2 = 1;
        if (0x3fff < param_1) goto LAB_00406bad;
    }
    uVar1 = uVar2;
    uVar9 = 2;
LAB_00406bad:
    param_2[8] = uVar1;
    param_2[9] = uVar9;
    uVar9 = param_1;
    if (0x3fff < param_1) {
        uVar9 = (unsigned short)(0x8000 - param_1);
    }
    uVar1 = FUN_004183f0(uVar9);
    param_2[1] = uVar1;
    uVar1 = FUN_004183f0((unsigned short)(uVar9 + 0x4000));
    param_2[3] = uVar1;
    if (uVar9 < DAT_00732e42) {
        local_8 = 0;
        param_2[0xc] = 0;
        if (DAT_00525738 == 1) {
            uVar1 = DAT_00523738[(unsigned int)(uVar9 >> 6)];
        } else {
            uVar1 = DAT_00523620[(unsigned int)(uVar9 >> 6)];
        }
    } else {
        local_8 = 4;
        param_2[0xc] = 4;
        uVar9 = (unsigned short)(0x4000 - uVar9);
        if (DAT_00525738 == 1) {
            uVar1 = DAT_00523738[(unsigned int)(uVar9 >> 6)];
        } else {
            uVar1 = DAT_00523850[(unsigned int)(uVar9 >> 6)];
        }
    }
    uVar2 = FUN_004183f0((unsigned short)(uVar9 + 0x4000));
    param_2[5] = uVar2;
    if (uVar2 == 0) {
        RaiseException(0xC0000094UL, 0, 0, NULL);
    }
    q = 0x80000000u / (unsigned int)uVar2;
    uVar9 = (unsigned short)q;
    if ((local_8 != 0) && (DAT_00525738 == 0)) {
        uVar9 = (unsigned short)(((unsigned int)DAT_0071ed00 * (q & 0xffff) + 0x8000) >> 0x10);
    }
    param_2[6] = uVar9;
    uVar9 = 0;
    param_2[0x18] = 0;
    param_2[0x19] = 0;
    if (param_2[0xc] == 0) {
        uVar5 = 0;
        param_2[7] = DAT_0070acec;
        uVar2 = 0x8000;
        uVar7 = 2;
        sVar8 = (short)(DAT_0070acec - 2);
        if (DAT_0070acec != 1) {
            do {
                uVar9 = (unsigned short)(uVar9 + 1);
                if ((unsigned short)(uVar2 + uVar1) < uVar2) {
                    uVar5 = (unsigned short)(uVar5 + 1);
                }
                param_2[uVar7 + 0x18] = uVar9;
                param_2[uVar7 + 0x19] = uVar5;
                bVar11 = (sVar8 != 0);
                uVar2 = (unsigned short)(uVar2 + uVar1);
                uVar7 = (unsigned short)(uVar7 + 2);
                sVar8 = (short)(sVar8 - 1);
            } while (bVar11);
        }
    } else {
        uVar7 = 0;
        uVar10 = 0;
        param_2[7] = DAT_00732a20;
        uVar2 = 0x8000;
        uVar5 = 2;
        uVar9 = DAT_00732a20;
        while (uVar9 = (unsigned short)(uVar9 - 1), uVar9 != 0) {
            uVar10 = (unsigned short)(uVar10 + 1);
            if ((unsigned short)(uVar2 + uVar1) < uVar2) {
                uVar7 = (unsigned short)(uVar7 + 1);
            }
            param_2[uVar5 + 0x18] = uVar7;
            param_2[uVar5 + 0x19] = uVar10;
            uVar2 = (unsigned short)(uVar2 + uVar1);
            uVar5 = (unsigned short)(uVar5 + 2);
        }
    }
    uVar9 = param_2[7];
    uVar3 = (unsigned int)(param_2[0xc] == 0);
    uVar2 = 0;
    while (uVar9 != 0) {
        uVar5 = (unsigned short)uVar3;
        uVar7 = 1;
        iVar4 = (int)(uVar3 + 0x18);
        while ((uVar9 = (unsigned short)(uVar9 - 1), uVar9 != 0) &&
               (uVar5 = (unsigned short)uVar3, param_2[uVar3 + 0x1a] == param_2[iVar4])) {
            uVar7 = (unsigned short)(uVar7 + 1);
            uVar5 = (unsigned short)(uVar5 + 2);
            uVar3 = (unsigned int)uVar5;
        }
        uVar3 = (unsigned int)(unsigned short)(uVar5 + 2);
        param_2[uVar2 + 0xc9a] = uVar7;
        uVar2 = (unsigned short)(uVar2 + 1);
    }
    uVar9 = (unsigned short)(param_1 + 0x4000) & 0x7fff;
    param_2[0xc99] = uVar2;
    if (0x3fff < uVar9) {
        uVar9 = (unsigned short)(0x8000 - uVar9);
    }
    if (uVar9 < DAT_00732e42) {
        sVar8 = 0;
        param_2[0xd] = 0;
        if (DAT_00525738 == 1) {
            uVar9 = DAT_00523738[(unsigned int)(uVar9 >> 6)];
        } else {
            uVar9 = DAT_00523620[(unsigned int)(uVar9 >> 6)];
        }
    } else {
        sVar8 = 4;
        param_2[0xd] = 4;
        if (DAT_00525738 == 1) {
            uVar9 = DAT_00523738[(unsigned int)((unsigned short)(0x4000 - uVar9) >> 6)];
        } else {
            uVar9 = DAT_00523850[(unsigned int)((unsigned short)(0x4000 - uVar9) >> 6)];
        }
    }
    puVar6 = param_2 + 0xe;
    param_2[0xf] = 0;
    *puVar6 = uVar9;
    if ((sVar8 == local_8) && (DAT_00525738 == 0)) {
        uVar9 = *param_2;
        if (((uVar9 < 0xe000) && (0x9fff < uVar9)) || ((uVar9 < 0x6000) && (0x1fff < uVar9))) {
            if (*puVar6 == 0) {
                RaiseException(0xC0000094UL, 0, 0, NULL);
            }
            uVar3 = (0x1000000u / (unsigned int)*puVar6) & 0xffff;
            *puVar6 = (unsigned short)(uVar3 << 8);
            param_2[0xf] = (unsigned short)(uVar3 >> 8);
            param_2[0xc98] = (unsigned short)(uVar1 >> 8);
        } else {
            *puVar6 = 0x100;
            param_2[0xf] = 0x100;
            param_2[0xc98] = (unsigned short)(unsigned char)(((unsigned int)uVar1 * 0xd800u) >> 0x18);
        }
    } else {
        param_2[0xc98] = (unsigned short)(unsigned char)(((unsigned int)*puVar6 * (unsigned int)uVar1 + 0x800000u) >> 0x18);
    }
    param_2[0xb] = param_2[0xc] | param_2[8] | param_2[9];
    param_2[10] = (unsigned short)((param_2[9] >> 1) + param_2[8]);
    uVar9 = param_2[7];
    param_2[0x10] = param_2[0x18];
    param_2[0x14] = param_2[0x19];
    uVar7 = (unsigned short)(DAT_0071f2e0 - param_2[0x19]);
    param_2[0x12] = uVar7;
    uVar1 = param_2[(unsigned int)uVar9 * 2 + 0x16];
    param_2[0x11] = uVar1;
    uVar2 = param_2[(unsigned int)uVar9 * 2 + 0x17];
    param_2[0x15] = uVar2;
    uVar2 = (unsigned short)(DAT_0071f2e0 - uVar2);
    uVar1 = (unsigned short)(uVar1 - param_2[0x18]);
    param_2[0x13] = uVar2;
    param_2[0x16] = uVar1;
    if ((short)uVar1 < 0) {
        param_2[0x16] = (unsigned short)(0u - uVar1);
    }
    uVar2 = (unsigned short)(uVar2 - uVar7);
    param_2[0x17] = uVar2;
    if ((short)uVar2 < 0) {
        param_2[0x17] = (unsigned short)(0u - uVar2);
    }
    if (DAT_00525518 == 2) {
        uVar1 = 0;
        if (uVar9 != 0) {
            do {
                uVar3 = (unsigned int)uVar1;
                uVar9 = param_2[uVar3 * 2 + 0x18];
                if (param_2[9] != 0) {
                    uVar9 = (unsigned short)(0u - uVar9);
                }
                uVar2 = param_2[uVar3 * 2 + 0x19];
                if (param_2[8] != 0) {
                    uVar2 = (unsigned short)(0u - uVar2);
                }
                if (DAT_0071b6b2 < 1) {
                    iVar4 = (int)((unsigned int)((int)(short)uVar9 * 2) -
                                  (unsigned int)((int)(short)uVar2) * (unsigned int)DAT_006db6d0);
                } else {
                    iVar4 = (int)((unsigned int)((int)(short)uVar2) * (unsigned int)DAT_006db6d0 +
                                  (unsigned int)((int)(short)uVar9 * 2));
                }
                uVar1 = (unsigned short)(uVar1 + 1);
                *(int *)(param_2 + uVar3 * 2 + 0x12dc) = iVar4;
            } while (uVar1 < param_2[7]);
            return;
        }
    } else {
        uVar1 = 0;
        if (uVar9 != 0) {
            do {
                uVar3 = (unsigned int)uVar1;
                uVar9 = param_2[uVar3 * 2 + 0x18];
                if (param_2[9] != 0) {
                    uVar9 = (unsigned short)(0u - uVar9);
                }
                uVar2 = param_2[uVar3 * 2 + 0x19];
                if (param_2[8] != 0) {
                    uVar2 = (unsigned short)(0u - uVar2);
                }
                if (DAT_0071b6b2 < 1) {
                    iVar4 = (int)((unsigned int)(int)(short)uVar9 -
                                  (unsigned int)((int)(short)uVar2) * (unsigned int)DAT_006db6d0);
                } else {
                    iVar4 = (int)((unsigned int)(int)(short)uVar9 +
                                  (unsigned int)((int)(short)uVar2) * (unsigned int)DAT_006db6d0);
                }
                uVar1 = (unsigned short)(uVar1 + 1);
                *(int *)(param_2 + uVar3 * 2 + 0x12dc) = iVar4;
            } while (uVar1 < param_2[7]);
        }
    }
    return;
}

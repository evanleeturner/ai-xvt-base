/* SPDX-License-Identifier: MIT */
/* z_bop 00411810 */
extern char DAT_009a7b70[];
extern char DAT_009a7b74[];
extern short DAT_00a00498;
extern short DAT_00a004a0;
extern short DAT_009a8c12;
extern short DAT_00a08244;
extern short DAT_00a08242;
extern short DAT_009e95f0;
extern unsigned int DAT_005505ec;
extern int DAT_005505f0;
extern unsigned char DAT_00bb7380[];
extern unsigned char DAT_0051a888[];
extern short DAT_009a73e8;
extern short DAT_009a73f2;
extern short DAT_009d8c06;
extern short DAT_009a739c;
extern short DAT_009a739e;

unsigned int __cdecl FUN_004acb90(unsigned int *, unsigned int *, unsigned int *);
int __cdecl FUN_004a4e70(void);
void __cdecl FUN_004a4ea0(short *, short *);
int __cdecl FUN_004aa7f0(void);
unsigned int __cdecl FUN_004aa870(void);

unsigned int __cdecl FUN_00411810(int param_1)
{
    unsigned char bVar1;
    short sVar2;
    int iVar3;
    unsigned int uVar4;
    unsigned int uVar5;
    unsigned int uVar6;
    unsigned int uVar7;
    unsigned int uVar8;
    short sVar9;
    short local_1a;
    short local_18;
    short local_16;
    unsigned int local_10;
    int local_c;
    unsigned int local_8;
    unsigned int local_4;

    uVar8 = 0;
    if (-1 < param_1) {
        DAT_00a00498 = (short)*(char *)(DAT_009a7b70 + (unsigned int)param_1 * 6u + 3u);
        iVar3 = (int)((unsigned int)param_1 * 6u);
        DAT_00a004a0 = (short)*(char *)(DAT_009a7b74 + iVar3);
        uVar8 = (((((unsigned int)param_1 * 3u) >> 8) << 8) |
                 (unsigned int)*(unsigned char *)(DAT_009a7b70 + iVar3 + 2)) & 0xffff00ffu;
        DAT_009a8c12 = (short)uVar8;
        DAT_00a08244 = (short)(*(unsigned char *)(DAT_009a7b74 + iVar3 + 1) & 3);
        return uVar8;
    }
    local_16 = 0;
    local_10 = 0;
    local_4 = 0;
    local_8 = 0;
    if (DAT_00a08242 != 0) {
        uVar8 = FUN_004acb90(&local_8, &local_4, &local_10);
    }
    if (DAT_009e95f0 != 0) {
        iVar3 = FUN_004a4e70();
        local_16 = (short)iVar3;
        FUN_004a4ea0(&local_1a, &local_18);
    }
    uVar4 = 0;
    iVar3 = FUN_004aa7f0();
    if (iVar3 != 0) {
        uVar4 = FUN_004aa870();
        uVar4 = uVar4 & 0xffff;
    }
    sVar9 = 0;
    uVar7 = 1;
    iVar3 = 0;
    local_c = 0;
    do {
        bVar1 = DAT_00bb7380[iVar3];
        uVar6 = uVar4;
        if (bVar1 != 0) {
            if ((uVar8 & uVar7) == 0) {
                if ((uVar7 & DAT_005505ec) != 0) {
                    if (bVar1 == 0xb2) {
                        uVar5 = 0xb2;
                    }
                    else if ((bVar1 < 0xb3) || (0xbb < bVar1)) {
                        uVar5 = 0;
                    }
                    else {
                        uVar5 = 0xba;
                    }
                    if ((uVar5 != 0) && (uVar6 = uVar5, (short)uVar4 != 0)) {
                        uVar8 = uVar8 | uVar7;
                        uVar6 = uVar4;
                    }
                }
            }
            else {
                if (bVar1 == 0x9c) {
                    local_c = 1;
                }
                else if (bVar1 == 0x9d) {
                    sVar9 = 1;
                }
                if ((uVar7 & DAT_005505ec) == 0) {
                    if ((short)uVar4 == 0) {
                        uVar6 = (unsigned int)bVar1;
                    }
                    else {
                        uVar8 = uVar8 & ~uVar7;
                    }
                }
            }
        }
        DAT_009a8c12 = (short)(unsigned short)uVar6;
        uVar7 = uVar7 * 2;
        iVar3 = iVar3 + 1;
        uVar4 = uVar6;
    } while (iVar3 < 0x14);
    DAT_005505ec = uVar8;
    sVar2 = (short)local_c;
    iVar3 = DAT_005505f0;
    if (DAT_009a8c12 == 0) {
        local_c = (int)((unsigned int)(int)(char)local_10 + 0x80u);
        iVar3 = local_c;
        if (DAT_005505f0 != -1) {
            int d = (int)((unsigned int)local_c - (unsigned int)DAT_005505f0);
            iVar3 = (int)((unsigned int)DAT_005505f0 +
                          (unsigned int)((int)((unsigned int)d + ((unsigned int)(d >> 0x1f) & 3U)) >> 2));
            {
                int t = (int)((unsigned int)iVar3 + 8u);
                local_c = (int)((unsigned int)t + ((unsigned int)(t >> 0x1f) & 0xfU)) >> 4;
            }
            {
                int t2 = (int)((unsigned int)DAT_005505f0 + 8u);
                if (local_c != ((int)((unsigned int)t2 + ((unsigned int)(t2 >> 0x1f) & 0xfU)) >> 4)) {
                    if (local_c < 0) {
                        local_c = 0;
                    }
                    if (0x10 < local_c) {
                        local_c = 0x10;
                    }
                    DAT_009a8c12 = (short)(unsigned short)DAT_0051a888[-local_c];
                }
            }
        }
    }
    DAT_005505f0 = iVar3;
    DAT_009a73e8 = 0;
    DAT_009a73f2 = 0;
    DAT_00a00498 = (short)local_8;
    DAT_00a004a0 = (short)local_4;
    DAT_00a08244 = (short)((unsigned int)(int)sVar2 + (unsigned int)((int)sVar9 * 2));
    DAT_009d8c06 = local_16;
    DAT_009a739c = local_1a;
    DAT_009a739e = local_18;
    return (((unsigned int)local_c >> 0x10) << 16) | (unsigned int)(unsigned short)DAT_009a8c12;
}

/* SPDX-License-Identifier: MIT */
/* z_bop 004f8b30 */
extern unsigned char DAT_00a91d12[];
extern unsigned char DAT_00a91d17[];
extern int _DAT_00669214;
extern short DAT_00a92142[];
extern short DAT_00a9216e[];
extern int DAT_0052c908[];
extern unsigned char DAT_0052cab0[];
extern unsigned char DAT_0052cab4[];
extern unsigned char DAT_0052cab8[];
extern unsigned char DAT_0052cabc[];
extern int DAT_00aa5c18[];
extern int DAT_00aa5bf8[];
extern unsigned int DAT_00aa5c10[];
extern int DAT_00aa5c28[];
extern char s_greyicon_0052d014[];

void FUN_004f7b10(int *param_1, int param_2, int param_3, short *param_4, short *param_5);
void FUN_004d6460(int *param_1, int param_2, int param_3, int param_4, int param_5);
void FUN_004d64c0(int *param_1, int param_2, int param_3);
void FUN_004d67e0(int *param_1, int param_2, int param_3, short param_4, int param_5);
void FUN_004d6950(int *param_1, int param_2, int param_3, short param_4);
void FUN_004b52c0(char *param_1, int *param_2, int param_3, int param_4, unsigned int param_5);

void __cdecl FUN_004f8b30(int *param_1, unsigned int param_2, short param_3, short param_4)
{
    unsigned int *puVar1;
    unsigned char bVar2;
    int iVar3;
    int iVar4;
    short sVar5;
    short sVar6;
    short sVar7;
    int iVar8;
    short local_1a;
    short local_18;
    short local_16;
    int local_14;
    int local_10[4];

    iVar4 = (int)param_3;
    bVar2 = DAT_00a91d12[iVar4 * 0x562];
    iVar3 = (int)((unsigned int)iVar4 * 0x2b1u + (unsigned int)_DAT_00669214);
    switch (DAT_00a91d17[iVar4 * 0x562]) {
    case 0:
        sVar7 = 0;
        break;
    case 1:
    case 4:
        sVar7 = 8;
        break;
    case 2:
        sVar7 = 0x18;
        break;
    case 3:
        sVar7 = 0x10;
        break;
    case 5:
        sVar7 = 0x20;
        break;
    default:
        sVar7 = 0;
    }
    FUN_004f7b10(param_1, DAT_00a92142[iVar3], DAT_00a9216e[iVar3], &local_16, &local_18);
    iVar8 = (int)((unsigned int)DAT_0052c908[(short)(unsigned short)bVar2] * 0x10u);
    iVar3 = (*(int *)(DAT_0052cab8 + iVar8) - *(int *)(DAT_0052cab0 + iVar8)) + 1;
    iVar4 = (*(int *)(DAT_0052cabc + iVar8) - *(int *)(DAT_0052cab4 + iVar8)) + 1;
    local_16 = (short)((unsigned int)(int)local_16 - (unsigned int)(int)(short)(iVar3 >> 1));
    local_18 = (short)((unsigned int)(int)local_18 - (unsigned int)(int)(short)(iVar4 >> 1));
    FUN_004d6460(local_10, (int)local_16, (int)local_18,
                 (int)((unsigned int)iVar3 + (unsigned int)(int)local_16 - 1u),
                 (int)((unsigned int)iVar4 + (unsigned int)(int)local_18 - 1u));
    if (param_4 < 0xc) {
        if (param_4 < 4) {
            sVar6 = param_4 + 1;
            local_1a = 0x10;
            sVar5 = (short)(sVar7 + param_4 * -2 + 7);
        } else {
            sVar5 = sVar7 + 1;
            if (param_4 < 8) {
                local_1a = (short)((0xb - param_4) * 2);
                sVar6 = 4;
            } else {
                local_1a = (short)((0xb - param_4) * 2);
                sVar6 = 0xc - param_4;
            }
        }
        if (7 < param_4) {
            local_14 = (int)param_4;
            FUN_004d64c0(local_10, 0xb - local_14, 0xb - local_14);
            FUN_004d67e0(local_10, 0, 0, (short)DAT_00aa5c18[sVar7], 1);
            FUN_004d6950(local_10, 0, 0, (short)DAT_00aa5bf8[sVar7 + local_14]);
        }
        if (0 < sVar6) {
            local_14 = (int)sVar6;
            do {
                iVar3 = (int)sVar5;
                iVar4 = (int)local_1a;
                sVar5 = sVar5 + 2;
                puVar1 = &DAT_00aa5c10[iVar3];
                FUN_004b52c0(s_greyicon_0052d014, (int *)(DAT_0052cab0 + iVar8),
                             local_16 - iVar4, local_18 - iVar4, *puVar1);
                FUN_004b52c0(s_greyicon_0052d014, (int *)(DAT_0052cab0 + iVar8),
                             local_16 + iVar4, local_18 - iVar4, *puVar1);
                FUN_004b52c0(s_greyicon_0052d014, (int *)(DAT_0052cab0 + iVar8),
                             local_16 - iVar4, local_18 + iVar4, *puVar1);
                FUN_004b52c0(s_greyicon_0052d014, (int *)(DAT_0052cab0 + iVar8),
                             local_16 + iVar4, local_18 + iVar4, *puVar1);
                local_1a = local_1a - 2;
                local_14 = (int)(short)((short)local_14 - 1);
            } while ((short)local_14 != 0);
            return;
        }
    } else {
        FUN_004d64c0(local_10, -2, -2);
        FUN_004d67e0(local_10, 0, 0, (short)DAT_00aa5c18[sVar7], 1);
        FUN_004d6950(local_10, 0, 0, (short)DAT_00aa5c28[sVar7]);
    }
    return;
}

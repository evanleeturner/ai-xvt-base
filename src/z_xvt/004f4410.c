/* SPDX-License-Identifier: MIT */
/* z_xvt 004f4410 */
extern unsigned char DAT_007438d2[];
extern unsigned char DAT_007438d7[];
extern int DAT_00650434;
extern short DAT_00743d02[];
extern short DAT_00743d2e[];
extern int DAT_00533800[];
extern char DAT_005339a8[];
extern int DAT_007577b8[];
extern unsigned int DAT_007577d0[];
extern int DAT_007577d8[];
extern int DAT_007577e8[];
extern char s_greyicon_00533ea0[];

void __cdecl FUN_004f33c0(int *, int, int, short *, short *);
void __cdecl FUN_004dcfd0(int *, int, int, int, int);
void __cdecl FUN_004dd070(int *, int, int);
void __cdecl FUN_004dd3e0(int *, int, int, short, int);
void __cdecl FUN_004dd570(int *, int, int, short);
void __cdecl FUN_004d0b60(char *, int *, int, int, unsigned int);

void __cdecl FUN_004f4410(int *param_1, unsigned int param_2, short param_3, short param_4)
{
    unsigned int *puVar1;
    unsigned char bVar2;
    int iVar3;
    int iVar4;
    short sVar5;
    short sVar6;
    int iVar7;
    short sStack_18;
    short sStack_16;
    short sStack_14;
    short sStack_12;
    int aiStack_10[4];

    iVar4 = (int)param_3;
    bVar2 = DAT_007438d2[iVar4 * 0x562];
    iVar3 = (int)((unsigned int)iVar4 * 0x2b1u + (unsigned int)DAT_00650434);
    switch (DAT_007438d7[iVar4 * 0x562]) {
    case 0:
        sVar6 = 0;
        break;
    case 1:
    case 4:
        sVar6 = 8;
        break;
    case 2:
        sVar6 = 0x18;
        break;
    case 3:
        sVar6 = 0x10;
        break;
    case 5:
        sVar6 = 0x20;
        break;
    default:
        sVar6 = 0;
    }
    FUN_004f33c0(param_1, DAT_00743d02[iVar3], DAT_00743d2e[iVar3], &sStack_12, &sStack_14);
    iVar7 = (int)((unsigned int)DAT_00533800[(short)(unsigned short)bVar2] * 0x10u);
    iVar3 = (int)(((unsigned int)*(int *)(DAT_005339a8 + iVar7 + 8) - (unsigned int)*(int *)(DAT_005339a8 + iVar7)) + 1u);
    iVar4 = (int)(((unsigned int)*(int *)(DAT_005339a8 + iVar7 + 0xc) - (unsigned int)*(int *)(DAT_005339a8 + iVar7 + 4)) + 1u);
    sStack_12 = (short)(sStack_12 - (short)(iVar3 >> 1));
    sStack_14 = (short)(sStack_14 - (short)(iVar4 >> 1));
    FUN_004dcfd0(aiStack_10, (int)sStack_12, (int)sStack_14,
                 (int)((unsigned int)iVar3 + (unsigned int)(int)sStack_12 - 1u),
                 (int)((unsigned int)iVar4 + (unsigned int)(int)sStack_14 - 1u));
    if (param_4 < 0xc) {
        if (param_4 < 4) {
            sVar5 = (short)(param_4 + 1);
            sStack_16 = 0x10;
            sStack_18 = (short)(sVar6 + param_4 * -2 + 7);
        } else {
            sStack_18 = (short)(sVar6 + 1);
            if (param_4 < 8) {
                sStack_16 = (short)((0xb - param_4) * 2);
                sVar5 = 4;
            } else {
                sStack_16 = (short)((0xb - param_4) * 2);
                sVar5 = (short)(0xc - param_4);
            }
        }
        if (7 < param_4) {
            iVar3 = 0xb - param_4;
            FUN_004dd070(aiStack_10, iVar3, iVar3);
            FUN_004dd3e0(aiStack_10, 0, 0, (short)DAT_007577d8[sVar6], 1);
            FUN_004dd570(aiStack_10, 0, 0, (short)DAT_007577b8[(int)sVar6 + (int)param_4]);
        }
        if (0 < sVar5) {
            do {
                puVar1 = &DAT_007577d0[sStack_18];
                iVar3 = (int)sStack_16;
                FUN_004d0b60(s_greyicon_00533ea0, (int *)(DAT_005339a8 + iVar7), sStack_12 - iVar3,
                             sStack_14 - iVar3, *puVar1);
                FUN_004d0b60(s_greyicon_00533ea0, (int *)(DAT_005339a8 + iVar7), sStack_12 + iVar3,
                             sStack_14 - iVar3, *puVar1);
                FUN_004d0b60(s_greyicon_00533ea0, (int *)(DAT_005339a8 + iVar7), sStack_12 - iVar3,
                             sStack_14 + iVar3, *puVar1);
                FUN_004d0b60(s_greyicon_00533ea0, (int *)(DAT_005339a8 + iVar7), sStack_12 + iVar3,
                             sStack_14 + iVar3, *puVar1);
                sStack_18 = (short)(sStack_18 + 2);
                sStack_16 = (short)(sStack_16 + -2);
                sVar5 = (short)(sVar5 + -1);
            } while (sVar5 != 0);
            return;
        }
    } else {
        FUN_004dd070(aiStack_10, -2, -2);
        FUN_004dd3e0(aiStack_10, 0, 0, (short)DAT_007577d8[sVar6], 1);
        FUN_004dd570(aiStack_10, 0, 0, (short)DAT_007577e8[sVar6]);
    }
    return;
}

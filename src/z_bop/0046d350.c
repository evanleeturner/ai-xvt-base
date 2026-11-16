/* SPDX-License-Identifier: MIT */
/* z_bop 0046d350 */
extern int DAT_009997d0;
extern unsigned char DAT_009997d4[0x200];
extern unsigned int DAT_009999d4;
extern int DAT_009999d8;
extern int DAT_009999dc;
extern unsigned char DAT_009999e0[0x200];
extern unsigned int DAT_00999be0;
extern int DAT_00999be4;
extern int DAT_009994f0;
extern int DAT_00999548;
extern int *DAT_009994c4;
extern unsigned int DAT_0099f6e0;
extern char DAT_00bb736f;

extern unsigned int DAT_00999bf0[];
extern unsigned int DAT_00999bf8[];
extern unsigned int DAT_00999c00[];
extern int DAT_00999c04[];
extern unsigned char DAT_00999c08[];
extern unsigned int DAT_00999e08[];

extern int DAT_005ee1b8[];
extern int DAT_005ee1bc[];
extern unsigned char DAT_005ee1c0[];
extern unsigned char DAT_005ee1c4[];
extern unsigned char DAT_005ee1c5[];
extern unsigned char DAT_005ee1c6[];
extern unsigned char DAT_005ee1c8[];
extern int DAT_00527020;

extern int DAT_005dd9b8[];
extern int DAT_005dd9bc[];
extern unsigned char DAT_005dd9c0[];
extern unsigned char DAT_005dd9c4[];
extern unsigned char DAT_005dd9c5[];
extern unsigned char DAT_005dd9c6[];
extern unsigned char DAT_005dd9c8[];
extern int DAT_0052701c;

extern int DAT_005599a8[];
extern int DAT_005599ac[];
extern int DAT_005599b0[];
extern unsigned char DAT_005599b4[];
extern unsigned char DAT_005599b5[];
extern unsigned char DAT_005599b6[];
extern unsigned char DAT_005599b7[];
extern unsigned char DAT_005599b8[];
extern int DAT_00527024;
extern int DAT_0052702c;

int __cdecl FUN_0046fad0(void);
unsigned int __cdecl FUN_0046fee0(int param_1);
void __cdecl FUN_00470060(void);

bool __cdecl FUN_0046d350(int param_1, unsigned int *param_2, unsigned int param_3);

bool __cdecl FUN_0046d350(int param_1, unsigned int *param_2, unsigned int param_3)
{
    unsigned int uVar1;
    short sVar2;
    unsigned char bVar3;
    int iVar4;
    int iVar5;
    unsigned int uVar6;
    unsigned int uVar7;
    unsigned int uVar8;
    unsigned int uVar9;
    unsigned short uVar10;
    unsigned char *puVar11;
    unsigned char *puVar12;
    unsigned char *puVar13;
    unsigned char *puVar14;
    bool bVar15;
    int local_420;
    unsigned char *local_41c;
    int local_404;
    unsigned short local_400[0x200];

    local_404 = 0;
    if ((int)param_3 < 4) {
        return 0;
    }
    uVar1 = *param_2;
    bVar3 = (unsigned char)uVar1 & 0x7f;
    uVar10 = (unsigned short)bVar3;
    if ((uVar1 < 0x3c) || (0x3f < uVar1)) {
        bVar15 = 1;
    } else {
        bVar15 = 0;
    }
    sVar2 = (short)param_3;

    if ((uVar1 == 1) && (DAT_00bb736f == 1)) {
        uVar10 = (unsigned short)((((unsigned short)DAT_009999dc & 0x7f) << 8) | 0x8081);
        DAT_009999dc = (int)((unsigned int)DAT_009999dc + 1u);
        if (0x7f < DAT_009999dc) {
            DAT_009999dc = 0;
        }
        local_41c = (unsigned char *)&local_400[1];
        local_420 = 2;
        local_400[0] = uVar10;
        if ((bVar15) && (iVar4 = FUN_0046fad0(), iVar4 == 0)) {
            local_400[1] = (unsigned short)(short)((unsigned int)(int)sVar2 - 4u);
            local_41c = (unsigned char *)&local_400[2];
            local_420 = 4;
        }
        uVar8 = param_3 - 4;
        puVar13 = (unsigned char *)(param_2 + 1);
        puVar14 = local_41c;
        for (uVar6 = uVar8 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
            puVar14[0] = puVar13[0]; puVar14[1] = puVar13[1];
            puVar14[2] = puVar13[2]; puVar14[3] = puVar13[3];
            puVar13 = puVar13 + 4;
            puVar14 = puVar14 + 4;
        }
        for (uVar6 = uVar8 & 3; uVar7 = DAT_00999be0, uVar6 != 0; uVar6 = uVar6 - 1) {
            *puVar14 = *puVar13;
            puVar13 = puVar13 + 1;
            puVar14 = puVar14 + 1;
        }
        puVar12 = local_41c + (param_3 - 4);
        local_420 = (int)((param_3 - 4) + (unsigned int)local_420);
        if (bVar15) {
            if (DAT_00999be4 == 0) {
                puVar11 = DAT_009999e0;
                for (uVar6 = DAT_00999be0 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
                    puVar12[0] = puVar11[0]; puVar12[1] = puVar11[1];
                    puVar12[2] = puVar11[2]; puVar12[3] = puVar11[3];
                    puVar11 = puVar11 + 4;
                    puVar12 = puVar12 + 4;
                }
                for (uVar7 = uVar7 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
                    *puVar12 = *puVar11;
                    puVar11 = puVar11 + 1;
                    puVar12 = puVar12 + 1;
                }
                local_420 = (int)((unsigned int)local_420 + DAT_00999be0);
            } else {
                *puVar12 = 0x39;
                DAT_00999be4 = 0;
                local_420 = (int)((unsigned int)local_420 + 1u);
            }
        }
        puVar13 = (unsigned char *)(param_2 + 1);
        puVar14 = DAT_009999e0 + 1;
        DAT_009999e0[0] = bVar3;
        for (uVar6 = uVar8 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
            puVar14[0] = puVar13[0]; puVar14[1] = puVar13[1];
            puVar14[2] = puVar13[2]; puVar14[3] = puVar13[3];
            puVar13 = puVar13 + 4;
            puVar14 = puVar14 + 4;
        }
        DAT_00999be0 = param_3 - 3;
        for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
            *puVar14 = *puVar13;
            puVar13 = puVar13 + 1;
            puVar14 = puVar14 + 1;
        }
    }
    else if (param_1 == 0) {
        uVar10 = (unsigned short)(uVar10 | (((unsigned short)DAT_009997d0 & 0x7f) << 8));
        DAT_009997d0 = (int)((unsigned int)DAT_009997d0 + 1u);
        if (0x7f < DAT_009997d0) {
            DAT_009997d0 = 0;
        }
        local_41c = (unsigned char *)&local_400[1];
        local_420 = 2;
        local_400[0] = uVar10;
        if ((bVar15) && (iVar4 = FUN_0046fad0(), iVar4 == 0)) {
            local_400[1] = (unsigned short)(short)((unsigned int)(int)sVar2 - 4u);
            local_41c = (unsigned char *)&local_400[2];
            local_420 = 4;
        }
        uVar8 = param_3 - 4;
        puVar13 = (unsigned char *)(param_2 + 1);
        puVar14 = local_41c;
        for (uVar6 = uVar8 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
            puVar14[0] = puVar13[0]; puVar14[1] = puVar13[1];
            puVar14[2] = puVar13[2]; puVar14[3] = puVar13[3];
            puVar13 = puVar13 + 4;
            puVar14 = puVar14 + 4;
        }
        for (uVar6 = uVar8 & 3; uVar7 = DAT_009999d4, uVar6 != 0; uVar6 = uVar6 - 1) {
            *puVar14 = *puVar13;
            puVar13 = puVar13 + 1;
            puVar14 = puVar14 + 1;
        }
        puVar12 = local_41c + (param_3 - 4);
        local_420 = (int)((param_3 - 4) + (unsigned int)local_420);
        if (bVar15) {
            if (DAT_009999d8 == 0) {
                puVar11 = DAT_009997d4;
                for (uVar6 = DAT_009999d4 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
                    puVar12[0] = puVar11[0]; puVar12[1] = puVar11[1];
                    puVar12[2] = puVar11[2]; puVar12[3] = puVar11[3];
                    puVar11 = puVar11 + 4;
                    puVar12 = puVar12 + 4;
                }
                for (uVar7 = uVar7 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
                    *puVar12 = *puVar11;
                    puVar11 = puVar11 + 1;
                    puVar12 = puVar12 + 1;
                }
                local_420 = (int)((unsigned int)local_420 + DAT_009999d4);
            } else {
                *puVar12 = 0x39;
                DAT_009999d8 = 0;
                local_420 = (int)((unsigned int)local_420 + 1u);
            }
        }
        puVar13 = (unsigned char *)(param_2 + 1);
        puVar14 = DAT_009997d4 + 1;
        DAT_009997d4[0] = bVar3;
        for (uVar6 = uVar8 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
            puVar14[0] = puVar13[0]; puVar14[1] = puVar13[1];
            puVar14[2] = puVar13[2]; puVar14[3] = puVar13[3];
            puVar13 = puVar13 + 4;
            puVar14 = puVar14 + 4;
        }
        DAT_009999d4 = param_3 - 3;
        for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
            *puVar14 = *puVar13;
            puVar13 = puVar13 + 1;
            puVar14 = puVar14 + 1;
        }
    }
    else if (DAT_009994f0 == param_1) {
        uVar10 = (unsigned short)(uVar10 | (((unsigned short)DAT_009999dc & 0x7f) << 8) | 0x8080);
        DAT_009999dc = (int)((unsigned int)DAT_009999dc + 1u);
        if (0x7f < DAT_009999dc) {
            DAT_009999dc = 0;
        }
        local_41c = (unsigned char *)&local_400[1];
        local_420 = 2;
        local_400[0] = uVar10;
        if ((bVar15) && (iVar4 = FUN_0046fad0(), iVar4 == 0)) {
            local_400[1] = (unsigned short)(short)((unsigned int)(int)sVar2 - 4u);
            local_41c = (unsigned char *)&local_400[2];
            local_420 = 4;
        }
        uVar8 = param_3 - 4;
        puVar13 = (unsigned char *)(param_2 + 1);
        puVar14 = local_41c;
        for (uVar6 = uVar8 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
            puVar14[0] = puVar13[0]; puVar14[1] = puVar13[1];
            puVar14[2] = puVar13[2]; puVar14[3] = puVar13[3];
            puVar13 = puVar13 + 4;
            puVar14 = puVar14 + 4;
        }
        for (uVar6 = uVar8 & 3; uVar7 = DAT_00999be0, uVar6 != 0; uVar6 = uVar6 - 1) {
            *puVar14 = *puVar13;
            puVar13 = puVar13 + 1;
            puVar14 = puVar14 + 1;
        }
        puVar12 = local_41c + (param_3 - 4);
        local_420 = (int)((param_3 - 4) + (unsigned int)local_420);
        if (bVar15) {
            if (DAT_00999be4 == 0) {
                puVar11 = DAT_009999e0;
                for (uVar6 = DAT_00999be0 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
                    puVar12[0] = puVar11[0]; puVar12[1] = puVar11[1];
                    puVar12[2] = puVar11[2]; puVar12[3] = puVar11[3];
                    puVar11 = puVar11 + 4;
                    puVar12 = puVar12 + 4;
                }
                for (uVar7 = uVar7 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
                    *puVar12 = *puVar11;
                    puVar11 = puVar11 + 1;
                    puVar12 = puVar12 + 1;
                }
                local_420 = (int)((unsigned int)local_420 + DAT_00999be0);
            } else {
                *puVar12 = 0x39;
                DAT_00999be4 = 0;
                local_420 = (int)((unsigned int)local_420 + 1u);
            }
        }
        puVar13 = (unsigned char *)(param_2 + 1);
        puVar14 = DAT_009999e0 + 1;
        DAT_009999e0[0] = bVar3;
        for (uVar6 = uVar8 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
            puVar14[0] = puVar13[0]; puVar14[1] = puVar13[1];
            puVar14[2] = puVar13[2]; puVar14[3] = puVar13[3];
            puVar13 = puVar13 + 4;
            puVar14 = puVar14 + 4;
        }
        DAT_00999be0 = param_3 - 3;
        for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
            *puVar14 = *puVar13;
            puVar13 = puVar13 + 1;
            puVar14 = puVar14 + 1;
        }
    }
    else {
        uVar8 = FUN_0046fee0(param_1);
        if ((uVar8 < DAT_0099f6e0) && (uVar8 < 0x28)) {
            iVar4 = DAT_00999c04[uVar8 * 0x8eu];
            iVar5 = (int)((unsigned int)iVar4 + 1u);
            DAT_00999c04[uVar8 * 0x8eu] = iVar5;
            uVar10 = (unsigned short)(uVar10 | (((unsigned short)iVar4 & 0x7f) << 8));
            if (0x7f < iVar5) {
                DAT_00999c04[uVar8 * 0x8eu] = 0;
            }
        }
        uVar10 = (unsigned short)(uVar10 | 0x8000);
        local_41c = (unsigned char *)&local_400[1];
        local_420 = 2;
        local_400[0] = uVar10;
        if ((bVar15) && (iVar4 = FUN_0046fad0(), iVar4 == 0)) {
            local_400[1] = (unsigned short)(short)((unsigned int)(int)sVar2 - 4u);
            local_41c = (unsigned char *)&local_400[2];
            local_420 = 4;
        }
        uVar6 = param_3 - 4;
        puVar13 = (unsigned char *)(param_2 + 1);
        puVar14 = local_41c;
        for (uVar7 = uVar6 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
            puVar14[0] = puVar13[0]; puVar14[1] = puVar13[1];
            puVar14[2] = puVar13[2]; puVar14[3] = puVar13[3];
            puVar13 = puVar13 + 4;
            puVar14 = puVar14 + 4;
        }
        for (uVar7 = uVar6 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
            *puVar14 = *puVar13;
            puVar13 = puVar13 + 1;
            puVar14 = puVar14 + 1;
        }
        local_420 = (int)((param_3 - 4) + (unsigned int)local_420);
        if (bVar15) {
            uVar7 = DAT_00999e08[uVar8 * 0x8eu];
            puVar12 = DAT_00999c08 + uVar8 * 0x238u;
            puVar11 = local_41c + (param_3 - 4);
            for (uVar9 = uVar7 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
                puVar11[0] = puVar12[0]; puVar11[1] = puVar12[1];
                puVar11[2] = puVar12[2]; puVar11[3] = puVar12[3];
                puVar12 = puVar12 + 4;
                puVar11 = puVar11 + 4;
            }
            for (uVar7 = uVar7 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
                *puVar11 = *puVar12;
                puVar12 = puVar12 + 1;
                puVar11 = puVar11 + 1;
            }
            local_420 = (int)((unsigned int)local_420 + DAT_00999e08[uVar8 * 0x8eu]);
        }
        DAT_00999c08[uVar8 * 0x238u] = bVar3;
        puVar13 = (unsigned char *)(param_2 + 1);
        puVar14 = DAT_00999c08 + uVar8 * 0x238u + 1;
        for (uVar7 = uVar6 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
            puVar14[0] = puVar13[0]; puVar14[1] = puVar13[1];
            puVar14[2] = puVar13[2]; puVar14[3] = puVar13[3];
            puVar13 = puVar13 + 4;
            puVar14 = puVar14 + 4;
        }
        for (uVar6 = uVar6 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
            *puVar14 = *puVar13;
            puVar13 = puVar13 + 1;
            puVar14 = puVar14 + 1;
        }
        DAT_00999e08[uVar8 * 0x8eu] = param_3 - 3;
    }

    if (((uVar1 != 1) || (DAT_00bb736f != 1)) && (DAT_00999548 != param_1)) {
        bVar3 = (unsigned char)(local_400[0] >> 8);
        if (uVar1 == 2) {
            puVar13 = (unsigned char *)param_2;
            puVar14 = DAT_005ee1c8 + (unsigned int)DAT_00527020 * 0x210u;
            for (uVar8 = param_3 >> 2; uVar8 != 0; uVar8 = uVar8 - 1) {
                puVar14[0] = puVar13[0]; puVar14[1] = puVar13[1];
                puVar14[2] = puVar13[2]; puVar14[3] = puVar13[3];
                puVar13 = puVar13 + 4;
                puVar14 = puVar14 + 4;
            }
            for (uVar8 = param_3 & 3; iVar4 = DAT_00527020, uVar8 != 0; uVar8 = uVar8 - 1) {
                *puVar14 = *puVar13;
                puVar13 = puVar13 + 1;
                puVar14 = puVar14 + 1;
            }
            iVar5 = (int)((unsigned int)DAT_00527020 * 0x210u);
            DAT_005ee1b8[(unsigned int)DAT_00527020 * 0x84u] = param_1;
            DAT_005ee1bc[(unsigned int)iVar4 * 0x84u] = (int)param_3;
            *(int *)(DAT_005ee1c0 + iVar5) = 0;
            DAT_005ee1c4[iVar5] = 0;
            DAT_005ee1c5[iVar5] = 0;
            DAT_005ee1c6[iVar5] = bVar3 & 0x7f;
            DAT_00527020 = (int)((unsigned int)DAT_00527020 + 1u);
            if (0xff < DAT_00527020) {
                DAT_00527020 = 0;
            }
        }
        puVar13 = (unsigned char *)param_2;
        puVar14 = DAT_005dd9c8 + (unsigned int)DAT_0052701c * 0x210u;
        for (uVar8 = param_3 >> 2; uVar8 != 0; uVar8 = uVar8 - 1) {
            puVar14[0] = puVar13[0]; puVar14[1] = puVar13[1];
            puVar14[2] = puVar13[2]; puVar14[3] = puVar13[3];
            puVar13 = puVar13 + 4;
            puVar14 = puVar14 + 4;
        }
        for (uVar8 = param_3 & 3; iVar4 = DAT_0052701c, uVar8 != 0; uVar8 = uVar8 - 1) {
            *puVar14 = *puVar13;
            puVar13 = puVar13 + 1;
            puVar14 = puVar14 + 1;
        }
        iVar5 = (int)((unsigned int)DAT_0052701c * 0x210u);
        DAT_005dd9b8[(unsigned int)DAT_0052701c * 0x84u] = param_1;
        DAT_005dd9bc[(unsigned int)iVar4 * 0x84u] = (int)param_3;
        *(int *)(DAT_005dd9c0 + iVar5) = 0;
        DAT_005dd9c4[iVar5] = 0;
        iVar4 = DAT_009994f0;
        if (param_1 == 0) {
            DAT_005dd9c5[iVar5] = 0;
        } else {
            DAT_005dd9c5[iVar5] = 2;
            if (iVar4 != param_1) {
                DAT_005dd9c5[iVar5] = 1;
            }
        }
        iVar4 = (int)((unsigned int)DAT_0052701c + 1u);
        DAT_0052701c = iVar4;
        DAT_005dd9c6[iVar5] = bVar3 & 0x7f;
        uVar10 = local_400[0];
        if (0x7f < iVar4) {
            DAT_0052701c = 0;
        }
    }

    if ((((DAT_00999548 == param_1) || (param_1 == 0)) ||
         ((DAT_009994c4 == (int *)0 || (DAT_009994f0 == param_1)))) &&
        ((DAT_0052702c < 0x400) || (FUN_00470060(), DAT_0052702c < 0x400))) {
        puVar14 = DAT_005599b8 + (unsigned int)DAT_00527024 * 0x210u;
        puVar13 = (unsigned char *)param_2;
        for (uVar8 = param_3 >> 2; uVar8 != 0; uVar8 = uVar8 - 1) {
            puVar14[0] = puVar13[0]; puVar14[1] = puVar13[1];
            puVar14[2] = puVar13[2]; puVar14[3] = puVar13[3];
            puVar13 = puVar13 + 4;
            puVar14 = puVar14 + 4;
        }
        for (uVar8 = param_3 & 3; iVar4 = DAT_00527024, uVar8 != 0; uVar8 = uVar8 - 1) {
            *puVar14 = *puVar13;
            puVar13 = puVar13 + 1;
            puVar14 = puVar14 + 1;
        }
        iVar5 = (int)((unsigned int)DAT_00527024 * 0x210u);
        DAT_005599a8[(unsigned int)DAT_00527024 * 0x84u] = DAT_00999548;
        DAT_005599ac[(unsigned int)iVar4 * 0x84u] = (int)param_3;
        DAT_005599b0[(unsigned int)iVar4 * 0x84u] = 0;
        DAT_005599b4[iVar5] = 0;
        DAT_005599b7[iVar5] = 0;
        uVar8 = FUN_0046fee0(DAT_00999548);
        if ((uVar1 == 1) && (DAT_00bb736f == 1)) {
            bVar15 = uVar8 < DAT_0099f6e0;
            DAT_005599b5[(unsigned int)DAT_00527024 * 0x210u] = 2;
            if (bVar15 < 0x28) {
                DAT_00999c00[uVar8 * 0x8eu] = (unsigned int)((uVar10 & 0x7f00) >> 8);
            }
        } else if (param_1 == 0) {
            bVar15 = uVar8 < DAT_0099f6e0;
            DAT_005599b5[(unsigned int)DAT_00527024 * 0x210u] = 0;
            if ((bVar15) && (uVar8 < 0x28)) {
                DAT_00999bf8[uVar8 * 0x8eu] = (unsigned int)((uVar10 & 0x7f00) >> 8);
            }
        } else if (DAT_009994f0 == param_1) {
            bVar15 = uVar8 < DAT_0099f6e0;
            DAT_005599b5[(unsigned int)DAT_00527024 * 0x210u] = 2;
            if (bVar15 < 0x28) {
                DAT_00999c00[uVar8 * 0x8eu] = (unsigned int)((uVar10 & 0x7f00) >> 8);
            }
        } else {
            bVar15 = uVar8 < DAT_0099f6e0;
            DAT_005599b5[(unsigned int)DAT_00527024 * 0x210u] = 1;
            if (bVar15 < 0x28) {
                DAT_00999bf0[uVar8 * 0x8eu] = (unsigned int)((uVar10 & 0x7f00) >> 8);
            }
        }
        DAT_0052702c = (int)((unsigned int)DAT_0052702c + 1u);
        DAT_005599b6[(unsigned int)DAT_00527024 * 0x210u] = (unsigned char)(local_400[0] >> 8) & 0x7f;
        DAT_00527024 = (int)((unsigned int)DAT_00527024 + 1u);
        if (0x3ff < DAT_00527024) {
            DAT_00527024 = 0;
        }
    }

    if (DAT_009994c4 != (int *)0) {
        if (DAT_00999548 != param_1) {
            local_404 = ((int (__stdcall *)(int *, int, int, int, void *, int))
                         *(int *)(*DAT_009994c4 + 0x68))
                            (DAT_009994c4, DAT_00999548, param_1, 0, local_400, local_420);
        }
        return local_404 == 0;
    }
    return 1;
}

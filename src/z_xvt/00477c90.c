/* SPDX-License-Identifier: MIT */
/* z_xvt 00477c90 */
extern int DAT_0052ae18;
extern int DAT_00525518;
extern unsigned short DAT_007091ae;
extern unsigned short DAT_006d4984;
extern unsigned char DAT_005f1d88[];
extern unsigned short DAT_007325e0[];
extern int DAT_00732f3c;
extern char DAT_00732e8f;

void __cdecl FUN_004604c0(int param_1, int param_2, unsigned int param_3, int param_4, int param_5, int param_6);
void __cdecl FUN_004a6a80(void);
void __cdecl FUN_004a6f90(void);
void __cdecl FUN_004774e0(int param_1, int param_2, int param_3, int param_4, float param_5);

void __cdecl FUN_00477c90(int param_1, int param_2, unsigned int param_3, int param_4, int param_5, int param_6)
{
    unsigned short uVar1;
    float fVar2;
    int iVar3;
    unsigned int uVar4;
    unsigned int uVar5;
    unsigned int uVar6;
    int iVar7;
    unsigned char *puVar8;
    unsigned int uVar9;
    unsigned char *puVar10;
    int local_14;

    iVar3 = (int)((unsigned int)param_4 + (unsigned int)param_2);
    if ((((0 < iVar3) && (uVar4 = param_3 + (unsigned int)param_1, 0 < (int)uVar4)) &&
         (param_1 < (int)(unsigned int)DAT_007091ae)) &&
        (((param_2 < (int)(unsigned int)DAT_006d4984 && (0 < param_4)) && (0 < (int)param_3)))) {
        if (DAT_0052ae18 != 0) {
            FUN_004604c0(param_1, param_2, param_3, param_4, param_5, param_6);
            return;
        }
        uVar9 = (unsigned int)((int)param_3 >> 3);
        local_14 = param_4 >> 3;
        if ((int)uVar9 < 3) {
            uVar9 = 3;
        }
        if (local_14 < 3) {
            local_14 = 3;
        }
        if ((int)param_3 < (int)uVar9) {
            uVar9 = param_3;
        }
        if (param_4 < local_14) {
            local_14 = param_4;
        }
        puVar8 = DAT_005f1d88;
        if (DAT_00525518 == 2) {
            if (0 < param_1) {
                puVar8 = DAT_005f1d88 + (unsigned int)param_1 * 2;
            }
            uVar1 = DAT_007325e0[param_5];
            if (0 < (int)uVar9) {
                unsigned int v32 = ((unsigned int)uVar1 << 16) | (unsigned int)uVar1;
                puVar10 = puVar8;
                for (uVar6 = uVar9 >> 1; uVar6 != 0; uVar6 = uVar6 - 1) {
                    __builtin_memcpy(puVar10, &v32, 4);
                    puVar10 = puVar10 + 4;
                }
                for (uVar6 = (unsigned int)((uVar9 & 1) != 0); uVar6 != 0; uVar6 = uVar6 - 1) {
                    __builtin_memcpy(puVar10, &uVar1, 2);
                    puVar10 = puVar10 + 2;
                }
            }
        } else {
            if (0 < param_1) {
                puVar8 = DAT_005f1d88 + param_1;
            }
            if (0 < (int)uVar9) {
                unsigned char b = (unsigned char)param_5;
                unsigned int v32 = ((unsigned int)b << 24) | ((unsigned int)b << 16) |
                                   ((unsigned int)b << 8) | (unsigned int)b;
                puVar10 = puVar8;
                for (uVar6 = uVar9 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
                    __builtin_memcpy(puVar10, &v32, 4);
                    puVar10 = puVar10 + 4;
                }
                for (uVar6 = uVar9 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
                    *puVar10 = b;
                    puVar10 = puVar10 + 1;
                }
            }
        }
        if (param_6 < 1) {
            param_6 = 1;
        }
        fVar2 = (float)DAT_00732f3c / (float)param_6;
        if (DAT_00732e8f == '\0') {
            FUN_004a6a80();
        }
        if (-1 < param_2) {
            uVar6 = uVar9 + (unsigned int)param_1;
            if ((0 < (int)uVar6) && (uVar5 = (unsigned int)DAT_007091ae, param_1 < (int)uVar5)) {
                iVar7 = param_1;
                if (param_1 < 0) {
                    iVar7 = 0;
                }
                if ((int)uVar5 < (int)uVar6) {
                    uVar6 = uVar5;
                }
                FUN_004774e0((int)puVar8, iVar7, (int)uVar6, param_2, fVar2);
            }
            iVar7 = (int)((param_3 - uVar9) + (unsigned int)param_1);
            if ((0 < (int)uVar4) && (uVar6 = (unsigned int)DAT_007091ae, iVar7 < (int)uVar6)) {
                if (iVar7 < 0) {
                    iVar7 = 0;
                }
                uVar5 = uVar4;
                if ((int)uVar6 < (int)uVar4) {
                    uVar5 = uVar6;
                }
                FUN_004774e0((int)puVar8, iVar7, (int)uVar5, param_2, fVar2);
            }
        }
        if (iVar3 <= (int)(unsigned int)DAT_006d4984) {
            uVar6 = uVar9 + (unsigned int)param_1;
            if ((0 < (int)uVar6) && (uVar5 = (unsigned int)DAT_007091ae, param_1 < (int)uVar5)) {
                iVar7 = param_1;
                if (param_1 < 0) {
                    iVar7 = 0;
                }
                if ((int)uVar5 < (int)uVar6) {
                    uVar6 = uVar5;
                }
                FUN_004774e0((int)puVar8, iVar7, (int)uVar6, (int)((unsigned int)iVar3 - 1u), fVar2);
            }
            iVar7 = (int)((param_3 - uVar9) + (unsigned int)param_1);
            if ((0 < (int)uVar4) && (uVar9 = (unsigned int)DAT_007091ae, iVar7 < (int)uVar9)) {
                if (iVar7 < 0) {
                    iVar7 = 0;
                }
                uVar6 = uVar4;
                if ((int)uVar9 < (int)uVar4) {
                    uVar6 = uVar9;
                }
                FUN_004774e0((int)puVar8, iVar7, (int)uVar6, (int)((unsigned int)iVar3 - 1u), fVar2);
            }
        }
        iVar3 = 1;
        if (1 < local_14) {
            do {
                iVar7 = (int)((unsigned int)param_2 + (unsigned int)iVar3);
                if ((-1 < iVar7) && (iVar7 < (int)(unsigned int)DAT_006d4984)) {
                    if (-1 < param_1) {
                        FUN_004774e0((int)puVar8, param_1, (int)((unsigned int)param_1 + 1u), iVar7, fVar2);
                    }
                    if ((int)uVar4 <= (int)(unsigned int)DAT_007091ae) {
                        FUN_004774e0((int)puVar8, (int)(uVar4 - 1u), (int)uVar4, iVar7, fVar2);
                    }
                }
                iVar3 = iVar3 + 1;
            } while (iVar3 < local_14);
        }
        for (iVar3 = (int)((unsigned int)param_4 - (unsigned int)local_14);
             iVar3 < (int)((unsigned int)param_4 - 1u); iVar3 = iVar3 + 1) {
            if (((local_14 <= iVar3) &&
                 (iVar7 = (int)((unsigned int)param_2 + (unsigned int)iVar3), -1 < iVar7)) &&
                (iVar7 < (int)(unsigned int)DAT_006d4984)) {
                if (-1 < param_1) {
                    FUN_004774e0((int)puVar8, param_1, (int)((unsigned int)param_1 + 1u), iVar7, fVar2);
                }
                if ((int)uVar4 <= (int)(unsigned int)DAT_007091ae) {
                    FUN_004774e0((int)puVar8, (int)(uVar4 - 1u), (int)uVar4, iVar7, fVar2);
                }
            }
        }
        if (DAT_00732e8f == '\0') {
            FUN_004a6f90();
        }
    }
    return;
}

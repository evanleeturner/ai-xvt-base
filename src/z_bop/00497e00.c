/* SPDX-License-Identifier: MIT */
/* z_bop 00497e00 */
extern int DAT_00527e90;
extern int DAT_005233d8;
extern unsigned short DAT_009d682c;
extern unsigned short DAT_009a1fe4;
extern unsigned char DAT_006122d8[];
extern unsigned short DAT_00a00530[];
extern int DAT_00a081ec;
extern char DAT_00a0813f;

void __cdecl FUN_0040c430(int a, int b, int c, int d, int e, int f);
void __cdecl FUN_004aba50(void);
void __cdecl FUN_00497940(int a, int b, int c, int d, float e);
void __cdecl FUN_004abe50(void);

void __cdecl FUN_00497e00(int param_1, int param_2, unsigned int param_3, int param_4, int param_5, int param_6)
{
    unsigned short uVar1;
    float fVar2;
    int iVar3;
    unsigned int uVar4;
    unsigned int uVar5;
    unsigned int uVar6;
    unsigned char *puVar7;
    unsigned int uVar8;
    unsigned char *puVar9;
    int iVar10;
    int local_14;

    iVar3 = (int)((unsigned int)param_4 + (unsigned int)param_2);
    if ((((0 < iVar3) && (uVar4 = param_3 + (unsigned int)param_1, 0 < (int)uVar4)) &&
         (param_1 < (int)(unsigned int)DAT_009d682c)) &&
        (((param_2 < (int)(unsigned int)DAT_009a1fe4 && (0 < param_4)) && (0 < (int)param_3)))) {
        if (DAT_00527e90 != 0) {
            FUN_0040c430(param_1, param_2, (int)param_3, param_4, param_5, param_6);
            return;
        }
        uVar8 = (unsigned int)((int)param_3 >> 3);
        local_14 = param_4 >> 3;
        if ((int)uVar8 < 3) {
            uVar8 = 3;
        }
        if (local_14 < 3) {
            local_14 = 3;
        }
        if ((int)param_3 < (int)uVar8) {
            uVar8 = param_3;
        }
        if (param_4 < local_14) {
            local_14 = param_4;
        }
        puVar7 = DAT_006122d8;
        if (DAT_005233d8 == 2) {
            if (0 < param_1) {
                puVar7 = DAT_006122d8 + (unsigned int)param_1 * 2;
            }
            uVar1 = DAT_00a00530[param_5];
            if (0 < (int)uVar8) {
                unsigned int pat = ((unsigned int)uVar1 << 16) | (unsigned int)uVar1;
                puVar9 = puVar7;
                for (uVar6 = uVar8 >> 1; uVar6 != 0; uVar6 = uVar6 - 1) {
                    __builtin_memcpy(puVar9, &pat, 4);
                    puVar9 = puVar9 + 4;
                }
                for (uVar6 = (unsigned int)((uVar8 & 1) != 0); uVar6 != 0; uVar6 = uVar6 - 1) {
                    __builtin_memcpy(puVar9, &uVar1, 2);
                    puVar9 = puVar9 + 2;
                }
            }
        } else {
            if (0 < param_1) {
                puVar7 = DAT_006122d8 + param_1;
            }
            if (0 < (int)uVar8) {
                unsigned char b = (unsigned char)param_5;
                unsigned int pat = ((unsigned int)b << 24) | ((unsigned int)b << 16) |
                                   ((unsigned int)b << 8) | (unsigned int)b;
                puVar9 = puVar7;
                for (uVar6 = uVar8 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
                    __builtin_memcpy(puVar9, &pat, 4);
                    puVar9 = puVar9 + 4;
                }
                for (uVar6 = uVar8 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
                    *puVar9 = b;
                    puVar9 = puVar9 + 1;
                }
            }
        }
        if (param_6 < 1) {
            param_6 = 1;
        }
        fVar2 = (float)DAT_00a081ec / (float)param_6;
        if (DAT_00a0813f == '\0') {
            FUN_004aba50();
        }
        if (-1 < param_2) {
            uVar6 = (unsigned int)param_1 + uVar8;
            if ((0 < (int)uVar6) && (uVar5 = (unsigned int)DAT_009d682c, param_1 < (int)uVar5)) {
                iVar10 = param_1;
                if (param_1 < 0) {
                    iVar10 = 0;
                }
                if ((int)uVar5 < (int)uVar6) {
                    uVar6 = uVar5;
                }
                FUN_00497940((int)puVar7, iVar10, (int)uVar6, param_2, fVar2);
            }
            iVar10 = (int)((param_3 - uVar8) + (unsigned int)param_1);
            if ((0 < (int)uVar4) && (uVar6 = (unsigned int)DAT_009d682c, iVar10 < (int)uVar6)) {
                if (iVar10 < 0) {
                    iVar10 = 0;
                }
                uVar5 = uVar4;
                if ((int)uVar6 < (int)uVar4) {
                    uVar5 = uVar6;
                }
                FUN_00497940((int)puVar7, iVar10, (int)uVar5, param_2, fVar2);
            }
        }
        if (iVar3 <= (int)(unsigned int)DAT_009a1fe4) {
            uVar6 = (unsigned int)param_1 + uVar8;
            if ((0 < (int)uVar6) && (uVar5 = (unsigned int)DAT_009d682c, param_1 < (int)uVar5)) {
                iVar10 = param_1;
                if (param_1 < 0) {
                    iVar10 = 0;
                }
                if ((int)uVar5 < (int)uVar6) {
                    uVar6 = uVar5;
                }
                FUN_00497940((int)puVar7, iVar10, (int)uVar6, (int)((unsigned int)iVar3 - 1u), fVar2);
            }
            iVar10 = (int)((param_3 - uVar8) + (unsigned int)param_1);
            if ((0 < (int)uVar4) && (uVar8 = (unsigned int)DAT_009d682c, iVar10 < (int)uVar8)) {
                if (iVar10 < 0) {
                    iVar10 = 0;
                }
                uVar6 = uVar4;
                if ((int)uVar8 < (int)uVar4) {
                    uVar6 = uVar8;
                }
                FUN_00497940((int)puVar7, iVar10, (int)uVar6, (int)((unsigned int)iVar3 - 1u), fVar2);
            }
        }
        iVar3 = 1;
        if (1 < local_14) {
            do {
                iVar10 = (int)((unsigned int)param_2 + (unsigned int)iVar3);
                if ((-1 < iVar10) && (iVar10 < (int)(unsigned int)DAT_009a1fe4)) {
                    if (-1 < param_1) {
                        FUN_00497940((int)puVar7, param_1, (int)((unsigned int)param_1 + 1u), iVar10, fVar2);
                    }
                    if ((int)uVar4 <= (int)(unsigned int)DAT_009d682c) {
                        FUN_00497940((int)puVar7, (int)(uVar4 - 1u), (int)uVar4, iVar10, fVar2);
                    }
                }
                iVar3 = iVar3 + 1;
            } while (iVar3 < local_14);
        }
        for (iVar3 = (int)((unsigned int)param_4 - (unsigned int)local_14);
             iVar3 < (int)((unsigned int)param_4 - 1u); iVar3 = iVar3 + 1) {
            if (((local_14 <= iVar3) &&
                 (iVar10 = (int)((unsigned int)param_2 + (unsigned int)iVar3), -1 < iVar10)) &&
                (iVar10 < (int)(unsigned int)DAT_009a1fe4)) {
                if (-1 < param_1) {
                    FUN_00497940((int)puVar7, param_1, (int)((unsigned int)param_1 + 1u), iVar10, fVar2);
                }
                if ((int)uVar4 <= (int)(unsigned int)DAT_009d682c) {
                    FUN_00497940((int)puVar7, (int)(uVar4 - 1u), (int)uVar4, iVar10, fVar2);
                }
            }
        }
        if (DAT_00a0813f == '\0') {
            FUN_004abe50();
        }
    }
    return;
}

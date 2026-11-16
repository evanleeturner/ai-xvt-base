/* SPDX-License-Identifier: MIT */
/* z_xvt 004dd570 */
extern int DAT_00758830;
extern int DAT_0075964b;   /* odd address: read via memcpy */
extern int DAT_00759657;   /* odd address: read via memcpy */

void __cdecl FUN_004dd000(int *param_1, int *param_2);
void __cdecl FUN_004dd030(int *param_1, int param_2, int param_3);
void __cdecl FUN_004dd0b0(int *param_1);

void __cdecl FUN_004dd570(int *param_1, int param_2, int param_3, unsigned short param_4)
{
    int iVar1;
    unsigned int uVar2;
    unsigned int uVar3;
    unsigned char uVar4;
    unsigned char *puVar5;
    unsigned short *puVar6;
    unsigned char *puVar7;
    unsigned short *puVar8;
    int local_54[4];   /* local_54, local_50, local_4c, local_48 */
    int local_44[4];   /* local_44, local_40, local_3c, local_38 */
    int local_18;
    int local_14;
    int local_10;
    int tmp_DAT_0075964b;
    int tmp_DAT_00759657;
    unsigned int uFill;

    local_10 = 1;
    local_18 = 1;
    local_14 = 1;
    if ((*param_1 < param_1[2]) && (param_1[1] < param_1[3])) {
        FUN_004dd000(local_44, param_1);
        FUN_004dd030(local_44, param_2, param_3);
        if ((local_44[0] < 0x281) &&
            (((-1 < local_44[2]) && (local_44[1] < 0x1e1)) && (-1 < local_44[3]))) {
            FUN_004dd000(local_54, local_44);
            FUN_004dd0b0(local_44);
            __builtin_memcpy(&tmp_DAT_0075964b, &DAT_0075964b, 4);
            iVar1 = tmp_DAT_0075964b;
            if (local_54[0] != local_44[0]) {
                local_18 = 0;
            }
            if (local_54[2] != local_44[2]) {
                local_14 = 0;
            }
            if (local_54[3] != local_44[3]) {
                local_10 = 0;
            }
            local_44[2] = (int)((unsigned int)local_44[2] - (unsigned int)local_44[0]);
            uVar2 = (unsigned int)local_44[2] + 1u;
            __builtin_memcpy(&tmp_DAT_00759657, &DAT_00759657, 4);
            if (tmp_DAT_00759657 == 8) {
                puVar5 = (unsigned char *)((unsigned int)local_44[1] * (unsigned int)tmp_DAT_0075964b
                                           + (unsigned int)local_44[0]
                                           + (unsigned int)DAT_00758830);
                uVar4 = (unsigned char)param_4;
                uFill = (unsigned int)uVar4 * 0x01010101u;
                if (local_54[1] == local_44[1]) {
                    puVar7 = puVar5;
                    for (uVar3 = uVar2 >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
                        __builtin_memcpy(puVar7, &uFill, 4);
                        puVar7 = puVar7 + 4;
                    }
                    for (uVar3 = uVar2 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
                        *puVar7 = uVar4;
                        puVar7 = puVar7 + 1;
                    }
                    puVar5 = puVar5 + iVar1;
                }
                if (local_44[1] + 1 < local_44[3]) {
                    local_44[3] = (int)((unsigned int)local_44[3] - ((unsigned int)local_44[1] + 1u));
                    do {
                        if (local_18 != 0) {
                            *puVar5 = uVar4;
                        }
                        if (local_14 != 0) {
                            *(puVar5 + local_44[2]) = uVar4;
                        }
                        puVar5 = puVar5 + iVar1;
                        local_44[3] = local_44[3] + -1;
                    } while (local_44[3] != 0);
                }
                if (local_10 != 0) {
                    for (uVar3 = uVar2 >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
                        __builtin_memcpy(puVar5, &uFill, 4);
                        puVar5 = puVar5 + 4;
                    }
                    for (uVar2 = uVar2 & 3; uVar2 != 0; uVar2 = uVar2 - 1) {
                        *puVar5 = uVar4;
                        puVar5 = puVar5 + 1;
                    }
                    return;
                }
            }
            else {
                if (tmp_DAT_00759657 != 0x10) {
                    return;
                }
                puVar6 = (unsigned short *)((unsigned int)local_44[1] * (unsigned int)tmp_DAT_0075964b
                                            + (unsigned int)local_44[0] * 2u
                                            + (unsigned int)DAT_00758830);
                if (local_54[1] == local_44[1]) {
                    uVar3 = uVar2;
                    puVar8 = puVar6;
                    if (0 < (int)uVar2) {
                        for (; uVar3 != 0; uVar3 = uVar3 - 1) {
                            *puVar8 = param_4;
                            puVar8 = puVar8 + 1;
                        }
                    }
                    puVar6 = (unsigned short *)((unsigned char *)puVar6 + iVar1);
                }
                if (local_44[1] + 1 < local_44[3]) {
                    local_44[3] = (int)((unsigned int)local_44[3] - ((unsigned int)local_44[1] + 1u));
                    do {
                        if (local_18 != 0) {
                            *puVar6 = param_4;
                        }
                        if (local_14 != 0) {
                            puVar6[local_44[2]] = param_4;
                        }
                        puVar6 = (unsigned short *)((unsigned char *)puVar6 + iVar1);
                        local_44[3] = local_44[3] + -1;
                    } while (local_44[3] != 0);
                }
                if ((local_10 != 0) && (0 < (int)uVar2)) {
                    for (; uVar2 != 0; uVar2 = uVar2 - 1) {
                        *puVar6 = param_4;
                        puVar6 = puVar6 + 1;
                    }
                }
            }
        }
    }
    return;
}

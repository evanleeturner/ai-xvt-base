/* SPDX-License-Identifier: MIT */
/* z_bop 004d6950 */
extern int DAT_00aa6cfc;
extern int DAT_00aa7b13;   /* odd address: packed, read via memcpy */
extern int DAT_00aa7b1f;   /* odd address: packed, read via memcpy */

void __cdecl FUN_004d6480(int *param_1, int *param_2);
void __cdecl FUN_004d64a0(int *param_1, int param_2, int param_3);
void __cdecl FUN_004d64e0(int *param_1);

void __cdecl FUN_004d6950(int *param_1, int param_2, int param_3, unsigned short param_4)
{
    unsigned int uVar2;
    unsigned int uVar3;
    unsigned char *puVar4;
    unsigned short *puVar5;
    unsigned char uVar6;
    unsigned char *puVar7;
    unsigned short *puVar8;
    int iVar1;
    int iVar9;
    unsigned int dwFill;
    int local_54[4];   /* local_54, local_50, local_4c, local_48 */
    int local_44[4];   /* local_44, local_40, local_3c, local_38 */
    int local_18;
    int local_14;
    int local_10;

    local_10 = 1;
    local_18 = 1;
    local_14 = 1;
    if ((*param_1 < param_1[2]) && (param_1[1] < param_1[3])) {
        FUN_004d6480(local_44, param_1);
        FUN_004d64a0(local_44, param_2, param_3);
        if ((local_44[0] < 0x281) &&
            (((-1 < local_44[2]) && (local_44[1] < 0x1e1)) && (-1 < local_44[3]))) {
            FUN_004d6480(local_54, local_44);
            FUN_004d64e0(local_44);
            __builtin_memcpy(&iVar1, (char *)&DAT_00aa7b13, 4);
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
            __builtin_memcpy(&iVar9, (char *)&DAT_00aa7b1f, 4);
            if (iVar9 == 8) {
                puVar4 = (unsigned char *)(unsigned int)
                    ((unsigned int)iVar1 * (unsigned int)local_44[1] +
                     (unsigned int)local_44[0] + (unsigned int)DAT_00aa6cfc);
                uVar6 = (unsigned char)param_4;
                dwFill = ((unsigned int)uVar6 << 24) | ((unsigned int)uVar6 << 16) |
                         ((unsigned int)uVar6 << 8) | (unsigned int)uVar6;
                if (local_54[1] == local_44[1]) {
                    puVar7 = puVar4;
                    for (uVar3 = uVar2 >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
                        __builtin_memcpy(puVar7, &dwFill, 4);
                        puVar7 = puVar7 + 4;
                    }
                    for (uVar3 = uVar2 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
                        *puVar7 = uVar6;
                        puVar7 = puVar7 + 1;
                    }
                    puVar4 = puVar4 + iVar1;
                }
                if ((int)((unsigned int)local_44[1] + 1u) < local_44[3]) {
                    local_44[3] = (int)((unsigned int)local_44[3] -
                                        ((unsigned int)local_44[1] + 1u));
                    do {
                        if (local_18 != 0) {
                            *puVar4 = uVar6;
                        }
                        if (local_14 != 0) {
                            *(unsigned char *)((unsigned int)local_44[2] + (unsigned int)puVar4) = uVar6;
                        }
                        puVar4 = puVar4 + iVar1;
                        local_44[3] = local_44[3] - 1;
                    } while (local_44[3] != 0);
                }
                if (local_10 != 0) {
                    for (uVar3 = uVar2 >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
                        __builtin_memcpy(puVar4, &dwFill, 4);
                        puVar4 = puVar4 + 4;
                    }
                    for (uVar2 = uVar2 & 3; uVar2 != 0; uVar2 = uVar2 - 1) {
                        *puVar4 = uVar6;
                        puVar4 = puVar4 + 1;
                    }
                    return;
                }
            }
            else {
                if (iVar9 != 0x10) {
                    return;
                }
                puVar5 = (unsigned short *)(unsigned int)
                    ((unsigned int)iVar1 * (unsigned int)local_44[1] +
                     (unsigned int)local_44[0] * 2u + (unsigned int)DAT_00aa6cfc);
                if (local_54[1] == local_44[1]) {
                    uVar3 = uVar2;
                    puVar8 = puVar5;
                    if (0 < (int)uVar2) {
                        for (; uVar3 != 0; uVar3 = uVar3 - 1) {
                            *puVar8 = param_4;
                            puVar8 = puVar8 + 1;
                        }
                    }
                    puVar5 = (unsigned short *)((unsigned char *)puVar5 + iVar1);
                }
                if ((int)((unsigned int)local_44[1] + 1u) < local_44[3]) {
                    local_44[3] = (int)((unsigned int)local_44[3] -
                                        ((unsigned int)local_44[1] + 1u));
                    do {
                        if (local_18 != 0) {
                            *puVar5 = param_4;
                        }
                        if (local_14 != 0) {
                            puVar5[local_44[2]] = param_4;
                        }
                        puVar5 = (unsigned short *)((unsigned char *)puVar5 + iVar1);
                        local_44[3] = local_44[3] - 1;
                    } while (local_44[3] != 0);
                }
                if ((local_10 != 0) && (0 < (int)uVar2)) {
                    for (; uVar2 != 0; uVar2 = uVar2 - 1) {
                        *puVar5 = param_4;
                        puVar5 = puVar5 + 1;
                    }
                }
            }
        }
    }
    return;
}

/* SPDX-License-Identifier: MIT */
/* z_bop 004b4d90 */
uint __cdecl FUN_004d64e0(int *param_1);
void __cdecl FUN_004d6480(int *param_1, int *param_2);

extern int DAT_00aa6cfc;
extern int DAT_00aa7b13;   /* packed, odd address: read via memcpy */
extern int DAT_00aa7b1f;   /* packed, odd address: read via memcpy */
extern char DAT_00aa7b23;

uint __cdecl FUN_004b4d90(int *param_1, int param_2, int param_3)
{
    ushort uVar1;
    uint uVar2;
    int iVar3;
    uint uVar4;
    ushort *puVar5;
    int iVar6;
    int iVar7;
    int iVar8;
    int local_2c;
    ushort *local_28;
    int local_24;
    int rectA[4];   /* local_20, local_1c, local_18, local_14 */
    int rectB[4];   /* local_10, local_c, local_8, local_4 */
    int tmpA;
    int tmpB;
    int tmpC;

    if (param_1 == (int *)0) {
        return 0;
    }
    rectA[0] = param_2;
    rectA[1] = param_3;
    rectA[2] = (int)((uint)*param_1 + (uint)param_2 + 0xffffffffu);
    rectA[3] = (int)((uint)param_1[1] + (uint)param_3 + 0xffffffffu);
    FUN_004d6480(&rectB[0], &rectA[0]);
    uVar2 = FUN_004d64e0(&rectA[0]);
    if ((rectA[0] <= rectA[2]) && (rectA[1] <= rectA[3])) {
        rectA[0] = (int)((uint)rectA[0] - (uint)rectB[0]);
        rectA[1] = (int)((uint)rectA[1] - (uint)rectB[1]);
        iVar6 = *param_1;
        iVar3 = (int)((uint)(int)((uint)(int)((uint)rectA[2] - (uint)rectA[0]) - (uint)rectB[2]) + (uint)iVar6);
        local_24 = (int)((uint)(int)((uint)(int)((uint)param_1[1] - (uint)rectB[3]) - (uint)rectA[1]) + (uint)rectA[3]);
        __builtin_memcpy(&tmpA, &DAT_00aa7b1f, 4);
        if (tmpA == 8) {
            __builtin_memcpy(&tmpB, &DAT_00aa7b13, 4);
            iVar6 = (int)((uint)(int)((uint)rectA[1] * (uint)iVar6) + (uint)param_1[4] + (uint)rectA[0]);
            iVar7 = (int)((uint)(int)((uint)(int)((uint)rectA[1] + (uint)param_3) * (uint)tmpB)
                          + (uint)rectA[0] + (uint)param_2 + (uint)DAT_00aa6cfc);
            if (0 < local_24) {
                do {
                    iVar8 = 0;
                    if (0 < iVar3) {
                        do {
                            if (*(char *)(iVar6 + iVar8) != '\0') {
                                *(char *)(iVar7 + iVar8) = *(char *)(iVar6 + iVar8);
                            }
                            iVar8 = iVar8 + 1;
                        } while (iVar8 < iVar3);
                    }
                    __builtin_memcpy(&tmpB, &DAT_00aa7b13, 4);
                    iVar7 = (int)((uint)iVar7 + (uint)tmpB);
                    iVar6 = (int)((uint)iVar6 + (uint)*param_1);
                    local_24 = local_24 - 1;
                } while (local_24 != 0);
                return uVar2;
            }
        }
        else {
            if (tmpA != 0x10) {
                return uVar2;
            }
            __builtin_memcpy(&tmpB, &DAT_00aa7b13, 4);
            local_2c = (int)((uint)(int)((uint)rectA[1] * (uint)iVar6) + (uint)param_1[4] + (uint)rectA[0]);
            local_28 = (ushort *)((uint)(int)((uint)(int)((uint)rectA[1] + (uint)param_3) * (uint)tmpB)
                                  + (uint)(int)((uint)(int)((uint)rectA[0] + (uint)param_2) * 2u)
                                  + (uint)DAT_00aa6cfc);
            if (DAT_00aa7b23 == '\0') {
                if (0 < local_24) {
                    do {
                        iVar6 = 0;
                        puVar5 = local_28;
                        if (0 < iVar3) {
                            do {
                                if (*(unsigned char *)(local_2c + iVar6) != 0) {
                                    uVar1 = *puVar5;
                                    uVar4 = (uint)param_1[*(unsigned char *)(local_2c + iVar6) + 5];
                                    uVar4 = (((uint)(uVar1 & 0x1f) + (((uint)(uVar1 & 0x7e0) + (uint)(uVar1 & 0xf800) * 4u) * 8u)) >> 1)
                                          + ((((uVar4 & 0xf800u) * 0x20u) + (uVar4 & 0x7e0u) * 8u + (uVar4 & 0x1fu)) >> 1);
                                    *puVar5 = (ushort)(((ushort)(uVar4 >> 5) & 0xf800)
                                                       + ((ushort)(uVar4 >> 3) & 0x7e0)
                                                       + ((ushort)uVar4 & 0x1f));
                                }
                                iVar6 = iVar6 + 1;
                                puVar5 = puVar5 + 1;
                            } while (iVar6 < iVar3);
                        }
                        __builtin_memcpy(&tmpC, &DAT_00aa7b13, 4);
                        local_2c = (int)((uint)local_2c + (uint)*param_1);
                        local_28 = (ushort *)((uint)local_28 + ((uint)tmpC & 0xfffffffeu));
                        local_24 = local_24 - 1;
                    } while (local_24 != 0);
                }
            }
            else if (0 < local_24) {
                do {
                    iVar6 = 0;
                    puVar5 = local_28;
                    if (0 < iVar3) {
                        do {
                            if (*(unsigned char *)(local_2c + iVar6) != 0) {
                                uVar1 = *puVar5;
                                uVar4 = (uint)param_1[*(unsigned char *)(local_2c + iVar6) + 5];
                                uVar4 = ((((uVar4 & 0x7c00u) * 0x40u) + (uVar4 & 0x3e0u) * 8u + (uVar4 & 0x1fu)) >> 1)
                                      + (((uint)(uVar1 & 0x1f) + (((uint)(uVar1 & 0x3e0) + (uint)(uVar1 & 0x7c00) * 8u) * 8u)) >> 1);
                                *puVar5 = (ushort)(((ushort)(uVar4 >> 3) & 0x3e0)
                                                   + ((ushort)(uVar4 >> 6) & 0x7c00)
                                                   + ((ushort)uVar4 & 0x1f));
                            }
                            iVar6 = iVar6 + 1;
                            puVar5 = puVar5 + 1;
                        } while (iVar6 < iVar3);
                    }
                    __builtin_memcpy(&tmpC, &DAT_00aa7b13, 4);
                    local_2c = (int)((uint)local_2c + (uint)*param_1);
                    local_28 = (ushort *)((uint)local_28 + ((uint)tmpC & 0xfffffffeu));
                    local_24 = local_24 - 1;
                } while (local_24 != 0);
                return uVar2;
            }
        }
    }
    return uVar2;
}

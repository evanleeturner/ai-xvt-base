/* SPDX-License-Identifier: MIT */
/* z_bop 004b5310 */
typedef unsigned int uint;

uint FUN_004d64e0(int *param_1);
void FUN_004d6480(int *param_1, int *param_2);
void FUN_004d64a0(int *param_1, int param_2, int param_3);

extern int DAT_00aa6cfc;
/* Packed globals: 0xaa7b13 and 0xaa7b1f are 4-byte values at unaligned addresses,
   read by memcpy. */
extern unsigned char DAT_00aa7b13[4];
extern unsigned char DAT_00aa7b1f[4];
extern char DAT_00aa7b23;

uint FUN_004b5310(int *param_1, int *param_2, int param_3, int param_4, uint param_5)
{
    int iVar1;
    uint uVar2;
    uint uVar3;
    uint uVar4;
    int iVar5;
    int iVar6;
    /* local_2c: local_2c, local_28, local_24, local_20 (contiguous rect) */
    int local_2c[4];
    int local_1c;
    int local_18;
    int local_14;
    /* local_10: local_10, local_c, local_8, local_4 (contiguous rect) */
    int local_10[4];
    int t0;
    int t1;

    if (param_1 == (int *)0x0) {
        return 0;
    }
    FUN_004d6480(local_2c, param_2);
    FUN_004d64a0(local_2c,
                 (int)((uint)param_3 - (uint)*param_2),
                 (int)((uint)param_4 - (uint)param_2[1]));
    FUN_004d6480(local_10, local_2c);
    uVar2 = FUN_004d64e0(local_2c);
    if (local_2c[0] <= local_2c[2]) {
        if (local_2c[3] < local_2c[1]) {
            return uVar2;
        }
        local_2c[0] = (int)((uint)local_2c[0] - (uint)local_10[0]);
        local_2c[1] = (int)((uint)local_2c[1] - (uint)local_10[1]);
        local_1c = *param_2;
        local_2c[2] = (int)((uint)(int)((uint)(int)((uint)(int)((uint)param_2[2] - (uint)local_1c)
                                                     - (uint)local_2c[0])
                                        - (uint)local_10[2]) + 1u + (uint)local_2c[2]);
        iVar6 = param_2[1];
        local_14 = (int)((uint)(int)((uint)(int)((uint)(int)((uint)param_2[3] - (uint)iVar6)
                                                 - (uint)local_2c[1])
                                    - (uint)local_10[3]) + 1u + (uint)local_2c[3]);

        __builtin_memcpy(&t0, DAT_00aa7b1f, 4);
        if (t0 == 8) {
            __builtin_memcpy(&t1, DAT_00aa7b13, 4);
            iVar5 = (int)((uint)(int)((uint)(param_4 + local_2c[1]) * (uint)t1)
                          + (uint)local_2c[0] + (uint)param_3 + (uint)DAT_00aa6cfc);
            local_1c = (int)((uint)local_1c
                             + (uint)(int)((uint)(int)((uint)iVar6 + (uint)local_2c[1]) * (uint)*param_1)
                             + (uint)param_1[4] + (uint)local_2c[0]);
            if (0 < local_14) {
                do {
                    iVar6 = 0;
                    if (0 < local_2c[2]) {
                        do {
                            if (*(char *)(local_1c + iVar6) != '\0') {
                                *(unsigned char *)(iVar5 + iVar6) = (unsigned char)param_5;
                            }
                            iVar6 = iVar6 + 1;
                        } while (iVar6 < local_2c[2]);
                    }
                    __builtin_memcpy(&t1, DAT_00aa7b13, 4);
                    iVar5 = iVar5 + t1;
                    local_1c = local_1c + *param_1;
                    local_14 = local_14 + -1;
                } while (local_14 != 0);
            }
        }
        else {
            __builtin_memcpy(&t0, DAT_00aa7b1f, 4);
            if (t0 == 0x10) {
                local_1c = (int)((uint)(int)((uint)(int)((uint)iVar6 + (uint)local_2c[1]) * (uint)*param_1)
                                 + (uint)param_1[4] + (uint)local_2c[0] + (uint)local_1c);
                __builtin_memcpy(&t1, DAT_00aa7b13, 4);
                local_18 = (int)((uint)(int)((uint)(param_4 + local_2c[1]) * (uint)t1)
                                 + (uint)(int)((uint)(local_2c[0] + param_3) * 2u)
                                 + (uint)DAT_00aa6cfc);
                if (0 < local_14) {
                    do {
                        iVar6 = 0;
                        if (0 < local_2c[2]) {
                            do {
                                if (*(unsigned char *)(local_1c + iVar6) != 0) {
                                    uVar3 = (uint)param_1[*(unsigned char *)(local_1c + iVar6) + 5] & 0x1f;
                                    if (DAT_00aa7b23 == '\0') {
                                        iVar5 = 0xb;
                                        uVar4 = param_5 & 0x7e0;
                                    }
                                    else {
                                        iVar5 = 10;
                                        uVar4 = param_5 & 0x3e0;
                                    }
                                    if (DAT_00aa7b23 == '\0') {
                                        iVar1 = 6;
                                    }
                                    else {
                                        iVar1 = 5;
                                    }
                                    *(short *)(local_18 + iVar6 * 2) =
                                        (short)((uint)(short)((uint)((int)((param_5 >> iVar5) * uVar3) / 0x1f)
                                                              << iVar1)
                                                + (uint)(short)(((uVar4 >> 5) * uVar3) / 0x1f))
                                        * 0x20
                                        + (short)(((param_5 & 0x1f) * uVar3) / 0x1f);
                                }
                                iVar6 = iVar6 + 1;
                            } while (iVar6 < local_2c[2]);
                        }
                        local_1c = local_1c + *param_1;
                        __builtin_memcpy(&t1, DAT_00aa7b13, 4);
                        local_18 = local_18 + (int)((uint)t1 & 0xfffffffeu);
                        local_14 = local_14 + -1;
                    } while (local_14 != 0);
                }
            }
        }
    }
    return uVar2;
}

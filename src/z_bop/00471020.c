/* SPDX-License-Identifier: MIT */
/* z_bop 00471020 */
extern unsigned char DAT_0099944a[4];
extern int DAT_0099945c;
extern unsigned char DAT_0099946a[4];
extern int DAT_00999470;
extern int DAT_00999478;
extern unsigned char DAT_00999482[4];
extern unsigned short DAT_009a1fe4;
extern float *DAT_0060f1d0;
extern float *DAT_0060f1e0;
extern int DAT_0060f1c4;

unsigned int __cdecl FUN_00471a10(int param_1, unsigned int *param_2, float *param_3, float *param_4);
unsigned int __cdecl FUN_00471ce0(unsigned int *param_1, float *param_2, float *param_3);
void __cdecl FUN_00471410(int param_1);

#define U(x) ((unsigned int)(x))
#define ADD(a, b) ((int)(U(a) + U(b)))
#define SUB(a, b) ((int)(U(a) - U(b)))
#define MUL(a, b) ((int)(U(a) * U(b)))
#define I(p) (*(int *)(p))
#define IP(base, off) I(ADD(base, off))

void __cdecl FUN_00471020(int param_1)
{
    int iVar1;
    unsigned int *puVar2;
    int iVar3;
    int iVar4;
    unsigned int uVar5;
    int iVar6;
    int iVar7;
    int iVar8;
    unsigned int *puVar9;
    int iVar10;
    int iVar11;
    int iVar12;
    int local_18;
    int *local_10;
    int local_4;
    int g44a;
    int g46a;
    int g82;

    iVar7 = DAT_00999470;
    iVar1 = ADD(DAT_0099945c, MUL(IP(param_1, 0xbc), 0x18));
    iVar8 = 0;
    __builtin_memcpy(&g44a, DAT_0099944a, 4);
    local_10 = (int *)ADD(MUL(IP(param_1, 0xb8), 0x70), g44a);
    IP(param_1, 0xc0) = DAT_00999470;
    iVar6 = 0;
    IP(param_1, 0xcc) = 0;
    __builtin_memcpy(&g46a, DAT_0099946a, 4);
    puVar2 = (unsigned int *)ADD(g46a, MUL(iVar7, 0x1c));
    if (0 < IP(param_1, 0x90)) {
        do {
            iVar8 = ADD(iVar8, 4);
            __builtin_memcpy(&g82, DAT_00999482, 4);
            I(ADD(SUB(g82, 4), iVar8)) = -1;
            iVar6 = ADD(iVar6, 1);
        } while (iVar6 < IP(param_1, 0x90));
    }
    local_4 = 0;
    puVar9 = puVar2;
    if (0 < IP(param_1, 0xc4)) {
        do {
            iVar7 = ADD(MUL(*local_10, 0x40), IP(param_1, 0xa0));
            if (local_10[3] == -1) {
                local_10[3] = (int)(unsigned int)DAT_009a1fe4;
                DAT_0060f1d0 = (float *)0;
                DAT_0060f1e0 = (float *)0;
                local_18 = SUB(4, (unsigned int)(IP(iVar7, 0x1c) == -1));
                iVar6 = 0;
                iVar8 = ADD(ADD(iVar7, 0x10), MUL(local_18, 4));
                iVar10 = 0;
                do {
                    iVar3 = MUL(I(SUB(iVar8, 4)), 4);
                    local_18 = SUB(local_18, 1);
                    DAT_0060f1c4 = 0;
                    __builtin_memcpy(&g82, DAT_00999482, 4);
                    iVar4 = I(ADD(iVar3, g82));
                    iVar11 = iVar10;
                    if (iVar4 == -1) {
                        uVar5 = FUN_00471a10(param_1, puVar9,
                                             (float *)ADD(iVar1, MUL(I(ADD(DAT_00999478, MUL(I(SUB(iVar8, 0x14)), 4))), 0x18)),
                                             (float *)ADD(iVar1, MUL(I(ADD(DAT_00999478, MUL(I(ADD(iVar7, MUL(iVar6, 4))), 4))), 0x18)));
                        if ((int)uVar5 < 0) {
                            if (DAT_0060f1c4 == 0) {
                                __builtin_memcpy(&g82, DAT_00999482, 4);
                                I(ADD(iVar3, g82)) = -2;
                            }
                        } else {
                            local_10[ADD(iVar10, 0x14)] = (int)puVar9;
                            puVar9[6] = (unsigned int)DAT_0060f1c4;
                            __builtin_memcpy(&g82, DAT_00999482, 4);
                            I(ADD(iVar3, g82)) = IP(param_1, 0xcc);
                            IP(param_1, 0xcc) = ADD(IP(param_1, 0xcc), 1);
                            puVar9 = (unsigned int *)(U(puVar9) + 0x1cu);
                            iVar11 = ADD(iVar10, 1);
                        }
                    } else if (iVar4 != -2) {
                        iVar11 = ADD(iVar10, 1);
                        local_10[ADD(iVar10, 0x14)] = (int)(unsigned int *)(U(puVar2) + U(iVar4) * 0x1cu);
                        __builtin_memcpy(&g82, DAT_00999482, 4);
                        if (puVar2[ADD(MUL(I(ADD(iVar3, g82)), 7), 6)] != 0) {
                            DAT_0060f1e0 = DAT_0060f1d0;
                            DAT_0060f1d0 = (float *)puVar2[ADD(MUL(I(ADD(iVar3, g82)), 7), 6)];
                        }
                    }
                    iVar6 = local_18;
                    iVar8 = SUB(iVar8, 4);
                    iVar10 = iVar11;
                } while (0 < local_18);
                iVar12 = iVar11;
                if (DAT_0060f1e0 != (float *)0) {
                    uVar5 = FUN_00471a10(param_1, puVar9, DAT_0060f1d0, DAT_0060f1e0);
                    if (-1 < (int)uVar5) {
                        iVar12 = ADD(iVar11, 1);
                        local_10[ADD(iVar11, 0x14)] = (int)puVar9;
                        puVar9 = (unsigned int *)(U(puVar9) + 0x1cu);
                        IP(param_1, 0xcc) = ADD(IP(param_1, 0xcc), 1);
                    }
                }
            } else {
                local_10[3] = (int)(unsigned int)DAT_009a1fe4;
                local_18 = SUB(4, (unsigned int)(IP(iVar7, 0x1c) == -1));
                iVar6 = 0;
                iVar8 = ADD(ADD(iVar7, 0x10), MUL(local_18, 4));
                iVar10 = 0;
                do {
                    iVar3 = I(SUB(iVar8, 4));
                    local_18 = SUB(local_18, 1);
                    __builtin_memcpy(&g82, DAT_00999482, 4);
                    iVar4 = I(ADD(g82, MUL(iVar3, 4)));
                    iVar12 = iVar10;
                    if (iVar4 == -1) {
                        uVar5 = FUN_00471ce0(puVar9,
                                             (float *)ADD(iVar1, MUL(I(ADD(DAT_00999478, MUL(I(SUB(iVar8, 0x14)), 4))), 0x18)),
                                             (float *)ADD(iVar1, MUL(I(ADD(DAT_00999478, MUL(I(ADD(iVar7, MUL(iVar6, 4))), 4))), 0x18)));
                        if ((int)uVar5 < 0) {
                            __builtin_memcpy(&g82, DAT_00999482, 4);
                            I(ADD(g82, MUL(iVar3, 4))) = -2;
                        } else {
                            local_10[ADD(iVar10, 0x14)] = (int)puVar9;
                            puVar9 = (unsigned int *)(U(puVar9) + 0x1cu);
                            __builtin_memcpy(&g82, DAT_00999482, 4);
                            I(ADD(g82, MUL(iVar3, 4))) = IP(param_1, 0xcc);
                            IP(param_1, 0xcc) = ADD(IP(param_1, 0xcc), 1);
                            iVar12 = ADD(iVar10, 1);
                        }
                    } else if (iVar4 != -2) {
                        iVar12 = ADD(iVar10, 1);
                        local_10[ADD(iVar10, 0x14)] = (int)(unsigned int *)(U(puVar2) + U(iVar4) * 0x1cu);
                    }
                    iVar6 = local_18;
                    iVar8 = SUB(iVar8, 4);
                    iVar10 = iVar12;
                } while (0 < local_18);
            }
            local_10[0x19] = iVar12;
            if (iVar12 == 0) {
                local_10[0x11] = 0;
                local_10[0x10] = 0;
            } else {
                FUN_00471410((int)local_10);
            }
            local_4 = ADD(local_4, 1);
            local_10 = (int *)(U(local_10) + 0x70u);
        } while (local_4 < IP(param_1, 0xc4));
    }
    DAT_00999470 = ADD(DAT_00999470, IP(param_1, 0xcc));
    return;
}

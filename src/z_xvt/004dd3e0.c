/* SPDX-License-Identifier: MIT */
/* z_xvt 004dd3e0 */
extern int DAT_00758830;
extern unsigned char DAT_0075964b[4];   /* 0x75964b is unaligned: read via memcpy */
extern unsigned char DAT_00759657[4];   /* 0x759657 is unaligned: read via memcpy */

void __cdecl FUN_004dd570(int *param_1, int param_2, int param_3, unsigned short param_4);
void __cdecl FUN_004dd000(int *param_1, int *param_2);
void __cdecl FUN_004dd030(int *param_1, int param_2, int param_3);
void __cdecl FUN_004dd0b0(int *param_1);

void __cdecl FUN_004dd3e0(int *param_1, int param_2, int param_3, unsigned short param_4, int param_5)
{
    int iVar1;
    unsigned int uVar2;
    unsigned char uVar3;
    unsigned int uVar4;
    unsigned int *puVar5;
    /* Ghidra's iStack_28..iStack_1c: four contiguous ints, address of the first is passed to callees */
    struct {
        int iStack_28;
        int iStack_24;
        int iStack_20;
        int iStack_1c;
    } L;
    unsigned int *puStack_8;
    int iVar6;

    if (param_5 == 0) {
        FUN_004dd570(param_1, param_2, param_3, param_4);
        return;
    }
    if ((*param_1 < param_1[2]) && (param_1[1] < param_1[3])) {
        FUN_004dd000(&L.iStack_28, param_1);
        FUN_004dd030(&L.iStack_28, param_2, param_3);
        if ((L.iStack_28 < 0x281) &&
            (((-1 < L.iStack_20) && (L.iStack_24 < 0x1e1)) && (-1 < L.iStack_1c))) {
            FUN_004dd0b0(&L.iStack_28);
            __builtin_memcpy(&iVar1, DAT_0075964b, 4);
            L.iStack_1c = (int)((unsigned int)L.iStack_1c + 1u);
            uVar4 = ((unsigned int)L.iStack_20 - (unsigned int)L.iStack_28) + 1u;
            __builtin_memcpy(&iVar6, DAT_00759657, 4);
            if (iVar6 == 8) {
                puStack_8 = (unsigned int *)((unsigned int)iVar1 * (unsigned int)L.iStack_24
                                             + (unsigned int)L.iStack_28
                                             + (unsigned int)DAT_00758830);
                if (L.iStack_24 < L.iStack_1c) {
                    L.iStack_1c = (int)((unsigned int)L.iStack_1c - (unsigned int)L.iStack_24);
                    do {
                        uVar3 = (unsigned char)param_4;
                        puVar5 = puStack_8;
                        for (uVar2 = uVar4 >> 2; uVar2 != 0; uVar2 = uVar2 - 1) {
                            *puVar5 = (unsigned int)uVar3 * 0x01010101u;
                            puVar5 = puVar5 + 1;
                        }
                        for (uVar2 = uVar4 & 3; uVar2 != 0; uVar2 = uVar2 - 1) {
                            *(unsigned char *)puVar5 = uVar3;
                            puVar5 = (unsigned int *)((unsigned int)puVar5 + 1u);
                        }
                        puStack_8 = (unsigned int *)((unsigned int)puStack_8 + (unsigned int)iVar1);
                        L.iStack_1c = (int)((unsigned int)L.iStack_1c - 1u);
                    } while (L.iStack_1c != 0);
                    return;
                }
            } else {
                if (iVar6 != 0x10) {
                    return;
                }
                puStack_8 = (unsigned int *)((unsigned int)iVar1 * (unsigned int)L.iStack_24
                                             + (unsigned int)L.iStack_28 * 2u
                                             + (unsigned int)DAT_00758830);
                if (L.iStack_24 < L.iStack_1c) {
                    L.iStack_1c = (int)((unsigned int)L.iStack_1c - (unsigned int)L.iStack_24);
                    do {
                        uVar2 = uVar4;
                        puVar5 = puStack_8;
                        if (0 < (int)uVar4) {
                            for (; uVar2 != 0; uVar2 = uVar2 - 1) {
                                *(unsigned short *)puVar5 = param_4;
                                puVar5 = (unsigned int *)((unsigned int)puVar5 + 2u);
                            }
                        }
                        puStack_8 = (unsigned int *)((unsigned int)puStack_8 + (unsigned int)iVar1);
                        L.iStack_1c = (int)((unsigned int)L.iStack_1c - 1u);
                    } while (L.iStack_1c != 0);
                }
            }
        }
    }
    return;
}

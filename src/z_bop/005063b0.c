/* SPDX-License-Identifier: MIT */
/* z_bop 005063b0 */
extern int DAT_00aa7b1f;
extern int DAT_00aa7b13;
extern int DAT_00aa7b8a;
extern int DAT_00aa7b8e;
extern int DAT_00aa7b82;
extern int DAT_00aa7b86;
extern unsigned int DAT_00aa6cfc;

void __cdecl FUN_005063b0(int param_1, int param_2, int param_3, unsigned short param_4)
{
    unsigned char uVar1;
    int iVar2;
    unsigned int uVar3;
    unsigned int uVar4;
    unsigned char *puVar5;
    unsigned int dw;
    int g1f, g13, g8a, g8e, g82, g86;

    __builtin_memcpy(&g1f, &DAT_00aa7b1f, 4);
    __builtin_memcpy(&g13, &DAT_00aa7b13, 4);
    __builtin_memcpy(&g8a, &DAT_00aa7b8a, 4);
    __builtin_memcpy(&g8e, &DAT_00aa7b8e, 4);
    __builtin_memcpy(&g82, &DAT_00aa7b82, 4);
    __builtin_memcpy(&g86, &DAT_00aa7b86, 4);

    iVar2 = g1f >> 4;
    if ((g8a <= param_3) && (param_3 <= g8e)) {
        uVar1 = (unsigned char)param_4;
        dw = (unsigned int)uVar1;
        dw = (dw << 8) | dw;
        dw = (dw << 16) | dw;
        if (param_2 < param_1) {
            if (((g82 <= param_2) || (param_2 = g82, g82 < param_1)) &&
                ((param_1 <= g86) || (param_1 = g86, param_2 < g86))) {
                puVar5 = (unsigned char *)(((unsigned int)param_2 << ((unsigned char)iVar2 & 0x1f)) +
                                           (unsigned int)g13 * (unsigned int)param_3 + DAT_00aa6cfc);
                if (iVar2 == 0) {
                    uVar4 = ((unsigned int)param_1 - (unsigned int)param_2) + 1;
                    for (uVar3 = uVar4 >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
                        __builtin_memcpy(puVar5, &dw, 4);
                        puVar5 = puVar5 + 4;
                    }
                    for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
                        *puVar5 = uVar1;
                        puVar5 = puVar5 + 1;
                    }
                    return;
                }
                if (iVar2 != 1) {
                    return;
                }
                iVar2 = (int)(((unsigned int)param_1 - (unsigned int)param_2) + 1);
                if (0 < iVar2) {
                    for (; iVar2 != 0; iVar2 = iVar2 + -1) {
                        __builtin_memcpy(puVar5, &param_4, 2);
                        puVar5 = puVar5 + 2;
                    }
                }
            }
        } else if (((param_2 <= g86) || (param_2 = g86, param_1 <= g86)) &&
                   ((g82 <= param_1) || (param_1 = g82, g82 <= param_2))) {
            puVar5 = (unsigned char *)(((unsigned int)param_1 << ((unsigned char)iVar2 & 0x1f)) +
                                       (unsigned int)g13 * (unsigned int)param_3 + DAT_00aa6cfc);
            if (iVar2 == 0) {
                uVar4 = ((unsigned int)param_2 - (unsigned int)param_1) + 1;
                for (uVar3 = uVar4 >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
                    __builtin_memcpy(puVar5, &dw, 4);
                    puVar5 = puVar5 + 4;
                }
                for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
                    *puVar5 = uVar1;
                    puVar5 = puVar5 + 1;
                }
                return;
            }
            if (iVar2 != 1) {
                return;
            }
            iVar2 = (int)(((unsigned int)param_2 - (unsigned int)param_1) + 1);
            if (0 < iVar2) {
                for (; iVar2 != 0; iVar2 = iVar2 + -1) {
                    __builtin_memcpy(puVar5, &param_4, 2);
                    puVar5 = puVar5 + 2;
                }
                return;
            }
        }
    }
    return;
}

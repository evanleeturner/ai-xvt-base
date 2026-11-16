/* SPDX-License-Identifier: MIT */
/* z_xvt 004f8160 */
extern int DAT_00759657;
extern int DAT_007596bf;
extern int DAT_007596c3;
extern int DAT_007596b7;
extern int DAT_007596bb;
extern int DAT_0075964b;
extern int DAT_00758830;

void __cdecl FUN_004f8160(int param_1, int param_2, int param_3, unsigned short param_4)
{
    unsigned char uVar1;
    int iVar2;
    unsigned int uVar3;
    unsigned int uVar4;
    unsigned char *puVar5;
    int g_57;
    int g_bf;
    int g_c3;
    int g_b7;
    int g_bb;
    int g_4b;
    int g_30;
    unsigned int pat;

    __builtin_memcpy(&g_57, (void *)&DAT_00759657, 4);
    iVar2 = g_57 >> 4;
    __builtin_memcpy(&g_bf, (void *)&DAT_007596bf, 4);
    __builtin_memcpy(&g_c3, (void *)&DAT_007596c3, 4);
    if ((g_bf <= param_3) && (param_3 <= g_c3)) {
        uVar1 = (unsigned char)param_4;
        pat = (unsigned int)uVar1 * 0x01010101u;
        __builtin_memcpy(&g_b7, (void *)&DAT_007596b7, 4);
        __builtin_memcpy(&g_bb, (void *)&DAT_007596bb, 4);
        __builtin_memcpy(&g_4b, (void *)&DAT_0075964b, 4);
        g_30 = DAT_00758830;
        if (param_2 < param_1) {
            if (((g_b7 <= param_2) || (param_2 = g_b7, g_b7 < param_1)) &&
                ((param_1 <= g_bb) || (param_1 = g_bb, param_2 < g_bb))) {
                puVar5 = (unsigned char *)(((unsigned int)param_2 << (iVar2 & 0x1f)) +
                                           (unsigned int)g_4b * (unsigned int)param_3 +
                                           (unsigned int)g_30);
                if (iVar2 == 0) {
                    uVar4 = (unsigned int)param_1 - (unsigned int)param_2 + 1u;
                    for (uVar3 = uVar4 >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
                        __builtin_memcpy(puVar5, &pat, 4);
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
                iVar2 = (int)((unsigned int)param_1 - (unsigned int)param_2 + 1u);
                if (0 < iVar2) {
                    for (; iVar2 != 0; iVar2 = iVar2 - 1) {
                        __builtin_memcpy(puVar5, &param_4, 2);
                        puVar5 = puVar5 + 2;
                    }
                }
            }
        } else if (((param_2 <= g_bb) || (param_2 = g_bb, param_1 <= g_bb)) &&
                   ((g_b7 <= param_1) || (param_1 = g_b7, g_b7 <= param_2))) {
            puVar5 = (unsigned char *)(((unsigned int)param_1 << (iVar2 & 0x1f)) +
                                       (unsigned int)g_4b * (unsigned int)param_3 +
                                       (unsigned int)g_30);
            if (iVar2 == 0) {
                uVar4 = (unsigned int)param_2 - (unsigned int)param_1 + 1u;
                for (uVar3 = uVar4 >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
                    __builtin_memcpy(puVar5, &pat, 4);
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
            iVar2 = (int)((unsigned int)param_2 - (unsigned int)param_1 + 1u);
            if (0 < iVar2) {
                for (; iVar2 != 0; iVar2 = iVar2 - 1) {
                    __builtin_memcpy(puVar5, &param_4, 2);
                    puVar5 = puVar5 + 2;
                }
                return;
            }
        }
    }
    return;
}

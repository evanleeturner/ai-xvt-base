/* SPDX-License-Identifier: MIT */
/* z_bop 00459f30 */
extern int DAT_009a1fe8;
void __cdecl FUN_0041bb00(unsigned short param_1);

void __cdecl FUN_00459f30(int param_1, int param_2)
{
    int iVar4 = param_1 * 0x23;
    int iVar5;
    int *puVar6;
    int *puVar7;
    int uVar1, uVar2, uVar3;

    puVar7 = *(int **)(*(int *)(DAT_009a1fe8 + 0x1f + iVar4) + 0xa9);
    if (puVar7 != NULL) {
        puVar6 = *(int **)(*(int *)(param_2 * 0x23 + 0x1f + DAT_009a1fe8) + 0xa9);
        if (puVar6 != NULL) {
            for (iVar5 = 0x118; iVar5 != 0; iVar5--) {
                *puVar7 = *puVar6;
                puVar6++;
                puVar7++;
            }
            *(short *)puVar7 = *(short *)puVar6;
        }
    }

    puVar7 = *(int **)(*(int *)(DAT_009a1fe8 + 0x1f + iVar4) + 0xa5);
    if (puVar7 != NULL) {
        puVar6 = *(int **)(*(int *)(param_2 * 0x23 + 0x1f + DAT_009a1fe8) + 0xa5);
        if (puVar6 != NULL) {
            *puVar7 = *puVar6;
            puVar7[1] = puVar6[1];
            *(short *)(puVar7 + 2) = *(short *)(puVar6 + 2);
        }
    }

    puVar7 = *(int **)(*(int *)(DAT_009a1fe8 + 0x1f + iVar4) + 0xad);
    if (puVar7 != NULL) {
        puVar6 = *(int **)(*(int *)(param_2 * 0x23 + 0x1f + DAT_009a1fe8) + 0xad);
        if (puVar6 != NULL) {
            for (iVar5 = 0x13; iVar5 != 0; iVar5--) {
                *puVar7 = *puVar6;
                puVar6++;
                puVar7++;
            }
        }
    }

    puVar7 = *(int **)(DAT_009a1fe8 + 0x1f + iVar4);
    if (puVar7 != NULL) {
        puVar6 = *(int **)(param_2 * 0x23 + 0x1f + DAT_009a1fe8);
        if (puVar6 != NULL) {
            uVar1 = *(int *)((int)puVar7 + 0xa9);
            uVar2 = *(int *)((int)puVar7 + 0xa5);
            uVar3 = *(int *)((int)puVar7 + 0xad);
            for (iVar5 = 0x2c; iVar5 != 0; iVar5--) {
                *puVar7 = *puVar6;
                puVar6++;
                puVar7++;
            }
            *(char *)puVar7 = *(char *)puVar6;
            *(int *)(*(int *)(DAT_009a1fe8 + 0x1f + iVar4) + 0xa9) = uVar1;
            *(int *)(*(int *)(DAT_009a1fe8 + 0x1f + iVar4) + 0xa5) = uVar2;
            *(int *)(*(int *)(DAT_009a1fe8 + 0x1f + iVar4) + 0xad) = uVar3;
        }
    }

    uVar1 = *(int *)(DAT_009a1fe8 + iVar4 + 0x1f);
    puVar7 = (int *)(param_2 * 0x23 + DAT_009a1fe8);
    puVar6 = (int *)(DAT_009a1fe8 + iVar4);
    for (iVar5 = 8; iVar5 != 0; iVar5--) {
        *puVar6 = *puVar7;
        puVar7++;
        puVar6++;
    }
    *(short *)puVar6 = *(short *)puVar7;
    *(char *)((int)puVar6 + 2) = *(char *)((int)puVar7 + 2);
    *(int *)(DAT_009a1fe8 + 0x1f + iVar4) = uVar1;
    FUN_0041bb00((unsigned short)param_1);
}

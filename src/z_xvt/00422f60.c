/* SPDX-License-Identifier: MIT */
/* z_xvt 00422f60 */
extern int DAT_006d4988;
void __cdecl FUN_0040f050(unsigned short param_1);

void __cdecl FUN_00422f60(int param_1, int param_2)
{
    unsigned int uVar1;
    unsigned int uVar2;
    unsigned int uVar3;
    int iVar4;
    int iVar5;
    unsigned int *puVar6;
    unsigned int *puVar7;

    iVar4 = param_1 * 0x23;
    puVar7 = *(unsigned int **)(*(unsigned int *)(DAT_006d4988 + 0x1f + iVar4) + 0xa7);
    if (puVar7 != NULL) {
        puVar6 = *(unsigned int **)(*(unsigned int *)(param_2 * 0x23 + 0x1f + DAT_006d4988) + 0xa7);
        if (puVar6 != NULL) {
            for (iVar5 = 0x10d; iVar5 != 0; iVar5--) {
                *puVar7 = *puVar6;
                puVar6++;
                puVar7++;
            }
            *(unsigned short *)puVar7 = *(unsigned short *)puVar6;
        }
    }

    puVar7 = *(unsigned int **)(*(unsigned int *)(DAT_006d4988 + 0x1f + iVar4) + 0xa3);
    if (puVar7 != NULL) {
        puVar6 = *(unsigned int **)(*(unsigned int *)(param_2 * 0x23 + 0x1f + DAT_006d4988) + 0xa3);
        if (puVar6 != NULL) {
            *puVar7 = *puVar6;
            puVar7[1] = puVar6[1];
        }
    }

    puVar7 = *(unsigned int **)(*(unsigned int *)(DAT_006d4988 + 0x1f + iVar4) + 0xab);
    if (puVar7 != NULL) {
        puVar6 = *(unsigned int **)(*(unsigned int *)(param_2 * 0x23 + 0x1f + DAT_006d4988) + 0xab);
        if (puVar6 != NULL) {
            for (iVar5 = 0x13; iVar5 != 0; iVar5--) {
                *puVar7 = *puVar6;
                puVar6++;
                puVar7++;
            }
        }
    }

    puVar7 = *(unsigned int **)(DAT_006d4988 + 0x1f + iVar4);
    if (puVar7 != NULL) {
        puVar6 = *(unsigned int **)(param_2 * 0x23 + 0x1f + DAT_006d4988);
        if (puVar6 != NULL) {
            uVar1 = *(unsigned int *)((int)puVar7 + 0xa7);
            uVar2 = *(unsigned int *)((int)puVar7 + 0xa3);
            uVar3 = *(unsigned int *)((int)puVar7 + 0xab);
            for (iVar5 = 0x2b; iVar5 != 0; iVar5--) {
                *puVar7 = *puVar6;
                puVar6++;
                puVar7++;
            }
            *(unsigned short *)puVar7 = *(unsigned short *)puVar6;
            *(unsigned char *)((int)puVar7 + 2) = *(unsigned char *)((int)puVar6 + 2);
            *(unsigned int *)(*(unsigned int *)(DAT_006d4988 + 0x1f + iVar4) + 0xa7) = uVar1;
            *(unsigned int *)(*(unsigned int *)(DAT_006d4988 + 0x1f + iVar4) + 0xa3) = uVar2;
            *(unsigned int *)(*(unsigned int *)(DAT_006d4988 + 0x1f + iVar4) + 0xab) = uVar3;
        }
    }

    uVar1 = *(unsigned int *)(DAT_006d4988 + iVar4 + 0x1f);
    puVar7 = (unsigned int *)(param_2 * 0x23 + DAT_006d4988);
    puVar6 = (unsigned int *)(DAT_006d4988 + iVar4);
    for (iVar5 = 8; iVar5 != 0; iVar5--) {
        *puVar6 = *puVar7;
        puVar7++;
        puVar6++;
    }
    *(unsigned short *)puVar6 = *(unsigned short *)puVar7;
    *(unsigned char *)((int)puVar6 + 2) = *(unsigned char *)((int)puVar7 + 2);
    *(unsigned int *)(DAT_006d4988 + 0x1f + iVar4) = uVar1;
    FUN_0040f050((unsigned short)param_1);
}

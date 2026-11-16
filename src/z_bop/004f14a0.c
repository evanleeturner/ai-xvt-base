/* SPDX-License-Identifier: MIT */
/* z_bop 004f14a0 */
extern int DAT_00aa5cb0;
extern int DAT_00aa5cb4;
extern int DAT_00aa5ac0;
extern int DAT_00aa5ac4;
extern int DAT_00aa5adc;
extern int DAT_00aa5ccc;
extern int DAT_00aa5df0;
extern int DAT_00aa5e10;
extern int DAT_00aa5e68;
extern int DAT_00b69e30;
extern int DAT_00b6a24c;
/* Packed globals: addresses are not 4-byte aligned (…5e, …aa mod 4 == 2) */
extern unsigned char DAT_00aa615e;
extern unsigned char DAT_00b6d5aa;
extern unsigned char DAT_00b6d86a;

int *FUN_004cfe80(int *param_1);
int FUN_004d11e0(void);
int FUN_004d1400(void);

int __stdcall FUN_004f14a0(void)
{
    int *puVar1;
    int *piVar2;
    int *piVar5;
    unsigned char *puVar4;
    unsigned char *pbVar2;
    int iVar3;
    int iVar6;
    int iVar7;
    int iVar8;
    int iVar9;
    int iVar10;
    int bVar11;
    int local_c;
    int *local_8;
    int local_4;
    int tmp;
    int zero;

    iVar10 = 10;
    puVar1 = FUN_004cfe80(&local_c);
    piVar2 = &DAT_00aa5cb0;
    do {
        iVar7 = 8;
        do {
            iVar8 = 0;
            bVar11 = (local_c == 0);
            if (0 < local_c) {
                piVar5 = puVar1 + 9;
                do {
                    if ((*piVar5 != 0) && (piVar5[-1] == *piVar2)) break;
                    piVar5 = piVar5 + 10;
                    iVar8 = iVar8 + 1;
                } while (iVar8 < local_c);
                bVar11 = (local_c == iVar8);
            }
            if (bVar11) {
                *piVar2 = 0;
            }
            piVar2 = piVar2 + 1;
            iVar7 = iVar7 - 1;
        } while (iVar7 != 0);
        iVar10 = iVar10 - 1;
    } while (iVar10 != 0);

    zero = 0;
    puVar4 = &DAT_00aa615e;
    piVar2 = &DAT_00aa5df0;
    do {
        iVar10 = 0;
        if (0 < local_c) {
            piVar5 = puVar1 + 9;
            do {
                if ((*piVar5 != 0) && (piVar5[-1] == *piVar2)) break;
                piVar5 = piVar5 + 10;
                iVar10 = iVar10 + 1;
            } while (iVar10 < local_c);
        }
        if (iVar10 == 8) {
            *piVar2 = 0;
            __builtin_memcpy(puVar4, &zero, 4);
        }
        puVar4 = puVar4 + 0x2a;
        piVar2 = piVar2 + 1;
    } while (piVar2 < &DAT_00aa5e10);

    pbVar2 = &DAT_00b6d5aa;
    do {
        iVar10 = 0;
        __builtin_memcpy(&tmp, pbVar2, 4);
        if (0 < local_c) {
            piVar5 = puVar1 + 9;
            do {
                if ((*piVar5 != 0) && (piVar5[-1] == tmp)) break;
                piVar5 = piVar5 + 10;
                iVar10 = iVar10 + 1;
            } while (iVar10 < local_c);
        }
        if (local_c == iVar10) {
            __builtin_memcpy(pbVar2, &zero, 4);
        }
        pbVar2 = pbVar2 + 0x58;
    } while (pbVar2 < &DAT_00b6d86a);

    iVar10 = 0;
    pbVar2 = &DAT_00b6d5aa;
    do {
        iVar8 = FUN_004d11e0();
        iVar7 = iVar10;
        __builtin_memcpy(&tmp, pbVar2, 4);
        if (iVar8 == tmp) break;
        pbVar2 = pbVar2 + 0x58;
        iVar10 = iVar10 + 1;
        iVar7 = DAT_00b6a24c;
    } while (pbVar2 < &DAT_00b6d86a);
    DAT_00b6a24c = iVar7;

    if (DAT_00b69e30 == 2) {
        local_c = 1;
    } else {
        local_c = FUN_004d1400();
    }

    local_8 = &DAT_00aa5df0;
    do {
        iVar10 = 0;
        bVar11 = (local_c == 0);
        if (0 < local_c) {
            do {
                if (*local_8 == 0) break;
                iVar10 = iVar10 + 1;
            } while (iVar10 < local_c);
            bVar11 = (local_c == iVar10);
        }
        if ((!bVar11) && (0 < DAT_00aa5e68)) {
            iVar7 = 0;
            iVar10 = *local_8;
            local_4 = DAT_00aa5e68;
            do {
                iVar6 = 0;
                iVar8 = iVar7;
                do {
                    if (*(int *)((unsigned char *)&DAT_00aa5cb0 + iVar8) == iVar10) {
                        if (iVar6 < 7) {
                            iVar3 = 7 - iVar6;
                            iVar9 = iVar8;
                            do {
                                iVar3 = iVar3 - 1;
                                *(int *)((unsigned char *)&DAT_00aa5cb0 + iVar9) =
                                    *(int *)((unsigned char *)&DAT_00aa5cb4 + iVar9);
                                *(int *)((unsigned char *)&DAT_00aa5ac0 + iVar9) =
                                    *(int *)((unsigned char *)&DAT_00aa5ac4 + iVar9);
                                iVar9 = iVar9 + 4;
                            } while (iVar3 != 0);
                        }
                        *(int *)((unsigned char *)&DAT_00aa5ccc + iVar7) = 0;
                        *(int *)((unsigned char *)&DAT_00aa5adc + iVar7) = -1;
                    }
                    iVar8 = iVar8 + 4;
                    iVar6 = iVar6 + 1;
                } while (iVar6 < 8);
                iVar7 = iVar7 + 0x20;
                local_4 = local_4 - 1;
            } while (local_4 != 0);
        }
        local_8 = local_8 + 1;
        if ((unsigned char *)0xaa5e0f < (unsigned char *)local_8) {
            return 1;
        }
    } while (1);
}

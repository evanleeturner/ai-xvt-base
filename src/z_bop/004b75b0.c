/* SPDX-License-Identifier: MIT */
/* z_bop 004b75b0 */
extern unsigned int DAT_00aa6d00;
extern unsigned char DAT_00aa6d04[];

void *__cdecl _malloc(unsigned int);
void __cdecl _free(void *);
void *__cdecl _realloc(void *, unsigned int);

unsigned int __cdecl FUN_004b75b0(int *param_1)
{
    unsigned char *pbVar1;
    int iVar2;
    int iVar3;
    unsigned int uVar4;
    unsigned int *_Memory;
    unsigned char *pbVar5;
    void *pvVar6;
    int iVar7;
    unsigned char *pbVar8;
    unsigned char *pbVar9;
    unsigned int uVar10;
    unsigned char bVar11;
    int iVar12;
    unsigned char bVar13;
    int iVar14;
    int iVar15;
    unsigned int *puVar16;
    int iVar17;
    int iVar18;
    unsigned int *puVar19;
    unsigned char *local_24;
    unsigned int local_1c;
    unsigned char *local_18;
    unsigned int *local_8;
    int local_4;

    iVar2 = *param_1;
    iVar3 = param_1[1];
    _Memory = (unsigned int *)_malloc((unsigned int)iVar2 * (unsigned int)iVar3);
    if (_Memory == (unsigned int *)0) {
        param_1[2] = 0;
        return 0;
    }
    local_1c = 0;
    local_18 = (unsigned char *)param_1[4];
    iVar7 = 0x280;
    if (*param_1 < 0x281) {
        iVar7 = *param_1;
    }
    local_4 = 0;
    local_8 = _Memory;
    if (param_1[1] < 1) {
LAB_004b77a9:
        if ((int)((unsigned int)*param_1 * (unsigned int)param_1[1]) < (int)local_1c) {
            return 0;
        }
        pvVar6 = _realloc(_Memory, local_1c);
        if (_Memory != (unsigned int *)0) {
            _free((void *)param_1[4]);
            param_1[4] = (int)pvVar6;
            param_1[2] = 1;
            param_1[3] = (int)local_1c;
            return 1;
        }
        _free((void *)0);
        param_1[2] = 0;
        return 0;
    }
LAB_004b7630:
    iVar14 = 0;
    iVar15 = 0;
    bVar13 = *local_18;
    local_24 = (unsigned char *)&DAT_00aa6d04;
    DAT_00aa6d04[0] = 0;
    pbVar5 = local_18;
    pbVar8 = (unsigned char *)&DAT_00aa6d04;
    if (0 < iVar7) {
        do {
            iVar12 = 0;
            do {
                if ((iVar7 <= iVar15) || (pbVar5[iVar12] != bVar13)) break;
                iVar15 = iVar15 + 1;
                iVar12 = iVar12 + 1;
            } while (iVar12 < 0x3f);
            bVar11 = (unsigned char)iVar12;
            if (bVar13 == 0) {
                iVar14 = iVar14 + 1;
                *pbVar8 = bVar11 | 0x40;
                pbVar9 = pbVar8 + 1;
                local_24 = pbVar8;
            }
            else if (iVar12 < 3) {
                if ((*local_24 & 0x80) == 0) {
                    iVar14 = iVar14 + 1;
                    iVar17 = 0;
                    *pbVar8 = bVar11 | 0x80;
                    pbVar9 = pbVar8 + 1;
                    local_24 = pbVar8;
                    if (0 < iVar12) {
                        do {
                            pbVar1 = pbVar5 + iVar17;
                            iVar17 = iVar17 + 1;
                            pbVar8[iVar17] = *pbVar1;
                        } while (iVar17 < iVar12);
                    }
                }
                else {
                    bVar13 = (unsigned char)((*local_24 & 0x7f) + bVar11);
                    if (bVar13 < 0x80) {
                        *local_24 = bVar13 | 0x80;
                        pbVar9 = pbVar8;
                        iVar17 = 0;
                        if (0 < iVar12) {
                            do {
                                iVar18 = iVar17 + 1;
                                pbVar8[iVar17] = pbVar5[iVar17];
                                iVar17 = iVar18;
                            } while (iVar18 < iVar12);
                        }
                    }
                    else {
                        iVar14 = iVar14 + 1;
                        iVar17 = 0;
                        *pbVar8 = bVar11 | 0x80;
                        pbVar9 = pbVar8 + 1;
                        local_24 = pbVar8;
                        if (0 < iVar12) {
                            do {
                                pbVar1 = pbVar5 + iVar17;
                                iVar17 = iVar17 + 1;
                                pbVar8[iVar17] = *pbVar1;
                            } while (iVar17 < iVar12);
                        }
                    }
                }
                pbVar9 = pbVar9 + iVar12;
                iVar14 = iVar14 + iVar12;
            }
            else {
                *pbVar8 = bVar11;
                iVar14 = iVar14 + 2;
                pbVar8[1] = bVar13;
                pbVar9 = pbVar8 + 2;
                local_24 = pbVar8;
            }
            if (iVar7 <= iVar15) goto LAB_004b7734;
            pbVar5 = pbVar5 + iVar12;
            bVar13 = *pbVar5;
            pbVar8 = pbVar9;
        } while (1);
    }
    DAT_00aa6d04[0] = 0x80;
    DAT_00aa6d00 = 5;
    goto LAB_004b7741;
LAB_004b7734:
    *pbVar9 = 0x80;
    DAT_00aa6d00 = (unsigned int)(iVar14 + 5);
LAB_004b7741:
    uVar4 = DAT_00aa6d00;
    local_1c = local_1c + DAT_00aa6d00;
    if ((int)((unsigned int)iVar2 * (unsigned int)iVar3) < (int)local_1c) {
        _free(_Memory);
        local_1c = (unsigned int)*param_1 * (unsigned int)param_1[1] + 1;
        goto LAB_004b77a9;
    }
    puVar16 = &DAT_00aa6d00;
    puVar19 = local_8;
    for (uVar10 = DAT_00aa6d00 >> 2; uVar10 != 0; uVar10 = uVar10 - 1) {
        *puVar19 = *puVar16;
        puVar16 = puVar16 + 1;
        puVar19 = puVar19 + 1;
    }
    for (uVar10 = uVar4 & 3; uVar10 != 0; uVar10 = uVar10 - 1) {
        *(unsigned char *)puVar19 = *(unsigned char *)puVar16;
        puVar16 = (unsigned int *)((int)puVar16 + 1);
        puVar19 = (unsigned int *)((int)puVar19 + 1);
    }
    local_8 = (unsigned int *)((int)local_8 + uVar4);
    local_18 = local_18 + *param_1;
    local_4 = local_4 + 1;
    if (param_1[1] <= local_4) goto LAB_004b77a9;
    goto LAB_004b7630;
}

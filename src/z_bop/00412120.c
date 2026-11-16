/* SPDX-License-Identifier: MIT */
/* z_bop 00412120 */
typedef struct _iobuf FILE;

extern char *PTR_PTR_0051bd08[];
extern char s_binary_0051bd70[];
extern char s__256s_0051bd80[];
extern char DAT_005506f8[];
extern char DAT_005505f8[];
extern char DAT_0051be28[];
extern char DAT_0051be10[];
extern char DAT_0051bdf0[];
extern char DAT_0051bdec[];
extern char DAT_0051be08[];
extern char DAT_0051be04[];
extern char s_false_0051bdfc[];
extern char DAT_0051bdf8[];
extern char s_READ_NODE_ERROR__0051be14[];
extern char s_READ_NODE_WARNING__0051bdd8[];
extern char s_Node_Type_Not_Supported__ignored_0051bda0[];
extern char s__e__e__e_0051bdcc[];
extern char s__e__e_0051bdc4[];
extern char s_DEFAULT_0051add0[];

extern int _fscanf(FILE *, const char *, ...);
extern int _printf(const char *, ...);
extern int __strnicmp(const char *, const char *, unsigned int);
extern int __strcmpi(const char *, const char *);
extern long _ftell(FILE *);
extern int _fseek(FILE *, long, int);
extern int _fgetc(FILE *);
extern int _atoi(const char *);

extern void FUN_00413c60(FILE *);
extern void FUN_004138a0(FILE *);
extern int FUN_00413aa0(FILE *);
extern int FUN_00413a30(FILE *);
extern void FUN_00413970(FILE *);
extern int FUN_00413bf0(FILE *);
extern void FUN_004138e0(FILE *);
extern int FUN_00413b80(FILE *);
extern void FUN_00413920(FILE *);
extern void FUN_004139f0(FILE *);
extern int FUN_00413b10(FILE *);
extern void FUN_004139b0(FILE *);
unsigned int FUN_00412120(FILE *param_1, int *param_2, unsigned int *param_3);

unsigned int FUN_00412120(FILE *param_1, int *param_2, unsigned int *param_3)
{
    char *puVar1;
    char uVar2;
    char cVar3;
    int iVar4;
    int iVar5;
    int *piVar6;
    char **ppuVar7;
    long _Offset;
    unsigned int uVar8;
    unsigned int uVar9;
    unsigned int uVar10;
    int *piVar11;
    char **ppuVar12;
    char *pcVar13;
    char *pcVar14;
    unsigned int local_38;
    int local_34;
    char **local_30;
    char **local_2c;
    int local_28;
    int local_24;
    char **local_20;
    int local_1c;
    int local_18;
    int local_14;
    int local_10;
    int *local_c;
    int local_8;
    int local_4;

    if (param_2 != (int *)0x0) {
        *param_3 = (unsigned int)param_2;
    }
    local_38 = 0;
    iVar4 = _fscanf(param_1, s__256s_0051bd80, DAT_005506f8);
    if (iVar4 != 1) {
        return 0xffffffff;
    }
    iVar4 = __strnicmp(DAT_005506f8, DAT_0051be28, 1);
    if (iVar4 == 0) {
        FUN_00413c60(param_1);
    }
    local_2c = (char **)PTR_PTR_0051bd08;
    iVar4 = 0;
    do {
        uVar8 = 0xffffffff;
        pcVar13 = DAT_005506f8;
        do {
            if (uVar8 == 0) break;
            uVar8 = uVar8 - 1;
            cVar3 = *pcVar13;
            pcVar13 = pcVar13 + 1;
        } while (cVar3 != '\0');
        iVar5 = __strnicmp(DAT_005506f8, *(char **)*local_2c, ~uVar8 - 1);
        if (iVar5 == 0) break;
        iVar4 = iVar4 + 1;
        local_2c = local_2c + 1;
    } while (local_2c < (char **)s_binary_0051bd70);
    local_2c = (char **)0x0;
    if (iVar4 == 8) {
        iVar4 = _fscanf(param_1, s__256s_0051bd80, DAT_005506f8);
        if (iVar4 != 1) {
            _printf(s_READ_NODE_ERROR__0051be14);
            return 0;
        }
        local_38 = 0xffffffff;
        pcVar13 = DAT_005506f8;
        do {
            if (local_38 == 0) break;
            local_38 = local_38 - 1;
            cVar3 = *pcVar13;
            pcVar13 = pcVar13 + 1;
        } while (cVar3 != '\0');
        local_38 = ~local_38;
        if (param_2 != (int *)0x0) {
            uVar8 = 0xffffffff;
            pcVar13 = DAT_005506f8;
            do {
                pcVar14 = pcVar13;
                if (uVar8 == 0) break;
                uVar8 = uVar8 - 1;
                pcVar14 = pcVar13 + 1;
                cVar3 = *pcVar13;
                pcVar13 = pcVar14;
            } while (cVar3 != '\0');
            uVar8 = ~uVar8;
            piVar6 = (int *)(pcVar14 + -uVar8);
            piVar11 = param_2;
            for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
                *piVar11 = *piVar6;
                piVar6 = piVar6 + 1;
                piVar11 = piVar11 + 1;
            }
            for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
                *(char *)piVar11 = (char)*piVar6;
                piVar6 = (int *)((int)piVar6 + 1);
                piVar11 = (int *)((int)piVar11 + 1);
            }
            local_2c = (char **)param_2;
            param_2 = (int *)((int)param_2 + local_38);
            *param_3 = (unsigned int)param_2;
        }
        local_20 = (char **)local_38;
        iVar4 = _fscanf(param_1, s__256s_0051bd80, DAT_005506f8);
        if (iVar4 != 1) {
            _printf(s_READ_NODE_ERROR__0051be14);
            return (unsigned int)local_20;
        }
        local_20 = (char **)PTR_PTR_0051bd08;
        iVar4 = 0;
        do {
            uVar8 = 0xffffffff;
            pcVar13 = DAT_005506f8;
            do {
                if (uVar8 == 0) break;
                uVar8 = uVar8 - 1;
                cVar3 = *pcVar13;
                pcVar13 = pcVar13 + 1;
            } while (cVar3 != '\0');
            iVar5 = __strnicmp(DAT_005506f8, *(char **)*local_20, ~uVar8 - 1);
            if (iVar5 == 0) break;
            iVar4 = iVar4 + 1;
            local_20 = local_20 + 1;
        } while (local_20 < (char **)s_binary_0051bd70);
    }
    if (0x19 < iVar4) {
        _printf(s_Node_Type_Not_Supported__ignored_0051bda0);
        iVar4 = FUN_00413b10(param_1);
        if (iVar4 != 0) {
            FUN_004138a0(param_1);
            FUN_004139b0(param_1);
        }
        return local_38;
    }
    piVar6 = param_2;
    if (param_2 != (int *)0x0) {
        *param_2 = (int)local_2c;
        param_2[1] = iVar4;
        piVar6 = param_2 + 6;
        param_2[4] = *(int *)(((char **)PTR_PTR_0051bd08)[iVar4] + 4);
        local_c = param_2;
    }
    ppuVar12 = (char **)piVar6;
    if (piVar6 != (int *)0x0) {
        local_c[5] = (int)piVar6;
        ppuVar12 = (char **)(piVar6 + *(int *)(((char **)PTR_PTR_0051bd08)[iVar4] + 4) * 3);
        local_2c = (char **)piVar6;
    }
    local_30 = (char **)PTR_PTR_0051bd08 + iVar4;
    local_38 = local_38 + 0x18 + *(int *)(((char **)PTR_PTR_0051bd08)[iVar4] + 4) * 0xc;
    if (iVar4 == 7) {
        iVar4 = FUN_00413a30(param_1);
        if (iVar4 == 0) {
            iVar4 = _fscanf(param_1, s__256s_0051bd80, DAT_005506f8);
            if (iVar4 != 1) {
                _printf(s_READ_NODE_ERROR__0051be14);
                return local_38;
            }
            uVar8 = 0xffffffff;
            pcVar13 = DAT_005506f8;
            do {
                if (uVar8 == 0) break;
                uVar8 = uVar8 - 1;
                cVar3 = *pcVar13;
                pcVar13 = pcVar13 + 1;
            } while (cVar3 != '\0');
            if (ppuVar12 != (char **)0x0) {
                uVar9 = 0xffffffff;
                pcVar13 = DAT_005506f8;
                do {
                    pcVar14 = pcVar13;
                    if (uVar9 == 0) break;
                    uVar9 = uVar9 - 1;
                    pcVar14 = pcVar13 + 1;
                    cVar3 = *pcVar13;
                    pcVar13 = pcVar14;
                } while (cVar3 != '\0');
                uVar9 = ~uVar9;
                piVar6 = (int *)(pcVar14 + -uVar9);
                piVar11 = (int *)ppuVar12;
                for (uVar10 = uVar9 >> 2; uVar10 != 0; uVar10 = uVar10 - 1) {
                    *piVar11 = *piVar6;
                    piVar6 = piVar6 + 1;
                    piVar11 = piVar11 + 1;
                }
                for (uVar9 = uVar9 & 3; uVar9 != 0; uVar9 = uVar9 - 1) {
                    *(char *)piVar11 = (char)*piVar6;
                    piVar6 = (int *)((int)piVar6 + 1);
                    piVar11 = (int *)((int)piVar11 + 1);
                }
                local_2c[2] = (char *)ppuVar12;
                ppuVar12 = (char **)((int)ppuVar12 + ~uVar8);
                local_2c[1] = (char *)0x1;
                *local_2c = (char *)0x1;
            }
            local_38 = local_38 + ~uVar8;
        } else {
            FUN_00413970(param_1);
            if (ppuVar12 != (char **)0x0) {
                local_2c[2] = (char *)ppuVar12;
                local_2c[1] = (char *)0x1;
                *local_2c = (char *)0x1;
            }
            while (1) {
                iVar4 = _fgetc(param_1);
                cVar3 = (char)iVar4;
                if (cVar3 == -1) {
                    _printf(s_READ_NODE_ERROR__0051be14);
                    return local_38;
                }
                local_38 = local_38 + 1;
                if (cVar3 == '\"') break;
                if (ppuVar12 != (char **)0x0) {
                    *(char *)ppuVar12 = cVar3;
                    ppuVar12 = (char **)((int)ppuVar12 + 1);
                }
            }
            if (ppuVar12 != (char **)0x0) {
                *(char *)ppuVar12 = '\0';
                ppuVar12 = (char **)((int)ppuVar12 + 1);
            }
        }
        if (ppuVar12 != (char **)0x0) {
            local_c[3] = 0;
            local_c[2] = 0;
        }
        return local_38;
    }
    FUN_004138a0(param_1);
    iVar4 = 0;
    piVar6 = (int *)(*local_30 + 4);
    if (0 < *(int *)(*local_30 + 4)) {
        do {
            DAT_005505f8[iVar4] = 0;
            iVar4 = iVar4 + 1;
        } while (iVar4 < *piVar6);
    }
    local_10 = 0;
    if (0 < *piVar6) {
        do {
            iVar4 = FUN_00413aa0(param_1);
            if (iVar4 != 0) break;
            local_20 = (char **)_ftell(param_1);
            iVar4 = _fscanf(param_1, s__256s_0051bd80, DAT_005506f8);
            if (iVar4 != 1) {
                _printf(s_READ_NODE_ERROR__0051be14);
                return local_38;
            }
            local_1c = 0;
            if (0 < *(int *)(*local_30 + 4)) {
                iVar4 = 0;
                do {
                    uVar8 = 0xffffffff;
                    pcVar13 = DAT_005506f8;
                    do {
                        if (uVar8 == 0) break;
                        uVar8 = uVar8 - 1;
                        cVar3 = *pcVar13;
                        pcVar13 = pcVar13 + 1;
                    } while (cVar3 != '\0');
                    iVar5 = __strnicmp(DAT_005506f8,
                                       (char *)**(int **)(*(int *)(*local_30 + 8) + iVar4), ~uVar8 - 1);
                    if (iVar5 == 0) break;
                    iVar4 = iVar4 + 4;
                    local_1c = local_1c + 1;
                } while (local_1c < *(int *)(*local_30 + 4));
            }
            puVar1 = *local_30;
            if (*(int *)(puVar1 + 4) == local_1c) {
                _fseek(param_1, (long)local_20, 0);
                break;
            }
            DAT_005505f8[local_1c] = 1;
            local_28 = local_1c * 4;
            piVar6 = (int *)(local_2c + local_1c * 3);
            switch (*(int *)(*(int *)(*(int *)(puVar1 + 8) + local_28) + 4)) {
            case 1:
                iVar4 = FUN_00413a30(param_1);
                if (iVar4 == 0) {
                    if (ppuVar12 == (char **)0x0) {
                        iVar4 = _fscanf(param_1, s__256s_0051bd80, DAT_005506f8);
                        if (iVar4 != 1) {
                            _printf(s_READ_NODE_ERROR__0051be14);
                            return local_38;
                        }
                        uVar8 = 0xffffffff;
                        pcVar13 = DAT_005506f8;
                        do {
                            if (uVar8 == 0) break;
                            uVar8 = uVar8 - 1;
                            cVar3 = *pcVar13;
                            pcVar13 = pcVar13 + 1;
                        } while (cVar3 != '\0');
                        local_38 = local_38 + ~uVar8;
                    } else {
                        piVar6[2] = (int)ppuVar12;
                        piVar6[1] = 1;
                        *piVar6 = 1;
                        iVar4 = _fscanf(param_1, DAT_0051be10, ppuVar12);
                        if (iVar4 != 1) {
                            _printf(s_READ_NODE_ERROR__0051be14);
                            return local_38;
                        }
                        uVar8 = 0xffffffff;
                        piVar6 = (int *)ppuVar12;
                        do {
                            if (uVar8 == 0) break;
                            uVar8 = uVar8 - 1;
                            cVar3 = *(char *)piVar6;
                            piVar6 = (int *)((int)piVar6 + 1);
                        } while (cVar3 != '\0');
                        ppuVar12 = (char **)((int)ppuVar12 + ~uVar8);
                        local_38 = local_38 + ~uVar8;
                    }
                } else {
                    FUN_00413970(param_1);
                    if (ppuVar12 != (char **)0x0) {
                        piVar6[2] = (int)ppuVar12;
                        piVar6[1] = 1;
                        *piVar6 = 1;
                    }
                    while (1) {
                        iVar4 = _fgetc(param_1);
                        cVar3 = (char)iVar4;
                        if (cVar3 == -1) {
                            _printf(s_READ_NODE_ERROR__0051be14);
                            return local_38;
                        }
                        local_38 = local_38 + 1;
                        if (cVar3 == '\"') break;
                        if (ppuVar12 != (char **)0x0) {
                            *(char *)ppuVar12 = cVar3;
                            ppuVar12 = (char **)((int)ppuVar12 + 1);
                        }
                    }
                    if (ppuVar12 != (char **)0x0) {
                        *(char *)ppuVar12 = '\0';
                        ppuVar12 = (char **)((int)ppuVar12 + 1);
                    }
                }
                break;
            case 3:
                if (ppuVar12 == (char **)0x0) {
                    uVar8 = FUN_00412120(param_1, (int *)0x0, (unsigned int *)0x0);
                    local_38 = local_38 + uVar8;
                } else {
                    uVar8 = FUN_00412120(param_1, (int *)ppuVar12, (unsigned int *)(piVar6 + 2));
                    local_38 = local_38 + uVar8;
                    ppuVar12 = (char **)((int)ppuVar12 + uVar8);
                    piVar6[1] = 1;
                    *piVar6 = 3;
                }
                break;
            case 4:
                iVar4 = FUN_00413bf0(param_1);
                if (iVar4 == 0) {
                    if (ppuVar12 != (char **)0x0) {
                        piVar6[2] = (int)ppuVar12;
                        ppuVar12 = ppuVar12 + 1;
                        piVar6[1] = 1;
                        *piVar6 = 4;
                        if (ppuVar12 != (char **)0x0) {
                            uVar8 = FUN_00412120(param_1, (int *)ppuVar12, (unsigned int *)(piVar6[2] + 4));
                            local_38 = local_38 + 4 + uVar8;
                            ppuVar12 = (char **)((int)ppuVar12 + uVar8);
                            break;
                        }
                    }
                    uVar8 = FUN_00412120(param_1, (int *)0x0, (unsigned int *)0x0);
                    local_38 = local_38 + 4 + uVar8;
                } else {
                    FUN_004138e0(param_1);
                    iVar4 = 0;
                    local_20 = (char **)_ftell(param_1);
                    iVar5 = FUN_00413b80(param_1);
                    while (iVar5 == 0) {
                        uVar8 = FUN_00412120(param_1, (int *)0x0, (unsigned int *)0x0);
                        if (uVar8 != 0) {
                            iVar4 = iVar4 + 1;
                            FUN_00413920(param_1);
                        }
                        iVar5 = FUN_00413b80(param_1);
                    }
                    _fseek(param_1, (long)local_20, 0);
                    local_38 = local_38 + iVar4 * 4;
                    if (ppuVar12 != (char **)0x0) {
                        piVar6[2] = (int)ppuVar12;
                        ppuVar12 = ppuVar12 + iVar4;
                        piVar6[1] = iVar4;
                        *piVar6 = 4;
                    }
                    iVar4 = 0;
                    iVar5 = FUN_00413b80(param_1);
                    while (iVar5 == 0) {
                        if (ppuVar12 == (char **)0x0) {
                            uVar8 = FUN_00412120(param_1, (int *)0x0, (unsigned int *)0x0);
                            if (uVar8 != 0) goto LAB_00412684;
                        } else {
                            uVar8 = FUN_00412120(param_1, (int *)ppuVar12, (unsigned int *)(piVar6[2] + iVar4 * 4));
                            if (uVar8 != 0) {
                                ppuVar12 = (char **)((int)ppuVar12 + uVar8);
                                iVar4 = iVar4 + 1;
LAB_00412684:
                                local_38 = local_38 + uVar8;
                                FUN_00413920(param_1);
                            }
                        }
                        iVar5 = FUN_00413b80(param_1);
                    }
                    FUN_004139f0(param_1);
                }
                break;
            case 5:
                iVar4 = _fscanf(param_1, DAT_0051bdf0, &local_34);
                if (iVar4 != 1) {
                    _printf(s_READ_NODE_ERROR__0051be14);
                    return local_38;
                }
                if (ppuVar12 != (char **)0x0) {
                    piVar6[2] = (int)ppuVar12;
                    piVar6[1] = 1;
                    *piVar6 = 5;
                    *ppuVar12 = (char *)local_34;
                    ppuVar12 = ppuVar12 + 1;
                }
                local_38 = local_38 + 4;
                break;
            case 6:
                iVar4 = FUN_00413bf0(param_1);
                if (iVar4 == 0) {
                    if (ppuVar12 != (char **)0x0) {
                        piVar6[2] = (int)ppuVar12;
                    }
                    iVar4 = _fscanf(param_1, DAT_0051bdf0, &local_34);
                    if (iVar4 != 1) {
                        _printf(s_READ_NODE_ERROR__0051be14);
                        return local_38;
                    }
                    if (ppuVar12 != (char **)0x0) {
                        *ppuVar12 = (char *)local_34;
                        ppuVar12 = ppuVar12 + 1;
                    }
                    local_38 = local_38 + 4;
                    if (ppuVar12 != (char **)0x0) {
                        piVar6[1] = 1;
                        *piVar6 = 6;
                    }
                } else {
                    FUN_004138e0(param_1);
                    if (ppuVar12 != (char **)0x0) {
                        piVar6[2] = (int)ppuVar12;
                    }
                    iVar4 = 0;
                    iVar5 = FUN_00413b80(param_1);
                    while (iVar5 == 0) {
                        iVar5 = _fscanf(param_1, DAT_0051bdf0, &local_34);
                        if (iVar5 != 1) {
                            _printf(s_READ_NODE_ERROR__0051be14);
                            return local_38;
                        }
                        if (ppuVar12 != (char **)0x0) {
                            *ppuVar12 = (char *)local_34;
                            ppuVar12 = ppuVar12 + 1;
                        }
                        iVar4 = iVar4 + 1;
                        local_38 = local_38 + 4;
                        FUN_00413920(param_1);
                        iVar5 = FUN_00413b80(param_1);
                    }
                    if (ppuVar12 != (char **)0x0) {
                        piVar6[1] = iVar4;
                        *piVar6 = 6;
                    }
                    FUN_004139f0(param_1);
                }
                break;
            case 7:
                iVar4 = _fscanf(param_1, DAT_0051bdec, &local_24);
                if (iVar4 != 1) {
                    _printf(s_READ_NODE_WARNING__0051bdd8);
                }
                if (ppuVar12 != (char **)0x0) {
                    piVar6[2] = (int)ppuVar12;
                    piVar6[1] = 0;
                    *piVar6 = 5;
                    *ppuVar12 = (char *)local_24;
                    ppuVar12 = ppuVar12 + 1;
                }
                local_38 = local_38 + 4;
                break;
            case 8:
                iVar4 = FUN_00413bf0(param_1);
                if (iVar4 == 0) {
                    if (ppuVar12 != (char **)0x0) {
                        piVar6[2] = (int)ppuVar12;
                    }
                    iVar4 = _fscanf(param_1, DAT_0051bdec, &local_24);
                    if (iVar4 != 1) {
                        _printf(s_READ_NODE_WARNING__0051bdd8);
                    }
                    if (ppuVar12 != (char **)0x0) {
                        *ppuVar12 = (char *)local_24;
                        ppuVar12 = ppuVar12 + 1;
                    }
                    local_38 = local_38 + 4;
                    if (ppuVar12 != (char **)0x0) {
                        piVar6[1] = 1;
                        *piVar6 = 8;
                    }
                } else {
                    FUN_004138e0(param_1);
                    if (ppuVar12 != (char **)0x0) {
                        piVar6[2] = (int)ppuVar12;
                    }
                    iVar4 = 0;
                    iVar5 = FUN_00413b80(param_1);
                    while (iVar5 == 0) {
                        iVar5 = _fscanf(param_1, DAT_0051bdec, &local_24);
                        if (iVar5 != 1) {
                            _printf(s_READ_NODE_WARNING__0051bdd8);
                        }
                        if (ppuVar12 != (char **)0x0) {
                            *ppuVar12 = (char *)local_24;
                            ppuVar12 = ppuVar12 + 1;
                        }
                        iVar4 = iVar4 + 1;
                        local_38 = local_38 + 4;
                        FUN_00413920(param_1);
                        iVar5 = FUN_00413b80(param_1);
                    }
                    if (ppuVar12 != (char **)0x0) {
                        piVar6[1] = iVar4;
                        *piVar6 = 8;
                    }
                    FUN_004139f0(param_1);
                }
                break;
            case 9:
                iVar4 = _fscanf(param_1, s__256s_0051bd80, DAT_005506f8);
                if (iVar4 != 1) {
                    _printf(s_READ_NODE_ERROR__0051be14);
                    return local_38;
                }
                iVar4 = __strcmpi(DAT_005506f8, DAT_0051be08);
                if ((iVar4 == 0) || (iVar4 = __strcmpi(DAT_005506f8, DAT_0051be04), iVar4 == 0)) {
                    uVar2 = 1;
                } else {
                    iVar4 = __strcmpi(DAT_005506f8, s_false_0051bdfc);
                    if ((iVar4 != 0) && (iVar4 = __strcmpi(DAT_005506f8, DAT_0051bdf8), iVar4 != 0)) {
                        _printf(s_READ_NODE_ERROR__0051be14);
                        return local_38;
                    }
                    uVar2 = 0;
                }
                local_38 = local_38 + 1;
                if (ppuVar12 != (char **)0x0) {
                    piVar6[2] = (int)ppuVar12;
                    piVar6[1] = 1;
                    *piVar6 = 9;
                    *(char *)ppuVar12 = uVar2;
                    ppuVar12 = (char **)((int)ppuVar12 + 1);
                }
                break;
            case 0xb:
            case 0xd:
                iVar4 = _fscanf(param_1, s__e__e__e_0051bdcc, &local_18, &local_14, &local_8);
                if (iVar4 != 3) {
                    _printf(s_READ_NODE_WARNING__0051bdd8);
                }
                piVar11 = (int *)ppuVar12;
                if (ppuVar12 != (char **)0x0) {
                    piVar6[2] = (int)ppuVar12;
                    piVar6[1] = 1;
                    *piVar6 = *(int *)(*(int *)(*(int *)(*local_30 + 8) + local_28) + 4);
                    piVar11 = (int *)(ppuVar12 + 3);
                    *ppuVar12 = (char *)local_18;
                    ppuVar12[1] = (char *)local_14;
                    ppuVar12[2] = (char *)local_8;
                }
                local_38 = local_38 + 0xc;
                ppuVar12 = (char **)piVar11;
                break;
            case 0xc:
            case 0xe:
                iVar4 = FUN_00413bf0(param_1);
                if (iVar4 == 0) {
                    if (ppuVar12 != (char **)0x0) {
                        piVar6[2] = (int)ppuVar12;
                    }
                    iVar4 = _fscanf(param_1, s__e__e__e_0051bdcc, &local_18, &local_14, &local_8);
                    if (iVar4 != 3) {
                        _printf(s_READ_NODE_WARNING__0051bdd8);
                    }
                    piVar11 = (int *)ppuVar12;
                    if (ppuVar12 != (char **)0x0) {
                        *ppuVar12 = (char *)local_18;
                        piVar11 = (int *)(ppuVar12 + 3);
                        ppuVar12[1] = (char *)local_14;
                        ppuVar12[2] = (char *)local_8;
                    }
                    local_38 = local_38 + 0xc;
                    ppuVar12 = (char **)piVar11;
joined_r0x004133c1:
                    if (ppuVar12 != (char **)0x0) {
                        piVar6[1] = 1;
                        *piVar6 = *(int *)(*(int *)(*(int *)(*local_30 + 8) + local_28) + 4);
                    }
                } else {
                    FUN_004138e0(param_1);
                    if (ppuVar12 != (char **)0x0) {
                        piVar6[2] = (int)ppuVar12;
                    }
                    iVar4 = 0;
                    iVar5 = FUN_00413b80(param_1);
                    while (iVar5 == 0) {
                        iVar5 = _fscanf(param_1, s__e__e__e_0051bdcc, &local_18, &local_14, &local_8);
                        if (iVar5 != 3) {
                            _printf(s_READ_NODE_WARNING__0051bdd8);
                        }
                        piVar11 = (int *)ppuVar12;
                        if (ppuVar12 != (char **)0x0) {
                            *ppuVar12 = (char *)local_18;
                            piVar11 = (int *)(ppuVar12 + 3);
                            ppuVar12[1] = (char *)local_14;
                            ppuVar12[2] = (char *)local_8;
                        }
                        iVar4 = iVar4 + 1;
                        local_38 = local_38 + 0xc;
                        FUN_00413920(param_1);
                        iVar5 = FUN_00413b80(param_1);
                        ppuVar12 = (char **)piVar11;
                    }
                    if (ppuVar12 != (char **)0x0) {
                        piVar6[1] = iVar4;
                        *piVar6 = *(int *)(*(int *)(*(int *)(*local_30 + 8) + local_28) + 4);
                    }
                    FUN_004139f0(param_1);
                }
                break;
            case 0xf:
                if (ppuVar12 != (char **)0x0) {
                    piVar6[2] = (int)ppuVar12;
                    piVar6[1] = 1;
                    *piVar6 = *(int *)(*(int *)(*(int *)(*local_30 + 8) + local_28) + 4);
                }
                iVar4 = 0x10;
                local_38 = local_38 + 0x40;
                do {
                    iVar5 = _fscanf(param_1, DAT_0051bdec, &local_24);
                    if (iVar5 != 1) {
                        _printf(s_READ_NODE_WARNING__0051bdd8);
                    }
                    if (ppuVar12 != (char **)0x0) {
                        *ppuVar12 = (char *)local_24;
                        ppuVar12 = ppuVar12 + 1;
                    }
                    iVar4 = iVar4 + -1;
                } while (iVar4 != 0);
                break;
            case 0x10:
                iVar4 = FUN_00413bf0(param_1);
                if (iVar4 == 0) {
                    if (ppuVar12 != (char **)0x0) {
                        piVar6[2] = (int)ppuVar12;
                    }
                    local_34 = 0;
                    do {
                        iVar4 = _fscanf(param_1, DAT_0051bdec, &local_24);
                        if (iVar4 != 1) {
                            _printf(s_READ_NODE_WARNING__0051bdd8);
                        }
                        if (ppuVar12 != (char **)0x0) {
                            *ppuVar12 = (char *)local_24;
                            ppuVar12 = ppuVar12 + 1;
                        }
                        local_38 = local_38 + 4;
                        local_34 = local_34 + 1;
                    } while (local_34 < 0x10);
                    goto joined_r0x004133c1;
                }
                FUN_004138e0(param_1);
                if (ppuVar12 != (char **)0x0) {
                    piVar6[2] = (int)ppuVar12;
                }
                iVar4 = 0;
                iVar5 = FUN_00413b80(param_1);
                while (iVar5 == 0) {
                    local_34 = 0;
                    do {
                        iVar5 = _fscanf(param_1, DAT_0051bdec, &local_24);
                        if (iVar5 != 1) {
                            _printf(s_READ_NODE_WARNING__0051bdd8);
                        }
                        if (ppuVar12 != (char **)0x0) {
                            *ppuVar12 = (char *)local_24;
                            ppuVar12 = ppuVar12 + 1;
                        }
                        local_38 = local_38 + 4;
                        local_34 = local_34 + 1;
                    } while (local_34 < 0x10);
                    iVar4 = iVar4 + 1;
                    FUN_00413920(param_1);
                    iVar5 = FUN_00413b80(param_1);
                }
                if (ppuVar12 != (char **)0x0) {
                    piVar6[1] = iVar4;
                    *piVar6 = *(int *)(*(int *)(*(int *)(*local_30 + 8) + local_28) + 4);
                }
                FUN_004139f0(param_1);
                break;
            case 0x11:
                iVar4 = _fscanf(param_1, s__e__e_0051bdc4, &local_18, &local_14, &local_8);
                if (iVar4 != 2) {
                    _printf(s_READ_NODE_WARNING__0051bdd8);
                }
                piVar11 = (int *)ppuVar12;
                if (ppuVar12 != (char **)0x0) {
                    piVar6[2] = (int)ppuVar12;
                    piVar6[1] = 1;
                    piVar11 = (int *)(ppuVar12 + 2);
                    *piVar6 = *(int *)(*(int *)(*(int *)(*local_30 + 8) + local_28) + 4);
                    *ppuVar12 = (char *)local_18;
                    ppuVar12[1] = (char *)local_14;
                }
                local_38 = local_38 + 8;
                ppuVar12 = (char **)piVar11;
                break;
            case 0x12:
                iVar4 = FUN_00413bf0(param_1);
                if (iVar4 == 0) {
                    if (ppuVar12 != (char **)0x0) {
                        piVar6[2] = (int)ppuVar12;
                    }
                    iVar4 = _fscanf(param_1, s__e__e_0051bdc4, &local_18, &local_14);
                    if (iVar4 != 2) {
                        _printf(s_READ_NODE_WARNING__0051bdd8);
                    }
                    piVar11 = (int *)ppuVar12;
                    if (ppuVar12 != (char **)0x0) {
                        *ppuVar12 = (char *)local_18;
                        piVar11 = (int *)(ppuVar12 + 2);
                        ppuVar12[1] = (char *)local_14;
                    }
                    local_38 = local_38 + 8;
                    ppuVar12 = (char **)piVar11;
                    goto joined_r0x004133c1;
                }
                FUN_004138e0(param_1);
                if (ppuVar12 != (char **)0x0) {
                    piVar6[2] = (int)ppuVar12;
                }
                iVar4 = 0;
                iVar5 = FUN_00413b80(param_1);
                while (iVar5 == 0) {
                    iVar5 = _fscanf(param_1, s__e__e_0051bdc4, &local_18, &local_14);
                    if (iVar5 != 2) {
                        _printf(s_READ_NODE_WARNING__0051bdd8);
                    }
                    piVar11 = (int *)ppuVar12;
                    if (ppuVar12 != (char **)0x0) {
                        *ppuVar12 = (char *)local_18;
                        piVar11 = (int *)(ppuVar12 + 2);
                        ppuVar12[1] = (char *)local_14;
                    }
                    iVar4 = iVar4 + 1;
                    local_38 = local_38 + 8;
                    FUN_00413920(param_1);
                    iVar5 = FUN_00413b80(param_1);
                    ppuVar12 = (char **)piVar11;
                }
                if (ppuVar12 != (char **)0x0) {
                    piVar6[1] = iVar4;
                    *piVar6 = *(int *)(*(int *)(*(int *)(*local_30 + 8) + local_28) + 4);
                }
                FUN_004139f0(param_1);
                break;
            case 0x13:
                if (ppuVar12 != (char **)0x0) {
                    piVar6[2] = (int)ppuVar12;
                    piVar6[1] = 1;
                    *piVar6 = *(int *)(*(int *)(*(int *)(*local_30 + 8) + local_28) + 4);
                }
                iVar4 = 4;
                local_38 = local_38 + 0x10;
                do {
                    iVar5 = _fscanf(param_1, DAT_0051bdec, &local_24);
                    if (iVar5 != 1) {
                        _printf(s_READ_NODE_WARNING__0051bdd8);
                    }
                    if (ppuVar12 != (char **)0x0) {
                        *ppuVar12 = (char *)local_24;
                        ppuVar12 = ppuVar12 + 1;
                    }
                    iVar4 = iVar4 + -1;
                } while (iVar4 != 0);
                break;
            case 0x14:
                iVar4 = FUN_00413bf0(param_1);
                if (iVar4 == 0) {
                    if (ppuVar12 != (char **)0x0) {
                        piVar6[2] = (int)ppuVar12;
                    }
                    local_34 = 0;
                    do {
                        iVar4 = _fscanf(param_1, DAT_0051bdec, &local_24);
                        if (iVar4 != 1) {
                            _printf(s_READ_NODE_WARNING__0051bdd8);
                        }
                        if (ppuVar12 != (char **)0x0) {
                            *ppuVar12 = (char *)local_24;
                            ppuVar12 = ppuVar12 + 1;
                        }
                        local_38 = local_38 + 4;
                        local_34 = local_34 + 1;
                    } while (local_34 < 4);
                    goto joined_r0x004133c1;
                }
                FUN_004138e0(param_1);
                if (ppuVar12 != (char **)0x0) {
                    piVar6[2] = (int)ppuVar12;
                }
                iVar4 = 0;
                iVar5 = FUN_00413b80(param_1);
                while (iVar5 == 0) {
                    local_34 = 0;
                    do {
                        iVar5 = _fscanf(param_1, DAT_0051bdec, &local_24);
                        if (iVar5 != 1) {
                            _printf(s_READ_NODE_WARNING__0051bdd8);
                        }
                        if (ppuVar12 != (char **)0x0) {
                            *ppuVar12 = (char *)local_24;
                            ppuVar12 = ppuVar12 + 1;
                        }
                        local_38 = local_38 + 4;
                        local_34 = local_34 + 1;
                    } while (local_34 < 4);
                    iVar4 = iVar4 + 1;
                    FUN_00413920(param_1);
                    iVar5 = FUN_00413b80(param_1);
                }
                if (ppuVar12 != (char **)0x0) {
                    piVar6[1] = iVar4;
                    *piVar6 = *(int *)(*(int *)(*(int *)(*local_30 + 8) + local_28) + 4);
                }
                FUN_004139f0(param_1);
                break;
            case 0x15:
                iVar4 = FUN_00413aa0(param_1);
                if (iVar4 == 0) {
                    iVar4 = _fscanf(param_1, s__256s_0051bd80, DAT_005506f8);
                    if (iVar4 != 1) {
                        _printf(s_READ_NODE_ERROR__0051be14);
                        return local_38;
                    }
                } else {
                    DAT_005506f8[0] = s_DEFAULT_0051add0[0];
                    DAT_005506f8[1] = s_DEFAULT_0051add0[1];
                    DAT_005506f8[2] = s_DEFAULT_0051add0[2];
                    DAT_005506f8[3] = s_DEFAULT_0051add0[3];
                    DAT_005506f8[4] = s_DEFAULT_0051add0[4];
                    DAT_005506f8[5] = s_DEFAULT_0051add0[5];
                    DAT_005506f8[6] = s_DEFAULT_0051add0[6];
                    DAT_005506f8[7] = s_DEFAULT_0051add0[7];
                }
                piVar11 = *(int **)(*(int *)(*(int *)(*local_30 + 8) + local_28) + 8);
                local_1c = piVar11[1];
                local_4 = piVar11[2];
                local_34 = 0;
                if (0 < *piVar11) {
                    do {
                        iVar4 = __strcmpi(DAT_005506f8, *(char **)(local_1c + local_34 * 4));
                        if (iVar4 == 0) break;
                        local_34 = local_34 + 1;
                    } while (local_34 < *piVar11);
                }
                if (*piVar11 == local_34) {
                    local_34 = _atoi(DAT_005506f8);
                } else {
                    local_34 = *(int *)(local_4 + local_34 * 4);
                }
                piVar11 = (int *)ppuVar12;
                if (ppuVar12 != (char **)0x0) {
                    piVar6[2] = (int)ppuVar12;
                    piVar11 = (int *)(ppuVar12 + 1);
                    piVar6[1] = 1;
                    *piVar6 = 0x15;
                    *ppuVar12 = (char *)local_34;
                }
                local_38 = local_38 + 4;
                ppuVar12 = (char **)piVar11;
                break;
            case 0x16:
                iVar4 = FUN_00413bf0(param_1);
                if (iVar4 == 0) {
                    if (ppuVar12 != (char **)0x0) {
                        piVar6[2] = (int)ppuVar12;
                    }
                    iVar4 = _fscanf(param_1, s__256s_0051bd80, DAT_005506f8);
                    if (iVar4 != 1) {
                        _printf(s_READ_NODE_ERROR__0051be14);
                        return local_38;
                    }
                    piVar11 = *(int **)(*(int *)(*(int *)(*local_30 + 8) + local_28) + 8);
                    local_1c = piVar11[1];
                    local_4 = piVar11[2];
                    local_34 = 0;
                    if (0 < *piVar11) {
                        do {
                            iVar4 = __strcmpi(DAT_005506f8, *(char **)(local_1c + local_34 * 4));
                            if (iVar4 == 0) break;
                            local_34 = local_34 + 1;
                        } while (local_34 < *piVar11);
                    }
                    if (*piVar11 == local_34) {
                        local_34 = _atoi(DAT_005506f8);
                    } else {
                        local_34 = *(int *)(local_4 + local_34 * 4);
                    }
                    if (ppuVar12 != (char **)0x0) {
                        *ppuVar12 = (char *)local_34;
                        ppuVar12 = ppuVar12 + 1;
                    }
                    local_38 = local_38 + 4;
                    goto joined_r0x004133c1;
                }
                FUN_004138e0(param_1);
                if (ppuVar12 != (char **)0x0) {
                    piVar6[2] = (int)ppuVar12;
                }
                local_20 = (char **)0x0;
                iVar4 = FUN_00413b80(param_1);
                while (iVar4 == 0) {
                    iVar4 = _fscanf(param_1, s__256s_0051bd80, DAT_005506f8);
                    if (iVar4 != 1) {
                        _printf(s_READ_NODE_ERROR__0051be14);
                        return local_38;
                    }
                    piVar11 = *(int **)(*(int *)(*(int *)(*local_30 + 8) + local_28) + 8);
                    local_1c = piVar11[1];
                    local_4 = piVar11[2];
                    local_34 = 0;
                    if (0 < *piVar11) {
                        do {
                            iVar4 = __strcmpi(DAT_005506f8, *(char **)(local_1c + local_34 * 4));
                            if (iVar4 == 0) break;
                            local_34 = local_34 + 1;
                        } while (local_34 < *piVar11);
                    }
                    if (*piVar11 == local_34) {
                        local_34 = _atoi(DAT_005506f8);
                    } else {
                        local_34 = *(int *)(local_4 + local_34 * 4);
                    }
                    if (ppuVar12 != (char **)0x0) {
                        *ppuVar12 = (char *)local_34;
                        ppuVar12 = ppuVar12 + 1;
                    }
                    local_38 = local_38 + 4;
                    local_20 = (char **)((int)local_20 + 1);
                    FUN_00413920(param_1);
                    iVar4 = FUN_00413b80(param_1);
                }
                if (ppuVar12 != (char **)0x0) {
                    piVar6[1] = (int)local_20;
                    *piVar6 = *(int *)(*(int *)(*(int *)(*local_30 + 8) + local_28) + 4);
                }
                FUN_004139f0(param_1);
            }
            local_10 = local_10 + 1;
        } while (local_10 < *(int *)(*local_30 + 4));
    }
    local_10 = 0;
    if (0 < *(int *)(*local_30 + 4)) {
        iVar4 = 0;
        ppuVar7 = local_2c;
        do {
            if (DAT_005505f8[local_10] == '\0') {
                if (ppuVar12 != (char **)0x0) {
                    piVar6 = *(int **)(*(int *)(*(int *)(*local_30 + 8) + iVar4) + 0xc);
                    *ppuVar7 = (char *)*piVar6;
                    ppuVar7[1] = (char *)piVar6[1];
                    ppuVar7[2] = (char *)ppuVar12;
                    local_2c = (char **)*(int *)(*(int *)(*(int *)(*local_30 + 8) + iVar4) + 0x14);
                    piVar6 = *(int **)(*(int *)(*(int *)(*local_30 + 8) + iVar4) + 0x10);
                    piVar11 = (int *)ppuVar12;
                    for (uVar8 = (unsigned int)local_2c >> 2; uVar8 != 0; uVar8 = uVar8 - 1) {
                        *piVar11 = *piVar6;
                        piVar6 = piVar6 + 1;
                        piVar11 = piVar11 + 1;
                    }
                    for (uVar8 = (unsigned int)local_2c & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
                        *(char *)piVar11 = (char)*piVar6;
                        piVar6 = (int *)((int)piVar6 + 1);
                        piVar11 = (int *)((int)piVar11 + 1);
                    }
                    ppuVar12 = (char **)
                               ((int)ppuVar12 + *(int *)(*(int *)(*(int *)(*local_30 + 8) + iVar4) + 0x14));
                }
                local_38 = local_38 + *(int *)(*(int *)(*(int *)(*local_30 + 8) + iVar4) + 0x14);
            }
            local_10 = local_10 + 1;
            ppuVar7 = ppuVar7 + 3;
            iVar4 = iVar4 + 4;
        } while (local_10 < *(int *)(*local_30 + 4));
    }
    iVar4 = 0;
    _Offset = _ftell(param_1);
    iVar5 = FUN_00413aa0(param_1);
    while (iVar5 == 0) {
        uVar8 = FUN_00412120(param_1, (int *)0x0, (unsigned int *)0x0);
        if (uVar8 != 0) {
            iVar4 = iVar4 + 1;
        }
        iVar5 = FUN_00413aa0(param_1);
    }
    _fseek(param_1, _Offset, 0);
    if (iVar4 == 0) {
        if (ppuVar12 != (char **)0x0) {
            local_c[3] = 0;
            local_c[2] = 0;
        }
    } else {
        local_38 = local_38 + iVar4 * 4;
        if (ppuVar12 == (char **)0x0) {
            piVar6 = (int *)0x0;
            ppuVar12 = local_2c;
        } else {
            local_c[3] = (int)ppuVar12;
            piVar6 = (int *)(ppuVar12 + iVar4);
            local_c[2] = iVar4;
        }
        iVar4 = FUN_00413aa0(param_1);
        while (iVar4 == 0) {
            if (piVar6 == (int *)0x0) {
                uVar8 = FUN_00412120(param_1, (int *)0x0, (unsigned int *)0x0);
                if (uVar8 != 0) {
                    local_38 = local_38 + uVar8;
                }
            } else {
                uVar8 = FUN_00412120(param_1, piVar6, (unsigned int *)ppuVar12);
                if (uVar8 != 0) {
                    piVar6 = (int *)((int)piVar6 + uVar8);
                    local_38 = local_38 + uVar8;
                    ppuVar12 = ppuVar12 + 1;
                }
            }
            iVar4 = FUN_00413aa0(param_1);
        }
    }
    FUN_004139b0(param_1);
    return local_38;
}

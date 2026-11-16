/* SPDX-License-Identifier: MIT */
/* z_bop 00465a20 */
extern int            DAT_009a8c2c;
extern int            DAT_009a8c24;
extern unsigned char *DAT_00a60a00;
extern unsigned char *DAT_00a60a10;
extern unsigned char *DAT_00a60a20;
extern char          *DAT_00a60a24;
extern unsigned int   DAT_00557360;
extern unsigned int   DAT_00557364;
extern int            DAT_009ecc34;
extern unsigned char  DAT_009d6940;
extern char           DAT_009e9680[];
extern int            DAT_009d8a30[];
extern unsigned int   DAT_00556ed8[];
extern unsigned int   DAT_00557560[];
extern int            DAT_0052342c;
extern int            DAT_005242dc;
extern int            DAT_00523420;
extern int            DAT_009ed23c;
extern int            DAT_00a07cc8;

int            FUN_004ac760(void);
void           FUN_00448da0(void);
void           FUN_00448f90(int, unsigned char *, int);
void           FUN_00417a60(unsigned char *);
int            FUN_00416710(void);
unsigned char *FUN_00416700(void);
int            FUN_00417900(unsigned int *, unsigned char *, int);
int            FUN_0046f700(void);
void           FUN_0046d350(int, unsigned int *, unsigned int);
int            FUN_004aa7f0(void);
unsigned int   FUN_004aa870(void);
int            _sprintf(char *, const char *, ...);
int           *FUN_0046e050(int *, int *);
int            FUN_0046f660(int);
void           FUN_00418650(int);
int           *FUN_00418760(int, unsigned int, unsigned int *);
int            FUN_0046e040(void);
void           FUN_00418f80(int *);
void           FUN_00463cc0(int);
void           FUN_00419570(int *, int, int);
void           FUN_00463c50(int);
void           FUN_00448ed0(void);
void           FUN_004195a0(int, int);

void __cdecl FUN_00465a20(int *param_1)
{
    unsigned char bVar1;
    unsigned char bVar2;
    int iVar3;
    unsigned char *pbVar4;
    unsigned int uVar5;
    int iVar6;
    int *piVar7;
    int *piVar8;
    unsigned char *pbVar9;
    unsigned int *puVar10;
    struct {
        unsigned int   uStack_6c;
        unsigned short uStack_68;
    } rec;
    int iStack_64;
    unsigned int uStack_60;
    unsigned int local_5c;
    int local_58;
    int local_54;
    unsigned char abStack_50[80];

    local_5c = 0;
    if (*param_1 == 0x3d) {
        iVar3 = FUN_004ac760();
        DAT_009a8c2c = (int)((unsigned int)DAT_009a8c2c + (unsigned int)iVar3);
        FUN_00448da0();
        FUN_00448f90(1, DAT_00a60a00, 0x30);
        FUN_00417a60((unsigned char *)(param_1 + 2));
        DAT_00557360 = 0x3c;
        iVar3 = FUN_00416710();
        pbVar4 = FUN_00416700();
        local_58 = FUN_00417900((unsigned int *)&DAT_00557364, pbVar4, iVar3);
        local_58 = (int)((unsigned int)local_58 * 4U);
        uVar5 = (unsigned int)local_58 + 4U;
        for (;;) {
            puVar10 = (unsigned int *)&DAT_00557360;
            iVar3 = FUN_0046f700();
            FUN_0046d350(iVar3, puVar10, uVar5);
            iVar3 = FUN_004aa7f0();
            if ((iVar3 != 0) && (uVar5 = FUN_004aa870(), uVar5 == 0x1b)) {
                FUN_00463c50(DAT_009ecc34);
                DAT_009d6940 = 1;
                DAT_009e9680[DAT_009ecc34 * 0x5bd] = 0;
                iVar3 = DAT_009ecc34;
                DAT_009d8a30[DAT_009ecc34] = 1;
                FUN_00463cc0(iVar3);
                return;
            }
            local_54 = 0;
            do {
                iVar3 = DAT_009a8c2c;
                iVar6 = FUN_004ac760();
                DAT_009a8c24 = (int)((unsigned int)DAT_009a8c24 +
                                     (((unsigned int)DAT_009a8c2c + (unsigned int)iVar6) - (unsigned int)iVar3));
                DAT_009a8c2c = iVar3;
                if (0x1ba8 < DAT_009a8c24) {
                    FUN_00448ed0();
                    return;
                }
                uVar5 = (unsigned int)((int)((unsigned int)0x1ba8 - (unsigned int)DAT_009a8c24) / 0x76);
                if (local_5c != uVar5) {
                    local_5c = uVar5;
                    if ((int)uVar5 < 0x32) {
                        _sprintf((char *)abStack_50, DAT_00a60a24, (int)uVar5 / 2, (uVar5 & 1) * 5);
                        pbVar4 = abStack_50;
                    } else {
                        pbVar4 = DAT_00a60a10;
                        if ((uVar5 & 1) != 0) {
                            pbVar4 = DAT_00a60a20;
                        }
                    }
                    FUN_00448f90(3, pbVar4, 0x30);
                }
                piVar7 = FUN_0046e050(&iStack_64, &local_58);
                iVar3 = DAT_009a8c2c;
                DAT_009a8c2c = iVar3;
                if (piVar7 != (int *)0) {
                    switch (*piVar7) {
                    case 1:
                        iVar3 = FUN_0046f660(iStack_64);
                        if (DAT_009e9680[iVar3 * 0x5bd] == '\0') {
                            DAT_00557360 = 0x15;
                            DAT_00557364 = (unsigned int)iVar3;
                            FUN_0046d350(iStack_64, (unsigned int *)&DAT_00557360, 8);
                        } else {
                            if (0 < (int)DAT_00556ed8[iVar3]) {
                                DAT_00556ed8[iVar3] = 0;
                            }
                            FUN_00418650(iVar3);
                            rec.uStack_6c = rec.uStack_6c & 0xff000000U;
                            bVar1 = *(unsigned char *)(piVar7 + 1);
                            if ((bVar1 & 0x7f) == 0x7f) {
                                __builtin_memcpy(&uVar5, (unsigned char *)piVar7 + 5, 4);
                                if ((bVar1 & 0x80) == 0) {
                                    pbVar4 = (unsigned char *)piVar7 + 9;
                                } else {
                                    pbVar4 = (unsigned char *)piVar7 + 10;
                                    rec.uStack_6c = (rec.uStack_6c & 0xff000000U) |
                                                    ((unsigned int)*((unsigned char *)piVar7 + 9) << 0x10);
                                }
                            } else {
                                uVar5 = DAT_00557560[iVar3];
                                uStack_60 = (unsigned int)(bVar1 & 0x7f);
                                if (uStack_60 < (uVar5 & 0x7f)) {
                                    uVar5 = uVar5 + 0x80;
                                }
                                uVar5 = (uVar5 & 0xffffff80U) | uStack_60;
                                if ((bVar1 & 0x80) == 0) {
                                    pbVar4 = (unsigned char *)piVar7 + 5;
                                } else {
                                    pbVar4 = (unsigned char *)piVar7 + 6;
                                    rec.uStack_6c = (rec.uStack_6c & 0xff000000U) |
                                                    ((unsigned int)*((unsigned char *)piVar7 + 5) << 0x10);
                                }
                            }
                            DAT_00557560[iVar3] = uVar5;
                            rec.uStack_6c = ((((unsigned int)*pbVar4) << 24) | (rec.uStack_6c & 0x00ffffffU)) & 0xfeffffffU;
                            rec.uStack_68 = (unsigned short)(((((unsigned short)((unsigned char)(((pbVar4[1] & 1) * 2) | (*pbVar4 & 1)))) << 8) | pbVar4[1]) & 0xfffe);
                            piVar8 = FUN_00418760(iVar3, uVar5, &rec.uStack_6c);
                            if (piVar8 != (int *)0) {
                                iVar3 = FUN_0046e040();
                                *piVar8 = 1;
                                if (iVar3 == 0) {
                                    *piVar8 = 0;
                                }
                                piVar8[1] = 1;
                            }
                        }
                        break;
                    case 2:
                        FUN_00418f80(piVar7);
                        FUN_004ac760();
                        DAT_009a8c2c = iVar3;
                        if (DAT_009d6940 != 0) {
                            return;
                        }
                        break;
                    case 8:
                        DAT_009d6940 = 1;
                        DAT_0052342c = 1;
                        DAT_009e9680[DAT_009ecc34 * 0x5bd] = 0;
                        return;
                    case 0x15:
                        iVar3 = piVar7[1];
                        if ((-1 < iVar3) && (iVar3 < 8)) {
                            DAT_009d8a30[iVar3] = 1;
                        }
                        if (DAT_009ecc34 == iVar3) {
                            DAT_009d6940 = 1;
                            DAT_009e9680[DAT_009ecc34 * 0x5bd] = 0;
                            iVar3 = DAT_009ecc34;
                            DAT_009d8a30[DAT_009ecc34] = 1;
                            FUN_00463cc0(iVar3);
                            return;
                        }
                        break;
                    case 0x16:
                        iVar3 = FUN_0046f660(iStack_64);
                        if (DAT_009e9680[iVar3 * 0x5bd] == '\0') {
                            DAT_00557360 = 0x15;
                            DAT_00557364 = (unsigned int)iVar3;
                            FUN_0046d350(iStack_64, (unsigned int *)&DAT_00557360, 8);
                        } else {
                            if (0 < (int)DAT_00556ed8[iVar3]) {
                                DAT_00556ed8[iVar3] = 0;
                            }
                            FUN_00418650(iVar3);
                            pbVar4 = (unsigned char *)piVar7 + 5;
                            rec.uStack_6c = 0;
                            rec.uStack_68 = 0;
                            uVar5 = (unsigned int)*(unsigned char *)(piVar7 + 1);
                            if (*(unsigned char *)(piVar7 + 1) != 0) {
                                do {
                                    uStack_60 = uVar5;
                                    bVar1 = *pbVar4;
                                    bVar2 = (unsigned char)(bVar1 & 0x7f);
                                    if (bVar2 == 0x7f) {
                                        __builtin_memcpy(&uVar5, pbVar4 + 1, 4);
                                        if ((bVar1 & 0x80) == 0) {
                                            pbVar9 = pbVar4 + 5;
                                        } else {
                                            pbVar9 = pbVar4 + 6;
                                            rec.uStack_6c = (rec.uStack_6c & 0xffffU) | ((unsigned int)pbVar4[5] << 16);
                                            rec.uStack_6c = rec.uStack_6c & 0x00ffffffU;
                                        }
                                    } else {
                                        uVar5 = DAT_00557560[iVar3];
                                        if ((unsigned int)bVar2 < (uVar5 & 0x7f)) {
                                            uVar5 = uVar5 + 0x80;
                                        }
                                        uVar5 = (uVar5 & 0xffffff80U) | (unsigned int)bVar2;
                                        if ((bVar1 & 0x80) == 0) {
                                            pbVar9 = pbVar4 + 1;
                                        } else {
                                            pbVar9 = pbVar4 + 2;
                                            rec.uStack_6c = (rec.uStack_6c & 0xffffU) | ((unsigned int)pbVar4[1] << 16);
                                            rec.uStack_6c = rec.uStack_6c & 0x00ffffffU;
                                        }
                                    }
                                    pbVar4 = pbVar9 + 2;
                                    DAT_00557560[iVar3] = uVar5;
                                    rec.uStack_6c = ((((unsigned int)*pbVar9) << 24) | (rec.uStack_6c & 0x00ffffffU)) & 0xfeffffffU;
                                    rec.uStack_68 = (unsigned short)(((((unsigned short)((unsigned char)(((pbVar9[1] & 1) * 2) | (*pbVar9 & 1)))) << 8) | pbVar9[1]) & 0xfffe);
                                    piVar8 = FUN_00418760(iVar3, uVar5, &rec.uStack_6c);
                                    if (piVar8 != (int *)0) {
                                        iVar6 = FUN_0046e040();
                                        *piVar8 = 1;
                                        if (iVar6 == 0) {
                                            *piVar8 = 0;
                                        }
                                        piVar8[1] = 1;
                                    }
                                    uVar5 = uStack_60 - 1;
                                } while (uStack_60 - 1 != 0);
                                uStack_60 = 0;
                            }
                        }
                        break;
                    case 0x17:
                        DAT_005242dc = piVar7[1];
                        break;
                    case 0x1d:
                        iVar3 = FUN_0046f700();
                        if (iVar3 == iStack_64) {
                            DAT_009a8c24 = 0;
                        } else {
                            iVar3 = FUN_0046f660(iStack_64);
                            if ((DAT_009e9680[iVar3 * 0x5bd] != '\0') && (0 < (int)DAT_00556ed8[iVar3])) {
                                DAT_00556ed8[iVar3] = 0;
                            }
                        }
                        break;
                    case 0x3d:
                        FUN_00463c50(DAT_009ecc34);
                        DAT_009d6940 = 1;
                        DAT_009e9680[DAT_009ecc34 * 0x5bd] = 0;
                        iVar3 = DAT_009ecc34;
                        DAT_009d8a30[DAT_009ecc34] = 1;
                        FUN_00463cc0(iVar3);
                        return;
                    case 0x3e:
                    case 0x3f:
                        if (piVar7[1] == DAT_00523420) {
                            local_54 = 1;
                        }
                    }
                }
                iVar3 = DAT_009ecc34;
            } while (local_54 == 0);
            if (*piVar7 == 0x3e) break;
            iVar3 = piVar7[2];
            iVar6 = piVar7[3];
            while (iVar6 != -1) {
                FUN_00419570(piVar7 + 5, piVar7[3], piVar7[4]);
                piVar7 = (int *)((int)piVar7 + piVar7[4] + 8);
                iVar6 = piVar7[3];
            }
            uVar5 = 8;
            DAT_00557360 = 0x10;
            DAT_00557364 = (unsigned int)iVar3;
        }
        if (DAT_009d8a30[DAT_009ecc34] != 0) {
            DAT_009d8a30[DAT_009ecc34] = 1;
            DAT_009d6940 = 1;
            DAT_009e9680[iVar3 * 0x5bd] = 0;
            FUN_00463cc0(DAT_009ecc34);
            return;
        }
        FUN_004ac760();
        FUN_004195a0(piVar7[2], piVar7[1]);
        DAT_00557360 = 0x1b;
        uVar5 = 4;
        puVar10 = (unsigned int *)&DAT_00557360;
        iVar3 = FUN_0046f700();
        FUN_0046d350(iVar3, puVar10, uVar5);
        DAT_009a8c2c = (int)((unsigned int)DAT_009ed23c + (unsigned int)DAT_00a07cc8);
        FUN_00448ed0();
    }
    return;
}

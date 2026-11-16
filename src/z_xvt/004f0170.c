/* SPDX-License-Identifier: MIT */
/* z_xvt 004f0170 */
/* Prototypes for every callee (signatures inferred from the decompile). */
extern int  _sprintf(char *, const char *, ...);
extern void FUN_004dcfd0(int *, int, int, int, int);
extern char *FUN_004dcfa0(int);
extern void FUN_004daac0(int, char *, int *, int, int, unsigned int);
extern void FUN_004dd030(int *, int, int);
extern unsigned char FUN_004db630(int);
extern unsigned char FUN_004db5f0(int);
extern unsigned char FUN_004db610(int);
extern void FUN_004ddc70(int *, int *);
extern int  FUN_004dd7d0(int *, int, int);
extern void FUN_004cb830(int, unsigned int *, int);
extern void FUN_004de190(char *, int, int, int);
extern void FUN_004db520(void);
extern int  FUN_004cde00(void);
extern void FUN_004d5180(int *);
extern void FUN_004d51b0(int *);
extern int  FUN_004cdbc0(void);
extern int  FUN_004cc810(void);
extern unsigned char FUN_004db530(void);
extern unsigned char FUN_004db550(void);
extern int  FUN_004cdbb0(void);
extern void FUN_004db510(int);

/* Globals */
extern int  DAT_00757870[];
extern int  DAT_007579b0[];
extern int  DAT_007579d0[];
extern int  DAT_007579f8;
extern int  DAT_00757a00[];
extern int  DAT_00757a20;
extern int  DAT_00757a28;
extern char DAT_0075637a;
extern char DAT_00756b0e;
extern char DAT_00757ce0;   /* record array base, stride 0x2a */
extern char DAT_00757cee;   /* packed int at +0xe of record (unaligned) */
extern char DAT_00757cf2;   /* packed int at +0x12 of record (unaligned) */
extern int  DAT_00758620;
extern int  DAT_00758624;
extern int  DAT_00758628;
extern int  DAT_0075862c;
extern char DAT_0081b740[];
extern int  DAT_0081b850;
extern char DAT_0081bc80;
extern unsigned int DAT_0081bcd0;
extern char DAT_0081bfba;   /* packed int at unaligned address */
extern char DAT_0085e7c5;
extern char DAT_005321f4;
extern char s__s__u___s_005337d0[];
extern char s_slotsound_0053309c[];
extern char s__c_s__c_s_00531334[];

void __cdecl FUN_004f0170(int param_1)
{
    unsigned char bVar1;
    unsigned char uVar2;
    int iVar3;
    int *piVar4;
    int iVar5;
    char *pcVar6;
    int *piVar7;
    int *puVar8;
    int iVar9;
    int iVar10;
    int *piVar11;
    int uVar12;
    unsigned int uVar13;
    int local_48;
    int local_44;
    char *local_40;
    int local_3c;
    int *local_38;
    int *local_34;
    int local_30;
    int local_2c;
    int iStack_24;
    int local_20[4];
    int auStack_10[4];
    int tmp;
    char *pc;

    FUN_004dcfd0(local_20, 0x5a, 0xcf, 0x100, 0xdd);
    local_3c = 0;
    if (0 < DAT_00757a28) {
        local_38 = DAT_00757870;
        local_44 = 0;
        local_34 = DAT_007579d0;
        local_40 = &DAT_0075637a;
        do {
            if (((DAT_00757a28 == 8) || (DAT_00757a28 == 7)) && (local_40 == &DAT_00756b0e)) {
                FUN_004dcfd0(local_20, 0x106, 0xcf, 0x1ac, 0xdd);
            }
            __builtin_memcpy(&tmp, &DAT_0081bfba, 4);
            if (tmp == 3) {
                _sprintf(DAT_0081b740, &DAT_005321f4, local_40);
            }
            else {
                iVar5 = local_3c + 1;
                pcVar6 = FUN_004dcfa0(0xcc);
                _sprintf(DAT_0081b740, s__s__u___s_005337d0, pcVar6, iVar5);
            }
            FUN_004daac0(0xc, DAT_0081b740, local_20, 0, 1, 0xffff);
            FUN_004dd030(local_20, 0, 0xf);
            local_48 = 0;
            if (0 < *local_34) {
                do {
                    bVar1 = FUN_004db630(2);
                    if ((bVar1 != 0) &&
                        ((uVar2 = FUN_004db5f0(2), uVar2 != 0) ||
                         (uVar2 = FUN_004db610(2), uVar2 != 0))) {
                        FUN_004ddc70(&local_2c, &local_30);
                        iVar3 = FUN_004dd7d0(local_20, local_2c, local_30);
                        iVar10 = DAT_0081b850;
                        iVar5 = DAT_007579f8;
                        if (iVar3 != 0) {
                            iVar3 = DAT_00757870[local_44 + local_48];
                            if (iVar3 == 0) {
LAB_004f037c:
                                iVar9 = 0;
                                piVar4 = local_38;
                                if (0 < local_48) {
                                    do {
                                        if (*piVar4 == 0) break;
                                        iVar9 = iVar9 + 1;
                                        piVar4 = piVar4 + 1;
                                    } while (iVar9 < local_48);
                                }
                            }
                            else {
                                piVar4 = DAT_007579b0;
                                do {
                                    if (*piVar4 == iVar3) {
                                        *piVar4 = 0;
                                    }
                                    piVar4 = piVar4 + 1;
                                } while (piVar4 < DAT_007579d0);
                                if (iVar10 != 2) {
                                    DAT_00758620 = 0x4f;
                                    DAT_00758628 = 10;
                                    DAT_00758624 = iVar3;
                                    FUN_004cb830(0, (unsigned int *)&DAT_00758620, 0xc);
                                    iVar10 = DAT_0081b850;
                                    iVar5 = DAT_007579f8;
                                }
                                iVar9 = local_48;
                                if (iVar3 == 0) goto LAB_004f037c;
                            }
                            iVar3 = 0;
                            DAT_00757870[local_44 + iVar9] = iVar5;
                            piVar4 = DAT_007579b0;
                            do {
                                if (*piVar4 == iVar5) break;
                                piVar4 = piVar4 + 1;
                                iVar3 = iVar3 + 1;
                            } while (piVar4 < DAT_007579d0);
                            if (iVar3 == 8) {
                                iVar3 = 0;
                                piVar4 = DAT_007579b0;
                                do {
                                    if (*piVar4 == 0) {
                                        DAT_007579b0[iVar3] = iVar5;
                                        break;
                                    }
                                    piVar4 = piVar4 + 1;
                                    iVar3 = iVar3 + 1;
                                } while (piVar4 < DAT_007579d0);
                            }
                            if (DAT_0085e7c5 != '\0') {
                                FUN_004de190(s_slotsound_0053309c, 1, 0, 0xff);
                                iVar10 = DAT_0081b850;
                                iVar5 = DAT_007579f8;
                            }
                            if (iVar10 != 2) {
                                DAT_00758628 = local_3c;
                                DAT_00758620 = 0x4f;
                                DAT_00758624 = iVar5;
                                DAT_0075862c = iVar9;
                                FUN_004cb830(0, (unsigned int *)&DAT_00758620, 0x10);
                                if (DAT_0081b850 != 2) {
                                    DAT_00758624 = DAT_007579f8;
                                    DAT_00758620 = 0x67;
                                    FUN_004cb830(0, (unsigned int *)&DAT_00758620, 8);
                                }
                            }
                            DAT_007579f8 = 0;
                            FUN_004db520();
                        }
                    }
                    if (DAT_00757870[local_44 + local_48] != 0) {
                        iVar5 = 1;
                        if (DAT_0081b850 != 2) {
                            iVar5 = FUN_004cde00();
                        }
                        iVar10 = 0;
                        if (0 < iVar5) {
                            pc = &DAT_00757cee;
                            do {
                                __builtin_memcpy(&tmp, pc, 4);
                                if (tmp == DAT_00757870[local_44 + local_48]) break;
                                pc = pc + 0x2a;
                                iVar10 = iVar10 + 1;
                            } while (iVar10 < iVar5);
                        }
                        if (iVar10 < iVar5) {
                            iVar5 = (int)&DAT_00757ce0 + (int)((unsigned int)iVar10 * 0x2au);
                            iVar10 = (int)((unsigned int)iVar10 * 0x2au);
                            uVar12 = 1;
                            __builtin_memcpy(&tmp, &DAT_00757cf2 + iVar10, 4);
                            pcVar6 = FUN_004dcfa0((int)((unsigned int)tmp + 0x8eu));
                            _sprintf(DAT_0081b740, s__c_s__c_s_00531334, 6, pcVar6, uVar12, iVar5);
                            FUN_004d5180(auStack_10);
                            FUN_004d51b0(local_20);
                            iVar5 = FUN_004cdbc0();
                            __builtin_memcpy(&tmp, &DAT_00757cee + iVar10, 4);
                            if ((iVar5 == tmp) ||
                                (uVar13 = DAT_0081bcd0, DAT_0081b850 == 2)) {
                                uVar13 = *(unsigned int *)(&DAT_0081bc80 +
                                         (((unsigned int)(param_1 % 0x18) & 0xfffffffeU) * 2));
                            }
                            FUN_004daac0(0xc, DAT_0081b740, local_20, 0, 1, uVar13);
                            FUN_004d51b0(auStack_10);
                            bVar1 = FUN_004db630(2);
                            if (((bVar1 == 0) &&
                                 ((iVar5 = FUN_004cc810(), iVar5 != 0) || (DAT_0081b850 == 2))) &&
                                ((uVar2 = FUN_004db530(), uVar2 != 0) ||
                                 (uVar2 = FUN_004db550(), uVar2 != 0))) {
                                FUN_004ddc70(&local_2c, &local_30);
                                iVar5 = FUN_004dd7d0(local_20, local_2c, local_30);
                                if (iVar5 != 0) {
                                    if (DAT_0081b850 == 2) {
                                        __builtin_memcpy(&DAT_007579f8, &DAT_00757cee + iVar10, 4);
                                        FUN_004db510(2);
                                        iVar5 = 0;
                                        piVar4 = DAT_007579b0;
LAB_004f07b8:
                                        if (*piVar4 != DAT_007579f8) goto code_r0x004f07be;
                                        DAT_007579b0[iVar5] = 0;
                                        iVar5 = local_48 + 1;
                                        if (iVar5 < 8) {
                                            iVar10 = 8 - iVar5;
                                            puVar8 = &DAT_00757870[local_44 + iVar5 - 1];
                                            do {
                                                iVar10 = iVar10 + -1;
                                                *puVar8 = puVar8[1];
                                                puVar8 = puVar8 + 1;
                                            } while (iVar10 != 0);
                                        }
                                        local_38[7] = 0;
                                        goto LAB_004f080d;
                                    }
                                    iVar5 = 0;
                                    if (0 < DAT_00757a20) {
                                        piVar4 = DAT_00757a00;
                                        do {
                                            __builtin_memcpy(&tmp, &DAT_00757cee + iVar10, 4);
                                            if (*piVar4 == tmp) break;
                                            piVar4 = piVar4 + 1;
                                            iVar5 = iVar5 + 1;
                                        } while (iVar5 < DAT_00757a20);
                                    }
                                    if (iVar5 == DAT_00757a20) {
                                        __builtin_memcpy(&DAT_00758624, &DAT_00757cee + iVar10, 4);
                                        DAT_00757a20 = DAT_00757a20 + 1;
                                        DAT_00757a00[iVar5] = DAT_00758624;
                                        DAT_00758620 = 0x68;
                                        DAT_00758628 = FUN_004cdbb0();
                                        FUN_004cb830(0, (unsigned int *)&DAT_00758620, 0xc);
                                        __builtin_memcpy(&DAT_007579f8, &DAT_00757cee + iVar10, 4);
                                        FUN_004db510(2);
                                        DAT_00758620 = 0x4f;
                                        DAT_00758624 = DAT_007579f8;
                                        DAT_00758628 = 10;
                                        FUN_004cb830(0, (unsigned int *)&DAT_00758620, 0xc);
                                        iVar10 = DAT_00757a28;
                                        iVar5 = DAT_007579f8;
                                        iVar3 = 0;
                                        piVar4 = DAT_007579b0;
                                        do {
                                            if (*piVar4 == DAT_007579f8) {
                                                DAT_007579b0[iVar3] = 0;
                                                break;
                                            }
                                            piVar4 = piVar4 + 1;
                                            iVar3 = iVar3 + 1;
                                        } while (piVar4 < DAT_007579d0);
                                        if (0 < iVar10) {
                                            piVar4 = DAT_00757870;
                                            iVar3 = 0;
                                            piVar11 = DAT_007579d0;
                                            iStack_24 = iVar10;
                                            do {
                                                iVar10 = 0;
                                                piVar7 = piVar4;
                                                if (0 < *piVar11) {
LAB_004f073e:
                                                    if (*piVar7 != iVar5) goto code_r0x004f0744;
                                                    iVar10 = iVar10 + 1;
                                                    if (iVar10 < 8) {
                                                        iVar9 = 8 - iVar10;
                                                        puVar8 = &DAT_00757870[iVar3 + iVar10 - 1];
                                                        do {
                                                            iVar9 = iVar9 + -1;
                                                            *puVar8 = puVar8[1];
                                                            puVar8 = puVar8 + 1;
                                                        } while (iVar9 != 0);
                                                    }
                                                    piVar4[7] = 0;
                                                }
LAB_004f0781:
                                                piVar4 = piVar4 + 8;
                                                iVar3 = iVar3 + 8;
                                                piVar11 = piVar11 + 1;
                                                iStack_24 = iStack_24 + -1;
                                            } while (iStack_24 != 0);
                                            iStack_24 = 0;
                                        }
                                    }
                                }
                            }
                        }
                    }
LAB_004f080d:
                    FUN_004dd030(local_20, 0, 0xf);
                    local_48 = local_48 + 1;
                } while (local_48 < *local_34);
            }
            local_38 = local_38 + 8;
            local_44 = local_44 + 8;
            local_34 = local_34 + 1;
            local_40 = local_40 + 0x1e5;
            local_3c = local_3c + 1;
        } while (local_3c < DAT_00757a28);
    }
    return;
code_r0x004f07be:
    piVar4 = piVar4 + 1;
    iVar5 = iVar5 + 1;
    if ((int *)0x7579cf < piVar4) goto LAB_004f080d;
    goto LAB_004f07b8;
code_r0x004f0744:
    iVar10 = iVar10 + 1;
    piVar7 = piVar7 + 1;
    if (*piVar11 <= iVar10) goto LAB_004f0781;
    goto LAB_004f073e;
}

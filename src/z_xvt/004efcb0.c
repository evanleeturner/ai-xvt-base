/* SPDX-License-Identifier: MIT */
/* z_xvt 004efcb0 */
extern int DAT_0081b850;
extern int DAT_00757a20;
extern int DAT_00757a28;
extern int DAT_007579b0[8];
extern int DAT_007579d0[];
extern int DAT_007579f8;
extern int DAT_00757a00[];
extern int DAT_0075786c[];
extern int DAT_00757870[];
extern int DAT_0075788c;
extern char DAT_00757cee[];   /* address not 4-aligned: accessed via memcpy */
extern char DAT_0081b740[];
extern char DAT_0081bc80[];
extern unsigned int DAT_0081bcd0;
extern unsigned int _DAT_0081b6fc;
extern unsigned int _DAT_00758620;
extern int DAT_00758624;
extern int DAT_00758628;

void FUN_004dcfd0(int *, int, int, int, int);
char *FUN_004dcfa0(int);
void FUN_004daac0(int, char *, int *, int, int, unsigned int);
int FUN_004cde00(void);
void FUN_004d5180(void *);
void FUN_004d51b0(void *);
int _sprintf(char *, const char *, ...);
int FUN_004cdbc0(void);
int FUN_004cc810(void);
unsigned char FUN_004db630(int);
void FUN_004ddc70(int *, int *);
unsigned char FUN_004db530(void);
unsigned char FUN_004db550(void);
int FUN_004dd7d0(int *, int, int);
void FUN_004db510(int);
int FUN_004cdbb0(void);
void FUN_004cb830(int, unsigned int *, int);
void FUN_004dd030(int *, int, int);

unsigned int __cdecl FUN_004efcb0(int param_1)
{
    int iVar1;
    unsigned char bVar2;
    unsigned char uVar3;
    char *pcVar4;
    int *piVar5;
    int iVar6;
    int iVar7;
    int iVar8;
    int *puVar9;
    int *puVar10;
    int iVar11;
    int iVar12;
    int uVar13;
    unsigned int uVar14;
    char *local_40;
    int *local_3c;
    int local_38;
    int local_30;
    int local_2c;
    int local_28[4];
    int local_18;
    int local_14;
    unsigned int local_10[4];
    int t0;
    int t1;

    FUN_004dcfd0(local_28, 0x58, 0x72, 0x1ae, 0x80);
    piVar5 = local_28;
    uVar14 = 0xffff;
    iVar12 = 1;
    iVar11 = 0;
    pcVar4 = FUN_004dcfa0(0x1cc);
    local_2c = 1;
    FUN_004daac0(0xf, pcVar4, piVar5, iVar11, iVar12, uVar14);
    if (DAT_0081b850 != 2) {
        local_2c = FUN_004cde00();
    }
    local_38 = 0;
    FUN_004dcfd0(local_28, 0x58, 0x86, 0x102, 0x94);
    if (0 < local_2c) {
        local_40 = DAT_00757cee;
        iVar11 = DAT_00757a20;
        do {
            iVar7 = 0;
            piVar5 = DAT_007579b0;
            __builtin_memcpy(&iVar12, local_40, 4);
            do {
                if (*piVar5 == iVar12) break;
                piVar5 = piVar5 + 1;
                iVar7 = iVar7 + 1;
            } while (piVar5 < (int *)DAT_007579d0);
            if ((iVar7 == 8) && (DAT_007579f8 != iVar12)) {
                iVar7 = 0;
                if (0 < iVar11) {
                    piVar5 = DAT_00757a00;
                    do {
                        if (*piVar5 == iVar12) break;
                        piVar5 = piVar5 + 1;
                        iVar7 = iVar7 + 1;
                    } while (iVar7 < iVar11);
                }
                FUN_004d5180(local_10);
                FUN_004d51b0(local_28);
                if (iVar7 == DAT_00757a20) {
                    iVar11 = (int)(local_40 - 0xe);
                    uVar13 = 1;
                    __builtin_memcpy(&t1, local_40 + 4, 4);
                    pcVar4 = FUN_004dcfa0((int)((unsigned int)t1 + 0x8eu));
                    _sprintf(DAT_0081b740, "%c%s %c%s", 6, pcVar4, uVar13, iVar11);
                    iVar11 = FUN_004cdbc0();
                    __builtin_memcpy(&t0, local_40, 4);
                    if ((iVar11 == t0) || (uVar14 = DAT_0081bcd0, DAT_0081b850 == 2)) {
                        uVar14 = *(unsigned int *)(DAT_0081bc80 +
                                 (((unsigned int)(param_1 % 0x18) & 0xfffffffeU) * 2));
                    }
                } else {
                    iVar11 = (int)(local_40 - 0xe);
                    __builtin_memcpy(&t1, local_40 + 4, 4);
                    pcVar4 = FUN_004dcfa0((int)((unsigned int)t1 + 0x8eu));
                    _sprintf(DAT_0081b740, "%s %s", pcVar4, iVar11);
                    uVar14 = _DAT_0081b6fc;
                }
                FUN_004daac0(0xc, DAT_0081b740, local_28, 0, 1, uVar14);
                local_38 = local_38 + 1;
                FUN_004d51b0(local_10);
                iVar11 = FUN_004cc810();
                if (((iVar11 != 0) || (DAT_0081b850 == 2)) &&
                    ((bVar2 = FUN_004db630(2)) == 0)) {
                    FUN_004ddc70(&local_14, &local_18);
                    uVar3 = FUN_004db530();
                    if (((uVar3 != 0) ||
                         ((uVar3 = FUN_004db550()) != 0)) &&
                        (iVar11 = FUN_004dd7d0(local_28, local_14, local_18), iVar11 != 0)) {
                        if (DAT_0081b850 == 2) {
                            __builtin_memcpy(&DAT_007579f8, local_40, 4);
                            FUN_004db510(2);
                        } else {
                            iVar11 = 0;
                            if (0 < DAT_00757a20) {
                                piVar5 = DAT_00757a00;
                                __builtin_memcpy(&t0, local_40, 4);
                                do {
                                    if (*piVar5 == t0) break;
                                    piVar5 = piVar5 + 1;
                                    iVar11 = iVar11 + 1;
                                } while (iVar11 < DAT_00757a20);
                            }
                            if (iVar11 == DAT_00757a20) {
                                DAT_00757a20 = (int)((unsigned int)DAT_00757a20 + 1u);
                                _DAT_00758620 = 0x68;
                                __builtin_memcpy(&DAT_00758624, local_40, 4);
                                DAT_00757a00[iVar11] = DAT_00758624;
                                DAT_00758628 = FUN_004cdbb0();
                                FUN_004cb830(0, &_DAT_00758620, 0xc);
                                __builtin_memcpy(&DAT_007579f8, local_40, 4);
                                FUN_004db510(2);
                                _DAT_00758620 = 0x4f;
                                DAT_00758624 = DAT_007579f8;
                                DAT_00758628 = 10;
                                FUN_004cb830(0, &_DAT_00758620, 0xc);
                                local_30 = DAT_00757a28;
                                iVar11 = DAT_007579f8;
                                piVar5 = DAT_007579b0;
                                do {
                                    if (*piVar5 == iVar11) {
                                        *piVar5 = 0;
                                    }
                                    piVar5 = piVar5 + 1;
                                } while (piVar5 < (int *)DAT_007579d0);
                                if (0 < local_30) {
                                    puVar10 = &DAT_0075788c;
                                    iVar12 = 0;
                                    local_3c = (int *)DAT_007579d0;
                                    do {
                                        iVar8 = 0;
                                        iVar7 = *local_3c;
                                        if (0 < iVar7) {
                                            do {
                                                if (DAT_00757870[iVar12 + iVar8] == iVar11) {
                                                    iVar1 = iVar8 + 1;
                                                    if (iVar1 < 8) {
                                                        iVar6 = 8 - iVar1;
                                                        puVar9 = DAT_0075786c + (iVar12 + iVar1);
                                                        do {
                                                            iVar6 = iVar6 + -1;
                                                            *puVar9 = puVar9[1];
                                                            puVar9 = puVar9 + 1;
                                                        } while (iVar6 != 0);
                                                    }
                                                    *puVar10 = 0;
                                                }
                                                iVar8 = iVar8 + 1;
                                            } while (iVar8 < iVar7);
                                        }
                                        puVar10 = puVar10 + 8;
                                        iVar12 = iVar12 + 8;
                                        local_3c = local_3c + 1;
                                        local_30 = local_30 + -1;
                                    } while (local_30 != 0);
                                }
                            }
                        }
                    }
                }
                if (local_38 == 4) {
                    FUN_004dcfd0(local_28, 0x103, 0x86, 0x1ae, 0x94);
                    iVar11 = DAT_00757a20;
                } else {
                    FUN_004dd030(local_28, 0, 0xf);
                    iVar11 = DAT_00757a20;
                }
            }
            local_40 = local_40 + 0x2a;
            local_2c = local_2c + -1;
        } while (local_2c != 0);
    }
    return 1;
}

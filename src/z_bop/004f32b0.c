/* SPDX-License-Identifier: MIT */
/* z_bop 004f32b0 */
extern int DAT_00aa5df0[8];
extern int DAT_00aa5e38;
extern int DAT_00aa5e40[];
extern int DAT_00aa5e60;
extern int DAT_00aa5e68;
extern int DAT_00aa5e10[];
extern int DAT_00aa5cb0[];
extern unsigned char DAT_00aa615e[];
extern int DAT_00aa6af0;
extern int DAT_00aa6af4;
extern int DAT_00aa6af8;
extern char DAT_00b69d20[];
extern int DAT_00b69e30;
extern unsigned int DAT_00b6a2c0;
extern unsigned int _DAT_00b69ccc;
extern char DAT_00b6a270[];

extern int _sprintf(char *, const char *, ...);
void FUN_004d6460(int *, int, int, int, int);
char *FUN_004ddc10(int);
void FUN_004db7d0(int, char *, int *, int, int, unsigned int);
int FUN_004d1400(void);
void FUN_004d4cf0(void *);
void FUN_004d4d20(void *);
int FUN_004d11e0(void);
int FUN_004cfee0(void);
char FUN_004dc2e0(int);
void FUN_004de090(int *, int *);
char FUN_004dc1e0(void);
char FUN_004dc200(void);
int FUN_004d6b90(int *, int, int);
void FUN_004dc1c0(int);
int FUN_004d11d0(void);
void FUN_004cef70(int, unsigned int *, int);
void FUN_004d64a0(int *, int, int);

int FUN_004f32b0(int param_1)
{
    int iVar9;
    int iVar11;
    int iVar12;
    int iVar6;
    int iVar7;
    int iVar1;
    int *piVar5;
    unsigned int *puVar8;
    int *puVar10;
    char *pcVar4;
    unsigned int uVar13;
    unsigned int uVar14;
    unsigned char *local_40;
    int *local_38;
    int local_34;
    int local_30[4];
    int local_20;
    int local_1c;
    int local_18;
    int local_14;
    unsigned int local_10[4];
    int tmp;

    FUN_004d6460(local_30, 0x58, 0x72, 0x1ae, 0x80);
    piVar5 = local_30;
    uVar14 = 0xffff;
    iVar12 = 1;
    iVar11 = 0;
    pcVar4 = FUN_004ddc10(0x1da);
    iVar9 = 1;
    FUN_004db7d0(0xf, pcVar4, piVar5, iVar11, iVar12, uVar14);
    if (DAT_00b69e30 != 2) {
        iVar9 = FUN_004d1400();
    }
    local_34 = 0;
    FUN_004d6460(local_30, 0x58, 0x86, 0x102, 0x94);
    if (0 < iVar9) {
        local_40 = DAT_00aa615e;
        iVar11 = DAT_00aa5e60;
        local_14 = iVar9;
        do {
            iVar12 = 0;
            piVar5 = DAT_00aa5df0;
            __builtin_memcpy(&tmp, local_40, 4);
            iVar9 = tmp;
            do {
                if (*piVar5 == iVar9) break;
                piVar5 = piVar5 + 1;
                iVar12 = (int)((unsigned int)iVar12 + 1U);
            } while (piVar5 < DAT_00aa5df0 + 8);
            if ((iVar12 == 8) && (DAT_00aa5e38 != iVar9)) {
                iVar12 = 0;
                if (0 < iVar11) {
                    piVar5 = DAT_00aa5e40;
                    do {
                        if (*piVar5 == iVar9) break;
                        piVar5 = piVar5 + 1;
                        iVar12 = (int)((unsigned int)iVar12 + 1U);
                    } while (iVar12 < iVar11);
                }
                FUN_004d4cf0(local_10);
                FUN_004d4d20(local_30);
                if (iVar12 == DAT_00aa5e60) {
                    iVar9 = (int)((unsigned int)(int)local_40 - 0xeU);
                    uVar13 = 1;
                    __builtin_memcpy(&tmp, local_40 + 4, 4);
                    pcVar4 = FUN_004ddc10((int)((unsigned int)tmp + 0x9aU));
                    _sprintf((char *)DAT_00b69d20, "%c%s %c%s", 6, pcVar4, uVar13, iVar9);
                    iVar9 = FUN_004d11e0();
                    __builtin_memcpy(&tmp, local_40, 4);
                    if ((iVar9 == tmp) || (uVar14 = DAT_00b6a2c0, DAT_00b69e30 == 2)) {
                        uVar14 = *(unsigned int *)(DAT_00b6a270 + ((unsigned int)(param_1 % 0x18) & 0xfffffffeU) * 2);
                    }
                } else {
                    iVar9 = (int)((unsigned int)(int)local_40 - 0xeU);
                    __builtin_memcpy(&tmp, local_40 + 4, 4);
                    pcVar4 = FUN_004ddc10((int)((unsigned int)tmp + 0x9aU));
                    _sprintf((char *)DAT_00b69d20, "%s %s", pcVar4, iVar9);
                    uVar14 = _DAT_00b69ccc;
                }
                FUN_004db7d0(0xc, (char *)DAT_00b69d20, local_30, 0, 1, uVar14);
                local_34 = (int)((unsigned int)local_34 + 1U);
                FUN_004d4d20(local_10);
                iVar9 = FUN_004cfee0();
                if (((iVar9 != 0) || (DAT_00b69e30 == 2)) &&
                    (FUN_004dc2e0(2) == 0)) {
                    FUN_004de090(&local_1c, &local_20);
                    if (((FUN_004dc1e0() != 0) ||
                         (FUN_004dc200() != 0)) &&
                        (iVar9 = FUN_004d6b90(local_30, local_1c, local_20), iVar9 != 0)) {
                        if (DAT_00b69e30 == 2) {
                            __builtin_memcpy(&DAT_00aa5e38, local_40, 4);
                            FUN_004dc1c0(2);
                        } else {
                            iVar9 = 0;
                            if (0 < DAT_00aa5e60) {
                                piVar5 = DAT_00aa5e40;
                                do {
                                    __builtin_memcpy(&tmp, local_40, 4);
                                    if (*piVar5 == tmp) break;
                                    piVar5 = piVar5 + 1;
                                    iVar9 = (int)((unsigned int)iVar9 + 1U);
                                } while (iVar9 < DAT_00aa5e60);
                            }
                            if (iVar9 == DAT_00aa5e60) {
                                DAT_00aa5e60 = (int)((unsigned int)DAT_00aa5e60 + 1U);
                                DAT_00aa6af0 = 0x68;
                                __builtin_memcpy(&DAT_00aa6af4, local_40, 4);
                                DAT_00aa5e40[iVar9] = DAT_00aa6af4;
                                DAT_00aa6af8 = FUN_004d11d0();
                                FUN_004cef70(0, (unsigned int *)&DAT_00aa6af0, 0xc);
                                __builtin_memcpy(&DAT_00aa5e38, local_40, 4);
                                FUN_004dc1c0(2);
                                DAT_00aa6af0 = 0x4f;
                                DAT_00aa6af4 = DAT_00aa5e38;
                                DAT_00aa6af8 = 10;
                                FUN_004cef70(0, (unsigned int *)&DAT_00aa6af0, 0xc);
                                iVar11 = DAT_00aa5e68;
                                iVar9 = DAT_00aa5e38;
                                piVar5 = DAT_00aa5df0;
                                do {
                                    if (*piVar5 == iVar9) {
                                        *piVar5 = 0;
                                    }
                                    piVar5 = piVar5 + 1;
                                } while (piVar5 < DAT_00aa5df0 + 8);
                                if (0 < iVar11) {
                                    puVar10 = DAT_00aa5cb0 + 7;
                                    iVar12 = 0;
                                    local_38 = DAT_00aa5e10;
                                    local_18 = iVar11;
                                    do {
                                        iVar6 = 0;
                                        iVar11 = *local_38;
                                        if (0 < iVar11) {
                                            do {
                                                if (DAT_00aa5cb0[(int)((unsigned int)iVar12 + (unsigned int)iVar6)] == iVar9) {
                                                    iVar1 = (int)((unsigned int)iVar6 + 1U);
                                                    if (iVar1 < 8) {
                                                        iVar7 = (int)(8U - (unsigned int)iVar1);
                                                        puVar8 = (unsigned int *)&DAT_00aa5cb0[(int)((unsigned int)iVar12 + (unsigned int)iVar1 - 1U)];
                                                        do {
                                                            iVar7 = (int)((unsigned int)iVar7 - 1U);
                                                            *puVar8 = puVar8[1];
                                                            puVar8 = puVar8 + 1;
                                                        } while (iVar7 != 0);
                                                    }
                                                    *puVar10 = 0;
                                                }
                                                iVar6 = (int)((unsigned int)iVar6 + 1U);
                                            } while (iVar6 < iVar11);
                                        }
                                        puVar10 = puVar10 + 8;
                                        iVar12 = (int)((unsigned int)iVar12 + 8U);
                                        local_38 = local_38 + 1;
                                        local_18 = (int)((unsigned int)local_18 - 1U);
                                    } while (local_18 != 0);
                                }
                            }
                        }
                    }
                }
                if (local_34 == 4) {
                    FUN_004d6460(local_30, 0x103, 0x86, 0x1ae, 0x94);
                    iVar11 = DAT_00aa5e60;
                } else {
                    FUN_004d64a0(local_30, 0, 0xf);
                    iVar11 = DAT_00aa5e60;
                }
            }
            local_40 = local_40 + 0x2a;
            local_14 = (int)((unsigned int)local_14 - 1U);
        } while (local_14 != 0);
    }
    return 1;
}

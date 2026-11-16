/* SPDX-License-Identifier: MIT */
/* z_xvt 004e7ec0 */
/* Prototypes */
extern void FUN_004ddc70(int *a, int *b);
extern void FUN_004dcfd0(int *rect, int l, int t, int r, int b);
extern int FUN_004dd7d0(int *rect, int x, int y);
extern unsigned char FUN_004db530(void);
extern unsigned char FUN_004db550(void);
extern unsigned char FUN_004db570(void);
extern unsigned char FUN_004db590(void);
extern void FUN_004dd030(int *rect, int dx, int dy);
extern void FUN_004d9ea0(int *p);
extern char *FUN_004dcfa0(int id);
extern int FUN_004d9a80(int *rect, char *up, char *down, char *text,
                        int a, int b, int c, char *snd);
extern void FUN_004539c0(void);
extern void FUN_004a7ac0(unsigned int *p);
extern void FUN_004526c0(char *p);
extern void FUN_00453aa0(int a, int b, int c);

/* Globals */
extern char s_jewelsound_00531990[];
extern char s_review1u_0053335c[];
extern char s_review1d_00533368[];
extern char s_review2u_00533374[];
extern char s_review2d_00533380[];
extern char s_review3u_0053338c[];
extern char s_review3d_00533398[];
extern char s_review6u_0053332c[];
extern char s_review6d_00533338[];
extern char s_review7u_00533344[];
extern char s_review7d_00533350[];

extern int DAT_0064ff48;
extern int DAT_0064ff4c;
extern unsigned int DAT_0064ff58[11];
extern float DAT_0064ff84;
extern int DAT_0064ff88;
extern int DAT_0064ff8c;
extern float DAT_0064ff94;
extern char *DAT_00757ca0;
extern int DAT_00757ca8;
extern float _DAT_005202ec;
extern double _DAT_005202f0;
extern double _DAT_00520300;

unsigned int __stdcall FUN_004e7ec0(void)
{
    int local_34;
    int local_30;
    int local_2c[4];
    int local_1c[7];
    int *piVar4;
    int iVar2;
    int iVar7;
    int uVar6;
    char *pcVar3;
    char *pcVar8;
    int i;

    local_1c[0] = 1;
    local_1c[1] = 1;
    local_1c[2] = 1;
    local_1c[3] = 0;
    local_1c[4] = 0;
    local_1c[5] = 1;
    local_1c[6] = 1;
    FUN_004ddc70(&local_30, &local_34);
    FUN_004dcfd0(local_2c, 0x16, 0x72, 0x2a, 0x8a);
    FUN_004ddc70(&local_30, &local_34);
    piVar4 = local_1c;
    do {
        if (*piVar4 != 0) {
            iVar2 = FUN_004dd7d0(local_2c, local_30, local_34);
            if (iVar2 != 0) {
                if (FUN_004db530() == 0) {
                    if (FUN_004db550() == 0) {
                        if (FUN_004db570() == 0) {
                            if (FUN_004db590() == 0) goto LAB_004e7f74;
                        }
                    }
                }
                *piVar4 = 2;
            }
        }
LAB_004e7f74:
        piVar4 = piVar4 + 1;
        FUN_004dd030(local_2c, 0, 0x1c);
    } while (piVar4 < local_1c + 3);

    FUN_004dcfd0(local_2c, 0x16, 0x132, 0x2a, 0x14a);
    piVar4 = local_1c + 5;
    do {
        if (*piVar4 != 0) {
            iVar2 = FUN_004dd7d0(local_2c, local_30, local_34);
            if (iVar2 != 0) {
                if (FUN_004db530() == 0) {
                    if (FUN_004db550() == 0) {
                        if (FUN_004db570() == 0) {
                            if (FUN_004db590() == 0) goto LAB_004e7ffa;
                        }
                    }
                }
                *piVar4 = 2;
            }
        }
LAB_004e7ffa:
        piVar4 = piVar4 + 1;
        FUN_004dd030(local_2c, 0, 0x1c);
        if (local_1c + 7 <= piVar4) {
            break;
        }
    } while (1);

    FUN_004d9ea0(local_1c);
    FUN_004dcfd0(local_2c, 0x16, 0xaa, 0x2a, 0xc2);
    pcVar8 = s_jewelsound_00531990;
    iVar7 = 0xd;
    uVar6 = 0;
    iVar2 = 0xc;
    pcVar3 = FUN_004dcfa0(0x119);
    iVar2 = FUN_004d9a80(local_2c, s_review3u_0053338c, s_review3d_00533398,
                         pcVar3, iVar2, uVar6, iVar7, pcVar8);
    if (iVar2 != 0) {
        DAT_0064ff4c = (int)((unsigned int)DAT_0064ff4c - 1u);
        if (DAT_0064ff4c == -2) {
            DAT_0064ff88 = (int)((unsigned int)DAT_0064ff88 - 1u);
            DAT_0064ff4c = 1;
            if (DAT_0064ff88 == -2) {
                DAT_0064ff88 = 1;
                DAT_0064ff8c = (int)((unsigned int)DAT_0064ff8c - 1u);
                if (DAT_0064ff8c == -2) {
                    DAT_0064ff8c = 1;
                }
            }
        }
        FUN_004539c0();
    }

    FUN_004dd030(local_2c, 0, -0x1c);
    pcVar8 = s_jewelsound_00531990;
    iVar7 = 0xc;
    uVar6 = 0;
    iVar2 = 0xc;
    pcVar3 = FUN_004dcfa0(0x117);
    FUN_004d9a80(local_2c, s_review2u_00533374, s_review2d_00533380,
                 pcVar3, iVar2, uVar6, iVar7, pcVar8);
    iVar2 = FUN_004dd7d0(local_2c, local_30, local_34);
    if (iVar2 != 0) {
        if (FUN_004db530() == 0) {
            if (FUN_004db550() != 0 &&
                (DAT_0064ff84 = DAT_0064ff84 + _DAT_005202ec,
                 (float)_DAT_00520300 <= DAT_0064ff84)) {
                DAT_0064ff84 = 0.0f;
            }
        } else {
            DAT_0064ff84 = DAT_0064ff84 - _DAT_005202ec;
            if (DAT_0064ff84 <= (float)_DAT_005202f0) {
                DAT_0064ff84 = 360.0f;
            }
        }
    }

    FUN_004dd030(local_2c, 0, -0x1c);
    pcVar8 = s_jewelsound_00531990;
    iVar7 = 0xb;
    uVar6 = 0;
    iVar2 = 0xc;
    pcVar3 = FUN_004dcfa0(0x118);
    FUN_004d9a80(local_2c, s_review1u_0053335c, s_review1d_00533368,
                 pcVar3, iVar2, uVar6, iVar7, pcVar8);
    iVar2 = FUN_004dd7d0(local_2c, local_30, local_34);
    if (iVar2 != 0) {
        if (FUN_004db530() == 0) {
            if (FUN_004db550() != 0 &&
                (DAT_0064ff94 = DAT_0064ff94 + _DAT_005202ec,
                 (float)_DAT_00520300 <= DAT_0064ff94)) {
                DAT_0064ff94 = 0.0f;
            }
        } else {
            DAT_0064ff94 = DAT_0064ff94 - _DAT_005202ec;
            if (DAT_0064ff94 <= (float)_DAT_005202f0) {
                DAT_0064ff94 = 360.0f;
            }
        }
    }

    FUN_004dcfd0(local_2c, 0x16, 0x14e, 0x2a, 0x166);
    pcVar8 = s_jewelsound_00531990;
    iVar7 = 0x11;
    uVar6 = 0;
    iVar2 = 0xc;
    pcVar3 = FUN_004dcfa0(0x11b);
    iVar2 = FUN_004d9a80(local_2c, s_review7u_00533344, s_review7d_00533350,
                         pcVar3, iVar2, uVar6, iVar7, pcVar8);
    if (iVar2 != 0) {
        DAT_0064ff48 = (int)((unsigned int)DAT_0064ff48 - 1u);
        if (DAT_0064ff48 < 0) {
            DAT_0064ff48 = (int)((unsigned int)DAT_00757ca8 - 1u);
        }
        for (i = 0; i < 11; i++) {
            DAT_0064ff58[i] = 0;
        }
        DAT_0064ff58[0] = *(unsigned int *)(DAT_00757ca0 + 0x40 + (unsigned int)DAT_0064ff48 * 0x44u);
        FUN_004a7ac0(DAT_0064ff58);
        FUN_004526c0(DAT_00757ca0 + (unsigned int)DAT_0064ff48 * 0x44u);
        iVar2 = *(int *)(DAT_00757ca0 + 0x40 + (unsigned int)DAT_0064ff48 * 0x44u);
        if (iVar2 == 6 || iVar2 == 7) {
            uVar6 = 100;
        } else {
            uVar6 = 0;
        }
        FUN_00453aa0(0, uVar6, 0);
    }

    FUN_004dd030(local_2c, 0, -0x1c);
    pcVar8 = s_jewelsound_00531990;
    iVar7 = 0x10;
    uVar6 = 0;
    iVar2 = 0xc;
    pcVar3 = FUN_004dcfa0(0x11a);
    iVar2 = FUN_004d9a80(local_2c, s_review6u_0053332c, s_review6d_00533338,
                         pcVar3, iVar2, uVar6, iVar7, pcVar8);
    if (iVar2 != 0) {
        DAT_0064ff48 = (int)((unsigned int)DAT_0064ff48 + 1u);
        if (DAT_00757ca8 <= DAT_0064ff48) {
            DAT_0064ff48 = 0;
        }
        for (i = 0; i < 11; i++) {
            DAT_0064ff58[i] = 0;
        }
        DAT_0064ff58[0] = *(unsigned int *)(DAT_00757ca0 + 0x40 + (unsigned int)DAT_0064ff48 * 0x44u);
        FUN_004a7ac0(DAT_0064ff58);
        FUN_004526c0(DAT_00757ca0 + (unsigned int)DAT_0064ff48 * 0x44u);
        iVar2 = *(int *)(DAT_00757ca0 + 0x40 + (unsigned int)DAT_0064ff48 * 0x44u);
        if (iVar2 != 6 && iVar2 != 7) {
            FUN_00453aa0(0, 0, 0);
            return 1;
        }
        FUN_00453aa0(0, 100, 0);
    }
    return 1;
}

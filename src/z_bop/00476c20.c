/* SPDX-License-Identifier: MIT */
/* z_bop 00476c20 */
typedef unsigned char byte_t;

extern int DAT_00999488;
extern unsigned char DAT_0099947e[4]; /* unaligned 4-byte global at 0x0099947e */
extern int DAT_0060f204;
extern int DAT_0060f1c0;
extern int DAT_0060f1e8;
extern int _DAT_0060f1f0;
extern int *DAT_0060f1d8;
extern int *_DAT_0060f208;
extern int DAT_009e8f54;
extern int DAT_005233d8;
extern int DAT_005233c4;
extern int DAT_005233ac;
extern unsigned int DAT_009994bc;

int FUN_004452d0(byte_t *a, int b, int c, int d);
unsigned int FUN_00476b00(int a, int b, int c, int d, int e, int f);
void FUN_004079e0(void);
void FUN_00476b90(void);
int FUN_00476c20(int *param_1, unsigned int *param_2, byte_t *param_3);

int FUN_00476c20(int *param_1, unsigned int *param_2, byte_t *param_3)
{
    char cVar1;
    unsigned short uVar2;
    unsigned short uVar3;
    unsigned short uVar4;
    unsigned short uVar5;
    int iVar6;
    int iVar7;
    int iVar8;
    unsigned int uVar9;
    unsigned int uVar10;
    int iVar11;
    int *piVar12;
    unsigned int *puVar13;
    byte_t *pbVar14;
    byte_t *pbVar15;
    unsigned int *puVar16;
    char *pcVar17;
    char *pcVar18;
    int iStack_108;
    byte_t *pbStack_104;
    unsigned int uStack_100;
    byte_t *pbStack_f0;
    int iStack_ec;
    byte_t *pbStack_e4;
    byte_t *pbStack_e0;
    byte_t *pbStack_dc;
    int iStack_d4;
    unsigned int auStack_d0[52];
    unsigned int tmp32;

    if (param_1 == (int *)0x0) {
        return 0;
    }
    if (param_3 != (byte_t *)0x0) {
        piVar12 = param_1;
        pbVar14 = param_3;
        for (iVar8 = 6; iVar8 != 0; iVar8 = iVar8 + -1) {
            *(int *)pbVar14 = *piVar12;
            piVar12 = piVar12 + 1;
            pbVar14 = pbVar14 + 4;
        }
        pbStack_104 = param_3;
        param_3 = param_3 + 0x18;
    }
    iStack_108 = 0x18;
    if (*param_1 != 0) {
        if (param_3 != (byte_t *)0x0) {
            *(byte_t **)pbStack_104 = param_3;
            uVar9 = 0xffffffff;
            pcVar18 = (char *)*param_1;
            do {
                pcVar17 = pcVar18;
                if (uVar9 == 0) break;
                uVar9 = uVar9 - 1;
                pcVar17 = pcVar18 + 1;
                cVar1 = *pcVar18;
                pcVar18 = pcVar17;
            } while (cVar1 != '\0');
            uVar9 = ~uVar9;
            pbVar14 = (byte_t *)(pcVar17 - uVar9);
            pbVar15 = param_3;
            for (uVar10 = uVar9 >> 2; uVar10 != 0; uVar10 = uVar10 - 1) {
                *(unsigned int *)pbVar15 = *(unsigned int *)pbVar14;
                pbVar14 = pbVar14 + 4;
                pbVar15 = pbVar15 + 4;
            }
            for (uVar9 = uVar9 & 3; uVar9 != 0; uVar9 = uVar9 - 1) {
                *pbVar15 = *pbVar14;
                pbVar14 = pbVar14 + 1;
                pbVar15 = pbVar15 + 1;
            }
            uVar9 = 0xffffffff;
            pcVar18 = (char *)*param_1;
            do {
                if (uVar9 == 0) break;
                uVar9 = uVar9 - 1;
                cVar1 = *pcVar18;
                pcVar18 = pcVar18 + 1;
            } while (cVar1 != '\0');
            param_3 = param_3 + ~uVar9;
        }
        uVar9 = 0xffffffff;
        pcVar18 = (char *)*param_1;
        do {
            if (uVar9 == 0) break;
            uVar9 = uVar9 - 1;
            cVar1 = *pcVar18;
            pcVar18 = pcVar18 + 1;
        } while (cVar1 != '\0');
        iStack_108 = (int)(~uVar9 + 0x18);
    }
    if (param_1[2] != 0) {
        if (param_3 != (byte_t *)0x0) {
            *(byte_t **)(pbStack_104 + 0xc) = param_3;
            param_3 = param_3 + (int)((unsigned int)param_1[2] * 4u);
        }
        iStack_108 = (int)((unsigned int)iStack_108 + (unsigned int)param_1[2] * 4u);
    }
    piVar12 = (int *)param_1[5];
    switch (param_1[1]) {
    case 1:
    case 0xf:
    case 0x10:
    case 0x11:
        if (DAT_00999488 < *piVar12) {
            DAT_00999488 = *piVar12;
        }
        uVar9 = (unsigned int)param_1[4] * 100u + 4u;
        if (param_2[0x21] == 0) {
            uVar9 = uVar9 + (unsigned int)DAT_0060f204 * 0xcu;
        }
        if (param_3 != (byte_t *)0x0) {
            *(byte_t **)(pbStack_104 + 0x14) = param_3;
            pbVar14 = (byte_t *)param_1[5];
            pbVar15 = param_3;
            for (uVar10 = uVar9 >> 2; uVar10 != 0; uVar10 = uVar10 - 1) {
                *(unsigned int *)pbVar15 = *(unsigned int *)pbVar14;
                pbVar14 = pbVar14 + 4;
                pbVar15 = pbVar15 + 4;
            }
            for (iVar8 = 0; iVar8 != 0; iVar8 = iVar8 + -1) {
                *pbVar15 = *pbVar14;
                pbVar14 = pbVar14 + 1;
                pbVar15 = pbVar15 + 1;
            }
            param_3 = param_3 + uVar9;
        }
        iStack_108 = (int)((unsigned int)iStack_108 + uVar9);
        break;
    case 2:
        if (param_3 != (byte_t *)0x0) {
            *(byte_t **)(pbStack_104 + 0x14) = param_3;
            puVar13 = (unsigned int *)param_1[5];
            pbVar14 = param_3;
            for (iVar8 = 0xc; iVar8 != 0; iVar8 = iVar8 + -1) {
                *(unsigned int *)pbVar14 = *puVar13;
                puVar13 = puVar13 + 1;
                pbVar14 = pbVar14 + 4;
            }
            param_3 = param_3 + 0x30;
        }
        iStack_108 = iStack_108 + 0x30;
        break;
    case 3:
        uVar9 = (unsigned int)param_1[4] * 0xcu;
        if (param_3 != (byte_t *)0x0) {
            *(byte_t **)(pbStack_104 + 0x14) = param_3;
            pbVar14 = (byte_t *)param_1[5];
            pbVar15 = param_3;
            for (uVar10 = uVar9 >> 2; uVar10 != 0; uVar10 = uVar10 - 1) {
                *(unsigned int *)pbVar15 = *(unsigned int *)pbVar14;
                pbVar14 = pbVar14 + 4;
                pbVar15 = pbVar15 + 4;
            }
            for (iVar8 = 0; iVar8 != 0; iVar8 = iVar8 + -1) {
                *pbVar15 = *pbVar14;
                pbVar14 = pbVar14 + 1;
                pbVar15 = pbVar15 + 1;
            }
            param_3 = param_3 + uVar9;
        }
        iStack_108 = (int)((unsigned int)iStack_108 + uVar9);
        DAT_0060f204 = param_1[4];
        __builtin_memcpy(&tmp32, DAT_0099947e, 4);
        if ((int)tmp32 < param_1[4]) {
            tmp32 = (unsigned int)param_1[4];
            __builtin_memcpy(DAT_0099947e, &tmp32, 4);
        }
        break;
    case 4:
        if (param_3 != (byte_t *)0x0) {
            *(byte_t **)(pbStack_104 + 0x14) = param_3;
            puVar13 = (unsigned int *)param_1[5];
            *(unsigned int *)param_3 = *puVar13;
            *(unsigned int *)(param_3 + 4) = puVar13[1];
            *(unsigned int *)(param_3 + 8) = puVar13[2];
            param_3 = param_3 + 0xc;
        }
        iStack_108 = iStack_108 + 0xc;
        break;
    case 5:
        if (param_3 != (byte_t *)0x0) {
            *(byte_t **)(pbStack_104 + 0x14) = param_3;
            puVar13 = (unsigned int *)param_1[5];
            pbVar14 = param_3;
            for (iVar8 = 9; iVar8 != 0; iVar8 = iVar8 + -1) {
                *(unsigned int *)pbVar14 = *puVar13;
                puVar13 = puVar13 + 1;
                pbVar14 = pbVar14 + 4;
            }
            param_3 = param_3 + 0x24;
        }
        iStack_108 = iStack_108 + 0x24;
        break;
    case 6:
        if (param_3 != (byte_t *)0x0) {
            *(byte_t **)(pbStack_104 + 0x14) = param_3;
            puVar13 = (unsigned int *)param_1[5];
            *(unsigned int *)param_3 = *puVar13;
            *(unsigned int *)(param_3 + 4) = puVar13[1];
            *(unsigned int *)(param_3 + 8) = puVar13[2];
            param_3 = param_3 + 0xc;
        }
        iStack_108 = iStack_108 + 0xc;
        break;
    case 7:
        uVar9 = 0xffffffff;
        do {
            if (uVar9 == 0) break;
            uVar9 = uVar9 - 1;
            iVar8 = *piVar12;
            piVar12 = (int *)((int)piVar12 + 1);
        } while ((char)iVar8 != '\0');
        uVar9 = ~uVar9;
        if (param_3 != (byte_t *)0x0) {
            *(byte_t **)(pbStack_104 + 0x14) = param_3;
            pbVar14 = (byte_t *)param_1[5];
            pbVar15 = param_3;
            for (uVar10 = uVar9 >> 2; uVar10 != 0; uVar10 = uVar10 - 1) {
                *(unsigned int *)pbVar15 = *(unsigned int *)pbVar14;
                pbVar14 = pbVar14 + 4;
                pbVar15 = pbVar15 + 4;
            }
            for (uVar10 = uVar9 & 3; uVar10 != 0; uVar10 = uVar10 - 1) {
                *pbVar15 = *pbVar14;
                pbVar14 = pbVar14 + 1;
                pbVar15 = pbVar15 + 1;
            }
            param_3 = param_3 + uVar9;
        }
        iStack_108 = (int)((unsigned int)iStack_108 + uVar9);
        break;
    case 9:
        uVar9 = (unsigned int)param_1[4] * 0x38u;
        if (param_3 != (byte_t *)0x0) {
            *(byte_t **)(pbStack_104 + 0x14) = param_3;
            pbVar14 = (byte_t *)param_1[5];
            pbVar15 = param_3;
            for (uVar10 = uVar9 >> 2; uVar10 != 0; uVar10 = uVar10 - 1) {
                *(unsigned int *)pbVar15 = *(unsigned int *)pbVar14;
                pbVar14 = pbVar14 + 4;
                pbVar15 = pbVar15 + 4;
            }
            for (iVar8 = 0; iVar8 != 0; iVar8 = iVar8 + -1) {
                *pbVar15 = *pbVar14;
                pbVar14 = pbVar14 + 1;
                pbVar15 = pbVar15 + 1;
            }
            param_3 = param_3 + uVar9;
        }
        iStack_108 = (int)((unsigned int)iStack_108 + uVar9);
        DAT_0060f1d8 = piVar12;
        break;
    case 0xb:
        uVar9 = (unsigned int)param_1[4] * 0xcu;
        if (param_3 != (byte_t *)0x0) {
            *(byte_t **)(pbStack_104 + 0x14) = param_3;
            pbVar14 = (byte_t *)param_1[5];
            pbVar15 = param_3;
            for (uVar10 = uVar9 >> 2; uVar10 != 0; uVar10 = uVar10 - 1) {
                *(unsigned int *)pbVar15 = *(unsigned int *)pbVar14;
                pbVar14 = pbVar14 + 4;
                pbVar15 = pbVar15 + 4;
            }
            for (iVar8 = 0; iVar8 != 0; iVar8 = iVar8 + -1) {
                *pbVar15 = *pbVar14;
                pbVar14 = pbVar14 + 1;
                pbVar15 = pbVar15 + 1;
            }
            param_3 = param_3 + uVar9;
        }
        iStack_108 = (int)((unsigned int)iStack_108 + uVar9);
        _DAT_0060f208 = piVar12;
        param_2[0x21] = (unsigned int)piVar12;
        break;
    case 0xd:
        uVar9 = (unsigned int)param_1[4];
        iVar8 = (int)(uVar9 * 8u);
        if (param_3 != (byte_t *)0x0) {
            *(byte_t **)(pbStack_104 + 0x14) = param_3;
            pbVar14 = (byte_t *)param_1[5];
            pbVar15 = param_3;
            for (iVar11 = (int)((uVar9 & 0x1fffffffu) << 1); iVar11 != 0; iVar11 = iVar11 + -1) {
                *(unsigned int *)pbVar15 = *(unsigned int *)pbVar14;
                pbVar14 = pbVar14 + 4;
                pbVar15 = pbVar15 + 4;
            }
            for (iVar11 = 0; iVar11 != 0; iVar11 = iVar11 + -1) {
                *pbVar15 = *pbVar14;
                pbVar14 = pbVar14 + 1;
                pbVar15 = pbVar15 + 1;
            }
            param_3 = param_3 + iVar8;
        }
        iStack_108 = (int)((unsigned int)iStack_108 + (unsigned int)iVar8);
        break;
    case 0x13:
        if (param_3 != (byte_t *)0x0) {
            *(byte_t **)(pbStack_104 + 0x14) = param_3;
            puVar13 = (unsigned int *)param_1[5];
            *(unsigned int *)param_3 = *puVar13;
            *(unsigned int *)(param_3 + 4) = puVar13[1];
            *(unsigned int *)(param_3 + 8) = puVar13[2];
            param_3 = param_3 + 0xc;
        }
        iStack_108 = iStack_108 + 0xc;
        break;
    case 0x14:
        if (((param_3 != (byte_t *)0x0) && (DAT_009e8f54 != 0)) && (DAT_005233d8 == 1)) {
            iVar8 = (int)((unsigned int)piVar12[5] * (unsigned int)piVar12[4]);
            if (piVar12[2] == iVar8) {
                iVar8 = piVar12[3];
            }
            iVar11 = 0x10;
            pbVar14 = (byte_t *)((int)(piVar12 + 6) + iVar8 + 0x1000);
            do {
                FUN_004452d0((byte_t *)(piVar12 + 6), (int)pbVar14, piVar12[4], piVar12[5]);
                iVar11 = iVar11 + -1;
                pbVar14 = pbVar14 + 0x200;
            } while (iVar11 != 0);
        }
        iVar8 = piVar12[5];
        iVar11 = piVar12[4];
        iVar7 = (int)((unsigned int)iVar8 * (unsigned int)iVar11);
        if (piVar12[2] == iVar7) {
            if (param_3 == (byte_t *)0x0) {
                uStack_100 = (unsigned int)(piVar12[3] + 0x18);
                if (((DAT_005233c4 == 0) && (8 < iVar11)) && (8 < iVar8)) {
                    uStack_100 = uStack_100 - (unsigned int)iVar7;
                }
            } else {
                *(byte_t **)(pbStack_104 + 0x14) = param_3;
                puVar13 = (unsigned int *)param_1[5];
                pbVar14 = param_3;
                for (iVar8 = 6; iVar8 != 0; iVar8 = iVar8 + -1) {
                    *(unsigned int *)pbVar14 = *puVar13;
                    puVar13 = puVar13 + 1;
                    pbVar14 = pbVar14 + 4;
                }
                pbVar14 = (byte_t *)(piVar12 + 6);
                iVar8 = *(int *)(pbStack_104 + 0x14);
                if (((DAT_005233c4 == 0) && (8 < *(int *)(iVar8 + 0x10))) &&
                    (8 < *(int *)(iVar8 + 0x14))) {
                    iVar11 = (int)((unsigned int)*(int *)(iVar8 + 0x14) *
                                   (unsigned int)*(int *)(iVar8 + 0x10));
                    pbVar14 = pbVar14 + iVar11;
                    *(int *)(iVar8 + 0xc) =
                        (int)((unsigned int)*(int *)(iVar8 + 0xc) - (unsigned int)iVar11);
                    *(int *)(iVar8 + 0x10) = *(int *)(iVar8 + 0x10) >> 1;
                    iVar11 = *(int *)(iVar8 + 0x14) >> 1;
                    *(int *)(iVar8 + 0x14) = iVar11;
                    *(int *)(iVar8 + 8) =
                        (int)((unsigned int)*(int *)(iVar8 + 0x10) * (unsigned int)iVar11);
                }
                uVar9 = *(unsigned int *)(iVar8 + 0xc);
                pbVar15 = param_3 + 0x18;
                for (uVar10 = uVar9 >> 2; uVar10 != 0; uVar10 = uVar10 - 1) {
                    *(int *)pbVar15 = *(int *)pbVar14;
                    pbVar14 = pbVar14 + 4;
                    pbVar15 = pbVar15 + 4;
                }
                for (uVar9 = uVar9 & 3; uVar9 != 0; uVar9 = uVar9 - 1) {
                    *pbVar15 = *pbVar14;
                    pbVar14 = pbVar14 + 1;
                    pbVar15 = pbVar15 + 1;
                }
                param_3 = param_3 + 0x18 + *(int *)(iVar8 + 0xc);
                uStack_100 = (unsigned int)(*(int *)(iVar8 + 0xc) + 0x18);
            }
        } else {
            uStack_100 = (unsigned int)iVar7 + 0x18u;
            if (param_3 == (byte_t *)0x0) {
                while ((1 < iVar11) && (1 < iVar8)) {
                    iVar11 = iVar11 >> 1;
                    iVar8 = iVar8 >> 1;
                    uStack_100 = uStack_100 + (unsigned int)iVar11 * (unsigned int)iVar8;
                }
            } else {
                *(byte_t **)(pbStack_104 + 0x14) = param_3;
                pbVar14 = (byte_t *)param_1[5];
                pbVar15 = param_3;
                for (uVar9 = uStack_100 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
                    *(unsigned int *)pbVar15 = *(unsigned int *)pbVar14;
                    pbVar14 = pbVar14 + 4;
                    pbVar15 = pbVar15 + 4;
                }
                for (uVar9 = uStack_100 & 3; uVar9 != 0; uVar9 = uVar9 - 1) {
                    *pbVar15 = *pbVar14;
                    pbVar14 = pbVar14 + 1;
                    pbVar15 = pbVar15 + 1;
                }
                pbStack_e0 = (byte_t *)(piVar12 + 6);
                if (piVar12[1] == 0) {
                    pbVar14 = (byte_t *)*piVar12;
                } else {
                    pbVar14 = pbStack_e0 + (int)((unsigned int)piVar12[5] * (unsigned int)piVar12[4]);
                }
                pbVar14 = pbVar14 + 0x2000;
                iVar8 = piVar12[4];
                iVar11 = piVar12[5];
                iVar7 = *(int *)(pbStack_104 + 0x14);
                *(int *)(iVar7 + 8) = (int)((unsigned int)iVar8 * (unsigned int)iVar11);
                *(int *)(iVar7 + 0xc) = (int)((unsigned int)iVar8 * (unsigned int)iVar11);
                pbVar15 = param_3 + uStack_100;
                while (param_3 = pbVar15, (1 < iVar8) && (1 < iVar11)) {
                    iVar8 = iVar8 >> 1;
                    iVar11 = iVar11 >> 1;
                    iVar6 = (int)((unsigned int)iVar8 * (unsigned int)iVar11);
                    *(int *)(iVar7 + 0xc) =
                        (int)((unsigned int)*(int *)(iVar7 + 0xc) + (unsigned int)iVar6);
                    if ((DAT_005233ac != 0) && (0 < iVar11)) {
                        pbStack_dc = pbStack_e0 + (int)((unsigned int)iVar8 * 2u) + 1;
                        pbStack_e4 = param_3;
                        iStack_d4 = iVar11;
                        do {
                            iStack_ec = 0;
                            if (0 < iVar8) {
                                pbStack_f0 = pbStack_dc;
                                pbVar15 = pbStack_e0;
                                do {
                                    uVar2 = *(unsigned short *)(pbVar14 + (unsigned int)*pbVar15 * 2);
                                    uVar3 = *(unsigned short *)(pbVar14 + (unsigned int)pbVar15[1] * 2);
                                    uVar4 = *(unsigned short *)(pbVar14 + (unsigned int)*pbStack_f0 * 2);
                                    uVar5 = *(unsigned short *)(pbVar14 + (unsigned int)pbStack_f0[-1] * 2);
                                    uVar9 = FUN_00476b00(
                                        (int)pbVar14,
                                        (int)((unsigned int)(uVar2 >> 0xb) + (unsigned int)(uVar3 >> 0xb) +
                                              (unsigned int)(uVar4 >> 0xb) + (unsigned int)(uVar5 >> 0xb)) >> 2,
                                        (int)((uVar3 >> 5 & 0x3f) + (uVar2 >> 5 & 0x3f) +
                                              (uVar4 >> 5 & 0x3f) + (uVar5 >> 5 & 0x3f)) >> 2,
                                        (int)((uVar3 & 0x1f) + (uVar2 & 0x1f) + (uVar4 & 0x1f) +
                                              (uVar5 & 0x1f)) >> 2,
                                        0, 0x100);
                                    pbVar15 = pbVar15 + 2;
                                    pbStack_e4[iStack_ec] = (byte_t)uVar9;
                                    iStack_ec = iStack_ec + 1;
                                    pbStack_f0 = pbStack_f0 + 2;
                                } while (iStack_ec < iVar8);
                            }
                            pbStack_e4 = pbStack_e4 + iVar8;
                            pbStack_e0 = pbStack_e0 + (int)((unsigned int)iVar8 * 4u);
                            pbStack_dc = pbStack_dc + (int)((unsigned int)iVar8 * 4u);
                            iStack_d4 = iStack_d4 + -1;
                        } while (iStack_d4 != 0);
                    }
                    uStack_100 = uStack_100 + (unsigned int)iVar6;
                    pbVar15 = param_3 + iVar6;
                    pbStack_e0 = param_3;
                }
            }
        }
        puVar13 = (unsigned int *)param_1[5];
        iVar8 = (int)puVar13[1];
        if (iVar8 == 0) {
            iVar8 = (int)(puVar13[5] * puVar13[4]);
            if ((int)puVar13[2] == iVar8) {
                iVar8 = (int)puVar13[3];
            }
            pbVar14 = (byte_t *)((int)puVar13 + iVar8 + 0x18);
            if (((byte_t *)*puVar13 == pbVar14) &&
                (uStack_100 = uStack_100 + (unsigned int)DAT_005233d8 * 0x1000u,
                 param_3 != (byte_t *)0x0)) {
                if (DAT_005233d8 == 2) {
                    pbVar14 = pbVar14 + 0x1000;
                }
                if (DAT_005233d8 == 1) {
                    iVar8 = 0;
                    do {
                        iVar11 = iVar8 + 1;
                        param_3[iVar8] =
                            *(byte_t *)((unsigned int)*(unsigned short *)(pbVar14 + iVar8 * 2 + 0x1000) +
                                        DAT_009994bc);
                        iVar8 = iVar11;
                    } while (iVar11 < 0x1000);
                } else {
                    pbVar15 = param_3;
                    for (iVar8 = (int)(((unsigned int)DAT_005233d8 & 0xfffffu) << 10); iVar8 != 0;
                         iVar8 = iVar8 + -1) {
                        *(unsigned int *)pbVar15 = *(unsigned int *)pbVar14;
                        pbVar14 = pbVar14 + 4;
                        pbVar15 = pbVar15 + 4;
                    }
                    for (iVar8 = 0; iVar8 != 0; iVar8 = iVar8 + -1) {
                        *pbVar15 = *pbVar14;
                        pbVar14 = pbVar14 + 1;
                        pbVar15 = pbVar15 + 1;
                    }
                }
                if (DAT_005233d8 == 2) {
                    if (*param_1 != 0) {
                        FUN_004079e0();
                    }
                    FUN_00476b90();
                }
                iVar8 = (int)((unsigned int)DAT_005233d8 << 0xc);
                goto LAB_004776ef;
            }
        } else {
            uStack_100 = uStack_100 +
                         (unsigned int)iVar8 * (unsigned int)DAT_005233d8 * 0x100u;
            if (param_3 != (byte_t *)0x0) {
                iVar11 = (int)(puVar13[5] * puVar13[4]);
                if ((int)puVar13[2] == iVar11) {
                    iVar11 = (int)puVar13[3];
                }
                puVar16 = (unsigned int *)((int)puVar13 + iVar11 + 0x18);
                if (DAT_005233d8 == 2) {
                    puVar16 = puVar16 + (int)((unsigned int)iVar8 * 0x40u);
                }
                if (DAT_005233d8 == 1) {
                    iVar8 = 0;
                    do {
                        iVar11 = iVar8 + 1;
                        param_3[iVar8] =
                            *(byte_t *)((unsigned int)*(unsigned short *)((int)puVar16 + iVar8 * 2 + 0x1000) +
                                        DAT_009994bc);
                        iVar8 = iVar11;
                    } while (iVar11 < 0x1000);
                } else {
                    pbVar14 = param_3;
                    for (iVar8 = (int)(((unsigned int)DAT_005233d8 & 0xfffffu) << 10); iVar8 != 0;
                         iVar8 = iVar8 + -1) {
                        *(unsigned int *)pbVar14 = *puVar16;
                        puVar16 = puVar16 + 1;
                        pbVar14 = pbVar14 + 4;
                    }
                }
                if (DAT_005233d8 == 2) {
                    if (*param_1 != 0) {
                        FUN_004079e0();
                    }
                    FUN_00476b90();
                }
                iVar8 = (int)(puVar13[1] * (unsigned int)DAT_005233d8 * 0x100u);
LAB_004776ef:
                param_3 = param_3 + iVar8;
            }
        }
        iStack_108 = (int)((unsigned int)iStack_108 + uStack_100);
        break;
    case 0x15:
        uVar9 = (unsigned int)param_1[4];
        iVar8 = (int)(uVar9 * 4u);
        if (param_3 != (byte_t *)0x0) {
            *(byte_t **)(pbStack_104 + 0x14) = param_3;
            pbVar14 = (byte_t *)param_1[5];
            pbVar15 = param_3;
            for (uVar9 = uVar9 & 0x3fffffffu; uVar9 != 0; uVar9 = uVar9 - 1) {
                *(unsigned int *)pbVar15 = *(unsigned int *)pbVar14;
                pbVar14 = pbVar14 + 4;
                pbVar15 = pbVar15 + 4;
            }
            for (iVar11 = 0; iVar11 != 0; iVar11 = iVar11 + -1) {
                *pbVar15 = *pbVar14;
                pbVar14 = pbVar14 + 1;
                pbVar15 = pbVar15 + 1;
            }
            param_3 = param_3 + iVar8;
        }
        iStack_108 = (int)((unsigned int)iStack_108 + (unsigned int)iVar8);
        break;
    case 0x16:
        if (param_3 != (byte_t *)0x0) {
            *(byte_t **)(pbStack_104 + 0x14) = param_3;
            puVar13 = (unsigned int *)param_1[5];
            *(unsigned int *)param_3 = *puVar13;
            *(unsigned int *)(param_3 + 4) = puVar13[1];
            *(unsigned int *)(param_3 + 8) = puVar13[2];
            *(unsigned int *)(param_3 + 0xc) = puVar13[3];
            param_3 = param_3 + 0x10;
        }
        iStack_108 = iStack_108 + 0x10;
        break;
    case 0x17:
        if (param_3 != (byte_t *)0x0) {
            *(byte_t **)(pbStack_104 + 0x14) = param_3;
            puVar13 = (unsigned int *)param_1[5];
            pbVar14 = param_3;
            for (iVar8 = 0xc; iVar8 != 0; iVar8 = iVar8 + -1) {
                *(unsigned int *)pbVar14 = *puVar13;
                puVar13 = puVar13 + 1;
                pbVar14 = pbVar14 + 4;
            }
            param_3 = param_3 + 0x30;
        }
        iStack_108 = iStack_108 + 0x30;
        break;
    case 0x19:
        if (param_3 != (byte_t *)0x0) {
            *(byte_t **)(pbStack_104 + 0x14) = param_3;
            puVar13 = (unsigned int *)param_1[5];
            pbVar14 = param_3;
            for (iVar8 = 0x12; iVar8 != 0; iVar8 = iVar8 + -1) {
                *(unsigned int *)pbVar14 = *puVar13;
                puVar13 = puVar13 + 1;
                pbVar14 = pbVar14 + 4;
            }
            param_3 = param_3 + 0x48;
        }
        iStack_108 = iStack_108 + 0x48;
    }
    iVar8 = 0;
    if (param_1[2] != 0) {
        puVar13 = auStack_d0;
        for (iVar11 = 0x34; iVar11 != 0; iVar11 = iVar11 + -1) {
            *puVar13 = *param_2;
            param_2 = param_2 + 1;
            puVar13 = puVar13 + 1;
        }
        DAT_0060f1c0 = 0;
        DAT_0060f1e8 = 0;
        iVar11 = 0;
        _DAT_0060f208 = (int *)0x0;
        _DAT_0060f1f0 = 0;
        DAT_0060f1d8 = (int *)0x0;
        if (0 < param_1[2]) {
            do {
                if (param_3 != (byte_t *)0x0) {
                    *(byte_t **)(*(int *)(pbStack_104 + 0xc) + iVar8) = param_3;
                }
                iVar7 = FUN_00476c20(*(int **)(param_1[3] + iVar8), auStack_d0, param_3);
                if (param_3 != (byte_t *)0x0) {
                    if (iVar7 == 0) {
                        *(unsigned int *)(*(int *)(pbStack_104 + 0xc) + iVar8) = 0;
                    }
                    param_3 = param_3 + iVar7;
                }
                iVar8 = iVar8 + 4;
                iStack_108 = (int)((unsigned int)iStack_108 + (unsigned int)iVar7);
                iVar11 = iVar11 + 1;
            } while (iVar11 < param_1[2]);
        }
    }
    return iStack_108;
}

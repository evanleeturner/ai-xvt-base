/* SPDX-License-Identifier: MIT */
/* z_xvt 0042bc90 */
/* Prototypes */
typedef unsigned char  uchar_t;
typedef unsigned short ushort_t;
typedef unsigned int   uint_t;

int   FUN_00432ca0(void);
int   FUN_00431d30(void);
int   FUN_004330b0(void);
void  FUN_00438a40(void);
void  FUN_00438c30(int a, char *b, int c);
void  FUN_00438b70(void);
int  *FUN_00432b70(int *a, int *b, int c);
int   FUN_00432c10(int a);
int  *FUN_00432cc0(int a);
void  FUN_004310c0(uint_t *a, int b);
void  FUN_00431110(int a, uint_t *b, int c);
DWORD __stdcall timeGetTime(void);

/* Globals */
extern int            DAT_0072f444;
extern int            DAT_0052552c;
extern uchar_t        DAT_0081e016[4];   /* unaligned 32-bit at 0x81e016 */
extern ushort_t       DAT_0071bcb6;
extern ushort_t       DAT_0071b738;
extern uint_t         DAT_005373d0[];    /* [0]=0x5373d0 [1]=0x5373d4 [2]=0x5373d8 ... */
extern char          *DAT_006a89c8;
extern char          *DAT_006a89cc;
extern char          *DAT_006a89d0;
extern char          *DAT_006a89d4;
extern int            DAT_00732e88;
extern int            DAT_0071ecf4;

unsigned int __stdcall FUN_0042bc90(void)
{
    int bVar3;
    DWORD local_110;
    int iVar5;
    int iVar6;
    int *piVar7;
    int *piVar9;
    DWORD DVar8;
    int local_114;
    int local_10c;
    DWORD local_108;
    int local_104;
    uchar_t local_100[256];
    uchar_t *p;
    unsigned short sv;
    uint_t uv;
    uint_t uVar11;
    uint_t uVar12;
    char cVar1;
    uchar_t bVar2;
    char *pcVar17;
    char *pcVar18;
    char *pcVar9;
    char *pcVar16;
    uchar_t *pbVar14;
    uchar_t *pbVar19;
    uint_t *puVar13;

    bVar3 = 1;
    local_110 = 0;
    iVar5 = FUN_00432ca0();
    if (DAT_0072f444 < 2) {
        DAT_0071bcb6 = (ushort_t)DAT_0052552c;
        __builtin_memcpy(&uv, DAT_0081e016, 4);
        DAT_0071b738 = (ushort_t)uv;
    }
    else {
        iVar6 = FUN_00431d30();
        if (iVar6 == 0) {
            DAT_005373d0[1] = (uint_t)DAT_0052552c;
            __builtin_memcpy(&DAT_005373d0[2], DAT_0081e016, 4);
            DAT_005373d0[0] = 0x11;
            FUN_00431110(iVar5, DAT_005373d0, 0xc);
            FUN_00438a40();
            FUN_00438c30(1, DAT_006a89c8, 0x30);
            do {
                piVar7 = FUN_00432b70(&local_114, &local_104, 0x3c);
                if (piVar7 == (int *)0) {
                    return 0;
                }
                if ((*piVar7 == 0x1d) &&
                    (DVar8 = timeGetTime(), 200 < (int)(DVar8 - local_110))) {
                    local_110 = DVar8;
                    iVar5 = FUN_00432c10(local_114);
                    piVar9 = FUN_00432cc0(iVar5);
                    if (piVar9 == (int *)0) {
                        uVar11 = 0xffffffffu;
                        pcVar17 = DAT_006a89cc;
                        do {
                            pcVar18 = pcVar17;
                            if (uVar11 == 0) break;
                            uVar11 = uVar11 - 1;
                            pcVar18 = pcVar17 + 1;
                            cVar1 = *pcVar17;
                            pcVar17 = pcVar18;
                        } while (cVar1 != '\0');
                        uVar11 = ~uVar11;
                        pbVar14 = (uchar_t *)(pcVar18 - uVar11);
                        pbVar19 = local_100;
                        for (uVar12 = uVar11 >> 2; uVar12 != 0; uVar12 = uVar12 - 1) {
                            pbVar19[0] = pbVar14[0];
                            pbVar19[1] = pbVar14[1];
                            pbVar19[2] = pbVar14[2];
                            pbVar19[3] = pbVar14[3];
                            pbVar14 = pbVar14 + 4;
                            pbVar19 = pbVar19 + 4;
                        }
                    }
                    else {
                        uVar11 = 0xffffffffu;
                        pcVar9 = (char *)piVar9;
                        do {
                            pcVar16 = pcVar9;
                            if (uVar11 == 0) break;
                            uVar11 = uVar11 - 1;
                            pcVar16 = pcVar9 + 1;
                            cVar1 = *pcVar9;
                            pcVar9 = pcVar16;
                        } while (cVar1 != '\0');
                        uVar11 = ~uVar11;
                        pbVar14 = (uchar_t *)(pcVar16 - uVar11);
                        pbVar19 = local_100;
                        for (uVar12 = uVar11 >> 2; uVar12 != 0; uVar12 = uVar12 - 1) {
                            pbVar19[0] = pbVar14[0];
                            pbVar19[1] = pbVar14[1];
                            pbVar19[2] = pbVar14[2];
                            pbVar19[3] = pbVar14[3];
                            pbVar14 = pbVar14 + 4;
                            pbVar19 = pbVar19 + 4;
                        }
                        for (uVar11 = uVar11 & 3; uVar11 != 0; uVar11 = uVar11 - 1) {
                            *pbVar19 = *pbVar14;
                            pbVar14 = pbVar14 + 1;
                            pbVar19 = pbVar19 + 1;
                        }
                        bVar3 = !bVar3;
                        pcVar17 = DAT_006a89d4;
                        if (bVar3) {
                            pcVar17 = DAT_006a89d0;
                        }
                        uVar11 = 0xffffffffu;
                        do {
                            pcVar18 = pcVar17;
                            if (uVar11 == 0) break;
                            uVar11 = uVar11 - 1;
                            pcVar18 = pcVar17 + 1;
                            cVar1 = *pcVar17;
                            pcVar17 = pcVar18;
                        } while (cVar1 != '\0');
                        uVar11 = ~uVar11;
                        iVar5 = -1;
                        pbVar14 = local_100;
                        do {
                            pbVar19 = pbVar14;
                            if (iVar5 == 0) break;
                            iVar5 = iVar5 + -1;
                            pbVar19 = pbVar14 + 1;
                            bVar2 = *pbVar14;
                            pbVar14 = pbVar19;
                        } while (bVar2 != 0);
                        pbVar14 = (uchar_t *)(pcVar18 - uVar11);
                        pbVar19 = pbVar19 - 1;
                        for (uVar12 = uVar11 >> 2; uVar12 != 0; uVar12 = uVar12 - 1) {
                            pbVar19[0] = pbVar14[0];
                            pbVar19[1] = pbVar14[1];
                            pbVar19[2] = pbVar14[2];
                            pbVar19[3] = pbVar14[3];
                            pbVar14 = pbVar14 + 4;
                            pbVar19 = pbVar19 + 4;
                        }
                    }
                    for (uVar11 = uVar11 & 3; uVar11 != 0; uVar11 = uVar11 - 1) {
                        *pbVar19 = *pbVar14;
                        pbVar14 = pbVar14 + 1;
                        pbVar19 = pbVar19 + 1;
                    }
                    FUN_00438c30(3, (char *)local_100, 0x30);
                }
            } while (*piVar7 != 0x12);
            FUN_00438b70();
            DAT_00732e88 = piVar7[1];
            if (0 < DAT_0072f444) {
                iVar5 = DAT_0072f444;
                p = (uchar_t *)&DAT_0071bcb6;
                do {
                    sv = (unsigned short)piVar7[2];
                    __builtin_memcpy(p, &sv, 2);
                    iVar5 = iVar5 + -1;
                    sv = (unsigned short)piVar7[3];
                    __builtin_memcpy(p - 0x57e, &sv, 2);
                    p = p + 0x5bd;
                    piVar7 = piVar7 + 2;
                } while (iVar5 != 0);
            }
        }
        else {
            if (0 < DAT_0072f444) {
                iVar5 = DAT_0072f444;
                p = (uchar_t *)&DAT_0071bcb6;
                do {
                    sv = 0x13;
                    __builtin_memcpy(p, &sv, 2);
                    sv = 0;
                    __builtin_memcpy(p - 0x57e, &sv, 2);
                    iVar5 = iVar5 + -1;
                    p = p + 0x5bd;
                } while (iVar5 != 0);
            }
            FUN_004330b0();
            __builtin_memcpy(&uv, DAT_0081e016, 4);
            iVar5 = (int)((unsigned int)DAT_0071ecf4 * 0x5bdu);
            sv = (unsigned short)DAT_0052552c;
            __builtin_memcpy((uchar_t *)&DAT_0071bcb6 + iVar5, &sv, 2);
            sv = (unsigned short)uv;
            __builtin_memcpy((uchar_t *)&DAT_0071b738 + iVar5, &sv, 2);
            FUN_00438a40();
            FUN_00438c30(1, DAT_006a89c8, 0x30);
            local_10c = 0;
            local_108 = timeGetTime();
            while (iVar5 = FUN_004330b0(), local_10c < (int)((unsigned int)iVar5 - 1u)) {
                piVar7 = FUN_00432b70(&local_114, &local_104, 0x3c);
                DVar8 = timeGetTime();
                if (piVar7 == (int *)0) {
                    if (60000 < DVar8 - local_108) {
                        return 0;
                    }
                }
                else {
                    local_108 = DVar8;
                    if ((*piVar7 == 0x1d) &&
                        (DVar8 = timeGetTime(), 200 < (int)(DVar8 - local_110))) {
                        local_110 = DVar8;
                        iVar5 = FUN_00432c10(local_114);
                        piVar9 = FUN_00432cc0(iVar5);
                        if (piVar9 == (int *)0) {
                            uVar11 = 0xffffffffu;
                            pcVar17 = DAT_006a89cc;
                            do {
                                pcVar18 = pcVar17;
                                if (uVar11 == 0) break;
                                uVar11 = uVar11 - 1;
                                pcVar18 = pcVar17 + 1;
                                cVar1 = *pcVar17;
                                pcVar17 = pcVar18;
                            } while (cVar1 != '\0');
                            uVar11 = ~uVar11;
                            pbVar14 = (uchar_t *)(pcVar18 - uVar11);
                            pbVar19 = local_100;
                            for (uVar12 = uVar11 >> 2; uVar12 != 0; uVar12 = uVar12 - 1) {
                                pbVar19[0] = pbVar14[0];
                                pbVar19[1] = pbVar14[1];
                                pbVar19[2] = pbVar14[2];
                                pbVar19[3] = pbVar14[3];
                                pbVar14 = pbVar14 + 4;
                                pbVar19 = pbVar19 + 4;
                            }
                        }
                        else {
                            uVar11 = 0xffffffffu;
                            pcVar9 = (char *)piVar9;
                            do {
                                pcVar16 = pcVar9;
                                if (uVar11 == 0) break;
                                uVar11 = uVar11 - 1;
                                pcVar16 = pcVar9 + 1;
                                cVar1 = *pcVar9;
                                pcVar9 = pcVar16;
                            } while (cVar1 != '\0');
                            uVar11 = ~uVar11;
                            pbVar14 = (uchar_t *)(pcVar16 - uVar11);
                            pbVar19 = local_100;
                            for (uVar12 = uVar11 >> 2; uVar12 != 0; uVar12 = uVar12 - 1) {
                                pbVar19[0] = pbVar14[0];
                                pbVar19[1] = pbVar14[1];
                                pbVar19[2] = pbVar14[2];
                                pbVar19[3] = pbVar14[3];
                                pbVar14 = pbVar14 + 4;
                                pbVar19 = pbVar19 + 4;
                            }
                            for (uVar11 = uVar11 & 3; uVar11 != 0; uVar11 = uVar11 - 1) {
                                *pbVar19 = *pbVar14;
                                pbVar14 = pbVar14 + 1;
                                pbVar19 = pbVar19 + 1;
                            }
                            bVar3 = !bVar3;
                            pcVar17 = DAT_006a89d4;
                            if (bVar3) {
                                pcVar17 = DAT_006a89d0;
                            }
                            uVar11 = 0xffffffffu;
                            do {
                                pcVar18 = pcVar17;
                                if (uVar11 == 0) break;
                                uVar11 = uVar11 - 1;
                                pcVar18 = pcVar17 + 1;
                                cVar1 = *pcVar17;
                                pcVar17 = pcVar18;
                            } while (cVar1 != '\0');
                            uVar11 = ~uVar11;
                            iVar5 = -1;
                            pbVar14 = local_100;
                            do {
                                pbVar19 = pbVar14;
                                if (iVar5 == 0) break;
                                iVar5 = iVar5 + -1;
                                pbVar19 = pbVar14 + 1;
                                bVar2 = *pbVar14;
                                pbVar14 = pbVar19;
                            } while (bVar2 != 0);
                            pbVar14 = (uchar_t *)(pcVar18 - uVar11);
                            pbVar19 = pbVar19 - 1;
                            for (uVar12 = uVar11 >> 2; uVar12 != 0; uVar12 = uVar12 - 1) {
                                pbVar19[0] = pbVar14[0];
                                pbVar19[1] = pbVar14[1];
                                pbVar19[2] = pbVar14[2];
                                pbVar19[3] = pbVar14[3];
                                pbVar14 = pbVar14 + 4;
                                pbVar19 = pbVar19 + 4;
                            }
                        }
                        for (uVar11 = uVar11 & 3; uVar11 != 0; uVar11 = uVar11 - 1) {
                            *pbVar19 = *pbVar14;
                            pbVar14 = pbVar14 + 1;
                            pbVar19 = pbVar19 + 1;
                        }
                        FUN_00438c30(3, (char *)local_100, 0x30);
                    }
                    if (*piVar7 == 0x11) {
                        iVar5 = FUN_00432c10(local_114);
                        iVar5 = (int)((unsigned int)iVar5 * 0x5bdu);
                        sv = (unsigned short)piVar7[1];
                        __builtin_memcpy((uchar_t *)&DAT_0071bcb6 + iVar5, &sv, 2);
                        sv = (unsigned short)piVar7[2];
                        __builtin_memcpy((uchar_t *)&DAT_0071b738 + iVar5, &sv, 2);
                        local_10c = (int)((unsigned int)local_10c + 1u);
                    }
                }
            }
            FUN_00438b70();
            DAT_005373d0[1] = (uint_t)DAT_00732e88;
            DAT_005373d0[0] = 0x12;
            if (0 < DAT_0072f444) {
                iVar5 = DAT_0072f444;
                puVar13 = &DAT_005373d0[2];
                p = (uchar_t *)&DAT_0071bcb6;
                do {
                    __builtin_memcpy(&sv, p, 2);
                    *puVar13 = (uint_t)sv;
                    iVar5 = iVar5 + -1;
                    __builtin_memcpy(&sv, p - 0x57e, 2);
                    puVar13[1] = (uint_t)sv;
                    puVar13 = puVar13 + 2;
                    p = p + 0x5bd;
                } while (iVar5 != 0);
            }
            FUN_004310c0(DAT_005373d0, (int)((unsigned int)DAT_0072f444 * 8u + 8u));
            do {
                piVar7 = FUN_00432b70(&local_114, &local_104, 0x3c);
                if (piVar7 == (int *)0) {
                    return 0;
                }
            } while (*piVar7 != 0x12);
        }
    }
    return 1;
}

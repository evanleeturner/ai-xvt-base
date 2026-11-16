/* SPDX-License-Identifier: MIT */
/* z_bop 004d00d0 */
extern unsigned char DAT_00acebf9[];
extern unsigned char DAT_00acebfd[];
extern unsigned char DAT_00acebf1[];
extern unsigned char DAT_00acec01[];
extern unsigned char DAT_00acf145[];
extern unsigned char DAT_00b69a89[];
extern unsigned char DAT_00acec35[];
extern unsigned char DAT_00acec45[];
extern unsigned char DAT_00acec49[];
extern unsigned char DAT_00b63f91[];
extern unsigned char DAT_00b63f95[];
extern unsigned char DAT_00b63f99[];
extern unsigned char DAT_00b63f9d[];
extern unsigned char DAT_00b63fa1[];
extern unsigned char DAT_00b63fa5[];
extern unsigned char DAT_00b63fa9[];
extern unsigned char DAT_00b63fad[];
extern unsigned char DAT_00b63fb1[];
extern unsigned char DAT_00b641b1[];
extern unsigned char DAT_00b641b5[];
extern unsigned char DAT_00b641b9[];
extern unsigned char DAT_00b641bd[];
extern unsigned char DAT_00b641c1[];
extern unsigned char DAT_00b641c5[];
extern int DAT_00665050[];
extern int DAT_00665410;

void __cdecl FUN_004cdc20(void);
void __cdecl FUN_004cef70(int param_1, void *param_2, unsigned int param_3);
void __cdecl FUN_004d13b0(int param_1);
DWORD __stdcall GetTickCount(void);

void __cdecl FUN_004d00d0(int param_1, int param_2)
{
    char cVar1;
    unsigned char *puVar2;
    unsigned char *puVar3;
    unsigned int uVar4;
    unsigned int uVar5;
    int iVar6;
    char *pcVar7;
    unsigned int uVar8;
    unsigned char *pbTbl;
    int *piVar9;
    char *pcVar10;
    char *pcVar11;
    unsigned int local_200[128];
    unsigned int t;
    unsigned int n;
    unsigned int c;

    if (param_1 == 3) {
        __builtin_memcpy(&t, DAT_00acebf9, 4);
        if (t == 0) {
            t = 1;
            __builtin_memcpy(DAT_00acebfd, &t, 4);
            FUN_004cdc20();
            return;
        }
        __builtin_memcpy(&c, DAT_00acebf1, 4);
        if ((*(int *)(param_2 + 4) == 1) && (*(int *)(param_2 + 8) != (int)c)) {
            t = 1;
            __builtin_memcpy(DAT_00acebfd, &t, 4);
            FUN_004cdc20();
            __builtin_memcpy(&local_200[1], DAT_00acebfd, 4);
            __builtin_memcpy(&local_200[2], DAT_00b69a89, 4);
            local_200[0] = 0x3b;
            local_200[3] = GetTickCount();
            uVar8 = 0;
            __builtin_memcpy(&n, DAT_00b69a89, 4);
            if (n != 0) {
                puVar2 = DAT_00b63f91;
                puVar3 = (unsigned char *)&local_200[4];
                do {
                    uVar8 = uVar8 + 1;
                    __builtin_memcpy(puVar3, puVar2, 4);
                    puVar3[4] = puVar2[0xc];
                    puVar3[5] = puVar2[0x14];
                    puVar3[6] = puVar2[0x10];
                    puVar3[7] = puVar2[0x18];
                    puVar2 = puVar2 + 0x238;
                    puVar3 = puVar3 + 8;
                    __builtin_memcpy(&n, DAT_00b69a89, 4);
                } while (uVar8 < n);
            }
            __builtin_memcpy(&n, DAT_00b69a89, 4);
            __builtin_memcpy(&c, DAT_00b69a89, 4);
            FUN_004cef70(*(int *)(param_2 + 8), &local_200[0], c * 8 + 0x10);
            return;
        }
    }
    else {
        if (param_1 == 5) {
            __builtin_memcpy(&t, DAT_00acebf9, 4);
            if (t == 0) {
                __builtin_memcpy(&c, DAT_00acebf1, 4);
                if ((*(int *)(param_2 + 4) == 1) && (*(int *)(param_2 + 8) == (int)c)) {
                    local_200[0] = 0x46;
                    __builtin_memcpy(&c, DAT_00acf145, 4);
                    FUN_004cef70((int)c, &local_200[0], 4);
                }
            }
            else if (*(int *)(param_2 + 4) == 1) {
                uVar8 = 0;
                __builtin_memcpy(&t, DAT_00acebfd, 4);
                if (t != 0) {
                    pbTbl = DAT_00acec45;
                    do {
                        __builtin_memcpy(&c, pbTbl, 4);
                        if ((int)c == *(int *)(param_2 + 8)) {
                            __builtin_memcpy(&n, DAT_00acec49 + uVar8 * 40, 4);
                            if (n != 0) {
                                t = 1;
                                __builtin_memcpy(DAT_00acec01, &t, 4);
                            }
                            break;
                        }
                        pbTbl = pbTbl + 40;
                        uVar8 = uVar8 + 1;
                        __builtin_memcpy(&t, DAT_00acebfd, 4);
                    } while (uVar8 < t);
                }
                FUN_004d13b0(*(int *)(param_2 + 8));
                uVar8 = 0;
                __builtin_memcpy(&n, DAT_00b69a89, 4);
                if (n != 0) {
                    pbTbl = DAT_00b63f91;
LAB_004d026e:
                    __builtin_memcpy(&c, pbTbl, 4);
                    if ((int)c != *(int *)(param_2 + 8)) goto code_r0x004d0272;
                    __builtin_memcpy(&n, DAT_00b69a89, 4);
                    n = n - 1;
                    __builtin_memcpy(DAT_00b69a89, &n, 4);
                    puVar2 = DAT_00b63f91 + n * 0x238;
                    puVar3 = DAT_00b63f91 + uVar8 * 0x238;
                    for (iVar6 = 0x8e; iVar6 != 0; iVar6 = iVar6 + -1) {
                        __builtin_memcpy(&t, puVar2, 4);
                        __builtin_memcpy(puVar3, &t, 4);
                        puVar2 = puVar2 + 4;
                        puVar3 = puVar3 + 4;
                    }
                    t = 0;
                    __builtin_memcpy(DAT_00b63f91 + n * 0x238, &t, 4);
                    t = 0x7f;
                    __builtin_memcpy(DAT_00b63f95 + n * 0x238, &t, 4);
                    __builtin_memcpy(DAT_00b63f9d + n * 0x238, &t, 4);
                    __builtin_memcpy(DAT_00b63fa5 + n * 0x238, &t, 4);
                    __builtin_memcpy(DAT_00b63f99 + n * 0x238, &t, 4);
                    __builtin_memcpy(DAT_00b63fa1 + n * 0x238, &t, 4);
                    __builtin_memcpy(DAT_00b63fa9 + n * 0x238, &t, 4);
                    t = 0;
                    __builtin_memcpy(DAT_00b63fad + n * 0x238, &t, 4);
                    DAT_00b63fb1[n * 0x238] = 0x39;
                    t = 1;
                    __builtin_memcpy(DAT_00b641b1 + n * 0x238, &t, 4);
                    t = 0;
                    __builtin_memcpy(DAT_00b641c5 + n * 0x238, &t, 4);
                    __builtin_memcpy(DAT_00b641b5 + n * 0x238, &t, 4);
                    __builtin_memcpy(DAT_00b641b9 + n * 0x238, &t, 4);
                    __builtin_memcpy(DAT_00b641bd + n * 0x238, &t, 4);
                    __builtin_memcpy(DAT_00b641c1 + n * 0x238, &t, 4);
                }
LAB_004d0418:
                iVar6 = 0;
                piVar9 = DAT_00665050;
                do {
                    if (*piVar9 == *(int *)(param_2 + 8)) {
                        DAT_00665050[iVar6 * 6 + 0] = 0;
                        DAT_00665050[iVar6 * 6 + 1] = 0;
                        DAT_00665050[iVar6 * 6 + 2] = 0;
                        DAT_00665050[iVar6 * 6 + 3] = 0;
                        DAT_00665050[iVar6 * 6 + 4] = 0;
                        DAT_00665050[iVar6 * 6 + 5] = 0;
                        break;
                    }
                    piVar9 = piVar9 + 6;
                    iVar6 = iVar6 + 1;
                } while (piVar9 < &DAT_00665410);
            }
            t = 1;
            __builtin_memcpy(DAT_00acebfd, &t, 4);
            FUN_004cdc20();
            return;
        }
        if (param_1 != 0x103) {
            return;
        }
        __builtin_memcpy(&t, DAT_00acebfd, 4);
        if ((*(int *)(param_2 + 4) == 1) && (uVar8 = 0, t != 0)) {
            pcVar7 = (char *)DAT_00acec35;
            do {
                __builtin_memcpy(&c, pcVar7 + 0x10, 4);
                if ((int)c == *(int *)(param_2 + 8)) {
                    uVar4 = 0xffffffff;
                    pcVar10 = (char *)(param_2 + 0x1c);
                    do {
                        pcVar11 = pcVar10;
                        if (uVar4 == 0) break;
                        uVar4 = uVar4 - 1;
                        pcVar11 = pcVar10 + 1;
                        cVar1 = *pcVar10;
                        pcVar10 = pcVar11;
                    } while (cVar1 != '\0');
                    uVar4 = ~uVar4;
                    pcVar10 = pcVar11 + -uVar4;
                    pcVar11 = pcVar7;
                    for (uVar5 = uVar4 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
                        __builtin_memcpy(&t, pcVar10, 4);
                        __builtin_memcpy(pcVar11, &t, 4);
                        pcVar10 = pcVar10 + 4;
                        pcVar11 = pcVar11 + 4;
                    }
                    for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
                        *pcVar11 = *pcVar10;
                        pcVar10 = pcVar10 + 1;
                        pcVar11 = pcVar11 + 1;
                    }
                    uVar4 = 0xffffffff;
                    pcVar10 = pcVar7;
                    do {
                        if (uVar4 == 0) break;
                        uVar4 = uVar4 - 1;
                        cVar1 = *pcVar10;
                        pcVar10 = pcVar10 + 1;
                    } while (cVar1 != '\0');
                    uVar5 = 0xffffffff;
                    pcVar10 = (char *)(~uVar4 + 0x1c + (unsigned int)param_2);
                    do {
                        pcVar11 = pcVar10;
                        if (uVar5 == 0) break;
                        uVar5 = uVar5 - 1;
                        pcVar11 = pcVar10 + 1;
                        cVar1 = *pcVar10;
                        pcVar10 = pcVar11;
                    } while (cVar1 != '\0');
                    uVar5 = ~uVar5;
                    pcVar10 = pcVar11 + -uVar5;
                    pcVar11 = pcVar7 + -0x10;
                    for (uVar4 = uVar5 >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
                        __builtin_memcpy(&t, pcVar10, 4);
                        __builtin_memcpy(pcVar11, &t, 4);
                        pcVar10 = pcVar10 + 4;
                        pcVar11 = pcVar11 + 4;
                    }
                    for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                        *pcVar11 = *pcVar10;
                        pcVar10 = pcVar10 + 1;
                        pcVar11 = pcVar11 + 1;
                    }
                    pcVar7[0xc] = '\0';
                    pcVar7[-4] = '\0';
                }
                pcVar7 = pcVar7 + 0x28;
                uVar8 = uVar8 + 1;
                __builtin_memcpy(&t, DAT_00acebfd, 4);
            } while (uVar8 < t);
        }
    }
    return;
code_r0x004d0272:
    pbTbl = pbTbl + 0x238;
    uVar8 = uVar8 + 1;
    __builtin_memcpy(&n, DAT_00b69a89, 4);
    if (n <= uVar8) goto LAB_004d0418;
    goto LAB_004d026e;
}

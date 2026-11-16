/* SPDX-License-Identifier: MIT */
/* z_xvt 004cca00 */
extern unsigned char DAT_0078072e[4];
extern unsigned char DAT_00780726[4];
extern unsigned char DAT_00780732[4];
extern unsigned char DAT_00780736[4];
extern unsigned char DAT_00780c7a[4];
extern unsigned char DAT_0081b5be[4];
extern unsigned char DAT_00815ac6[];   /* table, stride 0x238 */
extern unsigned char DAT_0078076a[];   /* table, stride 0x28 */
extern unsigned char DAT_0078077a[];   /* table, stride 0x28 */
extern int DAT_0064f070[];             /* table, stride 6 ints */
extern int DAT_0064f430[];

void __cdecl FUN_004ca450(void);
void __cdecl FUN_004cb830(int, void *, unsigned int);
void __cdecl FUN_004cddb0(int);
DWORD __stdcall GetTickCount(void);

void __cdecl FUN_004cca00(int param_1, int param_2)
{
    unsigned char auBuf[0x200];
    char cVar1;
    unsigned char *puVar2;
    unsigned char *puVar3;
    unsigned int uVar4;
    unsigned int uVar5;
    int iVar6;
    char *pcVar7;
    unsigned int uVar8;
    unsigned char *piVar9;
    char *pcVar10;
    char *pcVar11;
    unsigned int uA;
    unsigned int uB;
    unsigned int uCnt;
    int iA;
    int iB;
    int *piTbl;
    unsigned char *pbEnt;

    if (param_1 == 3) {
        __builtin_memcpy(&uA, DAT_0078072e, 4);
        if (uA == 0) {
            uA = 1;
            __builtin_memcpy(DAT_00780732, &uA, 4);
            FUN_004ca450();
            return;
        }
        __builtin_memcpy(&iA, DAT_00780726, 4);
        if ((*(int *)(param_2 + 4) == 1) && (*(int *)(param_2 + 8) != iA)) {
            uA = 1;
            __builtin_memcpy(DAT_00780732, &uA, 4);
            FUN_004ca450();
            __builtin_memcpy(&uA, DAT_00780732, 4);
            __builtin_memcpy(auBuf + 4, &uA, 4);
            __builtin_memcpy(&uA, DAT_0081b5be, 4);
            __builtin_memcpy(auBuf + 8, &uA, 4);
            uA = 0x3b;
            __builtin_memcpy(auBuf, &uA, 4);
            uA = GetTickCount();
            __builtin_memcpy(auBuf + 12, &uA, 4);
            uVar8 = 0;
            __builtin_memcpy(&uA, DAT_0081b5be, 4);
            if (uA != 0) {
                puVar2 = DAT_00815ac6;
                puVar3 = auBuf + 16;
                do {
                    uVar8 = uVar8 + 1;
                    __builtin_memcpy(&uB, puVar2, 4);
                    __builtin_memcpy(puVar3, &uB, 4);
                    puVar3[4] = puVar2[12];
                    puVar3[5] = puVar2[20];
                    puVar3[6] = puVar2[16];
                    puVar3[7] = puVar2[24];
                    puVar2 = puVar2 + 0x238;
                    puVar3 = puVar3 + 8;
                    __builtin_memcpy(&uA, DAT_0081b5be, 4);
                } while (uVar8 < uA);
            }
            __builtin_memcpy(&uA, DAT_0081b5be, 4);
            FUN_004cb830(*(int *)(param_2 + 8), auBuf, uA * 8 + 0x10);
            return;
        }
    }
    else {
        if (param_1 == 5) {
            __builtin_memcpy(&uA, DAT_0078072e, 4);
            if (uA == 0) {
                __builtin_memcpy(&iA, DAT_00780726, 4);
                if ((*(int *)(param_2 + 4) == 1) && (*(int *)(param_2 + 8) == iA)) {
                    uA = 0x46;
                    __builtin_memcpy(auBuf, &uA, 4);
                    __builtin_memcpy(&iB, DAT_00780c7a, 4);
                    FUN_004cb830(iB, auBuf, 4);
                }
            }
            else if (*(int *)(param_2 + 4) == 1) {
                uVar8 = 0;
                __builtin_memcpy(&uA, DAT_00780732, 4);
                if (uA != 0) {
                    piVar9 = DAT_0078077a;
                    do {
                        __builtin_memcpy(&iA, piVar9, 4);
                        if (iA == *(int *)(param_2 + 8)) {
                            __builtin_memcpy(&iB, DAT_0078077a + uVar8 * 0x28 + 4, 4);
                            if (iB != 0) {
                                uA = 1;
                                __builtin_memcpy(DAT_00780736, &uA, 4);
                            }
                            break;
                        }
                        piVar9 = piVar9 + 0x28;
                        uVar8 = uVar8 + 1;
                        __builtin_memcpy(&uA, DAT_00780732, 4);
                    } while (uVar8 < uA);
                }
                FUN_004cddb0(*(int *)(param_2 + 8));
                uVar8 = 0;
                __builtin_memcpy(&uA, DAT_0081b5be, 4);
                if (uA != 0) {
                    piVar9 = DAT_00815ac6;
LAB_004ccbab:
                    __builtin_memcpy(&iA, piVar9, 4);
                    if (iA != *(int *)(param_2 + 8)) goto code_r0x004ccbb1;
                    __builtin_memcpy(&uCnt, DAT_0081b5be, 4);
                    uCnt = uCnt - 1;
                    __builtin_memcpy(DAT_0081b5be, &uCnt, 4);
                    puVar2 = DAT_00815ac6 + uCnt * 0x238;
                    puVar3 = DAT_00815ac6 + uVar8 * 0x238;
                    for (iVar6 = 0x8e; iVar6 != 0; iVar6 = iVar6 + -1) {
                        __builtin_memcpy(&uB, puVar2, 4);
                        __builtin_memcpy(puVar3, &uB, 4);
                        puVar2 = puVar2 + 4;
                        puVar3 = puVar3 + 4;
                    }
                    uB = 0;
                    __builtin_memcpy(&uCnt, DAT_0081b5be, 4);
                    pbEnt = DAT_00815ac6 + uCnt * 0x238;
                    __builtin_memcpy(pbEnt + 0x00, &uB, 4);
                    uB = 0x7f;
                    __builtin_memcpy(&uCnt, DAT_0081b5be, 4);
                    pbEnt = DAT_00815ac6 + uCnt * 0x238;
                    __builtin_memcpy(pbEnt + 0x04, &uB, 4);
                    __builtin_memcpy(&uCnt, DAT_0081b5be, 4);
                    pbEnt = DAT_00815ac6 + uCnt * 0x238;
                    __builtin_memcpy(pbEnt + 0x0c, &uB, 4);
                    __builtin_memcpy(&uCnt, DAT_0081b5be, 4);
                    pbEnt = DAT_00815ac6 + uCnt * 0x238;
                    __builtin_memcpy(pbEnt + 0x14, &uB, 4);
                    __builtin_memcpy(&uCnt, DAT_0081b5be, 4);
                    pbEnt = DAT_00815ac6 + uCnt * 0x238;
                    __builtin_memcpy(pbEnt + 0x08, &uB, 4);
                    __builtin_memcpy(&uCnt, DAT_0081b5be, 4);
                    pbEnt = DAT_00815ac6 + uCnt * 0x238;
                    __builtin_memcpy(pbEnt + 0x10, &uB, 4);
                    __builtin_memcpy(&uCnt, DAT_0081b5be, 4);
                    pbEnt = DAT_00815ac6 + uCnt * 0x238;
                    __builtin_memcpy(pbEnt + 0x18, &uB, 4);
                    uB = 0;
                    __builtin_memcpy(&uCnt, DAT_0081b5be, 4);
                    pbEnt = DAT_00815ac6 + uCnt * 0x238;
                    __builtin_memcpy(pbEnt + 0x1c, &uB, 4);
                    __builtin_memcpy(&uCnt, DAT_0081b5be, 4);
                    DAT_00815ac6[uCnt * 0x238 + 0x20] = 0x39;
                    uB = 1;
                    __builtin_memcpy(&uCnt, DAT_0081b5be, 4);
                    pbEnt = DAT_00815ac6 + uCnt * 0x238;
                    __builtin_memcpy(pbEnt + 0x220, &uB, 4);
                    uB = 0;
                    __builtin_memcpy(&uCnt, DAT_0081b5be, 4);
                    pbEnt = DAT_00815ac6 + uCnt * 0x238;
                    __builtin_memcpy(pbEnt + 0x234, &uB, 4);
                    __builtin_memcpy(&uCnt, DAT_0081b5be, 4);
                    pbEnt = DAT_00815ac6 + uCnt * 0x238;
                    __builtin_memcpy(pbEnt + 0x224, &uB, 4);
                    __builtin_memcpy(&uCnt, DAT_0081b5be, 4);
                    pbEnt = DAT_00815ac6 + uCnt * 0x238;
                    __builtin_memcpy(pbEnt + 0x228, &uB, 4);
                    __builtin_memcpy(&uCnt, DAT_0081b5be, 4);
                    pbEnt = DAT_00815ac6 + uCnt * 0x238;
                    __builtin_memcpy(pbEnt + 0x22c, &uB, 4);
                    __builtin_memcpy(&uCnt, DAT_0081b5be, 4);
                    pbEnt = DAT_00815ac6 + uCnt * 0x238;
                    __builtin_memcpy(pbEnt + 0x230, &uB, 4);
                }
LAB_004ccd58:
                iVar6 = 0;
                piTbl = DAT_0064f070;
                do {
                    if (*piTbl == *(int *)(param_2 + 8)) {
                        DAT_0064f070[iVar6 * 6 + 0] = 0;
                        DAT_0064f070[iVar6 * 6 + 1] = 0;
                        DAT_0064f070[iVar6 * 6 + 2] = 0;
                        DAT_0064f070[iVar6 * 6 + 3] = 0;
                        DAT_0064f070[iVar6 * 6 + 4] = 0;
                        DAT_0064f070[iVar6 * 6 + 5] = 0;
                        break;
                    }
                    piTbl = piTbl + 6;
                    iVar6 = iVar6 + 1;
                } while (piTbl < DAT_0064f430);
            }
            uA = 1;
            __builtin_memcpy(DAT_00780732, &uA, 4);
            FUN_004ca450();
            return;
        }
        if (param_1 != 0x103) {
            return;
        }
        if (*(int *)(param_2 + 4) == 1) {
            uVar8 = 0;
            __builtin_memcpy(&uA, DAT_00780732, 4);
            if (uA != 0) {
                pcVar7 = (char *)DAT_0078076a;
                do {
                    __builtin_memcpy(&iA, pcVar7 + 0x10, 4);
                    if (iA == *(int *)(param_2 + 8)) {
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
                            __builtin_memcpy(&uB, pcVar10, 4);
                            __builtin_memcpy(pcVar11, &uB, 4);
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
                        pcVar10 = (char *)(~uVar4 + 0x1c + param_2);
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
                            __builtin_memcpy(&uB, pcVar10, 4);
                            __builtin_memcpy(pcVar11, &uB, 4);
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
                    __builtin_memcpy(&uA, DAT_00780732, 4);
                } while (uVar8 < uA);
            }
        }
    }
    return;
code_r0x004ccbb1:
    piVar9 = piVar9 + 0x238;
    uVar8 = uVar8 + 1;
    __builtin_memcpy(&uA, DAT_0081b5be, 4);
    if (uA <= uVar8) goto LAB_004ccd58;
    goto LAB_004ccbab;
}

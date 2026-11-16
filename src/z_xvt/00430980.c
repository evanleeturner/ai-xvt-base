/* SPDX-License-Identifier: MIT */
/* z_xvt 00430980 */
/* Prototypes */
unsigned long __stdcall timeGetTime(void);
int __cdecl FUN_00431110(int param_1, unsigned int *param_2, unsigned int param_3);
int __cdecl FUN_00438a10(void);

typedef int (__stdcall *FUN_00430980_Slot0c)(int *, int, int);
typedef int (__stdcall *FUN_00430980_Slot1c)(int *, int, int);
typedef int (__stdcall *FUN_00430980_Slot30)(int *, int, void *, int, int);

extern int DAT_007330a4;
extern char DAT_0071e6c8;
extern int DAT_007330f8;
extern int DAT_00739290;
extern int DAT_007330ac;
extern int DAT_007330a0;
extern int *DAT_00733074;
extern unsigned int DAT_00539800[];
extern int DAT_00733798[];
extern int DAT_00733120[];
extern char DAT_00733100[];
extern char LAB_0042fe30[];

void __cdecl FUN_00430980(int param_1, int param_2)
{
    char cVar1;
    int iVar3;
    int iVar5;
    unsigned int uVar6;
    unsigned int uVar7;
    int *piVar8;
    int *puVar2;
    int *puVar4;
    char *pcVar9;
    char *pcVar10;
    int *pVtbl;
    FUN_00430980_Slot0c pfnSlot0c;
    FUN_00430980_Slot1c pfnSlot1c;
    FUN_00430980_Slot30 pfnSlot30;

    if (param_1 == 3) {
        iVar3 = 0;
        if (DAT_007330a4 != 0) {
            if (DAT_0071e6c8 == '\0') {
                if (*(int *)(param_2 + 4) == 1) {
                    if (*(int *)(param_2 + 8) != DAT_007330f8) {
                        iVar3 = 0;
                        DAT_00539800[1] = 0;
                        DAT_00539800[2] = DAT_00739290;
                        DAT_00539800[0] = 0x3b;
                        DAT_00539800[3] = timeGetTime();
                        if (0 < DAT_00739290) {
                            puVar2 = (int *)&DAT_00539800[4];
                            puVar4 = &DAT_00733798[0];
                            do {
                                iVar3 = (int)((unsigned int)iVar3 + 1u);
                                puVar2[0] = puVar4[0];
                                *(char *)(puVar2 + 1) = *(char *)(puVar4 + 3);
                                *((char *)puVar2 + 5) = *(char *)(puVar4 + 5);
                                *((char *)puVar2 + 6) = *(char *)(puVar4 + 4);
                                *((char *)puVar2 + 7) = *(char *)(puVar4 + 6);
                                puVar2 = puVar2 + 2;
                                puVar4 = puVar4 + 0x8e;
                            } while (iVar3 < DAT_00739290);
                        }
                        FUN_00431110(*(int *)(param_2 + 8), &DAT_00539800[0], (unsigned int)DAT_00739290 * 8u + 0x10u);
                        DAT_00539800[0] = 0x3a;
                        DAT_00539800[1] = 0;
                        FUN_00431110(*(int *)(param_2 + 8), &DAT_00539800[0], 8);
                    }
                    iVar3 = 0;
                    DAT_007330ac = 0;
                    pVtbl = (int *)*DAT_00733074;
                    pfnSlot30 = (FUN_00430980_Slot30)pVtbl[0x30 / 4];
                    pfnSlot30(DAT_00733074, 0, (void *)LAB_0042fe30, 0, 0);
                    if (0 < DAT_007330ac) {
                        piVar8 = &DAT_00733120[0];
                        do {
                            if (DAT_007330f8 != *piVar8) {
                                pVtbl = (int *)*DAT_00733074;
                                pfnSlot0c = (FUN_00430980_Slot0c)pVtbl[0xc / 4];
                                pfnSlot0c(DAT_00733074, DAT_007330a0, *piVar8);
                            }
                            piVar8 = piVar8 + 10;
                            iVar3 = (int)((unsigned int)iVar3 + 1u);
                        } while (iVar3 < DAT_007330ac);
                        return;
                    }
                }
            }
            else if ((*(int *)(param_2 + 4) == 1) && (*(int *)(param_2 + 8) != DAT_007330f8)) {
                DAT_00539800[1] = 0;
                DAT_00539800[0] = 0x3b;
                DAT_00539800[2] = DAT_00739290;
                DAT_00539800[3] = timeGetTime();
                if (0 < DAT_00739290) {
                    puVar2 = (int *)&DAT_00539800[4];
                    puVar4 = &DAT_00733798[0];
                    do {
                        iVar3 = (int)((unsigned int)iVar3 + 1u);
                        puVar2[0] = puVar4[0];
                        *(char *)(puVar2 + 1) = *(char *)(puVar4 + 3);
                        *((char *)puVar2 + 5) = *(char *)(puVar4 + 5);
                        *((char *)puVar2 + 6) = *(char *)(puVar4 + 4);
                        *((char *)puVar2 + 7) = *(char *)(puVar4 + 6);
                        puVar2 = puVar2 + 2;
                        puVar4 = puVar4 + 0x8e;
                    } while (iVar3 < DAT_00739290);
                }
                FUN_00431110(*(int *)(param_2 + 8), &DAT_00539800[0], (unsigned int)DAT_00739290 * 8u + 0x10u);
                DAT_00539800[0] = 0x3a;
                DAT_00539800[1] = FUN_00438a10();
                DAT_00539800[2] = 2;
                DAT_00539800[3] = 0;
                FUN_00431110(*(int *)(param_2 + 8), &DAT_00539800[0], 0x10);
                return;
            }
        }
    }
    else if (param_1 == 5) {
        if (DAT_007330a4 != 0) {
            if (DAT_0071e6c8 == '\0') {
                if (*(int *)(param_2 + 4) == 1) {
                    iVar3 = 0;
                    DAT_007330ac = 0;
                    pVtbl = (int *)*DAT_00733074;
                    pfnSlot30 = (FUN_00430980_Slot30)pVtbl[0x30 / 4];
                    pfnSlot30(DAT_00733074, 0, (void *)LAB_0042fe30, 0, 0);
                    pVtbl = (int *)*DAT_00733074;
                    pfnSlot1c = (FUN_00430980_Slot1c)pVtbl[0x1c / 4];
                    pfnSlot1c(DAT_00733074, DAT_007330a0, *(int *)(param_2 + 8));
                    if (0 < DAT_00739290) {
                        piVar8 = &DAT_00733798[0];
                        do {
                            if (*piVar8 == *(int *)(param_2 + 8)) {
                                DAT_00739290 = (int)((unsigned int)DAT_00739290 - 1u);
                                puVar2 = &DAT_00733798[(int)((unsigned int)DAT_00739290 * 0x8eu)];
                                puVar4 = &DAT_00733798[(int)((unsigned int)iVar3 * 0x8eu)];
                                for (iVar5 = 0x8e; iVar5 != 0; iVar5 = (int)((unsigned int)iVar5 - 1u)) {
                                    *puVar4 = *puVar2;
                                    puVar2 = puVar2 + 1;
                                    puVar4 = puVar4 + 1;
                                }
                                DAT_00733798[(int)((unsigned int)DAT_00739290 * 0x8eu + 0u)] = 0;
                                DAT_00733798[(int)((unsigned int)DAT_00739290 * 0x8eu + 1u)] = 0x7f;
                                DAT_00733798[(int)((unsigned int)DAT_00739290 * 0x8eu + 3u)] = 0x7f;
                                DAT_00733798[(int)((unsigned int)DAT_00739290 * 0x8eu + 5u)] = 0x7f;
                                DAT_00733798[(int)((unsigned int)DAT_00739290 * 0x8eu + 2u)] = 0x7f;
                                DAT_00733798[(int)((unsigned int)DAT_00739290 * 0x8eu + 4u)] = 0x7f;
                                DAT_00733798[(int)((unsigned int)DAT_00739290 * 0x8eu + 6u)] = 0x7f;
                                DAT_00733798[(int)((unsigned int)DAT_00739290 * 0x8eu + 7u)] = 0;
                                *((char *)DAT_00733798 + (int)(0x20u + (unsigned int)DAT_00739290 * 0x238u)) = 0x39;
                                DAT_00733798[(int)((unsigned int)DAT_00739290 * 0x8eu + 0x88u)] = 1;
                                DAT_00733798[(int)((unsigned int)DAT_00739290 * 0x8eu + 0x89u)] = 0;
                                DAT_00733798[(int)((unsigned int)DAT_00739290 * 0x8eu + 0x8au)] = 0;
                                DAT_00733798[(int)((unsigned int)DAT_00739290 * 0x8eu + 0x8bu)] = 0;
                                return;
                            }
                            piVar8 = piVar8 + 0x8e;
                            iVar3 = (int)((unsigned int)iVar3 + 1u);
                        } while (iVar3 < DAT_00739290);
                        return;
                    }
                }
            }
            else if (*(int *)(param_2 + 4) == 1) {
                iVar3 = 0;
                if (0 < DAT_007330ac) {
                    piVar8 = &DAT_00733120[0];
                    do {
                        if (*piVar8 == *(int *)(param_2 + 8)) {
                            if (DAT_00733120[(int)(1u + (unsigned int)iVar3 * 10u)] != 0) {
                                DAT_00539800[0] = 0xd;
                                FUN_00431110(DAT_007330f8, &DAT_00539800[0], 4);
                            }
                            pVtbl = (int *)*DAT_00733074;
                            pfnSlot1c = (FUN_00430980_Slot1c)pVtbl[0x1c / 4];
                            pfnSlot1c(DAT_00733074, DAT_007330a0, *(int *)(param_2 + 8));
                            DAT_00733120[(int)(1u + (unsigned int)iVar3 * 10u)] = 0;
                            break;
                        }
                        piVar8 = piVar8 + 10;
                        iVar3 = (int)((unsigned int)iVar3 + 1u);
                    } while (iVar3 < DAT_007330ac);
                }
                iVar3 = 0;
                if (0 < DAT_00739290) {
                    piVar8 = &DAT_00733798[0];
                    do {
                        if (*piVar8 == *(int *)(param_2 + 8)) {
                            DAT_00739290 = (int)((unsigned int)DAT_00739290 - 1u);
                            puVar2 = &DAT_00733798[(int)((unsigned int)DAT_00739290 * 0x8eu)];
                            puVar4 = &DAT_00733798[(int)((unsigned int)iVar3 * 0x8eu)];
                            for (iVar5 = 0x8e; iVar5 != 0; iVar5 = (int)((unsigned int)iVar5 - 1u)) {
                                *puVar4 = *puVar2;
                                puVar2 = puVar2 + 1;
                                puVar4 = puVar4 + 1;
                            }
                            DAT_00733798[(int)((unsigned int)DAT_00739290 * 0x8eu + 0u)] = 0;
                            DAT_00733798[(int)((unsigned int)DAT_00739290 * 0x8eu + 1u)] = 0x7f;
                            DAT_00733798[(int)((unsigned int)DAT_00739290 * 0x8eu + 3u)] = 0x7f;
                            DAT_00733798[(int)((unsigned int)DAT_00739290 * 0x8eu + 5u)] = 0x7f;
                            DAT_00733798[(int)((unsigned int)DAT_00739290 * 0x8eu + 2u)] = 0x7f;
                            DAT_00733798[(int)((unsigned int)DAT_00739290 * 0x8eu + 4u)] = 0x7f;
                            DAT_00733798[(int)((unsigned int)DAT_00739290 * 0x8eu + 6u)] = 0x7f;
                            DAT_00733798[(int)((unsigned int)DAT_00739290 * 0x8eu + 7u)] = 0;
                            *((char *)DAT_00733798 + (int)(0x20u + (unsigned int)DAT_00739290 * 0x238u)) = 0x39;
                            DAT_00733798[(int)((unsigned int)DAT_00739290 * 0x8eu + 0x88u)] = 1;
                            DAT_00733798[(int)((unsigned int)DAT_00739290 * 0x8eu + 0x89u)] = 0;
                            DAT_00733798[(int)((unsigned int)DAT_00739290 * 0x8eu + 0x8au)] = 0;
                            DAT_00733798[(int)((unsigned int)DAT_00739290 * 0x8eu + 0x8bu)] = 0;
                            return;
                        }
                        piVar8 = piVar8 + 0x8e;
                        iVar3 = (int)((unsigned int)iVar3 + 1u);
                    } while (iVar3 < DAT_00739290);
                    return;
                }
            }
        }
    }
    else {
        if (param_1 != 0x103) {
            return;
        }
        if ((*(int *)(param_2 + 4) == 1) && (iVar3 = 0, 0 < DAT_007330ac)) {
            piVar8 = &DAT_00733120[0];
            while (*piVar8 != *(int *)(param_2 + 8)) {
                piVar8 = piVar8 + 10;
                iVar3 = (int)((unsigned int)iVar3 + 1u);
                if (DAT_007330ac <= iVar3) {
                    return;
                }
            }
            uVar6 = 0xffffffff;
            pcVar9 = (char *)(param_2 + 0x1c);
            do {
                pcVar10 = pcVar9;
                if (uVar6 == 0) break;
                uVar6 = uVar6 - 1;
                pcVar10 = pcVar9 + 1;
                cVar1 = *pcVar9;
                pcVar9 = pcVar10;
            } while (cVar1 != '\0');
            uVar6 = ~uVar6;
            pcVar9 = pcVar10 - uVar6;
            pcVar10 = &DAT_00733100[(int)(0x10u + (unsigned int)iVar3 * 0x28u)];
            for (uVar7 = uVar6 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
                *(unsigned int *)pcVar10 = *(unsigned int *)pcVar9;
                pcVar9 = pcVar9 + 4;
                pcVar10 = pcVar10 + 4;
            }
            for (uVar6 = uVar6 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
                *pcVar10 = *pcVar9;
                pcVar9 = pcVar9 + 1;
                pcVar10 = pcVar10 + 1;
            }
            uVar6 = 0xffffffff;
            pcVar9 = &DAT_00733100[(int)(0x10u + (unsigned int)iVar3 * 0x28u)];
            do {
                if (uVar6 == 0) break;
                uVar6 = uVar6 - 1;
                cVar1 = *pcVar9;
                pcVar9 = pcVar9 + 1;
            } while (cVar1 != '\0');
            uVar7 = 0xffffffff;
            pcVar9 = (char *)(~uVar6 + 0x1c + param_2);
            do {
                pcVar10 = pcVar9;
                if (uVar7 == 0) break;
                uVar7 = uVar7 - 1;
                pcVar10 = pcVar9 + 1;
                cVar1 = *pcVar9;
                pcVar9 = pcVar10;
            } while (cVar1 != '\0');
            uVar7 = ~uVar7;
            pcVar9 = pcVar10 - uVar7;
            pcVar10 = &DAT_00733100[(int)((unsigned int)iVar3 * 0x28u)];
            for (uVar6 = uVar7 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
                *(unsigned int *)pcVar10 = *(unsigned int *)pcVar9;
                pcVar9 = pcVar9 + 4;
                pcVar10 = pcVar10 + 4;
            }
            for (uVar7 = uVar7 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
                *pcVar10 = *pcVar9;
                pcVar9 = pcVar9 + 1;
                pcVar10 = pcVar10 + 1;
            }
            DAT_00733100[(int)((unsigned int)iVar3 * 0x28u + 0x1cu)] = 0;
            DAT_00733100[(int)((unsigned int)iVar3 * 0x28u + 0xcu)] = 0;
        }
    }
    return;
}

/* SPDX-License-Identifier: MIT */
/* z_bop 0046e080 */
/* Globals (types from decompiler evidence; layout is contiguous at these addresses) */
extern char DAT_009ec608;
extern int DAT_00999548;
extern int DAT_009994fc;
extern int DAT_0099f6e0;
extern unsigned int DAT_005597a0[4];   /* [0]=0x5597a0 [1]=0x5597a4 [2]=0x5597a8 [3]=0x5597ac */
extern unsigned int DAT_005597b0[];    /* 8-byte entries */
extern unsigned int DAT_00999be8[];    /* entries of 0x8e dwords (0x238 bytes) */
extern unsigned int DAT_00999550[];    /* entries of 10 dwords (0x28 bytes) */

int  __cdecl FUN_0046e040(void);
unsigned int __cdecl FUN_00448d70(void);
void __cdecl FUN_0046c730(void);
void __cdecl FUN_0046fa10(int param_1);
void __cdecl FUN_0046fa50(unsigned int param_1);
unsigned char __cdecl FUN_0046d350(int param_1, unsigned int *param_2, unsigned int param_3);
DWORD __stdcall timeGetTime(void);

void __cdecl FUN_0046e080(int param_1, int param_2)
{
    char cVar1;
    unsigned int *puVar2;
    int *piVar3;
    unsigned int *puVar4;
    int iVar5;
    int iVar6;
    unsigned int uVar7;
    unsigned int uVar8;
    char *pcVar9;
    char *pcVar10;

    if (param_1 == 3) {
        iVar6 = FUN_0046e040();
        if (iVar6 != 0) {
            iVar6 = 0;
            if (DAT_009ec608 == '\0') {
                if (*(int *)(param_2 + 4) == 1) {
                    if (*(int *)(param_2 + 8) != DAT_00999548) {
                        iVar6 = 0;
                        DAT_005597a0[1] = 0;
                        DAT_005597a0[2] = DAT_0099f6e0;
                        DAT_005597a0[0] = 0x3b;
                        DAT_005597a0[3] = timeGetTime();
                        if (0 < DAT_0099f6e0) {
                            puVar2 = DAT_00999be8;
                            puVar4 = DAT_005597b0;
                            do {
                                iVar6 = iVar6 + 1;
                                *puVar4 = *puVar2;
                                *(unsigned char *)(puVar4 + 1) = *(unsigned char *)(puVar2 + 3);
                                *(unsigned char *)((int)puVar4 + 5) = *(unsigned char *)(puVar2 + 5);
                                *(unsigned char *)((int)puVar4 + 6) = *(unsigned char *)(puVar2 + 4);
                                *(unsigned char *)((int)puVar4 + 7) = *(unsigned char *)(puVar2 + 6);
                                puVar2 = puVar2 + 0x8e;
                                puVar4 = puVar4 + 2;
                            } while (iVar6 < DAT_0099f6e0);
                        }
                        FUN_0046d350(*(int *)(param_2 + 8), DAT_005597a0, DAT_0099f6e0 * 8 + 0x10);
                        DAT_005597a0[0] = 0x3a;
                        DAT_005597a0[1] = 0;
                        FUN_0046d350(*(int *)(param_2 + 8), DAT_005597a0, 8);
                    }
                    iVar6 = 0;
                    DAT_009994fc = 0;
                    FUN_0046c730();
                    if (0 < DAT_009994fc) {
                        piVar3 = (int *)&DAT_00999550[8];
                        do {
                            if (*piVar3 != DAT_00999548) {
                                FUN_0046fa10((int)(piVar3 + -8));
                            }
                            piVar3 = piVar3 + 10;
                            iVar6 = iVar6 + 1;
                        } while (iVar6 < DAT_009994fc);
                        return;
                    }
                }
            }
            else if ((*(int *)(param_2 + 4) == 1) && (*(int *)(param_2 + 8) != DAT_00999548)) {
                DAT_005597a0[1] = 0;
                DAT_005597a0[0] = 0x3b;
                DAT_005597a0[2] = DAT_0099f6e0;
                DAT_005597a0[3] = timeGetTime();
                if (0 < DAT_0099f6e0) {
                    puVar2 = DAT_00999be8;
                    puVar4 = DAT_005597b0;
                    do {
                        iVar6 = iVar6 + 1;
                        *puVar4 = *puVar2;
                        *(unsigned char *)(puVar4 + 1) = *(unsigned char *)(puVar2 + 3);
                        *(unsigned char *)((int)puVar4 + 5) = *(unsigned char *)(puVar2 + 5);
                        *(unsigned char *)((int)puVar4 + 6) = *(unsigned char *)(puVar2 + 4);
                        *(unsigned char *)((int)puVar4 + 7) = *(unsigned char *)(puVar2 + 6);
                        puVar2 = puVar2 + 0x8e;
                        puVar4 = puVar4 + 2;
                    } while (iVar6 < DAT_0099f6e0);
                }
                FUN_0046d350(*(int *)(param_2 + 8), DAT_005597a0, DAT_0099f6e0 * 8 + 0x10);
                DAT_005597a0[0] = 0x3a;
                DAT_005597a0[1] = FUN_00448d70();
                DAT_005597a0[2] = 0x65;
                DAT_005597a0[3] = 0;
                FUN_0046d350(*(int *)(param_2 + 8), DAT_005597a0, 0x10);
                return;
            }
        }
    }
    else if (param_1 == 5) {
        iVar6 = FUN_0046e040();
        if (iVar6 != 0) {
            if (DAT_009ec608 == '\0') {
                if (*(int *)(param_2 + 4) == 1) {
                    iVar6 = 0;
                    DAT_009994fc = 0;
                    FUN_0046c730();
                    FUN_0046fa50(*(unsigned int *)(param_2 + 8));
                    if (0 < DAT_0099f6e0) {
                        piVar3 = (int *)DAT_00999be8;
                        do {
                            if (*piVar3 == *(int *)(param_2 + 8)) {
                                DAT_0099f6e0 = DAT_0099f6e0 + -1;
                                puVar2 = DAT_00999be8 + DAT_0099f6e0 * 0x8e;
                                puVar4 = DAT_00999be8 + iVar6 * 0x8e;
                                for (iVar5 = 0x8e; iVar5 != 0; iVar5 = iVar5 + -1) {
                                    *puVar4 = *puVar2;
                                    puVar2 = puVar2 + 1;
                                    puVar4 = puVar4 + 1;
                                }
                                DAT_00999be8[DAT_0099f6e0 * 0x8e] = 0;
                                DAT_00999be8[DAT_0099f6e0 * 0x8e + 1] = 0x7f;
                                DAT_00999be8[DAT_0099f6e0 * 0x8e + 3] = 0x7f;
                                DAT_00999be8[DAT_0099f6e0 * 0x8e + 5] = 0x7f;
                                DAT_00999be8[DAT_0099f6e0 * 0x8e + 2] = 0x7f;
                                DAT_00999be8[DAT_0099f6e0 * 0x8e + 4] = 0x7f;
                                DAT_00999be8[DAT_0099f6e0 * 0x8e + 6] = 0x7f;
                                DAT_00999be8[DAT_0099f6e0 * 0x8e + 7] = 0;
                                *((unsigned char *)DAT_00999be8 + DAT_0099f6e0 * 0x238 + 0x20) = 0x39;
                                DAT_00999be8[DAT_0099f6e0 * 0x8e + 0x88] = 1;
                                DAT_00999be8[DAT_0099f6e0 * 0x8e + 0x89] = 0;
                                DAT_00999be8[DAT_0099f6e0 * 0x8e + 0x8a] = 0;
                                DAT_00999be8[DAT_0099f6e0 * 0x8e + 0x8b] = 0;
                                return;
                            }
                            piVar3 = piVar3 + 0x8e;
                            iVar6 = iVar6 + 1;
                        } while (iVar6 < DAT_0099f6e0);
                        return;
                    }
                }
            }
            else if (*(int *)(param_2 + 4) == 1) {
                iVar6 = 0;
                if (0 < DAT_009994fc) {
                    piVar3 = (int *)&DAT_00999550[8];
                    do {
                        if (*piVar3 == *(int *)(param_2 + 8)) {
                            if (DAT_00999550[9 + iVar6 * 10] != 0) {
                                DAT_005597a0[0] = 0xd;
                                FUN_0046d350(DAT_00999548, DAT_005597a0, 4);
                            }
                            FUN_0046fa50(*(unsigned int *)(param_2 + 8));
                            DAT_00999550[9 + iVar6 * 10] = 0;
                            break;
                        }
                        piVar3 = piVar3 + 10;
                        iVar6 = iVar6 + 1;
                    } while (iVar6 < DAT_009994fc);
                }
                iVar6 = 0;
                if (0 < DAT_0099f6e0) {
                    piVar3 = (int *)DAT_00999be8;
                    do {
                        if (*piVar3 == *(int *)(param_2 + 8)) {
                            DAT_0099f6e0 = DAT_0099f6e0 + -1;
                            puVar2 = DAT_00999be8 + DAT_0099f6e0 * 0x8e;
                            puVar4 = DAT_00999be8 + iVar6 * 0x8e;
                            for (iVar5 = 0x8e; iVar5 != 0; iVar5 = iVar5 + -1) {
                                *puVar4 = *puVar2;
                                puVar2 = puVar2 + 1;
                                puVar4 = puVar4 + 1;
                            }
                            DAT_00999be8[DAT_0099f6e0 * 0x8e] = 0;
                            DAT_00999be8[DAT_0099f6e0 * 0x8e + 1] = 0x7f;
                            DAT_00999be8[DAT_0099f6e0 * 0x8e + 3] = 0x7f;
                            DAT_00999be8[DAT_0099f6e0 * 0x8e + 5] = 0x7f;
                            DAT_00999be8[DAT_0099f6e0 * 0x8e + 2] = 0x7f;
                            DAT_00999be8[DAT_0099f6e0 * 0x8e + 4] = 0x7f;
                            DAT_00999be8[DAT_0099f6e0 * 0x8e + 6] = 0x7f;
                            DAT_00999be8[DAT_0099f6e0 * 0x8e + 7] = 0;
                            *((unsigned char *)DAT_00999be8 + DAT_0099f6e0 * 0x238 + 0x20) = 0x39;
                            DAT_00999be8[DAT_0099f6e0 * 0x8e + 0x88] = 1;
                            DAT_00999be8[DAT_0099f6e0 * 0x8e + 0x89] = 0;
                            DAT_00999be8[DAT_0099f6e0 * 0x8e + 0x8a] = 0;
                            DAT_00999be8[DAT_0099f6e0 * 0x8e + 0x8b] = 0;
                            return;
                        }
                        piVar3 = piVar3 + 0x8e;
                        iVar6 = iVar6 + 1;
                    } while (iVar6 < DAT_0099f6e0);
                    return;
                }
            }
        }
    }
    else {
        if (param_1 != 0x103) {
            return;
        }
        if ((*(int *)(param_2 + 4) == 1) && (iVar6 = 0, 0 < DAT_009994fc)) {
            piVar3 = (int *)&DAT_00999550[8];
            while (*piVar3 != *(int *)(param_2 + 8)) {
                piVar3 = piVar3 + 10;
                iVar6 = iVar6 + 1;
                if (DAT_009994fc <= iVar6) {
                    return;
                }
            }
            uVar7 = 0xffffffff;
            pcVar9 = (char *)(param_2 + 0x1c);
            do {
                pcVar10 = pcVar9;
                if (uVar7 == 0) break;
                uVar7 = uVar7 - 1;
                pcVar10 = pcVar9 + 1;
                cVar1 = *pcVar9;
                pcVar9 = pcVar10;
            } while (cVar1 != '\0');
            uVar7 = ~uVar7;
            pcVar9 = pcVar10 + -uVar7;
            pcVar10 = (char *)DAT_00999550 + 0x10 + iVar6 * 0x28;
            for (uVar8 = uVar7 >> 2; uVar8 != 0; uVar8 = uVar8 - 1) {
                *(unsigned int *)pcVar10 = *(unsigned int *)pcVar9;
                pcVar9 = pcVar9 + 4;
                pcVar10 = pcVar10 + 4;
            }
            for (uVar7 = uVar7 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
                *pcVar10 = *pcVar9;
                pcVar9 = pcVar9 + 1;
                pcVar10 = pcVar10 + 1;
            }
            uVar7 = 0xffffffff;
            pcVar9 = (char *)DAT_00999550 + 0x10 + iVar6 * 0x28;
            do {
                if (uVar7 == 0) break;
                uVar7 = uVar7 - 1;
                cVar1 = *pcVar9;
                pcVar9 = pcVar9 + 1;
            } while (cVar1 != '\0');
            uVar8 = 0xffffffff;
            pcVar9 = (char *)(~uVar7 + 0x1c + param_2);
            do {
                pcVar10 = pcVar9;
                if (uVar8 == 0) break;
                uVar8 = uVar8 - 1;
                pcVar10 = pcVar9 + 1;
                cVar1 = *pcVar9;
                pcVar9 = pcVar10;
            } while (cVar1 != '\0');
            uVar8 = ~uVar8;
            pcVar9 = pcVar10 + -uVar8;
            pcVar10 = (char *)(DAT_00999550 + iVar6 * 10);
            for (uVar7 = uVar8 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
                *(unsigned int *)pcVar10 = *(unsigned int *)pcVar9;
                pcVar9 = pcVar9 + 4;
                pcVar10 = pcVar10 + 4;
            }
            for (uVar8 = uVar8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
                *pcVar10 = *pcVar9;
                pcVar9 = pcVar9 + 1;
                pcVar10 = pcVar10 + 1;
            }
            *((unsigned char *)DAT_00999550 + iVar6 * 0x28 + 0x1c) = 0;
            *((unsigned char *)DAT_00999550 + iVar6 * 0x28 + 0xc) = 0;
        }
    }
    return;
}

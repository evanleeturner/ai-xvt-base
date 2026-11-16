/* SPDX-License-Identifier: MIT */
/* z_bop 0043e790 */
typedef unsigned char byte;
typedef unsigned short ushort;
typedef unsigned int uint;

extern int DAT_009ecc34;
extern byte DAT_009e9670[];
extern byte DAT_009e968a[];
extern byte DAT_009e9698[];
extern int _DAT_00a07cec;
extern int DAT_009a1fe0;
extern int DAT_009a1fe8;
extern int DAT_009a1ff8;
extern byte DAT_009ed22b;   /* Ghidra: DAT_009ed228._3_1_ */
extern byte DAT_009ed22c;
extern byte DAT_009ed22d;
extern short _DAT_009ed22e;
extern byte DAT_009d8c84[];
extern byte DAT_0051c5b9[];
extern ushort DAT_00a08374; /* 2-byte per P-code MULTIEQUAL; signedness unverified */

uint __cdecl FUN_00435a10(byte a, byte b, byte c);
void __cdecl FUN_00403540(uint a, uint b);
/* Ghidra shows __fastcall with unused ECX/EDX; treated as cdecl with 3 stack args (unconfirmed) */
unsigned long long __cdecl FUN_00482640(ushort param_3, short param_4, int param_5);
void __cdecl FUN_00440760(int a, uint b);
void __stdcall FUN_004abe50(void);
void __cdecl FUN_0042f030(uint a);
void __stdcall FUN_004aba50(void);

void __stdcall FUN_0043e790(void)
{
    char cVar1;
    byte bVar2;
    short sVar3;
    int iVar4;
    int iVar5;
    uint uVar6;
    short sVar7;
    int iVar8;
    uint *puVar10;
    uint uVar11;
    unsigned long long uVar12;
    char *pcVar9;
    ushort local_12;
    ushort uStack_10;
    ushort uStack_e;
    int local_8;
    uint local_4;
    int iTmpA;
    int iTmpB;
    short sTmp;
    ushort wTmp;

    uStack_10 = 0;
    local_12 = 0;
    uStack_e = 0;
    __builtin_memcpy(&uVar11, DAT_009e9670 + (uint)DAT_009ecc34 * 0x5bdu, 4);
    local_4 = (uint)_DAT_00a07cec;
    if ((int)_DAT_00a07cec < DAT_009a1fe0) {
        local_8 = (int)((uint)_DAT_00a07cec * 0x23u);
        do {
            iVar8 = (int)((uint)local_8 + (uint)DAT_009a1fe8);
            if (*(char *)(iVar8 + 3) != '\0') {
                __builtin_memcpy(&iTmpA, (byte *)iVar8 + 0x1f, 4);
                if (*(char *)iTmpA == '\0') {
                    __builtin_memcpy(&iVar4, (byte *)iTmpA + 0xa9, 4);
                    __builtin_memcpy(&sTmp, (byte *)iVar4 + 0x1e5, 2);
                    if (sTmp != 0 && *(char *)(iVar4 + 7) == '\0') {
                        __builtin_memcpy(&iTmpB, (byte *)iVar8 + 0x1b, 4);
                        if (iTmpB == -1) {
                            if (*(ushort *)(iVar4 + 0x42) == uVar11 &&
                                (*(char *)(iVar4 + 0x5e) == '\f' || *(char *)(iVar4 + 0x5e) == '\x17')) {
                                iVar8 = 0;
                                if (*(byte *)(iVar4 + 0x21b) != 0) {
                                    do {
                                        cVar1 = *(char *)((uint)iVar8 + 0x21eu + (uint)iVar4);
                                        if ((cVar1 == -0x75 || cVar1 == -0x77) &&
                                            *(char *)((uint)iVar8 + 0x220u + (uint)iVar4) != '\0') {
                                            uStack_10 = 1;
                                        }
                                        iVar8 = (int)((uint)iVar8 + 1u);
                                    } while (iVar8 < (int)(uint)*(byte *)(iVar4 + 0x21b));
                                }
                                if (*(char *)(iVar4 + 0x241) != '\0') {
                                    __builtin_memcpy(&sTmp, (byte *)iVar4 + 0x23f, 2);
                                    if (sTmp != 0) {
                                        bVar2 = *(byte *)(iVar4 + 0x23d);
                                        if (bVar2 != 0 && bVar2 < 3) {
                                            uStack_e = (ushort)bVar2;
                                        }
                                    }
                                }
                            }
                        }
                        else {
                            __builtin_memcpy(&iTmpA, (byte *)(uVar11 * 0x23u + 0x1fu + (uint)DAT_009a1fe8), 4);
                            __builtin_memcpy(&iVar8, (byte *)iTmpA + 0xa9, 4);
                            if (*(ushort *)(iVar8 + 0x6a) == local_4) {
                                uVar6 = FUN_00435a10(DAT_009ed22b, DAT_009ed22c, DAT_009ed22d);
                                if ((int)((uVar6 & 0xffffu) - (uint)*(ushort *)(iVar8 + 0x6c)) < 5) {
                                    uStack_10 = 1;
                                }
                            }
                            __builtin_memcpy(&iVar8, (byte *)((uint)DAT_009a1fe8 + 0x1bu + (uint)local_8), 4);
                            iVar5 = (int)((uint)iVar8 * 0x5bdu);
                            __builtin_memcpy(&wTmp, DAT_009e968a + (uint)iVar5, 2);
                            if (wTmp == uVar11 && DAT_009e9698[(uint)iVar5] == 0) {
                                FUN_00403540(local_4, uVar11);
                                if (DAT_009a1ff8 < 0x10000) {
                                    uVar12 = FUN_00482640((ushort)uVar11, 0, iVar8);
                                    if ((short)uVar12 != 0) {
                                        uStack_10 = 1;
                                    }
                                }
                            }
                            __builtin_memcpy(&wTmp, DAT_009e968a + (uint)iVar5, 2);
                            if (wTmp == uVar11 && *(char *)(iVar4 + 0x241) != '\0') {
                                __builtin_memcpy(&sTmp, (byte *)iVar4 + 0x23f, 2);
                                if (sTmp != 0) {
                                    bVar2 = *(byte *)(iVar4 + 0x23d);
                                    if (bVar2 != 0 && bVar2 < 3) {
                                        uStack_e = (ushort)bVar2;
                                    }
                                }
                            }
                        }
                        if (DAT_009d8c84[(uint)*(byte *)((uint)DAT_009a1fe8 + 0x16u + (uint)local_8) * 0x566u] != 5) {
                            iVar8 = 0;
                            if (*(byte *)(iVar4 + 0x21d) != 0) {
                                pcVar9 = (char *)(iVar4 + 0x370);
                                do {
                                    if (*pcVar9 == '\x02' && *(ushort *)(pcVar9 + 0x42) == uVar11 &&
                                        *(char *)((uint)DAT_0051c5b9[(uint)iVar8 * 8u + (uint)*(byte *)(iVar4 + 4) * 0x101u]
                                                  + 0x33cu + (uint)iVar4) != '\0') {
                                        local_12 = 1;
                                    }
                                    pcVar9 = pcVar9 + 4;
                                    iVar8 = (int)((uint)iVar8 + 1u);
                                } while (iVar8 < (int)(uint)*(byte *)(iVar4 + 0x21d));
                            }
                            if (*(char *)(iVar4 + 0x241) != '\0') {
                                __builtin_memcpy(&sTmp, (byte *)iVar4 + 0x23f, 2);
                                if (sTmp != 0 && *(ushort *)(iVar4 + 0x244) == uVar11) {
                                    bVar2 = *(byte *)(iVar4 + 0x23d);
                                    if (bVar2 != 0 && bVar2 < 3) {
                                        uStack_e = (ushort)bVar2;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            local_8 = (int)((uint)local_8 + 0x23u);
            local_4 = local_4 + 1;
        } while ((int)local_4 < DAT_009a1fe0);
    }
    FUN_00440760((int)DAT_00a08374 + 0x5a, (uint)uStack_10);
    FUN_00440760((int)DAT_00a08374 + 0x5b, (uint)local_12);
    FUN_00440760((int)DAT_00a08374 + 0x5c, (uint)uStack_e);
    sVar7 = 0;
    if ((int)_DAT_00a07cec < DAT_009a1fe0) {
        iVar8 = (int)((uint)DAT_009a1fe0 - (uint)_DAT_00a07cec);
        puVar10 = (uint *)((uint)_DAT_00a07cec * 0x23u + 0x1fu + (uint)DAT_009a1fe8);
        do {
            if (*((byte *)puVar10 - 28) != 0) {
                __builtin_memcpy(&iTmpA, puVar10, 4);
                if (*(char *)iTmpA == '\0') {
                    __builtin_memcpy(&iVar4, (byte *)iTmpA + 0xa9, 4);
                    __builtin_memcpy(&sTmp, (byte *)iVar4 + 0x1e5, 2);
                    if (sTmp != 0 && *(char *)(iVar4 + 7) == '\0') {
                        __builtin_memcpy(&iTmpB, (byte *)puVar10 - 4, 4);
                        if (iTmpB == -1) {
                            if (*(ushort *)(iVar4 + 0x42) == uVar11 && *(char *)(iVar4 + 0x5e) == '\x17') {
                                __builtin_memcpy(&sVar3, (byte *)iVar4 + 0x23b, 2);
                                if (sVar7 < sVar3) {
                                    sVar7 = sVar3;
                                }
                            }
                        }
                        else {
                            iVar5 = (int)((uint)iTmpB * 0x5bdu);
                            __builtin_memcpy(&wTmp, DAT_009e968a + (uint)iVar5, 2);
                            if (wTmp == uVar11 && DAT_009e9698[(uint)iVar5] != 0) {
                                __builtin_memcpy(&sVar3, (byte *)iVar4 + 0x23b, 2);
                                if (sVar7 < sVar3) {
                                    sVar7 = sVar3;
                                }
                            }
                        }
                    }
                }
            }
            puVar10 = (uint *)((byte *)puVar10 + 0x23);
            iVar8 = iVar8 + -1;
        } while (iVar8 != 0);
    }
    if (sVar7 < 0x3b1) {
        if (sVar7 < 1) {
            uVar11 = 0;
        }
        else {
            uVar11 = (uint)((int)_DAT_009ed22e / 0x3b) & 1u;
        }
    }
    else {
        uVar11 = 2;
    }
    FUN_00440760((int)DAT_00a08374 + 0x5d, uVar11);
    FUN_004abe50();
    FUN_0042f030(uVar11);
    FUN_004aba50();
    return;
}

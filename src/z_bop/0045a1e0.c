/* SPDX-License-Identifier: MIT */
/* z_bop 0045a1e0 */
extern int DAT_009a1fe0;
extern int DAT_009a1fe8;
extern int _DAT_00a07cec;
extern int DAT_009ecc34;

extern unsigned char DAT_009d9192[];
extern unsigned char DAT_0051c5a6[];
extern unsigned char DAT_0051c5a0[];
extern unsigned char DAT_0051c573[];
extern unsigned char DAT_0051c635[];
extern unsigned char DAT_0051c633[];
extern unsigned char DAT_0051a1d0[];
extern unsigned char DAT_009e96b0[];
extern unsigned char DAT_009e96b1[];
extern unsigned char DAT_009e96b2[];
extern unsigned char DAT_009e96b3[];
extern unsigned char DAT_009e96b4[];
extern unsigned char DAT_009e96b5[];
extern unsigned char DAT_009e96ad[];
extern unsigned char DAT_009e96ae[];
extern unsigned char DAT_009e96af[];
extern unsigned char DAT_009e96ab[];
extern unsigned char DAT_009e9670[];
extern unsigned char DAT_009e9674[];
extern unsigned char DAT_009e9681[];
extern unsigned char DAT_009e9684[];
extern unsigned char DAT_009e9689[];
extern unsigned char DAT_009e968a[];
extern unsigned char DAT_009e968c[];
extern unsigned char DAT_009e968e[];
extern unsigned char DAT_009e9692[];
extern unsigned char DAT_009e9696[];
extern unsigned char DAT_009e9697[];
extern unsigned char DAT_009e9698[];
extern unsigned char DAT_009e9699[];
extern unsigned char DAT_009e969b[];
extern unsigned char DAT_009e969d[];
extern unsigned char DAT_009e9c1d[];
extern unsigned char DAT_009e9c21[];
extern unsigned char DAT_009e96bb[];
extern unsigned char DAT_009e96bd[];
extern unsigned char DAT_009e96bf[];
extern unsigned char DAT_009e96c1[];
extern unsigned char DAT_009e96c3[];
extern unsigned char DAT_009e96c5[];
extern unsigned char DAT_009e96c9[];
extern unsigned char DAT_009e96cd[];
extern unsigned char DAT_009e96d1[];
extern unsigned char DAT_009e96d5[];
extern unsigned char DAT_009e96d9[];
extern unsigned char DAT_009e9a79[];
extern unsigned char DAT_009e9a73[];
extern unsigned char DAT_009e9a75[];
extern unsigned char DAT_009e9a84[];
extern unsigned char DAT_009e9a86[];
extern unsigned char DAT_009e9a80[];
extern unsigned char DAT_009e9a8a[];
extern unsigned char DAT_009e9a67[];
extern unsigned char DAT_009d8b80[];
extern unsigned char DAT_009d8b84[];
extern unsigned char DAT_009d8b88[];
extern unsigned char DAT_009d8b8c[];
extern unsigned int DAT_009d8a54;
extern unsigned int DAT_009d8a58;
extern unsigned int DAT_009d8b60;

void __cdecl FUN_0041bb00(unsigned short param_1);
unsigned int __cdecl FUN_004269e0(unsigned int param_1);
void __cdecl FUN_00485270(int param_1);
int __cdecl FUN_0046b610(unsigned char *param_1);
void __cdecl FUN_004035d0(int param_1, int param_2, short param_3, short param_4);
void __cdecl FUN_00440d60(unsigned int param_1, int param_2);
void __cdecl FUN_004515d0(void);
void __cdecl FUN_0042e4d0(int param_1, unsigned int param_2, int param_3);
void __cdecl FUN_00462a90(void);

#define P(x) ((unsigned char *)(unsigned int)(x))

unsigned int __cdecl FUN_0045a1e0(int param_1, unsigned int param_2, unsigned int param_3, int param_4)
{
    unsigned char *pbVar1;
    char cVar2;
    int iVar3;
    _Bool bVar4;
    _Bool bVar5;
    unsigned int uVar6;
    unsigned int uVar7;
    unsigned int uVar8;
    int iVar9;
    unsigned char *pcVar10;
    int iVar11;
    int iVar12;
    int iVar13;
    unsigned int uVar14;
    int local_c;
    unsigned short local_8;
    int iVar1;
    int iVar2;
    int iVar4;
    int iVar5;
    unsigned int uVar5;
    short sVar3;
    short sVar4;

    bVar4 = 0;
    bVar5 = 0;
    local_c = 0;
    if (param_3 != 0) {
        local_c = _DAT_00a07cec;
        if (_DAT_00a07cec < DAT_009a1fe0) {
            pcVar10 = P((unsigned int)_DAT_00a07cec * 0x23u + 3u + (unsigned int)DAT_009a1fe8);
            do {
                if (*pcVar10 != 0) {
                    __builtin_memcpy(&iVar4, DAT_009d9192 + (unsigned int)pcVar10[0x13] * 0x566u, 4);
                    if (iVar4 == param_1) {
                        __builtin_memcpy(&uVar6, pcVar10 + 0x1c, 4);
                        __builtin_memcpy(&uVar6, P(uVar6) + 0xa9, 4);
                        cVar2 = (char)P(uVar6)[7];
                        if (cVar2 != 3 && cVar2 != 5 && cVar2 != 4) {
                            __builtin_memcpy(&local_8, pcVar10 - 3, 2);
                            if ((unsigned int)local_8 == param_3) {
                                bVar4 = 1;
                                bVar5 = 1;
                                break;
                            }
                        }
                    }
                }
                pcVar10 = pcVar10 + 0x23;
                local_c = local_c + 1;
            } while (local_c < DAT_009a1fe0);
        }
    }
    uVar14 = (unsigned int)local_c;
    if (!bVar4) {
        uVar14 = param_2;
        for (iVar11 = DAT_009a1fe0 - _DAT_00a07cec; iVar11 != 0; iVar11 = iVar11 - 1) {
            uVar14 = uVar14 + 1;
            if (DAT_009a1fe0 <= (int)uVar14) {
                uVar14 = (unsigned int)_DAT_00a07cec;
            }
            iVar13 = (int)(uVar14 * 0x23u + (unsigned int)DAT_009a1fe8);
            if (P(iVar13)[3] != 0) {
                __builtin_memcpy(&iVar4, DAT_009d9192 + (unsigned int)P(iVar13)[0x16] * 0x566u, 4);
                if (iVar4 == param_1) {
                    __builtin_memcpy(&uVar6, P(iVar13) + 0x1f, 4);
                    __builtin_memcpy(&uVar6, P(uVar6) + 0xa9, 4);
                    cVar2 = (char)P(uVar6)[7];
                    if (cVar2 != 3 && cVar2 != 5 && cVar2 != 4) {
                        break;
                    }
                }
            }
        }
        if (iVar11 == 0) {
            return 1;
        }
    }
    iVar13 = (int)(uVar14 * 0x23u);
    iVar1 = (int)((unsigned int)DAT_009a1fe8 + (unsigned int)iVar13);
    __builtin_memcpy(P(iVar1) + 0x1b, &param_1, 4);
    __builtin_memcpy(&iVar2, P(iVar1) + 0x1f, 4);
    P(iVar2)[0x92] = 1;
    __builtin_memcpy(&iVar2, P(iVar1) + 0x1f, 4);
    P(iVar2)[0x8b] = 1;
    iVar12 = 0;
    FUN_0041bb00((unsigned short)uVar14);
    iVar1 = (int)((unsigned int)DAT_009a1fe8 + (unsigned int)iVar13);
    __builtin_memcpy(&iVar2, P(iVar1) + 0x1f, 4);
    __builtin_memcpy(&iVar11, P(iVar2) + 0xa9, 4);
    do {
        P(iVar11)[0x220 + iVar12] = 1;
        P(iVar11)[0x222 + iVar12] = 0;
        P(iVar11)[0x224 + iVar12] = 0;
        local_8 = 0;
        __builtin_memcpy(P(iVar11) + 0x226 + iVar12 * 2, &local_8, 2);
        uVar6 = 0;
        __builtin_memcpy(P(iVar11) + 0x22a + iVar12 * 4, &uVar6, 4);
        uVar8 = FUN_004269e0((unsigned int)P((unsigned int)DAT_009a1fe8 + (unsigned int)iVar13)[3]);
        if (P(iVar11)[0x21e + iVar12] != 0) {
            iVar9 = (int)((uVar8 & 0xffff) * 0x101u);
            if (DAT_0051c5a6[iVar12 + iVar9] != 2) {
                P(iVar11)[0x224 + iVar12] = DAT_0051c5a0[iVar9 + iVar12];
            }
        }
        iVar12 = iVar12 + 1;
    } while (iVar12 < 2);
    iVar12 = (int)((unsigned int)param_1 * 0x5bdu);
    P(iVar11)[0x220] = DAT_009e96b1[iVar12];
    iVar9 = 0;
    P(iVar11)[0x221] = DAT_009e96b2[iVar12];
    do {
        pbVar1 = P(iVar11) + 0x235 + iVar9;
        iVar9 = iVar9 + 1;
        P(iVar11)[0x234 + iVar9] = (unsigned char)((*pbVar1 & 0x81) | 1);
        local_8 = 0;
        __builtin_memcpy(P(iVar11) + 0x235 + iVar9 * 2, &local_8, 2);
    } while (iVar9 < 2);
    P(iVar11)[0x235] = (unsigned char)(DAT_009e96b3[iVar12] | (P(iVar11)[0x235] & 0x80));
    P(iVar11)[0x236] = (unsigned char)(DAT_009e96b4[iVar12] | (P(iVar11)[0x236] & 0x80));
    local_8 = 0;
    __builtin_memcpy(P(iVar11) + 0x23b, &local_8, 2);
    __builtin_memcpy(&iVar3, DAT_0051c573 + (unsigned int)P(iVar11)[4] * 0x101u, 4);
    iVar9 = (int)((unsigned int)iVar3 * 2u);
    cVar2 = (char)DAT_009e96b0[iVar12];
    P(iVar11)[0x21a] = (unsigned char)cVar2;
    if (cVar2 == 0) {
        __builtin_memcpy(&iVar4, P(iVar11) + 0x211, 4);
        if (iVar9 < iVar4) {
            iVar5 = (int)((unsigned int)iVar4 + (unsigned int)iVar3 * 0xfffffffeu);
            __builtin_memcpy(P(iVar11) + 0x215, &iVar5, 4);
            __builtin_memcpy(P(iVar11) + 0x211, &iVar9, 4);
        }
    } else if (cVar2 == 1) {
        __builtin_memcpy(&iVar4, P(iVar11) + 0x211, 4);
        iVar9 = iVar4 >> 1;
        __builtin_memcpy(P(iVar11) + 0x211, &iVar9, 4);
        __builtin_memcpy(P(iVar11) + 0x215, &iVar9, 4);
    } else if (cVar2 == 2) {
        __builtin_memcpy(&iVar4, P(iVar11) + 0x211, 4);
        __builtin_memcpy(P(iVar11) + 0x215, &iVar4, 4);
        uVar6 = 0;
        __builtin_memcpy(P(iVar11) + 0x211, &uVar6, 4);
        __builtin_memcpy(&iVar4, P(iVar11) + 0x215, 4);
        if (iVar9 < iVar4) {
            iVar5 = (int)((unsigned int)iVar4 + (unsigned int)iVar3 * 0xfffffffeu);
            __builtin_memcpy(P(iVar11) + 0x211, &iVar5, 4);
            __builtin_memcpy(P(iVar11) + 0x215, &iVar9, 4);
        }
    }
    P(iVar11)[0x219] = DAT_009e96ae[iVar12];
    P(iVar11)[0x21c] = DAT_009e96ad[iVar12];
    P(iVar11)[0x23e] = DAT_009e96af[iVar12];
    iVar9 = DAT_009a1fe8;
    __builtin_memcpy(DAT_009e9670 + iVar12, &uVar14, 4);
    __builtin_memcpy(&local_8, P((unsigned int)iVar9 + (unsigned int)iVar13), 2);
    uVar6 = (unsigned int)local_8;
    __builtin_memcpy(DAT_009e9674 + iVar12, &uVar6, 4);
    DAT_009e9681[iVar12] = 0;
    DAT_009e9684[iVar12] = 0;
    if (bVar5 || param_2 != 0xffffffffu) {
        __builtin_memcpy(&local_8, DAT_009e96ab + iVar12, 2);
        __builtin_memcpy(P(iVar11) + 0xbd, &local_8, 2);
    }
    if (!bVar5) {
        DAT_009e9697[iVar12] = 0;
        DAT_009e9698[iVar12] = 0;
    }
    local_8 = 0xffff;
    __builtin_memcpy(DAT_009e968c + iVar12, &local_8, 2);
    __builtin_memcpy(DAT_009e969b + iVar12, &local_8, 2);
    local_8 = 0;
    __builtin_memcpy(DAT_009e9699 + iVar12, &local_8, 2);
    if (param_4 == 1) {
        DAT_009e9689[iVar12] = 1;
        local_8 = 0xffff;
        __builtin_memcpy(DAT_009e968a + iVar12, &local_8, 2);
        uVar6 = 0xffffffffu;
        __builtin_memcpy(DAT_009e968e + iVar12, &uVar6, 4);
        __builtin_memcpy(DAT_009e9692 + iVar12, &uVar6, 4);
    }
    if (param_2 != 0xffffffffu) {
        __builtin_memcpy(&iVar4, P(param_2 * 0x23u + 0x1fu + (unsigned int)DAT_009a1fe8), 4);
        if (iVar4 != 0) {
            local_8 = 0xffff;
            __builtin_memcpy(DAT_009e968a + iVar12, &local_8, 2);
            __builtin_memcpy(&local_8, P(iVar11) + 0x42, 2);
            if (local_8 < 0x8000) {
                __builtin_memcpy(DAT_009e968a + iVar12, &local_8, 2);
                local_8 = 0;
                __builtin_memcpy(DAT_009e9699 + iVar12, &local_8, 2);
            }
        }
    }
    __builtin_memcpy(&local_8, DAT_009e968a + iVar12, 2);
    if ((unsigned int)local_8 == uVar14) {
        local_8 = 0xffff;
        __builtin_memcpy(DAT_009e968a + iVar12, &local_8, 2);
    }
    FUN_00485270(param_1);
    DAT_009e9696[iVar12] = 0;
    iVar13 = FUN_0046b610(DAT_0051a1d0);
    P(iVar11)[0x34] = (unsigned char)iVar13;
    local_8 = 0xffff;
    __builtin_memcpy(P(iVar11) + 0x42, &local_8, 2);
    uVar6 = 0;
    __builtin_memcpy(DAT_009e9c1d + iVar12, &uVar6, 4);
    __builtin_memcpy(DAT_009e9c21 + iVar12, &uVar6, 4);
    local_8 = 0;
    __builtin_memcpy(DAT_009e96bb + iVar12, &local_8, 2);
    __builtin_memcpy(DAT_009e96bd + iVar12, &local_8, 2);
    __builtin_memcpy(DAT_009e96bf + iVar12, &local_8, 2);
    __builtin_memcpy(DAT_009e96c1 + iVar12, &local_8, 2);
    __builtin_memcpy(DAT_009e96c3 + iVar12, &local_8, 2);
    iVar13 = (int)((unsigned int)param_1 * 0x10u);
    local_8 = 0xffff;
    __builtin_memcpy(DAT_009e969d + iVar12, &local_8, 2);
    __builtin_memcpy(&iVar11, DAT_009e9670 + iVar12, 4);
    uVar6 = 0;
    __builtin_memcpy(DAT_009d8b80 + (unsigned int)param_1 * 8u, &uVar6, 4);
    __builtin_memcpy(DAT_009d8b84 + iVar13, &uVar6, 4);
    __builtin_memcpy(DAT_009d8b88 + iVar13, &uVar6, 4);
    __builtin_memcpy(DAT_009d8b8c + iVar13, &uVar6, 4);
    uVar14 = FUN_004269e0((unsigned int)P((unsigned int)iVar11 * 0x23u + 3u + (unsigned int)DAT_009a1fe8)[0]);
    iVar11 = (int)((uVar14 & 0xffff) * 0x101u);
    __builtin_memcpy(&sVar3, DAT_0051c635 + iVar11, 2);
    __builtin_memcpy(&sVar4, DAT_0051c633 + iVar11, 2);
    __builtin_memcpy(&iVar4, DAT_009e9670 + iVar12, 4);
    FUN_004035d0((int)((unsigned int)iVar4 * 0x23u + (unsigned int)DAT_009a1fe8), 0, sVar3, sVar4);
    uVar7 = DAT_009d8b60;
    uVar6 = DAT_009d8a54;
    uVar5 = DAT_009d8a58;
    __builtin_memcpy(DAT_009e96c5 + iVar12, &uVar5, 4);
    __builtin_memcpy(DAT_009e96c9 + iVar12, &uVar6, 4);
    __builtin_memcpy(DAT_009e96cd + iVar12, &uVar7, 4);
    __builtin_memcpy(&uVar5, DAT_009e96c5 + iVar12, 4);
    __builtin_memcpy(DAT_009e96d1 + iVar12, &uVar5, 4);
    __builtin_memcpy(&uVar5, DAT_009e96c9 + iVar12, 4);
    __builtin_memcpy(DAT_009e96d5 + iVar12, &uVar5, 4);
    __builtin_memcpy(&uVar5, DAT_009e96cd + iVar12, 4);
    __builtin_memcpy(DAT_009e96d9 + iVar12, &uVar5, 4);
    DAT_009e9a79[iVar12] = 0;
    local_8 = 0;
    __builtin_memcpy(DAT_009e9a73 + iVar12, &local_8, 2);
    __builtin_memcpy(DAT_009e9a75 + iVar12, &local_8, 2);
    __builtin_memcpy(DAT_009e9a84 + iVar12, &local_8, 2);
    uVar6 = 0x400;
    __builtin_memcpy(DAT_009e9a86 + iVar12, &uVar6, 4);
    __builtin_memcpy(DAT_009e9a80 + iVar12, &local_8, 2);
    __builtin_memcpy(DAT_009e9a8a + iVar12, &local_8, 2);
    __builtin_memcpy(&uVar6, DAT_009e9670 + iVar12, 4);
    local_8 = (unsigned short)uVar6;
    __builtin_memcpy(DAT_009e9a67 + iVar12, &local_8, 2);
    if (DAT_009e96b5[iVar12] == 0x13) {
        uVar14 = 0x13;
    } else {
        uVar14 = 0;
    }
    FUN_00440d60(uVar14, param_1);
    if (param_1 == DAT_009ecc34) {
        FUN_004515d0();
        FUN_0042e4d0(0xf, 0xffffffffu, param_1);
    }
    FUN_00462a90();
    return 0;
}

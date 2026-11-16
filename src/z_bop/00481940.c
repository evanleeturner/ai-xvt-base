/* SPDX-License-Identifier: MIT */
/* z_bop 00481940 */
extern unsigned char DAT_009e967c[];
extern unsigned char DAT_009e9670[];
extern unsigned int _DAT_00a07cec;
extern unsigned int DAT_009a1fe0;
extern unsigned char *DAT_009a1fe8;
extern unsigned int DAT_00a07ce8;
extern unsigned int DAT_009d1308;
extern unsigned int DAT_009a8c00;
extern char DAT_009d8c88[];
extern char DAT_009d8e25[];
extern char DAT_009d8e29[];
extern char DAT_009a2196[];
extern char DAT_009a8080[];
extern char DAT_009a8081[];
extern char DAT_009a8082[];
extern char DAT_009a8084[];
extern char DAT_009a8085[];
extern char DAT_009a8086[];
extern char DAT_009a808b[];
extern char DAT_009a808f[];
extern char DAT_009a8090[];
extern char DAT_009a8091[];

unsigned int __cdecl FUN_00484f80(unsigned short a);
unsigned char __cdecl FUN_00433390(unsigned int a, unsigned short b, unsigned short c);
void __cdecl FUN_00485000(int a, unsigned int b);
unsigned int __cdecl FUN_00451f50(unsigned int a, int b, int c, int d);

unsigned int __cdecl FUN_00481940(int param_1, int param_2)
{
    int iVar1;
    char cVar2;
    int iVar3;
    int bVar4;
    unsigned char bVar5;
    unsigned int uVar6;
    unsigned int uVar7;
    int iVar8;
    int iVar9;
    unsigned int uVar10;
    char *pcVar11;
    unsigned int uVar12;
    unsigned int uVar13;
    char *pcVar14;
    unsigned short uVar15;
    unsigned short local_1c;
    unsigned short uStack_1a;
    unsigned short uStack_18;
    unsigned int local_10;
    unsigned int local_8;
    unsigned short tmpW;
    unsigned int tmpD;
    unsigned char *pA;
    unsigned char *pB;

    iVar3 = (int)((unsigned int)param_2 * 0x5bdU);
    __builtin_memcpy(&tmpW, DAT_009e967c + iVar3, 2);
    uVar6 = (unsigned int)tmpW;
    uStack_1a = 0xffff;
    local_1c = 0xffff;
    local_8 = 0xffffffff;
    local_10 = 0xffffffff;
    if (_DAT_00a07cec < DAT_009a1fe0) {
        iVar9 = (int)(_DAT_00a07cec * 0x23U);
        uVar13 = _DAT_00a07cec;
        do {
            __builtin_memcpy(&tmpD, DAT_009e9670 + iVar3, 4);
            if (((DAT_009a1fe8[iVar9 + 3] != 0) && (tmpD != uVar13)) &&
                (DAT_009a1fe8[iVar9 + 2] != '\r')) {
                uVar15 = (unsigned short)uVar13;
                uVar7 = FUN_00484f80(uVar15);
                if ((char)uVar7 == '\0') {
                    uVar7 = (unsigned int)DAT_009a1fe8[0x16 + iVar9];
                    iVar1 = (int)(uVar7 * 0x566U);
                    if ((unsigned char)DAT_009d8c88[iVar1] != uVar6) {
                        uVar12 = 0;
                        pcVar14 = DAT_009d8e25 + iVar1;
                        bVar4 = 0;
                        pcVar11 = DAT_009d8e29 + uVar6 + iVar1;
                        do {
                            if (((*pcVar11 != '\0') && (*pcVar14 == (char)param_1)) &&
                                ((-1 < pcVar14[3]) &&
                                 (DAT_009a2196[uVar12 + uVar6 * 8 + uVar7 * 0x126] == '\x04'))) {
                                bVar4 = 1;
                            }
                            pcVar14 = pcVar14 + 0x4e;
                            pcVar11 = pcVar11 + 0x4e;
                            uVar12 = uVar12 + 1;
                        } while (uVar12 < 8);
                        iVar8 = (int)(uVar6 * 7U + (unsigned int)param_1);
                        iVar1 = (int)((unsigned int)iVar8 * 0x2aU);
                        if ((DAT_009a8080[iVar1] != '\n') && (DAT_009a8080[iVar1] != '\0')) {
                            bVar5 = FUN_00433390(uVar13, (unsigned short)(unsigned char)DAT_009a8081[iVar1],
                                                 (unsigned short)(unsigned char)DAT_009a8082[iVar1]);
                            if (bVar5 != 0) {
                                bVar4 = 1;
                            }
                        }
                        if ((DAT_009a8084[iVar1] != '\n') && (DAT_009a8084[iVar1] != '\0')) {
                            bVar5 = FUN_00433390(uVar13, (unsigned short)(unsigned char)DAT_009a8085[iVar1],
                                                 (unsigned short)(unsigned char)DAT_009a8086[iVar1]);
                            if (bVar5 != 0) {
                                bVar4 = 1;
                            }
                        }
                        if ((DAT_009a808b[iVar1] != '\n') && (DAT_009a808b[iVar1] != '\0')) {
                            bVar5 = FUN_00433390(uVar13, (unsigned short)(unsigned char)DAT_009a8081[iVar1],
                                                 (unsigned short)(unsigned char)DAT_009a8082[iVar1]);
                            if (bVar5 != 0) {
                                bVar4 = 1;
                            }
                        }
                        if ((DAT_009a808f[iVar1] != '\n') && (DAT_009a808f[iVar1] != '\0')) {
                            bVar5 = FUN_00433390(uVar13, (unsigned short)(unsigned char)DAT_009a8090[iVar1],
                                                 (unsigned short)(unsigned char)DAT_009a8091[iVar1]);
                            if (bVar5 != 0) {
                                bVar4 = 1;
                            }
                        }
                        if (bVar4) {
                            __builtin_memcpy(&pA, DAT_009a1fe8 + 0x1f + iVar9, 4);
                            __builtin_memcpy(&pB, pA + 0xa9, 4);
                            cVar2 = (char)pB[7];
                            if (cVar2 == '\0' || (cVar2 == '\x06')) {
                                FUN_00485000(param_2, uVar13);
                                uVar7 = FUN_00451f50(uVar13, param_2, 0, 1);
                                if (uVar7 == 0) {
                                    if (DAT_009a8c00 < local_10) {
                                        local_10 = DAT_009a8c00;
                                        local_1c = uVar15;
                                    }
                                } else if (DAT_009a8c00 < local_8) {
                                    local_8 = DAT_009a8c00;
                                    uStack_1a = uVar15;
                                }
                            }
                        }
                    }
                }
            }
            uVar13 = uVar13 + 1;
            iVar9 = (int)((unsigned int)iVar9 + 0x23U);
        } while (uVar13 < DAT_009a1fe0);
    }
    uStack_18 = (unsigned short)DAT_00a07ce8;
    if (DAT_00a07ce8 < DAT_009d1308 + DAT_00a07ce8) {
        local_10 = DAT_00a07ce8 * 0x23U;
        uVar13 = DAT_00a07ce8;
        do {
            uStack_18 = (unsigned short)uVar13;
            if (DAT_009a1fe8[local_10 + 3] != 0) {
                uVar7 = (unsigned int)DAT_009a1fe8[local_10 + 0x16];
                iVar9 = (int)(uVar7 * 0x566U);
                __builtin_memcpy(&tmpW, DAT_009e967c + iVar3, 2);
                if ((unsigned short)(unsigned char)DAT_009d8c88[iVar9] != tmpW) {
                    uVar12 = 0;
                    pcVar14 = DAT_009d8e25 + iVar9;
                    pcVar11 = DAT_009d8e29 + uVar6 + iVar9;
                    do {
                        if ((((*pcVar11 != '\0') && (*pcVar14 == (char)param_1)) && (-1 < pcVar14[3])) &&
                            (DAT_009a2196[uVar12 + uVar6 * 8 + uVar7 * 0x126] == '\x04')) {
                            FUN_00485000(param_2, uVar13);
                            if (DAT_009a8c00 < local_8) {
                                local_8 = DAT_009a8c00;
                                uStack_1a = uStack_18;
                            }
                        }
                        pcVar14 = pcVar14 + 0x4e;
                        pcVar11 = pcVar11 + 0x4e;
                        uVar12 = uVar12 + 1;
                    } while (uVar12 < 8);
                }
            }
            local_10 = local_10 + 0x23U;
            uVar13 = uVar13 + 1;
            uStack_18 = (unsigned short)uVar13;
        } while (uVar13 < DAT_009d1308 + DAT_00a07ce8);
    }
    uVar10 = ((unsigned int)uStack_18 << 16) | (unsigned int)uStack_1a;
    if (uStack_1a == 0xffff) {
        uVar10 = 0xffff0000U | (unsigned int)local_1c;
    }
    return uVar10;
}

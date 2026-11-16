/* SPDX-License-Identifier: MIT */
/* z_bop 00482be0 */
extern char DAT_009a2062;
extern int DAT_009d6990[8];
extern int DAT_009d6968[8];
extern int DAT_00a07cec;
extern int DAT_009a1fe0;
extern int DAT_009a1fe8;
extern int DAT_009ecc34;
extern unsigned char DAT_009e9670[];
extern unsigned char DAT_009d8c88[];
extern unsigned char DAT_009fd458[];
extern char DAT_009d6959;

unsigned int FUN_00484f80(unsigned short a);
void FUN_00482eb0(unsigned short a, unsigned short b, unsigned char c);

void __stdcall FUN_00482be0(void)
{
    int iVar4;
    unsigned int uVar2;
    unsigned int uVar6;
    unsigned char bVar5;
    int bVar7;
    int local_10;
    int local_c;
    unsigned int local_8;
    int local_4;
    int i;
    unsigned char *pEntry;
    unsigned char *pPlayer;
    unsigned char *p1f;
    unsigned char *pA9;
    int i1b;
    unsigned int pl0;
    unsigned short plC;
    unsigned short pl1A;
    unsigned int idx;

    local_4 = 10;
    local_10 = 0;
    if (DAT_009a2062 == 2) {
        iVar4 = 0;
        local_10 = 0;
        for (i = 0; &DAT_009d6990[i] < &DAT_009d6990[8]; i++) {
            if (local_10 < (int)((unsigned int)DAT_009d6968[i] + (unsigned int)DAT_009d6990[i])) {
                local_10 = (int)((unsigned int)DAT_009d6968[i] + (unsigned int)DAT_009d6990[i]);
                local_4 = iVar4;
            }
            iVar4 = iVar4 + 1;
        }
    }
    if (DAT_00a07cec < DAT_009a1fe0) {
        local_c = (int)((unsigned int)DAT_00a07cec * 0x23U);
        uVar6 = (unsigned int)DAT_00a07cec;
        do {
            pEntry = (unsigned char *)(unsigned int)((unsigned int)DAT_009a1fe8 + (unsigned int)local_c);
            if (pEntry[3] != 0) {
                pPlayer = DAT_009e9670 + (unsigned int)DAT_009ecc34 * 0x5bdU;
                __builtin_memcpy(&pl0, pPlayer, 4);
                if (pl0 != uVar6) {
                    bVar5 = 0;
                    __builtin_memcpy(&p1f, pEntry + 0x1f, 4);
                    __builtin_memcpy(&plC, pPlayer + 0xc, 2);
                    uVar2 = (unsigned int)p1f[0x89];
                    if ((DAT_009a2062 == 2) &&
                        ((local_8 = (unsigned int)plC, local_8 == uVar2) &&
                         (pEntry[2] == 0))) {
                        bVar5 = 0x2f;
                    }
                    else if ((local_4 == 10) ||
                             ((int)((unsigned int)DAT_009d6990[uVar2] + (unsigned int)DAT_009d6968[uVar2]) != local_10)) {
                        __builtin_memcpy(&i1b, pEntry + 0x1b, 4);
                        if ((i1b != -1) && (i1b != DAT_009ecc34)) {
                            if (DAT_009a2062 == 2) {
                                idx = (unsigned int)*(unsigned char *)((uVar6 & 0xffff) * 0x23U + 0x16U + (unsigned int)DAT_009a1fe8);
                                if ((unsigned int)DAT_009d8c88[idx * 0x566U] == (unsigned int)plC) {
                                    bVar7 = 0;
                                }
                                else {
                                    bVar7 = DAT_009fd458[(unsigned int)DAT_009d8c88[idx * 0x566U] + (unsigned int)plC * 0x1e5U] == 0;
                                }
                                bVar5 = (unsigned char)((((unsigned int)(-(unsigned int)(!bVar7))) & 0xa0U) + 0x33U);
                            }
                            else {
                                switch (p1f[0x88]) {
                                case 0:
                                    bVar5 = 0x3f;
                                    break;
                                case 1:
                                case 4:
                                    bVar5 = 0x37;
                                    break;
                                case 2:
                                    bVar5 = 0x33;
                                    break;
                                default:
                                    bVar5 = 0x3b;
                                }
                            }
                        }
                    }
                    else {
                        bVar5 = 0xd4;
                    }
                    if (bVar5 != 0) {
                        if (DAT_009d6959 == 0) {
                            uVar2 = (unsigned int)plC;
                            __builtin_memcpy(&pA9, p1f + 0xa9, 4);
                            if (pA9[uVar2 + 0x1f6U] == 0) {
                                idx = (unsigned int)*(unsigned char *)((uVar6 & 0xffff) * 0x23U + 0x16U + (unsigned int)DAT_009a1fe8);
                                if ((unsigned int)DAT_009d8c88[idx * 0x566U] == uVar2) {
                                    bVar7 = 0;
                                }
                                else {
                                    bVar7 = DAT_009fd458[(unsigned int)DAT_009d8c88[idx * 0x566U] + uVar2 * 0x1e5U] == 0;
                                }
                                if (bVar7) goto LAB_00482e70;
                            }
                        }
                        uVar2 = FUN_00484f80((unsigned short)uVar6);
                        __builtin_memcpy(&pl1A, pPlayer + 0x1a, 2);
                        if (((char)uVar2 == 0) && ((unsigned int)pl1A != uVar6)) {
                            FUN_00482eb0((unsigned short)uVar6, 0xffff, bVar5);
                        }
                    }
                }
            }
LAB_00482e70:
            uVar6 = uVar6 + 1;
            local_c = (int)((unsigned int)local_c + 0x23U);
        } while ((int)uVar6 < DAT_009a1fe0);
    }
    return;
}

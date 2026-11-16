/* SPDX-License-Identifier: MIT */
/* z_bop 004833d0 */
extern unsigned int DAT_009a1fe8;
extern int DAT_009a1fe0;
extern unsigned char DAT_009e9670[];
extern unsigned char DAT_009e9683[];
extern unsigned char DAT_009e968a[];
extern unsigned char DAT_009e9696[];
extern unsigned char DAT_009e9699[];
extern unsigned char DAT_009e9a67[];
extern unsigned char DAT_009e9a77[];
extern unsigned char DAT_009e9a8a[];
extern int DAT_009ecc34;
extern unsigned int DAT_00a00870[];
extern unsigned int DAT_00a00874[];
extern unsigned short _DAT_00a07be0;
extern unsigned short _DAT_00a07be2;

unsigned int __cdecl FUN_0042e4d0(unsigned int param_1, unsigned int param_2, int param_3);
unsigned int __cdecl FUN_004adc40(unsigned int param_1);
unsigned int __cdecl FUN_004ae2a0(unsigned int param_1, unsigned int param_2);
unsigned int __cdecl FUN_00483800(unsigned int param_1, int param_2);
void __cdecl FUN_00451f50(unsigned int param_1, int param_2, int param_3, int param_4);
void __cdecl FUN_00450650(int param_1, int param_2);

void __cdecl FUN_004833d0(unsigned int param_1, int param_2)
{
    unsigned char bVar1;
    int bVar2;
    unsigned int iVar3;
    unsigned int uVar4;
    int iVar5;
    unsigned int uVar6;
    unsigned int uVar7;
    int local_4;
    unsigned int pA;
    unsigned int pB;
    unsigned short s16;
    unsigned short z16;
    unsigned int u32;

    if ((short)param_1 != -1) {
        uVar7 = param_1 & 0xffff;
        iVar3 = uVar7 * 0x23 + DAT_009a1fe8;
        if ((*(char *)(iVar3 + 3) != '\0') && (*(char *)(iVar3 + 2) != '\r')) {
            iVar3 = (unsigned int)param_2 * 0x5bd;
            __builtin_memcpy(&uVar4, &DAT_009e9670[iVar3], 4);
            if (uVar4 != uVar7) {
                bVar2 = 1;
                if (uVar4 != 0xffffffff) {
                    __builtin_memcpy(&pA, (unsigned char *)(uVar4 * 0x23 + 0x1f + DAT_009a1fe8), 4);
                    __builtin_memcpy(&pB, (unsigned char *)(pA + 0xa9), 4);
                    if ((*(unsigned char *)(pB + 0x1e5) & 4) == 0) {
                        bVar2 = 0;
                    }
                }
                if (bVar2) {
                    __builtin_memcpy(&s16, &DAT_009e968a[iVar3], 2);
                    if (((short)param_1 != -1) && ((short)s16 != (short)param_1)) {
                        FUN_0042e4d0(0x31, 0xffffffff, param_2);
                        s16 = (unsigned short)param_1;
                        __builtin_memcpy(&DAT_009e968a[iVar3], &s16, 2);
                        iVar5 = DAT_009a1fe0;
                        z16 = 0;
                        __builtin_memcpy(&DAT_009e9699[iVar3], &z16, 2);
                        if ((int)uVar7 < iVar5) {
                            if (DAT_009e9683[iVar3] == '\0') {
                                uVar7 = FUN_00483800(param_1, param_2);
                                s16 = (unsigned short)uVar7;
                                __builtin_memcpy(&DAT_009e9699[iVar3], &s16, 2);
                            }
                            else {
                                __builtin_memcpy(&s16, &DAT_009e968a[iVar3], 2);
                                bVar1 = *(unsigned char *)((unsigned int)s16 * 0x23 + 3 + DAT_009a1fe8);
                                if (bVar1 < 0x49) {
                                    local_4 = (int)DAT_00a00870[(unsigned int)bVar1 * 0x65];
                                }
                                else {
                                    local_4 = (int)FUN_004adc40((unsigned int)bVar1);
                                }
                                uVar7 = 0;
                                if (0 < local_4) {
                                    do {
                                        __builtin_memcpy(&s16, &DAT_009e968a[iVar3], 2);
                                        bVar1 = *(unsigned char *)((unsigned int)s16 * 0x23 + 3 + DAT_009a1fe8);
                                        uVar4 = (unsigned int)bVar1;
                                        if (bVar1 < 0x49) {
                                            uVar6 = uVar7;
                                            if ((int)DAT_00a00870[uVar4 * 0x65] <= (int)uVar7) {
                                                uVar6 = DAT_00a00870[uVar4 * 0x65] - 1;
                                            }
                                            iVar5 = (int)DAT_00a00874[uVar4 * 0x65 + uVar6];
                                        }
                                        else {
                                            iVar5 = (int)FUN_004ae2a0(uVar4, uVar7);
                                        }
                                        if ((iVar5 == 1) || (iVar5 == 3)) {
                                            s16 = (unsigned short)uVar7;
                                            __builtin_memcpy(&DAT_009e9699[iVar3], &s16, 2);
                                            break;
                                        }
                                        uVar7 = (unsigned int)(unsigned short)((short)uVar7 + 1);
                                    } while ((int)uVar7 < local_4);
                                }
                            }
                        }
                        __builtin_memcpy(&s16, &DAT_009e9a8a[iVar3], 2);
                        if ((short)s16 != 0) {
                            __builtin_memcpy(&s16, &DAT_009e968a[iVar3], 2);
                            __builtin_memcpy(&DAT_009e9a67[iVar3], &s16, 2);
                        }
                        DAT_009e9696[iVar3] = 0;
                        __builtin_memcpy(&u32, &DAT_009e9670[iVar3], 4);
                        if (u32 != 0xffffffff) {
                            __builtin_memcpy(&pA, (unsigned char *)(u32 * 0x23 + 0x1f + DAT_009a1fe8), 4);
                            __builtin_memcpy(&pB, (unsigned char *)(pA + 0xa9), 4);
                            z16 = 0;
                            __builtin_memcpy((unsigned char *)(pB + 0x23b), &z16, 2);
                        }
                        if (DAT_009e9683[iVar3] != '\0') {
                            FUN_00451f50(param_1, param_2, 1, 0);
                            return;
                        }
                        __builtin_memcpy(&u32, &DAT_009e9670[iVar3], 4);
                        __builtin_memcpy(&pA, (unsigned char *)(u32 * 0x23 + 0x1f + DAT_009a1fe8), 4);
                        __builtin_memcpy(&pB, (unsigned char *)(pA + 0xa9), 4);
                        if (((*(unsigned char *)(pB + 0x1e1) & 1) != 0) &&
                            (((DAT_009e9a77[iVar3] == 0x13 || (DAT_009e9a77[iVar3] == 0)) ||
                              (DAT_009e9a77[(unsigned int)DAT_009ecc34 * 0x5bd] == 0x14)))) {
                            FUN_00451f50(param_1, param_2, 1, 0);
                            return;
                        }
                    }
                }
                else {
                    _DAT_00a07be0 = 0x60;
                    _DAT_00a07be2 = 0x57;
                    FUN_00450650(0x56, param_2);
                }
            }
        }
    }
    return;
}

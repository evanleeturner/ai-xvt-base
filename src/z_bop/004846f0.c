/* SPDX-License-Identifier: MIT */
/* z_bop 004846f0 */
/* Prototypes for every callee. */
unsigned int __cdecl FUN_00484f80(unsigned short param_1);
unsigned int __cdecl FUN_00435a10(unsigned char a, unsigned char b, unsigned char c);
void __cdecl FUN_00403540(unsigned int a, unsigned int b);
/* Ghidra showed __fastcall with extraout_ECX/EDX, but the callee ends with RET 0x0,
   so it is __cdecl with the three stack arguments only. */
unsigned long long __cdecl FUN_00482640(unsigned short a, int b, int c);
void __cdecl FUN_004034e0(unsigned int a, unsigned int b);

extern unsigned short DAT_00a07cec;
extern int DAT_009a1fe0;
extern int DAT_009a1fe8;
extern unsigned int DAT_009a1ff8;
extern unsigned int DAT_009a8c00;
extern unsigned int DAT_009ed228;   /* byte 3 (0x9ed22b) is read separately */
extern unsigned char DAT_009ed22c;
extern unsigned char DAT_009ed22d;
extern unsigned short DAT_009e968a;

unsigned short __cdecl FUN_004846f0(unsigned short param_1, unsigned short param_2)
{
    unsigned char *iVar1;
    unsigned int iVar2;
    unsigned int uVar3;
    int bVar4;
    unsigned char *iVar5;
    unsigned int uVar6;
    unsigned char *iVar7;
    unsigned int uVar8;
    unsigned long long uVar9;
    unsigned short local_10;
    unsigned short local_e;
    unsigned int local_4;
    unsigned int t0;
    unsigned int t1;
    unsigned short s0;
    unsigned short s1;

    if (param_1 == 0xffff) {
        return 0xffff;
    }
    local_4 = 0xffffffff;
    local_10 = 0xffff;
    local_e = DAT_00a07cec;
    if ((int)(unsigned int)DAT_00a07cec < DAT_009a1fe0) {
        do {
            uVar3 = (unsigned int)local_e;
            iVar2 = uVar3 * 0x23;
            iVar5 = (unsigned char *)((unsigned int)DAT_009a1fe8 + iVar2);
            if ((((*(char *)(iVar5 + 3) != '\0') && (local_e != param_1)) && (local_e != param_2)) &&
                (*(char *)(iVar5 + 2) != '\r')) {
                bVar4 = 0;
                __builtin_memcpy(&t0, iVar5 + 0x1f, 4);
                __builtin_memcpy(&t1, (unsigned char *)t0 + 0xa9, 4);
                iVar5 = (unsigned char *)t1;
                __builtin_memcpy(&s0, iVar5 + 0x1e5, 2);
                if (((s0 != 0) && (*(char *)(iVar5 + 7) == '\0')) &&
                    (uVar6 = FUN_00484f80(local_e), (char)uVar6 == '\0')) {
                    __builtin_memcpy(&t0, (unsigned char *)((unsigned int)DAT_009a1fe8 + 0x1b + iVar2), 4);
                    if ((int)t0 == -1) {
                        if ((*(unsigned short *)(iVar5 + 0x42) != param_1) ||
                            ((*(char *)(iVar5 + 0x5e) != '\f') && (*(char *)(iVar5 + 0x5e) != '\x17'))) {
                            goto LAB_00484926;
                        }
LAB_004848e8:
                        bVar4 = 1;
                    } else {
                        uVar6 = (unsigned int)param_1;
                        if ((int)uVar6 < DAT_009a1fe0) {
                            iVar7 = (unsigned char *)(uVar6 * 0x23 + (unsigned int)DAT_009a1fe8);
                            __builtin_memcpy(&t0, iVar7 + 0x1f, 4);
                            __builtin_memcpy(&t1, (unsigned char *)t0 + 0xa9, 4);
                            iVar1 = (unsigned char *)t1;
                            if (*(unsigned short *)(iVar1 + 0x6a) == local_e) {
                                __builtin_memcpy(&t0, iVar7 + 0x1b, 4);
                                if ((int)t0 == -1) {
                                    bVar4 = 1;
                                } else {
                                    uVar8 = FUN_00435a10((unsigned char)(DAT_009ed228 >> 24),
                                                         DAT_009ed22c, DAT_009ed22d);
                                    if ((int)((uVar8 & 0xffff) -
                                              (unsigned int)*(unsigned short *)(iVar1 + 0x6c)) < 5) {
                                        bVar4 = 1;
                                    }
                                }
                            }
                        }
                        __builtin_memcpy(&iVar2, (unsigned char *)((unsigned int)DAT_009a1fe8 + 0x1b + iVar2), 4);
                        __builtin_memcpy(&s1, (unsigned char *)&DAT_009e968a + iVar2 * 0x5bd, 2);
                        if (s1 == param_1) {
                            FUN_00403540(uVar3, uVar6);
                            if (DAT_009a1ff8 < 0x10000) {
                                uVar9 = FUN_00482640(param_1, 0, (int)iVar2);
                                if ((short)uVar9 != 0) {
                                    bVar4 = 1;
                                }
                            }
                            __builtin_memcpy(&s0, iVar5 + 0x23b, 2);
                            if (s0 != 0) goto LAB_004848e8;
                        }
                    }
                    if (bVar4) {
                        FUN_004034e0((unsigned int)param_1, uVar3);
                        if (DAT_009a8c00 < local_4) {
                            local_4 = DAT_009a8c00;
                            local_10 = local_e;
                        }
                    }
                }
            }
LAB_00484926:
            local_e = (unsigned short)(local_e + 1);
        } while ((int)(unsigned int)local_e < DAT_009a1fe0);
    }
    return local_10;
}

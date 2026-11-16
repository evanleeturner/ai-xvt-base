/* SPDX-License-Identifier: MIT */
/* z_bop 004b6780 */
extern int DAT_00aa6cfc;
extern unsigned char DAT_00aa7b13[4];
extern unsigned char DAT_00aa7b1f[4];
extern unsigned char DAT_00aac111[4];

void __cdecl FUN_004d6480(int *param_1, int *param_2);
unsigned int __cdecl FUN_004d64e0(int *param_1);
unsigned int __cdecl FUN_004d6f50(unsigned int param_1);
void __cdecl FUN_004b69b0(int *param_1, int param_2, int param_3, int param_4,
                          int param_5, int param_6, int param_7, char param_8);
void __cdecl FUN_004b6ca0(int *param_1, int param_2, int param_3, int param_4,
                          int param_5, int param_6, int param_7, short param_8);

unsigned int __cdecl FUN_004b6780(int *param_1, int param_2, int param_3,
                                  unsigned int param_4, int param_5)
{
    /* contiguous locals: the callee fills local_10..local_4 from local_20..local_14 */
    struct {
        int local_20;
        int local_1c;
        int local_18;
        int local_14;
        int local_10;
        int local_c;
        int local_8;
        int local_4;
    } L;
    int iVar1;
    unsigned int uVar2;
    int iVar3;
    int iVar4;
    int iVar5;
    int iVar6;
    int bpp;
    int pitch;
    int flag;

    if (param_1 == (int *)0) {
        return 0;
    }
    L.local_20 = param_2;
    L.local_1c = param_3;
    L.local_18 = (int)((unsigned int)*param_1 + (unsigned int)param_2 - 1u);
    L.local_14 = (int)((unsigned int)param_1[1] + (unsigned int)param_3 - 1u);
    FUN_004d6480(&L.local_10, &L.local_20);
    uVar2 = FUN_004d64e0(&L.local_20);
    if (L.local_20 <= L.local_18) {
        if (L.local_14 < L.local_1c) {
            return uVar2;
        }
        L.local_10 = (int)((unsigned int)L.local_20 - (unsigned int)L.local_10);
        L.local_c = (int)((unsigned int)L.local_1c - (unsigned int)L.local_c);
        iVar5 = (int)(((unsigned int)param_1[1] - (unsigned int)L.local_4
                       - (unsigned int)L.local_c) + (unsigned int)L.local_14);
        iVar1 = (int)(((unsigned int)L.local_18 - (unsigned int)L.local_10
                       - (unsigned int)L.local_8) + (unsigned int)*param_1);
        __builtin_memcpy(&bpp, DAT_00aa7b1f, 4);
        __builtin_memcpy(&pitch, DAT_00aa7b13, 4);
        if (param_1[2] == 0) {
            if (bpp == 8) {
                iVar3 = (int)((unsigned int)L.local_c * (unsigned int)*param_1
                              + (unsigned int)param_1[4] + (unsigned int)L.local_10);
                iVar4 = (int)(((unsigned int)param_3 + (unsigned int)L.local_c) * (unsigned int)pitch
                              + (unsigned int)L.local_10 + (unsigned int)DAT_00aa6cfc
                              + (unsigned int)param_2);
                if (0 < iVar5) {
                    do {
                        iVar6 = 0;
                        if (0 < iVar1) {
                            do {
                                if (*(char *)(iVar3 + iVar6) != '\0') {
                                    *(char *)(iVar4 + iVar6) = (char)param_4;
                                }
                                iVar6 = iVar6 + 1;
                            } while (iVar6 < iVar1);
                        }
                        iVar3 = (int)((unsigned int)iVar3 + (unsigned int)*param_1);
                        iVar4 = (int)((unsigned int)iVar4 + (unsigned int)pitch);
                        iVar5 = iVar5 + -1;
                    } while (iVar5 != 0);
                }
            }
            else if (bpp == 0x10) {
                __builtin_memcpy(&flag, DAT_00aac111, 4);
                if ((param_5 != 0) && (flag != 0)) {
                    param_4 = FUN_004d6f50(param_4);
                }
                iVar3 = (int)((unsigned int)*param_1 * (unsigned int)L.local_c
                              + (unsigned int)param_1[4] + (unsigned int)L.local_10);
                iVar4 = (int)(((unsigned int)param_3 + (unsigned int)L.local_c) * (unsigned int)pitch
                              + ((unsigned int)L.local_10 + (unsigned int)param_2) * 2u
                              + (unsigned int)DAT_00aa6cfc);
                if (0 < iVar5) {
                    do {
                        iVar6 = 0;
                        if (0 < iVar1) {
                            do {
                                if (*(char *)(iVar3 + iVar6) != '\0') {
                                    *(short *)(iVar4 + iVar6 * 2) = (short)param_4;
                                }
                                iVar6 = iVar6 + 1;
                            } while (iVar6 < iVar1);
                        }
                        iVar3 = (int)((unsigned int)iVar3 + (unsigned int)*param_1);
                        iVar4 = (int)((unsigned int)iVar4 + ((unsigned int)pitch & 0xfffffffeu));
                        iVar5 = iVar5 + -1;
                    } while (iVar5 != 0);
                }
            }
        }
        else if (bpp == 8) {
            FUN_004b69b0(param_1, (int)((unsigned int)L.local_10 + (unsigned int)param_2),
                         (int)((unsigned int)param_3 + (unsigned int)L.local_c),
                         L.local_10, L.local_c, iVar1, iVar5, (char)param_4);
        }
        else if (bpp == 0x10) {
            __builtin_memcpy(&flag, DAT_00aac111, 4);
            if ((param_5 != 0) && (flag != 0)) {
                param_4 = FUN_004d6f50(param_4);
            }
            FUN_004b6ca0(param_1, (int)((unsigned int)L.local_10 + (unsigned int)param_2),
                         (int)((unsigned int)param_3 + (unsigned int)L.local_c),
                         L.local_10, L.local_c, iVar1, iVar5, (short)param_4);
        }
    }
    return uVar2;
}

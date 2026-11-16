/* SPDX-License-Identifier: MIT */
/* z_bop 004d25d0 */
extern unsigned char DAT_00acebf9[4];
extern unsigned char DAT_00acebf1[4];
extern unsigned char DAT_00acebf5[4];
extern unsigned char DAT_00acf145[4];
extern unsigned char DAT_00acec45[];
extern unsigned char DAT_00b69a89[4];
extern unsigned char DAT_00b641c5[];
extern unsigned char DAT_00b63fa1[];
extern unsigned char DAT_00b63f99[];
extern unsigned char DAT_00b53579[4];
extern unsigned char DAT_00b53571[4];
extern unsigned char DAT_00acf571[];

unsigned int FUN_004d23b0(int param_1);
DWORD __stdcall GetTickCount(void);
int FUN_004cef70(int param_1, unsigned int *param_2, int param_3);

unsigned int __stdcall FUN_004d25d0(void)
{
    int iVar1;
    unsigned int uVar2;
    DWORD DVar3;
    unsigned int uVar4;
    unsigned char *piVar5;
    unsigned int local_8[2];
    unsigned char *e;
    unsigned int t;
    unsigned int t2;
    unsigned int cnt;
    unsigned int idx;
    unsigned int v;

    __builtin_memcpy(&t, DAT_00acebf9, 4);
    if (t == 0) {
        __builtin_memcpy(&t, DAT_00acebf1, 4);
        uVar2 = FUN_004d23b0((int)t);
        __builtin_memcpy(&t, DAT_00b69a89, 4);
        if (t <= uVar2) {
            return 0;
        }
        DVar3 = GetTickCount();
        __builtin_memcpy(&t, DAT_00b641c5 + uVar2 * 0x238, 4);
        if (45000 < DVar3 - t) {
            DVar3 = GetTickCount();
            __builtin_memcpy(DAT_00b641c5 + uVar2 * 0x238, &DVar3, 4);
            __builtin_memcpy(&cnt, DAT_00b53579, 4);
            if (cnt < 0x400) {
                __builtin_memcpy(&idx, DAT_00b53571, 4);
                e = DAT_00acf571 + idx * 0x210;
                v = 0x46;
                __builtin_memcpy(e + 0x10, &v, 4);
                __builtin_memcpy(&t, DAT_00acebf1, 4);
                __builtin_memcpy(e, &t, 4);
                v = 4;
                __builtin_memcpy(e + 4, &v, 4);
                v = 0;
                __builtin_memcpy(e + 8, &v, 4);
                e[0xc] = 0;
                e[0xf] = 0;
                e[0xd] = 0;
                __builtin_memcpy(&t, DAT_00b63fa1 + uVar2 * 0x238, 4);
                uVar4 = t + 1;
                if (0x7f < uVar4) {
                    uVar4 = 0;
                }
                __builtin_memcpy(DAT_00b63fa1 + uVar2 * 0x238, &uVar4, 4);
                e[0xe] = (unsigned char)uVar4;
                __builtin_memcpy(&cnt, DAT_00b53579, 4);
                cnt = cnt + 1;
                __builtin_memcpy(DAT_00b53579, &cnt, 4);
                __builtin_memcpy(&idx, DAT_00b53571, 4);
                idx = idx + 1;
                __builtin_memcpy(DAT_00b53571, &idx, 4);
                if (0x3ff < idx) {
                    idx = 0;
                    __builtin_memcpy(DAT_00b53571, &idx, 4);
                }
            }
        }
    }
    else {
        piVar5 = DAT_00acec45;
        do {
            __builtin_memcpy(&iVar1, piVar5, 4);
            if (iVar1 != 0) {
                __builtin_memcpy(&t, DAT_00acf145, 4);
                if (t != (unsigned int)iVar1) {
                    __builtin_memcpy(&t, DAT_00acebf5, 4);
                    if (t != (unsigned int)iVar1) {
                        __builtin_memcpy(&t, piVar5 + 4, 4);
                        if (t != 0) {
                            uVar2 = FUN_004d23b0(iVar1);
                            __builtin_memcpy(&t, DAT_00b69a89, 4);
                            if (uVar2 < t) {
                                DVar3 = GetTickCount();
                                __builtin_memcpy(&t, DAT_00b641c5 + uVar2 * 0x238, 4);
                                if (45000 < DVar3 - t) {
                                    DVar3 = GetTickCount();
                                    __builtin_memcpy(DAT_00b641c5 + uVar2 * 0x238, &DVar3, 4);
                                    __builtin_memcpy(&cnt, DAT_00b53579, 4);
                                    if (cnt < 0x400) {
                                        local_8[0] = 0x5b;
                                        __builtin_memcpy(&t2, piVar5, 4);
                                        FUN_004cef70((int)t2, local_8, 4);
                                        __builtin_memcpy(&idx, DAT_00b53571, 4);
                                        e = DAT_00acf571 + idx * 0x210;
                                        v = 0x47;
                                        __builtin_memcpy(e + 0x10, &v, 4);
                                        __builtin_memcpy(&t2, piVar5, 4);
                                        __builtin_memcpy(e, &t2, 4);
                                        v = 4;
                                        __builtin_memcpy(e + 4, &v, 4);
                                        v = 0;
                                        __builtin_memcpy(e + 8, &v, 4);
                                        e[0xc] = 0;
                                        e[0xf] = 0;
                                        e[0xd] = 1;
                                        __builtin_memcpy(&t, DAT_00b63f99 + uVar2 * 0x238, 4);
                                        uVar4 = t + 1;
                                        if (0x7f < uVar4) {
                                            uVar4 = 0;
                                        }
                                        __builtin_memcpy(DAT_00b63f99 + uVar2 * 0x238, &uVar4, 4);
                                        e[0xe] = (unsigned char)uVar4;
                                        __builtin_memcpy(&cnt, DAT_00b53579, 4);
                                        cnt = cnt + 1;
                                        __builtin_memcpy(DAT_00b53579, &cnt, 4);
                                        __builtin_memcpy(&idx, DAT_00b53571, 4);
                                        idx = idx + 1;
                                        __builtin_memcpy(DAT_00b53571, &idx, 4);
                                        if (0x3ff < idx) {
                                            idx = 0;
                                            __builtin_memcpy(DAT_00b53571, &idx, 4);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            piVar5 = piVar5 + 40;
        } while (piVar5 < DAT_00acf145);
    }
    return 1;
}

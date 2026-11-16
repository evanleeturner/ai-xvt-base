/* SPDX-License-Identifier: MIT */
/* z_bop 004e6310 */
extern unsigned int DAT_00aa6af0[];

extern unsigned char DAT_00b6a5c6[];
extern unsigned char DAT_00b6a5aa[];
extern unsigned char DAT_00b6a5ae[];
extern unsigned char DAT_00bb0106[];
extern unsigned char DAT_00bb0112[];
extern unsigned char DAT_00bb011e[];
extern unsigned char DAT_00bb10a6[];
extern unsigned char DAT_00bb10b2[];
extern unsigned char DAT_00bb10be[];
extern unsigned char DAT_00aa615e[];
extern unsigned char DAT_00aa6166[];
extern unsigned char DAT_00aa62ae[];
extern unsigned char DAT_00aa62b4[];

extern int DAT_00b6a2a8;
extern int DAT_00b69e30;
extern int DAT_00aa6114;
extern int DAT_00aa6134;

extern unsigned char DAT_00bb7394, DAT_00bb7395, DAT_00bb7396, DAT_00bb7397;
extern unsigned char DAT_00bb7398, DAT_00bb7399, DAT_00bb739a, DAT_00bb739b;
extern unsigned char DAT_00bb739c, DAT_00bb739d, DAT_00bb739e, DAT_00bb739f;
extern unsigned char DAT_00bb736f, DAT_00bb73a4, DAT_00bb7370, DAT_00bb73a5;
extern unsigned char DAT_00bb73a6;

int *FUN_004cfe80(int *param_1);
unsigned int FUN_004d2250(int param_1);
void FUN_004cef70(int param_1, unsigned int *param_2, unsigned int param_3);
unsigned int FUN_004d1400(void);
int __cdecl _rand(void);

void __cdecl FUN_004e6310(int param_1)
{
    unsigned int iVar1;
    unsigned int iVar2;
    unsigned int uVar5;
    int iVar6;
    int iVar7;
    int *puVar4;
    int *piVar9;
    int *piVar10;
    unsigned char *pbVar10;
    unsigned char *pbVar4;
    int local_4;
    unsigned int t0;
    unsigned int t1;
    unsigned int t2;
    unsigned int t3;
    unsigned int t4;
    unsigned int idx;
    int lim;
    int cmpv;
    unsigned int *buf = DAT_00aa6af0;

    buf[0] = 0x41;
    for (iVar6 = 0; iVar6 < 8; iVar6++) {
        __builtin_memcpy(&t0, DAT_00b6a5c6 + iVar6 * 4, 4);
        buf[1 + iVar6] = t0;
    }
    __builtin_memcpy(&t0, DAT_00b6a5aa, 4);
    buf[10] = t0;
    __builtin_memcpy(&t1, DAT_00b6a5ae + t0 * 4u, 4);
    buf[11] = t1;

    if (DAT_00b6a2a8 == 0) {
        buf[12] = 8;
        iVar7 = 0xe;
        buf[13] = FUN_004d1400();
        iVar6 = 0;
        puVar4 = FUN_004cfe80(&local_4);
        if (0 < local_4) {
            piVar10 = puVar4 + 8;
            do {
                if (piVar10[1] == 1) {
                    buf[iVar7] = (unsigned int)*piVar10;
                    iVar7 = iVar7 + 3;
                    buf[iVar7 - 2] = (unsigned int)(*(unsigned char *)(piVar10 - 8) - 1);
                    uVar5 = FUN_004d2250(*piVar10);
                    buf[iVar7 - 1] = uVar5;
                }
                piVar10 = piVar10 + 10;
                iVar6 = iVar6 + 1;
            } while (iVar6 < local_4);
        }
        FUN_004cef70(param_1, buf, (unsigned int)iVar7 << 2);
        __builtin_memcpy(&cmpv, DAT_00b6a5aa, 4);
        if (cmpv != 4) goto LAB_004e6652;
        iVar6 = 0;
        buf[5] = 0;
        buf[0] = 0x78;
        buf[4] = 0;
        if (DAT_00b69e30 == 2) {
            piVar10 = (int *)(DAT_00aa6114 + 0x140 + DAT_00aa6134 * 0x148);
            idx = (unsigned int)*piVar10;
            __builtin_memcpy(&t0, DAT_00bb0106 + idx * 0xa0u, 4);
            buf[1] = t0;
            __builtin_memcpy(&t1, DAT_00bb0112 + idx * 0xa0u, 4);
            buf[3] = t1;
            lim = (int)t1;
            if (-1 < lim) {
                do {
                    if (9 < iVar6) break;
                    __builtin_memcpy(&t2, DAT_00bb011e + ((unsigned int)iVar6 + idx * 0x28u) * 4u, 4);
                    if ((int)t2 == 0) {
                        buf[5] = buf[5] + 1;
                    } else if ((int)t2 == 1) {
                        buf[4] = buf[4] + 1;
                    }
                    iVar6 = iVar6 + 1;
                    __builtin_memcpy(&lim, DAT_00bb0112 + idx * 0xa0u, 4);
                } while (iVar6 <= lim);
            }
        } else {
            piVar10 = (int *)(DAT_00aa6114 + 0x140 + DAT_00aa6134 * 0x148);
            idx = (unsigned int)*piVar10;
            __builtin_memcpy(&t0, DAT_00bb10a6 + idx * 0xa0u, 4);
            buf[1] = t0;
            __builtin_memcpy(&t1, DAT_00bb10b2 + idx * 0xa0u, 4);
            buf[3] = t1;
            iVar6 = 0;
            lim = (int)t1;
            if (-1 < lim) {
                do {
                    if (9 < iVar6) break;
                    __builtin_memcpy(&t2, DAT_00bb10be + ((unsigned int)iVar6 + idx * 0x28u) * 4u, 4);
                    if ((int)t2 == 0) {
                        buf[5] = buf[5] + 1;
                    } else if ((int)t2 == 1) {
                        buf[4] = buf[4] + 1;
                    }
                    iVar6 = iVar6 + 1;
                    __builtin_memcpy(&lim, DAT_00bb10b2 + idx * 0xa0u, 4);
                } while (iVar6 <= lim);
            }
        }
        buf[2] = (unsigned int)DAT_00bb73a6;
        uVar5 = 0x18;
    } else {
        iVar6 = 0xe;
        pbVar10 = DAT_00aa615e;
        buf[12] = 8;
        buf[13] = 8;
        puVar4 = FUN_004cfe80(&local_4);
        do {
            iVar7 = 0;
            if (0 < local_4) {
                piVar9 = puVar4 + 8;
                do {
                    __builtin_memcpy(&t0, pbVar10, 4);
                    if (((*piVar9 != 0) && (piVar9[1] != 0)) && ((int)t0 == *piVar9)) {
                        t1 = (unsigned int)(*(unsigned char *)(puVar4 + iVar7 * 10) - 1);
                        __builtin_memcpy(pbVar10 + 4, &t1, 4);
                        break;
                    }
                    piVar9 = piVar9 + 10;
                    iVar7 = iVar7 + 1;
                } while (iVar7 < local_4);
            }
            if (iVar7 == local_4) {
                t1 = 0;
                __builtin_memcpy(pbVar10, &t1, 4);
            }
            pbVar10 = pbVar10 + 0x2a;
        } while (pbVar10 < DAT_00aa62ae);
        pbVar10 = DAT_00aa615e;
        do {
            __builtin_memcpy(&iVar1, pbVar10, 4);
            __builtin_memcpy(&iVar2, pbVar10 + 4, 4);
            buf[iVar6] = iVar1;
            buf[iVar6 + 1] = iVar2;
            iVar6 = iVar6 + 3;
            pbVar10 = pbVar10 + 0x2a;
            uVar5 = FUN_004d2250((int)iVar1);
            buf[iVar6 - 1] = uVar5;
        } while (pbVar10 < DAT_00aa62ae);
        FUN_004cef70(param_1, buf, (unsigned int)iVar6 * 4u);
        iVar6 = 1;
        buf[0] = 0x71;
        pbVar4 = DAT_00aa6166;
        do {
            __builtin_memcpy(&t0, pbVar4, 4);
            __builtin_memcpy(&t1, pbVar4 + 4, 4);
            buf[iVar6] = t0;
            __builtin_memcpy(&t2, pbVar4 + 8, 4);
            buf[iVar6 + 1] = t1;
            iVar6 = iVar6 + 5;
            __builtin_memcpy(&t3, pbVar4 + 12, 4);
            buf[iVar6 - 3] = t2;
            __builtin_memcpy(&t4, pbVar4 + 16, 4);
            buf[iVar6 - 2] = t3;
            buf[iVar6 - 1] = t4;
            pbVar4 = pbVar4 + 0x2a;
        } while (pbVar4 < DAT_00aa62b4 + 2);
        uVar5 = (unsigned int)iVar6 * 4u;
    }
    FUN_004cef70(param_1, buf, uVar5);
LAB_004e6652:
    buf[0] = 0x59;
    buf[1] = (unsigned int)DAT_00bb7394;
    buf[2] = (unsigned int)DAT_00bb7395;
    buf[3] = (unsigned int)DAT_00bb7396;
    buf[4] = (unsigned int)DAT_00bb7397;
    buf[5] = (unsigned int)DAT_00bb7398;
    buf[6] = (unsigned int)DAT_00bb7399;
    buf[7] = (unsigned int)DAT_00bb739a;
    buf[8] = (unsigned int)DAT_00bb739b;
    buf[9] = (unsigned int)DAT_00bb739c;
    buf[10] = (unsigned int)DAT_00bb739d;
    buf[11] = (unsigned int)DAT_00bb739e;
    buf[12] = (unsigned int)DAT_00bb739f;
    buf[13] = (unsigned int)_rand();
    buf[14] = (unsigned int)DAT_00bb736f;
    buf[15] = (unsigned int)DAT_00bb73a4;
    buf[16] = (unsigned int)DAT_00bb7370;
    buf[17] = (unsigned int)DAT_00bb73a5;
    buf[18] = (unsigned int)DAT_00bb73a6;
    FUN_004cef70(param_1, buf, 0x4c);
    return;
}

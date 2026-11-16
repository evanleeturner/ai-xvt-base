/* SPDX-License-Identifier: MIT */
/* z_bop 0041b830 */
#define NULL ((void *)0)

typedef unsigned long DWORD;
void __stdcall RaiseException(DWORD, DWORD, DWORD, const DWORD *);

extern unsigned char *DAT_009a1fe8;
extern unsigned char DAT_00521f1c[];

int __cdecl FUN_0041f410(int, int, int);
int __cdecl FUN_0041ba60(unsigned int);

void __cdecl FUN_0041b830(unsigned char *param_1, unsigned int param_2, unsigned int param_3)
{
    unsigned char bVar1;
    int iVar2;
    int iVar3;
    unsigned char *pbVar4;
    unsigned int uVar5;
    unsigned char *pbVar6;
    int iVar7;
    unsigned int uVar8;
    int t0, t1, t2, t3, t4, t5;
    int tI;
    short tS;

    iVar7 = (int)((param_2 & 0xffff) * 0x23);
    iVar2 = (int)((param_3 & 0xffff) * 0x23);

    __builtin_memcpy(&t0, DAT_009a1fe8 + iVar7 + 4, 4);
    __builtin_memcpy(&t1, DAT_009a1fe8 + iVar2 + 4, 4);
    t0 = (int)((unsigned int)t0 - (unsigned int)t1);
    __builtin_memcpy(&t2, DAT_009a1fe8 + iVar7 + 8, 4);
    __builtin_memcpy(&t3, DAT_009a1fe8 + iVar2 + 8, 4);
    t2 = (int)((unsigned int)t2 - (unsigned int)t3);
    __builtin_memcpy(&t4, DAT_009a1fe8 + iVar7 + 0xc, 4);
    __builtin_memcpy(&t5, DAT_009a1fe8 + iVar2 + 0xc, 4);
    t4 = (int)((unsigned int)t4 - (unsigned int)t5);

    iVar3 = FUN_0041f410(t0, t2, t4);

    __builtin_memcpy(&t1, DAT_00521f1c + (unsigned int)DAT_009a1fe8[3 + iVar7] * 0x1a, 4);
    __builtin_memcpy(&t3, DAT_00521f1c + (unsigned int)DAT_009a1fe8[3 + iVar2] * 0x1a, 4);
    iVar7 = (int)(((unsigned int)iVar3 - (unsigned int)t1) - (unsigned int)t3);

    if (iVar7 < 0) {
        iVar2 = 0;
    }
    else {
        iVar2 = FUN_0041ba60(param_3);
        iVar3 = FUN_0041ba60(param_2);
        iVar2 = (int)((unsigned int)iVar2 + (unsigned int)iVar3) >> 8;
        if (iVar2 == 0) {
            return;
        }
        iVar3 = (int)((unsigned int)(iVar7 >> 8) * 0x33dbU);
        if (iVar3 == (int)0x80000000 && iVar2 == -1) {
            RaiseException(0xC0000094UL, 0, 0, NULL);
        }
        iVar2 = iVar3 / iVar2;
    }

    uVar8 = 0;
    bVar1 = *param_1;
    if (bVar1 != 0) {
        pbVar6 = param_1 + 1;
        pbVar4 = param_1 + 0x41;
        do {
            __builtin_memcpy(&tS, pbVar4, 2);
            if (tS == (short)param_3) {
                uVar5 = uVar8 + 1;
                __builtin_memcpy(param_1 + uVar8 * 4 + 1, &iVar2, 4);
                if ((int)(unsigned int)*param_1 <= (int)uVar5) goto LAB_0041b9ae;
                pbVar6 = param_1 + uVar5 * 2 + 0x41;
                pbVar4 = param_1 + uVar5 * 4 + 1;
                goto LAB_0041b981;
            }
            __builtin_memcpy(&tI, pbVar6, 4);
            if (iVar2 < tI) break;
            pbVar6 = pbVar6 + 4;
            pbVar4 = pbVar4 + 2;
            uVar8 = uVar8 + 1;
        } while ((int)uVar8 < (int)(unsigned int)bVar1);
    }
    uVar5 = (unsigned int)bVar1;
    if (uVar8 != uVar5) {
        if (uVar5 == 0x10) {
            __builtin_memcpy(&t0, param_1 + 0x3d, 4);
            __builtin_memcpy(&t1, param_1 + 0x61, 4);
            if (t0 < t1) {
                __builtin_memcpy(param_1 + 0x61, &t0, 4);
            }
            uVar5 = 0xf;
            *param_1 = (unsigned char)(bVar1 - 1);
        }
        if ((int)uVar8 < (int)uVar5) {
            pbVar6 = param_1 + uVar5 * 4 + 1;
            iVar7 = (int)(uVar5 - uVar8);
            pbVar4 = param_1 + uVar5 * 2 + 0x41;
            do {
                __builtin_memcpy(&tI, pbVar6 - 4, 4);
                __builtin_memcpy(pbVar6, &tI, 4);
                pbVar6 = pbVar6 - 4;
                iVar7 = iVar7 - 1;
                __builtin_memcpy(&tS, pbVar4 - 2, 2);
                __builtin_memcpy(pbVar4, &tS, 2);
                pbVar4 = pbVar4 - 2;
            } while (iVar7 != 0);
        }
        __builtin_memcpy(param_1 + uVar8 * 4 + 1, &iVar2, 4);
        tS = (short)param_3;
        __builtin_memcpy(param_1 + uVar8 * 2 + 0x41, &tS, 2);
        *param_1 = (unsigned char)(*param_1 + 1);
        return;
    }
    if (bVar1 == 0x10) {
        __builtin_memcpy(&tI, param_1 + 0x61, 4);
        if (tI <= iVar2) {
            return;
        }
        __builtin_memcpy(param_1 + 0x61, &iVar2, 4);
        return;
    }
    __builtin_memcpy(param_1 + uVar8 * 4 + 1, &iVar2, 4);
    tS = (short)param_3;
    __builtin_memcpy(param_1 + uVar8 * 2 + 0x41, &tS, 2);
    *param_1 = (unsigned char)(*param_1 + 1);
    return;

LAB_0041b981:
    for (;;) {
        __builtin_memcpy(&tI, pbVar4, 4);
        if (iVar2 <= tI) break;
        __builtin_memcpy(pbVar4 - 4, &tI, 4);
        __builtin_memcpy(&tS, pbVar6, 2);
        __builtin_memcpy(pbVar6 - 2, &tS, 2);
        uVar5 = uVar5 + 1;
        __builtin_memcpy(pbVar4, &iVar2, 4);
        tS = (short)param_3;
        __builtin_memcpy(pbVar6, &tS, 2);
        pbVar6 = pbVar6 + 2;
        pbVar4 = pbVar4 + 4;
        if ((unsigned int)*param_1 == uVar5) break;
    }
    uVar8 = uVar5 - 1;

LAB_0041b9ae:
    if (0 < (int)uVar8) {
        iVar7 = (int)(uVar8 - 1);
        pbVar6 = param_1 + uVar8 * 2 + 0x3f;
        pbVar4 = param_1 + uVar8 * 4 - 3;
        for (;;) {
            __builtin_memcpy(&tI, pbVar4, 4);
            if (!(iVar2 < tI)) break;
            __builtin_memcpy(pbVar4 + 4, &tI, 4);
            __builtin_memcpy(&tS, pbVar6, 2);
            __builtin_memcpy(pbVar6 + 2, &tS, 2);
            iVar7 = iVar7 - 1;
            __builtin_memcpy(pbVar4, &iVar2, 4);
            tS = (short)param_3;
            __builtin_memcpy(pbVar6, &tS, 2);
            pbVar6 = pbVar6 - 2;
            pbVar4 = pbVar4 - 4;
            if (iVar7 < 0) {
                return;
            }
        }
    }
    return;
}

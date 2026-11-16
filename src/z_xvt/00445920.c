/* SPDX-License-Identifier: MIT */
/* z_xvt 00445920 */
extern unsigned int DAT_006d4988;
extern unsigned short DAT_006d9d34;
extern unsigned char DAT_0052b08f[];
extern unsigned char DAT_0052b091[];
extern unsigned short DAT_0071e6be;

unsigned int __cdecl FUN_00478320(unsigned int a, unsigned short b);

void __cdecl FUN_00445920(int param_1, int param_2, int param_3, unsigned short param_4)
{
    unsigned int iVar5;
    unsigned int iVar3;
    unsigned int uVar9;
    unsigned int uVar6;
    unsigned int uVar7;
    unsigned int uVar2;
    unsigned int uVar4;
    unsigned int prod;
    unsigned int rem;
    unsigned int tmp;
    unsigned short w;
    unsigned short val;
    unsigned short ua;
    unsigned short ub;
    unsigned short s;
    unsigned short f;

    iVar5 = (unsigned int)param_1 * 0x23;
    __builtin_memcpy(&iVar3, (unsigned char *)(DAT_006d4988 + 0x1f + iVar5), 4);
    __builtin_memcpy(&w, (unsigned char *)(iVar3 + 0x79), 2);
    uVar9 = (unsigned int)param_2 - (unsigned int)w;
    if (uVar9 != 0) {
        if (uVar9 < 0x8000) {
            __builtin_memcpy(&val, &DAT_0052b08f[(unsigned int)DAT_006d9d34 * 0xef], 2);
            uVar6 = FUN_00478320((((unsigned int)DAT_006d9d34 * 0xef >> 16) << 16) | (unsigned int)val, 0x4000);
            uVar6 = uVar6 & 0xffff;
            if (uVar6 == 0) {
                uVar6 = 1;
            }
            __builtin_memcpy(&val, &DAT_0052b08f[(unsigned int)DAT_006d9d34 * 0xef], 2);
            uVar7 = FUN_00478320((unsigned int)(unsigned short)(val - (unsigned short)uVar6), param_4);
            uVar6 = uVar6 + (uVar7 & 0xffff);
            __builtin_memcpy(&iVar3, (unsigned char *)(DAT_006d4988 + 0x1f + iVar5), 4);
            __builtin_memcpy(&tmp, (unsigned char *)(iVar3 + 0xa7), 4);
            __builtin_memcpy(&s, (unsigned char *)(tmp + 0xbf), 2);
            if (s == 0) {
                uVar6 = uVar6 * 3;
            }
            if (uVar6 <= uVar9) {
                uVar9 = uVar6;
            }
            __builtin_memcpy(&ua, (unsigned char *)(iVar3 + 0x7b), 2);
            uVar2 = ua;
            prod = (unsigned int)DAT_0071e6be * uVar9;
            uVar4 = (unsigned int)((unsigned long long)prod / 0xec);
            rem = uVar4 * (unsigned int)-0xec + prod;
            f = (unsigned short)((unsigned long long)(rem * 0x10000u) / 0xec);
            ub = (unsigned short)(f + (unsigned short)uVar2);
            __builtin_memcpy((unsigned char *)(iVar3 + 0x7b), &ub, 2);
            __builtin_memcpy(&iVar3, (unsigned char *)(DAT_006d4988 + 0x1f + iVar5), 4);
            __builtin_memcpy(&ub, (unsigned char *)(iVar3 + 0x7b), 2);
            if (ub < (unsigned short)uVar2) {
                __builtin_memcpy(&s, (unsigned char *)(iVar3 + 0x79), 2);
                s = (unsigned short)(s + 1);
                __builtin_memcpy((unsigned char *)(iVar3 + 0x79), &s, 2);
            }
            __builtin_memcpy(&iVar3, (unsigned char *)(DAT_006d4988 + 0x1f + iVar5), 4);
            __builtin_memcpy(&s, (unsigned char *)(iVar3 + 0x79), 2);
            s = (unsigned short)(s + (unsigned short)uVar4);
            __builtin_memcpy((unsigned char *)(iVar3 + 0x79), &s, 2);
            __builtin_memcpy(&iVar3, (unsigned char *)(DAT_006d4988 + 0x1f + iVar5), 4);
            __builtin_memcpy(&s, (unsigned char *)(iVar3 + 0x79), 2);
            if (0xe10 < s) {
                s = 0xe10;
                __builtin_memcpy((unsigned char *)(iVar3 + 0x79), &s, 2);
                return;
            }
        } else if (param_3 == 1) {
            __builtin_memcpy(&val, &DAT_0052b091[(unsigned int)DAT_006d9d34 * 0xef], 2);
            uVar6 = FUN_00478320((((unsigned int)DAT_006d9d34 * 0xef >> 16) << 16) | (unsigned int)val, 0x4000);
            uVar6 = uVar6 & 0xffff;
            if (uVar6 == 0) {
                uVar6 = 1;
            }
            __builtin_memcpy(&val, &DAT_0052b091[(unsigned int)DAT_006d9d34 * 0xef], 2);
            uVar7 = FUN_00478320((((unsigned int)DAT_006d9d34 * 0xef >> 16) << 16) |
                                 (unsigned int)(unsigned short)(val - (unsigned short)uVar6),
                                 (unsigned short)((unsigned int)(-(int)param_4) - 1u));
            uVar6 = uVar6 + (uVar7 & 0xffff);
            uVar7 = (unsigned int)0 - uVar9;
            if (uVar6 <= (unsigned int)0 - uVar9) {
                uVar7 = uVar6;
            }
            prod = (unsigned int)DAT_0071e6be * uVar7;
            uVar4 = (unsigned int)((unsigned long long)prod / 0xec);
            __builtin_memcpy(&iVar3, (unsigned char *)(DAT_006d4988 + 0x1f + iVar5), 4);
            __builtin_memcpy(&ua, (unsigned char *)(iVar3 + 0x7b), 2);
            uVar2 = ua;
            rem = uVar4 * (unsigned int)-0xec + prod;
            f = (unsigned short)((unsigned long long)(rem * 0x10000u) / 0xec);
            ub = (unsigned short)((unsigned short)uVar2 - f);
            __builtin_memcpy((unsigned char *)(iVar3 + 0x7b), &ub, 2);
            __builtin_memcpy(&iVar3, (unsigned char *)(DAT_006d4988 + 0x1f + iVar5), 4);
            __builtin_memcpy(&ub, (unsigned char *)(iVar3 + 0x7b), 2);
            if ((unsigned short)uVar2 < ub) {
                __builtin_memcpy(&s, (unsigned char *)(iVar3 + 0x79), 2);
                s = (unsigned short)(s - 1);
                __builtin_memcpy((unsigned char *)(iVar3 + 0x79), &s, 2);
            }
            __builtin_memcpy(&iVar3, (unsigned char *)(DAT_006d4988 + 0x1f + iVar5), 4);
            __builtin_memcpy(&s, (unsigned char *)(iVar3 + 0x79), 2);
            s = (unsigned short)(s - (unsigned short)uVar4);
            __builtin_memcpy((unsigned char *)(iVar3 + 0x79), &s, 2);
            __builtin_memcpy(&iVar3, (unsigned char *)(DAT_006d4988 + 0x1f + iVar5), 4);
            __builtin_memcpy(&s, (unsigned char *)(iVar3 + 0x79), 2);
            if (0x8000 < s) {
                s = 0;
                __builtin_memcpy((unsigned char *)(iVar3 + 0x79), &s, 2);
            }
        }
    }
    return;
}

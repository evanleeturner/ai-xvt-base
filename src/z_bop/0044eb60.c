/* SPDX-License-Identifier: MIT */
/* z_bop 0044eb60 */
#define RD(v, a) __builtin_memcpy(&(v), (const void *)(a), sizeof(v))
#define WR(a, v) __builtin_memcpy((void *)(a), &(v), sizeof(v))
#define ENT(idx) ((unsigned int)(idx) * 0x23U + (unsigned int)DAT_009a1fe8)

extern unsigned char DAT_009e9683[];
extern unsigned char DAT_009e9684[];
extern unsigned char DAT_009e9685[];
extern unsigned char DAT_009e9670[];
extern unsigned char DAT_009e9a67[];
extern unsigned char DAT_009e9a5b[];
extern unsigned char DAT_009e9a5f[];
extern unsigned char DAT_009e9a63[];
extern unsigned char DAT_009e9a6b[];
extern unsigned char DAT_009e9a6d[];
extern unsigned char DAT_009e9a6f[];
extern unsigned char DAT_009e9a71[];
extern unsigned char DAT_009e9a73[];
extern unsigned char DAT_009e9a75[];
extern unsigned char DAT_009e9a80[];
extern unsigned char DAT_009e9a84[];
extern unsigned char DAT_009e9a86[];
extern unsigned char DAT_009e9a8a[];
extern unsigned char DAT_009e96c5[];
extern unsigned char DAT_009e96c9[];
extern unsigned char DAT_009e96cd[];
extern unsigned char DAT_00521f1c[];
extern int DAT_009a1fe8;
extern int DAT_009a8d9c;
extern short DAT_009a8070;
extern short DAT_00a080f0;
extern int DAT_00a004b0;
extern int DAT_009fe7f4;
extern int DAT_00a00490;
extern int DAT_00a07cd4;
extern int DAT_00a07cd8;
extern int DAT_00a07cdc;

void __cdecl FUN_00435c60(int param_1);
void __cdecl FUN_0046a8d0(int param_1, int param_2, int param_3);
void __cdecl FUN_00427940(short param_1, short param_2, short param_3, short param_4,
                          short param_5, short param_6, unsigned int param_7);
void __cdecl FUN_00458e20(unsigned int param_1, int param_2);
void __cdecl FUN_00442bf0(unsigned int param_1, short param_2, int param_3);
void __cdecl FUN_00427720(int param_1, int param_2);

void __thiscall FUN_0044eb60(void *this_ptr, int param_1)
{
    unsigned short uVar1;
    long long lVar2;
    int iVar3;
    int uVar4;
    int uVar5;
    short uVar6;
    unsigned int uVar7;
    unsigned int uVar8;
    unsigned char bVar9;
    short sVar10;
    int iVar11;
    int iVar12;
    unsigned int local_20;
    int a, b, c;
    short s0, s1, s2, s3;
    unsigned short us;
    unsigned int ui;
    unsigned int tmp;
    short zero16 = 0;
    short one16 = 1;
    unsigned short ffff = 0xffff;

    iVar3 = (int)((unsigned int)param_1 * 0x5bdU);
    if (DAT_009e9683[iVar3] != '\0') {
        FUN_00435c60(param_1);
        goto LAB_0044f053;
    }
    RD(uVar1, DAT_009e9a67 + iVar3);
    if (uVar1 == 0xffff) {
        RD(iVar12, DAT_009e9670 + iVar3);
        iVar12 = (int)((unsigned int)iVar12 * 0x23U + (unsigned int)DAT_009a1fe8);
        RD(a, iVar12 + 4);
        RD(b, DAT_009e9a5b + iVar3);
        RD(c, iVar12 + 8);
        {
            int d;
            int e;
            int f;
            RD(d, DAT_009e9a5f + iVar3);
            RD(e, iVar12 + 0xc);
            RD(f, DAT_009e9a63 + iVar3);
            FUN_0046a8d0((int)((unsigned int)a - (unsigned int)b),
                         (int)((unsigned int)c - (unsigned int)d),
                         (int)((unsigned int)e - (unsigned int)f));
        }
        uVar6 = DAT_00a080f0;
        WR(DAT_009e9a6f + iVar3, zero16);
        WR(DAT_009e9a6b + iVar3, uVar6);
        sVar10 = DAT_009a8070;
        WR(DAT_009e9a6d + iVar3, sVar10);
        RD(s0, DAT_009e9a6f + iVar3);
        RD(s1, DAT_009e9a6b + iVar3);
        RD(s2, DAT_009e9a73 + iVar3);
        RD(s3, DAT_009e9a75 + iVar3);
        FUN_00427940(s0, s1, sVar10, 0, s2, s3, 0);
        goto LAB_0044f053;
    }
    {
        short v84, v8a, v80;
        RD(v84, DAT_009e9a84 + iVar3);
        RD(v8a, DAT_009e9a8a + iVar3);
        if (v84 == 0 || v8a != 0) {
            if (v8a != 0) {
                RD(v80, DAT_009e9a80 + iVar3);
                if (v80 != 0) goto LAB_0044ec71;
                if (v8a != 0) {
                    FUN_00442bf0((((unsigned int)this_ptr >> 16) << 16) | (unsigned int)uVar1, 0, param_1);
                    goto LAB_0044f053;
                }
            }
            RD(sVar10, ENT(uVar1) + 0x14);
            WR(DAT_009e9a6f + iVar3, sVar10);
            RD(us, DAT_009e9a67 + iVar3);
            RD(sVar10, ENT(us) + 0x12);
            WR(DAT_009e9a6b + iVar3, sVar10);
            RD(us, DAT_009e9a67 + iVar3);
            RD(sVar10, ENT(us) + 0x10);
            WR(DAT_009e9a6d + iVar3, sVar10);
            RD(s0, DAT_009e9a6f + iVar3);
            RD(s1, DAT_009e9a6b + iVar3);
            RD(s2, DAT_009e9a71 + iVar3);
            RD(s3, DAT_009e9a73 + iVar3);
            {
                short s4;
                RD(s4, DAT_009e9a75 + iVar3);
                RD(us, DAT_009e9a67 + iVar3);
                FUN_00427940(s0, s1, sVar10, s2, s3, s4, ENT(us));
            }
            RD(us, DAT_009e9a67 + iVar3);
            RD(a, ENT(us) + 4);
            WR(DAT_009e9a5b + iVar3, a);
            RD(us, DAT_009e9a67 + iVar3);
            RD(a, ENT(us) + 8);
            WR(DAT_009e9a5f + iVar3, a);
            RD(us, DAT_009e9a67 + iVar3);
            RD(a, ENT(us) + 0xc);
            WR(DAT_009e9a63 + iVar3, a);
            RD(us, DAT_009e9a67 + iVar3);
            RD(a, ENT(us) + 0x1b);
            if (a == param_1) {
                RD(a, DAT_009e9a5b + iVar3);
                RD(b, DAT_009e96c5 + iVar3);
                a = (int)((unsigned int)a + (unsigned int)b);
                WR(DAT_009e9a5b + iVar3, a);
                RD(a, DAT_009e9a5f + iVar3);
                RD(b, DAT_009e96c9 + iVar3);
                a = (int)((unsigned int)a + (unsigned int)b);
                WR(DAT_009e9a5f + iVar3, a);
                RD(a, DAT_009e9a63 + iVar3);
                RD(b, DAT_009e96cd + iVar3);
                a = (int)((unsigned int)a + (unsigned int)b);
                WR(DAT_009e9a63 + iVar3, a);
            }
        } else {
LAB_0044ec71:
            RD(sVar10, ENT(uVar1) + 0x14);
            WR(DAT_009e9a6f + iVar3, sVar10);
            RD(us, DAT_009e9a67 + iVar3);
            RD(sVar10, ENT(us) + 0x12);
            WR(DAT_009e9a6b + iVar3, sVar10);
            RD(us, DAT_009e9a67 + iVar3);
            RD(sVar10, ENT(us) + 0x10);
            WR(DAT_009e9a6d + iVar3, sVar10);
            RD(s0, DAT_009e9a6f + iVar3);
            RD(s1, DAT_009e9a6b + iVar3);
            RD(s2, DAT_009e9a73 + iVar3);
            RD(s3, DAT_009e9a75 + iVar3);
            FUN_00427940(s0, s1, sVar10, 0, s2, s3, 0);
            RD(us, DAT_009e9a67 + iVar3);
            FUN_00458e20((unsigned int)us, 0);
            uVar5 = DAT_00a07cd8;
            uVar4 = DAT_00a07cd4;
            a = DAT_00a07cdc;
            WR(DAT_009e9a5b + iVar3, a);
            iVar11 = DAT_00a004b0;
            WR(DAT_009e9a5f + iVar3, uVar5);
            WR(DAT_009e9a63 + iVar3, uVar4);
            iVar12 = DAT_009fe7f4;
            RD(a, DAT_009e9a86 + iVar3);
            lVar2 = (long long)a * (long long)iVar11;
            RD(b, DAT_009e9a5b + iVar3);
            tmp = (unsigned int)b - ((unsigned int)lVar2 >> 0xf | (unsigned int)((unsigned long long)lVar2 >> 0x20) << 0x11);
            WR(DAT_009e9a5b + iVar3, tmp);
            iVar11 = DAT_00a00490;
            RD(a, DAT_009e9a86 + iVar3);
            lVar2 = (long long)a * (long long)iVar12;
            RD(b, DAT_009e9a5f + iVar3);
            tmp = (unsigned int)b - ((unsigned int)lVar2 >> 0xf | (unsigned int)((unsigned long long)lVar2 >> 0x20) << 0x11);
            WR(DAT_009e9a5f + iVar3, tmp);
            RD(a, DAT_009e9a86 + iVar3);
            lVar2 = (long long)a * (long long)iVar11;
            RD(b, DAT_009e9a63 + iVar3);
            tmp = (unsigned int)b - ((unsigned int)lVar2 >> 0xf | (unsigned int)((unsigned long long)lVar2 >> 0x20) << 0x11);
            WR(DAT_009e9a63 + iVar3, tmp);
            iVar12 = 0;
            RD(us, DAT_009e9a67 + iVar3);
            {
                unsigned char tb = *(unsigned char *)(ENT(us) + 3);
                RD(DAT_009a8d9c, DAT_00521f1c + (unsigned int)tb * 0x1a);
            }
            for (; 0x7fff < DAT_009a8d9c; DAT_009a8d9c = DAT_009a8d9c >> 1) {
                iVar12 = iVar12 + 1;
            }
            uVar7 = (unsigned int)((long long)DAT_009a8d9c * (long long)DAT_00a004b0) >> 0xf |
                    (unsigned int)((unsigned long long)((long long)DAT_009a8d9c * (long long)DAT_00a004b0) >> 0x20) << 0x11;
            local_20 = (unsigned int)((long long)DAT_009a8d9c * (long long)DAT_009fe7f4) >> 0xf |
                       (unsigned int)((unsigned long long)((long long)DAT_009a8d9c * (long long)DAT_009fe7f4) >> 0x20) << 0x11;
            uVar8 = (unsigned int)((long long)DAT_009a8d9c * (long long)DAT_00a00490) >> 0xf |
                    (unsigned int)((unsigned long long)((long long)DAT_009a8d9c * (long long)DAT_00a00490) >> 0x20) << 0x11;
            if (iVar12 != 0) {
                bVar9 = (unsigned char)iVar12;
                uVar7 = uVar7 << (bVar9 & 0x1f);
                local_20 = local_20 << (bVar9 & 0x1f);
                uVar8 = uVar8 << (bVar9 & 0x1f);
            }
            RD(a, DAT_009e9a5b + iVar3);
            tmp = (unsigned int)a - uVar7;
            WR(DAT_009e9a5b + iVar3, tmp);
            RD(a, DAT_009e9a5f + iVar3);
            tmp = (unsigned int)a - local_20;
            WR(DAT_009e9a5f + iVar3, tmp);
            RD(a, DAT_009e9a63 + iVar3);
            tmp = (unsigned int)a - uVar8;
            WR(DAT_009e9a63 + iVar3, tmp);
        }
    }
LAB_0044f053:
    RD(ui, DAT_009e9685 + iVar3);
    if (DAT_009e9684[iVar3] == '\x02' && 0x212 < ui) {
        RD(us, DAT_009e9a67 + iVar3);
        if ((unsigned int)us != 0xffffffffU) {
            WR(DAT_009e9a84 + iVar3, one16);
            FUN_00427720(0x12, param_1);
            WR(DAT_009e9a67 + iVar3, ffff);
        }
        RD(a, DAT_009e9670 + iVar3);
        if (a != -1) {
            sVar10 = 0;
            RD(ui, DAT_009e9685 + iVar3);
            if (0x24d < ui) {
                sVar10 = (short)((unsigned int)(short)ui * 8U - 0x1270U);
            }
            FUN_00427940(sVar10, 0x4000, 0, 0, 0, 0, 0);
            RD(a, DAT_009e9670 + iVar3);
            iVar12 = (int)((unsigned int)a * 0x23U + (unsigned int)DAT_009a1fe8);
            RD(b, iVar12 + 8);
            RD(c, DAT_00521f1c + (unsigned int)(*(unsigned char *)(iVar12 + 3)) * 0x1a);
            iVar11 = (int)((unsigned int)b - (unsigned int)c);
            WR(DAT_009e9a5f + iVar3, iVar11);
            RD(a, DAT_009e9685 + iVar3);
            iVar12 = (int)((unsigned int)a - 0x213U);
            if (0xec < iVar12) {
                iVar12 = 0xec;
            }
            tmp = (unsigned int)iVar11 - (unsigned int)iVar12 * (unsigned int)iVar12;
            WR(DAT_009e9a5f + iVar3, tmp);
        }
    }
    return;
}

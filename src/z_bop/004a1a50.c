/* SPDX-License-Identifier: MIT */
/* z_bop 004a1a50 */
extern unsigned short DAT_009a73b0;
extern unsigned short DAT_00a07cec;
extern int            DAT_009a1fe0;
extern unsigned int   DAT_009a1fe8;
extern unsigned char  DAT_009a73b6[4];
extern unsigned int   DAT_00a08104;
extern unsigned int   DAT_009a8c00;
extern unsigned int   DAT_009d8a58;
extern unsigned int   DAT_009d8a54;
extern unsigned int   DAT_009d8b60;
extern unsigned char  DAT_009d8cd4[];
extern unsigned short DAT_009a73c0;
extern unsigned short DAT_009a73c2;
extern unsigned char  DAT_00521f1c[];
extern short          DAT_00527768[];
extern short          DAT_005277a0[];
extern short          DAT_005277d8[];

void         FUN_004034e0(unsigned int a, unsigned int b);
unsigned int FUN_00404620(unsigned int a);
void         FUN_004a4cb0(unsigned int a);
unsigned int FUN_004a4d70(unsigned int a, unsigned int b);
void         FUN_004035d0(unsigned int a, short b, short c, short d);
void         FUN_004a44d0(short a, int b);

/* Unaligned access through __builtin_memcpy; d must be a 2/4-byte local. */
#define RD16(d, a) __builtin_memcpy(&(d), (const void *)(unsigned int)(a), 2)
#define RD32(d, a) __builtin_memcpy(&(d), (const void *)(unsigned int)(a), 4)
#define WR16(a, s) (w16 = (unsigned short)(s), __builtin_memcpy((void *)(unsigned int)(a), &w16, 2))
#define WR32(a, s) (w32 = (unsigned int)(s), __builtin_memcpy((void *)(unsigned int)(a), &w32, 4))

unsigned int __stdcall caseD_11(void)
{
    unsigned int psVar1;
    unsigned short uVar2;
    unsigned short uVar3;
    unsigned int iVar4;
    unsigned int iVar5;
    unsigned short uVar6;
    unsigned int iVar7;
    unsigned int uVar8;
    unsigned int uVar9;
    short sVar10;
    unsigned short uVar11;
    unsigned int pB;
    unsigned int t1;
    unsigned int t2;
    unsigned int t3;
    unsigned int t4;
    short s1;
    short s2;
    unsigned short us1;
    unsigned short w16;
    unsigned int w32;
    unsigned int doit;
    unsigned int bidx;

    uVar9 = (unsigned int)DAT_009a73b0;
    uVar8 = 0xff;
    for (uVar6 = DAT_00a07cec; (int)(unsigned int)uVar6 < DAT_009a1fe0; uVar6 = uVar6 + 1) {
        iVar7 = (unsigned int)uVar6 * 0x23 + DAT_009a1fe8;
        if (*(unsigned char *)(iVar7 + 3) != 0) {
            RD32(pB, DAT_009a73b6);
            if (*(unsigned char *)(iVar7 + 0x16) == *(unsigned char *)(pB + 0x2d)) {
                RD32(t1, iVar7 + 0x1f);
                RD32(t2, t1 + 0xa9);
                if (*(unsigned char *)(t2 + 5) == 0xff) {
                    uVar8 = (unsigned int)uVar6;
                    break;
                }
            }
        }
    }
    if ((short)uVar8 == 0xff) {
        FUN_004a44d0(0, 1);
        uVar8 = FUN_004a4d70((unsigned int)DAT_009a73b0, 0x8000);
        return uVar8 & 0xffff0000;
    }
    FUN_004034e0(uVar8, uVar9);
    iVar4 = uVar8 * 0x23;
    iVar5 = DAT_009a1fe8 + iVar4;
    RD32(t1, iVar5 + 0x1f);
    RD32(iVar7, t1 + 0xa9);
    uVar8 = 0x20000;
    RD32(t2, (unsigned int)DAT_00521f1c + (unsigned int)*(unsigned char *)(iVar5 + 3) * 0x1a);
    if ((int)t2 < 3000) {
        uVar8 = 0x8000;
    }
    RD16(s1, iVar7 + 0x1e5);
    if ((uVar8 < DAT_009a8c00) || (s1 == 0)) {
        RD32(pB, DAT_009a73b6);
        RD32(t1, iVar5 + 4);
        WR32(pB + 0x1f, t1);
        RD32(pB, DAT_009a73b6);
        RD32(t1, DAT_009a1fe8 + 8 + iVar4);
        WR32(pB + 0x23, t1);
        RD32(pB, DAT_009a73b6);
        RD32(t1, DAT_009a1fe8 + 0xc + iVar4);
        WR32(pB + 0x27, t1);
        RD16(s1, iVar7 + 0x1e5);
        if (s1 == 0) {
            sVar10 = 0x4000;
        } else {
            sVar10 = 0;
        }
        FUN_004a44d0(sVar10, 1);
        if ((int)DAT_009a8c00 < 0x10001) {
            uVar11 = 0x4000;
        } else {
            uVar11 = 0xffff;
        }
        FUN_004a4d70(uVar9, (unsigned int)uVar11);
        WR32(DAT_00a08104 + 0xb1, 0);
        WR32(DAT_00a08104 + 0xb5, 0);
        uVar8 = DAT_00a08104;
        WR32(DAT_00a08104 + 0xb9, 0);
        return uVar8 & 0xffff0000;
    }
    doit = 0;
    if (*(unsigned char *)(iVar7 + 0xab) == 2) {
        RD16(s1, iVar7 + 0x5c);
        RD32(pB, DAT_009a73b6);
        WR16(pB + 0x32, s1);
        doit = 1;
    } else {
        RD16(s1, DAT_009a1fe8 + 0x10 + uVar9 * 0x23);
        RD16(s2, iVar5 + 0x10);
        if (s1 != s2) {
            RD16(s1, iVar5 + 0x10);
            RD32(pB, DAT_009a73b6);
            WR16(pB + 0x32, s1);
            doit = 1;
        }
    }
    if (doit) {
        uVar8 = FUN_00404620(DAT_00a08104);
        FUN_004a4cb0((unsigned int)(((unsigned short)(uVar8 >> 3) & 0x1fff) + 0x4000));
    }
    RD32(t1, DAT_009a1fe8 + 0x1f + iVar4);
    RD16(uVar6, t1 + 0x79);
    iVar5 = uVar9 * 0x23;
    RD32(t1, DAT_009a1fe8 + 0x1f + iVar5);
    RD16(uVar2, t1 + 0x79);
    if (uVar2 < uVar6) {
        RD16(uVar3, DAT_00a08104 + 0xbd);
        FUN_004a4d70(uVar9, (unsigned int)uVar3 + (unsigned int)(uVar6 - uVar2) * 0x32u);
        RD16(us1, DAT_00a08104 + 0xbd);
        if (uVar3 <= us1) goto LAB_004a1c96;
        uVar11 = 0xffff;
    } else {
        if (uVar2 <= uVar6) goto LAB_004a1c96;
        RD16(uVar3, DAT_00a08104 + 0xbd);
        FUN_004a4d70(uVar9, (unsigned int)uVar3 + (unsigned int)(uVar2 - uVar6) * (unsigned int)-0x32);
        RD16(us1, DAT_00a08104 + 0xbd);
        if (us1 <= uVar3) goto LAB_004a1c96;
        uVar11 = 0;
    }
    FUN_004a4d70(uVar9, (unsigned int)uVar11);
LAB_004a1c96:
    RD16(sVar10, iVar7 + 0xd);
    RD16(s1, DAT_00a08104 + 0xd);
    uVar6 = (unsigned short)((unsigned int)s1 - (unsigned int)sVar10);
    if (0x7fff < uVar6) {
        uVar6 = (unsigned short)(0u - (unsigned int)uVar6);
    }
    if (uVar6 < 0x400) {
        WR16(DAT_00a08104 + 0xd, sVar10);
        *(unsigned char *)(DAT_00a08104 + 0x9c) = 0;
    } else {
        RD32(pB, DAT_009a73b6);
        WR16(pB + 0x2e, sVar10);
        RD16(us1, DAT_00a08104 + 0xd);
        RD32(pB, DAT_009a73b6);
        RD16(uVar3, pB + 0x2e);
        if (us1 < uVar3) {
            *(unsigned char *)(DAT_00a08104 + 0x9c) = 2;
        } else {
            *(unsigned char *)(DAT_00a08104 + 0x9c) = 1;
        }
        WR16(DAT_00a08104 + 0x9e, 0xffff);
        *(unsigned char *)(DAT_00a08104 + 0x9d) = 0;
    }
    RD16(sVar10, DAT_009a1fe8 + 0x14 + iVar4);
    psVar1 = DAT_009a1fe8 + 0x14 + iVar5;
    RD16(s1, psVar1);
    uVar6 = (unsigned short)((unsigned int)s1 - (unsigned int)sVar10);
    if (0x7fff < uVar6) {
        uVar6 = (unsigned short)(0u - (unsigned int)uVar6);
    }
    if (uVar6 < 0x400) {
        WR16(psVar1, sVar10);
        RD32(t1, DAT_009a1fe8 + 0x1f + iVar5);
        *(unsigned char *)(t1 + 0x92) = 1;
        *(unsigned char *)(DAT_00a08104 + 0xa4) = 0;
    } else {
        RD32(pB, DAT_009a73b6);
        WR16(pB + 0x30, sVar10);
        WR16(DAT_00a08104 + 0xa5, 0xffff);
        *(unsigned char *)(DAT_00a08104 + 0xa4) = 1;
    }
    bidx = (unsigned int)DAT_009d8cd4[(unsigned int)DAT_009a73c0 * 0x566 + (unsigned int)DAT_009a73c2 * 0x52];
    FUN_004035d0(DAT_009a1fe8 + iVar4, DAT_00527768[bidx], DAT_005277a0[bidx], DAT_005277d8[bidx]);
    RD32(t2, (unsigned int)DAT_00521f1c + (unsigned int)*(unsigned char *)(DAT_009a1fe8 + iVar4 + 3) * 0x1a);
    if (2999 < (int)t2) {
        DAT_009d8a58 = DAT_009d8a58 << 4;
        DAT_009d8a54 = DAT_009d8a54 << 4;
        DAT_009d8b60 = DAT_009d8b60 << 4;
    }
    iVar7 = (unsigned int)*(unsigned char *)(iVar7 + 5) * 0x23 + DAT_009a1fe8;
    RD32(t1, iVar7 + 0x1b);
    if (t1 != 0xffffffffu) {
        RD32(t2, iVar7 + 0x1f);
        RD16(us1, t2 + 0x79);
        if (us1 < 0xb) {
            WR32(DAT_00a08104 + 0xb1, 0);
            WR32(DAT_00a08104 + 0xb5, 0);
            uVar8 = DAT_00a08104;
            WR32(DAT_00a08104 + 0xb9, 0);
            return uVar8 & 0xffff0000;
        }
    }
    RD32(t1, DAT_009a1fe8 + iVar4 + 4);
    RD32(t2, (unsigned int)DAT_009a73b0 * 0x23 + 4 + DAT_009a1fe8);
    t3 = (t1 - t2) + DAT_009d8a58;
    WR32(DAT_00a08104 + 0xb1, t3);
    RD32(t1, DAT_009a1fe8 + 8 + iVar4);
    RD32(t2, (unsigned int)DAT_009a73b0 * 0x23 + 8 + DAT_009a1fe8);
    t3 = (t1 - t2) + DAT_009d8a54;
    WR32(DAT_00a08104 + 0xb5, t3);
    uVar8 = DAT_009d8b60 & 0xffff0000;
    RD32(t1, DAT_009a1fe8 + 0xc + iVar4);
    RD32(t2, (unsigned int)DAT_009a73b0 * 0x23 + 0xc + DAT_009a1fe8);
    t4 = (t1 - t2) + DAT_009d8b60;
    WR32(DAT_00a08104 + 0xb9, t4);
    return uVar8;
}

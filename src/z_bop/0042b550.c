/* SPDX-License-Identifier: MIT */
/* z_bop 0042b550 */
/* Callee prototypes: calling conventions are not given by the evidence, so default is used. */
void FUN_00427bd0(short a, short b, int c);
void FUN_00427d30(short a, int b, int c);
long long FUN_004ae080(unsigned int a);
long long FUN_004ae120(unsigned int a);
long long FUN_004adff0(unsigned int a);
long long FUN_004ae020(unsigned int a);
long long FUN_004ae160(unsigned int a);

extern unsigned short DAT_009d6950;
extern int DAT_009d6944;
extern int DAT_009a1fe8;
extern short DAT_009d91ea;   /* width not evidenced; 2 bytes assumed */
extern int DAT_00a08104;
extern int DAT_00a07bd0;
extern int DAT_009a8d80;
extern int DAT_009a8d60;
extern int DAT_009a8d8c;
extern int DAT_009d12e8;
extern int DAT_009d1264;
extern int DAT_009d1154;
extern int DAT_009a8e20;
extern int DAT_009a8e1c;
extern int DAT_009a8e28;

#define PUT8(a, v)  (*(unsigned char *)(a) = (unsigned char)(v))
#define PUT16(a, v) do { t16 = (unsigned short)(v); __builtin_memcpy((void *)(a), &t16, 2); } while (0)
#define PUT32(a, v) do { t32 = (unsigned int)(v); __builtin_memcpy((void *)(a), &t32, 4); } while (0)
#define GET16(a, o) do { __builtin_memcpy(&t16, (void *)(a), 2); (o) = t16; } while (0)
#define GET32(a, o) do { __builtin_memcpy(&t32, (void *)(a), 4); (o) = t32; } while (0)
#define FX(r, a, b) do { pr = (long long)(a) * (long long)(b); \
    (r) = (unsigned int)pr >> 0xf | (unsigned int)((unsigned long long)pr >> 0x20) << 0x11; } while (0)

void __stdcall FUN_0042b550(void)
{
    unsigned short uVar1;
    unsigned short uVar2;
    int iVar3;
    unsigned int uVar4;
    unsigned int uVar5;
    int iVar6;
    int E;
    int p;
    unsigned short uVar7;
    unsigned short uVar8;
    long long lVar9;
    long long lVar10;
    long long pr;
    unsigned short auStack_124[14];
    unsigned short auStack_108[14];
    unsigned short auStack_ec[14];
    unsigned short auStack_d0[16];
    unsigned int local_68;
    unsigned int local_60;
    unsigned int local_54;
    unsigned int local_44;
    unsigned int local_40;
    unsigned int local_3c;
    unsigned int local_38;
    unsigned int local_34;
    unsigned int local_30;
    unsigned int local_2c;
    unsigned int local_28;
    unsigned int local_20;
    int local_1c;
    int local_18;
    int local_14;
    short local_e;
    short local_c;
    short local_a;
    short local_8;
    unsigned short local_6;
    unsigned short t16;
    unsigned int t32;

    DAT_009d6950 = 0;
    DAT_009d6944 = 0;
    local_1c = 0;
    local_18 = 0;
    local_14 = 0;
    auStack_124[1] = 0x62;  auStack_d0[1] = 0x4000; auStack_ec[1] = 0;      auStack_108[1] = 0;
    auStack_124[2] = 99;    auStack_d0[2] = 0x4000; auStack_ec[2] = 0;      auStack_108[2] = 0;
    auStack_124[3] = 0x62;  auStack_d0[3] = 0;      auStack_ec[3] = 0;      auStack_108[3] = 0;
    auStack_124[4] = 99;    auStack_d0[4] = 0;      auStack_ec[4] = 0x4000; auStack_108[4] = 0;
    auStack_124[5] = 0x62;  auStack_d0[5] = 0x4000; auStack_ec[5] = 0xc000; auStack_108[5] = 0;
    auStack_124[6] = 99;    auStack_d0[6] = 0x4000; auStack_ec[6] = 0xc000; auStack_108[6] = 0x8000;
    auStack_124[7] = 0x62;  auStack_d0[7] = 0x8000; auStack_ec[7] = 0;      auStack_108[7] = 0;
    auStack_124[8] = 99;    auStack_d0[8] = 0x8000; auStack_ec[8] = 0x8000; auStack_108[8] = 0;
    auStack_124[9] = 0x62;  auStack_d0[9] = 0x4000; auStack_ec[9] = 0x8000; auStack_108[9] = 0;
    auStack_124[10] = 99;   auStack_d0[10] = 0x4000; auStack_ec[10] = 0x8000; auStack_108[10] = 0x4000;
    auStack_124[0xb] = 0x62; auStack_d0[0xb] = 0x4000; auStack_ec[0xb] = 0x4000; auStack_108[0xb] = 0;
    local_6 = 1;
    auStack_124[0xc] = 99;  auStack_d0[0xc] = 0x4000; auStack_ec[0xc] = 0x4000; auStack_108[0xc] = 0x4000;
    do {
        uVar4 = (unsigned int)local_6;
        uVar1 = auStack_124[uVar4];
        iVar3 = (int)(uVar4 * 0x23);
        E = DAT_009a1fe8 + iVar3;

        PUT8(E + 3, uVar1);
        PUT16(E, 1);
        GET32(E + 0x1f, p);
        PUT8(p, 6);
        PUT8(E + 2, 0xe);
        GET32(E + 0x1f, p); PUT16(p + 0x77, 0);
        GET32(E + 0x1f, p); PUT16(p + 0x79, 0);
        GET32(E + 0x1f, p); PUT16(p + 0x7b, 0);
        GET32(E + 0x1f, p); PUT32(p + 0x7d, 0x7fff);
        GET32(E + 0x1f, p); PUT16(p + 0x81, 0);
        GET32(E + 0x1f, p); PUT16(p + 0x83, 0);
        GET32(E + 0x1f, p); PUT16(p + 0x85, 0);
        GET32(E + 0x1f, p); PUT8(p + 0x87, 0);
        GET32(E + 0x1f, p); PUT8(p + 0x88, 0);
        GET32(E + 0x1f, p); PUT8(p + 0x92, 1);
        GET32(E + 0x1f, iVar6);
        PUT8(iVar6 + 0x8b, *(unsigned char *)(iVar6 + 0x92));
        PUT8(E + 0x16, 1);
        DAT_009d91ea = 5;
        DAT_00a08104 = (int)(uVar4 * 0x462 + (unsigned int)DAT_00a07bd0);
        GET32(E + 0x1f, p);
        PUT32(p + 0xa9, DAT_00a08104);

        uVar7 = 0;
        do {
            uVar8 = (unsigned short)(uVar7 + 1);
            PUT8((unsigned int)uVar7 + (unsigned int)DAT_009a1fe8 + 0x19 + (unsigned int)iVar3, 0);
            uVar7 = uVar8;
        } while (uVar8 < 2);

        uVar7 = 0;
        do {
            uVar8 = (unsigned short)(uVar7 + 1);
            uVar5 = (unsigned int)uVar7;
            PUT8(uVar5 + 0x2d8 + (unsigned int)DAT_00a08104, 0);
            PUT8(uVar5 + 0x30a + (unsigned int)DAT_00a08104, 0);
            PUT8(uVar5 + 0x33c + (unsigned int)DAT_00a08104, 0xff);
            uVar7 = uVar8;
        } while (uVar8 < 0x32);

        PUT32(DAT_00a08104 + 0xcb, 0x7fff);
        PUT32(DAT_00a08104 + 199, 0x7fff);
        PUT32(DAT_00a08104 + 0xc3, 0);
        PUT16(DAT_00a08104 + 0xd1, 0);
        PUT8(DAT_00a08104 + 0x1e9, 0);
        PUT8(DAT_00a08104 + 0x1ec, 0);
        PUT8(DAT_00a08104 + 0x1ea, 0);
        PUT8(DAT_00a08104 + 0x1eb, 0);
        PUT8(DAT_00a08104 + 0x29, 0);

        uVar7 = 0;
        do {
            uVar8 = (unsigned short)(uVar7 + 1);
            PUT32((unsigned int)DAT_00a08104 + 0x15 + (unsigned int)uVar7 * 4, 0);
            uVar7 = uVar8;
        } while (uVar8 < 5);

        PUT16(DAT_00a08104 + 0x1e3, 1);
        GET16(DAT_00a08104 + 0x1e3, uVar2);
        PUT16(DAT_00a08104 + 0x1e5, uVar2);
        PUT8(DAT_00a08104 + 0x21a, 0);
        uVar2 = auStack_108[uVar4];
        PUT32(DAT_00a08104 + 0x211, 0x7fff);
        PUT32(DAT_00a08104 + 0x215, 0);
        PUT16(E + 0x14, uVar2);
        uVar2 = auStack_d0[uVar4];
        PUT16(E + 0x10, auStack_ec[uVar4]);
        PUT16(E + 0x12, uVar2);

        iVar6 = E;
        {
            unsigned short a12, a10;
            GET16(iVar6 + 0x12, a12);
            GET16(iVar6 + 0x10, a10);
            FUN_00427bd0((short)a12, (short)a10, iVar6);
        }
        {
            unsigned short a14;
            GET16(E + 0x14, a14);
            FUN_00427d30((short)a14, 0, E);
        }

        lVar9 = FUN_004ae080((unsigned int)uVar1);
        local_e = (short)(-(short)lVar9);
        if (uVar1 == 0x62) {
            local_c = 0;
            lVar9 = FUN_004ae120(0x62);
            local_a = (short)lVar9;
            local_8 = 0;
        } else if (uVar1 == 99) {
            local_c = 0;
            lVar9 = FUN_004adff0(99);
            lVar10 = FUN_004ae120(99);
            local_a = (short)((short)lVar10 + (short)lVar9);
            lVar9 = FUN_004ae020(99);
            lVar10 = FUN_004ae160(99);
            local_8 = (short)((short)lVar10 + (short)lVar9);
        }

        iVar6 = (int)local_e;
        FX(local_68, iVar6, DAT_009d12e8);
        FX(local_60, iVar6, DAT_009d1264);
        FX(local_54, iVar6, DAT_009d1154);

        PUT32(E + 4, (unsigned int)local_1c + local_68 * 0xFFFFFFFEu);
        PUT32(E + 8, (unsigned int)local_18 + local_60 * 0xFFFFFFFEu);
        PUT32(E + 0xc, (unsigned int)local_14 + local_54 * 0xFFFFFFFEu);

        iVar6 = (int)local_c;
        FX(local_44, iVar6, DAT_009a8d80);
        FX(local_38, iVar6, DAT_009a8d60);
        FX(local_34, iVar6, DAT_009a8d8c);
        iVar6 = (int)local_a;
        FX(local_40, iVar6, DAT_009d12e8);
        FX(local_3c, iVar6, DAT_009d1264);
        FX(local_30, iVar6, DAT_009d1154);
        iVar6 = (int)local_8;
        FX(local_2c, iVar6, DAT_009a8e20);
        FX(local_28, iVar6, DAT_009a8e1c);
        FX(local_20, iVar6, DAT_009a8e28);

        local_1c = (int)((unsigned int)local_1c + (local_44 + local_40 + local_2c) * 2u);
        local_18 = (int)((unsigned int)local_18 + (local_3c + local_38 + local_28) * 2u);
        local_14 = (int)((unsigned int)local_14 + (local_34 + local_30 + local_20) * 2u);

        {
            unsigned int v;
            GET32(E + 4, v);
            GET32(E + 0x1f, p);
            PUT32(p + 6, v);
            local_6 = (unsigned short)(local_6 + 1);
            GET32(E + 8, v);
            GET32(E + 0x1f, p);
            PUT32(p + 10, v);
            GET32(E + 0xc, v);
            GET32(E + 0x1f, p);
            PUT32(p + 0xe, v);
        }
    } while (local_6 < 0xd);
    return;
}

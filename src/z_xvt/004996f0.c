/* SPDX-License-Identifier: MIT */
/* z_xvt 004996f0 */
extern unsigned char *DAT_00645a80;
extern int DAT_00703c88;
extern int DAT_00732a38;
extern int DAT_006db7a4;
extern int DAT_00703ac0;
extern int DAT_006d4988;
extern int DAT_007091a0;
extern int DAT_00732940;
extern int DAT_006db798;
extern int DAT_006db71c;
extern unsigned int DAT_0071f2d8;
extern unsigned int _DAT_0071f2dc;
extern unsigned int DAT_0070ac30;
extern unsigned int _DAT_0070ac34;
extern short DAT_006d49a0;
extern unsigned char DAT_006d4a60[];
extern unsigned char DAT_0070acf0[];
extern unsigned char DAT_007092c0[];
extern unsigned char DAT_0071e6a0[];
extern unsigned short DAT_00732fe4;
extern unsigned int DAT_00732e48;
extern unsigned char DAT_0070ace0;
extern unsigned int DAT_0071eae8;
extern unsigned int DAT_006da530;
extern unsigned int DAT_0072f4e0;
extern unsigned int DAT_00732e9c;
extern unsigned int DAT_006da4f0;
extern unsigned int DAT_0070a000;
extern unsigned int _DAT_00732a3c;
extern unsigned int DAT_006d4980;
extern unsigned int DAT_00703acc;
extern unsigned int DAT_0073088c;
extern unsigned int DAT_0071b704;
extern unsigned int DAT_00732904;
extern unsigned int DAT_006da4f4;
extern unsigned int DAT_00703ac8;
extern unsigned int DAT_00703c90;
extern unsigned int DAT_0071e53c;
extern unsigned char DAT_00703ca0[];
extern unsigned int DAT_006da9e8;
extern unsigned int DAT_006d9d44;
extern unsigned char DAT_006da3e0[];
extern unsigned short DAT_00703abc;
extern unsigned short DAT_006db6c0;
extern unsigned int DAT_00525564;
extern unsigned char DAT_0071b730[];

#define RD32(p) ((unsigned int)(p)[0] | ((unsigned int)(p)[1] << 8) | ((unsigned int)(p)[2] << 16) | ((unsigned int)(p)[3] << 24))
#define RD16(p) ((unsigned short)((unsigned int)(p)[0] | ((unsigned int)(p)[1] << 8)))
#define WR32(p, v) ((p)[0] = (unsigned char)(v), (p)[1] = (unsigned char)((unsigned int)(v) >> 8), (p)[2] = (unsigned char)((unsigned int)(v) >> 16), (p)[3] = (unsigned char)((unsigned int)(v) >> 24))
#define CP4(d, s) ((d)[0] = (s)[0], (d)[1] = (s)[1], (d)[2] = (s)[2], (d)[3] = (s)[3])

void __stdcall FUN_004996f0(void)
{
    int iVar9;
    int iVar8;
    int iVar3;
    unsigned char *puVar6;
    unsigned char *puVar7;
    unsigned char *puVar10;
    unsigned char *e;
    unsigned char *p;
    unsigned char *q;
    unsigned char *r;
    unsigned char *d;
    unsigned char *s;
    unsigned int pv;
    unsigned int t;
    unsigned int uVar4;
    unsigned int uVar5;

    iVar9 = 0;
    puVar7 = DAT_00645a80;
    if (0 < DAT_00703c88 + DAT_00732a38) {
        iVar8 = 0;
        puVar6 = DAT_00645a80;
        do {
            if ((iVar9 < DAT_006db7a4) || (puVar7 = puVar6, DAT_00703ac0 <= iVar9)) {
                e = (unsigned char *)DAT_006d4988 + iVar8;
                puVar7 = puVar6 + 1;
                e[3] = *puVar6;
                if (e[3] == 0) {
                    d = e;
                    for (iVar3 = 7; iVar3 != 0; iVar3 = iVar3 + -1) {
                        d[0] = 0; d[1] = 0; d[2] = 0; d[3] = 0;
                        d = d + 4;
                    }
                    d[0] = 0;
                    d[1] = 0;
                    d[2] = 0;
                    t = 0xffffffffu;
                    WR32(e + 0x1b, t);
                    p = (unsigned char *)RD32(e + 0x1f);
                    if (p != 0) {
                        d = p;
                        for (iVar3 = 0x22; iVar3 != 0; iVar3 = iVar3 + -1) {
                            d[0] = 0; d[1] = 0; d[2] = 0; d[3] = 0;
                            d = d + 4;
                        }
                        d[0] = 0;
                        p[0x86] = 0xff;
                        q = (unsigned char *)RD32(p + 0xa7);
                        if (q != 0) {
                            d = q;
                            for (iVar3 = 0xf9; iVar3 != 0; iVar3 = iVar3 + -1) {
                                d[0] = 0; d[1] = 0; d[2] = 0; d[3] = 0;
                                d = d + 4;
                            }
                            d[0] = 0;
                            d[1] = 0;
                        }
                        q = (unsigned char *)RD32(p + 0xa3);
                        if (q != 0) {
                            q[0] = 0; q[1] = 0; q[2] = 0; q[3] = 0;
                            q[4] = 0; q[5] = 0; q[6] = 0; q[7] = 0;
                        }
                        q = (unsigned char *)RD32(p + 0xab);
                        if (q != 0) {
                            d = q;
                            for (iVar3 = 0x13; iVar3 != 0; iVar3 = iVar3 + -1) {
                                d[0] = 0; d[1] = 0; d[2] = 0; d[3] = 0;
                                d = d + 4;
                            }
                        }
                    }
                } else {
                    d = e;
                    s = puVar7;
                    for (iVar3 = 8; iVar3 != 0; iVar3 = iVar3 + -1) {
                        CP4(d, s);
                        s = s + 4;
                        d = d + 4;
                    }
                    d[0] = s[0];
                    d[1] = s[1];
                    d[2] = s[2];
                    puVar7 = puVar6 + 36;
                    pv = RD32(e + 0x1f);
                    if (pv != 0) {
                        pv = pv - 1u;
                        WR32(e + 0x1f, pv);
                        pv = RD32(e + 0x1f);
                        pv = pv + (unsigned int)DAT_007091a0;
                        WR32(e + 0x1f, pv);
                    }
                    p = (unsigned char *)RD32(e + 0x1f);
                    if (p != 0) {
                        d = p;
                        s = puVar7;
                        for (iVar3 = 0x2b; iVar3 != 0; iVar3 = iVar3 + -1) {
                            CP4(d, s);
                            s = s + 4;
                            d = d + 4;
                        }
                        d[0] = s[0];
                        d[1] = s[1];
                        d[2] = s[2];
                        puVar10 = puVar6 + 0xd3;

                        pv = RD32(p + 0xa7);
                        if (pv != 0) {
                            pv = pv - 1u;
                            WR32(p + 0xa7, pv);
                            pv = RD32(p + 0xa7);
                            pv = pv + (unsigned int)DAT_00732940;
                            WR32(p + 0xa7, pv);
                        }
                        pv = RD32(p + 0xa3);
                        if (pv != 0) {
                            pv = pv - 1u;
                            WR32(p + 0xa3, pv);
                            pv = RD32(p + 0xa3);
                            pv = pv + (unsigned int)DAT_006db798;
                            WR32(p + 0xa3, pv);
                        }
                        pv = RD32(p + 0xab);
                        if (pv != 0) {
                            pv = pv - 1u;
                            WR32(p + 0xab, pv);
                            pv = RD32(p + 0xab);
                            pv = pv + (unsigned int)DAT_006db71c;
                            WR32(p + 0xab, pv);
                        }

                        q = (unsigned char *)RD32(p + 0xa7);
                        if (q != 0) {
                            d = q;
                            s = puVar10;
                            for (iVar3 = 0x10d; iVar3 != 0; iVar3 = iVar3 + -1) {
                                CP4(d, s);
                                s = s + 4;
                                d = d + 4;
                            }
                            d[0] = s[0];
                            d[1] = s[1];
                            puVar10 = puVar6 + 0x509;
                            iVar3 = 0;
                            do {
                                q = (unsigned char *)RD32(p + 0xa7);
                                t = RD32(q + 0x3f2 + iVar3);
                                if (t != 0) {
                                    t = t - 1u;
                                    WR32(q + 0x3f2 + iVar3, t);
                                    t = RD32(q + 0x3f2 + iVar3);
                                    t = t + (unsigned int)DAT_006d4988;
                                    WR32(q + 0x3f2 + iVar3, t);
                                }
                                iVar3 = iVar3 + 4;
                            } while (iVar3 < 0x40);
                            q = (unsigned char *)RD32(p + 0xa7);
                            t = RD32(q + 0x432);
                            if (t != 0) {
                                t = t - 1u;
                                WR32(q + 0x432, t);
                                t = RD32(q + 0x432);
                                t = t + (unsigned int)DAT_006d4988;
                                WR32(q + 0x432, t);
                            }
                        }

                        q = (unsigned char *)RD32(p + 0xa3);
                        puVar7 = puVar10;
                        if (q != 0) {
                            puVar7 = puVar10 + 8;
                            CP4(q, puVar10);
                            q[4] = puVar10[4];
                            q[5] = puVar10[5];
                            q[6] = puVar10[6];
                            q[7] = puVar10[7];
                        }
                        r = (unsigned char *)RD32(p + 0xab);
                        if (r != 0) {
                            d = r;
                            s = puVar7;
                            for (iVar3 = 0x13; iVar3 != 0; iVar3 = iVar3 + -1) {
                                CP4(d, s);
                                s = s + 4;
                                d = d + 4;
                            }
                            puVar7 = puVar7 + 0x4c;
                        }
                    }
                }
            }
            iVar8 = iVar8 + 0x23;
            iVar9 = iVar9 + 1;
            puVar6 = puVar7;
        } while (iVar9 < DAT_00703c88 + DAT_00732a38);
    }

    DAT_0071f2d8 = RD32(puVar7);
    _DAT_0071f2dc = RD32(puVar7 + 4);
    DAT_0070ac30 = RD32(puVar7 + 8);
    _DAT_0070ac34 = RD32(puVar7 + 12);
    s = puVar7 + 16;
    d = (unsigned char *)&DAT_006d49a0;
    for (iVar9 = 0x28; iVar9 != 0; iVar9 = iVar9 + -1) {
        CP4(d, s);
        s = s + 4;
        d = d + 4;
    }
    d[0] = s[0];
    d[1] = s[1];
    iVar9 = (int)DAT_006d49a0;
    s = puVar7 + 0xb2;
    d = DAT_006d4a60;
    for (uVar4 = (unsigned int)iVar9 * 0x126u >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
        CP4(d, s);
        s = s + 4;
        d = d + 4;
    }
    for (uVar4 = (unsigned int)iVar9 * 0x126u & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
        *d = *s;
        s = s + 1;
        d = d + 1;
    }
    puVar7 = puVar7 + 0xb2 + (unsigned int)(int)DAT_006d49a0 * 0x126u;
    uVar4 = (unsigned int)(int)DAT_006d49a0 * 0x566u;
    s = puVar7;
    d = DAT_0070acf0;
    for (uVar5 = uVar4 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
        CP4(d, s);
        s = s + 4;
        d = d + 4;
    }
    for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
        *d = *s;
        s = s + 1;
        d = d + 1;
    }
    puVar7 = puVar7 + (unsigned int)(int)DAT_006d49a0 * 0x566u;

    s = puVar7;
    d = DAT_007092c0;
    for (iVar9 = 0x34c; iVar9 != 0; iVar9 = iVar9 + -1) {
        CP4(d, s);
        s = s + 4;
        d = d + 4;
    }
    s = puVar7 + 0xd30;
    d = DAT_0071e6a0;
    for (iVar9 = 5; iVar9 != 0; iVar9 = iVar9 + -1) {
        CP4(d, s);
        s = s + 4;
        d = d + 4;
    }
    d[0] = s[0];
    d[1] = s[1];
    DAT_00732fe4 = RD16(puVar7 + 0xd46);
    DAT_00732e48 = RD32(puVar7 + 0xd48);
    DAT_0070ace0 = puVar7[0xd4c];
    DAT_0071eae8 = RD32(puVar7 + 0xd4d);
    DAT_006da530 = RD32(puVar7 + 0xd51);
    DAT_0072f4e0 = RD32(puVar7 + 0xd55);
    DAT_00732e9c = RD32(puVar7 + 0xd59);
    DAT_006da4f0 = RD32(puVar7 + 0xd5d);
    DAT_0070a000 = RD32(puVar7 + 0xd61);
    _DAT_00732a3c = RD32(puVar7 + 0xd65);
    DAT_006d4980 = RD32(puVar7 + 0xd69);
    DAT_00703acc = RD32(puVar7 + 0xd6d);
    DAT_0073088c = RD32(puVar7 + 0xd71);
    DAT_0071b704 = RD32(puVar7 + 0xd75);
    DAT_00732904 = RD32(puVar7 + 0xd79);
    DAT_006da4f4 = RD32(puVar7 + 0xd7d);
    DAT_00703ac8 = RD32(puVar7 + 0xd81);
    DAT_00703c90 = RD32(puVar7 + 0xd85);
    DAT_0071e53c = RD32(puVar7 + 0xd89);
    DAT_006db7a4 = (int)RD32(puVar7 + 0xd8d);
    DAT_00703ac0 = (int)RD32(puVar7 + 0xd91);
    DAT_00732a38 = (int)RD32(puVar7 + 0xd95);
    DAT_00703c88 = (int)RD32(puVar7 + 0xd99);
    s = puVar7 + 0xd9d;
    d = DAT_00703ca0;
    for (iVar9 = 0x1540; iVar9 != 0; iVar9 = iVar9 + -1) {
        CP4(d, s);
        s = s + 4;
        d = d + 4;
    }
    DAT_006da9e8 = RD32(puVar7 + 0x629d);
    DAT_006d9d44 = RD32(puVar7 + 0x62a1);
    s = puVar7 + 0x62a5;
    d = DAT_006da3e0;
    for (iVar9 = 0x40; iVar9 != 0; iVar9 = iVar9 + -1) {
        CP4(d, s);
        s = s + 4;
        d = d + 4;
    }
    DAT_00703abc = RD16(puVar7 + 0x63a5);
    DAT_006db6c0 = RD16(puVar7 + 0x63a7);
    DAT_00525564 = RD32(puVar7 + 0x63a9);
    s = puVar7 + 0x63ad;
    d = DAT_0071b730;
    for (iVar9 = 0xb7a; iVar9 != 0; iVar9 = iVar9 + -1) {
        CP4(d, s);
        s = s + 4;
        d = d + 4;
    }
    return;
}

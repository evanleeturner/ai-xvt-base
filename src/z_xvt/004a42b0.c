/* SPDX-License-Identifier: MIT */
/* z_xvt 004a42b0 */
extern unsigned char  DAT_00525508;
extern int            DAT_00525518;
extern short          DAT_0052571c;
extern short          DAT_00645ae0;
extern short          DAT_00645b38;
extern unsigned short DAT_00655184;
extern unsigned char  DAT_00655f0c[];   /* 12-byte records indexed by DAT_00655184 */
extern short          DAT_006d4984;
extern short          DAT_006d9d30;
extern int            DAT_006da4e4;
extern short          DAT_006da538;
extern int            DAT_006da840;
extern undefined4     DAT_006da9ec;
extern int            DAT_006db710;
extern int            DAT_006db714;
extern short          DAT_00703ad0;
extern int            DAT_00703c74;
extern short          DAT_00703c78;
extern int            DAT_007092b4;
extern char           DAT_0071bb37[];   /* stride 0x5bd bytes */
extern short          DAT_0071e530;
extern int            DAT_0071ecf4;
extern short          DAT_0071ecf8;
extern unsigned char  DAT_0071f2e2;
extern int            DAT_00732e48;

/* Calling convention of FUN_00426a50 is not confirmed by the evidence; cdecl assumed.
   0x525508 is 1 byte in P-code, so it is declared as unsigned char. */
void FUN_0048ee20(char param_1);
void FUN_00426a50(undefined4 a, undefined4 b, undefined4 c, undefined4 d, undefined4 e,
                  undefined4 f, undefined4 g, undefined4 h, undefined4 i);

void __fastcall FUN_004a42b0(undefined4 param_1, undefined4 param_2)
{
    unsigned short uVar2;
    unsigned short uVar3;
    unsigned int   uVar4;
    unsigned int   uVar6;
    unsigned int   uVar7;
    unsigned int   uVar9;
    unsigned int   uVar16;
    unsigned int   uVar17;
    unsigned int   iVar13;
    unsigned int   idx;
    unsigned int   last;

    uVar2 = (unsigned short)DAT_00525508;
    if (DAT_00732e48 < 1) {
        uVar9  = (unsigned short)DAT_00703ad0;
        uVar7  = (unsigned short)DAT_0052571c;
        uVar3  = (unsigned short)((unsigned int)(unsigned short)DAT_006da4e4 - (unsigned int)(unsigned short)DAT_00703ad0);
        iVar13 = (unsigned int)DAT_00655184 * 0xc;
        uVar16 = *(unsigned short *)(DAT_00655f0c + iVar13 + 2);
        uVar6  = *(unsigned short *)(DAT_00655f0c + iVar13);
        uVar4  = (unsigned short)((unsigned int)(unsigned short)DAT_006da840 - (unsigned int)(unsigned short)DAT_0052571c);
    } else {
        FUN_0048ee20('\0');
        uVar6  = DAT_0071f2e2;
        uVar9  = (unsigned short)DAT_00703ad0;
        uVar3  = (unsigned short)((unsigned int)(unsigned short)DAT_006da4e4 - (unsigned int)(unsigned short)DAT_00703ad0);
        iVar13 = (unsigned int)DAT_00655184 * 0xc;
        uVar4  = (unsigned short)((((unsigned int)(unsigned short)(uVar6 * 4u) - (unsigned int)(unsigned short)DAT_0052571c)
                                   + (unsigned int)(unsigned short)DAT_006da840) + 1u);
        uVar7  = (unsigned short)((unsigned int)(unsigned short)DAT_0052571c - (unsigned int)(unsigned short)uVar6 * 2u);
        uVar6  = *(unsigned short *)(DAT_00655f0c + iVar13);
        uVar16 = *(unsigned short *)(DAT_00655f0c + iVar13 + 2);
    }
    last = (unsigned short)((unsigned int)(unsigned short)DAT_00525518 * (unsigned int)(unsigned short)DAT_006da538);
    uVar17 = last;

    FUN_00426a50(DAT_006da9ec, uVar2, uVar7, uVar9, uVar6, uVar16, uVar4, uVar3, uVar17);

    idx = (unsigned int)DAT_0071ecf4 * 0x5bdu;
    if (DAT_0071bb37[idx] == '\x12') {
        if (DAT_00645ae0 != -1) {
            FUN_00426a50(DAT_006da9ec, uVar2,
                         (unsigned short)DAT_006d9d30,
                         (unsigned short)DAT_00703c78,
                         *(unsigned short *)(DAT_00655f0c + 0xc + (unsigned int)DAT_00655184 * 0xc),
                         (unsigned short)(DAT_006d4984 + -0xb),
                         (unsigned short)((unsigned int)(unsigned short)DAT_006db714 - (unsigned int)(unsigned short)DAT_006d9d30),
                         (unsigned short)((unsigned int)(unsigned short)DAT_006db710 - (unsigned int)(unsigned short)DAT_00703c78),
                         last);
        }
        if (DAT_00645b38 != -1) {
            FUN_00426a50(DAT_006da9ec, uVar2,
                         (unsigned short)DAT_0071e530,
                         (unsigned short)DAT_0071ecf8,
                         *(unsigned short *)(DAT_00655f0c + 0x18 + (unsigned int)DAT_00655184 * 0xc),
                         (unsigned short)(DAT_006d4984 + -0x16),
                         (unsigned short)((unsigned int)(unsigned short)DAT_00703c74 - (unsigned int)(unsigned short)DAT_0071e530),
                         (unsigned short)((unsigned int)(unsigned short)DAT_007092b4 - (unsigned int)(unsigned short)DAT_0071ecf8),
                         last);
        }
    } else {
        if (DAT_00645ae0 != -1) {
            iVar13 = (unsigned int)DAT_00655184 * 0xc;
            FUN_00426a50(DAT_006da9ec, uVar2,
                         (unsigned short)DAT_006d9d30,
                         (unsigned short)DAT_00703c78,
                         *(unsigned short *)(DAT_00655f0c + 0xc + iVar13),
                         *(unsigned short *)(DAT_00655f0c + 0xe + iVar13),
                         (unsigned short)((unsigned int)(unsigned short)DAT_006db714 - (unsigned int)(unsigned short)DAT_006d9d30),
                         (unsigned short)((unsigned int)(unsigned short)DAT_006db710 - (unsigned int)(unsigned short)DAT_00703c78),
                         last);
        }
        if (DAT_00645b38 != -1) {
            iVar13 = (unsigned int)DAT_00655184 * 0xc;
            FUN_00426a50(DAT_006da9ec, uVar2,
                         (unsigned short)DAT_0071e530,
                         (unsigned short)DAT_0071ecf8,
                         *(unsigned short *)(DAT_00655f0c + 0x18 + iVar13),
                         *(unsigned short *)(DAT_00655f0c + 0x1a + iVar13),
                         (unsigned short)((unsigned int)(unsigned short)DAT_00703c74 - (unsigned int)(unsigned short)DAT_0071e530),
                         (unsigned short)((unsigned int)(unsigned short)DAT_007092b4 - (unsigned int)(unsigned short)DAT_0071ecf8),
                         last);
            return;
        }
    }
    return;
}

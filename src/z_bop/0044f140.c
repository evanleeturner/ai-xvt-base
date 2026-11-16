/* SPDX-License-Identifier: MIT */
/* z_bop 0044f140 */
extern unsigned char DAT_009e9683[];
extern unsigned char DAT_009e9684[];
extern unsigned char DAT_009e9685[];
extern unsigned char DAT_009e9a67[];
extern unsigned char DAT_009e9a84[];
extern int DAT_009ecc34;
extern int DAT_00527ed0;
extern char DAT_00a0813f;
extern int DAT_00527e90;
extern int DAT_00527e94;
extern short DAT_009cd264;
extern short DAT_009a8062;
extern short DAT_009ed664;
extern int DAT_00a07ce8;
extern int DAT_009a8e24;
extern char DAT_009a73f4;
extern unsigned char DAT_009d6941;
extern short DAT_009cd274;
extern int DAT_00a00860;
extern unsigned char *DAT_009a1fe8;
extern unsigned int DAT_009a8d9c;
extern unsigned char DAT_00521f1c[];
extern unsigned int DAT_00a08104;
extern int DAT_009a8e34;
extern int DAT_009d1308;
extern unsigned char *DAT_009a8c1c;
extern char DAT_009ec476;
extern int DAT_005235f0;
extern int DAT_0051a520;
extern int DAT_009a8c2c;
extern short DAT_009a7ba8;
extern short DAT_009e9660;
extern unsigned char DAT_00523408;
extern unsigned char DAT_005233c8;

void FUN_00435bc0(void);
void FUN_0040b7b0(void);
void FUN_00438ef0(int a);
void FUN_004aba50(void);
void __fastcall FUN_00452960(unsigned int a, unsigned int b);
void FUN_00441c30(void);
void FUN_004abe50(void);
void __fastcall FUN_00485f00(void *a, int b);
void FUN_004244e0(void);
void FUN_004865e0(void);
void FUN_004244d0(void);
void thunk_FUN_004b3070(void);
void FUN_00426080(void);
void FUN_00410930(void);
void FUN_00436310(void);
unsigned char FUN_0044ff10(unsigned int a, unsigned int b);
void FUN_004362a0(unsigned int a, int b);
unsigned char FUN_00450020(int a, int b, int c, unsigned int d);
void FUN_00436680(void);
void FUN_0044fe40(unsigned int a);
void FUN_00427a60(short a, short b, short c, int d, unsigned char *e);
void FUN_0042b380(unsigned short a);
void FUN_0044f880(unsigned char *a);
void FUN_0041fc20(unsigned int a);
void FUN_00472400(unsigned char *a);
void FUN_0041ff70(unsigned short a);
void FUN_00401000(int a);
void __fastcall FUN_00446550(void *a, unsigned int b);
void FUN_004012c0(int a);
void FUN_00482be0(void);
int FUN_004ac760(void);
void FUN_00486200(void);

void __stdcall FUN_0044f140(void)
{
    int iVar8;
    int iVar9;
    int iVar5;
    char cVar2;
    unsigned char bVar3;
    unsigned short uVar4;
    unsigned int uVar6;
    unsigned char bVar7;
    void *this;
    void *this_00 = 0;
    void *extraout_ECX = 0;
    void *extraout_ECX_00 = 0;
    void *extraout_ECX_01 = 0;
    void *extraout_ECX_02 = 0;
    void *extraout_ECX_03 = 0;
    unsigned int extraout_EDX = 0;
    unsigned int extraout_EDX_00 = 0;
    unsigned int extraout_EDX_01 = 0;
    short sVar10;
    unsigned int unaff_EDI = 0;
    unsigned int local_4 = 0;
    unsigned char *pEnt;
    unsigned char *node;

    iVar8 = (int)((unsigned int)DAT_009ecc34 * 0x5bdU);
    if (DAT_009e9683[iVar8] != 0) {
        DAT_00527ed0 = 0;
        DAT_00a0813f = 0;
        FUN_00435bc0();
        if (DAT_00527e90 != 0) {
            FUN_0040b7b0();
        }
        FUN_00438ef0(DAT_009ecc34);
        FUN_004aba50();
        FUN_00452960((unsigned int)extraout_ECX, extraout_EDX);
        FUN_00441c30();
        FUN_004abe50();
        DAT_00527ed0 = 1;
        return;
    }
    cVar2 = (char)DAT_009e9684[iVar8];
    this = (void *)((((unsigned int)DAT_009ecc34 >> 8) << 8) | (unsigned char)cVar2);
    if (cVar2 == '\x02') {
        __builtin_memcpy(&this, &DAT_009e9685[iVar8], 4);
        if ((unsigned int)this < 0x213U) {
            DAT_00527ed0 = 0;
            DAT_00a0813f = 0;
            FUN_00485f00(this, 1);
            FUN_004244e0();
            FUN_004865e0();
            if (DAT_00527e90 != 0) {
                FUN_0040b7b0();
            }
            FUN_00438ef0(DAT_009ecc34);
            FUN_004aba50();
            FUN_00452960((unsigned int)extraout_ECX_00, extraout_EDX_00);
            FUN_00441c30();
            FUN_004abe50();
            DAT_00527ed0 = 1;
            return;
        }
    } else if (cVar2 == '\x01') {
        FUN_004244d0();
        this = extraout_ECX_01;
    }
    DAT_00527ed0 = 0;
    DAT_00a0813f = 0;
    if (DAT_009cd264 != 0) {
        this = (void *)0;
        if (DAT_00527e90 != 0) {
            thunk_FUN_004b3070();
            this = extraout_ECX_02;
        }
        DAT_009cd264 = 0;
    }
    FUN_00485f00(this, 1);
    DAT_009a8062 = 0;
    if (DAT_00527e90 != 0) {
        DAT_009ed664 = 0x3000;
        FUN_00426080();
        FUN_004aba50();
        FUN_00410930();
        FUN_004abe50();
    }
    sVar10 = 0;
    FUN_00436310();
    if (0 < DAT_00a07ce8) {
        do {
            if (sVar10 == DAT_009a8e24) {
                int c1 = 0;
                if (DAT_009a73f4 == '\0') c1 = 1;
                else if (DAT_009d6941 != 0) c1 = 1;
                else if (DAT_009e9684[(int)((unsigned int)DAT_009ecc34 * 0x5bdU)] == 2) c1 = 1;
                if (c1) {
                    sVar10 = DAT_009cd274;
                    if (DAT_009cd274 == DAT_00a07ce8) break;
                }
            }
            unaff_EDI = (unsigned int)(int)sVar10;
            {
                unsigned short a1;
                short a2;
                __builtin_memcpy(&a1, &DAT_009e9a67[(int)((unsigned int)DAT_009ecc34 * 0x5bdU)], 2);
                __builtin_memcpy(&a2, &DAT_009e9a84[(int)((unsigned int)DAT_009ecc34 * 0x5bdU)], 2);
                if (a1 != unaff_EDI || a2 != 0 || DAT_00a00860 != 0) {
                    pEnt = DAT_009a1fe8 + (unaff_EDI * 0x23U);
                    if (pEnt[3] != 0) {
                        __builtin_memcpy(&DAT_009a8d9c, &DAT_00521f1c[(unsigned int)pEnt[3] * 0x1aU], 4);
                        switch (pEnt[2]) {
                        case 0: case 1: case 2: case 3: case 4: case 5: case 0xe:
                        {
                            unsigned char *p9;
                            __builtin_memcpy(&p9, pEnt + 0x1f, 4);
                            __builtin_memcpy(&DAT_00a08104, p9 + 0xa9, 4);
                            bVar7 = FUN_0044ff10(unaff_EDI, DAT_009a8d9c);
                            iVar8 = bVar7;
                            break;
                        }
                        case 6: case 7:
                            bVar7 = FUN_0044ff10(unaff_EDI, DAT_009a8d9c);
                            iVar8 = bVar7;
                            break;
                        default:
                            goto next1;
                        case 0xb:
                            bVar7 = FUN_0044ff10(unaff_EDI, DAT_009a8d9c);
                            iVar8 = bVar7;
                            break;
                        case 0xd:
                            bVar7 = FUN_0044ff10(unaff_EDI, DAT_009a8d9c);
                            iVar8 = bVar7;
                        }
                        if (iVar8 != 0) {
                            FUN_004362a0(unaff_EDI, DAT_009a8e34);
                        }
                    }
                }
            }
next1:
            sVar10 = (short)(sVar10 + 1);
        } while (sVar10 < DAT_00a07ce8);
    }
    sVar10 = (short)DAT_00a07ce8;
    if ((int)(short)DAT_00a07ce8 < (int)((unsigned int)DAT_009d1308 + (unsigned int)DAT_00a07ce8)) {
        do {
            unaff_EDI = (unsigned int)(int)sVar10;
            pEnt = DAT_009a1fe8 + (unaff_EDI * 0x23U);
            if (pEnt[3] != 0) {
                __builtin_memcpy(&DAT_009a8d9c, &DAT_00521f1c[(unsigned int)pEnt[3] * 0x1aU], 4);
                if (7 < pEnt[2] && pEnt[2] < 0xc) {
                    int p4, p8, pc;
                    __builtin_memcpy(&p4, pEnt + 4, 4);
                    __builtin_memcpy(&p8, pEnt + 8, 4);
                    __builtin_memcpy(&pc, pEnt + 0xc, 4);
                    bVar7 = FUN_00450020(p4, p8, pc, DAT_009a8d9c);
                    if (bVar7 != 0) {
                        FUN_004362a0(unaff_EDI, DAT_009a8e34);
                    }
                }
            }
            sVar10 = (short)(sVar10 + 1);
        } while ((int)sVar10 < (int)((unsigned int)DAT_009d1308 + (unsigned int)DAT_00a07ce8));
    }
    FUN_00436680();
    for (node = DAT_009a8c1c; node != 0; ) {
        unsigned char *next;
        short s14, s12, s10;
        __builtin_memcpy(&uVar4, node + 4, 2);
        unaff_EDI = (unaff_EDI & 0xffff0000U) | uVar4;
        iVar9 = (int)(short)uVar4;
        iVar5 = (int)((unsigned int)iVar9 * 0x23U);
        bVar3 = DAT_009a1fe8[iVar5 + 2];
        if (iVar9 < DAT_00a07ce8) {
            switch (bVar3) {
            case 0: case 1: case 2: case 3: case 4: case 5: case 0xe:
            {
                unsigned char *p9;
                __builtin_memcpy(&p9, DAT_009a1fe8 + iVar5 + 0x1f, 4);
                __builtin_memcpy(&DAT_00a08104, p9 + 0xa9, 4);
                FUN_0044fe40(unaff_EDI);
                uVar6 = (unsigned int)DAT_00527e94;
                if (bVar3 == 0xe) {
                    DAT_009ec476 = 0;
                }
                pEnt = DAT_009a1fe8 + iVar5;
                if (pEnt[3] == '$') {
                    DAT_00527e94 = 0;
                    local_4 = uVar6;
                }
                __builtin_memcpy(&s14, pEnt + 0x14, 2);
                __builtin_memcpy(&s12, pEnt + 0x12, 2);
                __builtin_memcpy(&s10, pEnt + 0x10, 2);
                FUN_00427a60(s14, s12, s10, 0, pEnt);
                if (bVar3 == 0xe) {
                    FUN_0042b380(uVar4);
                } else {
                    FUN_0044f880(DAT_009a1fe8 + iVar5);
                    FUN_0041fc20(unaff_EDI);
                    FUN_00472400(DAT_009a1fe8 + iVar5);
                    DAT_005235f0 = 0;
                }
                if (DAT_009a1fe8[3 + iVar5] == '$') {
                    DAT_00527e94 = (int)local_4;
                }
                DAT_009ec476 = 1;
                break;
            }
            case 6: case 7:
                FUN_0044fe40(unaff_EDI);
                pEnt = DAT_009a1fe8 + iVar5;
                __builtin_memcpy(&s14, pEnt + 0x14, 2);
                __builtin_memcpy(&s12, pEnt + 0x12, 2);
                __builtin_memcpy(&s10, DAT_009a1fe8 + 0x10 + iVar5, 2);
                FUN_00427a60(s14, s12, s10, 0, pEnt);
                FUN_0041ff70(uVar4);
                break;
            case 0xb:
                FUN_0044fe40(unaff_EDI);
                pEnt = DAT_009a1fe8 + iVar5;
                __builtin_memcpy(&s14, pEnt + 0x14, 2);
                __builtin_memcpy(&s12, pEnt + 0x12, 2);
                __builtin_memcpy(&s10, DAT_009a1fe8 + 0x10 + iVar5, 2);
                FUN_00427a60(s14, s12, s10, 0, pEnt);
                FUN_00401000(iVar9);
                break;
            case 0xd:
                FUN_0044fe40(unaff_EDI);
                pEnt = DAT_009a1fe8 + iVar5;
                __builtin_memcpy(&s14, pEnt + 0x14, 2);
                __builtin_memcpy(&s12, pEnt + 0x12, 2);
                __builtin_memcpy(&s10, DAT_009a1fe8 + 0x10 + iVar5, 2);
                FUN_00427a60(s14, s12, s10, 0, pEnt);
                FUN_00401000(iVar9);
            }
        } else if (7 < bVar3 && bVar3 < 0xc) {
            FUN_0044fe40(unaff_EDI);
            __builtin_memcpy(&s14, DAT_009a1fe8 + iVar5 + 0x14, 2);
            __builtin_memcpy(&s12, DAT_009a1fe8 + iVar5 + 0x12, 2);
            __builtin_memcpy(&s10, DAT_009a1fe8 + 0x10 + iVar5, 2);
            FUN_00427a60(s14, s12, s10, 0, 0);
            FUN_0044f880(DAT_009a1fe8 + iVar5);
            FUN_00446550(this_00, uVar4);
            DAT_005235f0 = 0;
        }
        __builtin_memcpy(&next, node + 8, 4);
        node = next;
    }
    if (DAT_00527e90 == 0) {
        DAT_009ed664 = 0x3000;
        FUN_00426080();
        FUN_004aba50();
        FUN_00410930();
        FUN_004abe50();
    }
    DAT_0051a520 = 1;
    FUN_004865e0();
    DAT_0051a520 = 0;
    if (DAT_00527e90 == 0) {
        FUN_004012c0(1);
        FUN_00482be0();
    }
    DAT_009a7ba8 = 0;
    DAT_009e9660 = 0;
    iVar8 = FUN_004ac760();
    DAT_009a8c2c = (int)((unsigned int)DAT_009a8c2c + (unsigned int)iVar8);
    DAT_009a7ba8 = (short)DAT_009a8c2c;
    FUN_00486200();
    if (DAT_00527e90 != 0) {
        FUN_0040b7b0();
    }
    FUN_00438ef0(DAT_009ecc34);
    DAT_00523408 = 0;
    iVar8 = FUN_004ac760();
    DAT_009a8c2c = (int)((unsigned int)DAT_009a8c2c + (unsigned int)iVar8);
    DAT_00523408 = DAT_005233c8;
    DAT_009a7ba8 = (short)((short)DAT_009a8c2c - DAT_009a7ba8);
    FUN_004aba50();
    FUN_00452960((unsigned int)extraout_ECX_03, extraout_EDX_01);
    FUN_00441c30();
    FUN_004abe50();
    DAT_00527ed0 = 1;
}

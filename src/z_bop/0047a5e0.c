/* SPDX-License-Identifier: MIT */
/* z_bop 0047a5e0 */
extern unsigned short DAT_009d6940;
extern int DAT_00523410;
extern unsigned char *DAT_009a1fe8;
extern unsigned char DAT_009e9a67;
extern unsigned char DAT_009e9a69;
extern unsigned char DAT_009e9a8a;
extern unsigned char DAT_009e9a84;
extern unsigned char DAT_009e9a80;
extern unsigned char DAT_009e9a73;
extern unsigned char DAT_009e9a75;
extern unsigned char DAT_009e9670;
extern unsigned char DAT_009e9683;
extern unsigned char DAT_009e9681;
extern unsigned char DAT_009e9684;
extern unsigned char DAT_009e9680;
extern unsigned char DAT_009e967e;
extern unsigned char DAT_009e96c1;
extern unsigned char DAT_009e96c3;
extern unsigned char DAT_009e9a5a;
extern int DAT_009ecc34;
extern int DAT_009d6930;
extern int DAT_009ecc3c;
extern int DAT_005233d8;
extern int DAT_00523400;
extern void (*DAT_009a8c20)(void);
extern short DAT_00a07be0;
extern unsigned char DAT_005233f8;
extern unsigned int DAT_00523404;
extern char DAT_00a07cce;
extern int DAT_00a080f8;
extern int DAT_009a8c2c;
extern int DAT_00527ed0;
extern short DAT_009a8c12;
extern int DAT_00a00730;
extern int DAT_009a7b54;
extern unsigned char DAT_00523440;
extern char DAT_00a07c70;
extern unsigned short DAT_009ec474;
extern unsigned short DAT_009ec5fe;

void FUN_00427720(int, int);
unsigned int FUN_00411810(int);
void FUN_00411540(void);
void FUN_00450550(void);
void FUN_00450650(int, int);
void FUN_0042e4d0(int, int, int);
void FUN_00484160(unsigned int);
int FUN_004ac760(void);
void FUN_004aba50(void);
void __fastcall FUN_00452960(void);
void FUN_004abe50(void);
void FUN_004abee0(void);
void FUN_0042d6d0(void);
void FUN_00410fe0(void);
void FUN_00411010(void);
void FUN_00455c80(unsigned short);
int FUN_0045a1e0(int, int, int, int);
void FUN_00485080(int);
void FUN_004851d0(int);
void FUN_00452880(int);
void FUN_00405890(int);
unsigned int FUN_00438010(int);
unsigned int FUN_004820b0(int);
void FUN_004833d0(unsigned int, int);
void FUN_0047ac10(int);
void FUN_00481420(int);
void FUN_00480570(int);

/*
 * FINDING: FUN_00452960 (call in case 0x8f, after FUN_004aba50) is a __fastcall
 * whose ECX/EDX are leftover register values in the original (extraout_ECX/EDX).
 * It is declared with no parameters so no ECX/EDX setup is emitted and the
 * live register state passes through untouched.
 * Note: the __builtin_memcpy uses are for unaligned (stride 0x5bd) field
 * accesses; they inline to plain moves and are not real calls.
 */
void __cdecl FUN_0047a5e0(int param_1)
{
    int iVar4;
    unsigned int uVar5;
    unsigned short uVar2;
    short sVar3;
    unsigned short w;
    unsigned short w2;
    unsigned short z16;
    unsigned int d32;
    unsigned short tmp;

    if ((char)DAT_009d6940 == 1) {
        return;
    }
    if (DAT_00523410 == 0) {
        iVar4 = (int)((unsigned int)param_1 * 0x5bdU);
        __builtin_memcpy(&uVar2, &DAT_009e9a67 + iVar4, 2);
        if ((uVar2 != 0xffff) && (DAT_009a1fe8[(unsigned int)uVar2 * 0x23U + 3U] == 0)) {
            if ((&DAT_009e9683)[iVar4] == 0) {
                z16 = 0;
                __builtin_memcpy(&DAT_009e9a8a + iVar4, &z16, 2);
                __builtin_memcpy(&DAT_009e9a84 + iVar4, &z16, 2);
                __builtin_memcpy(&DAT_009e9a80 + iVar4, &z16, 2);
                __builtin_memcpy(&d32, &DAT_009e9670 + iVar4, 4);
                w = (unsigned short)d32;
                __builtin_memcpy(&DAT_009e9a67 + iVar4, &w, 2);
                FUN_00427720(0, param_1);
                __builtin_memcpy(&DAT_009e9a73 + iVar4, &z16, 2);
                __builtin_memcpy(&DAT_009e9a75 + iVar4, &z16, 2);
            } else {
                w = 0xffff;
                __builtin_memcpy(&DAT_009e9a67 + iVar4, &w, 2);
            }
        }
        if ((&DAT_009e9683)[iVar4] != 0) {
            __builtin_memcpy(&w2, &DAT_009e9a69 + iVar4, 2);
            if ((w2 != 0xffff) && (DAT_009a1fe8[(unsigned int)w2 * 0x23U + 3U] == 0)) {
                w = 0xffff;
                __builtin_memcpy(&DAT_009e9a69 + iVar4, &w, 2);
            }
        }
    }
    FUN_00411810(param_1);
    FUN_00411540();
    if ((param_1 == DAT_009ecc34) && (DAT_00523410 == 0)) {
        switch (DAT_009d6930) {
        case 0x4c:
            if (DAT_009ecc3c == 0) {
                DAT_009ecc3c = 1;
                FUN_00450650(0x193, param_1);
            } else {
                DAT_009ecc3c = 0;
                FUN_00450550();
                FUN_00450650(0x192, param_1);
            }
            break;
        case 0x81:
            if (DAT_005233d8 == 1) {
                DAT_00523400 = (int)((unsigned int)DAT_00523400 + 0x40U);
                if (DAT_00523400 == 0x300) {
                    DAT_00523400 = 0x100;
                }
                (*DAT_009a8c20)();
                DAT_00a07be0 = (short)(((unsigned int)DAT_00523400 - 0x100U) >> 6) + 1;
                FUN_00450650(0x11f, param_1);
                FUN_0042e4d0(0x2b, -1, param_1);
            }
            break;
        case 0x83:
            DAT_005233f8 = DAT_005233f8 + 1;
            if (3 < DAT_005233f8) {
                DAT_005233f8 = 0;
            }
            FUN_00484160((((((unsigned int)(DAT_009d6930 - 0x4c)) >> 8) << 8) | (unsigned int)DAT_005233f8) & 0xffff00ffU);
            FUN_00450650((int)DAT_005233f8 + 0x66, param_1);
            FUN_0042e4d0(0x2b, -1, param_1);
            break;
        case 0x88:
            DAT_00523404 = (unsigned int)(DAT_00523404 == 0);
            break;
        case 0x8c:
            if (DAT_00a07cce == 0) {
                DAT_00a07cce = 1;
            } else {
                DAT_00a07cce = 0;
            }
            break;
        case 0x8f:
            if (DAT_00a080f8 == 1) {
                iVar4 = FUN_004ac760();
                DAT_009a8c2c = (int)((unsigned int)DAT_009a8c2c + (unsigned int)iVar4);
                FUN_0042e4d0(0x2b, -1, param_1);
                FUN_00450650(1, param_1);
                DAT_00527ed0 = 0;
                FUN_004aba50();
                FUN_00452960();
                FUN_004abe50();
                FUN_004abee0();
                DAT_00527ed0 = 1;
                FUN_0042d6d0();
                do {
                    uVar5 = FUN_00411810(-2);
                } while ((short)uVar5 == 0);
                FUN_004ac760();
                FUN_00450650(2, param_1);
                DAT_009a8c12 = 0;
                DAT_00a00730 = 0;
                FUN_00410fe0();
            }
            break;
        case 0x92:
            if (DAT_009a7b54 == 0) {
                DAT_009a7b54 = 1;
                FUN_00450650(0x191, param_1);
            } else {
                DAT_009a7b54 = 0;
                FUN_00450650(400, param_1);
            }
            break;
        case 0x95:
            FUN_0042e4d0(0x2b, -1, param_1);
            FUN_00450650(0, param_1);
            break;
        case 0xe8:
            FUN_0042e4d0(0x2b, -1, param_1);
            FUN_00411010();
        }
    }
    if ((1 < DAT_00523440) && (DAT_00a07c70 != 0)) {
        return;
    }
    iVar4 = (int)((unsigned int)param_1 * 0x5bdU);
    if ((&DAT_009e9681)[iVar4] != 0) {
        if (DAT_00523410 != 0) {
            return;
        }
        __builtin_memcpy(&d32, &DAT_009e9670 + iVar4, 4);
        if ((int)d32 == -1) {
            return;
        }
        if (DAT_009a1fe8[d32 * 0x23U + 3U] != 0) {
            return;
        }
        __builtin_memcpy(&w, &DAT_009e967e + iVar4, 2);
        FUN_00455c80(w);
        iVar4 = FUN_0045a1e0(param_1, -1, 0, 0);
        if (iVar4 == 0) {
            if (param_1 != DAT_009ecc34) {
                return;
            }
            FUN_00452880(0x122);
            return;
        }
        FUN_00485080(param_1);
        FUN_004851d0(param_1);
        return;
    }
    if ((&DAT_009e9684)[iVar4] != 0) goto LAB_0047ab0a;
    __builtin_memcpy(&w, &DAT_009e9a80 + iVar4, 2);
    if (((((unsigned char)DAT_009ec474 & 0xd) == 1) && (w == 0)) && ((&DAT_009e9683)[iVar4] == 0)) {
        FUN_00405890(param_1);
    }
    __builtin_memcpy(&w, &DAT_009e96c1 + iVar4, 2);
    uVar2 = w & 0xe;
    if (((unsigned char)DAT_009ec474 & 0xe) == 2) {
        __builtin_memcpy(&tmp, &DAT_009e96c3 + iVar4, 2);
        if (uVar2 == 2) {
            tmp = (unsigned short)(tmp + DAT_009ec5fe);
        } else {
            tmp = DAT_009ec5fe;
        }
        __builtin_memcpy(&DAT_009e96c3 + iVar4, &tmp, 2);
        __builtin_memcpy(&DAT_009e96c1 + iVar4, &DAT_009ec474, 2);
        if (tmp < 0x3b) {
            DAT_009ec474 = DAT_009ec474 & 0xfffd;
        }
        goto LAB_0047ab0a;
    }
    __builtin_memcpy(&tmp, &DAT_009e96c3 + iVar4, 2);
    if ((uVar2 == 2) && (tmp < 0x3b)) {
        if ((&DAT_009e9683)[iVar4] == 0) {
            __builtin_memcpy(&w, &DAT_009e9a80 + iVar4, 2);
            if (((DAT_009d6940 >> 8) & 0xff) != 0 || w != 0)
                goto LAB_0047aaf4;
            uVar5 = FUN_004820b0(param_1);
            sVar3 = (short)uVar5;
        } else {
            uVar5 = FUN_00438010(param_1);
            sVar3 = (short)uVar5;
        }
        if (sVar3 != -1) {
            FUN_004833d0(uVar5, param_1);
        }
    }
LAB_0047aaf4:
    __builtin_memcpy(&DAT_009e96c1 + iVar4, &DAT_009ec474, 2);
    z16 = 0;
    __builtin_memcpy(&DAT_009e96c3 + iVar4, &z16, 2);
LAB_0047ab0a:
    if ((&DAT_009e9a5a)[iVar4] == 0) {
        FUN_0047ac10(param_1);
    } else {
        FUN_00481420(param_1);
    }
    if ((&DAT_009e9680)[iVar4] != 0) {
        FUN_00480570(param_1);
    }
    return;
}

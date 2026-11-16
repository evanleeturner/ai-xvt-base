/* SPDX-License-Identifier: MIT */
/* z_xvt 0045aa10 */
extern char DAT_0071b743[];
extern int DAT_0071ecf4;
extern int DAT_0052ae54;
extern char DAT_00732e8f;
extern int DAT_0052ae18;
extern short DAT_006ffbe4;
extern short DAT_006da9e2;
extern int DAT_0071f714;
extern int DAT_00732a38;
extern int DAT_006db7a4;
extern char DAT_006d9d94;
extern char DAT_007092c1;
extern short DAT_00703ac0;
extern int DAT_00703c88;
extern int DAT_006d4988;
extern unsigned char DAT_0052686c[];
extern unsigned short DAT_006db718;
extern int DAT_00732e54;
extern int DAT_00732910;
extern int DAT_006db7b4;
extern int DAT_006db7ac;
extern int DAT_006db7b0;
extern int DAT_006db59c;
extern int DAT_0072f44c;
extern int DAT_0072f450;
extern int DAT_0072f448;
extern char DAT_0071e536;
extern int DAT_00525730;
extern int DAT_005279c8;
extern short DAT_006da534;
extern short DAT_0071b724;
extern int DAT_006db5a8;
extern char DAT_00525548;
extern char DAT_00525508;
extern unsigned char DAT_0071bb1b[];
extern unsigned char DAT_0071bb1f[];
extern unsigned char DAT_0071bb23[];
extern unsigned char DAT_0071bb27[];
extern unsigned char DAT_0071bb44[];

void FUN_004337a0(void);
void FUN_0045f800(void);
void FUN_004a84a0(int);
void FUN_004a6a80(void);
void __fastcall FUN_004a42b0(unsigned int, unsigned int);
void FUN_004b2ae0(void);
void FUN_004a6f90(void);
void thunk_FUN_004b67f9(void);
void FUN_00465970(int);
void FUN_0044ae00(void);
void FUN_00464b00(void);
void FUN_00433e70(void);
unsigned char FUN_0045b880(unsigned int, unsigned int);
void FUN_00433e00(unsigned int, int);
unsigned int FUN_0045b990(int, int, int, unsigned int);
void FUN_004342f0(void);
int FUN_0040ca00(int, int, int);
int FUN_0040ca30(int, int, int);
int FUN_0040ca60(int, int, int);
void FUN_004832a0(short, short, short, int, int);
void FUN_004010f0(unsigned short);
void FUN_0045b420(int);
void FUN_0049c2b0(unsigned int);
void FUN_00486310(int);
void FUN_0049c590(unsigned short);
void FUN_0047f0f0(int);
void FUN_00466020(void);
void FUN_0047f3d0(int);
void FUN_00442410(void);
int FUN_0048f530(void);
void FUN_00465c40(void);
void __fastcall FUN_00412e60(void *, unsigned short);

#define COMPUTE_VEC(rec)                                                              \
    do {                                                                              \
        int iVar18, iVar19, iVar20, iVar21, iVar22, iVar23;                           \
        __builtin_memcpy(&iVar18, (unsigned char *)(rec) + 4, 4);                     \
        __builtin_memcpy(&iVar19, (unsigned char *)(rec) + 8, 4);                     \
        __builtin_memcpy(&iVar20, (unsigned char *)(rec) + 0xc, 4);                   \
        __builtin_memcpy(&iVar21, DAT_0071bb1b + (unsigned int)DAT_0071ecf4 * 0x5bd, 4); \
        __builtin_memcpy(&iVar22, DAT_0071bb1f + (unsigned int)DAT_0071ecf4 * 0x5bd, 4); \
        __builtin_memcpy(&iVar23, DAT_0071bb23 + (unsigned int)DAT_0071ecf4 * 0x5bd, 4); \
        DAT_0072f44c = (int)((unsigned int)iVar18 - (unsigned int)iVar21);            \
        DAT_0072f450 = (int)((unsigned int)iVar19 - (unsigned int)iVar22);            \
        DAT_0072f448 = (int)((unsigned int)iVar20 - (unsigned int)iVar23);            \
        DAT_006db7ac = FUN_0040ca00(DAT_0072f44c, DAT_0072f450, DAT_0072f448);        \
        DAT_006db7b0 = FUN_0040ca30(DAT_0072f44c, DAT_0072f450, DAT_0072f448);        \
        DAT_006db7b4 = FUN_0040ca60(DAT_0072f44c, DAT_0072f450, DAT_0072f448);        \
    } while (0)

#define CALL_832A0(rec, last)                                                         \
    do {                                                                              \
        short sVar24, sVar25, sVar26;                                                 \
        __builtin_memcpy(&sVar24, (unsigned char *)(rec) + 0x14, 2);                  \
        __builtin_memcpy(&sVar25, (unsigned char *)(rec) + 0x12, 2);                  \
        __builtin_memcpy(&sVar26, (unsigned char *)(rec) + 0x10, 2);                  \
        FUN_004832a0(sVar24, sVar25, sVar26, 0, (last));                              \
    } while (0)

/* Captures the live ECX/EDX left over by the previous call (Ghidra's extraout_*). */
#define CAPTURE_ECX(v) __asm__ volatile("" : "=c"(v))
#define CAPTURE_ECX_EDX(c, d) __asm__ volatile("" : "=c"(c), "=d"(d))

void __stdcall FUN_0045aa10(void)
{
    unsigned char bVar2;
    unsigned short uVar3;
    int iVar4;
    unsigned int uVar6;
    unsigned int extraout_ECX;
    unsigned int extraout_EDX;
    unsigned int extraout_ECX_01;
    unsigned int extraout_EDX_00;
    int extraout_ECX_00;
    void *this_;
    int iVar7;
    short sVar8;
    int iVar9;
    unsigned int unaff_EDI = 0;
    unsigned int uVar10;

    if (DAT_0071b743[(unsigned int)DAT_0071ecf4 * 0x5bd] != '\0') {
        DAT_0052ae54 = 0;
        DAT_00732e8f = 0;
        FUN_004337a0();
        if (DAT_0052ae18 != 0) {
            FUN_0045f800();
        }
        FUN_004a84a0(DAT_0071ecf4);
        FUN_004a6a80();
        CAPTURE_ECX_EDX(extraout_ECX, extraout_EDX);
        FUN_004a42b0(extraout_ECX, extraout_EDX);
        FUN_004b2ae0();
        FUN_004a6f90();
        DAT_0052ae54 = 1;
        return;
    }
    DAT_0052ae54 = 0;
    DAT_00732e8f = 0;
    iVar7 = DAT_0071ecf4;
    if (DAT_006ffbe4 != 0) {
        iVar7 = 0;
        if (DAT_0052ae18 != 0) {
            thunk_FUN_004b67f9();
            CAPTURE_ECX(extraout_ECX_00);
            iVar7 = extraout_ECX_00;
        }
        DAT_006ffbe4 = 0;
    }
    FUN_00465970(iVar7);
    DAT_006da9e2 = 0;
    if (DAT_0052ae18 != 0) {
        DAT_0071f714 = 0x3000;
        FUN_0044ae00();
        FUN_004a6a80();
        FUN_00464b00();
        FUN_004a6f90();
    }
    sVar8 = 0;
    FUN_00433e70();
    if (0 < DAT_00732a38) {
        do {
            unsigned short uVar13;
            short sVar14;
            if ((sVar8 == DAT_006db7a4) &&
                (((DAT_006d9d94 == '\0' || (DAT_007092c1 != '\0')) &&
                  (sVar8 = DAT_00703ac0, (int)DAT_00703ac0 == DAT_00732a38)))) break;
            unaff_EDI = (unsigned int)sVar8;
            __builtin_memcpy(&uVar13, DAT_0071bb27 + (unsigned int)DAT_0071ecf4 * 0x5bd, 2);
            __builtin_memcpy(&sVar14, DAT_0071bb44 + (unsigned int)DAT_0071ecf4 * 0x5bd, 2);
            if (((uVar13 != unaff_EDI) || (sVar14 != 0)) || (DAT_00732910 != 0)) {
                iVar7 = (int)(unaff_EDI * 0x23 + (unsigned int)DAT_006d4988);
                if (*(unsigned char *)(iVar7 + 3) != 0) {
                    DAT_006db718 = *(unsigned short *)(DAT_0052686c + (unsigned int)*(unsigned char *)(iVar7 + 3) * 0x16);
                    switch (*(unsigned char *)(iVar7 + 2)) {
                    case 0:
                    case 1:
                    case 2:
                    case 3:
                    case 4:
                    case 5:
                    case 0xe: {
                        int iVar11, iVar12;
                        __builtin_memcpy(&iVar11, (unsigned char *)iVar7 + 0x1f, 4);
                        __builtin_memcpy(&iVar12, (unsigned char *)iVar11 + 0xa7, 4);
                        DAT_00732e54 = iVar12;
                        iVar7 = (int)(unsigned int)FUN_0045b880(unaff_EDI, (unsigned int)DAT_006db718);
                        break;
                    }
                    case 6:
                    case 7:
                        iVar7 = (int)(unsigned int)FUN_0045b880(unaff_EDI, (unsigned int)DAT_006db718);
                        break;
                    default:
                        goto next1;
                    case 0xb:
                        iVar7 = (int)(unsigned int)FUN_0045b880(unaff_EDI, (unsigned int)DAT_006db718);
                        break;
                    case 0xd:
                        iVar7 = (int)(unsigned int)FUN_0045b880(unaff_EDI, (unsigned int)DAT_006db718);
                    }
                    if (iVar7 != 0) {
                        FUN_00433e00(unaff_EDI, DAT_006db7b4);
                    }
                }
            }
next1:
            sVar8 = (short)(sVar8 + 1);
        } while (sVar8 < DAT_00732a38);
    }
    sVar8 = (short)DAT_00732a38;
    if ((int)(short)DAT_00732a38 < (int)((unsigned int)DAT_00703c88 + (unsigned int)DAT_00732a38)) {
        do {
            unaff_EDI = (unsigned int)sVar8;
            iVar7 = (int)(unaff_EDI * 0x23 + (unsigned int)DAT_006d4988);
            if (*(unsigned char *)(iVar7 + 3) != 0) {
                DAT_006db718 = *(unsigned short *)(DAT_0052686c + (unsigned int)*(unsigned char *)(iVar7 + 3) * 0x16);
                if ((7 < *(unsigned char *)(iVar7 + 2)) && (*(unsigned char *)(iVar7 + 2) < 0xc)) {
                    int iVar15, iVar16, iVar17;
                    __builtin_memcpy(&iVar15, (unsigned char *)iVar7 + 4, 4);
                    __builtin_memcpy(&iVar16, (unsigned char *)iVar7 + 8, 4);
                    __builtin_memcpy(&iVar17, (unsigned char *)iVar7 + 0xc, 4);
                    uVar6 = FUN_0045b990(iVar15, iVar16, iVar17, (unsigned int)DAT_006db718);
                    if ((short)uVar6 != 0) {
                        FUN_00433e00(unaff_EDI, DAT_006db7b4);
                    }
                }
            }
            sVar8 = (short)(sVar8 + 1);
        } while ((int)sVar8 < (int)((unsigned int)DAT_00703c88 + (unsigned int)DAT_00732a38));
    }
    FUN_004342f0();
    for (iVar7 = DAT_006db59c; iVar7 != 0; __builtin_memcpy(&iVar7, (unsigned char *)iVar7 + 8, 4)) {
        uVar3 = *(unsigned short *)(iVar7 + 4);
        unaff_EDI = (unaff_EDI & 0xffff0000u) | (unsigned int)uVar3;
        iVar9 = (int)(short)uVar3;
        iVar4 = (int)((unsigned int)iVar9 * 0x23);
        uVar10 = (unsigned int)DAT_006d4988 + (unsigned int)iVar4;
        bVar2 = *(unsigned char *)(uVar10 + 2);
        if (iVar9 < DAT_00732a38) {
            switch (bVar2) {
            case 0:
            case 1:
            case 2:
            case 3:
            case 4:
            case 5:
            case 0xe: {
                int iVar11, iVar12;
                __builtin_memcpy(&iVar11, (unsigned char *)uVar10 + 0x1f, 4);
                __builtin_memcpy(&iVar12, (unsigned char *)iVar11 + 0xa7, 4);
                DAT_00732e54 = iVar12;
                iVar9 = (int)((unsigned int)uVar3 * 0x23 + (unsigned int)DAT_006d4988);
                COMPUTE_VEC(iVar9);
                if (bVar2 == 0xe) {
                    DAT_0071e536 = 0;
                }
                iVar9 = (int)((unsigned int)DAT_006d4988 + (unsigned int)iVar4);
                CALL_832A0(iVar9, iVar9);
                if (bVar2 == 0xe) {
                    FUN_004010f0(uVar3);
                } else {
                    FUN_0045b420((int)((unsigned int)DAT_006d4988 + (unsigned int)iVar4));
                    FUN_0049c2b0(unaff_EDI);
                    FUN_00486310((int)((unsigned int)DAT_006d4988 + (unsigned int)iVar4));
                    DAT_00525730 = 0;
                }
                DAT_0071e536 = 1;
                break;
            }
            case 6:
            case 7:
                iVar9 = (int)((unsigned int)uVar3 * 0x23 + (unsigned int)DAT_006d4988);
                COMPUTE_VEC(iVar9);
                iVar9 = (int)((unsigned int)DAT_006d4988 + (unsigned int)iVar4);
                CALL_832A0(iVar9, iVar9);
                FUN_0049c590(uVar3);
                break;
            case 0xb:
                unaff_EDI = (unsigned int)uVar3 * 0x23 + (unsigned int)DAT_006d4988;
                COMPUTE_VEC(unaff_EDI);
                {
                    int iVar1 = (int)((unsigned int)DAT_006d4988 + (unsigned int)iVar4);
                    CALL_832A0(iVar1, iVar1);
                }
                FUN_0047f0f0(iVar9);
                break;
            case 0xd:
                unaff_EDI = (unsigned int)uVar3 * 0x23 + (unsigned int)DAT_006d4988;
                COMPUTE_VEC(unaff_EDI);
                {
                    int iVar1 = (int)((unsigned int)DAT_006d4988 + (unsigned int)iVar4);
                    CALL_832A0(iVar1, iVar1);
                }
                FUN_0047f0f0(iVar9);
            }
        } else if ((7 < bVar2) && (bVar2 < 0xc)) {
            iVar9 = (int)((unsigned int)uVar3 * 0x23 + (unsigned int)DAT_006d4988);
            COMPUTE_VEC(iVar9);
            CALL_832A0((unsigned char *)((unsigned int)DAT_006d4988 + (unsigned int)iVar4), 0);
            FUN_0045b420((int)((unsigned int)DAT_006d4988 + (unsigned int)iVar4));
            CAPTURE_ECX(this_);
            FUN_00412e60(this_, uVar3);
            DAT_00525730 = 0;
        }
    }
    if (DAT_0052ae18 == 0) {
        DAT_0071f714 = 0x3000;
        FUN_0044ae00();
        FUN_004a6a80();
        FUN_00464b00();
        FUN_004a6f90();
    }
    DAT_005279c8 = 1;
    FUN_00466020();
    DAT_005279c8 = 0;
    if (DAT_0052ae18 == 0) {
        FUN_0047f3d0(1);
        FUN_00442410();
    }
    DAT_006da534 = 0;
    DAT_0071b724 = 0;
    iVar7 = FUN_0048f530();
    DAT_006db5a8 = (int)((unsigned int)DAT_006db5a8 + (unsigned int)iVar7);
    DAT_006da534 = (short)DAT_006db5a8;
    FUN_00465c40();
    if (DAT_0052ae18 != 0) {
        FUN_0045f800();
    }
    FUN_004a84a0(DAT_0071ecf4);
    DAT_00525548 = 0;
    iVar7 = FUN_0048f530();
    DAT_006db5a8 = (int)((unsigned int)DAT_006db5a8 + (unsigned int)iVar7);
    DAT_00525548 = DAT_00525508;
    DAT_006da534 = (short)((short)DAT_006db5a8 - DAT_006da534);
    FUN_004a6a80();
    CAPTURE_ECX_EDX(extraout_ECX_01, extraout_EDX_00);
    FUN_004a42b0(extraout_ECX_01, extraout_EDX_00);
    FUN_004b2ae0();
    FUN_004a6f90();
    DAT_0052ae54 = 1;
    return;
}

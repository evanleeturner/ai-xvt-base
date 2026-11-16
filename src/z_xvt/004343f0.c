/* SPDX-License-Identifier: MIT */
/* z_xvt 004343f0 */
extern int *DAT_006db59c;
extern unsigned short DAT_006da9e2;
extern int DAT_006d4988;
extern int DAT_0071ecf4;
extern int DAT_0072f44c;
extern int DAT_0072f450;
extern int DAT_0072f448;
extern int DAT_006db7b4;
extern int DAT_006db7ac;
extern int DAT_006db7b0;
extern int DAT_00732e54;
extern int DAT_00732a38;
extern int DAT_00525730;
extern unsigned char DAT_0071e536;
extern char DAT_007092d9;
extern unsigned char DAT_0052686c[];
extern unsigned char DAT_0071bb1b[];
extern unsigned char DAT_0071bb1f[];
extern unsigned char DAT_0071bb23[];
extern unsigned char DAT_0071bb27[];
extern unsigned char DAT_0071b73c[];
extern unsigned char DAT_0071b74a[];

void __cdecl FUN_00465970(int);
int __cdecl FUN_0040ca60(int, int, int);
int __cdecl FUN_0040ca00(int, int, int);
int __cdecl FUN_0040ca30(int, int, int);
void __cdecl FUN_004351e0(unsigned int, int, int, int);
void __cdecl FUN_004832a0(short, short, short, int, int);
void __cdecl FUN_004010f0(unsigned short);
void __cdecl FUN_0045b420(int);
void __cdecl FUN_0049c2b0(unsigned int);
void __cdecl FUN_00486310(int);
void __cdecl FUN_00466020(void);
unsigned char __cdecl FUN_00444040(unsigned int, unsigned int);
unsigned int __cdecl FUN_004440e0(unsigned short);
void __cdecl FUN_00442720(unsigned short, unsigned char);
void __cdecl FUN_0049c590(unsigned short);
void __cdecl FUN_0047f0f0(unsigned int);
void __cdecl FUN_00412e60(unsigned short);
void __cdecl FUN_0047f3d0(int);
void __cdecl FUN_004a6a80(void);
void __cdecl FUN_00434990(unsigned int);
void __cdecl FUN_004a6f90(void);
void __cdecl FUN_00465c40(void);

void __cdecl FUN_004343f0(int param_1)
{
    char cVar1;
    unsigned int uVar2;
    int *piVar3;
    unsigned int uVar5;
    int iVar6;
    unsigned char bVar7;
    unsigned int iVar8;
    unsigned short uVar9;
    int t0, t1, t2;
    int a0, a1, a2;
    int b0, b1, b2;
    short sv[3];
    unsigned short us0;

    piVar3 = DAT_006db59c;
    DAT_006da9e2 = 0;
    do {
        if (DAT_006db59c == (int *)0) {
            DAT_006db59c = piVar3;
            return;
        }
        iVar6 = 0;
        uVar2 = (unsigned int)DAT_006db59c[1];
        __builtin_memcpy(&t0, (unsigned char *)(unsigned int)(DAT_006d4988 + 0xc + uVar2 * 0x23), 4);
        if (param_1 == 0) {
            if (t0 < -0x10000) goto LAB_00434451;
        }
        else if (-0x10001 < t0) {
LAB_00434451:
            iVar6 = 1;
        }
        iVar8 = uVar2 * 0x23;
        if (iVar6 != 0) {
            FUN_00465970(iVar6);
            iVar6 = (int)((unsigned int)DAT_006d4988 + iVar8);
            __builtin_memcpy(&a0, (unsigned char *)iVar6 + 4, 4);
            __builtin_memcpy(&b0, DAT_0071bb1b + (unsigned int)DAT_0071ecf4 * 0x5bd, 4);
            DAT_0072f44c = (int)((unsigned int)a0 - (unsigned int)b0);
            __builtin_memcpy(&a1, (unsigned char *)iVar6 + 8, 4);
            __builtin_memcpy(&b1, DAT_0071bb1f + (unsigned int)DAT_0071ecf4 * 0x5bd, 4);
            DAT_0072f450 = (int)((unsigned int)a1 - (unsigned int)b1);
            __builtin_memcpy(&a2, (unsigned char *)iVar6 + 0xc, 4);
            __builtin_memcpy(&b2, DAT_0071bb23 + (unsigned int)DAT_0071ecf4 * 0x5bd, 4);
            DAT_0072f448 = (int)((unsigned int)a2 - (unsigned int)b2);
            DAT_006db7b4 = FUN_0040ca60(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
            DAT_006db7ac = FUN_0040ca00(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
            DAT_006db7b0 = FUN_0040ca30(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
            iVar6 = (int)((unsigned int)DAT_006d4988 + iVar8);
            cVar1 = *(char *)(iVar6 + 2);
            uVar9 = (unsigned short)uVar2;
            switch (cVar1) {
            case 0: case 1: case 2: case 3: case 4: case 5: case 0xe:
                __builtin_memcpy(&us0, DAT_0052686c + (unsigned int)*(unsigned char *)(iVar6 + 3) * 0x16, 2);
                if ((int)(unsigned int)us0 < (*DAT_006db59c >> 4)) {
                    FUN_004351e0(uVar2, DAT_006db7ac, DAT_006db7b0, DAT_006db7b4);
                }
                else {
                    __builtin_memcpy(&t0, (unsigned char *)iVar6 + 0x1f, 4);
                    __builtin_memcpy(&t1, (unsigned char *)t0 + 0xa7, 4);
                    DAT_00732e54 = t1;
                    if (cVar1 == 0xe) {
                        DAT_0071e536 = 0;
                    }
                    __builtin_memcpy(sv, (unsigned char *)iVar6 + 0x10, 6);
                    FUN_004832a0(sv[2], sv[1], sv[0], 0, iVar6);
                    if (cVar1 == 0xe) {
                        FUN_004010f0(uVar9);
                    }
                    else {
                        FUN_0045b420((int)((unsigned int)DAT_006d4988 + iVar8));
                        FUN_0049c2b0(uVar2);
                        FUN_00486310((int)((unsigned int)DAT_006d4988 + iVar8));
                        DAT_00525730 = 0;
                    }
                    DAT_0071e536 = 1;
                    FUN_00466020();
                }
                __builtin_memcpy(&us0, DAT_0071bb27 + (unsigned int)DAT_0071ecf4 * 0x5bd, 2);
                __builtin_memcpy(&t0, (unsigned char *)(unsigned int)(DAT_006d4988 + iVar8 + 0x1b), 4);
                if ((us0 != uVar2) && (t0 != -1)) {
                    __builtin_memcpy(&iVar6, (unsigned char *)(unsigned int)(DAT_006d4988 + iVar8 + 0x1f), 4);
                    switch (*(unsigned char *)(iVar6 + 0x86)) {
                    case 0:
                        bVar7 = 0x3f;
                        break;
                    case 1:
                    case 4:
                        bVar7 = 0x37;
                        break;
                    case 2:
                        bVar7 = 0x33;
                        break;
                    default:
                        bVar7 = 0x3b;
                    }
                    if (((DAT_007092d9 != '\0') ||
                         ((__builtin_memcpy(&us0, DAT_0071b73c + (unsigned int)DAT_0071ecf4 * 0x5bd, 2),
                           uVar5 = (unsigned int)us0,
                           __builtin_memcpy(&t2, (unsigned char *)iVar6 + 0xa7, 4),
                           *(char *)(uVar5 + 0x1f0 + (unsigned int)t2) != '\0') ||
                          (FUN_00444040(uVar2, uVar5) == 0))) &&
                        ((uVar5 = FUN_004440e0(uVar9), (char)uVar5 == '\0') &&
                         (__builtin_memcpy(&us0, DAT_0071b74a + (unsigned int)DAT_0071ecf4 * 0x5bd, 2),
                          us0 != uVar2))) {
                        FUN_00442720(uVar9, bVar7);
                    }
                }
                break;
            case 6:
            case 7:
                __builtin_memcpy(sv, (unsigned char *)iVar6 + 0x10, 6);
                FUN_004832a0(sv[2], sv[1], sv[0], 0, iVar6);
                FUN_0049c590(uVar9);
                FUN_00466020();
                break;
            case 8:
            case 9:
                __builtin_memcpy(&us0, DAT_0052686c + (unsigned int)*(unsigned char *)(iVar6 + 3) * 0x16, 2);
                if (*DAT_006db59c >> 4 <= (int)(unsigned int)us0) {
                    DAT_006da9e2 = 0;
                    if (DAT_00732a38 <= (int)uVar2) goto LAB_004347b5;
                    __builtin_memcpy(sv, (unsigned char *)iVar6 + 0x10, 6);
                    FUN_004832a0(sv[2], sv[1], sv[0], 0, iVar6);
                    FUN_0047f0f0(uVar2);
                    goto LAB_004347d6;
                }
                FUN_004351e0(uVar2, DAT_006db7ac, DAT_006db7b0, DAT_006db7b4);
                break;
            case 10:
            case 11:
            case 13:
                DAT_006da9e2 = 0;
                if ((int)uVar2 < DAT_00732a38) {
                    __builtin_memcpy(sv, (unsigned char *)iVar6 + 0x10, 6);
                    FUN_004832a0(sv[2], sv[1], sv[0], 0, iVar6);
                    FUN_0047f0f0(uVar2);
                }
                else {
LAB_004347b5:
                    DAT_006da9e2 = 0;
                    __builtin_memcpy(sv, (unsigned char *)iVar6 + 0x10, 6);
                    FUN_004832a0(sv[2], sv[1], sv[0], 0, 0);
                    FUN_00412e60(uVar9);
                }
LAB_004347d6:
                FUN_00466020();
                FUN_0047f3d0(0);
                DAT_006da9e2 = 0;
            }
            __builtin_memcpy(&us0, DAT_0071bb27 + (unsigned int)DAT_0071ecf4 * 0x5bd, 2);
            if (us0 == uVar2) {
                bVar7 = 0x2f;
LAB_00434828:
                FUN_00442720(uVar9, bVar7);
            }
            else {
                __builtin_memcpy(&us0, DAT_0071b74a + (unsigned int)DAT_0071ecf4 * 0x5bd, 2);
                if (us0 == uVar2) {
                    bVar7 = 0x3b;
                    goto LAB_00434828;
                }
            }
            FUN_004a6a80();
            FUN_00434990(uVar2);
            FUN_004a6f90();
            FUN_00465c40();
        }
        DAT_006db59c = (int *)DAT_006db59c[2];
    } while (1);
}

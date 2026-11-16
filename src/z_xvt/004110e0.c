/* SPDX-License-Identifier: MIT */
/* z_xvt 004110e0 */
/* Unresolved: callee signatures were not provided, so every prototype below is inferred
   from call sites. FUN_00478470 is declared as returning unsigned int (Ghidra truncates
   it to ushort in most places). The record base DAT_006d4988 has a 0x23-byte stride, so
   multi-byte record fields are accessed with memcpy-based macros. The same applies to
   the odd-offset fields of the puVar5 struct, which are kept as raw byte offsets because
   no struct layout was supplied. The upper bits of the return value are decompiler
   artifacts and are reproduced as written. */

#define LD16(p) ({ unsigned short t_; __builtin_memcpy(&t_, (const void *)(p), 2); t_; })
#define LD32(p) ({ unsigned int t_; __builtin_memcpy(&t_, (const void *)(p), 4); t_; })
#define ST16(p, v) do { unsigned short t_ = (unsigned short)(v); __builtin_memcpy((void *)(p), &t_, 2); } while (0)
#define ST32(p, v) do { unsigned int t_ = (unsigned int)(v); __builtin_memcpy((void *)(p), &t_, 4); } while (0)
#define LDP(p) ((unsigned char *)LD32(p))
#define R30(o) (DAT_006d4988 + iVar30 + (o))
#define SEL (DAT_0071ecf4 * 0x5bd)

extern int DAT_00525550;
extern unsigned char *DAT_006d4988;
extern int DAT_006d4980;
extern unsigned char DAT_0070ad44[];
extern unsigned char DAT_0070b215[];
extern unsigned char DAT_0070ad49[];
extern unsigned char DAT_0070ad40[];
extern unsigned short DAT_0070ac42[];
extern unsigned short DAT_0070ac44[];
extern unsigned short DAT_00703abc;
extern int _DAT_00732a3c;
extern unsigned short DAT_00732a3c;
extern int DAT_00732a38;
extern int DAT_00703c88;
extern char DAT_007092d4;
extern char DAT_007092c1;
extern char DAT_007092c0;
extern unsigned char DAT_00730880;
extern int DAT_0071ecf4;
extern unsigned char DAT_0071b730[];
extern unsigned char DAT_0071b73e[];
extern unsigned char DAT_0071b73c[];
extern unsigned char DAT_0071b741[];
extern unsigned short _DAT_0071e6a6;
extern unsigned short DAT_00732910;
extern unsigned short _DAT_00732950;
extern unsigned short _DAT_00732952;
extern char DAT_0085e795;
extern unsigned char DAT_0052686c[];
extern unsigned char DAT_0052686a[];
extern unsigned char DAT_0052b09b[];
extern unsigned char DAT_0052b08b[];
extern unsigned short DAT_00523b70[];
extern unsigned char DAT_00523b88[];
extern unsigned char DAT_00523b98[];
extern unsigned char DAT_00523bb0[];
extern unsigned char DAT_006d4a86[];
extern unsigned char DAT_006d4ab0[];

int FUN_0042b150(int);
unsigned int FUN_0044a6c0(unsigned int, unsigned int);
unsigned short FUN_0041a1d0(unsigned int, short, unsigned short, unsigned int);
int FUN_0040b730(int, unsigned int, int, int, unsigned int, unsigned short);
int FUN_0040c070(int, int, unsigned int, int);
unsigned int FUN_00478470(void);
unsigned int FUN_0040a1a0(unsigned int, unsigned int, int);
unsigned int FUN_0044a600(unsigned int, unsigned int);
unsigned int FUN_00478320(unsigned int, unsigned int);
void FUN_004a1d70(int, int);
void FUN_004a34c0(unsigned int, void *, int);
unsigned char FUN_00444040(unsigned int, unsigned int);
unsigned short *FUN_0042b1e0(int, unsigned int);
unsigned int FUN_004784d0(void);
void FUN_004a6a80(void);
void FUN_004afb90(void);
void FUN_004a6f90(void);
void FUN_004a01a0(unsigned int, unsigned int);
unsigned int FUN_004a10d0(unsigned int, int);
void FUN_00423d00(int);
void FUN_00443b90(int, unsigned int, unsigned int);
void FUN_0049f9c0(unsigned short, unsigned short, int);
int FUN_0044ad00(unsigned int);
unsigned int FUN_004490f0(unsigned int);
int FUN_00449790(unsigned int, unsigned int);
int FUN_00449b30(unsigned int, unsigned int);
unsigned int FUN_00421ff0(unsigned int, char);

unsigned int FUN_004110e0(unsigned int param_1, unsigned int param_2, unsigned int param_3,
                          unsigned int param_4, unsigned int param_5, unsigned short param_6,
                          unsigned int param_7, unsigned short param_8, unsigned int param_9)
{
    unsigned char *piVar1;
    unsigned char *psVar2;
    char cVar3;
    unsigned char bVar4;
    unsigned char *puVar5;
    char *pcVar6;
    int iVar7;
    unsigned char bVar8;
    unsigned char bVar9;
    unsigned short uVar10;
    unsigned char bVar11;
    unsigned char bVar12;
    unsigned short uVar13;
    short sVar14;
    unsigned short uVar15;
    short sVar16;
    unsigned short uVar17;
    unsigned int uVar18;
    int iVar19;
    unsigned short *puVar20;
    unsigned short uVar22;
    unsigned int uVar21;
    int iVar23;
    int iVar24;
    int iVar25;
    unsigned int uVar26;
    unsigned short uVar27;
    unsigned int uVar28;
    unsigned int uVar29;
    int iVar30;
    unsigned char local_1f;
    unsigned short local_1c;
    unsigned short uStack_1a;

    uVar29 = param_4;
    uVar10 = DAT_00703abc;
    bVar9 = 0;
    bVar12 = 0;
    local_1f = 1;
    if (DAT_00525550 != 0) {
        return 1;
    }
    uVar18 = param_1 & 0xffff;
    iVar30 = uVar18 * 0x23;
    uVar28 = (unsigned int)DAT_006d4988[iVar30 + 0x16];
    iVar19 = uVar28 * 0x566;
    if (DAT_0070ad44[uVar28 * 0x566] == 0x14 || DAT_0070b215[iVar19] == 0x14) {
        return ((((unsigned int)iVar19 >> 16) & 0xffff) << 16) | 1;
    }
    uVar28 = param_3 & 0xffff;
    if (((int)_DAT_00732a3c <= (int)uVar28) && ((int)uVar28 < DAT_006d4980) &&
        ((uVar26 = (unsigned int)DAT_006d4988[uVar28 * 0x23 + 0x16], iVar19 = uVar26 * 0x566,
          DAT_0070ad44[uVar26 * 0x566] == 0x14 || (DAT_0070b215[iVar19] == 0x14)))) {
        return ((((unsigned int)iVar19 >> 16) & 0xffff) << 16) | 1;
    }
    puVar5 = LDP(LDP(DAT_006d4988 + iVar30 + 0x1f) + 0xa7);
    iVar19 = FUN_0042b150((int)puVar5);
    DAT_00703abc = *(unsigned short *)(iVar19 + 0x16);
    if ((unsigned short)param_3 == 0xfffe) {
        bVar12 = 1;
        param_3 = (param_3 & 0xffff0000) | 0xffff;
        uStack_1a = 0x35;
        param_4 = 0;
        uVar29 = uVar29 & 0xffff;
    }
    else if ((unsigned short)param_3 == 0xffff) {
        uStack_1a = 0x35;
LAB_004112ae:
        uVar29 = 0x20000;
    }
    else {
        iVar24 = (int)(DAT_006d4988 + uVar28 * 0x23);
        if (LD32(iVar24 + 0x1f) == 0) {
            uStack_1a = (unsigned short)*(unsigned char *)(iVar24 + 3);
            if (0x7fff < *(unsigned short *)(DAT_0052686c + (unsigned int)uStack_1a * 0x16)) goto LAB_004112ae;
            uVar29 = (unsigned int)*(unsigned short *)(DAT_0052686c + (unsigned int)uStack_1a * 0x16) << 2;
        }
        else {
            uVar29 = (unsigned int)LD16(LD32(iVar24 + 0x1f) + 0x7d);
            uStack_1a = (unsigned short)*(unsigned char *)(iVar24 + 3);
        }
    }
    cVar3 = (char)DAT_006d4988[2 + iVar30];
    if ((cVar3 == 4) || (cVar3 == 5)) {
        uVar29 = uVar29 >> 4;
    }
    if (cVar3 == 3) {
        uVar29 = uVar29 >> 2;
    }
    if (0x7ffe < uVar29) {
        uVar29 = 0x7ffe;
    }
    uVar13 = (unsigned short)uVar29;
    iVar24 = (int)(short)uVar13;
    ST32(puVar5 + 0xcd, LD32(puVar5 + 0xcd) + iVar24);
    local_1c = 0xffff;
    if ((unsigned short)param_3 == 0xffff) {
LAB_0041147d:
        if (((unsigned short)param_3 == 0xffff) ||
            (iVar23 = (int)(DAT_006d4988 + (param_3 & 0xffff) * 0x23), LD32(iVar23 + 0x1f) != 0)) {
            if (bVar12) {
                ST32(puVar5 + 0xd9, LD32(puVar5 + 0xd9) + iVar24);
            }
        }
        else if (*(char *)(iVar23 + 2) == '\n') {
            ST32(puVar5 + 0xd5, LD32(puVar5 + 0xd5) + iVar24);
        }
    }
    else {
        iVar7 = (param_3 & 0xffff) * 0x23;
        iVar23 = (int)(DAT_006d4988 + iVar7);
        pcVar6 = (char *)LDP(DAT_006d4988 + 0x1f + iVar7);
        if (pcVar6 == (char *)0) goto LAB_0041147d;
        cVar3 = *pcVar6;
        if (cVar3 == '\0') {
            iVar25 = (int)LD32(iVar23 + 0x1b);
            local_1c = (unsigned short)param_3;
        }
        else {
            local_1c = LD16(LD32(iVar23 + 0x1f) + 0x83);
            iVar25 = (int)LD32((unsigned int)local_1c * 0x23 + 0x1b + DAT_006d4988);
            if ((cVar3 == '\x01') &&
                (pcVar6 = (char *)LDP(LD32(iVar23 + 0x1f) + 0xa3), pcVar6 != (char *)0)) {
                iVar25 = (int)*pcVar6;
            }
        }
        if (iVar25 == -1) {
            if (cVar3 == '\0') {
                ST32(puVar5 + 0xd5, LD32(puVar5 + 0xd5) + iVar24);
            }
            else {
                uVar29 = (unsigned int)local_1c;
                iVar23 = (int)(DAT_006d4988 + uVar29 * 0x23);
                if (*(char *)(iVar23 + 2) == '\0') {
                    piVar1 = puVar5 +
                             (unsigned int)DAT_0070ad49[(unsigned int)*(unsigned char *)(iVar23 + 0x16) * 0x566] * 4 +
                             0x1c1;
                    ST32(piVar1, LD32(piVar1) + iVar24);
                }
                else if (((int)uVar29 < DAT_00732a38) || (DAT_00703c88 + DAT_00732a38 <= (int)uVar29)) {
                    ST32(puVar5 + 0xd9, LD32(puVar5 + 0xd9) + iVar24);
                }
                else {
                    ST32(puVar5 + 0xdd, LD32(puVar5 + 0xdd) + iVar24);
                }
            }
        }
        else {
            ST32(puVar5 + (unsigned int)iVar25 * 4 + 0x1a1,
                 LD32(puVar5 + (unsigned int)iVar25 * 4 + 0x1a1) + iVar24);
            if (*LDP(DAT_006d4988 + 0x1f + iVar7) == 0) {
                ST32(puVar5 + 0xd5, LD32(puVar5 + 0xd5) + iVar24);
            }
        }
        piVar1 = puVar5 + (unsigned int)DAT_006d4988[(unsigned int)local_1c * 0x23 + 0x16] * 4 + 0xe1;
        ST32(piVar1, LD32(piVar1) + iVar24);
    }
    if ((int)LD32(R30(0x1b)) != -1) {
        ST32(puVar5 + 0xd1, LD32(puVar5 + 0xd1) + iVar24);
    }
    if ((short)param_2 != -1) {
        if ((int)(short)LD16(puVar5 + 0x20b) + (int)(short)LD16(puVar5 + 0x20d) == 0) {
            if (DAT_007092d4 == '\0') {
LAB_0041153c:
                uVar29 = FUN_0044a6c0((unsigned int)DAT_006d4988[3 + iVar30], (param_2 & 0xffff) - 1);
            }
            else {
                uVar29 = FUN_0044a6c0((unsigned int)DAT_006d4988[3 + iVar30], (param_2 & 0xffff) - 1);
            }
            if (uVar29 != 0) {
                uVar13 = FUN_0041a1d0(param_1, (short)param_2, uVar13,
                                      ((unsigned int)uStack_1a << 16) | local_1c);
            }
        }
        else if (DAT_007092d4 == '\0') goto LAB_0041153c;
    }
    cVar3 = (char)DAT_006d4988[2 + iVar30];
    if (cVar3 == '\x0e') {
        uVar13 = 0;
    }
    psVar2 = puVar5 + (param_4 & 0xffff) * 2 + 0x20b;
    sVar16 = (short)LD16(psVar2);
    if ((short)uVar13 < sVar16) {
        ST16(psVar2, sVar16 - uVar13);
        iVar24 = (int)LD32(R30(0x1b));
        if (iVar24 != -1) {
            DAT_0070ac42[iVar24 * 8] = 0x3b;
            DAT_00730880 = (unsigned char)param_4;
            uVar29 = (param_4 & 0xffff) ^ 1;
            sVar16 = (short)LD16(puVar5 + uVar29 * 2 + 0x20b);
            if ((short)LD16(psVar2) < sVar16) {
                sVar14 = (short)(((int)sVar16 - (int)(short)LD16(psVar2)) / 2);
                ST16(puVar5 + uVar29 * 2 + 0x20b, sVar16 - sVar14);
                ST16(psVar2, (short)LD16(psVar2) + sVar14);
                puVar5[0x210] = 1;
            }
        }
        if (LD32(DAT_0071b730 + SEL) != uVar18) {
            FUN_0040b730(DAT_0071ecf4, uVar18, 3, -1, uVar18, 0x2000);
        }
LAB_00411efc:
        if (DAT_00525550 == 0) {
            uVar29 = 0;
            if (((puVar5[7] != 0) && (puVar5[7] != 5)) ||
                (uVar29 = 0, LD16(puVar5 + 0xc3) < LD16(puVar5 + 199)))
                goto LAB_004126c7;
            if (((unsigned short)param_3 != 0xffff) &&
                (pcVar6 = (char *)LDP(DAT_006d4988 + (param_3 & 0xffff) * 0x23 + 0x1f),
                 pcVar6 != (char *)0)) {
                if (*pcVar6 != '\0') {
                    param_3 = ((((unsigned int)pcVar6 >> 16) & 0xffff) << 16) | LD16(pcVar6 + 0x83);
                }
                FUN_004a01a0(param_3, param_1);
            }
            uVar29 = FUN_004a10d0(uVar18, 0);
            iVar24 = (int)LD32(R30(0x1b));
            if (iVar24 != -1) {
                FUN_00423d00(iVar24);
            }
            if (DAT_007092c1 == '\0') {
                iVar24 = (int)LD32(R30(0x1b));
                if (iVar24 != -1) {
                    FUN_00443b90(iVar24, (unsigned int)local_1c, uVar29);
                }
            }
            else if ((int)LD32(R30(0x1b)) == DAT_0071ecf4) {
                DAT_007092c0 = 1;
            }
            if ((int)LD32(R30(0x1b)) != -1) {
                _DAT_0071e6a6 = 0;
            }
            uVar15 = (unsigned short)param_1;
            FUN_0049f9c0(uVar15, (unsigned short)DAT_006d4988[iVar30 + 0x16], 2);
            uVar13 = DAT_00732a3c;
            if ((int)(_DAT_00732a3c & 0xffff) < DAT_006d4980) {
                do {
                    iVar24 = (int)(DAT_006d4988 + (unsigned int)uVar13 * 0x23);
                    if ((*(char *)(iVar24 + 3) != '\0') &&
                        (iVar24 = (int)LDP(LDP(iVar24 + 0x1f) + 0xa7),
                         LD16(iVar24 + 0x66) == uVar15)) {
                        ST16(iVar24 + 0x66, 0xffff);
                    }
                    uVar13 = uVar13 + 1;
                } while ((int)(unsigned int)uVar13 < DAT_006d4980);
            }
            uVar13 = DAT_00732a3c;
            if ((int)(_DAT_00732a3c & 0xffff) < DAT_006d4980) {
                do {
                    iVar24 = (int)(DAT_006d4988 + (unsigned int)uVar13 * 0x23);
                    if ((*(char *)(iVar24 + 3) != '\0') &&
                        (iVar24 = (int)LDP(LDP(iVar24 + 0x1f) + 0xa7),
                         LD16(iVar24 + 0x68) == uVar15)) {
                        ST16(iVar24 + 0x68, 0xffff);
                    }
                    uVar13 = uVar13 + 1;
                } while ((int)(unsigned int)uVar13 < DAT_006d4980);
            }
            FUN_004a34c0(param_1, puVar5, 0x87);
            uVar13 = LD16(DAT_0071b73e + SEL);
            if (DAT_006d4988[iVar30 + 0x16] == uVar13) {
                iVar24 = FUN_0040b730(DAT_0071ecf4, uVar18, 6, -1, uVar18, 0xc000);
                if (iVar24 == 0) {
                    uVar13 = 0xc000;
                    iVar24 = 0x10;
                    uVar29 = 0xffffffff;
LAB_004121d4:
                    FUN_0040b730(DAT_0071ecf4, uVar29, iVar24, -1, uVar18, uVar13);
                }
            }
            else {
                uVar29 = (unsigned int)local_1c;
                if ((((uVar29 != 0xffffffff) && ((int)uVar29 < DAT_006d4980)) &&
                     (LD32(DAT_0071b730 + SEL) != uVar29)) &&
                    (DAT_006d4988[uVar29 * 0x23 + 0x16] == uVar13)) {
                    cVar3 = (char)DAT_006d4988[iVar30 + 2];
                    if ((cVar3 == '\0') || (cVar3 == '\x01')) {
                        uVar13 = 0xffff;
                        iVar24 = 9;
                    }
                    else {
                        uVar13 = 0xa000;
                        iVar24 = 10;
                    }
                    goto LAB_004121d4;
                }
            }
            iVar24 = FUN_0040c070(1, 0x1b, uVar18, 0xffff);
            if (((iVar24 != 0) &&
                 ((bVar12 = FUN_00444040(param_1, (unsigned int)LD16(DAT_0071b73c + SEL)), bVar12 == 0))) &&
                (iVar24 = (int)LD32(DAT_0071b730 + SEL), iVar24 != -1)) {
                cVar3 = (char)DAT_006d4988[(unsigned int)iVar24 * 0x23 + 3];
                if ((((cVar3 == '\x01') || (cVar3 == '\x02')) || (cVar3 == '\x03')) ||
                    ((cVar3 == '\x0e' || (cVar3 == '\x04')))) {
                    bVar12 = 1;
                }
                else {
                    bVar12 = 0;
                }
                if (bVar12) {
                    uVar29 = 0x54;
                }
                else {
                    uVar29 = 0x24;
                }
                FUN_0040a1a0(uVar29, 0xffffffff, DAT_0071ecf4);
            }
            uVar28 = (unsigned int)DAT_006d4988[3 + iVar30];
            iVar24 = FUN_0044ad00(uVar28);
            if (iVar24 == 0) {
                bVar4 = puVar5[4];
                uVar13 = FUN_00478470();
                for (uVar13 = (uVar13 & 0x3fff) + 0x2000;
                     LD16(DAT_0052b09b + (unsigned int)bVar4 * 0xef) < uVar13; uVar13 = uVar13 >> 1) {
                }
                uVar15 = FUN_00478470();
                if (uVar15 < 0x8000) {
                    uVar13 = -uVar13;
                }
                ST16(LDP(DAT_006d4988 + 0x1f + iVar30) + 0x77, uVar13);
                uVar29 = FUN_00478470();
                sVar16 = ((short)(uVar29 & 0xffff0007) + 8) * 0xec;
                uVar29 = ((((uVar29 & 0xffff0007) >> 16) & 0xffff) << 16) | (unsigned short)sVar16;
                ST16(LDP(DAT_006d4988 + 0x1f + iVar30) + 0x7f, sVar16);
                puVar5[7] = 3;
            }
            else if (((*(unsigned short *)(DAT_0052686c + (unsigned int)uStack_1a * 0x16) < 0x448) ||
                      (DAT_0052686a[(unsigned int)uStack_1a * 0x16] != 0)) &&
                     ((uStack_1a != 0x3a &&
                       ((short)LD16(LDP(DAT_006d4988 + 0x1f + iVar30) + 0x79) != 0)))) {
                uVar13 = FUN_00478470();
                if ((uVar13 < 0x4000) && ((int)LD32(R30(0x1b)) != -1)) {
                    ST16(LDP(DAT_006d4988 + iVar30 + 0x1f) + 0x7f, 1);
                    uVar29 = FUN_0040a1a0(0x15, uVar18, DAT_0071ecf4);
                    local_1f = 0;
                    puVar5[7] = 4;
                }
                else {
                    uVar29 = FUN_004490f0(uVar28);
                    uVar17 = (unsigned short)uVar29;
                    local_1c = 0;
                    uVar13 = 0;
                    uVar15 = 0;
                    if (1 < uVar17) {
                        uVar27 = 0;
                        uVar15 = FUN_00478470();
                        uVar15 = uVar15 & 1;
                        if (uVar17 != 0) {
                            do {
                                uVar26 = (unsigned int)uVar27;
                                iVar24 = FUN_00449790(uVar28, uVar26);
                                if ((puVar5[uVar26 + 0x2ca] == 0) &&
                                    ((iVar24 == 2 || (iVar24 == 0x14)))) {
                                    if (uVar15 == 0) {
                                        iVar24 = FUN_00449b30(uVar28, uVar26);
                                        if (0 < iVar24) break;
                                    }
                                    else {
                                        iVar24 = FUN_00449b30(uVar28, uVar26);
                                        if (iVar24 < 0) break;
                                    }
                                }
                                uVar27 = uVar27 + 1;
                            } while (uVar27 < uVar17);
                            if ((uVar27 < uVar17) &&
                                (uVar28 = FUN_00421ff0(param_1, (char)uVar27), (short)uVar28 != -1)) {
                                uVar13 = FUN_00478470();
                                uVar13 = (uVar13 & 0x3fff) + 0x4000;
                                uVar17 = FUN_00478470();
                                local_1c = (uVar17 & 0x7ff) + 0x800;
                                uVar17 = local_1c;
                                if (uVar15 != 0) {
                                    uVar17 = -local_1c;
                                    uVar13 = -uVar13;
                                }
                                iVar23 = (uVar28 & 0xffff) * 0x23;
                                ST16(LDP(DAT_006d4988 + 0x1f + iVar23) + 0x77, uVar13);
                                psVar2 = DAT_006d4988 + 0x10 + iVar23;
                                ST16(psVar2, LD16(psVar2) + uVar17);
                                *(LDP(DAT_006d4988 + 0x1f + iVar23) + 0x90) = 1;
                                iVar24 = (int)LDP(DAT_006d4988 + 0x1f + iVar23);
                                *(unsigned char *)(iVar24 + 0x89) = *(unsigned char *)(iVar24 + 0x90);
                                DAT_006d4988[0x1a + iVar23] = 2;
                                puVar5[uVar27 + 0x2ca] = 4;
                                uVar28 = FUN_004784d0();
                                if ((uVar28 & 0x8000) == 0) {
                                    uVar28 = 0x1b;
                                }
                                else {
                                    uVar28 = 0x1a;
                                }
                                FUN_0040a1a0(uVar28, uVar18, DAT_0071ecf4);
                                local_1f = 0;
                            }
                        }
                    }
                    bVar4 = puVar5[4];
                    uVar17 = FUN_00478470();
                    for (uVar17 = (uVar17 & 0x3fff) + 0x2000;
                         LD16(DAT_0052b09b + (unsigned int)bVar4 * 0xef) < uVar17; uVar17 = uVar17 >> 1) {
                    }
                    if (uVar13 < 0x8000) {
                        uVar17 = -uVar17;
                    }
                    ST16(LDP(DAT_006d4988 + 0x1f + iVar30) + 0x77, uVar17);
                    if (local_1c != 0) {
                        local_1c = local_1c >> 1;
                        if (uVar15 == 0) {
                            local_1c = -local_1c;
                        }
                        psVar2 = DAT_006d4988 + 0x10 + iVar30;
                        ST16(psVar2, LD16(psVar2) + local_1c);
                        *(LDP(DAT_006d4988 + 0x1f + iVar30) + 0x90) = 1;
                        iVar24 = (int)LDP(DAT_006d4988 + 0x1f + iVar30);
                        *(unsigned char *)(iVar24 + 0x89) = *(unsigned char *)(iVar24 + 0x90);
                    }
                    puVar5[7] = 3;
                    iVar30 = (int)(DAT_006d4988 + iVar30);
                    if ((int)LD32(iVar30 + 0x1b) == -1) {
                        uVar13 = FUN_00478470();
                        uVar15 = FUN_00478470();
                        ST16(LDP(iVar30 + 0x1f) + 0x7f, ((uVar13 & 3) + (uVar15 & 7) + 1) * 0xec);
                    }
                    else {
                        ST16(LDP(iVar30 + 0x1f) + 0x7f, 0x2c4);
                    }
                    uVar29 = uVar29 & 0xffff;
                    puVar5[uVar29 + 0x2ca] = 2;
                }
            }
            else {
                ST16(LDP(DAT_006d4988 + 0x1f + iVar30) + 0x7f, 1);
                uVar29 = FUN_0040a1a0(0x15, uVar18, DAT_0071ecf4);
                local_1f = 0;
                puVar5[7] = 4;
            }
            goto LAB_004126c7;
        }
    }
    else {
        if ((((DAT_0085e795 == '\x02') && (cVar3 != '\0')) && (sVar16 != 0)) &&
            (LD16(puVar5 + 0xc3) == 0)) {
            FUN_0040c070(1, 0x10, uVar18, 0xffff);
        }
        uVar13 = uVar13 - (short)LD16(psVar2);
        ST16(psVar2, 0);
        if ((int)LD32(R30(0x1b)) != -1) {
            if ((((short)param_4 == 0) && (!bVar12)) && (uVar15 = FUN_00478470(), uVar15 < 0x1000)) {
                FUN_0040a1a0(0x56, 0xffffffff, (int)LD32(R30(0x1b)));
            }
            uVar29 = (param_4 & 0xffff) ^ 1;
            sVar16 = (short)LD16(puVar5 + uVar29 * 2 + 0x20b);
            if (sVar16 != 0) {
                ST16(puVar5 + uVar29 * 2 + 0x20b, sVar16 - sVar16 / 2);
                ST16(psVar2, (short)LD16(psVar2) + sVar16 / 2);
                puVar5[0x210] = 1;
            }
        }
        if (uVar13 == 0) goto LAB_00411efc;
        if (((uStack_1a == 0x8d) || (uStack_1a == 0x8e)) || (uStack_1a == 0x93)) {
            if (LD16(puVar5 + 0xc9) < 1000) {
                if (uStack_1a == 0x8d) {
                    ST16(puVar5 + 0xc9, LD16(puVar5 + 0xc9) + 1);
                }
                if (uStack_1a == 0x8e) {
                    ST16(puVar5 + 0xc9, (short)LD16(puVar5 + 0xc9) + 2);
                }
                if (uStack_1a == 0x93) {
                    ST16(puVar5 + 0xc9, (short)LD16(puVar5 + 0xc9) + 4);
                }
            }
            sVar16 = (short)LD16(DAT_0052b08b + (unsigned int)puVar5[4] * 0xef) -
                     (short)LD16(puVar5 + 0xc9);
            if (((short)LD16(puVar5 + 0x1df) != 0) && (sVar16 < 0xb)) {
                if (0 < (short)uVar13) {
                    uVar29 = (unsigned short)(uVar13 + 199) / 200;
                    do {
                        uVar28 = 0;
                        do {
                            if ((LD16(puVar5 + 0x1df) & DAT_00523b70[uVar28]) != 0) {
                                bVar9 = 1;
                                ST16(puVar5 + 0x1df, (DAT_00523b70[uVar28] ^ 0x3ff) & LD16(puVar5 + 0x1df));
                                break;
                            }
                            uVar13 = (short)uVar28 + 1;
                            uVar28 = (unsigned int)uVar13;
                        } while (uVar13 < 10);
                        if (((unsigned short)uVar28 < 10) && ((int)LD32(R30(0x1b)) != -1)) {
                            iVar24 = uVar28 * 2;
                            ST16(puVar5 + iVar24 + 0x29e, 0);
                            ST16(puVar5 + iVar24 + 0x2b4, *(unsigned short *)(DAT_00523b98 + iVar24));
                        }
                        uVar13 = (short)uVar29 - 1;
                        uVar29 = (unsigned int)uVar13;
                    } while (uVar13 != 0);
                }
                if ((sVar16 < 1) && ((int)LD32(R30(0x1b)) != -1)) {
                    uVar29 = 0;
                    do {
                        iVar24 = uVar29 * 2;
                        if ((LD16(puVar5 + 0x1df) & DAT_00523b70[uVar29]) != 0) {
                            ST16(puVar5 + 0x1df, (DAT_00523b70[uVar29] ^ 0x3ff) & LD16(puVar5 + 0x1df));
                            ST16(puVar5 + iVar24 + 0x29e, 0);
                            ST16(puVar5 + iVar24 + 0x2b4, *(unsigned short *)(DAT_00523b98 + iVar24));
                        }
                        uVar13 = (short)uVar29 + 1;
                        uVar29 = (unsigned int)uVar13;
                    } while (uVar13 < 10);
                    ST16(puVar5 + 0x1df, 0);
                }
                bVar8 = 0;
                if ((short)LD16(puVar5 + 0x1df) == 0) {
                    ST16(puVar5 + 0xc9, LD16(DAT_0052b08b + (unsigned int)puVar5[4] * 0xef));
                    ST16(puVar5 + 0x20b, 0);
                    ST16(puVar5 + 0x20d, 0);
                    FUN_004a34c0(param_1, puVar5, 0x88);
                    iVar24 = FUN_0040c070(1, 0x1c, uVar18, 0xffff);
                    if (((iVar24 != 0) &&
                         ((bVar11 = FUN_00444040(param_1, (unsigned int)LD16(DAT_0071b73c + SEL)), bVar11 == 0))) &&
                        (iVar24 = (int)LD32(DAT_0071b730 + SEL), iVar24 != -1)) {
                        cVar3 = (char)DAT_006d4988[(unsigned int)iVar24 * 0x23 + 3];
                        if (((cVar3 == '\x01') || (cVar3 == '\x02')) ||
                            ((cVar3 == '\x03' || ((cVar3 == '\x0e' || (cVar3 == '\x04')))))) {
                            bVar8 = 1;
                        }
                        if (bVar8) {
                            uVar29 = 0x54;
                        }
                        else {
                            uVar29 = 0x24;
                        }
                        FUN_0040a1a0(uVar29, 0xffffffff, DAT_0071ecf4);
                    }
                    if ((int)LD32(R30(0x1b)) == -1) {
                        uVar29 = 0;
                        if (puVar5[0x213] != 0) {
                            do {
                                if (puVar5[(uVar29 + 0xd1) * 4] == 2) {
                                    puVar20 = FUN_0042b1e0((int)puVar5, uVar29);
                                    *puVar20 = 0xffff;
                                }
                                uVar29 = uVar29 + 1;
                            } while (uVar29 < puVar5[0x213]);
                        }
                        uVar29 = 0;
                        if (puVar5[0x211] != 0) {
                            do {
                                puVar5[uVar29 + 0x216] = 0;
                                uVar29 = uVar29 + 1;
                            } while (uVar29 < puVar5[0x211]);
                        }
                    }
                    uVar29 = (unsigned int)DAT_006d4988[0x16 + iVar30];
                    iVar24 = uVar29 * 0x126;
                    *(unsigned short *)(DAT_006d4a86 + iVar24) =
                        (unsigned short)(*(unsigned short *)(DAT_006d4a86 + iVar24) + 1);
                    if (DAT_0070ad40[uVar29 * 0x566] == puVar5[0xb0]) {
                        DAT_006d4ab0[iVar24] = 1;
                    }
                }
            }
            if ((int)LD32(R30(0x1b)) == DAT_0071ecf4) {
                FUN_0040a1a0(0x21, uVar18, DAT_0071ecf4);
            }
        }
        else {
            if (((short)param_2 != -1) &&
                (uVar29 = FUN_0044a600((unsigned int)DAT_006d4988[3 + iVar30], (param_2 & 0xffff) - 1),
                 uVar29 != 0)) {
                uVar13 = FUN_0041a1d0(param_1, (short)param_2, uVar13,
                                      ((unsigned int)uStack_1a << 16) | local_1c);
            }
            uVar15 = LD16(puVar5 + 0xc3);
            ST16(puVar5 + 0xc3, uVar13 + uVar15);
            if ((DAT_0085e795 == '\x02') && (DAT_006d4988[2 + iVar30] != 0)) {
                uVar29 = FUN_00478320((((unsigned int)DAT_006d4988 >> 16) << 16) | LD16(puVar5 + 199), 0xf333);
                if ((uVar15 < (unsigned short)uVar29) && ((unsigned short)uVar29 <= LD16(puVar5 + 0xc3))) {
                    iVar24 = 0x16;
                }
                else {
                    uVar29 = FUN_00478320(((uVar29 >> 16) << 16) | LD16(puVar5 + 199), 0xc000);
                    if ((uVar15 < (unsigned short)uVar29) && ((unsigned short)uVar29 <= LD16(puVar5 + 0xc3))) {
                        iVar24 = 0x15;
                    }
                    else {
                        uVar29 = FUN_00478320(((uVar29 >> 16) << 16) | LD16(puVar5 + 199), 0x4000);
                        if (((unsigned short)uVar29 <= uVar15) || (LD16(puVar5 + 0xc3) < (unsigned short)uVar29))
                            goto LAB_00411866;
                        iVar24 = 0x14;
                    }
                }
                FUN_0040c070(1, iVar24, uVar18, 0xffff);
            }
LAB_00411866:
            iVar24 = (int)LD32(R30(0x1b));
            if ((iVar24 != -1) && (!bVar12)) {
                if (iVar24 != -1) {
                    DAT_0070ac44[iVar24 * 8] = (unsigned short)(DAT_0070ac44[iVar24 * 8] + 0x3b);
                }
                uVar13 = FUN_00478470();
                if (uVar13 < 0x4000) {
                    uVar13 = FUN_00478470();
                    uVar15 = FUN_00478470();
                    uVar17 = FUN_00478470();
                    uVar29 = (unsigned int)(unsigned short)((uVar13 & 7) + (uVar15 & 1) + (uVar17 & 1));
                    if ((LD16(puVar5 + 0x1df) & DAT_00523b70[uVar29]) != 0) {
                        ST16(puVar5 + 0x1df, (DAT_00523b70[uVar29] ^ 0x3ff) & LD16(puVar5 + 0x1df));
                        _DAT_00732950 = (unsigned short)DAT_00523b88[uVar29];
                        _DAT_00732952 = 0x57;
                        if (((int)LD32(R30(0x1b)) == DAT_0071ecf4) &&
                            (DAT_0071b741[SEL] == '\0')) {
                            FUN_004a1d70(0x56, DAT_0071ecf4);
                        }
                        ST16(puVar5 + uVar29 * 2 + 0x29e, 0);
                        ST16(puVar5 + uVar29 * 2 + 0x2b4, *(unsigned short *)(DAT_00523b98 + uVar29 * 2));
                    }
                }
            }
        }
        if ((LD16(puVar5 + 0xc3) < LD16(puVar5 + 0xc5)) || (bVar12)) {
            if (((int)LD32(R30(0x1b)) == DAT_0071ecf4) && (!bVar12)) {
                uVar29 = FUN_004784d0();
                if ((uVar29 & 0x8000) == 0) {
                    uVar29 = 0x20;
                }
                else {
                    uVar29 = 0x1f;
                }
                FUN_0040a1a0(uVar29, uVar18, DAT_0071ecf4);
                local_1f = 0;
            }
            if (LD32(DAT_0071b730 + SEL) != uVar18) {
                uVar13 = 0x4000;
                iVar24 = 3;
                goto LAB_00411eaa;
            }
        }
        else {
            uVar29 = FUN_00478470();
            uVar13 = *(unsigned short *)(DAT_00523bb0 + (uVar29 & 0xf) * 2);
            if (((DAT_007092c1 == '\0') || (uVar13 != 1)) &&
                ((LD16(puVar5 + 0x1d9) & uVar13) != 0)) {
                iVar24 = (int)LD32(R30(0x1b));
                if (iVar24 != -1) {
                    FUN_0040a1a0(0x1e, uVar18, iVar24);
                    bVar12 = 0;
                    local_1f = 0;
                    cVar3 = (char)DAT_006d4988[3 + iVar30];
                    if (((cVar3 == '\x01') || (cVar3 == '\x02')) ||
                        ((cVar3 == '\x03' || ((cVar3 == '\x0e' || (cVar3 == '\x04')))))) {
                        bVar12 = 1;
                    }
                    if (bVar12) {
                        if (uVar13 < 9) {
                            if (((uVar13 == 8) || (uVar13 == 2)) || (uVar13 == 4)) {
                                uVar13 = 0xe;
                            }
                        }
                        else if (uVar13 < 0x101) {
                            if ((uVar13 == 0x100) || (uVar13 == 0x80)) {
                                uVar13 = 0x180;
                            }
                        }
                        else if (uVar13 < 0x401) {
                            if ((uVar13 == 0x400) || (uVar13 == 0x200)) {
LAB_00411deb:
                                uVar13 = 0xe00;
                            }
                        }
                        else if ((uVar13 == 0x800) || (uVar13 == 0x1000)) goto LAB_00411deb;
                    }
                }
                bVar9 = 1;
                ST16(puVar5 + 0x1db, LD16(puVar5 + 0x1db) & ~uVar13);
            }
            if ((puVar5[7] == 0) && (LD32(DAT_0071b730 + SEL) != uVar18)) {
                uVar13 = 0x6000;
                iVar24 = 5;
LAB_00411eaa:
                FUN_0040b730(DAT_0071ecf4, uVar18, iVar24, -1, uVar18, uVar13);
            }
        }
        if (((!bVar9) || ((int)LD32(R30(0x1b)) != DAT_0071ecf4)) ||
            (DAT_00732910 != 0)) goto LAB_00411efc;
        if (DAT_00525550 == 0) {
            FUN_004a6a80();
            FUN_004afb90();
            FUN_004a6f90();
            goto LAB_00411efc;
        }
    }
    cVar3 = (char)puVar5[7];
    uVar29 = ((((unsigned int)DAT_00525550 >> 8) & 0xffffff) << 8) | (unsigned char)cVar3;
    if ((cVar3 == '\0') || (cVar3 == '\x05')) {
        uVar13 = LD16(puVar5 + 199);
        uVar22 = (unsigned short)((unsigned int)DAT_00525550 >> 16);
        uVar29 = ((unsigned int)uVar22 << 16) | uVar13;
        if (uVar13 <= LD16(puVar5 + 0xc3)) {
            uVar29 = ((unsigned int)uVar22 << 16) | (unsigned short)(uVar13 - 1);
            ST16(puVar5 + 0xc3, uVar13 - 1);
        }
    }
LAB_004126c7:
    uVar21 = ((uVar29 >> 16) << 16) | DAT_00703abc;
    *(unsigned short *)(iVar19 + 0x16) = DAT_00703abc;
    DAT_00703abc = uVar10;
    return ((((uVar21 >> 8) & 0xffffff) << 8) | local_1f) & 0xffff00ff;
}

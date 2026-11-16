/* SPDX-License-Identifier: MIT */
/* z_bop 004422c0 */
/* Globals (types inferred from decompile/P-code) */
extern void *DAT_009ecc34;
extern unsigned int DAT_009d7680;
extern unsigned short DAT_009cc450;
extern unsigned int (__cdecl *DAT_009a8c30)(unsigned int, unsigned int);
extern unsigned short DAT_0052747c;
extern unsigned int DAT_009a8074;
extern int DAT_00a08374;
extern unsigned char DAT_00556540[];
extern unsigned char DAT_00556360[];
extern unsigned char DAT_00556720[];
extern int DAT_005233ec;
extern int DAT_00527e90;
extern unsigned char DAT_009e9689[];
extern unsigned char DAT_009e968a[];
extern unsigned char DAT_009e9699[];
extern unsigned char DAT_009e9a5b[];
extern unsigned char DAT_009e9a5f[];
extern unsigned char DAT_009e9a63[];
extern unsigned char DAT_009e9a67[];
extern unsigned char DAT_009e9a84[];
extern unsigned char DAT_009e9683[];
extern unsigned char DAT_009e9670[];
extern unsigned char DAT_00523408;
extern unsigned char DAT_005233c8;
extern unsigned short DAT_009a8062;
extern int DAT_009fd39c;
extern int DAT_009fd3a0;
extern int DAT_009fd398;
extern int DAT_00a07cdc;
extern int DAT_00a07cd8;
extern int DAT_00a07cd4;
extern int DAT_009a1fe0;
extern int DAT_009a1fe8;
extern int DAT_009d8a58;
extern int DAT_009d8a54;
extern int DAT_009d8b60;
extern int DAT_009a8e2c;
extern int DAT_009a8e30;
extern int DAT_009a8e34;
extern unsigned char DAT_009ec476;
extern int DAT_00a08104;
extern int DAT_00a07ce8;
extern unsigned int DAT_009a8e24;
extern char DAT_009a73f4;
extern unsigned char DAT_009d6940[];
extern short DAT_009cd274;
extern unsigned int DAT_009a8d9c;
extern unsigned char DAT_00521f1c[];
extern int DAT_005235f4;
extern int DAT_00a00860;

/* Prototypes */
void __cdecl FUN_00426f40(unsigned short, unsigned short, short, unsigned int);
void __cdecl FUN_0040b530(void);
void __cdecl FUN_00442bf0(unsigned int, int, int);
void __cdecl FUN_00485f00(int, int);
int __cdecl FUN_004aebe0(unsigned int, unsigned int);
int __cdecl FUN_004aed40(unsigned int, unsigned int);
int __cdecl FUN_004aec90(unsigned int, unsigned int);
void __cdecl FUN_004037b0(unsigned int, int, int, int);
int __cdecl FUN_00427420(int, int, int);
int __cdecl FUN_00427490(int, int, int);
int __cdecl FUN_00427500(int, int, int);
void __cdecl FUN_00427a60(short, short, short, int, int);
void __cdecl FUN_0044fdf0(unsigned int);
void __cdecl FUN_0041fc20(unsigned int);
void __cdecl FUN_00472400(unsigned int);
void __cdecl FUN_0041ff70(unsigned int);
void __cdecl FUN_00446550(unsigned short);
unsigned char __cdecl FUN_0044ff10(unsigned int, unsigned int);
void __cdecl FUN_00401000(unsigned int);
void __cdecl FUN_004865e0(void);
void __cdecl FUN_004012c0(int);
int __cdecl FUN_00427570(int, int);
int __cdecl FUN_00427590(int, int);
void __cdecl FUN_00442bc0(int, int, int, int, int);
void __cdecl FUN_00486200(void);
void __cdecl FUN_00427070(void);

#define RD16(dst, p) __builtin_memcpy(&(dst), (p), 2)
#define RD32(dst, p) __builtin_memcpy(&(dst), (p), 4)
#define PLOFF ((int)((unsigned int)(int)DAT_009ecc34 * 0x5bdU))
#define PL(sym) ((sym) + PLOFF)

void __cdecl FUN_004422c0(unsigned int param_1, unsigned int param_2, unsigned short param_3,
                          unsigned short param_4, short param_5)
{
    unsigned char uVar1;
    unsigned char bVar2;
    unsigned short uVar3;
    unsigned char *puVar4;
    unsigned int uVar5;
    unsigned int uVar6;
    int iVar7;
    unsigned short uVar8;
    unsigned int uVar9;
    void *pvVar10;
    short sVar12;
    unsigned int uVar13;
    int iVar14;
    int iVar15;
    unsigned char *puVar16;
    unsigned char *puVar17;
    unsigned short uVar18;
    int iStack_1c;
    int iStack_18;
    int iStack_14;
    unsigned short t16;
    unsigned short t16b;
    unsigned int t32;
    int iEnt;
    int pObj;
    short s14, s12, s10;
    unsigned int packed;

    uVar9 = DAT_009d7680;
    uVar8 = DAT_009cc450;
    RD32(puVar4, PL(DAT_009e9a5b));
    RD32(uVar5, PL(DAT_009e9a5f));
    RD32(uVar6, PL(DAT_009e9a63));
    DAT_009d7680 = 0;
    uVar13 = (*DAT_009a8c30)(param_1 & 0xffff, param_2 & 0xffff);
    FUN_00426f40(param_3, param_4, param_5, uVar13);
    if (param_5 != 0) {
        puVar17 = (unsigned char *)((unsigned int)DAT_0052747c + DAT_009a8074);
        if (DAT_00a08374 == 0) {
            puVar16 = DAT_00556540;
        } else if (DAT_00a08374 == 0x90) {
            puVar16 = DAT_00556360;
        } else {
            puVar16 = puVar4;
            if (DAT_00a08374 == 0x120) {
                puVar16 = DAT_00556720;
            }
        }
        if (DAT_005233ec == 0x13) {
            sVar12 = 200;
            do {
                uVar1 = *puVar16;
                puVar16 = puVar16 + 1;
                *puVar17 = uVar1;
                puVar17 = puVar17 + 1;
                sVar12 = sVar12 - 1;
            } while (sVar12 != 0);
        } else {
            sVar12 = 0x1e0;
            do {
                uVar1 = *puVar16;
                puVar16 = puVar16 + 1;
                *puVar17 = uVar1;
                puVar17 = puVar17 + 1;
                sVar12 = sVar12 - 1;
            } while (sVar12 != 0);
        }
    }
    if (DAT_00527e90 != 0) {
        FUN_0040b530();
    }
    RD16(t16, PL(DAT_009e968a));
    packed = ((((unsigned int)(int)DAT_009ecc34 * 0x16fU) >> 0x10) << 16) | (unsigned int)t16;
    FUN_00442bf0(packed, 1, (int)DAT_009ecc34);
    if (PL(DAT_009e9689)[0] == '\0') {
        DAT_009cc450 = DAT_009cc450 | 0x400;
    } else {
        DAT_009cc450 = DAT_009cc450 | 0x200;
    }
    DAT_00523408 = 0x30;
    FUN_00485f00((int)DAT_009ecc34, 1);
    DAT_009a8062 = 0;
    iVar14 = PLOFF;
    RD32(t32, PL(DAT_009e9a5b));
    DAT_009fd39c = (int)((unsigned int)DAT_00a07cdc - t32);
    RD32(t32, PL(DAT_009e9a5f));
    DAT_009fd3a0 = (int)((unsigned int)DAT_00a07cd8 - t32);
    RD32(t32, DAT_009e9a63 + iVar14);
    DAT_009fd398 = (int)((unsigned int)DAT_00a07cd4 - t32);
    RD16(t16, DAT_009e968a + iVar14);
    if ((DAT_009e9689[iVar14] != '\0') && ((int)(unsigned int)t16 < DAT_009a1fe0)) {
        RD16(t16b, DAT_009e9699 + iVar14);
        DAT_009d8a58 = FUN_004aebe0((unsigned int)*(unsigned char *)((unsigned int)t16 * 0x23 + 3 + (unsigned int)DAT_009a1fe8),
                                    (unsigned int)t16b);
        RD16(t16, PL(DAT_009e968a));
        RD16(t16b, PL(DAT_009e9699));
        DAT_009d8a54 = FUN_004aed40((unsigned int)*(unsigned char *)((unsigned int)t16 * 0x23 + 3 + (unsigned int)DAT_009a1fe8),
                                    (unsigned int)t16b);
        RD16(t16, PL(DAT_009e968a));
        RD16(t16b, PL(DAT_009e9699));
        iVar14 = FUN_004aec90((unsigned int)*(unsigned char *)((unsigned int)t16 * 0x23 + 3 + (unsigned int)DAT_009a1fe8),
                              (unsigned int)t16b);
        DAT_009d8b60 = (int)(0U - (unsigned int)iVar14);
        RD16(t16, PL(DAT_009e968a));
        FUN_004037b0((unsigned int)t16 * 0x23 + (unsigned int)DAT_009a1fe8, DAT_009d8a58, DAT_009d8a54, DAT_009d8b60);
        RD32(t32, PL(DAT_009e9a5b));
        iStack_14 = (int)(((unsigned int)DAT_00a07cdc - t32) + (unsigned int)DAT_009d8a58);
        RD32(t32, PL(DAT_009e9a5f));
        iStack_18 = (int)(((unsigned int)DAT_00a07cd8 - t32) + (unsigned int)DAT_009d8a54);
        RD32(t32, PL(DAT_009e9a63));
        iStack_1c = (int)(((unsigned int)DAT_009d8b60 - t32) + (unsigned int)DAT_00a07cd4);
    }
    DAT_009a8e2c = FUN_00427420(DAT_009fd39c, DAT_009fd3a0, DAT_009fd398);
    DAT_009a8e30 = FUN_00427490(DAT_009fd39c, DAT_009fd3a0, DAT_009fd398);
    DAT_009a8e34 = FUN_00427500(DAT_009fd39c, DAT_009fd3a0, DAT_009fd398);
    RD16(uVar18, PL(DAT_009e968a));
    iVar14 = (int)((unsigned int)uVar18 * 0x23 + (unsigned int)DAT_009a1fe8);
    RD32(pObj, (unsigned char *)iVar14 + 0x1f);
    if (pObj == 0) {
        if ((7 < *(unsigned char *)(iVar14 + 2)) && (*(unsigned char *)(iVar14 + 2) < 0xc)) {
            RD16(s14, (unsigned char *)iVar14 + 0x14);
            RD16(s12, (unsigned char *)iVar14 + 0x12);
            RD16(s10, (unsigned char *)iVar14 + 0x10);
            FUN_00427a60(s14, s12, s10, 0, 0);
            DAT_009ec476 = 1;
            FUN_00446550(uVar18);
        }
    } else {
        switch (*(unsigned char *)(iVar14 + 2)) {
        case 0:
        case 1:
        case 2:
        case 3:
        case 4:
        case 5:
            RD32(DAT_00a08104, (unsigned char *)pObj + 0xa9);
            RD16(s14, (unsigned char *)iVar14 + 0x14);
            RD16(s12, (unsigned char *)iVar14 + 0x12);
            RD16(s10, (unsigned char *)iVar14 + 0x10);
            FUN_00427a60(s14, s12, s10, 0, iVar14);
            RD16(t16, PL(DAT_009e968a));
            FUN_0044fdf0((unsigned int)t16);
            RD16(t16, PL(DAT_009e968a));
            packed = ((((unsigned int)(int)DAT_009ecc34 * 0x16fU) >> 0x10) << 16) | (unsigned int)t16;
            FUN_0041fc20(packed);
            RD16(t16, PL(DAT_009e968a));
            FUN_00472400((unsigned int)t16 * 0x23 + (unsigned int)DAT_009a1fe8);
            break;
        case 6:
        case 7:
            RD16(s14, (unsigned char *)iVar14 + 0x14);
            RD16(s12, (unsigned char *)iVar14 + 0x12);
            RD16(s10, (unsigned char *)iVar14 + 0x10);
            FUN_00427a60(s14, s12, s10, 0, iVar14);
            RD16(t16, PL(DAT_009e968a));
            FUN_0041ff70(t16);
        }
    }
    uVar13 = 0;
    if (0 < DAT_00a07ce8) {
        do {
            if ((uVar13 == DAT_009a8e24) &&
                (((DAT_009a73f4 == '\0') || (DAT_009d6940[1] != 0)) &&
                 (uVar13 = (unsigned int)(unsigned short)DAT_009cd274,
                  (unsigned short)DAT_009cd274 == DAT_00a07ce8))) {
                break;
            }
            uVar18 = (unsigned short)uVar13;
            iVar14 = PLOFF;
            RD16(t16, DAT_009e9a67 + iVar14);
            RD16(t16b, DAT_009e9a84 + iVar14);
            if (((t16 != uVar18) || (t16b != 0)) || (DAT_00a00860 != 0)) {
                iVar7 = (int)(uVar13 * 0x23);
                bVar2 = *(unsigned char *)((unsigned int)DAT_009a1fe8 + 3U + (unsigned int)iVar7);
                iVar15 = (int)((unsigned int)DAT_009a1fe8 + (unsigned int)iVar7);
                if (bVar2 != 0) {
                    RD32(DAT_009a8d9c, DAT_00521f1c + (unsigned int)bVar2 * 0x1a);
                    bVar2 = *(unsigned char *)(iVar15 + 2);
                    if (5 < bVar2) {
                        if (bVar2 < 8) {
                            if (DAT_009e9683[iVar14] == '\0') {
                                RD32(pObj, (unsigned char *)iVar15 + 0x1f);
                                RD16(uVar3, (unsigned char *)pObj + 0x85);
                                RD16(t16, DAT_009e968a + iVar14);
                                RD32(t32, DAT_009e9670 + iVar14);
                                if (((t16 == uVar3) || (t32 == (unsigned int)uVar3)) &&
                                    (FUN_0044ff10(uVar13, DAT_009a8d9c) != 0)) {
                                    iEnt = (int)((unsigned int)DAT_009a1fe8 + (unsigned int)iVar7);
                                    RD16(s14, (unsigned char *)iEnt + 0x14);
                                    RD16(s12, (unsigned char *)iEnt + 0x12);
                                    RD16(s10, (unsigned char *)((unsigned int)DAT_009a1fe8 + 0x10U + (unsigned int)iVar7));
                                    FUN_00427a60(s14, s12, s10, 0, iEnt);
                                    FUN_0041ff70(uVar18);
                                }
                            } else {
                                RD32(pObj, (unsigned char *)iVar15 + 0x1f);
                                RD16(uVar3, (unsigned char *)pObj + 0x85);
                                RD16(t16, DAT_009e968a + iVar14);
                                if ((uVar3 == t16) && (FUN_0044ff10(uVar13, DAT_009a8d9c) != 0)) {
                                    iEnt = (int)((unsigned int)DAT_009a1fe8 + (unsigned int)iVar7);
                                    RD16(s14, (unsigned char *)iEnt + 0x14);
                                    RD16(s12, (unsigned char *)iEnt + 0x12);
                                    RD16(s10, (unsigned char *)((unsigned int)DAT_009a1fe8 + 0x10U + (unsigned int)iVar7));
                                    FUN_00427a60(s14, s12, s10, 0, iEnt);
                                    FUN_0041ff70(uVar18);
                                }
                            }
                        } else if ((bVar2 == 0xd) && (FUN_0044ff10(uVar13, DAT_009a8d9c) != 0)) {
                            iEnt = (int)((unsigned int)DAT_009a1fe8 + (unsigned int)iVar7);
                            RD16(s14, (unsigned char *)iEnt + 0x14);
                            RD16(s12, (unsigned char *)iEnt + 0x12);
                            RD16(s10, (unsigned char *)((unsigned int)DAT_009a1fe8 + 0x10U + (unsigned int)iVar7));
                            FUN_00427a60(s14, s12, s10, 0, iEnt);
                            FUN_00401000(uVar13);
                        }
                    }
                }
            }
            uVar13 = (unsigned int)(unsigned short)(uVar18 + 1);
        } while ((int)(unsigned int)(unsigned short)(uVar18 + 1) < DAT_00a07ce8);
    }
    FUN_004865e0();
    if (DAT_009a8062 != 0) {
        DAT_005235f4 = 0;
        FUN_004012c0(0);
        DAT_005235f4 = 0;
    }
    if (PL(DAT_009e9689)[0] != '\0') {
        RD16(t16, PL(DAT_009e968a));
        uVar13 = (unsigned int)t16;
        if (((int)uVar13 < DAT_009a1fe0) &&
            (*(char *)(uVar13 * 0x23 + 2U + (unsigned int)DAT_009a1fe8) != '\0')) {
            DAT_009a8e2c = FUN_00427420(iStack_14, iStack_18, iStack_1c);
            DAT_009a8e30 = FUN_00427490(iStack_14, iStack_18, iStack_1c);
            DAT_009a8e34 = FUN_00427500(iStack_14, iStack_18, iStack_1c);
            iVar14 = FUN_00427570(DAT_009a8e2c, DAT_009a8e34);
            iVar15 = FUN_00427590(DAT_009a8e30, DAT_009a8e34);
            FUN_00442bc0((int)((unsigned int)iVar14 - 2U), (int)((unsigned int)iVar15 - 2U), 4, 4, 0xce);
        }
    }
    FUN_00486200();
    DAT_00523408 = DAT_005233c8;
    FUN_00427070();
    pvVar10 = DAT_009ecc34;
    DAT_009cc450 = uVar8;
    __builtin_memcpy(DAT_009e9a5b + (int)((unsigned int)(int)pvVar10 * 0x5bdU), &puVar4, 4);
    __builtin_memcpy(DAT_009e9a5f + (int)((unsigned int)(int)pvVar10 * 0x5bdU), &uVar5, 4);
    __builtin_memcpy(DAT_009e9a63 + (int)((unsigned int)(int)pvVar10 * 0x5bdU), &uVar6, 4);
    DAT_009d7680 = uVar9;
    return;
}

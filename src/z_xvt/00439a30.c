/* SPDX-License-Identifier: MIT */
/* z_xvt 00439a30 */
/* Globals: types inferred from the decompile; per-player block stride is 0x5bd (odd), so
   every multi-byte access into it is done through memcpy (packed/unaligned). */
extern char DAT_007092c0;
extern int DAT_00525550;
extern unsigned char DAT_0071bb27[];
extern unsigned char DAT_0071b743[];
extern unsigned char DAT_0071bb4a[];
extern unsigned char DAT_0071bb44[];
extern unsigned char DAT_0071bb40[];
extern unsigned char DAT_0071b730[];
extern unsigned char DAT_0071bb33[];
extern unsigned char DAT_0071bb35[];
extern unsigned char DAT_0071bb29[];
extern unsigned char DAT_0071b741[];
extern unsigned char DAT_0071b744[];
extern unsigned char DAT_0071b73e[];
extern unsigned char DAT_0071b781[];
extern unsigned char DAT_0071b783[];
extern unsigned char DAT_0071bb1a[];
extern unsigned char DAT_0071b740[];
extern unsigned char DAT_0071e528[];
extern int DAT_006d4988;
extern uint DAT_0071ecf4;
extern int DAT_007092b0;
extern int DAT_00525518;
extern uint DAT_00525540;
extern void (__cdecl *DAT_006db5a0)(void);
extern short _DAT_00732950;
extern unsigned char DAT_00525538;
extern int _DAT_0071b010;
extern int DAT_00525ad0[];
extern unsigned char DAT_00732e4c;
extern unsigned char DAT_00525ae8[];
extern unsigned short DAT_0052553c;
extern unsigned short DAT_00525ae0[];
extern unsigned char DAT_006d9d94;
extern unsigned char DAT_00525af0[];
extern unsigned char DAT_0071e536;
extern uint DAT_00525544;
extern char DAT_00732a1e;
extern int DAT_00732e48;
extern int DAT_006db5a8;
extern int DAT_0052ae54;
extern short DAT_006db592;
extern int DAT_007327e0;
extern char DAT_0070accc;
extern unsigned char DAT_00525580;
extern char DAT_007329cc;
extern unsigned short DAT_0071e534;
extern unsigned short DAT_0071e6be;
extern char DAT_007092c1;

void FUN_0040cbf0(int a, uint b);
uint FUN_00405cf0(uint a);
void FUN_004054e0(void);
void FUN_004a1d70(uint a, uint b);
void FUN_0040a1a0(int a, int b, uint c);
int FUN_0048f530(void);
void FUN_004a6a80(void);
/* FINDING: Ghidra shows two register args (extraout_ECX/EDX); modelled as __fastcall. */
void __fastcall FUN_004a42b0(uint a, uint b);
void FUN_004a6f90(void);
void FUN_004a7040(void);
void FUN_00491150(void);
void FUN_004651e0(void);
void FUN_00465210(void);
void FUN_0041e890(uint a);
int FUN_00423200(uint a, int b, int c, int d);
void FUN_004441e0(uint a);
int *FUN_00432cc0(uint a);
void FUN_004a3490(int a, int *b);
void FUN_004a41d0(int a);
void FUN_00447010(uint a);
uint FUN_00436120(uint a);
uint FUN_00441930(uint a);
void FUN_00442bf0(uint a, uint b);
void FUN_0043a080(uint a, uint b, uint c, uint d);
void FUN_00440d20(uint a);
void FUN_0043fe30(uint a);

void __cdecl FUN_00439a30(uint param_1)
{
    unsigned short uVar2;
    unsigned short tmp16;
    unsigned short tmp16b;
    int iVar3;
    int *piVar4;
    unsigned char *pcVar5;
    uint uVar6;
    uint extraout_ECX = 0;
    uint extraout_EDX = 0;
    uint unaff_EBX = 0;
    uint unaff_ESI = 0;
    uint unaff_EDI = 0;
    int tmp32;

    if (DAT_007092c0 == '\x01') {
        return;
    }
    if (DAT_00525550 == 0) {
        iVar3 = (int)(param_1 * 0x5bd);
        __builtin_memcpy(&uVar2, DAT_0071bb27 + iVar3, 2);
        if ((uVar2 != 0xffff) && (*(char *)((uint)uVar2 * 0x23 + 3 + DAT_006d4988) == '\0')) {
            if (DAT_0071b743[iVar3] == '\0') {
                tmp16 = 0;
                __builtin_memcpy(DAT_0071bb4a + iVar3, &tmp16, 2);
                __builtin_memcpy(DAT_0071bb44 + iVar3, &tmp16, 2);
                __builtin_memcpy(DAT_0071bb40 + iVar3, &tmp16, 2);
                __builtin_memcpy(&tmp32, DAT_0071b730 + iVar3, 4);
                tmp16 = (unsigned short)tmp32;
                __builtin_memcpy(DAT_0071bb27 + iVar3, &tmp16, 2);
                FUN_0040cbf0(0, param_1);
                tmp16 = 0;
                __builtin_memcpy(DAT_0071bb33 + iVar3, &tmp16, 2);
                __builtin_memcpy(DAT_0071bb35 + iVar3, &tmp16, 2);
            }
            else {
                tmp16 = 0xffff;
                __builtin_memcpy(DAT_0071bb27 + iVar3, &tmp16, 2);
            }
        }
        if (DAT_0071b743[iVar3] != '\0') {
            __builtin_memcpy(&uVar2, DAT_0071bb29 + iVar3, 2);
            if ((uVar2 != 0xffff) &&
                (*(char *)((uint)uVar2 * 0x23 + 3 + DAT_006d4988) == '\0')) {
                tmp16 = 0xffff;
                __builtin_memcpy(DAT_0071bb29 + iVar3, &tmp16, 2);
            }
        }
    }
    FUN_00405cf0(param_1);
    FUN_004054e0();
    if ((param_1 == DAT_0071ecf4) && (DAT_00525550 == 0)) {
        switch (DAT_007092b0) {
        case 0x7d:
            if (DAT_00525518 == 1) {
                DAT_00525540 = DAT_00525540 + 0x40;
                if (DAT_00525540 == 0x300) {
                    DAT_00525540 = 0x100;
                }
                (*DAT_006db5a0)();
                _DAT_00732950 = (short)((DAT_00525540 - 0x100U) >> 6) + 1;
                FUN_004a1d70(0x115, param_1);
                FUN_0040a1a0(0x2b, 0xffffffff, param_1);
            }
            break;
        case 0x7f:
            DAT_00525538 = DAT_00525538 + 1;
            if (3 < DAT_00525538) {
                DAT_00525538 = 0;
            }
            uVar6 = (uint)DAT_00525538;
            _DAT_0071b010 = DAT_00525ad0[uVar6];
            DAT_00732e4c = DAT_00525ae8[uVar6];
            DAT_0052553c = DAT_00525ae0[uVar6];
            DAT_006d9d94 = DAT_00525af0[uVar6 * 2];
            DAT_0071e536 = 1;
            FUN_004a1d70(uVar6 + 0x65, param_1);
            FUN_0040a1a0(0x2b, 0xffffffff, param_1);
            break;
        case 0x84:
            DAT_00525544 = (uint)(DAT_00525544 == 0);
            break;
        case 0x88:
            if (DAT_00732a1e == '\0') {
                DAT_00732a1e = '\x01';
            }
            else {
                DAT_00732a1e = '\0';
            }
            break;
        case 0x8b:
            if (DAT_00732e48 == 1) {
                iVar3 = FUN_0048f530();
                DAT_006db5a8 = (int)((uint)DAT_006db5a8 + (uint)iVar3);
                FUN_0040a1a0(0x2b, 0xffffffff, param_1);
                FUN_004a1d70(1, param_1);
                DAT_0052ae54 = 0;
                FUN_004a6a80();
                FUN_004a42b0(extraout_ECX, extraout_EDX);
                FUN_004a6f90();
                FUN_004a7040();
                DAT_0052ae54 = 1;
                FUN_00491150();
                do {
                    uVar6 = FUN_00405cf0(0xfffffffe);
                } while ((short)uVar6 == 0);
                FUN_0048f530();
                FUN_004a1d70(2, param_1);
                DAT_006db592 = 0;
                DAT_007327e0 = 0;
                FUN_004651e0();
            }
            break;
        case 0x8e:
            if (DAT_0070accc == '\0') {
                DAT_0070accc = '\x01';
            }
            else {
                DAT_0070accc = '\0';
            }
            break;
        case 0x91:
            FUN_0040a1a0(0x2b, 0xffffffff, param_1);
            FUN_004a1d70(0, param_1);
            break;
        case 0xe4:
            FUN_0040a1a0(0x2b, 0xffffffff, param_1);
            FUN_00465210();
        }
    }
    if ((1 < DAT_00525580) && (DAT_007329cc != '\0')) {
        return;
    }
    iVar3 = (int)(param_1 * 0x5bd);
    if (DAT_0071b741[iVar3] != '\0') {
        if (DAT_00525550 != 0) {
            return;
        }
        __builtin_memcpy(&tmp32, DAT_0071b730 + iVar3, 4);
        if (tmp32 == -1) {
            return;
        }
        if (*(char *)((uint)tmp32 * 0x23 + 3 + DAT_006d4988) != '\0') {
            return;
        }
        __builtin_memcpy(&tmp16, DAT_0071b73e + iVar3, 2);
        FUN_0041e890(tmp16);
        iVar3 = FUN_00423200(param_1, 0xffffffff, 0, 0);
        if (iVar3 != 0) {
            FUN_004441e0(param_1);
            if (param_1 == DAT_0071ecf4) {
                return;
            }
            piVar4 = FUN_00432cc0(param_1);
            FUN_004a3490(0, piVar4);
            FUN_004a1d70(0x173, DAT_0071ecf4);
            iVar3 = 0;
            pcVar5 = DAT_0071b740;
            do {
                if (*pcVar5 == '\x01') {
                    iVar3 = iVar3 + 1;
                }
                pcVar5 = pcVar5 + 0x5bd;
            } while (pcVar5 < DAT_0071e528);
            if (DAT_0071b740[(int)(DAT_0071ecf4 * 0x5bd)] != '\x01') {
                return;
            }
            if (iVar3 != 1) {
                _DAT_00732950 = (short)iVar3;
                FUN_004a1d70(0x174, DAT_0071ecf4);
                return;
            }
            FUN_004a1d70(0x175, DAT_0071ecf4);
            return;
        }
        if (param_1 != DAT_0071ecf4) {
            return;
        }
        FUN_004a41d0(0x118);
        return;
    }
    if (DAT_0071b744[iVar3] != '\0') goto LAB_00439fbd;
    __builtin_memcpy(&tmp16, DAT_0071bb40 + iVar3, 2);
    if (((((unsigned char)DAT_0071e534 & 0xd) == 1) && (tmp16 == 0)) &&
        (DAT_0071b743[iVar3] == '\0')) {
        FUN_00447010(param_1);
    }
    __builtin_memcpy(&uVar2, DAT_0071b781 + iVar3, 2);
    uVar2 = uVar2 & 0xe;
    if (((unsigned char)DAT_0071e534 & 0xe) == 2) {
        __builtin_memcpy(&tmp16, DAT_0071b783 + iVar3, 2);
        if (uVar2 == 2) {
            tmp16 = (unsigned short)(tmp16 + DAT_0071e6be);
        }
        else {
            tmp16 = DAT_0071e6be;
        }
        __builtin_memcpy(DAT_0071b783 + iVar3, &tmp16, 2);
        __builtin_memcpy(DAT_0071b781 + iVar3, &DAT_0071e534, 2);
        if (tmp16 < 0x3b) {
            DAT_0071e534 = DAT_0071e534 & 0xfffd;
        }
        goto LAB_00439fbd;
    }
    __builtin_memcpy(&tmp16, DAT_0071b783 + iVar3, 2);
    if ((uVar2 == 2) && (tmp16 < 0x3b)) {
        if (DAT_0071b743[iVar3] == '\0') {
            __builtin_memcpy(&tmp16b, DAT_0071bb40 + iVar3, 2);
            if ((DAT_007092c1 != '\0') || (tmp16b != 0))
                goto LAB_00439fa7;
            uVar6 = FUN_00441930(param_1);
            uVar2 = (unsigned short)uVar6;
        }
        else {
            uVar6 = FUN_00436120(param_1);
            uVar2 = (unsigned short)uVar6;
        }
        if (uVar2 != 0xffff) {
            FUN_00442bf0(uVar2, param_1);
        }
    }
LAB_00439fa7:
    __builtin_memcpy(DAT_0071b781 + iVar3, &DAT_0071e534, 2);
    tmp16 = 0;
    __builtin_memcpy(DAT_0071b783 + iVar3, &tmp16, 2);
LAB_00439fbd:
    if (DAT_0071bb1a[iVar3] == '\0') {
        FUN_0043a080(param_1, unaff_EDI, unaff_ESI, unaff_EBX);
    }
    else {
        FUN_00440d20(param_1);
    }
    if (DAT_0071b740[iVar3] != '\0') {
        FUN_0043fe30(param_1);
    }
    return;
}

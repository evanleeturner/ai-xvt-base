/* SPDX-License-Identifier: MIT */
/* z_xvt 004d4250 */
/* Prototypes for every callee */
int __cdecl FUN_004d4600(HINSTANCE param_1);
void __cdecl FUN_004d3ca0(int param_1);
int __cdecl FUN_004df760(int param_1, int param_2, int param_3);
void __cdecl FUN_004ca990(void);
int __cdecl FUN_004d4d00(void);
void __cdecl FUN_004d4d90(void);
/* Ghidra: (int, int *). The second argument is the address of an unaligned
   global (0x77dc56), so it is passed as a byte pointer rather than an int *. */
int __cdecl FUN_004db790(int param_1, unsigned char *param_2);
void __cdecl FUN_004dd860(void);
void __cdecl FUN_004d00f0(int param_1, int param_2, int param_3);
void __cdecl FUN_004cfd60(void);
void __cdecl FUN_004cfff0(void);
void __cdecl FUN_004cfbc0(int param_1, int param_2, int param_3);
void __cdecl FUN_004d4dc0(void);
void __cdecl FUN_004df050(int param_1);

/* Globals. Those at addresses that are not a multiple of their width are
   byte arrays accessed through __builtin_memcpy. */
extern unsigned char DAT_0077dc4e[4];       /* int (__cdecl *)(void) */
extern int DAT_00759ad4;
extern int DAT_00759ad8;
extern int DAT_00759adc;
extern int DAT_00759ae0;
extern int DAT_00759ae4;
extern int DAT_00759ae8;
extern unsigned char DAT_00759673[4];       /* int */
extern unsigned char DAT_00780736[4];       /* int */
extern unsigned char DAT_0075dc46[4];       /* int */
extern unsigned char DAT_0075dc4e[0x20000]; /* 0x8000 dwords */
extern unsigned char DAT_0077e0a2[];        /* table, stride 0x440; +0 frame fn, +4 handler fn */
extern unsigned char DAT_0077dc66[4];       /* int index */
extern unsigned char DAT_0077dc6a[4];       /* int */
extern unsigned char DAT_007806f2[4];       /* int */
extern unsigned char DAT_0077dc52[4];       /* int */
extern unsigned char DAT_0077dc56[];        /* passed by address */
extern unsigned char DAT_00758d59;
extern unsigned char DAT_00758fd0[];
extern int DAT_00758830;
extern unsigned char DAT_00758f2e[32];      /* 8 dwords */
extern unsigned char DAT_00758f4e[32];      /* 8 dwords */
extern int DAT_00759b08;
extern int DAT_00759b0c;
extern int DAT_00759b14;
extern int DAT_00759b34;
extern unsigned char DAT_0075965e[4];       /* HWND */
extern unsigned char DAT_00758ee0;
extern unsigned char DAT_00758ee1;
extern int DAT_00759b2c;
extern DWORD DAT_00759b28;
extern DWORD DAT_00759b18;
extern int DAT_00759b1c;
extern unsigned char DAT_007806ee[4];       /* int */

unsigned int __cdecl FUN_004d4250(HINSTANCE param_1)
{
    int (__cdecl *pfnInit)(void);
    void (__cdecl *pcVar1)(int);
    int (__cdecl *pfnFrame)(int);
    int bVar2;
    int iVar3;
    DWORD DVar4;
    BOOL BVar5;
    int iVar6;
    DWORD local_24;
    DWORD local_20;
    /* tagMSG is incomplete in xvt.h: 28-byte buffer (hwnd, message, wParam,
       lParam, time, pt.x, pt.y); wParam is the dword at index 2 (offset 8). */
    unsigned int tStack_1c[7];
    int tmp;
    int tmpIdx;
    int tmpParam;
    int tmpA;
    int tmpB;
    HWND hwndTmp;

    iVar6 = 0;
    bVar2 = 0;
    iVar3 = FUN_004d4600(param_1);
    if (iVar3 == 0) {
        return 0;
    }
    __builtin_memcpy(&pfnInit, DAT_0077dc4e, 4);
    if (pfnInit != 0) {
        __builtin_memcpy(&pfnInit, DAT_0077dc4e, 4);
        iVar3 = (*pfnInit)();
        if (iVar3 != 0) {
            FUN_004d3ca0(1);
            return 0;
        }
    }
    local_24 = GetTickCount();
    DAT_00759ad4 = 0xffff;
    DAT_00759ad8 = FUN_004df760(0x40, 0xc4, 0x40);
    DAT_00759adc = FUN_004df760(0xff, 0, 0);
    DAT_00759ae0 = FUN_004df760(0xff, 0xff, 0);
    DAT_00759ae4 = FUN_004df760(0, 0, 0xff);
    DAT_00759ae8 = FUN_004df760(0x80, 0x80, 0xff);
    local_20 = local_24;
    while (1) {
        while (1) {
            for (;;) {
                __builtin_memcpy(&tmp, DAT_00759673, 4);
                if (tmp != 0) {
                    break;
                }
                BVar5 = PeekMessageA((LPMSG)tStack_1c, (HWND)0, 0, 0, 0);
                if (BVar5 != 0) {
                    BVar5 = GetMessageA((LPMSG)tStack_1c, (HWND)0, 0, 0);
                    if (BVar5 == 0) {
                        return tStack_1c[2];
                    }
                    TranslateMessage((LPMSG)tStack_1c);
                    DispatchMessageA((LPMSG)tStack_1c);
                }
            }
            if ((bVar2) && (iVar6 == 0)) {
                bVar2 = 0;
                tmp = 0;
                __builtin_memcpy(DAT_00780736, &tmp, 4);
                __builtin_memcpy(&tmp, DAT_0075dc46, 4);
                if (tmp != 0) {
                    __builtin_memset(DAT_0075dc4e, 0, 0x20000);
                }
                FUN_004ca990();
                __builtin_memcpy(&tmpIdx, DAT_0077dc66, 4);
                __builtin_memcpy(&tmp, DAT_0077e0a2 + (unsigned int)tmpIdx * 0x440U, 4);
                if (tmp != 0) {
                    GetKeyboardState(DAT_00758fd0);
                    DAT_00758830 = FUN_004d4d00();
                    __builtin_memcpy(&tmpIdx, DAT_0077dc66, 4);
                    __builtin_memcpy(&pcVar1, DAT_0077e0a2 + 4 + (unsigned int)tmpIdx * 0x440U, 4);
                    __builtin_memcpy(&tmpIdx, DAT_0077dc66, 4);
                    __builtin_memcpy(&pfnFrame, DAT_0077e0a2 + (unsigned int)tmpIdx * 0x440U, 4);
                    __builtin_memcpy(&tmpParam, DAT_007806f2, 4);
                    iVar6 = (*pfnFrame)(tmpParam);
                    __builtin_memcpy(&tmpA, DAT_0077dc6a, 4);
                    if ((tmpA == 1) || (iVar6 == 1)) {
                        tmpA = 0;
                        __builtin_memcpy(DAT_0077dc6a, &tmpA, 4);
                        if (pcVar1 != 0) {
                            __builtin_memcpy(&tmpParam, DAT_007806f2, 4);
                            (*pcVar1)(tmpParam);
                        }
                    }
                    FUN_004d4d90();
                    __builtin_memcpy(&tmp, DAT_0077dc52, 4);
                    if (tmp != 0) {
                        __builtin_memcpy(&tmp, DAT_0077dc52, 4);
                        FUN_004db790(tmp, DAT_0077dc56);
                        tmp = 0;
                        __builtin_memcpy(DAT_0077dc52, &tmp, 4);
                    }
                    if (DAT_00758d59 == '\x01') {
                        FUN_004dd860();
                    }
                    __builtin_memset(DAT_00758f2e, 0, 32);
                    __builtin_memset(DAT_00758f4e, 0, 32);
                    __builtin_memcpy(&tmpParam, DAT_007806f2, 4);
                    tmpParam = (int)((unsigned int)tmpParam + 1U);
                    __builtin_memcpy(DAT_007806f2, &tmpParam, 4);
                    if (iVar6 == 1) {
                        if (((DAT_00759b08 != 0) && (DAT_00759b0c != 0)) && (DAT_00759b14 == 0)) {
                            FUN_004d00f0(DAT_00759b34, 0x200, 2000);
                        }
                        FUN_004cfd60();
                        __builtin_memcpy(&hwndTmp, DAT_0075965e, 4);
                        PostMessageA(hwndTmp, 0x10, 0, 0);
                    }
                    __builtin_memcpy(&tmp, DAT_0075dc46, 4);
                    if (tmp != 0) {
                        tmp = (int)((unsigned int)tmp - 1U);
                        __builtin_memcpy(DAT_0075dc46, &tmp, 4);
                    }
                    DAT_00758ee0 = 0;
                    DAT_00758ee1 = 0;
                    if (DAT_00759b2c == 2) {
                        DVar4 = GetTickCount();
                        if (DAT_00759b28 < DVar4) {
                            FUN_004cfff0();
                        }
                    }
                    if (DAT_00759b0c != 0) {
                        DVar4 = GetTickCount();
                        if (DAT_00759b18 < DVar4) {
                            if (DAT_00759b1c == 0) {
                                DAT_00759b14 = 1;
                            } else {
                                FUN_004cfbc0(DAT_00759b0c, 0, 0);
                            }
                        }
                    }
                }
            }
            BVar5 = PeekMessageA((LPMSG)tStack_1c, (HWND)0, 0, 0, 0);
            if (BVar5 != 0) {
                break;
            }
            if (iVar6 == 0) {
                DVar4 = GetTickCount();
                __builtin_memcpy(&tmpB, DAT_007806ee, 4);
                if (tmpB <= (int)(DVar4 - local_24)) {
                    bVar2 = 1;
                    FUN_004d4dc0();
                    local_24 = DVar4;
                }
                if (99 < (int)(DVar4 - local_20)) {
                    FUN_004df050(0);
                    FUN_004df050(1);
                    local_20 = DVar4;
                }
            }
        }
        BVar5 = GetMessageA((LPMSG)tStack_1c, (HWND)0, 0, 0);
        if (BVar5 == 0) {
            break;
        }
        TranslateMessage((LPMSG)tStack_1c);
        DispatchMessageA((LPMSG)tStack_1c);
    }
    return tStack_1c[2];
}

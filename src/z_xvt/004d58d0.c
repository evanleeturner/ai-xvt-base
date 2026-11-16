/* SPDX-License-Identifier: MIT */
/* z_xvt 004d58d0 */
typedef int (*FUN_004d58d0_cb1)(int);   /* convention unverified: cdecl assumed */
typedef void (*FUN_004d58d0_cb2)(int);  /* convention unverified: cdecl assumed */

extern unsigned char DAT_00759673[4];   /* int, unaligned (0x759673) */
extern unsigned char DAT_007806ee[4];   /* int, unaligned */
extern unsigned char DAT_0075dc46[4];   /* int, unaligned */
extern unsigned char DAT_0075dc4e[];    /* 0x8000 dwords, unaligned */
extern unsigned char DAT_0077dc66[4];   /* int, unaligned */
extern unsigned char DAT_0077dc6a[4];   /* int, unaligned */
extern unsigned char DAT_007806f2[4];   /* int, unaligned */
extern unsigned char DAT_0077e0a2[];    /* table base, stride 0x440, unaligned */
extern unsigned char DAT_0077e0a6[];    /* table base, stride 0x440, unaligned */
extern unsigned char DAT_00758fd0[256];
extern int DAT_00758830;
extern char DAT_00758d59;
extern unsigned char DAT_00758f2e[32];  /* 8 dwords, unaligned */
extern unsigned char DAT_00758f4e[32];  /* 8 dwords, unaligned */
extern unsigned char DAT_00758ee0;
extern unsigned char DAT_00758ee1;
extern int DAT_00759b0c;
extern unsigned int DAT_00759b18;
extern int DAT_00759b1c;
extern int DAT_00759b14;

/* Callee conventions/signatures not evidenced; cdecl assumed. */
void FUN_004d4dc0(void);
void FUN_004df050(int param_1);
void FUN_004ca990(void);
int FUN_004d4d00(void);
void FUN_004d4d90(void);
void FUN_004dd860(void);
void FUN_004cfbc0(int param_1, int param_2, int param_3);
/* 0x004d5280: local thunk (jmp to the GetTickCount import), Ghidra name "GetTickCount" */
DWORD __stdcall FUN_004d5280(void);

unsigned int __stdcall FUN_004d58d0(void)
{
    FUN_004d58d0_cb2 pcVar1;
    FUN_004d58d0_cb1 pcVar8;
    int bVar2;
    BOOL BVar3;
    DWORD DVar4;
    int iVar5;
    int iVar6;
    DWORD local_24;
    DWORD local_20;
    unsigned int local_1c[7];   /* MSG storage, 0x1c bytes (MSG is incomplete in xvt.h) */
    int tmp;
    int zero;
    unsigned int utmp;
    unsigned int idx;

    bVar2 = 0;
    local_24 = GetTickCount();
    local_20 = local_24;
    while (1) {
        for (;;) {
            __builtin_memcpy(&tmp, DAT_00759673, 4);
            if (tmp != 0) {
                break;
            }
            BVar3 = PeekMessageA((LPMSG)local_1c, (HWND)0, 0, 0, 0);
            if (BVar3 != 0) {
                BVar3 = GetMessageA((LPMSG)local_1c, (HWND)0, 0, 0);
                if (BVar3 == 0) {
                    return 2;
                }
                TranslateMessage((LPMSG)local_1c);
                DispatchMessageA((LPMSG)local_1c);
            }
        }
        if (bVar2) {
            break;
        }
        BVar3 = PeekMessageA((LPMSG)local_1c, (HWND)0, 0, 0, 0);
        if (BVar3 == 0) {
            DVar4 = GetTickCount();
            __builtin_memcpy(&tmp, DAT_007806ee, 4);
            if (tmp <= (int)(DVar4 - local_24)) {
                bVar2 = 1;
                FUN_004d4dc0();
                local_24 = DVar4;
            }
            if (99 < (int)(DVar4 - local_20)) {
                FUN_004df050(0);
                FUN_004df050(1);
                local_20 = DVar4;
            }
        } else {
            BVar3 = GetMessageA((LPMSG)local_1c, (HWND)0, 0, 0);
            if (BVar3 == 0) {
                return 2;
            }
            TranslateMessage((LPMSG)local_1c);
            DispatchMessageA((LPMSG)local_1c);
        }
    }

    zero = 0;
    __builtin_memcpy(&tmp, DAT_0075dc46, 4);
    if (tmp != 0) {
        for (iVar5 = 0; iVar5 != 0x20000; iVar5++) {
            DAT_0075dc4e[iVar5] = 0;
        }
    }
    FUN_004ca990();

    __builtin_memcpy(&tmp, DAT_0077dc66, 4);
    idx = (unsigned int)tmp * 0x440u;
    __builtin_memcpy(&pcVar8, DAT_0077e0a2 + idx, 4);
    if (pcVar8 != (FUN_004d58d0_cb1)0) {
        GetKeyboardState(DAT_00758fd0);
        DAT_00758830 = FUN_004d4d00();
        __builtin_memcpy(&tmp, DAT_0077dc66, 4);
        idx = (unsigned int)tmp * 0x440u;
        __builtin_memcpy(&pcVar1, DAT_0077e0a6 + idx, 4);
        __builtin_memcpy(&pcVar8, DAT_0077e0a2 + idx, 4);
        __builtin_memcpy(&tmp, DAT_007806f2, 4);
        iVar5 = pcVar8(tmp);
        __builtin_memcpy(&tmp, DAT_0077dc6a, 4);
        if (tmp == 1 || iVar5 == 1) {
            __builtin_memcpy(DAT_0077dc6a, &zero, 4);
            if (pcVar1 != (FUN_004d58d0_cb2)0) {
                __builtin_memcpy(&tmp, DAT_007806f2, 4);
                pcVar1(tmp);
            }
        }
        FUN_004d4d90();
        if (DAT_00758d59 == '\x01') {
            FUN_004dd860();
        }
        for (iVar6 = 0; iVar6 != 32; iVar6++) {
            DAT_00758f2e[iVar6] = 0;
        }
        for (iVar6 = 0; iVar6 != 32; iVar6++) {
            DAT_00758f4e[iVar6] = 0;
        }
        __builtin_memcpy(&utmp, DAT_007806f2, 4);
        utmp = utmp + 1u;
        __builtin_memcpy(DAT_007806f2, &utmp, 4);
        if (iVar5 == 1) {
            return 1;
        }
        __builtin_memcpy(&utmp, DAT_0075dc46, 4);
        if (utmp != 0) {
            utmp = utmp - 1u;
            __builtin_memcpy(DAT_0075dc46, &utmp, 4);
        }
        DAT_00758ee0 = 0;
        DAT_00758ee1 = 0;
        if (DAT_00759b0c != 0) {
            DVar4 = FUN_004d5280();
            if (DAT_00759b18 < DVar4) {
                if (DAT_00759b1c != 0) {
                    FUN_004cfbc0(DAT_00759b0c, 0, 0);
                    return 0;
                }
                DAT_00759b14 = 1;
            }
        }
    }
    return 0;
}

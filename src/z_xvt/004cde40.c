/* SPDX-License-Identifier: MIT */
/* z_xvt 004cde40 */
/* Prototypes of everything this function calls. */
extern int __stdcall Ordinal_4(int, void *, int, int, int);
extern int __cdecl FUN_004ce2a0(int *param_1, undefined4 *param_2, char *param_3, char *param_4);

/* COM vtable slots used (IDirectPlayLobby): Release (+8), Connect (+0xc), SetConnectionSettings (+0x30) */
typedef unsigned long (__stdcall *PFN_FUN_004cde40_REL)(int *);
typedef int (__stdcall *PFN_FUN_004cde40_CONN)(int *, int, void *, int);
typedef int (__stdcall *PFN_FUN_004cde40_SET)(int *, int, int, undefined4 *);

extern char s_modem_game_005324ec[];

/* 0x7806fa / 0x7806fe are not 4-aligned: packed globals, accessed via memcpy. */
extern unsigned char DAT_007806fa[4];
extern unsigned char DAT_007806fe[4];

extern undefined4 DAT_005202a0, DAT_005202a4, DAT_005202a8, DAT_005202ac;
extern undefined4 DAT_005202b0, DAT_005202b4, DAT_005202b8, DAT_005202bc;
extern undefined4 DAT_005202c0, DAT_005202c4, DAT_005202c8, DAT_005202cc;
extern undefined4 DAT_005202d0, DAT_005202d4, DAT_005202d8, DAT_005202dc;
extern undefined4 DAT_00521b90, DAT_00521b94, DAT_00521b98, DAT_00521b9c;
extern undefined4 DAT_00521ba0, DAT_00521ba4, DAT_00521ba8, DAT_00521bac;
extern undefined4 DAT_00521bb0, DAT_00521bb4, DAT_00521bb8, DAT_00521bbc;
extern undefined4 DAT_00520290, DAT_00520294, DAT_00520298, DAT_0052029c;

#define FUN_004cde40_GETDPL(v) __builtin_memcpy(&(v), DAT_007806fe, 4)
#define FUN_004cde40_CLRDPL() do { int *z_ = (int *)0; __builtin_memcpy(DAT_007806fe, &z_, 4); } while (0)

/*
 * NOTE: the stack-argument reads at entry+0x24 / +0x28 (P-code: stack 0x24 compared
 * against 4, indexed jump table at 0x4ce290) mean the real function consumes
 * parameters beyond the three Ghidra shows. They are declared here as
 * param_4..param_8 (unused) + in_stack_00000024 / in_stack_00000028 so the
 * offsets match. The single caller (0x004c96a0) decompiles with a mangled frame,
 * so these two extra values are UNRESOLVED as to their true source.
 */
unsigned char __cdecl FUN_004cde40(undefined4 param_1, int param_2, undefined4 param_3,
                                   undefined4 param_4, undefined4 param_5, undefined4 param_6,
                                   undefined4 param_7, undefined4 param_8,
                                   undefined4 in_stack_00000024, char *in_stack_00000028)
{
    int iVar1;
    int *lpDPL;
    undefined4 uVar4;
    undefined4 *local_90;
    undefined4 local_104[5];       /* [0] GUID/size, [1..4] reused as DPNAME (-0x100) */
    undefined4 local_dc[10];       /* DPLCONNECTION-sized block (0x28) */
    char acStack_b4[16];
    undefined4 auStack_a4[20];     /* DPSESSIONDESC2-sized block (0x50) */
    undefined4 buf_w[16];
    char *buf = (char *)buf_w;
    int i;

    /* "modem game" + zero fill to 64 bytes */
    __builtin_memcpy(buf, s_modem_game_005324ec, 11);
    for (i = 11; i < 64; i++) {
        buf[i] = 0;
    }

    switch (in_stack_00000024) {
    case 0:
        local_104[0] = DAT_005202a0;
        local_104[1] = DAT_005202a4;
        local_104[2] = DAT_005202a8;
        local_104[3] = DAT_005202ac;
        local_90 = &local_104[0];
        break;
    case 1:
        local_104[0] = DAT_005202c0;
        local_104[1] = DAT_005202c4;
        local_104[2] = DAT_005202c8;
        local_104[3] = DAT_005202cc;
        auStack_a4[1] = DAT_00521bb0;
        auStack_a4[2] = DAT_00521bb4;
        auStack_a4[3] = DAT_00521bb8;
        auStack_a4[4] = DAT_00521bbc;
        local_90 = &local_104[0];
        break;
    case 2:
        local_104[0] = DAT_005202b0;
        local_104[1] = DAT_005202b4;
        local_104[2] = DAT_005202b8;
        local_104[3] = DAT_005202bc;
        auStack_a4[1] = DAT_00521ba0;
        auStack_a4[2] = DAT_00521ba4;
        auStack_a4[3] = DAT_00521ba8;
        auStack_a4[4] = DAT_00521bac;
        local_90 = &local_104[0];
        break;
    case 3:
        local_104[0] = DAT_005202d0;
        local_104[1] = DAT_005202d4;
        local_104[2] = DAT_005202d8;
        local_104[3] = DAT_005202dc;
        auStack_a4[1] = DAT_00521b90;
        auStack_a4[2] = DAT_00521b94;
        auStack_a4[3] = DAT_00521b98;
        auStack_a4[4] = DAT_00521b9c;
        in_stack_00000028 = (char *)local_dc;
        local_dc[2] = 0;           /* local_d4 */
        local_dc[3] = 0;           /* local_d0 */
        local_dc[0] = 2;
        local_dc[1] = 0x2580;
        local_dc[4] = 1;           /* local_cc */
        local_90 = &local_104[0];
        break;
    }

    uVar4 = 0;
    iVar1 = Ordinal_4(0, DAT_007806fe, 0, 0, 0);
    if (iVar1 != 0) {
        return 0;
    }

    FUN_004cde40_GETDPL(lpDPL);
    iVar1 = FUN_004ce2a0(lpDPL, local_90, acStack_b4, in_stack_00000028);
    if (iVar1 != 0) {
        FUN_004cde40_GETDPL(lpDPL);
        (*(PFN_FUN_004cde40_REL *)(*lpDPL + 8))(lpDPL);
        FUN_004cde40_CLRDPL();
        return 0;
    }

    for (i = 0; i < 0x14; i++) {
        auStack_a4[i] = 0;
    }
    auStack_a4[0] = 0x50;
    auStack_a4[1] = 0;
    auStack_a4[2] = DAT_00520290;
    auStack_a4[3] = DAT_00520294;
    auStack_a4[4] = DAT_00520298;
    auStack_a4[5] = DAT_0052029c;
    auStack_a4[6] = buf_w[12];     /* uStack_10 */
    auStack_a4[7] = buf_w[13];     /* uStack_c  */
    auStack_a4[8] = buf_w[14];     /* uStack_8  */
    auStack_a4[9] = buf_w[15];     /* uStack_4  */
    auStack_a4[10] = 0x10;
    auStack_a4[11] = 0;
    auStack_a4[12] = param_3;
    auStack_a4[13] = 0;
    auStack_a4[14] = 0;
    auStack_a4[15] = 0;
    auStack_a4[16] = 0;
    auStack_a4[17] = 0;
    auStack_a4[18] = 0;
    auStack_a4[19] = 0;

    local_104[1] = 0x10;
    local_104[2] = 0;
    local_104[3] = param_1;

    for (i = 0; i < 10; i++) {
        local_dc[i] = 0;
    }
    local_dc[0] = 0x28;
    local_dc[1] = 2;
    if (param_2 == 0) {
        local_dc[1] = 1;
    }
    local_dc[2] = (undefined4)auStack_a4;
    local_dc[3] = (undefined4)&local_104[1];
    local_dc[9] = local_104[0];    /* uStack_b8 */
    local_dc[4] = uVar4;           /* local_cc  */

    FUN_004cde40_GETDPL(lpDPL);
    iVar1 = (*(PFN_FUN_004cde40_SET *)(*lpDPL + 0x30))(lpDPL, 0, 0, local_dc);
    if (iVar1 == 0) {
        FUN_004cde40_GETDPL(lpDPL);
        iVar1 = (*(PFN_FUN_004cde40_CONN *)(*lpDPL + 0xc))(lpDPL, 0, DAT_007806fa, 0);
        FUN_004cde40_GETDPL(lpDPL);
        (*(PFN_FUN_004cde40_REL *)(*lpDPL + 8))(lpDPL);
        FUN_004cde40_CLRDPL();
        return iVar1 == 0;
    }
    FUN_004cde40_GETDPL(lpDPL);
    (*(PFN_FUN_004cde40_REL *)(*lpDPL + 8))(lpDPL);
    FUN_004cde40_CLRDPL();
    return 0;
}

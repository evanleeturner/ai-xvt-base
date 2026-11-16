/* SPDX-License-Identifier: MIT */
/* z_xvt 004a0880 */
/* Prototypes of callees (types inferred from call sites) */
int  __cdecl FUN_004a1980(unsigned int a);
int  __cdecl FUN_004a1590(int a, unsigned int b, int c, unsigned short d, int e, unsigned int f);
void __cdecl FUN_0040b730(int a, unsigned int b, int c, int d, unsigned int e, unsigned short f);
void __cdecl FUN_004a1d70(int a, int b);
void __cdecl FUN_0040c070(int a, int b, unsigned int c, unsigned int d);

/* Globals (byte-addressed; many fields are unaligned, accessed via memcpy) */
extern int            DAT_006d4988;
extern char           DAT_006d4a02;
extern char           DAT_007092d4;
extern int            DAT_0071ecf4;
extern unsigned char  DAT_0070aee5[];
extern unsigned char  DAT_0070aee9[];
extern unsigned char  DAT_0070ad48[];
extern unsigned char  DAT_0070ad42[];
extern unsigned char  DAT_005247e8[];
extern unsigned char  DAT_0052686a[];
extern unsigned char  DAT_0072f508[];
extern unsigned char  DAT_0071b73c[];
extern unsigned char  DAT_0071b73a[];
extern unsigned char  DAT_0071b79d[];
extern unsigned char  DAT_0071b7a1[];
extern unsigned char  DAT_0071b7c5[];
extern unsigned char  DAT_0071b825[];
extern unsigned char  DAT_0071b885[];
extern unsigned char  DAT_0071b8e5[];
extern unsigned char  DAT_0071b917[];
extern unsigned char  DAT_0071b949[];
extern unsigned char  DAT_0071b97b[];
extern unsigned char  DAT_0071b987[];
extern unsigned char  DAT_0071b993[];
extern unsigned char  DAT_0071b99f[];
extern unsigned char  DAT_0071b9af[];
extern unsigned char  DAT_0071b9bf[];
extern unsigned char  DAT_0071b9c9[];
extern unsigned char  DAT_0071b9d9[];
extern unsigned short DAT_00732fe2;
extern unsigned short DAT_00732950;

#define INC16(p) do { unsigned short t_; __builtin_memcpy(&t_, (p), 2); t_ = (unsigned short)(t_ + 1); __builtin_memcpy((p), &t_, 2); } while (0)

void __cdecl FUN_004a0880(unsigned int param_1, int param_2, int param_3, int param_4, int param_5, int param_6)
{
    char cVar1;
    int iVar2;
    unsigned short uVar3;
    unsigned int uVar4;
    int iVar5;
    int iVar6;
    unsigned int uVar7;
    unsigned char *pcVar8;
    unsigned char *pcVar9;
    unsigned int uVar10;
    unsigned short uStack_12;
    unsigned short local_10;
    int local_4;
    unsigned short w;

    uVar10 = param_1 & 0xffff;
    uStack_12 = 0;
    local_10 = 1;
    uVar3 = (unsigned short)*(unsigned char *)(uVar10 * 0x23u + 0x16u + (unsigned int)DAT_006d4988);
    uVar7 = (unsigned int)uVar3;
    iVar5 = (int)(uVar7 * 0x566u);
    pcVar8 = DAT_0070aee5 + iVar5;
    local_4 = 8;
    iVar2 = (int)((unsigned int)param_4 * 0x5bdu);
    __builtin_memcpy(&w, DAT_0071b73c + iVar2, 2);
    pcVar9 = DAT_0070aee9 + (unsigned int)w + iVar5;
    do {
        if ((*pcVar8 == 0) && (*pcVar9 == 1)) {
            cVar1 = (char)pcVar8[1];
            if (cVar1 == 0) {
LAB_004a093d:
                uStack_12 = 0x6000;
            }
            else if (cVar1 == 2) {
                uStack_12 = 0xf000;
            }
            else if (cVar1 == 10) goto LAB_004a093d;
        }
        pcVar8 = pcVar8 + 0x4e;
        pcVar9 = pcVar9 + 0x4e;
        local_4 = local_4 + -1;
        if (local_4 == 0) {
            __builtin_memcpy(&w, DAT_0071b73c + iVar2, 2);
            if (DAT_0072f508[(unsigned int)w * 0x1e5u + (unsigned int)DAT_0070ad48[iVar5]] == 0) {
                uVar4 = (unsigned int)DAT_005247e8[DAT_0070ad42[(unsigned int)*(unsigned char *)(uVar10 * 0x23u + 0x16u + (unsigned int)DAT_006d4988) * 0x566u]];
                if (DAT_0052686a[uVar4 * 0x16u] == 0) {
                    iVar5 = FUN_004a1980(uVar10);
                }
                else {
                    if (((uVar4 < 0x46) || (0x4a < uVar4)) && ((uVar4 < 0x50 || (0x54 < uVar4)))) {
                        iVar5 = 0x28;
                    }
                    else {
                        iVar5 = 500;
                    }
                    if (DAT_006d4a02 == 2) {
                        iVar5 = (int)((unsigned int)iVar5 * 3u);
                    }
                }
                if (param_3 == 1) {
                    INC16(DAT_0071b885 + uVar7 * 2u + iVar2);
                    if (param_5 == -1) {
                        INC16(DAT_0071b993 + param_6 * 2 + iVar2);
                    }
                    else {
                        INC16(DAT_0071b949 + param_6 * 2 + iVar2);
                    }
                    iVar5 = iVar5 / 10;
                    local_10 = 10;
                }
                else if (param_3 == 2) {
                    INC16(DAT_0071b825 + uVar7 * 2u + iVar2);
                    if (param_5 == -1) {
                        INC16(DAT_0071b987 + param_6 * 2 + iVar2);
                    }
                    else {
                        INC16(DAT_0071b9af + param_5 * 2 + iVar2);
                        INC16(DAT_0071b917 + param_6 * 2 + iVar2);
                        INC16(DAT_0071b9d9 + param_4 * 2 + (int)((unsigned int)param_5 * 0x5bdu));
                    }
                    local_10 = 6;
                    iVar5 = iVar5 / 2;
                }
                else if (param_3 == 3) {
                    INC16(DAT_0071b7c5 + uVar7 * 2u + iVar2);
                    if (param_5 == -1) {
                        INC16(DAT_0071b97b + param_6 * 2 + iVar2);
                    }
                    else {
                        INC16(DAT_0071b99f + param_5 * 2 + iVar2);
                        INC16(DAT_0071b8e5 + param_6 * 2 + iVar2);
                        INC16(DAT_0071b9c9 + param_4 * 2 + (int)((unsigned int)param_5 * 0x5bdu));
                    }
                    local_10 = 1;
                }
                if (DAT_007092d4 == 2) {
                    iVar5 = (int)((unsigned int)iVar5 * 2u);
                }
                else if (DAT_007092d4 == 0) {
                    iVar5 = iVar5 / 2;
                }
                __builtin_memcpy(&w, DAT_0071b73c + iVar2, 2);
                iVar6 = FUN_004a1590(2, ((unsigned int)uStack_12 << 16) | (unsigned int)uVar3,
                                     param_4, local_10, param_2, (unsigned int)w);
                if (-1 < iVar6) {
                    int v;
                    __builtin_memcpy(&v, DAT_0071b79d + iVar2, 4);
                    v = (int)((unsigned int)v + (unsigned int)iVar5);
                    __builtin_memcpy(DAT_0071b79d + iVar2, &v, 4);
                }
                if ((param_3 == 3) && (DAT_0071ecf4 == param_4)) {
                    FUN_0040b730(DAT_0071ecf4, 0xffffffffu, 0x15, -1, 0xffffffffu, uStack_12);
                    return;
                }
            }
            else if ((param_3 == 2) || (param_3 == 3)) {
                int v;
                int c;
                unsigned short s;
                uVar7 = (unsigned int)DAT_005247e8[DAT_0070ad42[(unsigned int)*(unsigned char *)(uVar10 * 0x23u + 0x16u + (unsigned int)DAT_006d4988) * 0x566u]];
                if (DAT_0052686a[uVar7 * 0x16u] == 0) {
                    iVar5 = FUN_004a1980(uVar10);
                }
                else {
                    if (((uVar7 < 0x46) || (0x4a < uVar7)) && ((uVar7 < 0x50 || (0x54 < uVar7)))) {
                        iVar5 = 0x28;
                    }
                    else {
                        iVar5 = 500;
                    }
                    if (DAT_006d4a02 == 2) {
                        iVar5 = (int)((unsigned int)iVar5 * 3u);
                    }
                }
                __builtin_memcpy(&v, DAT_0071b79d + iVar2, 4);
                v = (int)((unsigned int)v - (unsigned int)iVar5);
                __builtin_memcpy(DAT_0071b79d + iVar2, &v, 4);
                INC16(DAT_0071b9bf + iVar2);
                __builtin_memcpy(&c, DAT_0071b7a1 + iVar2, 4);
                c = (int)((unsigned int)c + (unsigned int)-500);
                __builtin_memcpy(DAT_0071b7a1 + iVar2, &c, 4);
                __builtin_memcpy(&s, DAT_0071b73a + iVar2, 2);
                DAT_00732fe2 = s;
                FUN_004a1d70(0x10f, param_4);
                if (iVar5 != 0) {
                    DAT_00732950 = (unsigned short)iVar5;
                    FUN_004a1d70(199, param_4);
                }
                if (DAT_0071ecf4 == param_4) {
                    FUN_0040c070(2, 0x39, uVar10, 0xffff);
                }
            }
            return;
        }
    } while (1);
}

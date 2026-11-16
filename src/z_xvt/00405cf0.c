/* SPDX-License-Identifier: MIT */
/* z_xvt 00405cf0 */
/* Prototypes for every callee */
unsigned int __cdecl FUN_0048faa0(unsigned int *a, unsigned int *b, unsigned int *c);
int __cdecl FUN_00436ec0(void);
void __cdecl FUN_00436ef0(int a, int b); /* 8 arg bytes per stack-check; values unresolved */
int __cdecl FUN_004a53f0(void);
unsigned int __cdecl FUN_004a5470(void);

extern char DAT_006da500[];
extern char DAT_006da504[];
extern short DAT_0073254c;
extern short DAT_00732554;
extern unsigned short DAT_006db592;
extern short DAT_00732f94;
extern short DAT_00732f92;
extern short DAT_0071b6b0;
extern unsigned char DAT_0085e7a0[];
extern unsigned int DAT_00536920;
extern int DAT_00536924;
extern unsigned char DAT_00523618[];
extern short DAT_006d9d88;
extern short DAT_006d9d92;
extern short DAT_0070acc6;
extern short DAT_006d9d3c;
extern short DAT_006d9d3e;

unsigned int __cdecl FUN_00405cf0(int param_1)
{
    unsigned char bVar1;
    short sVar2;
    int iVar3;
    unsigned int uVar4;
    unsigned int uVar5;
    unsigned int uVar6;
    unsigned int uVar7;
    unsigned int uVar8;
    short sVar9;
    short local_1a;   /* uninitialized in the original */
    short local_18;   /* uninitialized in the original */
    short local_16;
    unsigned int local_10;
    int local_c;
    unsigned int local_8;
    unsigned int local_4;
    int diff;
    int t;
    int old;
    volatile short *pLocal1a;
    volatile short *pLocal18;

    uVar8 = 0;
    if (-1 < param_1) {
        DAT_0073254c = (short)*(char *)((int)&DAT_006da500 + (unsigned int)param_1 * 6U + 3U);
        iVar3 = (int)((unsigned int)param_1 * 6U);
        DAT_00732554 = (short)*(char *)((int)&DAT_006da504 + iVar3);
        uVar8 = ((((unsigned int)param_1 * 3U) >> 8 << 8) |
                 (unsigned int)*(unsigned char *)((int)&DAT_006da500 + iVar3 + 2)) & 0xffff00ffU;
        DAT_006db592 = (unsigned short)uVar8;
        DAT_00732f94 = (short)(*(unsigned char *)((int)&DAT_006da504 + iVar3 + 1) & 3);
        return uVar8;
    }
    local_16 = 0;
    local_10 = 0;
    local_4 = 0;
    local_8 = 0;
    if (DAT_00732f92 != 0) {
        uVar8 = FUN_0048faa0(&local_8, &local_4, &local_10);
    }
    if (DAT_0071b6b0 != 0) {
        iVar3 = FUN_00436ec0();
        local_16 = (short)iVar3;
        FUN_00436ef0(0, 0);
    }
    uVar4 = 0;
    iVar3 = FUN_004a53f0();
    if (iVar3 != 0) {
        uVar4 = FUN_004a5470();
        uVar4 = uVar4 & 0xffff;
    }
    sVar9 = 0;
    uVar7 = 1;
    iVar3 = 0;
    local_c = 0;
    do {
        bVar1 = DAT_0085e7a0[iVar3];
        uVar6 = uVar4;
        if (bVar1 != 0) {
            if ((uVar8 & uVar7) == 0) {
                if ((DAT_00536920 & uVar7) != 0) {
                    if (bVar1 == 0xae) {
                        uVar5 = 0xae;
                    }
                    else if ((bVar1 < 0xaf) || (0xb7 < bVar1)) {
                        uVar5 = 0;
                    }
                    else {
                        uVar5 = 0xb6;
                    }
                    if ((uVar5 != 0) && (uVar6 = uVar5, (short)uVar4 != 0)) {
                        uVar8 = uVar8 | uVar7;
                        uVar6 = uVar4;
                    }
                }
            }
            else {
                if (bVar1 == 0x98) {
                    local_c = 1;
                }
                else if (bVar1 == 0x99) {
                    sVar9 = 1;
                }
                if ((DAT_00536920 & uVar7) == 0) {
                    if ((short)uVar4 == 0) {
                        uVar6 = (unsigned int)bVar1;
                    }
                    else {
                        uVar8 = uVar8 & ~uVar7;
                    }
                }
            }
        }
        DAT_006db592 = (unsigned short)uVar6;
        uVar7 = uVar7 * 2U;
        iVar3 = iVar3 + 1;
        uVar4 = uVar6;
    } while (iVar3 < 0x14);
    DAT_00536920 = uVar8;
    sVar2 = (short)local_c;
    iVar3 = DAT_00536924;
    if (DAT_006db592 == 0) {
        local_c = (int)((unsigned int)(int)(char)local_10 + 0x80U);
        iVar3 = local_c;
        if (DAT_00536924 != -1) {
            diff = (int)((unsigned int)local_c - (unsigned int)DAT_00536924);
            iVar3 = (int)((unsigned int)DAT_00536924 +
                          (unsigned int)((int)((unsigned int)diff + ((unsigned int)(diff >> 0x1f) & 3U)) >> 2));
            t = (int)((unsigned int)iVar3 + 8U);
            local_c = (int)((unsigned int)t + ((unsigned int)(t >> 0x1f) & 0xfU)) >> 4;
            old = (int)((unsigned int)DAT_00536924 + 8U);
            if (local_c != (int)((unsigned int)old + ((unsigned int)(old >> 0x1f) & 0xfU)) >> 4) {
                if (local_c < 0) {
                    local_c = 0;
                }
                if (0x10 < local_c) {
                    local_c = 0x10;
                }
                DAT_006db592 = (unsigned short)(unsigned char)(&DAT_00523618[0])[-local_c];
            }
        }
    }
    DAT_00536924 = iVar3;
    DAT_006d9d88 = 0;
    DAT_006d9d92 = 0;
    DAT_0073254c = (short)local_8;
    DAT_00732554 = (short)local_4;
    DAT_00732f94 = (short)(sVar2 + sVar9 * 2);
    DAT_0070acc6 = local_16;
    pLocal1a = &local_1a;
    pLocal18 = &local_18;
    DAT_006d9d3c = *pLocal1a;
    DAT_006d9d3e = *pLocal18;
    return (((unsigned int)local_c >> 0x10) << 0x10) | (unsigned int)DAT_006db592;
}

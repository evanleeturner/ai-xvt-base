/* SPDX-License-Identifier: MIT */
/* z_bop 00406d10 */
extern unsigned char *DAT_009a1fe8;
extern unsigned char *DAT_009a8e18;
extern int DAT_009e9640;
extern int DAT_00a00854;
extern unsigned int DAT_00a07cd4;
extern unsigned int DAT_00a07cd8;
extern unsigned int DAT_00a07cdc;
extern unsigned char DAT_005241f8[];
extern unsigned char DAT_009d8c85[];
extern unsigned char DAT_009d8c87[];
extern unsigned char DAT_009d8c88[];
extern unsigned char DAT_0051a44f[];
extern unsigned char s_starshipprotectpln_0051a2fc[];
extern unsigned char DAT_0051a194[];

unsigned short __cdecl FUN_00459750(int param_1);
short __cdecl FUN_00405860(unsigned int param_1);
void __cdecl FUN_00458e20(unsigned int param_1, int param_2);
unsigned char __cdecl FUN_00425cb0(void);
void __cdecl FUN_00407910(unsigned short param_1);

unsigned short __cdecl FUN_00406d10(unsigned short param_1, unsigned short param_2)
{
    unsigned int iVar1;
    unsigned int uVar2;
    unsigned char bVar3;
    unsigned short uVar4;
    short sVar5;
    unsigned int uVar6;
    unsigned int iVar7;
    unsigned int iVar8;
    unsigned int uVar9;
    unsigned short uVar10;
    unsigned char *sub;
    unsigned char *ent;
    unsigned int tmp32;
    unsigned short tmp16;
    int cmp;

    iVar1 = (((unsigned int)param_1 * 8u) - (unsigned int)param_1) * 5u;
    uVar6 = (unsigned int)*(DAT_009a1fe8 + 0x16 + iVar1);
    iVar7 = uVar6 * 0x566u;
    bVar3 = DAT_005241f8[DAT_009d8c85[iVar7]];
    if (bVar3 == 0) {
        return 0xffff;
    }
    uVar4 = FUN_00459750(7);
    if (uVar4 == 0xffff) {
        uVar4 = (unsigned short)((short)DAT_009e9640 + 0x80);
        while ((int)(unsigned int)uVar4 < DAT_00a00854) {
            iVar8 = (unsigned int)uVar4 * 0x23u + (unsigned int)DAT_009a1fe8;
            if (DAT_0051a44f[*(unsigned char *)(iVar8 + 3)] == 0) {
                __builtin_memcpy(&sub, (unsigned char *)(iVar8 + 0x1f), 4);
                if (*(char *)(sub + 0x89) == (char)DAT_009d8c88[iVar7]) {
                    break;
                }
            }
            uVar4 = (unsigned short)(uVar4 + 1);
        }
    }
    uVar6 = (unsigned int)uVar4;
    if ((unsigned int)DAT_00a00854 == uVar6) {
        return 0xffff;
    }
    iVar8 = uVar6 * 0x23u;
    __builtin_memcpy(&sub, DAT_009a1fe8 + 0x1f + iVar8, 4);
    *sub = 1;
    *(DAT_009a1fe8 + 2 + iVar8) = 7;
    *(DAT_009a1fe8 + 3 + iVar8) = bVar3;
    tmp16 = 1;
    __builtin_memcpy(sub + 0x83, &tmp16, 2);
    __builtin_memcpy(sub + 0x85, &param_1, 2);
    *(sub + 0x87) = *(DAT_009a1fe8 + 3 + iVar1);
    *(sub + 0x88) = DAT_009d8c87[iVar7];
    tmp16 = 0;
    __builtin_memcpy(DAT_009a1fe8 + 0x12 + iVar8, &tmp16, 2);
    uVar9 = (unsigned int)bVar3;
    __builtin_memcpy(DAT_009a1fe8 + 0x14 + iVar8, &tmp16, 2);
    __builtin_memcpy(DAT_009a1fe8 + 0x10 + iVar8, &tmp16, 2);
    __builtin_memcpy(&tmp16, s_starshipprotectpln_0051a2fc + uVar9 * 2u + 10u, 2);
    __builtin_memcpy(sub + 0x79, &tmp16, 2);
    __builtin_memcpy(&tmp16, sub + 0x79, 2);
    __builtin_memcpy(DAT_009a8e18 + 8 + ((uVar6 - (unsigned int)DAT_009e9640) * 10u), &tmp16, 2);
    __builtin_memcpy(&tmp32, DAT_0051a194 + uVar9 * 4u, 4);
    __builtin_memcpy(sub + 0x7d, &tmp32, 4);
    sVar5 = FUN_00405860(uVar9);
    __builtin_memcpy(&sub, DAT_009a1fe8 + 0x1f + iVar8, 4);
    __builtin_memcpy(sub + 0x81, &sVar5, 2);
    FUN_00458e20((unsigned int)param_1, 0);
    uVar2 = DAT_00a07cd8;
    iVar7 = DAT_00a07cd4 + 0x180u;
    __builtin_memcpy(&sub, DAT_009a1fe8 + 0x1f + iVar8, 4);
    __builtin_memcpy(sub + 6, &DAT_00a07cdc, 4);
    ent = DAT_009a1fe8 + iVar8;
    __builtin_memcpy(&sub, ent + 0x1f, 4);
    __builtin_memcpy(&tmp32, sub + 6, 4);
    __builtin_memcpy(ent + 4, &tmp32, 4);
    __builtin_memcpy(&sub, DAT_009a1fe8 + 0x1f + iVar8, 4);
    __builtin_memcpy(sub + 10, &uVar2, 4);
    ent = DAT_009a1fe8 + iVar8;
    __builtin_memcpy(&sub, ent + 0x1f, 4);
    __builtin_memcpy(&tmp32, sub + 10, 4);
    __builtin_memcpy(ent + 8, &tmp32, 4);
    __builtin_memcpy(&sub, DAT_009a1fe8 + 0x1f + iVar8, 4);
    __builtin_memcpy(sub + 0xe, &iVar7, 4);
    ent = DAT_009a1fe8 + iVar8;
    __builtin_memcpy(&sub, ent + 0x1f, 4);
    __builtin_memcpy(&tmp32, sub + 0xe, 4);
    __builtin_memcpy(ent + 0xc, &tmp32, 4);
    uVar10 = (unsigned short)(uVar4 - (short)DAT_009e9640);
    iVar7 = (unsigned int)uVar10 * 10u;
    bVar3 = FUN_00425cb0();
    *(DAT_009a8e18 + 1 + iVar7) = (unsigned char)((bVar3 & 3) + 3);
    __builtin_memcpy(DAT_009a8e18 + 4 + iVar7, &param_2, 2);
    if ((param_2 == 0xffff) || (0x7fff < param_2)) {
        tmp16 = 0;
        __builtin_memcpy(DAT_009a8e18 + 6 + iVar7, &tmp16, 2);
    } else {
        __builtin_memcpy(&tmp16, DAT_009a1fe8 + (unsigned int)param_2 * 0x23u, 2);
        __builtin_memcpy(DAT_009a8e18 + 6 + iVar7, &tmp16, 2);
    }
    *(DAT_009a8e18 + iVar7) = 0xff;
    __builtin_memcpy(&sub, DAT_009a1fe8 + 0x1f + iVar8, 4);
    {
        unsigned char *p = DAT_009a8e18 + iVar7;
        __builtin_memcpy(sub + 0xa5, &p, 4);
    }
    __builtin_memcpy(&cmp, DAT_009a1fe8 + (unsigned int)param_2 * 0x23u + 0x1b, 4);
    if (cmp != -1) {
        FUN_00407910(uVar10);
    }
    return uVar4;
}

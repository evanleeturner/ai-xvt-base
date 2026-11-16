/* SPDX-License-Identifier: MIT */
/* z_bop 0041cfd0 */
extern short DAT_009d8112;
extern unsigned char *DAT_009a1fe8;
extern int DAT_009ec604;
extern int DAT_00a081e8;
extern int DAT_00a08298;
extern unsigned short DAT_009ec5fe;
extern int DAT_009a1fd4;
extern int DAT_009a1fd8;
extern int DAT_009a1fd0;
extern unsigned short DAT_00a07cec;
extern int DAT_009a1fe0;
extern int DAT_009d80c0;
extern int DAT_009d8c18;
extern int DAT_009d1150;
extern int DAT_00a07bdc;
extern int DAT_00a07c54;
extern int DAT_00a07c58;
extern int DAT_00a07ce8;
extern int DAT_009d1308;

void FUN_00427bd0(short a, short b, int c);
unsigned int FUN_00425d80(short a, unsigned int b);
unsigned int FUN_0041c1d0(unsigned short a, unsigned short b);
unsigned int FUN_00446700(unsigned short a, unsigned short b);

unsigned short __cdecl FUN_0041cfd0(unsigned short param_1, short param_2)
{
    char cVar1;
    long long lVar2;
    int iVar3;
    unsigned int uVar4;
    unsigned int uVar5;
    short sVar6;
    unsigned short uVar7;
    unsigned char *iVar8;
    unsigned char *pS;
    unsigned char *pE;
    unsigned int t;
    int a;
    unsigned int prod;
    unsigned short w79;
    short s8c, s8e, s90, s10, s12;
    int i4, i8, ic;
    int fx;
    char c8b;
    unsigned char b12;
    unsigned short w53;
    unsigned int arg2;

    sVar6 = (short)((unsigned int)(int)param_2 * (unsigned int)(int)DAT_009d8112);
    iVar8 = DAT_009a1fe8 + (unsigned int)param_1 * 0x23;
    __builtin_memcpy(&i4, iVar8 + 4, 4);
    DAT_009ec604 = i4;
    __builtin_memcpy(&i8, iVar8 + 8, 4);
    DAT_00a081e8 = i8;
    __builtin_memcpy(&ic, iVar8 + 0xc, 4);
    DAT_00a08298 = ic;
    __builtin_memcpy(&pS, iVar8 + 0x1f, 4);
    __builtin_memcpy(&w79, pS + 0x79, 2);
    t = (unsigned int)w79 * 0x1234u + 0x80u;
    a = (int)t >> 8;
    prod = (unsigned int)a * (unsigned int)DAT_009ec5fe;
    uVar4 = (unsigned int)((int)prod / 0xec) & 0xffff;
    c8b = (char)pS[0x8b];
    if (c8b != '\0') {
        __builtin_memcpy(&s12, iVar8 + 0x12, 2);
        __builtin_memcpy(&s10, iVar8 + 0x10, 2);
        FUN_00427bd0(s12, s10, (int)iVar8);
    }
    iVar3 = (int)sVar6;
    __builtin_memcpy(&pS, iVar8 + 0x1f, 4);
    __builtin_memcpy(&s8c, pS + 0x8c, 2);
    lVar2 = (long long)(int)s8c * (long long)(int)uVar4;
    fx = (int)(((unsigned int)lVar2 >> 0xf) | ((unsigned int)((unsigned long long)lVar2 >> 0x20) << 0x11));
    DAT_009a1fd4 = (int)((unsigned int)iVar3 * (unsigned int)fx + (unsigned int)DAT_009ec604);
    __builtin_memcpy(&s8e, pS + 0x8e, 2);
    lVar2 = (long long)(int)s8e * (long long)(int)uVar4;
    fx = (int)(((unsigned int)lVar2 >> 0xf) | ((unsigned int)((unsigned long long)lVar2 >> 0x20) << 0x11));
    DAT_009a1fd8 = (int)((unsigned int)fx * (unsigned int)iVar3 + (unsigned int)DAT_00a081e8);
    __builtin_memcpy(&s90, pS + 0x90, 2);
    lVar2 = (long long)(int)s90 * (long long)(int)uVar4;
    fx = (int)(((unsigned int)lVar2 >> 0xf) | ((unsigned int)((unsigned long long)lVar2 >> 0x20) << 0x11));
    DAT_009a1fd0 = (int)((unsigned int)fx * (unsigned int)iVar3 + (unsigned int)DAT_00a08298);
    uVar7 = DAT_00a07cec;
    if ((int)(unsigned int)DAT_00a07cec < DAT_009a1fe0) {
        do {
            pE = DAT_009a1fe8 + (unsigned int)uVar7 * 0x23;
            if ((pE[3] != 0) && (uVar7 != param_1) &&
                ((cVar1 = (char)pE[2], cVar1 == '\x04') || (cVar1 == '\x05') || (cVar1 == '\x03'))) {
                __builtin_memcpy(&DAT_009d80c0, pE + 4, 4);
                __builtin_memcpy(&DAT_009d8c18, pE + 8, 4);
                __builtin_memcpy(&DAT_009d1150, pE + 0xc, 4);
                __builtin_memcpy(&pS, pE + 0x1f, 4);
                __builtin_memcpy(&w79, pS + 0x79, 2);
                arg2 = ((unsigned int)(unsigned short)((unsigned int)DAT_009d80c0 >> 0x10) << 16) |
                       (unsigned int)(unsigned short)DAT_009d8112;
                uVar4 = FUN_00425d80((short)w79, arg2);
                uVar4 = uVar4 & 0xffff;
                __builtin_memcpy(&pS, pE + 0x1f, 4);
                if (pS[0x8b] != 0) {
                    __builtin_memcpy(&s12, pE + 0x12, 2);
                    __builtin_memcpy(&s10, pE + 0x10, 2);
                    FUN_00427bd0(s12, s10, (int)pE);
                }
                __builtin_memcpy(&pS, pE + 0x1f, 4);
                __builtin_memcpy(&s8c, pS + 0x8c, 2);
                lVar2 = (long long)(int)s8c * (long long)(int)uVar4;
                fx = (int)(((unsigned int)lVar2 >> 0xf) | ((unsigned int)((unsigned long long)lVar2 >> 0x20) << 0x11));
                DAT_00a07bdc = (int)((unsigned int)fx * (unsigned int)iVar3 + (unsigned int)DAT_009d80c0);
                __builtin_memcpy(&pS, pE + 0x1f, 4);
                __builtin_memcpy(&s8e, pS + 0x8e, 2);
                lVar2 = (long long)(int)s8e * (long long)(int)uVar4;
                fx = (int)(((unsigned int)lVar2 >> 0xf) | ((unsigned int)((unsigned long long)lVar2 >> 0x20) << 0x11));
                DAT_00a07c54 = (int)((unsigned int)fx * (unsigned int)iVar3 + (unsigned int)DAT_009d8c18);
                __builtin_memcpy(&pS, pE + 0x1f, 4);
                __builtin_memcpy(&s90, pS + 0x90, 2);
                lVar2 = (long long)(int)s90 * (long long)(int)uVar4;
                fx = (int)(((unsigned int)lVar2 >> 0xf) | ((unsigned int)((unsigned long long)lVar2 >> 0x20) << 0x11));
                DAT_00a07c58 = (int)((unsigned int)fx * (unsigned int)iVar3 + (unsigned int)DAT_009d1150);
                uVar4 = FUN_0041c1d0(param_1, uVar7);
                if ((short)uVar4 != 0) {
                    return uVar7;
                }
            }
            uVar7 = (unsigned short)(uVar7 + 1);
        } while ((int)(unsigned int)uVar7 < DAT_009a1fe0);
    }
    uVar4 = 0;
    __builtin_memcpy(&pS, DAT_009a1fe8 + 0x1f + (unsigned int)param_1 * 0x23, 4);
    b12 = pS[0x12];
    if (b12 != 0) {
        do {
            __builtin_memcpy(&w53, pS + 0x53 + uVar4 * 2, 2);
            uVar7 = w53;
            if ((DAT_00a07ce8 <= (int)(unsigned int)uVar7) &&
                ((int)(unsigned int)uVar7 < (int)((unsigned int)DAT_009d1308 + (unsigned int)DAT_00a07ce8))) {
                uVar5 = FUN_00446700(param_1, uVar7);
                if ((short)uVar5 != 0) {
                    return uVar7;
                }
            }
            uVar4 = (unsigned int)(unsigned short)((short)uVar4 + 1);
        } while (uVar4 < (unsigned int)pS[0x12]);
    }
    return 0xffff;
}

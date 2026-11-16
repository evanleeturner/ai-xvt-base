/* SPDX-License-Identifier: MIT */
/* z_bop 00467710 */
/* Globals: types are inferred from the decompile only; DAT_00a08104 is treated as a byte pointer. */
extern unsigned char *DAT_00a08104;
extern unsigned short DAT_009a73c0;
extern unsigned short DAT_009a73b0;
extern unsigned short DAT_009a73c2;
extern unsigned char *DAT_009a1fe8;
extern unsigned char DAT_009d8cc5[];
extern unsigned char DAT_009d8c80[];
extern unsigned char DAT_009d9192[];
extern unsigned char DAT_009d8c30[];
extern unsigned char DAT_009d8cd2[];
extern unsigned char DAT_009a20f4[];
extern unsigned char DAT_009a20f6[];
extern unsigned char DAT_009a2117[];
extern unsigned char DAT_009a2118[];
extern unsigned char DAT_0051c5aa[];
extern unsigned char DAT_0051c5ac[];
extern unsigned char DAT_00524120[];
extern unsigned char DAT_009a7a40[];
extern unsigned char DAT_009d1320[];
extern char s_waitforboardpln_0051a358[];
extern unsigned short DAT_00a08292;
extern unsigned short _DAT_00a07be2;
extern unsigned short _DAT_00a07be4;

unsigned int __cdecl FUN_00425ba0(unsigned int a, unsigned int b);
unsigned int __cdecl FUN_00405820(unsigned int a);
unsigned int __cdecl FUN_00425b70(unsigned int a, unsigned short b);
unsigned int __cdecl FUN_00430200(int a, int b, int c, unsigned short d);
void __cdecl FUN_00451bf0(int a, int b);
unsigned int __cdecl FUN_0043c740(unsigned int a, unsigned char *b);
void __cdecl FUN_00450650(int a, int b);

unsigned char __stdcall FUN_00467710(void)
{
    unsigned char bVar1;
    char cVar2;
    unsigned short uVar3;
    unsigned short uVar7;
    unsigned int uVar4;
    unsigned char *puVar5;
    unsigned int uVar6;
    int iVar8;
    char *pcVar9;
    char *pcVar10;
    unsigned char bVar11;
    unsigned char bVar12;
    unsigned short local_6;
    unsigned short uStack_4;
    unsigned short sh0;
    unsigned short sh1;
    unsigned int i0;
    unsigned int i1;
    int sv;
    unsigned char *pp;
    unsigned char *p;

    p = DAT_00a08104;
    __builtin_memcpy(&sh0, p + 0x92, 2);
    if (sh0 == 0) {
        return 0;
    }
    bVar11 = 0;
    uVar7 = (unsigned short)(((unsigned int)DAT_009a73c0 * 0x2b3u) >> 0x10);
    switch (DAT_009d8cc5[(unsigned int)DAT_009a73c0 * 0x566u]) {
    case 1:
        bVar11 = (p[0x1e5] & 1) == 0;
        cVar2 = (char)DAT_009a1fe8[(unsigned int)DAT_009a73b0 * 0x23u + 2];
        if ((cVar2 == 3) || (cVar2 == 4)) {
            __builtin_memcpy(&i0, p + 0x215, 4);
            __builtin_memcpy(&i1, p + 0x211, 4);
            if (i0 + i1 == 0) {
                bVar11 = 1;
            }
        }
        uStack_4 = 0x186;
        break;
    case 2:
        bVar11 = (p[0x1e5] & 0x10) == 0;
        uStack_4 = 0x18a;
        break;
    case 3:
        bVar11 = 0;
        local_6 = 0;
        bVar1 = p[0x232];
        bVar12 = 0;
        if (bVar1 != 0) {
            do {
                bVar11 = bVar12;
                uVar4 = (unsigned int)local_6;
                if (p[uVar4 + 0x233] != 0) {
                    iVar8 = (int)((unsigned int)p[4] * 0x101u + uVar4);
                    uVar4 = (unsigned int)DAT_0051c5aa[iVar8];
                    uVar3 = (unsigned short)DAT_0051c5aa[iVar8];
                    while (uVar3 <= DAT_0051c5ac[iVar8]) {
                        if (p[uVar4 * 4u + 0x372] != 0) {
                            bVar11 = 1;
                        }
                        uVar3 = (unsigned short)((unsigned short)uVar4 + 1);
                        uVar4 = (unsigned int)uVar3;
                    }
                }
                local_6 = (unsigned short)(local_6 + 1);
                bVar12 = bVar11;
            } while (local_6 < bVar1);
        }
        bVar11 = !bVar11 || (p[0x1e5] & 8) == 0;
        uStack_4 = 0x18a;
        break;
    case 4:
        __builtin_memcpy(&i0, p + 0xcb, 4);
        uVar4 = FUN_00425ba0(i0, 0x8000);
        __builtin_memcpy(&i1, p + 0xc3, 4);
        bVar11 = uVar4 <= i1;
        uStack_4 = 0x188;
        break;
    case 5:
        uVar3 = 0;
        do {
            if (p[(unsigned int)uVar3 + 0x1ec] != 0) {
                bVar11 = 1;
            }
            uVar3 = (unsigned short)(uVar3 + 1);
        } while (uVar3 < 10);
        uStack_4 = 0x18b;
        break;
    case 6:
        uVar3 = 0x8000;
        uVar4 = FUN_00405820(((unsigned int)uVar7 << 16) | (unsigned int)DAT_009a73b0);
        uVar4 = FUN_00425b70(uVar4, uVar3);
        __builtin_memcpy(&i0, p + 0x215, 4);
        __builtin_memcpy(&i1, p + 0x211, 4);
        bVar11 = (i0 + i1) <= (uVar4 & 0xffff);
        uStack_4 = 0x184;
        break;
    case 7:
        uVar3 = 0x4000;
        uVar4 = FUN_00405820(((unsigned int)uVar7 << 16) | (unsigned int)DAT_009a73b0);
        uVar4 = FUN_00425b70(uVar4, uVar3);
        __builtin_memcpy(&i0, p + 0x215, 4);
        __builtin_memcpy(&i1, p + 0x211, 4);
        bVar11 = (i0 + i1) <= (uVar4 & 0xffff);
        uStack_4 = 0x185;
        break;
    case 8:
        __builtin_memcpy(&i0, p + 0xcb, 4);
        uVar4 = FUN_00425ba0(i0, 0x4000);
        __builtin_memcpy(&i1, p + 0xc3, 4);
        bVar11 = uVar4 <= i1;
        uStack_4 = 0x187;
        break;
    case 9:
        __builtin_memcpy(&i0, p + 0xcb, 4);
        uVar4 = FUN_00425ba0(i0, 0xc000);
        __builtin_memcpy(&i1, p + 0xc3, 4);
        bVar11 = uVar4 <= i1;
        uStack_4 = 0x189;
    }
    if (bVar11) {
        if (p[0x73] == 0) {
            uVar4 = (unsigned int)DAT_009a73c0;
            __builtin_memcpy(&sh0, DAT_009a20f4 + uVar4 * 0x126u, 2);
            sh0 = (unsigned short)(sh0 + 1);
            __builtin_memcpy(DAT_009a20f4 + uVar4 * 0x126u, &sh0, 2);
            iVar8 = (int)(uVar4 * 0x126u);
            pcVar9 = (char *)(p + 0xb0);
            cVar2 = (char)DAT_009d8c80[uVar4 * 0x566u];
            if (*pcVar9 == cVar2) {
                DAT_009a2117[iVar8] = 1;
            }
            puVar5 = p + 0x74;
            if (*puVar5 != 0) {
                __builtin_memcpy(&sh0, DAT_009a20f6 + iVar8, 2);
                sh0 = (unsigned short)(sh0 - 1);
                __builtin_memcpy(DAT_009a20f6 + iVar8, &sh0, 2);
                if (*pcVar9 == cVar2) {
                    DAT_009a2118[iVar8] = 0;
                }
                *puVar5 = 0;
            }
            FUN_00430200(1, 0xf, (int)(unsigned int)DAT_009a73b0, 0xffff);
            __builtin_memcpy(&sv, DAT_009d9192 + (unsigned int)DAT_009a73c0 * 0x566u, 4);
            if (sv != -1) {
                __builtin_memcpy(&pp, DAT_009a1fe8 + (unsigned int)DAT_009a73b0 * 0x23u + 0x1f, 4);
                DAT_00a08292 = (unsigned short)pp[0x88];
                FUN_00451bf0(0, (int)DAT_009d8c30 + (int)((unsigned int)DAT_009a73c0 * 0x566u));
                uVar6 = FUN_0043c740((unsigned int)DAT_009a73c0, DAT_00a08104);
                _DAT_00a07be2 = (unsigned short)uVar6;
                _DAT_00a07be4 = uStack_4;
                __builtin_memcpy(&sv, DAT_009d9192 + (unsigned int)DAT_009a73c0 * 0x566u, 4);
                FUN_00450650(0x183, sv);
            }
            bVar12 = ((unsigned int)DAT_009a73c2 * 0x52u) == 0;
            iVar8 = 0x10;
            pcVar9 = (char *)(DAT_009d1320 +
                              (unsigned int)DAT_009a7a40[
                                  (unsigned int)DAT_00524120[
                                      DAT_009d8cd2[(unsigned int)DAT_009a73c0 * 0x566u +
                                                   (unsigned int)DAT_009a73c2 * 0x52u]]] * 0x55u);
            pcVar10 = s_waitforboardpln_0051a358;
            do {
                if (iVar8 == 0) break;
                iVar8 = iVar8 - 1;
                bVar12 = *pcVar9 == *pcVar10;
                pcVar9 = pcVar9 + 1;
                pcVar10 = pcVar10 + 1;
            } while (bVar12);
            __builtin_memcpy(&sh0, p + 0xcf, 2);
            if (bVar12 && sh0 == 0) {
                __builtin_memcpy(&sh1, p + 0x1e3, 2);
                __builtin_memcpy(p + 0x1e5, &sh1, 2);
            }
        }
        p[0x73] = 1;
    }
    return bVar11;
}

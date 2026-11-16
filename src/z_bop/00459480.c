/* SPDX-License-Identifier: MIT */
/* z_bop 00459480 */
extern unsigned char *DAT_009a1fe8;
extern unsigned int DAT_009a20b0;
extern unsigned int DAT_009cd26c;
extern unsigned int DAT_009fd438;
extern unsigned short DAT_009d8112;

unsigned short __cdecl FUN_00459750(unsigned int param_1);
void __cdecl FUN_00459f30(unsigned int param_1, unsigned int param_2);
unsigned int __cdecl FUN_00425cb0(void);
unsigned int __cdecl FUN_00425d80(short param_1, unsigned int param_2);
void __cdecl FUN_00427bd0(short param_1, short param_2, int param_3);
void __cdecl FUN_004464c0(int param_1);

unsigned short __cdecl FUN_00459480(unsigned int param_1)
{
    unsigned char *e;
    unsigned char *p;
    unsigned short uVar8;
    unsigned short uVar9;
    unsigned int uVar10;
    unsigned int uVar14;
    unsigned int iVar12;
    unsigned int uVar4;
    short sVar11;
    short sVar13;
    short s;
    unsigned short us;
    unsigned int u32;
    long long lVar5;
    long long lVar6;
    long long lVar7;

    uVar8 = FUN_00459750(0xd);
    if (uVar8 == 0xffff) {
        return 0xffff;
    }
    FUN_00459f30((unsigned int)uVar8, param_1 & 0xffff);
    iVar12 = (unsigned int)uVar8 * 0x23;

    __builtin_memcpy(&p, DAT_009a1fe8 + 0x1f + iVar12, 4);
    *p = 5;
    DAT_009a1fe8[2 + iVar12] = 0xd;
    __builtin_memcpy(&p, DAT_009a1fe8 + 0x1f + iVar12, 4);
    p[1] = 2;
    DAT_009a1fe8[3 + iVar12] = 0x9d;
    __builtin_memcpy(&p, DAT_009a1fe8 + 0x1f + iVar12, 4);
    p[0x87] = DAT_009a1fe8[(param_1 & 0xffff) * 0x23 + 3];
    u32 = 0xffffffffu;
    __builtin_memcpy(DAT_009a1fe8 + 0x1b + iVar12, &u32, 4);

    uVar9 = (unsigned short)FUN_00425cb0();
    sVar13 = (short)((uVar9 & 0x1fff) + 0x100);
    uVar9 = (unsigned short)FUN_00425cb0();
    sVar11 = (short)((uVar9 & 0x1fff) + 0x100);
    uVar10 = FUN_00425cb0();
    if ((uVar10 & 1) != 0) {
        sVar13 = (short)-sVar13;
    }
    uVar10 = FUN_00425cb0();
    if ((uVar10 & 1) != 0) {
        sVar11 = (short)-sVar11;
    }

    e = DAT_009a1fe8 + iVar12;

    __builtin_memcpy(&s, e + 0x10, 2);
    s = (short)((unsigned int)(int)s + (unsigned int)-0x8000);
    __builtin_memcpy(e + 0x10, &s, 2);
    __builtin_memcpy(&s, e + 0x10, 2);
    s = (short)((unsigned int)(int)s + (unsigned int)(int)sVar13);
    __builtin_memcpy(e + 0x10, &s, 2);

    __builtin_memcpy(&s, e + 0x12, 2);
    s = (short)((unsigned int)-0x8000 - (unsigned int)(int)s);
    __builtin_memcpy(e + 0x12, &s, 2);
    __builtin_memcpy(&s, e + 0x12, 2);
    s = (short)((unsigned int)(int)s + (unsigned int)(int)sVar11);
    __builtin_memcpy(e + 0x12, &s, 2);

    __builtin_memcpy(&uVar9, e + 0x12, 2);
    if (0x7fff < uVar9) {
        us = (unsigned short)(0u - (unsigned int)uVar9);
        __builtin_memcpy(e + 0x12, &us, 2);
        __builtin_memcpy(&s, e + 0x10, 2);
        s = (short)((unsigned int)(int)s + (unsigned int)-0x8000);
        __builtin_memcpy(e + 0x10, &s, 2);
    }

    __builtin_memcpy(&p, e + 0x1f, 4);
    p[0x92] = 1;
    __builtin_memcpy(&p, e + 0x1f, 4);
    p[0x8b] = p[0x92];

    uVar9 = (unsigned short)FUN_00425cb0();
    __builtin_memcpy(&p, e + 0x1f, 4);
    us = (unsigned short)((uVar9 & 0xf) + 0x23);
    __builtin_memcpy(p + 0x79, &us, 2);

    __builtin_memcpy(&p, e + 0x1f, 4);
    us = 0;
    __builtin_memcpy(p + 0x83, &us, 2);

    uVar9 = (unsigned short)FUN_00425cb0();
    __builtin_memcpy(&p, e + 0x1f, 4);
    us = (unsigned short)((uVar9 & 3) + 0x27);
    __builtin_memcpy(p + 0x81, &us, 2);

    e[0x19] = 2;

    e = e + 0; /* iVar12 = iVar12 + DAT_009a1fe8 */
    e = DAT_009a1fe8 + iVar12;

    __builtin_memcpy(&p, e + 0x1f, 4);
    __builtin_memcpy(&u32, e + 4, 4);
    __builtin_memcpy(p + 6, &u32, 4);

    __builtin_memcpy(&uVar4, e + 8, 4);
    __builtin_memcpy(&p, e + 0x1f, 4);
    __builtin_memcpy(p + 10, &uVar4, 4);

    __builtin_memcpy(&p, e + 0x1f, 4);
    __builtin_memcpy(&u32, e + 0xc, 4);
    __builtin_memcpy(p + 0xe, &u32, 4);

    __builtin_memcpy(&p, e + 0x1f, 4);
    __builtin_memcpy(&s, p + 0x79, 2);
    uVar10 = FUN_00425d80(s, ((uVar4 >> 0x10) << 0x10) | (unsigned int)DAT_009d8112);
    uVar14 = uVar10 & 0xffff;
    if ((short)uVar10 != 0) {
        short a, b, c, d;
        __builtin_memcpy(&p, e + 0x1f, 4);
        if (p[0x8b] != 0) {
            __builtin_memcpy(&a, e + 0x12, 2);
            __builtin_memcpy(&b, e + 0x10, 2);
            FUN_00427bd0(a, b, (int)e);
        }
        __builtin_memcpy(&p, e + 0x1f, 4);
        __builtin_memcpy(&a, p + 0x8c, 2);
        lVar5 = (long long)(int)a * (long long)(int)uVar14;
        __builtin_memcpy(&p, e + 0x1f, 4);
        __builtin_memcpy(&b, p + 0x8e, 2);
        lVar6 = (long long)(int)b * (long long)(int)uVar14;
        __builtin_memcpy(&p, e + 0x1f, 4);
        __builtin_memcpy(&c, p + 0x90, 2);
        lVar7 = (long long)(int)c * (long long)(int)uVar14;
        (void)d;

        DAT_009a20b0 = (((unsigned int)lVar5 >> 0xf) |
                        ((unsigned int)((unsigned long long)lVar5 >> 0x20) << 0x11)) << 2;
        DAT_009cd26c = (((unsigned int)lVar6 >> 0xf) |
                        ((unsigned int)((unsigned long long)lVar6 >> 0x20) << 0x11)) << 2;
        DAT_009fd438 = (((unsigned int)lVar7 >> 0xf) |
                        ((unsigned int)((unsigned long long)lVar7 >> 0x20) << 0x11)) << 2;
        FUN_004464c0((int)e);
    }
    return uVar8;
}

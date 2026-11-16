/* SPDX-License-Identifier: MIT */
/* z_bop 0045d6e0 */
/* Unresolved: the high 16 bits of unaff_EDI are never set in the decompile
   (register carried over from the caller). Initialised to 0 here. The value
   is passed whole to FUN_00403070, so its high half may matter. */

extern unsigned short DAT_00a07cec;
extern int            DAT_009a1fe0;
extern int            DAT_009a1fe8;
extern unsigned int   DAT_009a1ff8;
extern unsigned short DAT_009a73b0;
extern unsigned char  DAT_009a73da;
extern unsigned char  DAT_009a73db[4];   /* packed, address not 4-aligned */
extern unsigned char  DAT_009a73df[4];   /* packed */
extern unsigned char  DAT_009a73e3[4];   /* packed */
extern int            DAT_00a08104;
extern unsigned char  DAT_009d8c88[];
extern unsigned char  DAT_009fd458[];

unsigned char FUN_00433390(unsigned int a, unsigned short b, unsigned short c);
void          FUN_00403540(unsigned int a, unsigned int b);
unsigned char FUN_00403070(unsigned int a);
unsigned char FUN_0045dba0(unsigned short a, int b);
unsigned int  FUN_0041f410(int a, int b, int c);

#define LD32(dst, addr) __builtin_memcpy(&(dst), (const void *)(addr), 4)

unsigned short __cdecl FUN_0045d6e0(unsigned short param_1, unsigned short param_2,
                                    short param_3, unsigned short param_4,
                                    unsigned short param_5)
{
    char cVar1;
    int iVar2;
    int bVar3;
    unsigned short uVar4;
    int iVar5;
    int iVar6;
    int iVar7;
    unsigned int uVar8;
    unsigned short uVar9;
    unsigned int uVar10;
    unsigned int unaff_EDI = 0;
    unsigned short local_10;
    unsigned short local_e;
    unsigned short uStack_c;
    unsigned int local_4;
    int t0, t1, t2;
    unsigned int u0, u1;
    int d0, d1, d2;

    local_e = DAT_00a07cec;
    local_4 = 0xffffffff;
    local_10 = 0xffff;
    if ((int)(unsigned int)DAT_00a07cec < DAT_009a1fe0) {
        do {
            iVar5 = (int)((unsigned int)local_e * 0x23u + (unsigned int)DAT_009a1fe8);
            if (*(char *)(iVar5 + 3) != '\0') {
                uVar10 = 0;
                LD32(t0, iVar5 + 0x1f);
                LD32(iVar5, t0 + 0xa9);
                bVar3 = 0;
                uStack_c = (unsigned short)iVar5;
                do {
                    if (*(char *)(uVar10 + 0x1ec + iVar5) != '\0') {
                        bVar3 = 1;
                    }
                    uVar9 = (unsigned short)((short)uVar10 + 1);
                    uVar10 = (unsigned int)uVar9;
                } while (uVar9 < 10);
                if (bVar3) {
                    bVar3 = FUN_00433390(((unsigned int)uStack_c << 16) | local_e, param_1, param_2);
                    uVar9 = (unsigned short)(unsigned char)bVar3;
                    bVar3 = FUN_00433390(((unsigned int)uStack_c << 16) | local_e, param_4, param_5);
                    uVar4 = (unsigned short)(unsigned char)bVar3;
                    if (param_3 == 1) {
                        uVar9 = uVar9 | uVar4;
                    } else {
                        uVar9 = uVar9 & uVar4;
                    }
                    if ((uVar9 != 0) &&
                        (unaff_EDI = (unaff_EDI & 0xffff0000u) | DAT_00a07cec,
                         (int)(unsigned int)DAT_00a07cec < DAT_009a1fe0)) {
                        do {
                            uVar10 = unaff_EDI & 0xffff;
                            iVar2 = (int)(uVar10 * 0x23u);
                            uVar9 = (unsigned short)unaff_EDI;
                            if (*(char *)(DAT_009a1fe8 + iVar2 + 3) != '\0') {
                                LD32(t0, DAT_009a1fe8 + iVar2 + 0x1f);
                                LD32(iVar6, t0 + 0xa9);
                                cVar1 = *(char *)(iVar6 + 0x5e);
                                if ((((((cVar1 == '\v') || (cVar1 == '\f')) || (cVar1 == '\x17')) &&
                                      (*(unsigned short *)(iVar6 + 0x42) == local_e)) ||
                                     (*(unsigned short *)(iVar5 + 0x6e) == uVar9)) &&
                                    (*(unsigned short *)(DAT_00a08104 + 0x36e) != local_e)) {
                                    if (uVar9 == 0xffff) {
                                        iVar6 = 0;
                                    } else {
                                        iVar6 = (int)(uVar10 * 0x23u + (unsigned int)DAT_009a1fe8);
                                        LD32(t0, iVar6 + 0x1f);
                                        if (t0 == 0) {
                                            bVar3 = *(char *)(iVar6 + 3) == '\0';
LAB_0045d994:
                                            iVar6 = (int)(1u - (unsigned int)bVar3);
                                        } else if (*(char *)(iVar6 + 3) == '\0') {
                                            iVar6 = 0;
                                        } else {
                                            LD32(t0, iVar6 + 0x1f);
                                            LD32(iVar6, t0 + 0xa9);
                                            if (iVar6 == 0) {
                                                iVar6 = 1;
                                            } else if ((*(char *)(iVar6 + 0x5e) == '\x15') &&
                                                       (*(char *)(iVar6 + 0x5f) != '\0')) {
                                                iVar6 = 0;
                                            } else {
                                                cVar1 = *(char *)(iVar6 + 7);
                                                if ((cVar1 == '\x01') || ((cVar1 == '\x03') || (cVar1 == '\x04'))) {
                                                    iVar6 = 0;
                                                } else if ((*(char *)(iVar6 + 0x23d) == '\x03') &&
                                                           (*(char *)(iVar6 + 0x241) != '\0')) {
                                                    FUN_00403540(uVar10, (unsigned int)DAT_009a73b0);
                                                    cVar1 = *(char *)((unsigned int)DAT_009a73b0 * 0x23u + 2 +
                                                                      (unsigned int)DAT_009a1fe8);
                                                    if ((cVar1 == '\0') || ((cVar1 == '\x01') || (cVar1 == '\x02'))) {
                                                        if ((int)DAT_009a1ff8 < 0x8001) goto LAB_0045d956;
                                                        iVar6 = 0;
                                                    } else {
                                                        if ((int)DAT_009a1ff8 < 0x10001) goto LAB_0045d956;
                                                        iVar6 = 0;
                                                    }
                                                } else {
LAB_0045d956:
                                                    iVar7 = (int)((unsigned int)DAT_009a1fe8 + uVar10 * 0x23u);
                                                    LD32(t0, iVar7 + 0x1b);
                                                    if (t0 == -1) {
                                                        LD32(t1, iVar7 + 0x1f);
                                                        LD32(t2, t1 + 0xa9);
                                                        LD32(u0, t2 + 0xcb);
                                                        LD32(u1, iVar6 + 0xc3);
                                                        bVar3 = u0 < u1;
                                                        goto LAB_0045d994;
                                                    }
                                                    iVar6 = 1;
                                                }
                                            }
                                        }
                                    }
                                    if (iVar6 != 0) {
                                        LD32(t0, DAT_009a1fe8 + iVar2 + 0x1b);
                                        if (t0 != -1) {
                                            LD32(t1, (unsigned int)DAT_009a73b0 * 0x23u + 0x1f + (unsigned int)DAT_009a1fe8);
                                            uVar8 = (unsigned int)*(unsigned char *)(t1 + 0x89);
                                            if (DAT_009d8c88[(unsigned int)*(unsigned char *)(DAT_009a1fe8 + iVar2 + 0x16) * 0x566] == uVar8) {
                                                bVar3 = 0;
                                            } else {
                                                bVar3 = DAT_009fd458[(unsigned int)DAT_009d8c88[(unsigned int)*(unsigned char *)(DAT_009a1fe8 + iVar2 + 0x16) * 0x566] + uVar8 * 0x1e5] == '\0';
                                            }
                                            if (!bVar3) goto LAB_0045db52;
                                        }
                                        if ((DAT_009a73da & 4) != 0) {
                                            bVar3 = FUN_00403070(unaff_EDI);
                                            if ((short)(unsigned char)bVar3 == 0) goto LAB_0045db52;
                                        }
                                        if ((DAT_009a73da & 1) != 0) {
                                            bVar3 = FUN_0045dba0(uVar9, 0xff);
                                            if ((short)(unsigned char)bVar3 == 0) goto LAB_0045db52;
                                        }
                                        if ((DAT_009a73da & 0x20) == 0) {
                                            FUN_00403540((unsigned int)DAT_009a73b0, uVar10);
                                        } else {
                                            iVar6 = DAT_009a1fe8 + iVar2;
                                            LD32(t0, iVar6 + 4);
                                            LD32(d0, DAT_009a73db);
                                            LD32(t1, iVar6 + 8);
                                            LD32(d1, DAT_009a73df);
                                            LD32(t2, iVar6 + 0xc);
                                            LD32(d2, DAT_009a73e3);
                                            DAT_009a1ff8 = FUN_0041f410((int)((unsigned int)t0 - (unsigned int)d0),
                                                                        (int)((unsigned int)t1 - (unsigned int)d1),
                                                                        (int)((unsigned int)t2 - (unsigned int)d2));
                                        }
                                        if ((DAT_009a73da & 0x10) == 0) {
LAB_0045db3c:
                                            if (DAT_009a1ff8 < local_4) {
                                                local_4 = DAT_009a1ff8;
                                                local_10 = uVar9;
                                            }
                                        } else if ((int)DAT_009a1ff8 < 0x10001) {
                                            bVar3 = 0;
                                            LD32(t1, (unsigned int)DAT_009a73b0 * 0x23u + 0x1f + (unsigned int)DAT_009a1fe8);
                                            uVar10 = (unsigned int)*(unsigned char *)(t1 + 0x89);
                                            if (DAT_009d8c88[(unsigned int)*(unsigned char *)(DAT_009a1fe8 + 0x16 + iVar2) * 0x566] != uVar10) {
                                                bVar3 = DAT_009fd458[(unsigned int)DAT_009d8c88[(unsigned int)*(unsigned char *)(DAT_009a1fe8 + 0x16 + iVar2) * 0x566] + uVar10 * 0x1e5] == '\0';
                                            }
                                            if (bVar3) goto LAB_0045db3c;
                                        }
                                    }
                                }
                            }
LAB_0045db52:
                            unaff_EDI = (unaff_EDI & 0xffff0000u) | (unsigned short)(uVar9 + 1);
                        } while ((int)(unsigned int)(unsigned short)(uVar9 + 1) < DAT_009a1fe0);
                    }
                }
            }
            local_e = (unsigned short)(local_e + 1);
        } while ((int)(unsigned int)local_e < DAT_009a1fe0);
    }
    return local_10;
}

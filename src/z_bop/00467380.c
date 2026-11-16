/* SPDX-License-Identifier: MIT */
/* z_bop 00467380 */
extern unsigned int DAT_009a73b6;   /* packed: address 0x9a73b6 is unaligned */
extern unsigned int DAT_00a08104;
extern unsigned int DAT_009a1fe8;
extern int DAT_009a1fe0;
extern unsigned int DAT_009a1ff8;
extern unsigned short DAT_009a73b0;
extern unsigned short DAT_00a08294;
extern char DAT_009d1320[];
extern char s_disableldr1pln_0051a310[];

/* callees end with RET 0x0: caller cleans the stack, so __cdecl */
unsigned int __cdecl FUN_00403540(unsigned int a, unsigned int b);
unsigned int __cdecl FUN_00484f80(unsigned int a);
unsigned int __cdecl FUN_00404450(unsigned int a);

unsigned int __stdcall FUN_00467380(void)
{
    unsigned char cVar1;
    unsigned short uVar2;
    unsigned int iVar3;
    unsigned int uVar4;
    unsigned int iVar5;
    unsigned int uVar6;
    short sVar7;
    unsigned char *pbase;
    unsigned char *ptbl;
    unsigned char *pent;
    unsigned char *pobj;
    char *pcVar9;
    char *pcVar10;
    int bVar11;
    unsigned short t16;
    unsigned int t32;
    unsigned char t8;
    unsigned short tmpA;
    unsigned short tmpB;

    __builtin_memcpy(&uVar6, &DAT_009a73b6, 4);
    pbase = (unsigned char *)uVar6;
    __builtin_memcpy(&uVar2, pbase + 0x18, 2);
    t32 = DAT_00a08104;
    __builtin_memcpy(&tmpA, (unsigned char *)t32 + 0x36e, 2);
    if (tmpA == uVar2) {
        goto reset;
    }
    if (uVar2 == 0xffff) {
        iVar5 = 0;
        goto LAB_00467510;
    }
    iVar3 = (unsigned int)uVar2 * 0x23;
    ptbl = (unsigned char *)DAT_009a1fe8;
    pent = ptbl + iVar3;
    __builtin_memcpy(&iVar5, pent + 0x1f, 4);
    if (iVar5 == 0) {
        bVar11 = (pent[3] == 0);
    } else {
        if (pent[3] == 0) {
            iVar5 = 0;
            goto LAB_00467510;
        }
        __builtin_memcpy(&iVar5, (unsigned char *)iVar5 + 0xa9, 4);
        if (iVar5 == 0) {
            iVar5 = 1;
            goto LAB_00467510;
        }
        pobj = (unsigned char *)iVar5;
        if ((pobj[0x5e] == 0x15) && (pobj[0x5f] != 0)) {
            iVar5 = 0;
            goto LAB_00467510;
        }
        cVar1 = pobj[7];
        if ((cVar1 == 1) || ((cVar1 == 3) || (cVar1 == 4))) {
            iVar5 = 0;
            goto LAB_00467510;
        }
        if ((pobj[0x23d] == 3) && (pobj[0x241] != 0)) {
            FUN_00403540((unsigned int)uVar2, (unsigned int)DAT_009a73b0);
            cVar1 = *(unsigned char *)((unsigned int)DAT_009a73b0 * 0x23 + 2 + DAT_009a1fe8);
            if ((cVar1 == 0) || ((cVar1 == 1) || (cVar1 == 2))) {
                if (0x8000 < DAT_009a1ff8) {
                    iVar5 = 0;
                    goto LAB_00467510;
                }
            } else if (0x10000 < DAT_009a1ff8) {
                iVar5 = 0;
                goto LAB_00467510;
            }
        }
        __builtin_memcpy(&t32, (unsigned char *)(DAT_009a1fe8 + iVar3) + 0x1b, 4);
        if (t32 != 0xffffffffU) {
            iVar5 = 1;
            goto LAB_00467510;
        }
        {
            unsigned int a;
            unsigned int b;
            unsigned int c;
            unsigned int d;
            __builtin_memcpy(&a, (unsigned char *)(DAT_009a1fe8 + iVar3) + 0x1f, 4);
            __builtin_memcpy(&b, (unsigned char *)a + 0xa9, 4);
            __builtin_memcpy(&c, (unsigned char *)b + 0xcb, 4);
            __builtin_memcpy(&d, (unsigned char *)iVar5 + 0xc3, 4);
            bVar11 = (c < d);
        }
    }
    iVar5 = 1 - (unsigned int)bVar11;

LAB_00467510:
    __builtin_memcpy(&uVar6, &DAT_009a73b6, 4);
    if (iVar5 == 0) {
        goto reset;
    }
    uVar4 = (unsigned int)uVar2;
    pent = (unsigned char *)(uVar4 * 0x23 + DAT_009a1fe8);
    pbase = (unsigned char *)uVar6;
    __builtin_memcpy(&tmpA, pbase + 0x1a, 2);
    __builtin_memcpy(&tmpB, pent, 2);
    bVar11 = (tmpA == tmpB);
    if (!bVar11) {
        goto reset;
    }
    iVar5 = 0xf;
    pcVar9 = (char *)((int)&DAT_009d1320[0] + (unsigned int)*(unsigned char *)(pbase + 0xb) * 0x55);
    pcVar10 = s_disableldr1pln_0051a310;
    do {
        if (iVar5 == 0) break;
        iVar5 = iVar5 - 1;
        bVar11 = (*pcVar9 == *pcVar10);
        pcVar9 = pcVar9 + 1;
        pcVar10 = pcVar10 + 1;
    } while (bVar11);
    if (bVar11 && ((int)uVar4 < DAT_009a1fe0)) {
        unsigned int a;
        unsigned int b;
        unsigned short v;
        __builtin_memcpy(&a, pent + 0x1f, 4);
        __builtin_memcpy(&b, (unsigned char *)a + 0xa9, 4);
        __builtin_memcpy(&v, (unsigned char *)b + 0x1e5, 2);
        if (v == 0) {
            t16 = 0xffff;
            __builtin_memcpy(pbase + 0x2b, &t16, 2);
            return (uVar6 & 0xffff0000U) | 1;
        }
    }
    __builtin_memcpy(&uVar6, pent + 0x1b, 4);
    if ((uVar6 != 0xffffffffU) &&
        (uVar6 = (unsigned int)DAT_009a1fe0, (int)uVar4 < DAT_009a1fe0)) {
        uVar6 = FUN_00484f80((unsigned int)uVar2);
        if ((unsigned char)uVar6 == 1) {
            uVar6 = FUN_00403540((unsigned int)DAT_009a73b0, uVar4);
            if (0x4000 < DAT_009a1ff8) {
                goto reset;
            }
        }
    }
    if (DAT_00a08294 == 0xe) {
        unsigned char *e2;
        e2 = (unsigned char *)(DAT_009a1fe8 + uVar4 * 0x23);
        __builtin_memcpy(&sVar7, e2 + 0x17, 2);
        __builtin_memcpy(&iVar5, e2 + 0x1f, 4);
        uVar6 = 0;
        if (iVar5 != 0) {
            __builtin_memcpy(&uVar6, (unsigned char *)iVar5 + 0xa9, 4);
            if (uVar6 != 0) {
                __builtin_memcpy(&sVar7, (unsigned char *)uVar6 + 0x1e5, 2);
            }
        }
        if (sVar7 == 0) {
            unsigned short v2;
            __builtin_memcpy(&uVar6, &DAT_009a73b6, 4);
            __builtin_memcpy(&v2, (unsigned char *)uVar6 + 0x2b, 2);
            if (v2 != uVar2) {
                uVar6 = FUN_00404450((unsigned int)uVar2);
                if ((short)uVar6 == 0) {
                    goto reset;
                }
            }
        }
    }
    return uVar6 & 0xffff0000U;

reset:
    __builtin_memcpy(&uVar6, &DAT_009a73b6, 4);
    pbase = (unsigned char *)uVar6;
    t16 = 0xffff;
    __builtin_memcpy(pbase + 0x18, &t16, 2);
    t16 = 0;
    __builtin_memcpy(pbase + 0x1a, &t16, 2);
    t8 = 0;
    pbase[0x1e] = t8;
    __builtin_memcpy(&uVar6, &DAT_009a73b6, 4);
    t16 = 0xffff;
    __builtin_memcpy((unsigned char *)uVar6 + 0x2b, &t16, 2);
    return (uVar6 & 0xffff0000U) | 1;
}

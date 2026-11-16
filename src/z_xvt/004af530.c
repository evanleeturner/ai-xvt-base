/* SPDX-License-Identifier: MIT */
/* z_xvt 004af530 */
extern char DAT_0071b730[];
extern char DAT_0071bb37[];
extern int DAT_0071ecf4;
extern int DAT_006d4988;
extern unsigned short DAT_00655c48;
extern unsigned short DAT_00655c4a;
extern unsigned short DAT_00655c4c;
extern unsigned short DAT_00655c4e;
extern unsigned short DAT_0071f2de;
extern unsigned short DAT_00655f8e;
extern unsigned short DAT_00655f84;
extern unsigned short DAT_00655f86;
extern unsigned short DAT_00655f88;
extern unsigned short DAT_00655f8a;
extern unsigned short DAT_00655f8c;
extern unsigned short DAT_006d4a4e;
extern unsigned short DAT_00656cd4;
extern unsigned int DAT_0071f2f0[];
extern unsigned char *DAT_00656c44[];
extern void (__cdecl *DAT_0070a184)(unsigned int, unsigned int, unsigned int, unsigned int, unsigned int);
extern void (__cdecl *DAT_00703c80)(void);

void FUN_004a6f90(void);
unsigned int __cdecl FUN_0040a1a0(unsigned int param_1, unsigned int param_2, int param_3);
void FUN_004a6a80(void);
void FUN_0048ee20(char param_1);
void FUN_0048ed00(unsigned int a, unsigned int b, unsigned int c, unsigned int d);
void FUN_0048ece0(unsigned int a, unsigned int b);
void FUN_0048ed70(unsigned int a);
void FUN_0048eda0(unsigned int a);
/* UNRESOLVED: the callee also reads ECX/EDX (leftover register values from the
   previous call); they cannot be expressed in C. The callee returns with RET 0,
   so the caller cleans the stack (cdecl). */
void __cdecl FUN_0048f270(unsigned char *param_3);

void __stdcall FUN_004af530(void)
{
    char cVar1;
    int iVar2;
    _Bool bVar3;
    unsigned short uVar4;
    unsigned short uVar5;
    unsigned int uVar8;
    unsigned short s0, s1;
    unsigned short t0, t1;

    __builtin_memcpy(&iVar2, DAT_0071b730 + (unsigned int)DAT_0071ecf4 * 0x5bdu, 4);
    if (iVar2 == -1 ||
        (cVar1 = *(char *)((unsigned int)iVar2 * 0x23u + 3u + (unsigned int)DAT_006d4988),
         ((cVar1 != 1 && cVar1 != 2) && cVar1 != 3) && (cVar1 != 0x0e && cVar1 != 4))) {
        bVar3 = 0;
    } else {
        bVar3 = 1;
    }

    if (bVar3 && ((unsigned int)DAT_00655c48 + (unsigned int)DAT_00655c4a != 0)) {
        unsigned char *p;
        unsigned int q;
        int base;

        __builtin_memcpy(&q, (void *)((unsigned int)iVar2 * 0x23u + 0x1fu + (unsigned int)DAT_006d4988), 4);
        __builtin_memcpy(&base, (unsigned char *)q + 0xa7, 4);
        p = (unsigned char *)base;

        __builtin_memcpy(&s0, p + 199, 2);
        uVar5 = (unsigned short)(s0 / 3);
        uVar4 = 2;
        if (uVar5 != 0) {
            __builtin_memcpy(&s1, p + 0xc3, 2);
            uVar4 = (unsigned short)(s1 / uVar5);
        }

        __builtin_memcpy(&t0, p + 0x20b, 2);
        __builtin_memcpy(&t1, p + 0x20d, 2);
        if ((unsigned short)(t0 + t1) < 100 && uVar4 == 2) {
            uVar5 = (unsigned short)((DAT_0071f2de / 0x3b) & 1);
            if (uVar5 != 0) {
                FUN_004a6f90();
                FUN_0040a1a0(0x22, 0xffffffff, DAT_0071ecf4);
                FUN_004a6a80();
                DAT_00655f8e = (unsigned short)(DAT_00655f8e + 1);
                if (0x2ef < DAT_00655f8e) {
                    DAT_00655f8e = 1;
                }
            }
        } else {
            uVar5 = 0;
        }

        if (DAT_0071bb37[(unsigned int)DAT_0071ecf4 * 0x5bdu] == '\0') {
            if (DAT_00656cd4 != uVar5) {
                DAT_00656cd4 = uVar5;
                (*DAT_0070a184)(DAT_0071f2f0[(unsigned int)uVar5 + (unsigned int)DAT_00655c4c],
                                DAT_00655c48, DAT_00655c4a, DAT_00655c4e, 0);
            }
            if (DAT_00655f8e != 0) {
                FUN_0048ee20('\0');
                FUN_0048ed00(DAT_00655f84, DAT_00655f86,
                             (unsigned int)DAT_00655f84 + (unsigned int)DAT_00655f8c,
                             (unsigned int)DAT_006d4a4e + (unsigned int)DAT_00655f86);
                FUN_0048ece0(DAT_00655f84, DAT_00655f86);
                FUN_0048ed70((unsigned int)uVar5 + (unsigned int)DAT_00655f8a);
                uVar8 = 0;
                FUN_0048eda0((((unsigned int)(uVar5 == 0) - 1u) & 2u) + (unsigned int)DAT_00655f88);
                if (0x200 < DAT_00655f8e) {
                    uVar5 = (unsigned short)(DAT_00655f8e - 0x200);
                    (*DAT_00703c80)();
                    uVar8 = (unsigned int)(uVar5 >> 4);
                }
                FUN_0048f270(DAT_00656c44[uVar8]);
            }
        }
    }
    return;
}

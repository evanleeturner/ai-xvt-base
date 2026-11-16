/* SPDX-License-Identifier: MIT */
/* z_xvt 004d4dc0 */
typedef int (__stdcall *PFN_0x2c)(int *, int *, int);
typedef int (__stdcall *PFN_0x14)(int *, unsigned int *, int *, unsigned int *, unsigned int, unsigned int *);
typedef int (__stdcall *PFN_0x64)(int *, int, unsigned int *, int, int);
typedef int (__stdcall *PFN_0x6c)(int *);
typedef int (__stdcall *PFN_0x80)(int *, int);
typedef int (__stdcall *PFN_0x1c)(int *, int, int, int *, unsigned int *, int);

/* Unaligned 4-byte globals: only ever accessed through __builtin_memcpy. */
extern int DAT_00759663;
extern int DAT_00759653;
extern int DAT_00759657;
extern int DAT_00759667;
extern int DAT_0075966b;
extern int DAT_0075966f;
extern int DAT_007596c7;
extern int DAT_007596cb;
extern char DAT_00759645;
extern char DAT_00759646;
extern char DAT_0075965c;
extern char DAT_00759ad3;
extern int DAT_0081b6e4;
extern int DAT_0081b6f0;

/* FINDING (unresolved): Ghidra passes extraout_ECX / extraout_EDX (register
   leftovers from FUN_0051a4c0 / FUN_004d4d90) into FUN_004df5d0. C cannot name
   those values, so they are modelled as 0. This is a known fidelity gap.
   FUN_004dcfd0 ends with RET 0x0 (stack-check), so it is __cdecl. */
extern void __stdcall FUN_0051a4c0(void);
extern void __stdcall FUN_004d4d90(void);
extern void __stdcall FUN_0051a4e0(void);
extern void __stdcall FUN_004d50a0(void);
extern void __stdcall FUN_004dbd90(void);
extern int  __stdcall FUN_004d3c60(void);
extern void __fastcall FUN_004df5d0(int param_1, unsigned int param_2);
extern void __cdecl FUN_004dcfd0(unsigned int *param_1, int param_2, int param_3, int param_4, int param_5);

void __stdcall FUN_004d4dc0(void)
{
    int iVar1;
    unsigned int uVar2;
    unsigned int *puVar3;
    unsigned int local_f0[4];          /* rect: local_f0, local_ec, local_e8, local_e4 */
    unsigned int local_e0[4];
    unsigned int auStack_d0[27];       /* [9] overlaps puStack_ac (lpSurface) */
    unsigned int *puStack_ac;
    unsigned int local_64[25];
    int iTmp;
    int iZero;
    int *pObj_5966b;
    int *pObj_5966f;
    int *pObj_59667;
    unsigned int uCount;

    __builtin_memcpy(&iTmp, &DAT_00759663, 4);
    if (iTmp != 0) {
        FUN_0051a4c0();
        iVar1 = 0;
        uVar2 = 0;
        if (DAT_00759645 != '\0') {
            FUN_004d4d90();
        }
        local_f0[0] = 0;
        local_f0[1] = 0;
        local_f0[2] = 0x280;
        local_f0[3] = 0x1e0;
        if ((DAT_0081b6e4 == 0) && (DAT_0081b6f0 == 0)) {
            __builtin_memcpy(&iTmp, &DAT_00759657, 4);
            if ((DAT_00759ad3 == '\x01') && (iTmp == 8)) {
                FUN_004df5d0(iVar1, uVar2);
                DAT_00759ad3 = '\0';
            }
            do {
                while (1) {
                    __builtin_memcpy(&pObj_5966b, &DAT_0075966b, 4);
                    __builtin_memcpy(&pObj_5966f, &DAT_0075966f, 4);
                    iVar1 = (*(PFN_0x2c *)(*pObj_5966b + 0x2c))(pObj_5966b, pObj_5966f, 1);
                    if (iVar1 == 0) goto LAB_004d4e68;
                    if (iVar1 != -0x7789fe3e) break;
                    iVar1 = FUN_004d3c60();
                    if (iVar1 != 0) goto LAB_004d4e68;
                }
            } while (iVar1 == -0x7789fde4);
        }
        else {
            puVar3 = local_64;
            for (iVar1 = 0x19; iVar1 != 0; iVar1 = iVar1 - 1) {
                *puVar3 = 0;
                puVar3 = puVar3 + 1;
            }
            local_64[0] = 100;
            local_64[2] = 0xcc0020;
            FUN_004dcfd0(local_e0, 0, 0, 0x280, 0x1e0);
            for (;;) {
                __builtin_memcpy(&pObj_5966b, &DAT_0075966b, 4);
                __builtin_memcpy(&pObj_5966f, &DAT_0075966f, 4);
                iVar1 = (*(PFN_0x14 *)(*pObj_5966b + 0x14))(pObj_5966b, local_e0, pObj_5966f, local_e0, 0x1020000, local_64);
                if (iVar1 == 0) break;
                if (iVar1 == -0x7789fe3e) {
                    iVar1 = FUN_004d3c60();
                    if (iVar1 != 0) {
                        return;
                    }
                }
                else if (iVar1 != -0x7789fde4) {
                    return;
                }
            }
        }
LAB_004d4e68:
        if (DAT_0075965c != '\0') {
            __builtin_memcpy(&iTmp, &DAT_007596cb, 4);
            if (iTmp != 0) {
                iZero = 0;
                __builtin_memcpy(&DAT_007596cb, &iZero, 4);
                __builtin_memcpy(&iTmp, &DAT_007596c7, 4);
                if (iTmp != 0) {
                    __builtin_memcpy(&pObj_59667, &DAT_00759667, 4);
                    if (pObj_59667 != (int *)0) {
                        puVar3 = auStack_d0;
                        for (iVar1 = 0x1b; iVar1 != 0; iVar1 = iVar1 - 1) {
                            *puVar3 = 0;
                            puVar3 = puVar3 + 1;
                        }
                        auStack_d0[0] = 0x6c;
                        do {
                            while (1) {
                                __builtin_memcpy(&pObj_59667, &DAT_00759667, 4);
                                iVar1 = (*(PFN_0x64 *)(*pObj_59667 + 100))(pObj_59667, 0, auStack_d0, 0, 0);
                                if (iVar1 == 0) goto LAB_004d4fa8;
                                if (iVar1 != -0x7789fe3e) break;
                                __builtin_memcpy(&pObj_59667, &DAT_00759667, 4);
                                (*(PFN_0x6c *)(*pObj_59667 + 0x6c))(pObj_59667);
                            }
                        } while (iVar1 == -0x7789fde4);
LAB_004d4fa8:
                        puStack_ac = (unsigned int *)auStack_d0[9];
                        if (puStack_ac != (unsigned int *)0) {
                            __builtin_memcpy(&puVar3, &DAT_007596c7, 4);
                            __builtin_memcpy(&uCount, &DAT_00759653, 4);
                            for (iVar1 = (int)(((uCount * 0xfU) & 0x7ffffffU) << 3); iVar1 != 0; iVar1 = iVar1 - 1) {
                                *puStack_ac = *puVar3;
                                puVar3 = puVar3 + 1;
                                puStack_ac = puStack_ac + 1;
                            }
                            __builtin_memcpy(&pObj_59667, &DAT_00759667, 4);
                            (*(PFN_0x80 *)(*pObj_59667 + 0x80))(pObj_59667, 0);
                        }
                    }
                }
            }
            if ((DAT_0081b6e4 == 0) && (DAT_0081b6f0 == 0)) {
                for (;;) {
                    __builtin_memcpy(&pObj_5966f, &DAT_0075966f, 4);
                    __builtin_memcpy(&pObj_59667, &DAT_00759667, 4);
                    iVar1 = (*(PFN_0x1c *)(*pObj_5966f + 0x1c))(pObj_5966f, 0, 0, pObj_59667, local_f0, 0);
                    if (iVar1 == 0) break;
                    if (iVar1 == -0x7789fe3e) {
                        iVar1 = FUN_004d3c60();
                        if (iVar1 != 0) {
                            FUN_0051a4e0();
                            return;
                        }
                    }
                    else if (iVar1 != -0x7789fde4) {
                        FUN_0051a4e0();
                        return;
                    }
                }
            }
            else {
                FUN_004dbd90();
            }
        }
        if (DAT_00759646 != '\0') {
            FUN_004d50a0();
        }
        FUN_0051a4e0();
    }
    return;
}

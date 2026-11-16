/* SPDX-License-Identifier: MIT */
/* z_bop 004abee0 */
extern int *DAT_0066ddd4;
extern int *DAT_0066e700;
extern int *DAT_00803f80;
extern int *DAT_00a68748;
extern int DAT_0066ddd0;
extern int DAT_00527eac;
extern int DAT_00527ea8;
extern int DAT_00527ea4;
extern DWORD DAT_00527f7c;
extern int DAT_00527f80;
extern int DAT_00622cb0;
extern DWORD DAT_00803b70;
extern DWORD DAT_00803f90;

extern DWORD __stdcall timeGetTime(void);
void __stdcall RaiseException(DWORD, DWORD, DWORD, const DWORD *);

void __cdecl FUN_004ab970(int a);
int  __stdcall FUN_004aafe0(void);
int  __stdcall FUN_004aba10(void);
BOOL __stdcall FUN_004ac460(void);

#define VT(o, off) (*(void **)(*(unsigned char **)(o) + (off)))

void __stdcall FUN_004abee0(void)
{
    int iVar3;
    int iVar4;
    DWORD DVar5;
    DWORD *puVar6;
    DWORD *puVar7;
    DWORD auStack_78[27];
    int iStack_c;
    int iStack_8;
#define uStack_68  auStack_78[4]
#define puStack_54 ((DWORD *)auStack_78[9])
#define uStack_10  auStack_78[26]

    if (DAT_00527eac == 0) {
        if (DAT_00527ea8 == 0) {
            puVar6 = auStack_78;
            for (iVar3 = 0x1b; iVar3 != 0; iVar3 = iVar3 - 1) {
                *puVar6 = 0;
                puVar6 = puVar6 + 1;
            }
            auStack_78[0] = 0x6c;
            auStack_78[1] = 1;
            uStack_10 = 0x200;
            iVar3 = ((int (__stdcall *)(int *, DWORD *, int **, int))VT(DAT_0066ddd4, 0x18))
                        (DAT_0066ddd4, auStack_78, &DAT_0066e700, 0);
            if (iVar3 != 0) {
                FUN_004ab970(4);
                return;
            }
            iVar3 = FUN_004aafe0();
            if (iVar3 == 0) {
                FUN_004ab970(0xc);
                return;
            }
        }
        puVar6 = auStack_78;
        for (iVar3 = 0x1b; iVar3 != 0; iVar3 = iVar3 - 1) {
            *puVar6 = 0;
            puVar6 = puVar6 + 1;
        }
        auStack_78[0] = 0x6c;
        ((int (__stdcall *)(int *, DWORD *))VT(DAT_0066e700, 0x58))(DAT_0066e700, auStack_78);
        DAT_00803b70 = uStack_68;
        puVar6 = auStack_78;
        for (iVar3 = 0x1b; iVar3 != 0; iVar3 = iVar3 - 1) {
            *puVar6 = 0;
            puVar6 = puVar6 + 1;
        }
        auStack_78[0] = 0x6c;
        while (iVar3 = ((int (__stdcall *)(int *, int, DWORD *, int, int))VT(DAT_0066e700, 100))
                           (DAT_0066e700, 0, auStack_78, 0, 0), iVar3 != 0) {
            if (iVar3 != -0x7789fde4) {
                return;
            }
        }
        iVar3 = FUN_004aba10();
        puVar6 = &DAT_00803f90;
        puVar7 = puStack_54;
        for (iVar3 = (int)((((DWORD)iVar3 * 0xfU) & 0x7ffffffU) << 3); iVar3 != 0; iVar3 = iVar3 - 1) {
            *puVar7 = *puVar6;
            puVar6 = puVar6 + 1;
            puVar7 = puVar7 + 1;
        }
        ((int (__stdcall *)(int *, DWORD *))VT(DAT_0066e700, 0x80))(DAT_0066e700, puStack_54);
        if ((DAT_00527ea8 == 0) && (DAT_0066e700 != (int *)0)) {
            ((int (__stdcall *)(int *))VT(DAT_0066e700, 8))(DAT_0066e700);
            DAT_0066e700 = (int *)0;
        }
    }
    else {
        if (DAT_00527ea4 != 0) {
            if (DAT_00527f7c == 0) {
                iVar3 = ((int (__stdcall *)(int *, int *))VT(DAT_0066ddd4, 0x44))(DAT_0066ddd4, &iStack_8);
                if (iVar3 == 0) {
                    do {
                        if (iStack_8 == 0) break;
                        iVar3 = ((int (__stdcall *)(int *, int *))VT(DAT_0066ddd4, 0x44))(DAT_0066ddd4, &iStack_8);
                    } while (iVar3 == 0);
                    do {
                        if (iStack_8 != 0) break;
                        iVar3 = ((int (__stdcall *)(int *, int *))VT(DAT_0066ddd4, 0x44))(DAT_0066ddd4, &iStack_8);
                    } while (iVar3 == 0);
                }
                DAT_00527f7c = timeGetTime();
                iVar3 = ((int (__stdcall *)(int *, int *))VT(DAT_0066ddd4, 0x3c))(DAT_0066ddd4, &DAT_00622cb0);
                if (iVar3 != 0) {
                    for (iVar3 = 0; iVar3 < 100; iVar3 = iVar3 + 1) {
                        do {
                            if (iStack_8 == 0) break;
                            iVar4 = ((int (__stdcall *)(int *, int *))VT(DAT_0066ddd4, 0x44))(DAT_0066ddd4, &iStack_8);
                        } while (iVar4 == 0);
                        do {
                            if (iStack_8 != 0) break;
                            iVar4 = ((int (__stdcall *)(int *, int *))VT(DAT_0066ddd4, 0x44))(DAT_0066ddd4, &iStack_8);
                        } while (iVar4 == 0);
                    }
                    DVar5 = timeGetTime();
                    {
                        int divisor = (int)(DVar5 - DAT_00527f7c);
                        if (divisor == 0) {
                            RaiseException(0xC0000094UL, 0, 0, NULL);
                        } else {
                            DAT_00622cb0 = 10000000 / divisor;
                        }
                    }
                }
            }
            else {
                DVar5 = timeGetTime();
                iVar3 = (int)((DWORD)DAT_00622cb0 * (DVar5 - DAT_00527f7c)) % 100000;
                if ((((iVar3 < DAT_00527f80) || ((int)(100000U - (DWORD)DAT_00527f80) < iVar3)) &&
                     (iVar3 = ((int (__stdcall *)(int *, int *))VT(DAT_0066ddd4, 0x44))(DAT_0066ddd4, &iStack_8), iVar3 == 0)) &&
                    (iStack_8 == 0)) {
                    do {
                        if (iStack_8 != 0) break;
                        iVar3 = ((int (__stdcall *)(int *, int *))VT(DAT_0066ddd4, 0x44))(DAT_0066ddd4, &iStack_8);
                    } while (iVar3 == 0);
                    DAT_00527f7c = timeGetTime();
                }
            }
        }
        iStack_c = ((int (__stdcall *)(int *, int, int))VT(DAT_0066e700, 0x2c))(DAT_0066e700, 0, 1);
        if (iStack_c == -0x7789ff1f) {
            iVar3 = ((int (__stdcall *)(int *, int, int))VT(DAT_0066ddd4, 0x50))(DAT_0066ddd4, DAT_0066ddd0, 0x51);
            if (iVar3 != 0) {
                __asm__ __volatile__("int3");
                return;
            }
            iVar3 = ((int (__stdcall *)(int *, int, int))VT(DAT_0066e700, 0x2c))(DAT_0066e700, 0, 1);
            if (iVar3 == -0x7789fe3e) {
                ((int (__stdcall *)(int *))VT(DAT_0066e700, 0x6c))(DAT_0066e700);
                ((int (__stdcall *)(int *))VT(DAT_00803f80, 0x6c))(DAT_00803f80);
                ((int (__stdcall *)(int *))VT(DAT_00a68748, 0x6c))(DAT_00a68748);
                iVar3 = ((int (__stdcall *)(int *, int, int))VT(DAT_0066e700, 0x2c))(DAT_0066e700, 0, 1);
            }
            if (iVar3 != 0) {
                __asm__ __volatile__("int3");
                return;
            }
            iStack_c = ((int (__stdcall *)(int *, int, int))VT(DAT_0066ddd4, 0x50))(DAT_0066ddd4, DAT_0066ddd0, 8);
            if (iStack_c != 0) {
                __asm__ __volatile__("int3");
                return;
            }
        }
        if ((iStack_c == -0x7789fe3e) && (FUN_004ac460() != 0)) {
            FUN_004aafe0();
            return;
        }
    }
    return;
#undef uStack_68
#undef puStack_54
#undef uStack_10
}

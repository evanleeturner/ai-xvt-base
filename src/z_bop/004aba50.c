/* SPDX-License-Identifier: MIT */
/* z_bop 004aba50 */
/* Unresolved: callee signatures are inferred from the decompile only; assumed __cdecl
   (the stack-check gate passed with these). */
unsigned int *__cdecl FUN_004d5410(void);
int __cdecl FUN_004d5070(void);
void __cdecl FUN_004ac790(unsigned int *param_1);
int __cdecl FUN_004aba10(void);
void __cdecl FUN_004ac780(int param_1);
void __cdecl FUN_0040e0c0(int param_1, int param_2, int param_3, int param_4);

extern int DAT_00527ea0;
extern unsigned int *DAT_005233f4;
extern unsigned int *DAT_00527ecc;
extern int DAT_009d8c08;
extern int DAT_00527f78;
extern int DAT_00527eac;
extern int DAT_00527ed0;
extern unsigned int DAT_0066e710;
extern unsigned int DAT_00803f90;
/* 2-byte value at odd address 0x009ed23b: read via memcpy */
extern unsigned char _DAT_009ed23b[2];
extern int *DAT_0066e700;
extern int *DAT_00803f80;
extern int *DAT_0066ddcc;
extern int DAT_00803b70;
extern int DAT_0066ddc4;
extern int DAT_005233e0;
extern int DAT_005233d8;
extern int DAT_0066dde0;
extern int DAT_005233e8;

typedef int (__stdcall *FUN_vtbl_call)(int *, int, int *, int, int);
typedef int (__stdcall *FUN_vtbl_call2)(int *, int *);

void __stdcall FUN_004aba50(void)
{
    int iVar1;
    int *piVar2;
    int local_6c[27];
    unsigned short v_9ed23b;
    FUN_vtbl_call pfnCreate;
    FUN_vtbl_call2 pfnDesc;

    if (DAT_00527ea0 == 1) {
        DAT_005233f4 = FUN_004d5410();
        DAT_00527ecc = DAT_005233f4;
        DAT_009d8c08 = FUN_004d5070();
        return;
    }
    if (DAT_00527f78 != 0) {
        DAT_00527f78 = (int)((unsigned int)DAT_00527f78 + 1U);
        return;
    }
    DAT_00527f78 = 1;
    if (DAT_00527eac == 0) {
        if (DAT_00527ed0 != 0) {
            FUN_004ac790(&DAT_0066e710);
            iVar1 = FUN_004aba10();
            FUN_004ac780((int)((unsigned int)iVar1 * 0x1e0U));
            DAT_005233f4 = &DAT_0066e710;
            DAT_00527ecc = &DAT_0066e710;
            return;
        }
        FUN_004ac790(&DAT_00803f90);
        iVar1 = FUN_004aba10();
        FUN_004ac780((int)((unsigned int)iVar1 * 0x1e0U));
        DAT_005233f4 = &DAT_00803f90;
        DAT_00527ecc = &DAT_00803f90;
    }
    else {
        __builtin_memcpy(&v_9ed23b, _DAT_009ed23b, 2);
        if (v_9ed23b == 0) {
            piVar2 = local_6c;
            for (iVar1 = 0x1b; iVar1 != 0; iVar1 = iVar1 + -1) {
                *piVar2 = 0;
                piVar2 = piVar2 + 1;
            }
            local_6c[0] = 0x6c;
            do {
                pfnCreate = (FUN_vtbl_call)*(int *)(*DAT_0066e700 + 100);
                iVar1 = pfnCreate(DAT_0066e700, 0, local_6c, 0, 0);
                if (iVar1 == 0) {
                    FUN_004ac790((unsigned int *)local_6c[9]);
                    DAT_005233f4 = (unsigned int *)local_6c[9];
                    DAT_00527ecc = (unsigned int *)local_6c[9];
                    piVar2 = local_6c;
                    for (iVar1 = 0x1b; iVar1 != 0; iVar1 = iVar1 + -1) {
                        *piVar2 = 0;
                        piVar2 = piVar2 + 1;
                    }
                    local_6c[0] = 0x6c;
                    pfnDesc = (FUN_vtbl_call2)*(int *)(*DAT_0066e700 + 0x58);
                    pfnDesc(DAT_0066e700, local_6c);
                    DAT_00803b70 = local_6c[2];
                    iVar1 = (int)((((unsigned int)DAT_0066ddc4 - (unsigned int)DAT_005233e0) >> 1) * (unsigned int)DAT_005233d8);
                    local_6c[2] = (int)((((unsigned int)DAT_0066dde0 - (unsigned int)DAT_005233e8) >> 1) * (unsigned int)local_6c[2]);
                    DAT_005233f4 = (unsigned int *)((unsigned int)(int)DAT_005233f4 + (unsigned int)local_6c[2] + (unsigned int)iVar1);
                    DAT_00527ecc = (unsigned int *)((unsigned int)(int)DAT_00527ecc + (unsigned int)local_6c[2] + (unsigned int)iVar1);
                    iVar1 = FUN_004aba10();
                    FUN_004ac780((int)((unsigned int)iVar1 * 0x1e0U));
                    return;
                }
            } while (iVar1 == -0x7789fde4);
            return;
        }
        if (DAT_00527ed0 == 0) {
            piVar2 = local_6c;
            for (iVar1 = 0x1b; iVar1 != 0; iVar1 = iVar1 + -1) {
                *piVar2 = 0;
                piVar2 = piVar2 + 1;
            }
            local_6c[0] = 0x6c;
            while (1) {
                pfnCreate = (FUN_vtbl_call)*(int *)(*DAT_00803f80 + 100);
                iVar1 = pfnCreate(DAT_00803f80, 0, local_6c, 0, 0);
                if (iVar1 == 0) break;
                if (iVar1 != -0x7789fde4) {
                    return;
                }
            }
            FUN_004ac790((unsigned int *)local_6c[9]);
            DAT_005233f4 = (unsigned int *)local_6c[9];
            DAT_00527ecc = (unsigned int *)local_6c[9];
            piVar2 = local_6c;
            for (iVar1 = 0x1b; iVar1 != 0; iVar1 = iVar1 + -1) {
                *piVar2 = 0;
                piVar2 = piVar2 + 1;
            }
            local_6c[0] = 0x6c;
            pfnDesc = (FUN_vtbl_call2)*(int *)(*DAT_00803f80 + 0x58);
            pfnDesc(DAT_00803f80, local_6c);
            DAT_00803b70 = local_6c[2];
            iVar1 = (int)((((unsigned int)DAT_0066ddc4 - (unsigned int)DAT_005233e0) >> 1) * (unsigned int)DAT_005233d8);
            local_6c[2] = (int)((((unsigned int)DAT_0066dde0 - (unsigned int)DAT_005233e8) >> 1) * (unsigned int)local_6c[2]);
            DAT_005233f4 = (unsigned int *)((unsigned int)(int)DAT_005233f4 + (unsigned int)local_6c[2] + (unsigned int)iVar1);
            DAT_00527ecc = (unsigned int *)((unsigned int)(int)DAT_00527ecc + (unsigned int)local_6c[2] + (unsigned int)iVar1);
            iVar1 = FUN_004aba10();
            FUN_004ac780((int)((unsigned int)iVar1 * 0x1e0U));
            if (DAT_009d8c08 != DAT_00803b70) {
                DAT_009d8c08 = DAT_00803b70;
                FUN_0040e0c0((int)DAT_005233f4, DAT_00803b70, 0x1e0, -1);
                return;
            }
        }
        else {
            piVar2 = local_6c;
            for (iVar1 = 0x1b; iVar1 != 0; iVar1 = iVar1 + -1) {
                *piVar2 = 0;
                piVar2 = piVar2 + 1;
            }
            local_6c[0] = 0x6c;
            while (1) {
                pfnCreate = (FUN_vtbl_call)*(int *)(*DAT_0066ddcc + 100);
                iVar1 = pfnCreate(DAT_0066ddcc, 0, local_6c, 0, 0);
                if (iVar1 == 0) break;
                if (iVar1 != -0x7789fde4) {
                    return;
                }
            }
            FUN_004ac790((unsigned int *)local_6c[9]);
            DAT_005233f4 = (unsigned int *)local_6c[9];
            DAT_00527ecc = (unsigned int *)local_6c[9];
            piVar2 = local_6c;
            for (iVar1 = 0x1b; iVar1 != 0; iVar1 = iVar1 + -1) {
                *piVar2 = 0;
                piVar2 = piVar2 + 1;
            }
            local_6c[0] = 0x6c;
            pfnDesc = (FUN_vtbl_call2)*(int *)(*DAT_0066ddcc + 0x58);
            pfnDesc(DAT_0066ddcc, local_6c);
            DAT_00803b70 = local_6c[2];
            iVar1 = FUN_004aba10();
            FUN_004ac780((int)((unsigned int)iVar1 * 0x1e0U));
            if (DAT_009d8c08 != DAT_00803b70) {
                DAT_009d8c08 = DAT_00803b70;
                FUN_0040e0c0((int)DAT_005233f4, DAT_00803b70, 0x1e0, -1);
                return;
            }
        }
    }
    return;
}

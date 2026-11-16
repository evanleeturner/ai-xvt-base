/* SPDX-License-Identifier: MIT */
/* z_xvt 00483950 */
extern int DAT_0070ab18;
extern int DAT_0070ab14;
extern int DAT_0070ac20;

int __cdecl FUN_0044a780(int a, int b);
long long __cdecl __ftol(void);
long long __cdecl FUN_004187b0(short a);
long long __cdecl FUN_00418450(short a);
int __cdecl FUN_00413db0(int a, int b, int c, int d);
int __cdecl FUN_00413df0(int a, int b, int c, int d);
/* Ghidra reports __fastcall(ECX, EDX, a, b), but the callee reads only the two
   stack args and ends with RET 0x0: it is __cdecl with two stack arguments.
   The extraout_ECX/EDX values are decompiler artifacts. */
long long __cdecl FUN_00413ca0(int a, int b);
int __cdecl FUN_00413d00(int a, int b, int c, int d, int e, int f);

void __cdecl FUN_00483950(short param_1, int param_2, int param_3, int param_4, int param_5, int param_6)
{
    int iVar1;
    int iVar2;
    int iVar3;
    int iVar4;
    int iVar5;
    int iVar6;
    long long lVar7;
    long long lVar9;
    long long lVar10;
    long long uVar8;
    int local_1c;
    int local_18;
    int local_14;
    int local_10;
    int local_c;
    int local_8;
    int local_4;

    DAT_0070ab18 = param_4;
    DAT_0070ab14 = param_5;
    DAT_0070ac20 = param_6;
    iVar1 = FUN_0044a780(param_2, param_3);
    if (iVar1 != 0) {
        lVar7 = __ftol();
        iVar1 = (int)lVar7;
        lVar7 = __ftol();
        iVar4 = (int)lVar7;
        lVar7 = __ftol();
        iVar5 = (int)lVar7;
        lVar7 = FUN_004187b0(param_1);
        iVar2 = (int)(short)lVar7;
        lVar7 = FUN_00418450(param_1);
        iVar6 = (int)(short)lVar7;
        if (iVar2 < 0) {
            iVar3 = (int)(0u - (unsigned int)iVar2);
            local_14 = FUN_00413df0(iVar1, iVar1, iVar3, iVar2);
            uVar8 = FUN_00413ca0(iVar6, iVar5);
            local_c = FUN_00413df0(iVar1, iVar4, iVar3, (int)uVar8);
            uVar8 = FUN_00413ca0(iVar6, iVar4);
            local_4 = FUN_00413df0(iVar1, iVar5, iVar3, (int)(0u - (unsigned int)(int)uVar8));
            uVar8 = FUN_00413ca0(iVar6, iVar5);
            local_18 = FUN_00413df0(iVar1, iVar4, iVar3, (int)(0u - (unsigned int)(int)uVar8));
            local_10 = FUN_00413df0(iVar4, iVar4, iVar3, iVar2);
            uVar8 = FUN_00413ca0(iVar6, iVar1);
            local_8 = FUN_00413df0(iVar4, iVar5, iVar3, (int)uVar8);
            uVar8 = FUN_00413ca0(iVar6, iVar4);
            local_1c = FUN_00413df0(iVar1, iVar5, iVar3, (int)uVar8);
            uVar8 = FUN_00413ca0(iVar6, iVar1);
            iVar1 = FUN_00413df0(iVar4, iVar5, iVar3, (int)(0u - (unsigned int)(int)uVar8));
            iVar4 = FUN_00413df0(iVar5, iVar5, iVar3, iVar2);
        } else {
            iVar3 = (int)(0x7fffu - (unsigned int)iVar2);
            local_14 = FUN_00413db0(iVar1, iVar1, iVar3, iVar2);
            uVar8 = FUN_00413ca0(iVar6, iVar5);
            local_c = FUN_00413db0(iVar1, iVar4, iVar3, (int)uVar8);
            uVar8 = FUN_00413ca0(iVar6, iVar4);
            local_4 = FUN_00413db0(iVar1, iVar5, iVar3, (int)(0u - (unsigned int)(int)uVar8));
            uVar8 = FUN_00413ca0(iVar6, iVar5);
            local_18 = FUN_00413db0(iVar1, iVar4, iVar3, (int)(0u - (unsigned int)(int)uVar8));
            local_10 = FUN_00413db0(iVar4, iVar4, iVar3, iVar2);
            uVar8 = FUN_00413ca0(iVar6, iVar1);
            local_8 = FUN_00413db0(iVar4, iVar5, iVar3, (int)uVar8);
            uVar8 = FUN_00413ca0(iVar6, iVar4);
            local_1c = FUN_00413db0(iVar1, iVar5, iVar3, (int)uVar8);
            uVar8 = FUN_00413ca0(iVar6, iVar1);
            iVar1 = FUN_00413db0(iVar4, iVar5, iVar3, (int)(0u - (unsigned int)(int)uVar8));
            iVar4 = FUN_00413db0(iVar5, iVar5, iVar3, iVar2);
        }
        lVar7 = __ftol();
        iVar2 = (int)((unsigned int)param_4 - (unsigned int)(int)lVar7);
        lVar7 = __ftol();
        iVar6 = (int)((unsigned int)param_5 + (unsigned int)(int)lVar7);
        lVar7 = __ftol();
        iVar3 = (int)((unsigned int)param_6 - (unsigned int)(int)lVar7);
        iVar5 = FUN_00413d00(iVar2, iVar6, iVar3, local_14, local_18, local_1c);
        iVar1 = FUN_00413d00(iVar2, iVar6, iVar3, local_c, local_10, iVar1);
        iVar4 = FUN_00413d00(iVar2, iVar6, iVar3, local_4, local_8, iVar4);
        lVar7 = __ftol();
        lVar9 = __ftol();
        lVar10 = __ftol();
        DAT_0070ab18 = (int)((unsigned int)(int)lVar10 + (unsigned int)iVar5);
        DAT_0070ab14 = (int)((unsigned int)iVar1 - (unsigned int)(int)lVar7);
        DAT_0070ac20 = (int)((unsigned int)iVar4 + (unsigned int)(int)lVar9);
    }
    return;
}

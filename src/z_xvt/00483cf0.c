/* SPDX-License-Identifier: MIT */
/* z_xvt 00483cf0 */
long long __cdecl FUN_004187b0(short param_1);
long long __cdecl FUN_00418450(short param_1);
int __cdecl FUN_00413db0(int a, int b, int c, int d);
int __cdecl FUN_00413df0(int a, int b, int c, int d);
int __cdecl FUN_00413d00(int a, int b, int c, int d, int e, int f);
long long __cdecl FUN_00413ca0(int a, int b);

extern int DAT_00732540;
extern int DAT_00732544;
extern int DAT_00732550;
extern int DAT_00732560;
extern int DAT_00732564;
extern int DAT_00732580;
extern int DAT_007329c4;
extern int DAT_007329c8;
extern int DAT_00732a14;

void __cdecl FUN_00483cf0(int param_1, int param_2, int param_3, short param_4)
{
    int iVar1;
    int iVar2;
    int iVar3;
    int iVar4;
    long long lVar5;
    long long lVar6;
    long long uVar7;
    int local_1c;
    int local_18;
    int local_14;
    int local_10;
    int local_c;
    int local_8;
    int local_4;

    if (param_4 != 0) {
        lVar5 = FUN_004187b0(param_4);
        lVar6 = FUN_00418450(param_4);
        iVar1 = (int)(short)lVar5;
        if ((short)lVar5 < 0) {
            iVar4 = (int)(0u - (unsigned int)iVar1);
            local_14 = FUN_00413df0(param_1, param_1, iVar4, iVar1);
            iVar3 = (int)(short)lVar6;
            uVar7 = FUN_00413ca0(iVar3, param_3);
            local_c = FUN_00413df0(param_1, param_2, iVar4, (int)uVar7);
            uVar7 = FUN_00413ca0(iVar3, param_2);
            local_4 = FUN_00413df0(param_1, param_3, iVar4, (int)(0u - (unsigned int)(int)uVar7));
            uVar7 = FUN_00413ca0(iVar3, param_3);
            local_18 = FUN_00413df0(param_1, param_2, iVar4, (int)(0u - (unsigned int)(int)uVar7));
            local_10 = FUN_00413df0(param_2, param_2, iVar4, iVar1);
            uVar7 = FUN_00413ca0(iVar3, param_1);
            local_8 = FUN_00413df0(param_2, param_3, iVar4, (int)uVar7);
            uVar7 = FUN_00413ca0(iVar3, param_2);
            local_1c = FUN_00413df0(param_1, param_3, iVar4, (int)uVar7);
            uVar7 = FUN_00413ca0(iVar3, param_1);
            iVar3 = FUN_00413df0(param_2, param_3, iVar4, (int)(0u - (unsigned int)(int)uVar7));
            iVar1 = FUN_00413df0(param_3, param_3, iVar4, iVar1);
        } else {
            iVar4 = (int)(0x7fffu - (unsigned int)iVar1);
            local_14 = FUN_00413db0(param_1, param_1, iVar4, iVar1);
            iVar3 = (int)(short)lVar6;
            uVar7 = FUN_00413ca0(iVar3, param_3);
            local_c = FUN_00413db0(param_1, param_2, iVar4, (int)uVar7);
            uVar7 = FUN_00413ca0(iVar3, param_2);
            local_4 = FUN_00413db0(param_1, param_3, iVar4, (int)(0u - (unsigned int)(int)uVar7));
            uVar7 = FUN_00413ca0(iVar3, param_3);
            local_18 = FUN_00413db0(param_1, param_2, iVar4, (int)(0u - (unsigned int)(int)uVar7));
            local_10 = FUN_00413db0(param_2, param_2, iVar4, iVar1);
            uVar7 = FUN_00413ca0(iVar3, param_1);
            local_8 = FUN_00413db0(param_2, param_3, iVar4, (int)uVar7);
            uVar7 = FUN_00413ca0(iVar3, param_2);
            local_1c = FUN_00413db0(param_1, param_3, iVar4, (int)uVar7);
            uVar7 = FUN_00413ca0(iVar3, param_1);
            iVar3 = FUN_00413db0(param_2, param_3, iVar4, (int)(0u - (unsigned int)(int)uVar7));
            iVar1 = FUN_00413db0(param_3, param_3, iVar4, iVar1);
        }
        iVar4 = FUN_00413d00(DAT_00732550, DAT_00732540, DAT_00732544, local_14, local_18, local_1c);
        iVar2 = FUN_00413d00(DAT_00732550, DAT_00732540, DAT_00732544, local_c, local_10, iVar3);
        DAT_00732544 = FUN_00413d00(DAT_00732550, DAT_00732540, DAT_00732544, local_4, local_8, iVar1);
        DAT_00732540 = iVar2;
        DAT_00732550 = iVar4;
        iVar4 = FUN_00413d00(DAT_00732580, DAT_00732560, DAT_00732564, local_14, local_18, local_1c);
        iVar2 = FUN_00413d00(DAT_00732580, DAT_00732560, DAT_00732564, local_c, local_10, iVar3);
        DAT_00732564 = FUN_00413d00(DAT_00732580, DAT_00732560, DAT_00732564, local_4, local_8, iVar1);
        DAT_00732560 = iVar2;
        DAT_00732580 = iVar4;
        iVar4 = FUN_00413d00(DAT_007329c4, DAT_007329c8, DAT_00732a14, local_14, local_18, local_1c);
        iVar3 = FUN_00413d00(DAT_007329c4, DAT_007329c8, DAT_00732a14, local_c, local_10, iVar3);
        DAT_00732a14 = FUN_00413d00(DAT_007329c4, DAT_007329c8, DAT_00732a14, local_4, local_8, iVar1);
        DAT_007329c4 = iVar4;
        DAT_007329c8 = iVar3;
    }
    return;
}

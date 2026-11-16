/* SPDX-License-Identifier: MIT */
/* z_xvt 0040fb70 */
extern int DAT_006d4974;
extern int DAT_0071e6c4;
extern int DAT_006d4978;
extern int DAT_00732f38;
extern int DAT_0073294c;
extern int DAT_0070a180;
extern int DAT_006d4970;
extern int DAT_00732fe8;
extern int DAT_007329b0;
extern int DAT_0070acd8;
extern int DAT_007329b4;
extern int DAT_00703ad8;
extern int DAT_006ffbe0;
extern int DAT_007091a8;
extern int DAT_0070a17c;

void __stdcall RaiseException(DWORD, DWORD, DWORD, const DWORD *);
/* Callee ends with RET 0 (caller cleans); reads only its two stack arguments. */
int __cdecl FUN_00413ca0(int param_3, int param_4);

unsigned int __cdecl FUN_0040fb70(int param_1)
{
    int iVar1;
    int iVar2;
    int iVar3;
    int iVar4;
    int iVar5;
    int iVar6;
    unsigned short uVar8;
    unsigned int uVar7;
    int iVar9;
    int iVar10;
    unsigned int uVar11;
    int iVar12;
    unsigned int uVar13;
    int iVar14;
    int iVar15;
    int uVar16;
    int local_28;
    unsigned int local_24;
    int local_20;
    int local_1c;
    int local_18;
    int divN;
    int divD;

    iVar1 = (int)((unsigned int)DAT_006d4974 - (unsigned int)DAT_0071e6c4);
    iVar2 = (int)((unsigned int)DAT_006d4978 - (unsigned int)DAT_00732f38);
    iVar12 = (int)((unsigned int)DAT_0073294c - (unsigned int)DAT_0070a180);
    iVar3 = (int)((unsigned int)DAT_006d4970 - (unsigned int)DAT_00732fe8);
    iVar14 = (int)((unsigned int)DAT_0070a180 - (unsigned int)DAT_0071e6c4);
    iVar4 = (int)((unsigned int)DAT_007329b0 - (unsigned int)DAT_0070acd8);
    iVar5 = (int)((unsigned int)DAT_007329b4 - (unsigned int)DAT_00703ad8);
    if (param_1 < iVar14) {
        iVar6 = (int)((unsigned int)param_1 - (unsigned int)iVar14);
        local_24 = (unsigned int)iVar12 - (unsigned int)iVar1;
        uVar8 = (unsigned short)((unsigned int)iVar6 >> 0x10);
        if (-1 < (int)local_24) {
            return (unsigned int)uVar8 << 0x10;
        }
        if (iVar6 < (int)local_24) {
            return (unsigned int)uVar8 << 0x10;
        }
        iVar10 = (int)(0u - ((unsigned int)param_1 + (unsigned int)iVar14));
    }
    else if ((int)(0u - (unsigned int)iVar14) == param_1 || (int)(0u - (unsigned int)param_1) < iVar14) {
        iVar10 = (int)(0u - ((unsigned int)param_1 + (unsigned int)iVar14));
        local_24 = 0;
        iVar6 = (int)((unsigned int)param_1 - (unsigned int)iVar14);
    }
    else {
        iVar10 = (int)(0u - ((unsigned int)param_1 + (unsigned int)iVar14));
        local_24 = (unsigned int)iVar12 - (unsigned int)iVar1;
        if ((int)local_24 < 0) {
            return local_24 & 0xffff0000;
        }
        if ((int)local_24 <= iVar10) {
            return local_24 & 0xffff0000;
        }
        iVar6 = (int)((unsigned int)param_1 - (unsigned int)iVar14);
    }
    iVar14 = (int)((unsigned int)DAT_0070acd8 - (unsigned int)DAT_00732f38);
    uVar8 = (unsigned short)((unsigned int)iVar6 >> 0x10);
    if (param_1 < iVar14) {
        local_18 = (int)((unsigned int)param_1 - (unsigned int)iVar14);
        iVar15 = (int)((unsigned int)iVar4 - (unsigned int)iVar2);
        if (-1 < iVar15) {
            return (unsigned int)uVar8 << 0x10;
        }
        if (local_18 < iVar15) {
            return (unsigned int)uVar8 << 0x10;
        }
        local_20 = (int)(0u - ((unsigned int)iVar14 + (unsigned int)param_1));
    }
    else if ((int)(0u - (unsigned int)iVar14) == param_1 || (int)(0u - (unsigned int)param_1) < iVar14) {
        local_18 = (int)((unsigned int)param_1 - (unsigned int)iVar14);
        local_20 = (int)(0u - ((unsigned int)param_1 + (unsigned int)iVar14));
        iVar15 = 0;
    }
    else {
        local_20 = (int)(0u - ((unsigned int)param_1 + (unsigned int)iVar14));
        iVar15 = (int)((unsigned int)iVar4 - (unsigned int)iVar2);
        if (iVar15 < 0) {
            return (unsigned int)uVar8 << 0x10;
        }
        if (iVar15 <= local_20) {
            return (unsigned int)uVar8 << 0x10;
        }
        local_18 = (int)((unsigned int)param_1 - (unsigned int)iVar14);
    }
    iVar14 = (int)((unsigned int)DAT_00703ad8 - (unsigned int)DAT_00732fe8);
    if (param_1 < iVar14) {
        local_28 = (int)((unsigned int)param_1 - (unsigned int)iVar14);
        iVar9 = (int)((unsigned int)iVar5 - (unsigned int)iVar3);
        if (-1 < iVar9) {
            return (unsigned int)uVar8 << 0x10;
        }
        if (local_28 < iVar9) {
            return (unsigned int)uVar8 << 0x10;
        }
        local_1c = (int)(0u - ((unsigned int)iVar14 + (unsigned int)param_1));
    }
    else if ((int)(0u - (unsigned int)iVar14) == param_1 || (int)(0u - (unsigned int)param_1) < iVar14) {
        local_28 = (int)((unsigned int)param_1 - (unsigned int)iVar14);
        local_1c = (int)(0u - ((unsigned int)iVar14 + (unsigned int)param_1));
        iVar9 = 0;
    }
    else {
        local_1c = (int)(0u - ((unsigned int)param_1 + (unsigned int)iVar14));
        iVar9 = (int)((unsigned int)iVar5 - (unsigned int)iVar3);
        if (iVar9 < 0) {
            return (unsigned int)uVar8 << 0x10;
        }
        if (iVar9 <= local_1c) {
            return (unsigned int)uVar8 << 0x10;
        }
        local_28 = (int)((unsigned int)param_1 - (unsigned int)iVar14);
    }
    local_20 = (int)((unsigned int)local_20 << 8);
    local_1c = (int)((unsigned int)local_1c << 8);
    if (local_24 == 0) {
        iVar14 = (int)((unsigned int)DAT_0073294c - (unsigned int)DAT_006d4974);
        if (param_1 < iVar14) {
            divN = (int)((unsigned int)iVar6 << 8);
            divD = (int)((unsigned int)iVar12 - (unsigned int)iVar1);
            if (divD == 0 || (divN == (-2147483647 - 1) && divD == -1)) goto div_fault;
            uVar11 = (unsigned int)(divN / divD);
            uVar13 = 0;
        }
        else if ((int)(0u - (unsigned int)iVar14) == param_1 || (int)(0u - (unsigned int)param_1) < iVar14) {
            uVar11 = 0xff;
            uVar13 = 0;
        }
        else {
            divN = (int)((unsigned int)iVar10 << 8);
            divD = (int)((unsigned int)iVar12 - (unsigned int)iVar1);
            if (divD == 0 || (divN == (-2147483647 - 1) && divD == -1)) goto div_fault;
            uVar11 = (unsigned int)(divN / divD);
            uVar13 = 0;
        }
    }
    else {
        divN = (int)((unsigned int)iVar6 << 8);
        divD = (int)local_24;
        if (divD == 0 || (divN == (-2147483647 - 1) && divD == -1)) goto div_fault;
        uVar13 = (unsigned int)(divN / divD);
        divN = (int)((unsigned int)iVar10 << 8);
        if (divD == 0 || (divN == (-2147483647 - 1) && divD == -1)) goto div_fault;
        uVar7 = (unsigned int)(divN / divD);
        uVar11 = uVar7;
        if (-1 < (int)local_24) {
            uVar11 = uVar13;
            uVar13 = uVar7;
        }
    }
    if (iVar15 == 0) {
        iVar12 = (int)((unsigned int)DAT_007329b0 - (unsigned int)DAT_006d4978);
        if (param_1 < iVar12) {
            divN = (int)((unsigned int)local_18 << 8);
            divD = (int)((unsigned int)iVar4 - (unsigned int)iVar2);
            if (divD == 0 || (divN == (-2147483647 - 1) && divD == -1)) goto div_fault;
            uVar7 = (unsigned int)(divN / divD);
        }
        else if ((int)(0u - (unsigned int)iVar12) == param_1 || (int)(0u - (unsigned int)param_1) < iVar12) {
            uVar7 = 0xff;
        }
        else {
            divN = local_20;
            divD = (int)((unsigned int)iVar4 - (unsigned int)iVar2);
            if (divD == 0 || (divN == (-2147483647 - 1) && divD == -1)) goto div_fault;
            uVar7 = (unsigned int)(divN / divD);
        }
        uVar8 = (unsigned short)(uVar7 >> 0x10);
        if ((int)uVar11 < 0) {
            return (unsigned int)uVar8 << 0x10;
        }
        if ((int)uVar13 < 0) {
            uVar13 = 0;
        }
        if ((int)uVar7 < (int)uVar13) {
            return (unsigned int)uVar8 << 0x10;
        }
    }
    else {
        divN = (int)((unsigned int)local_18 << 8);
        divD = iVar15;
        if (divD == 0 || (divN == (-2147483647 - 1) && divD == -1)) goto div_fault;
        uVar7 = (unsigned int)(divN / divD);
        uVar8 = (unsigned short)(uVar7 >> 0x10);
        if (-1 < iVar15) {
            if ((int)uVar7 < (int)uVar13) {
                return (unsigned int)uVar8 << 0x10;
            }
            if ((int)uVar7 < (int)uVar11) {
                uVar11 = uVar7;
            }
            divN = local_20;
            if (divD == 0 || (divN == (-2147483647 - 1) && divD == -1)) goto div_fault;
            uVar7 = (unsigned int)(divN / divD);
            if ((int)uVar11 < (int)uVar7) {
                return uVar7 & 0xffff0000;
            }
            if ((int)uVar13 < (int)uVar7) {
                uVar13 = uVar7;
            }
            goto LAB_0040ff0a;
        }
        if ((int)uVar11 < (int)uVar7) {
            return (unsigned int)uVar8 << 0x10;
        }
        if ((int)uVar13 < (int)uVar7) {
            uVar13 = uVar7;
        }
        divN = local_20;
        if (divD == 0 || (divN == (-2147483647 - 1) && divD == -1)) goto div_fault;
        uVar7 = (unsigned int)(divN / divD);
        if ((int)uVar7 < (int)uVar13) {
            return uVar7 & 0xffff0000;
        }
    }
    if ((int)uVar7 < (int)uVar11) {
        uVar11 = uVar7;
    }
LAB_0040ff0a:
    if (iVar9 == 0) {
        iVar4 = (int)((unsigned int)DAT_007329b4 - (unsigned int)DAT_006d4970);
        if (param_1 < iVar4) {
            divN = (int)((unsigned int)local_28 << 8);
            divD = (int)((unsigned int)iVar5 - (unsigned int)iVar3);
            if (divD == 0 || (divN == (-2147483647 - 1) && divD == -1)) goto div_fault;
            uVar7 = (unsigned int)(divN / divD);
        }
        else if ((int)(0u - (unsigned int)iVar4) == param_1 || (int)(0u - (unsigned int)param_1) < iVar4) {
            uVar7 = 0xff;
        }
        else {
            divN = local_1c;
            divD = (int)((unsigned int)iVar5 - (unsigned int)iVar3);
            if (divD == 0 || (divN == (-2147483647 - 1) && divD == -1)) goto div_fault;
            uVar7 = (unsigned int)(divN / divD);
        }
        uVar8 = (unsigned short)(uVar7 >> 0x10);
        if ((int)uVar11 < 0) {
            return (unsigned int)uVar8 << 0x10;
        }
        if ((int)uVar13 < 0) {
            uVar13 = 0;
        }
        if ((int)uVar7 < (int)uVar13) {
            return (unsigned int)uVar8 << 0x10;
        }
    }
    else {
        divN = (int)((unsigned int)local_28 << 8);
        divD = iVar9;
        if (divD == 0 || (divN == (-2147483647 - 1) && divD == -1)) goto div_fault;
        uVar7 = (unsigned int)(divN / divD);
        uVar8 = (unsigned short)(uVar7 >> 0x10);
        if (iVar9 < 0) {
            if ((int)uVar11 < (int)uVar7) {
                return (unsigned int)uVar8 << 0x10;
            }
            if ((int)uVar13 < (int)uVar7) {
                uVar13 = uVar7;
            }
            divN = local_1c;
            if (divD == 0 || (divN == (-2147483647 - 1) && divD == -1)) goto div_fault;
            uVar7 = (unsigned int)(divN / divD);
            if ((int)uVar7 < (int)uVar13) {
                return uVar7 & 0xffff0000;
            }
        }
        else {
            if ((int)uVar7 < (int)uVar13) {
                return (unsigned int)uVar8 << 0x10;
            }
            if ((int)uVar7 < (int)uVar11) {
                uVar11 = uVar7;
            }
            divN = local_1c;
            if (divD == 0 || (divN == (-2147483647 - 1) && divD == -1)) goto div_fault;
            uVar7 = (unsigned int)(divN / divD);
            if ((int)uVar11 < (int)uVar7) {
                return uVar7 & 0xffff0000;
            }
            if ((int)uVar13 < (int)uVar7) {
                uVar13 = uVar7;
            }
        }
    }
    if ((int)uVar13 < 0x100) {
        uVar13 = (unsigned int)(unsigned short)(uVar13 << 7);
        DAT_006ffbe0 = iVar3;
        DAT_007091a8 = iVar2;
        DAT_0070a17c = iVar1;
        uVar16 = FUN_00413ca0((int)uVar13, iVar1);
        DAT_0070a17c = uVar16;
        uVar16 = FUN_00413ca0((int)uVar13, DAT_007091a8);
        DAT_007091a8 = uVar16;
        uVar16 = FUN_00413ca0((int)uVar13, DAT_006ffbe0);
        DAT_006ffbe0 = uVar16;
        return ((unsigned int)(unsigned short)((unsigned int)uVar16 >> 0x10) << 0x10) | 0xffff;
    }
    return uVar7 & 0xffff0000;

div_fault:
    RaiseException(0xC0000094UL, 0, 0, NULL);
    return 0;
}

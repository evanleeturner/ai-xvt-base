/* SPDX-License-Identifier: MIT */
/* z_xvt 0046307a */
extern ushort DAT_00730894;
extern ushort DAT_00732e58;
extern short  DAT_0070acc0;
extern ushort DAT_005f10ec;
extern ushort DAT_005f10f4;
extern ushort DAT_005f10e8;
extern ushort DAT_005f10f8;
extern byte   DAT_006d4a4e;
extern ushort DAT_00732e52;
extern ushort DAT_006d9980;
extern ushort DAT_00732e8c;
extern ushort DAT_00732a34;
extern char   DAT_0070a188;
extern char   DAT_006d4998;
extern byte   DAT_0070acc2;
extern byte  *DAT_00709ff4;
extern int    DAT_006ffbf0;
extern byte  *DAT_005f1d20;
extern int    DAT_0052552c;
extern uint   DAT_00525534;
extern uint   DAT_0052550c;
extern int    DAT_0070acc8;
extern char   DAT_0071e524;
extern byte   DAT_006da9fa;
extern byte   DAT_00732e50;
extern byte   DAT_0071b012;
extern int   *DAT_005f1d24;

void __cdecl FUN_00463e10(short param_1);

/* in_AL and in_ZF are register inputs at entry (AL and ZF); they are
   deliberately left as uninitialized locals, as in the decompile. */
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wuninitialized"
void __stdcall FUN_0046307a(void)
{
    byte bVar1;
    byte bVar2;
    int bVar3;
    byte in_AL;
    int in_ZF;
    int iVar4;
    byte uVar5;
    byte *pbVar6;
    byte bVar7;
    uint uVar8;
    uint uVar9;
    byte bVar10;
    int iVar11;
    int iVar12;
    byte *puVar13;
    int cStack00000008;
    uint uStack0000000c;

    if (in_ZF) {
        if ((DAT_00730894 != 0) && ((short)DAT_00732e58 < DAT_0070acc0)) {
            DAT_005f10ec = DAT_0070acc0;
            DAT_005f10f4 = DAT_00732e58;
            DAT_005f10e8 = (ushort)(DAT_006d4a4e + DAT_00732e52);
            DAT_005f10f8 = DAT_00732e52;
            if ((int)(uint)DAT_00732e58 < (int)(short)DAT_006d9980) {
                DAT_005f10f4 = DAT_006d9980;
            }
            if ((int)(uint)DAT_00732e52 < (int)(short)DAT_00732e8c) {
                DAT_005f10f8 = DAT_00732e8c;
            }
            if ((int)(short)DAT_00732a34 < (int)(uint)DAT_005f10e8) {
                DAT_005f10e8 = DAT_00732a34;
            }
            if (DAT_005f10f8 < DAT_005f10e8) {
                FUN_00463e10(0);
            }
        }
        DAT_00732e58 = DAT_006d9980;
        DAT_00732e52 = (ushort)(DAT_00732e52 + DAT_006d4a4e);
        return;
    }
    if (in_AL < 0x20) {
        return;
    }
    if ((((DAT_0070a188 != '\0') && (DAT_006d4998 == '\0')) && (0x60 < in_AL)) && (in_AL < 0x7b)) {
        in_AL = (byte)(in_AL - 0x20);
    }
    pbVar6 = (byte *)((uint)(byte)(in_AL - 0x20) * (uint)DAT_0070acc2 + (uint)DAT_00709ff4);
    bVar1 = *pbVar6;
    bVar2 = pbVar6[1];
    pbVar6 = pbVar6 + 2;
    uStack0000000c = (uint)(ushort)bVar1;
    if (((int)DAT_0070acc0 <= (int)((uint)(int)(short)DAT_00732e58 + uStack0000000c)) && (DAT_006ffbf0 != 0)) {
        if ((DAT_00730894 != 0) && ((short)DAT_00732e58 < DAT_0070acc0)) {
            DAT_005f10ec = DAT_0070acc0;
            DAT_005f10f4 = DAT_00732e58;
            DAT_005f10e8 = (ushort)(DAT_006d4a4e + DAT_00732e52);
            DAT_005f10f8 = DAT_00732e52;
            if ((int)(uint)DAT_00732e58 < (int)(short)DAT_006d9980) {
                DAT_005f10f4 = DAT_006d9980;
            }
            if ((int)(uint)DAT_00732e52 < (int)(short)DAT_00732e8c) {
                DAT_005f10f8 = DAT_00732e8c;
            }
            if ((int)(short)DAT_00732a34 < (int)(uint)DAT_005f10e8) {
                DAT_005f10e8 = DAT_00732a34;
            }
            if (DAT_005f10f8 < DAT_005f10e8) {
                FUN_00463e10(0);
            }
        }
        DAT_00732e58 = DAT_006d9980;
        DAT_00732e52 = (ushort)(DAT_00732e52 + bVar2 + 2);
    }
    bVar10 = 0;
    bVar3 = 0;
    iVar11 = (int)(short)DAT_00732e52;
    iVar4 = iVar11;
    if (iVar11 < (short)DAT_00732e8c) {
        iVar4 = (int)(short)DAT_00732e8c;
    }
    uVar8 = *(uint *)(DAT_005f1d20 + (uint)iVar4 * 4U);
    if (DAT_0052552c == 0x13) {
LAB_0046337a:
        if (DAT_00525534 == 0xa0000) goto LAB_0046338d;
    }
    else if (DAT_00525534 == 0xa0000) {
        uVar8 = uVar8 % DAT_0052550c;
        iVar12 = (int)((uint)iVar11 + 1U + (uint)bVar2);
        if ((short)DAT_00732a34 < iVar12) {
            iVar12 = (int)(short)DAT_00732a34;
        }
        if (0xffff < (int)(((uint)iVar12 - (uint)iVar4) * (uint)DAT_0070acc8 + uVar8)) {
            bVar3 = 1;
        }
        goto LAB_0046337a;
    }
    bVar3 = 1;
LAB_0046338d:
    if (bVar3) {
        if (iVar11 < (int)((uint)iVar11 + (uint)bVar2)) {
            do {
                bVar7 = *pbVar6;
                uVar8 = uStack0000000c;
                if (DAT_0071e524 != '\0') {
                    uVar8 = uStack0000000c + 1;
                }
                iVar4 = (int)(short)DAT_00732e58;
                iVar12 = (int)(short)DAT_006d9980;
                if (iVar4 < iVar12) {
                    if ((int)uVar8 <= (int)((uint)iVar12 - (uint)iVar4)) break;
                    uVar8 = (uVar8 - (uint)iVar12) + (uint)iVar4;
                    bVar7 = (byte)((uint)bVar7 << ((char)DAT_006d9980 - (char)DAT_00732e58 & 0x1fU));
                    iVar4 = iVar12;
                }
                if ((short)DAT_00732a34 <= iVar11) break;
                if ((short)DAT_00732e8c <= iVar11) {
                    if (((int)DAT_0070acc0 < (int)(uVar8 + (uint)iVar4)) &&
                        (uVar8 = (uint)(int)DAT_0070acc0 - (uint)iVar4, (int)uVar8 < 1)) break;
                    uVar9 = *(uint *)(DAT_005f1d20 + (uint)iVar11 * 4U) + (uint)iVar4;
                    if ((DAT_0052552c != 0x13) && (DAT_00525534 == 0xa0000)) {
                        uVar9 = uVar9 % DAT_0052550c;
                    }
                    puVar13 = (byte *)(uVar9 + DAT_00525534);
                    while (uVar8 != 0) {
                        uVar8 = uVar8 - 1;
                        uVar5 = DAT_006da9fa;
                        if ((((bVar7 & 0x80) == 0) && (uVar5 = DAT_00732e50, DAT_0071e524 != '\0')) &&
                            ((bVar10 & 0x80) != 0)) {
                            uVar5 = DAT_0071b012;
                        }
                        *puVar13 = uVar5;
                        puVar13 = puVar13 + 1;
                        bVar10 = (byte)((uint)bVar10 * 2U);
                        bVar7 = (byte)((uint)bVar7 * 2U);
                    }
                }
                iVar11 = iVar11 + 1;
                bVar10 = *pbVar6;
                pbVar6 = pbVar6 + 2;
                bVar10 = bVar10 >> 1;
            } while (iVar11 < (int)((uint)(int)(short)DAT_00732e52 + (uint)bVar2));
        }
    }
    else {
        iVar4 = (int)(uVar8 + DAT_00525534);
        if (iVar11 < (int)((uint)bVar2 + (uint)iVar11)) {
            do {
                bVar7 = *pbVar6;
                uVar8 = uStack0000000c;
                if (DAT_0071e524 != '\0') {
                    uVar8 = uStack0000000c + 1;
                }
                cStack00000008 = (int)(short)DAT_00732e58;
                iVar12 = (int)(short)DAT_006d9980;
                if (cStack00000008 < iVar12) {
                    if ((int)uVar8 <= (int)((uint)iVar12 - (uint)cStack00000008)) break;
                    uVar8 = (uVar8 - (uint)iVar12) + (uint)cStack00000008;
                    bVar7 = (byte)((uint)bVar7 << ((char)DAT_006d9980 - (char)cStack00000008 & 0x1fU));
                    cStack00000008 = iVar12;
                }
                if ((short)DAT_00732a34 <= iVar11) break;
                if ((short)DAT_00732e8c <= iVar11) {
                    if (((int)DAT_0070acc0 < (int)((uint)cStack00000008 + uVar8)) &&
                        (uVar8 = (uint)(int)DAT_0070acc0 - (uint)cStack00000008, (int)uVar8 < 1)) break;
                    puVar13 = (byte *)((uint)cStack00000008 + (uint)iVar4);
                    while (uVar8 != 0) {
                        uVar8 = uVar8 - 1;
                        uVar5 = DAT_006da9fa;
                        if ((((bVar7 & 0x80) == 0) && (uVar5 = DAT_00732e50, DAT_0071e524 != '\0')) &&
                            ((bVar10 & 0x80) != 0)) {
                            uVar5 = DAT_0071b012;
                        }
                        *puVar13 = uVar5;
                        puVar13 = puVar13 + 1;
                        bVar7 = (byte)((uint)bVar7 * 2U);
                        bVar10 = (byte)((uint)bVar10 * 2U);
                    }
                    iVar4 = (int)((uint)iVar4 + (uint)*DAT_005f1d24);
                }
                iVar11 = iVar11 + 1;
                bVar10 = *pbVar6 >> 1;
                pbVar6 = pbVar6 + 2;
            } while (iVar11 < (int)((uint)(int)(short)DAT_00732e52 + (uint)bVar2));
        }
    }
    DAT_00732e58 = (ushort)(DAT_00732e58 + bVar1);
    if ((DAT_0070acc0 <= (short)DAT_00732e58) && (DAT_006ffbf0 != 0)) {
        if ((DAT_00730894 != 0) && ((short)DAT_00732e58 < DAT_0070acc0)) {
            DAT_005f10ec = DAT_0070acc0;
            DAT_005f10f4 = DAT_00732e58;
            DAT_005f10e8 = (ushort)(DAT_006d4a4e + DAT_00732e52);
            DAT_005f10f8 = DAT_00732e52;
            if ((int)(uint)DAT_00732e58 < (int)(short)DAT_006d9980) {
                DAT_005f10f4 = DAT_006d9980;
            }
            if ((int)(uint)DAT_00732e52 < (int)(short)DAT_00732e8c) {
                DAT_005f10f8 = DAT_00732e8c;
            }
            if ((int)(short)DAT_00732a34 < (int)(uint)DAT_005f10e8) {
                DAT_005f10e8 = DAT_00732a34;
            }
            if (DAT_005f10f8 < DAT_005f10e8) {
                FUN_00463e10(0);
            }
        }
        DAT_00732e58 = DAT_006d9980;
        DAT_00732e52 = (ushort)(DAT_00732e52 + bVar2 + 2);
    }
    return;
}
#pragma clang diagnostic pop

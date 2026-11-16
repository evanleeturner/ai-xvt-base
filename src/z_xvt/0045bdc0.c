/* SPDX-License-Identifier: MIT */
/* z_xvt 0045bdc0 */
int * FUN_0045bcb0(uint param_1);
void FUN_004b696b(int *param_1);
int * FUN_004b88a0(int param_1, int param_2, int param_3);
void FUN_0045bbe0(const char *fmt, ...);
int FUN_004b63ea(int param_1, int param_2, int *param_3, void *param_4);

extern int DAT_006cca80;
extern byte DAT_005279e8[];
extern byte DAT_005279d8[];

#define puStack_5c (*(undefined4 **)&local_6c[4])
#define iStack_c (*(int *)&local_6c[24])
#define iStack_8 (*(int *)&local_6c[25])

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wuninitialized"

int * __cdecl FUN_0045bdc0(int param_1, int param_2, undefined4 param_3, byte *param_4)
{
    byte *pbVar1;
    byte bVar2;
    int *piVar3;
    int iVar4;
    int iVar5;
    undefined4 *puVar6;
    undefined4 *puVar7;
    int iVar8;
    uint uVar9;
    uint uVar10;
    undefined4 *puVar11;
    int *unaff_EDI;
    int unaff_retaddr;
    byte bVar12;
    byte bVar13;
    void *pvStack_88;
    int iStack_84;
    undefined4 local_6c[27];

    piVar3 = (int *)FUN_0045bcb0((uint)param_4);
    iVar4 = DAT_006cca80;
    if (piVar3[2] != 0) {
        FUN_004b696b(piVar3);
        return piVar3;
    }
    piVar3 = FUN_004b88a0(param_1, param_2, DAT_006cca80);
    puVar11 = local_6c;
    for (iVar5 = 0x1b; iVar5 != 0; iVar5 = (int)((uint)iVar5 - 1)) {
        *puVar11 = 0;
        puVar11 = puVar11 + 1;
    }
    local_6c[0] = 0x6c;
    iVar5 = (**(int (__stdcall **)(int *, int, undefined4 *, int))(*(int *)piVar3[3] + 100))(
        (int *)piVar3[3], 0, local_6c, 1);
    if (iVar5 != 0) {
        FUN_0045bbe0("Failed to Lock Texture surface: returned(%x)\n", iVar5);
    }
    bVar13 = 0;
    iVar5 = 0;
    if (0 < iStack_c) {
        do {
            bVar2 = *param_4;
            if (bVar2 == 0xff) break;
            iVar8 = 0;
            puVar6 = (undefined4 *)((uint)iVar4 + (uint)puStack_5c);
            puVar11 = puStack_5c;
            while (bVar2 != 0xfe) {
                bVar2 = *param_4;
                if (bVar2 == 0xfb) {
                    bVar13 = param_4[1];
                    param_4 = param_4 + 3;
                }
                else if (bVar2 == 0xfc) {
                    pbVar1 = param_4 + 1;
                    param_4 = param_4 + 2;
                    bVar2 = (byte)(*pbVar1 + 1);
                    if (iVar8 < param_1) {
                        iVar8 = (int)((uint)iVar8 + (uint)bVar2);
                        if (param_1 < iVar8) {
                            bVar2 = (byte)((char)param_1 - ((char)iVar8 - bVar2));
                            iVar8 = param_1;
                        }
                        for (; bVar2 != 0; bVar2 = (byte)(bVar2 - 1)) {
                            *(unsigned short *)puVar11 = 0;
                            puVar11 = (undefined4 *)((uint)puVar11 + 2);
                        }
                    }
                }
                else {
                    if (bVar2 == 0xfd) {
                        bVar2 = param_4[1];
                        bVar12 = param_4[2];
                        param_4 = param_4 + 3;
                    }
                    else {
                        param_4 = param_4 + 1;
                        bVar12 = (byte)((bVar2 >> (DAT_005279e8[unaff_retaddr] & 0x1f)) + bVar13);
                        bVar2 = (byte)(bVar2 & DAT_005279d8[unaff_retaddr]);
                    }
                    bVar2 = (byte)(bVar2 + 1);
                    if (iVar8 < param_1) {
                        iVar8 = (int)((uint)iVar8 + (uint)bVar2);
                        if (param_1 < iVar8) {
                            bVar2 = (byte)((char)param_1 - ((char)iVar8 - bVar2));
                            iVar8 = param_1;
                        }
                        if (bVar2 != 0) {
                            puVar7 = puVar11;
                            do {
                                bVar2 = (byte)(bVar2 - 1);
                                puVar11 = (undefined4 *)((uint)puVar7 + 2);
                                *(unsigned short *)puVar7 =
                                    *(unsigned short *)((uint)iStack_8 + (uint)bVar12 * 2);
                                puVar7 = puVar11;
                            } while (bVar2 != 0);
                        }
                    }
                }
                bVar2 = *param_4;
            }
            param_4 = param_4 + 1;
            puStack_5c = puVar11;
            if (puVar11 < puVar6) {
                uVar9 = (uint)puVar6 + (1 - (uint)puVar11);
                uVar10 = uVar9 >> 1;
                puStack_5c = (undefined4 *)((uint)puVar11 + uVar10 * 2);
                for (uVar9 = uVar9 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
                    *puVar11 = 0;
                    puVar11 = puVar11 + 1;
                }
                for (uVar9 = (uint)((uVar10 & 1) != 0); uVar9 != 0; uVar9 = uVar9 - 1) {
                    *(unsigned short *)puVar11 = 0;
                    puVar11 = (undefined4 *)((uint)puVar11 + 2);
                }
            }
            iVar5 = (int)((uint)iVar5 + 1);
        } while (iVar5 < iStack_c);
    }
    if (iVar5 < iStack_c) {
        iStack_c = (int)((uint)iStack_c - (uint)iVar5);
        do {
            puVar11 = (undefined4 *)((uint)iVar4 + (uint)puStack_5c);
            if (puStack_5c < puVar11) {
                uVar9 = (uint)puVar11 + (1 - (uint)puStack_5c);
                uVar10 = uVar9 >> 1;
                puVar11 = puStack_5c;
                for (uVar9 = uVar9 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
                    *puVar11 = 0;
                    puVar11 = puVar11 + 1;
                }
                for (uVar9 = (uint)((uVar10 & 1) != 0); uVar9 != 0; uVar9 = uVar9 - 1) {
                    *(unsigned short *)puVar11 = 0;
                    puVar11 = (undefined4 *)((uint)puVar11 + 2);
                }
                puStack_5c = (undefined4 *)((uint)puStack_5c + uVar10 * 2);
            }
            iStack_c = (int)((uint)iStack_c - 1);
        } while (iStack_c != 0);
    }
    iVar4 = FUN_004b63ea(iStack_84, 1, unaff_EDI, pvStack_88);
    if (iVar4 == 0) {
        FUN_0045bbe0("AddToTextureCache returned NULL!\n");
        return (int *)0;
    }
    return unaff_EDI;
}

#pragma clang diagnostic pop

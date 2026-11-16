/* SPDX-License-Identifier: MIT */
/* z_xvt 0042e7d0 */
typedef unsigned char byte;
typedef unsigned int uint;
typedef unsigned short ushort;

/* Globals. Types are inferred from use; unaligned ones are read/written with memcpy. */
extern int DAT_006db5a8;
extern int DAT_006db5a4;
extern int DAT_0071ecf4;
extern byte DAT_0071bce5[];   /* int read at +idx*0x5bd (unaligned) */
extern byte DAT_0071bcb8[];   /* int read at +idx*0x5bd (unaligned) */
extern char DAT_0071b740[];
extern int DAT_0070aaf0[];
extern byte *DAT_006a89a0;
extern byte *DAT_006a89b0;
extern byte *DAT_006a89c0;
extern char *DAT_006a89c4;
extern uint DAT_005373d0;
extern uint DAT_005373d4;
extern uint DAT_005375d0[];
extern int DAT_00536f48[];
extern byte DAT_007092c0;
extern int DAT_0052556c;
extern int DAT_00525560;
extern int DAT_005250d4;
extern int DAT_00732a18;
extern int ram0x0071f2ec;
extern byte DAT_0081f7de;     /* table of 8 entries, stride 0x58, unaligned int at +0 */
extern byte DAT_0081f812;     /* int flag at entry+0x34 (unaligned) */
extern byte DAT_0081fa9a;
extern byte DAT_0081fa9e;

int   __cdecl FUN_0048f530(void);
void  __cdecl FUN_004a7510(void);
void  __cdecl FUN_00438a40(void);
void  __cdecl FUN_00438c30(int, byte *, int);
void  __cdecl FUN_0049a2f0(byte *);
int   __cdecl FUN_00498fb0(void);
byte *__cdecl FUN_00498fa0(void);
int   __cdecl FUN_0049a190(uint *, byte *, int);
int   __cdecl FUN_00432ca0(void);
void  __cdecl FUN_00431110(int, uint *, uint);
int   __cdecl FUN_004a53f0(void);
uint  __cdecl FUN_004a5470(void);
int   __cdecl _sprintf(char *, const char *, ...);
int  *__cdecl FUN_00431d40(int *, int *);
int   __cdecl FUN_00432c10(int);
void  __cdecl FUN_0049af60(int);
int  *__cdecl FUN_0049b0d0(int, uint, void *);
int   __cdecl FUN_00431d30(void);
void  __cdecl FUN_0049b8a0(int *);
void  __cdecl FUN_0049bf90(int *, int, int);
void  __cdecl FUN_004310c0(uint *, int);
void  __cdecl FUN_00438b70(void);
void  __cdecl FUN_004a74f0(void);
void  __cdecl FUN_0049bfc0(int, int);

void __cdecl FUN_0042e7d0(int *param_1)
{
    byte bVar1;
    byte bVar2;
    int iVar3;
    byte *pbVar4;
    uint uVar5;
    int iVar6;
    int *piVar7;
    int *piVar8;
    int iVar9;
    byte *pbVar10;
    uint *puVar11;
    byte *pbVar11;
    uint auStack_6c[2];   /* auStack_6c[0] is uStack_6c; uStack_68 lives at byte +4 */
    ushort uStack_68;
    int iStack_64;
    uint uStack_60;
    uint local_5c;
    int local_58;
    int local_54;
    byte abStack_50[80];
    int iTmp;
    int iTmp2;

    local_5c = 0;
    if (*param_1 == 0x3d) {
        iVar3 = FUN_0048f530();
        DAT_006db5a8 = (int)((uint)DAT_006db5a8 + (uint)iVar3);
        __builtin_memcpy(&iTmp, DAT_0071bce5 + DAT_0071ecf4 * 0x5bd, 4);
        if (iTmp != 0) {
            FUN_004a7510();
        }
        FUN_00438a40();
        FUN_00438c30(1, DAT_006a89a0, 0x30);
        FUN_0049a2f0((byte *)(param_1 + 2));
        DAT_005373d0 = 0x3c;
        iVar3 = FUN_00498fb0();
        pbVar4 = FUN_00498fa0();
        local_58 = FUN_0049a190(&DAT_005373d4, pbVar4, iVar3);
        local_58 = (int)((uint)local_58 * 4u);
        uVar5 = (uint)local_58 + 4u;
        while (1) {
            puVar11 = &DAT_005373d0;
            iVar3 = FUN_00432ca0();
            FUN_00431110(iVar3, puVar11, uVar5);
            iVar3 = FUN_004a53f0();
            if ((iVar3 != 0) && (uVar5 = FUN_004a5470(), uVar5 == 0x1b)) {
                DAT_005373d4 = (uint)DAT_0071ecf4;
                DAT_005373d0 = 0x15;
                FUN_004310c0(&DAT_005373d0, 8);
                iVar6 = DAT_0071ecf4;
                DAT_007092c0 = 1;
                iVar3 = DAT_0071ecf4 * 0x5bd;
                iVar9 = 0;
                DAT_0071b740[iVar3] = 0;
                DAT_0070aaf0[iVar6] = 1;
                pbVar11 = &DAT_0081f7de;
                goto LAB_0042ee41;
            }
            local_54 = 0;
            do {
                iVar3 = DAT_006db5a8;
                iVar6 = FUN_0048f530();
                DAT_006db5a4 = (int)((uint)DAT_006db5a4 + (((uint)DAT_006db5a8 + (uint)iVar6) - (uint)iVar3));
                DAT_006db5a8 = iVar3;
                if (0x1ba8 < DAT_006db5a4) {
                    FUN_00438b70();
                    return;
                }
                uVar5 = (uint)((int)(0x1ba8u - (uint)DAT_006db5a4) / 0x76);
                if (uVar5 != local_5c) {
                    local_5c = uVar5;
                    if ((int)uVar5 < 0x32) {
                        _sprintf((char *)abStack_50, DAT_006a89c4, (int)uVar5 / 2, (uVar5 & 1) * 5);
                        pbVar4 = abStack_50;
                    } else {
                        pbVar4 = DAT_006a89b0;
                        if ((uVar5 & 1) != 0) {
                            pbVar4 = DAT_006a89c0;
                        }
                    }
                    FUN_00438c30(3, pbVar4, 0x30);
                }
                piVar7 = FUN_00431d40(&iStack_64, &local_58);
                iVar3 = DAT_006db5a8;
                DAT_006db5a8 = iVar3;
                if (piVar7 != (int *)0) {
                    switch (*piVar7) {
                    case 1:
                        iVar3 = FUN_00432c10(iStack_64);
                        if (DAT_0071b740[iVar3 * 0x5bd] == '\0') {
                            DAT_005373d0 = 0x15;
                            DAT_005373d4 = (uint)iVar3;
                            FUN_00431110(iStack_64, &DAT_005373d0, 8);
                        } else {
                            if (0 < DAT_00536f48[iVar3]) {
                                DAT_00536f48[iVar3] = 0;
                            }
                            FUN_0049af60(iVar3);
                            auStack_6c[0] = 0;
                            bVar1 = *(byte *)(piVar7 + 1);
                            if ((bVar1 & 0x7f) == 0x7f) {
                                __builtin_memcpy(&uVar5, (byte *)piVar7 + 5, 4);
                                if ((bVar1 & 0x80) == 0) {
                                    pbVar4 = (byte *)piVar7 + 9;
                                } else {
                                    pbVar4 = (byte *)piVar7 + 10;
                                    auStack_6c[0] = (uint)*((byte *)piVar7 + 9) << 0x10;
                                }
                            } else {
                                uVar5 = DAT_005375d0[iVar3];
                                uStack_60 = (uint)(bVar1 & 0x7f);
                                if (uStack_60 < (uVar5 & 0x7f)) {
                                    uVar5 = uVar5 + 0x80;
                                }
                                uVar5 = (uVar5 & 0xffffff80) | uStack_60;
                                if ((bVar1 & 0x80) == 0) {
                                    pbVar4 = (byte *)piVar7 + 5;
                                } else {
                                    pbVar4 = (byte *)piVar7 + 6;
                                    auStack_6c[0] = (uint)*((byte *)piVar7 + 5) << 0x10;
                                }
                            }
                            DAT_005375d0[iVar3] = uVar5;
                            auStack_6c[0] = ((uint)*pbVar4 << 0x18 | (auStack_6c[0] & 0xffffff)) & 0xfeffffff;
                            uStack_68 = (ushort)(((uint)(byte)((pbVar4[1] & 1) * 2 | (*pbVar4 & 1)) << 8 | (uint)pbVar4[1]) & 0xfffe);
                            __builtin_memcpy((byte *)auStack_6c + 4, &uStack_68, 2);
                            piVar8 = FUN_0049b0d0(iVar3, uVar5, auStack_6c);
                            if (piVar8 != (int *)0) {
                                iVar3 = FUN_00431d30();
                                *piVar8 = 1;
                                if (iVar3 == 0) {
                                    *piVar8 = 0;
                                }
                                piVar8[1] = 1;
                            }
                        }
                        break;
                    case 2:
                        FUN_0049b8a0(piVar7);
                        FUN_0048f530();
                        DAT_006db5a8 = iVar3;
                        if (DAT_007092c0 != '\0') {
                            return;
                        }
                        break;
                    case 8:
                        DAT_007092c0 = 1;
                        DAT_0052556c = 1;
                        DAT_0071b740[DAT_0071ecf4 * 0x5bd] = 0;
                        return;
                    case 0x15:
                        iVar3 = piVar7[1];
                        if ((-1 < iVar3) && (iVar3 < 8)) {
                            DAT_0070aaf0[iVar3] = 1;
                        }
                        iVar6 = DAT_0071ecf4;
                        if (DAT_0071ecf4 == iVar3) {
                            DAT_007092c0 = 1;
                            iVar3 = DAT_0071ecf4 * 0x5bd;
                            iVar9 = 0;
                            DAT_0071b740[iVar3] = 0;
                            DAT_0070aaf0[iVar6] = 1;
                            pbVar11 = &DAT_0081f7de;
                            goto LAB_0042eb74;
                        }
                        break;
                    case 0x16:
                        iVar3 = FUN_00432c10(iStack_64);
                        if (DAT_0071b740[iVar3 * 0x5bd] == '\0') {
                            DAT_005373d0 = 0x15;
                            DAT_005373d4 = (uint)iVar3;
                            FUN_00431110(iStack_64, &DAT_005373d0, 8);
                        } else {
                            if (0 < DAT_00536f48[iVar3]) {
                                DAT_00536f48[iVar3] = 0;
                            }
                            FUN_0049af60(iVar3);
                            pbVar4 = (byte *)piVar7 + 5;
                            auStack_6c[0] = 0;
                            auStack_6c[1] = 0;
                            uVar5 = (uint)*(byte *)(piVar7 + 1);
                            if (*(byte *)(piVar7 + 1) != 0) {
                                do {
                                    uStack_60 = uVar5;
                                    bVar1 = *pbVar4;
                                    bVar2 = bVar1 & 0x7f;
                                    if (bVar2 == 0x7f) {
                                        __builtin_memcpy(&uVar5, pbVar4 + 1, 4);
                                        if ((bVar1 & 0x80) == 0) {
                                            pbVar10 = pbVar4 + 5;
                                        } else {
                                            pbVar10 = pbVar4 + 6;
                                            auStack_6c[0] = ((uint)pbVar4[5] << 0x10) | (auStack_6c[0] & 0xffff);
                                            auStack_6c[0] = auStack_6c[0] & 0xffffff;
                                        }
                                    } else {
                                        uVar5 = DAT_005375d0[iVar3];
                                        if ((uint)bVar2 < (uVar5 & 0x7f)) {
                                            uVar5 = uVar5 + 0x80;
                                        }
                                        uVar5 = (uVar5 & 0xffffff80) | (uint)bVar2;
                                        if ((bVar1 & 0x80) == 0) {
                                            pbVar10 = pbVar4 + 1;
                                        } else {
                                            pbVar10 = pbVar4 + 2;
                                            auStack_6c[0] = ((uint)pbVar4[1] << 0x10) | (auStack_6c[0] & 0xffff);
                                            auStack_6c[0] = auStack_6c[0] & 0xffffff;
                                        }
                                    }
                                    pbVar4 = pbVar10 + 2;
                                    DAT_005375d0[iVar3] = uVar5;
                                    auStack_6c[0] = ((uint)*pbVar10 << 0x18 | (auStack_6c[0] & 0xffffff)) & 0xfeffffff;
                                    uStack_68 = (ushort)(((uint)(byte)((pbVar10[1] & 1) * 2 | (*pbVar10 & 1)) << 8 | (uint)pbVar10[1]) & 0xfffe);
                                    __builtin_memcpy((byte *)auStack_6c + 4, &uStack_68, 2);
                                    piVar8 = FUN_0049b0d0(iVar3, uVar5, auStack_6c);
                                    if (piVar8 != (int *)0) {
                                        iVar6 = FUN_00431d30();
                                        *piVar8 = 1;
                                        if (iVar6 == 0) {
                                            *piVar8 = 0;
                                        }
                                        piVar8[1] = 1;
                                    }
                                    uVar5 = uStack_60 - 1;
                                } while (uStack_60 - 1 != 0);
                                uStack_60 = 0;
                            }
                        }
                        break;
                    case 0x17:
                        DAT_005250d4 = piVar7[1];
                        break;
                    case 0x1d:
                        iVar3 = FUN_00432ca0();
                        if (iVar3 == iStack_64) {
                            DAT_006db5a4 = 0;
                        } else {
                            iVar3 = FUN_00432c10(iStack_64);
                            if ((DAT_0071b740[iVar3 * 0x5bd] != '\0') && (0 < DAT_00536f48[iVar3])) {
                                DAT_00536f48[iVar3] = 0;
                            }
                        }
                        break;
                    case 0x3d:
                        DAT_005373d4 = (uint)DAT_0071ecf4;
                        DAT_005373d0 = 0x15;
                        FUN_004310c0(&DAT_005373d0, 8);
                        iVar6 = DAT_0071ecf4;
                        DAT_007092c0 = 1;
                        iVar3 = DAT_0071ecf4 * 0x5bd;
                        iVar9 = 0;
                        DAT_0071b740[iVar3] = 0;
                        DAT_0070aaf0[iVar6] = 1;
                        pbVar11 = &DAT_0081f7de;
                        goto LAB_0042ef17;
                    case 0x3e:
                    case 0x3f:
                        if (piVar7[1] == DAT_00525560) {
                            local_54 = 1;
                        }
                    }
                }
                iVar3 = DAT_0071ecf4;
            } while (local_54 == 0);
            if (*piVar7 == 0x3e) {
                break;
            }
            iVar3 = piVar7[2];
            iVar6 = piVar7[3];
            while (iVar6 != -1) {
                FUN_0049bf90(piVar7 + 5, piVar7[3], piVar7[4]);
                piVar7 = (int *)((int)piVar7 + piVar7[4] + 8);
                iVar6 = piVar7[3];
            }
            uVar5 = 8;
            DAT_005373d0 = 0x10;
            DAT_005373d4 = (uint)iVar3;
        }
        iVar6 = 0;
        if (DAT_0070aaf0[DAT_0071ecf4] != 0) {
            DAT_0070aaf0[DAT_0071ecf4] = 1;
            DAT_007092c0 = 1;
            DAT_0071b740[iVar3 * 0x5bd] = 0;
            pbVar11 = &DAT_0081f7de;
            do {
                __builtin_memcpy(&iTmp, pbVar11, 4);
                __builtin_memcpy(&iTmp2, DAT_0071bcb8 + iVar3 * 0x5bd, 4);
                if (iTmp == iTmp2) {
                    goto LAB_0042efa2;
                }
                pbVar11 = pbVar11 + 0x58;
                iVar6 = iVar6 + 1;
            } while (pbVar11 < &DAT_0081fa9e);
            iVar6 = 0;
LAB_0042efa2:
            iTmp = 1;
            __builtin_memcpy(&DAT_0081f812 + iVar6 * 0x58, &iTmp, 4);
            return;
        }
        FUN_0048f530();
        __builtin_memcpy(&iTmp, DAT_0071bce5 + DAT_0071ecf4 * 0x5bd, 4);
        if (iTmp != 0) {
            FUN_004a74f0();
        }
        FUN_0049bfc0(piVar7[2], piVar7[1]);
        DAT_005373d0 = 0x1b;
        uVar5 = 4;
        puVar11 = &DAT_005373d0;
        iVar3 = FUN_00432ca0();
        FUN_00431110(iVar3, puVar11, uVar5);
        DAT_006db5a8 = (int)((uint)DAT_00732a18 + (uint)ram0x0071f2ec);
        FUN_00438b70();
    }
    return;

LAB_0042ee41:
    __builtin_memcpy(&iTmp, pbVar11, 4);
    __builtin_memcpy(&iTmp2, DAT_0071bcb8 + iVar3, 4);
    if (iTmp == iTmp2) {
        goto LAB_0042ee55;
    }
    pbVar11 = pbVar11 + 0x58;
    iVar9 = iVar9 + 1;
    if (!(&DAT_0081fa9a + 3 < pbVar11)) {
        goto LAB_0042ee41;
    }
    iVar9 = 0;
LAB_0042ee55:
    iTmp = 1;
    __builtin_memcpy(&DAT_0081f812 + iVar9 * 0x58, &iTmp, 4);
    return;

LAB_0042ef17:
    __builtin_memcpy(&iTmp, pbVar11, 4);
    __builtin_memcpy(&iTmp2, DAT_0071bcb8 + iVar3, 4);
    if (iTmp == iTmp2) {
        goto LAB_0042ef2b;
    }
    pbVar11 = pbVar11 + 0x58;
    iVar9 = iVar9 + 1;
    if (!(&DAT_0081fa9a + 3 < pbVar11)) {
        goto LAB_0042ef17;
    }
    iVar9 = 0;
LAB_0042ef2b:
    iTmp = 1;
    __builtin_memcpy(&DAT_0081f812 + iVar9 * 0x58, &iTmp, 4);
    return;

LAB_0042eb74:
    __builtin_memcpy(&iTmp, pbVar11, 4);
    __builtin_memcpy(&iTmp2, DAT_0071bcb8 + iVar3, 4);
    if (iTmp == iTmp2) {
        goto LAB_0042eb87;
    }
    pbVar11 = pbVar11 + 0x58;
    iVar9 = iVar9 + 1;
    if (!(&DAT_0081fa9a + 3 < pbVar11)) {
        goto LAB_0042eb74;
    }
    iVar9 = 0;
LAB_0042eb87:
    iTmp = 1;
    __builtin_memcpy(&DAT_0081f812 + iVar9 * 0x58, &iTmp, 4);
    return;
}

/* SPDX-License-Identifier: MIT */
/* z_xvt 0040cd90 */
extern int DAT_006d4988;
extern int DAT_006d4980;
extern int DAT_00732a3c;
extern int DAT_00732a38;
extern int DAT_00703c88;
extern int DAT_00732904;
extern int DAT_006db798;
extern int DAT_0071b704;
extern char DAT_007092c1;
extern char DAT_00525bf7[];

void __cdecl FUN_0040ed40(byte *param_1, ushort param_2, uint param_3);
int __cdecl FUN_0042b150(int param_1);

void __cdecl FUN_0040cd90(byte *param_1, ushort param_2)
{
    int iVar1;
    char cVar2;
    byte bVar3;
    ushort uVar4;
    int bVar5;
    int iVar6;
    int iVar7;
    int iVar8;
    ushort uVar9;
    ushort uVar10;
    uint uVar11;
    uint uVar12;
    uint uVar13;
    byte *pbVar14;
    uint uVar15;
    uint uVar16;
    int doCall;
    int v1b;
    int v1f;
    int vx;
    short s16;
    ushort u16;

    uVar13 = 0;
    uVar11 = param_2;
    iVar6 = (int)((uint)param_2 * 0x23u + (uint)DAT_006d4988);
    __builtin_memcpy(&v1b, (void *)(iVar6 + 0x1b), 4);
    if (v1b == -1) {
        __builtin_memcpy(&v1f, (void *)(iVar6 + 0x1f), 4);
        __builtin_memcpy(&uVar9, (void *)(v1f + 0x83), 2);
        switch (*(byte *)(iVar6 + 2)) {
        case 0:
        case 1:
        case 2:
            if (DAT_007092c1 == '\0') {
                uVar12 = (uint)DAT_00732a3c & 0xffff;
                iVar7 = -1;
                if ((int)uVar12 < DAT_006d4980) {
                    do {
                        iVar7 = (int)(uVar12 * 0x23u);
                        iVar1 = (int)((uint)DAT_006d4988 + (uint)iVar7);
                        uVar9 = (ushort)uVar12;
                        if ((*(char *)((uint)DAT_006d4988 + 0x16u + (uint)iVar7) != *(char *)(iVar6 + 0x16)) &&
                            (*(char *)(iVar1 + 3) != '\0')) {
                            __builtin_memcpy(&v1b, (void *)(iVar1 + 0x1b), 4);
                            __builtin_memcpy(&v1f, (void *)(iVar1 + 0x1f), 4);
                            if ((v1b != -1) && (v1f != 0)) {
                                FUN_0040ed40((byte *)(v1f + 0x12), uVar9, uVar11);
                            }
                            cVar2 = *(char *)((uint)DAT_006d4988 + 2u + (uint)iVar7);
                            if ((cVar2 == '\x04') || (cVar2 == '\x03') || (cVar2 == '\x05')) {
                                __builtin_memcpy(&iVar1, (void *)((uint)DAT_006d4988 + (uint)iVar7 + 0x1fu), 4);
                                if (iVar1 != 0) {
                                    __builtin_memcpy(&iVar1, (void *)(iVar1 + 0xa7), 4);
                                    if (iVar1 != 0) {
                                        iVar8 = FUN_0042b150(iVar1);
                                        if (*(char *)(iVar8 + 0x34) != '\x1e' &&
                                            *(short *)(iVar1 + 0x68) == -1) {
                                            __builtin_memcpy(&vx, (void *)((uint)DAT_006d4988 + 0x1fu + (uint)iVar7), 4);
                                            FUN_0040ed40((byte *)(vx + 0x12), uVar9, uVar11);
                                        }
                                    }
                                }
                            }
                        }
                        uVar12 = (uint)(ushort)(uVar9 + 1);
                    } while ((int)uVar12 < DAT_006d4980);
                }
                uVar11 = ((uint)iVar7 & 0xffff0000u) | (ushort)DAT_00732a38;
                if ((int)((uint)DAT_00732a38 & 0xffff) < (int)((uint)DAT_00703c88 + (uint)DAT_00732a38)) {
                    do {
                        bVar3 = *(byte *)((uVar11 & 0xffff) * 0x23u + 3u + (uint)DAT_006d4988);
                        if ((99 < bVar3) && (bVar3 < 0x6a)) {
                            FUN_0040ed40(param_1, param_2, uVar11);
                        }
                        uVar9 = (ushort)((ushort)uVar11 + 1);
                        uVar11 = (uVar11 & 0xffff0000u) | uVar9;
                    } while ((int)(uint)uVar9 < (int)((uint)DAT_00703c88 + (uint)DAT_00732a38));
                    return;
                }
            }
            break;
        case 3:
        case 4:
        case 5:
            __builtin_memcpy(&iVar6, (void *)(v1f + 0xa7), 4);
            iVar7 = FUN_0042b150(iVar6);
            if ((*(char *)(iVar7 + 0x34) != '\x1e') && (*(short *)(iVar6 + 0x68) == -1)) {
                uVar12 = (uVar13 << 16) | (ushort)DAT_00732a3c;
                if ((int)((uint)DAT_00732a3c & 0xffff) < DAT_006d4980) {
                    do {
                        if (((ushort)uVar12 != param_2) &&
                            (iVar6 = (int)((uVar12 & 0xffff) * 0x23u + (uint)DAT_006d4988),
                             *(char *)(iVar6 + 3) != '\0')) {
                            doCall = 0;
                            __builtin_memcpy(&v1b, (void *)(iVar6 + 0x1b), 4);
                            if (v1b == -1) {
                                if (*(char *)(iVar6 + 2) != '\r') {
                                    pbVar14 = param_1;
                                    uVar15 = uVar11;
                                    uVar16 = uVar12;
                                    doCall = 1;
                                }
                            } else {
                                __builtin_memcpy(&v1f, (void *)(iVar6 + 0x1f), 4);
                                if (v1f != 0) {
                                    pbVar14 = (byte *)(v1f + 0x12);
                                    uVar15 = uVar12;
                                    uVar16 = uVar11;
                                    doCall = 1;
                                }
                            }
                            if (doCall) {
                                FUN_0040ed40(pbVar14, (ushort)uVar15, uVar16);
                            }
                        }
                        uVar9 = (ushort)((ushort)uVar12 + 1);
                        uVar12 = (uVar12 & 0xffff0000u) | uVar9;
                        if (DAT_006d4980 <= (int)(uint)uVar9) {
                            return;
                        }
                    } while (1);
                }
            }
            break;
        case 6:
        case 7:
            uVar11 = 0xffff0000u | (ushort)DAT_00732a3c;
            uVar4 = *(ushort *)((uint)DAT_006db798 + 4u + ((uint)param_2 - (uint)DAT_0071b704) * 8u);
            if ((int)((uint)DAT_00732a3c & 0xffff) < DAT_00732904) {
                do {
                    iVar6 = (int)((uVar11 & 0xffff) * 0x23u + (uint)DAT_006d4988);
                    uVar10 = (ushort)uVar11;
                    if ((*(byte *)(iVar6 + 3) != 0) &&
                        (((int)(uVar11 & 0xffff) < DAT_006d4980) ||
                         (((DAT_00525bf7[*(byte *)(iVar6 + 3)] != '\0') && (uVar10 != param_2)) &&
                          (__builtin_memcpy(&vx, (void *)(iVar6 + 0x1f), 4),
                           __builtin_memcpy(&u16, (void *)(vx + 0x83), 2),
                           u16 != uVar9))) &&
                        ((uVar9 != uVar10) && (*(char *)(iVar6 + 2) != '\r'))) {
                        bVar5 = 0;
                        uVar12 = (uint)uVar4;
                        if (((int)uVar12 < DAT_006d4980) && (DAT_00732a3c <= (int)uVar12)) {
                            __builtin_memcpy(&v1b, (void *)(uVar12 * 0x23u + 0x1bu + (uint)DAT_006d4988), 4);
                            if (v1b != -1) {
                                bVar5 = 1;
                            }
                        }
                        if (!bVar5) {
                            __builtin_memcpy(&v1b, (void *)((uint)uVar9 * 0x23u + 0x1bu + (uint)DAT_006d4988), 4);
                        }
                        if (bVar5 || (v1b != -1) || (uVar4 == uVar10)) {
                            FUN_0040ed40(param_1, param_2, uVar11);
                        }
                    }
                    uVar11 = (uVar11 & 0xffff0000u) | (ushort)(uVar10 + 1);
                } while ((int)(uint)(ushort)(uVar10 + 1) < DAT_00732904);
            }
            if (DAT_007092c1 == '\0') {
                uVar11 = (uVar13 << 16) | (ushort)DAT_00732a38;
                if ((int)((uint)DAT_00732a38 & 0xffff) < (int)((uint)DAT_00703c88 + (uint)DAT_00732a38)) {
                    do {
                        if (*(char *)((uVar11 & 0xffff) * 0x23u + 3u + (uint)DAT_006d4988) != '\0') {
                            FUN_0040ed40(param_1, param_2, uVar11);
                        }
                        uVar9 = (ushort)((ushort)uVar11 + 1);
                        uVar11 = (uVar11 & 0xffff0000u) | uVar9;
                    } while ((int)(uint)uVar9 < (int)((uint)DAT_00703c88 + (uint)DAT_00732a38));
                }
            }
        }
    } else {
        __builtin_memcpy(&v1f, (void *)(iVar6 + 0x1f), 4);
        __builtin_memcpy(&vx, (void *)(v1f + 0xa7), 4);
        __builtin_memcpy(&s16, (void *)(vx + 0x1df), 2);
        if ((s16 != 0) &&
            (uVar11 = ((uint)v1b & 0xffff0000u) | (ushort)DAT_00732a3c,
             (int)((uint)DAT_00732a3c & 0xffff) < DAT_006d4980)) {
            do {
                iVar7 = (int)((uVar11 & 0xffff) * 0x23u + (uint)DAT_006d4988);
                if ((*(char *)(iVar7 + 3) != '\0') &&
                    (((ushort)uVar11 != param_2) && (*(char *)(iVar7 + 2) != '\r') &&
                     (*(char *)(iVar6 + 0x16) != *(char *)(iVar7 + 0x16)))) {
                    FUN_0040ed40(param_1, param_2, uVar11);
                }
                uVar9 = (ushort)((ushort)uVar11 + 1);
                uVar11 = (uVar11 & 0xffff0000u) | uVar9;
            } while ((int)(uint)uVar9 < DAT_006d4980);
        }
        if (DAT_007092c1 == '\0') {
            uVar11 = (uVar13 << 16) | (ushort)DAT_00732a38;
            if ((int)((uint)DAT_00732a38 & 0xffff) < (int)((uint)DAT_00703c88 + (uint)DAT_00732a38)) {
                do {
                    if (*(char *)((uVar11 & 0xffff) * 0x23u + 3u + (uint)DAT_006d4988) != '\0') {
                        FUN_0040ed40(param_1, param_2, uVar11);
                    }
                    uVar9 = (ushort)((ushort)uVar11 + 1);
                    uVar11 = (uVar11 & 0xffff0000u) | uVar9;
                } while ((int)(uint)uVar9 < (int)((uint)DAT_00703c88 + (uint)DAT_00732a38));
                return;
            }
        }
    }
    return;
}

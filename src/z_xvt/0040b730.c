/* SPDX-License-Identifier: MIT */
/* z_xvt 0040b730 */
extern char DAT_0085e794;
extern int DAT_0071ecf4;
extern unsigned char DAT_0071b730[];
extern unsigned int DAT_006d4980;
extern int DAT_006d4988;
extern unsigned int DAT_00732a3c;
extern char DAT_006d4a44;
extern unsigned char DAT_00523a68[];
extern unsigned char DAT_00523a69;
extern unsigned char DAT_00523a6a;
extern unsigned char DAT_00523a79;
extern unsigned char DAT_00523a7d;
extern unsigned char DAT_00523a80[];
extern unsigned char bRam00523a81;
extern unsigned char DAT_00523a98[];
extern unsigned char bRam00523a97;
extern unsigned char DAT_0073b7e0[];

unsigned short FUN_004784d0(void);
void FUN_0040c4c0(unsigned int a, char b, char c, char d, short e);
void __stdcall RaiseException(DWORD, DWORD, DWORD, const DWORD *);

unsigned int __cdecl FUN_0040b730(int param_1, unsigned int param_2, int param_3, int param_4,
                                  unsigned int param_5, unsigned short param_6)
{
    unsigned char bVar1;
    unsigned short uVar2;
    unsigned int uVar3;
    unsigned int uVar4;
    int iVar5;
    unsigned int uVar6;
    unsigned int *puVar7;
    int iVar8;
    unsigned char *psVar9;
    unsigned char *pcVar10;
    short sVar11;
    unsigned int uVar12;
    char cVar13;
    unsigned int uStack_28;
    unsigned int uStack_24;
    unsigned int uStack_20;
    unsigned int uStack_1c;
    unsigned int auStack_18[6];
    unsigned char *pbase;
    unsigned char *pb;
    unsigned char *pp;
    int iTmp;

    if (DAT_0085e794 == '\0') {
        return 0;
    }
    if (param_1 == -1) {
        return 0;
    }
    if (param_1 != DAT_0071ecf4) {
        return 0;
    }
    __builtin_memcpy(&iVar8, &DAT_0071b730[(unsigned int)param_1 * 0x5bdU], 4);
    if (iVar8 == -1) {
        return 0;
    }
    if (param_6 != 0xffff) {
        if (DAT_0085e794 == '\x01') {
            param_6 = param_6 >> 1;
        }
        uVar2 = FUN_004784d0();
        if (param_6 <= uVar2) {
            return 0;
        }
    }
    pbase = (unsigned char *)(unsigned int)DAT_006d4988;
    if (param_2 == 0xffffffff) {
        uStack_24 = 0;
        if (DAT_00732a3c < DAT_006d4980) {
            puVar7 = auStack_18;
            pcVar10 = pbase + (DAT_00732a3c * 0x23U + 3U);
            uVar12 = DAT_00732a3c;
            do {
                char *pc;
                int iv;
                __builtin_memcpy(&pc, pcVar10 + 0x1c, 4);
                __builtin_memcpy(&iv, pcVar10 + 0x18, 4);
                if ((((*pcVar10 != 0) && (*pc == '\0')) &&
                     (iv != param_1)) &&
                    (pbase[(unsigned int)iVar8 * 0x23U + 0x16U] == pcVar10[0x13])) {
                    *puVar7 = uVar12;
                    puVar7 = puVar7 + 1;
                    uStack_24 = uStack_24 + 1;
                }
                pcVar10 = pcVar10 + 0x23;
                uVar12 = uVar12 + 1;
            } while (uVar12 < DAT_006d4980);
        }
        if (uStack_24 == 0) {
            if (param_3 == 0xc) {
                FUN_0040c4c0(0x25, '\0', '\0', '\0', -1);
                FUN_0040c4c0(0x25, '\0', '\0', '\0', -1);
            }
            return 0;
        }
        uVar2 = FUN_004784d0();
        if ((unsigned short)(uVar2 / (unsigned short)uStack_24) == 0) {
            uVar12 = 0;
        } else {
            uVar12 = (unsigned int)uVar2 % (unsigned int)(unsigned short)(uVar2 / (unsigned short)uStack_24);
        }
        param_2 = auStack_18[uVar12];
        if (param_5 == param_2) {
            uVar3 = uVar12 + 1;
            if (uStack_24 <= uVar3) {
                uVar3 = 0;
            }
            if (uVar3 == uVar12) {
                return 0;
            }
        }
    } else {
        if ((int)DAT_006d4980 <= (int)param_2) {
            return 0;
        }
        if ((param_3 != 1) &&
            (pbase[(unsigned int)iVar8 * 0x23U + 0x16U] !=
             pbase[param_2 * 0x23U + 0x16U])) {
            return 0;
        }
    }
    __builtin_memcpy(&iTmp, pbase + (param_2 * 0x23U + 0x1fU), 4);
    __builtin_memcpy(&pp, (unsigned char *)(unsigned int)iTmp + 0xa7, 4);
    __builtin_memcpy(&uStack_20, pp, 4);
    if (6 < uStack_20) {
        uStack_20 = 0;
    }
    uStack_1c = 0;
    auStack_18[0] = (unsigned int)pp[0xb0] * 0x61U;
    iVar8 = (int)(auStack_18[0] + 0x6fU);
    if ((param_5 == 0xffffffff) || ((int)DAT_006d4980 <= (int)param_5)) {
        sVar11 = -1;
    } else {
        psVar9 = pbase + param_5 * 0x23U;
        __builtin_memcpy(&sVar11, psVar9, 2);
        __builtin_memcpy(&iTmp, psVar9 + 0x1f, 4);
        __builtin_memcpy(&pb, (unsigned char *)(unsigned int)iTmp + 0xa7, 4);
        uStack_1c = (unsigned int)pb[0xb0];
    }
    uVar12 = (unsigned int)DAT_00523a68[param_3];
    cVar13 = (char)param_3;
    if (param_4 == -1) {
        switch (param_3) {
        case 3:
        case 5:
            if (DAT_006d4a44 != '\0') {
                return 0;
            }
            auStack_18[0] = (unsigned int)DAT_00523a80[param_3];
            uVar6 = FUN_004784d0();
            if ((auStack_18[0] & 0xffff) == 0) {
                RaiseException(0xC0000094UL, 0, 0, NULL);
            }
            uVar3 = (uVar6 & 0xffff) / (auStack_18[0] & 0xffff);
            if ((short)uVar3 == 0) {
                uVar3 = 0;
            } else {
                uVar3 = (uVar6 & 0xffff) % uVar3;
            }
            if ((DAT_00523a98[param_3] != 0) &&
                (iVar5 = (int)(uStack_1c * 0x61U + uVar12), uVar6 = auStack_18[0],
                 DAT_00523a98[param_3] <= DAT_0073b7e0[(unsigned int)iVar5 + uVar3])) {
                do {
                    if (uVar6 == 0) {
                        uVar3 = 0xffffffff;
                        break;
                    }
                    uVar3 = uVar3 + 1;
                    if ((int)auStack_18[0] <= (int)uVar3) {
                        uVar3 = 0;
                    }
                    uVar6 = uVar6 - 1;
                } while (DAT_00523a98[param_3] <= DAT_0073b7e0[(unsigned int)iVar5 + uVar3]);
            }
            if (uVar3 == 0xffffffff) {
                return 0;
            }
            if (uStack_20 != 0) {
                pcVar10 = &DAT_0073b7e0[uVar3 + uStack_1c * 0x61U + uVar12];
                *pcVar10 = (unsigned char)(*pcVar10 + 1);
                FUN_0040c4c0(DAT_00523a6a + uStack_20 + (unsigned int)iVar8 - 1U, '\x01', '\x02', '\0', sVar11);
            }
            FUN_0040c4c0(uVar12 + (unsigned int)iVar8 + uVar3, '\x01', cVar13, '\x01', sVar11);
            return 1;
        case 6:
            if (DAT_006d4a44 != '\0') {
                return 0;
            }
            uVar6 = (unsigned int)DAT_00523a80[param_3];
            uVar4 = FUN_004784d0();
            if (uVar6 == 0) {
                RaiseException(0xC0000094UL, 0, 0, NULL);
            }
            uVar3 = (uVar4 & 0xffff) / uVar6;
            if ((short)uVar3 == 0) {
                uVar3 = 0;
            } else {
                uVar3 = (uVar4 & 0xffff) % uVar3;
            }
            if ((DAT_00523a98[param_3] != 0) &&
                (auStack_18[0] = uStack_1c * 0x61U, uStack_28 = uVar6,
                 DAT_00523a98[param_3] <=
                     DAT_0073b7e0[auStack_18[0] + uVar12 + uVar3])) {
                do {
                    if (uStack_28 == 0) {
                        uVar3 = 0xffffffff;
                        break;
                    }
                    uVar3 = uVar3 + 1;
                    if ((int)uVar6 <= (int)uVar3) {
                        uVar3 = 0;
                    }
                    uStack_28 = uStack_28 - 1;
                } while (DAT_00523a98[param_3] <=
                         DAT_0073b7e0[auStack_18[0] + uVar12 + uVar3]);
            }
            if (uVar3 == 0xffffffff) {
                return 0;
            }
            iVar5 = (int)(uStack_1c * 0x61U + uVar12);
            DAT_0073b7e0[uVar3 + (unsigned int)iVar5] =
                (unsigned char)(DAT_0073b7e0[uVar3 + (unsigned int)iVar5] + 1);
            FUN_0040c4c0(uVar12 + (unsigned int)iVar8 + uVar3, '\x01', cVar13, '\0', sVar11);
            return 1;
        case 9:
        case 10:
        case 0x15:
            uVar6 = (unsigned int)DAT_00523a80[param_3];
            uVar4 = FUN_004784d0();
            if (uVar6 == 0) {
                RaiseException(0xC0000094UL, 0, 0, NULL);
            }
            uVar3 = (uVar4 & 0xffff) / uVar6;
            if ((short)uVar3 == 0) {
                uVar3 = 0;
            } else {
                uVar3 = (uVar4 & 0xffff) % uVar3;
            }
            if ((DAT_00523a98[param_3] != 0) &&
                (auStack_18[0] = uStack_1c * 0x61U, uStack_28 = uVar6,
                 DAT_00523a98[param_3] <=
                     DAT_0073b7e0[auStack_18[0] + uVar12 + uVar3])) {
                do {
                    if (uStack_28 == 0) {
                        uVar3 = 0xffffffff;
                        break;
                    }
                    uVar3 = uVar3 + 1;
                    if ((int)uVar6 <= (int)uVar3) {
                        uVar3 = 0;
                    }
                    uStack_28 = uStack_28 - 1;
                } while (DAT_00523a98[param_3] <=
                         DAT_0073b7e0[auStack_18[0] + uVar12 + uVar3]);
            }
            if (uVar3 == 0xffffffff) {
                return 0;
            }
            iVar5 = (int)(uStack_1c * 0x61U + uVar12);
            DAT_0073b7e0[uVar3 + (unsigned int)iVar5] =
                (unsigned char)(DAT_0073b7e0[uVar3 + (unsigned int)iVar5] + 1);
            FUN_0040c4c0(uVar12 + (unsigned int)iVar8 + uVar3, '\x01', cVar13, '\0', sVar11);
            return 1;
        case 0xc:
            uVar2 = FUN_004784d0();
            if ((unsigned short)(uVar2 >> 2) == 0) {
                uVar3 = 0;
            } else {
                uVar3 = (unsigned int)uVar2 % (unsigned int)(unsigned short)(uVar2 >> 2);
            }
            FUN_0040c4c0(uVar12 + (unsigned int)iVar8 + uVar3, '\x01', cVar13, '\0', sVar11);
            return 1;
        case 0x10:
            auStack_18[0] = (unsigned int)DAT_00523a80[param_3];
            uVar6 = FUN_004784d0();
            if ((auStack_18[0] & 0xffff) == 0) {
                RaiseException(0xC0000094UL, 0, 0, NULL);
            }
            uVar3 = (uVar6 & 0xffff) / (auStack_18[0] & 0xffff);
            if ((short)uVar3 == 0) {
                uVar3 = 0;
            } else {
                uVar3 = (uVar6 & 0xffff) % uVar3;
            }
            if ((DAT_00523a98[param_3] != 0) &&
                (iVar5 = (int)(uStack_1c * 0x61U + uVar12), uVar6 = auStack_18[0],
                 DAT_00523a98[param_3] <= DAT_0073b7e0[(unsigned int)iVar5 + uVar3])) {
                do {
                    if (uVar6 == 0) {
                        uVar3 = 0xffffffff;
                        break;
                    }
                    uVar3 = uVar3 + 1;
                    if ((int)auStack_18[0] <= (int)uVar3) {
                        uVar3 = 0;
                    }
                    uVar6 = uVar6 - 1;
                } while (DAT_00523a98[param_3] <= DAT_0073b7e0[(unsigned int)iVar5 + uVar3]);
            }
            if (uVar3 == 0xffffffff) {
                return 0;
            }
            iVar5 = (int)(uStack_1c * 0x61U + uVar12);
            DAT_0073b7e0[uVar3 + (unsigned int)iVar5] =
                (unsigned char)(DAT_0073b7e0[uVar3 + (unsigned int)iVar5] + 1);
            FUN_0040c4c0(uVar12 + (unsigned int)iVar8 + uVar3, '\x01', cVar13, '\0', sVar11);
            FUN_0040c4c0(DAT_00523a79 + uStack_1c + (unsigned int)iVar8, '\x01', '\x11', '\x01', sVar11);
            return 1;
        }
    } else {
        if (param_3 == 1) {
            if (uStack_20 != 0) {
                FUN_0040c4c0(DAT_00523a68[0] + uStack_20 + (unsigned int)iVar8 - 1U, '\x01', '\0', '\0', sVar11);
            }
            uVar2 = FUN_004784d0();
            if (uVar2 < 0x5555) {
                FUN_0040c4c0((unsigned int)DAT_00523a69 + (unsigned int)iVar8, '\x01', '\x01', '\x01', sVar11);
            }
            if (param_4 < (int)(unsigned int)bRam00523a81) {
                FUN_0040c4c0(uVar12 + (unsigned int)iVar8 + (unsigned int)param_4, '\x01', '\x01', '\x02', sVar11);
                return 1;
            }
            return 0;
        }
        if (param_3 != 0x17) {
            if (param_4 < (int)(unsigned int)DAT_00523a80[param_3]) {
                FUN_0040c4c0(uVar12 + (unsigned int)iVar8 + (unsigned int)param_4, '\x01', cVar13, '\0', sVar11);
                return 1;
            }
            return 0;
        }
        uVar2 = FUN_004784d0();
        bVar1 = bRam00523a97;
        if (uVar2 < 0x8000) {
            uVar6 = FUN_004784d0();
            if ((unsigned int)bVar1 == 0) {
                RaiseException(0xC0000094UL, 0, 0, NULL);
            }
            uVar3 = (uVar6 & 0xffff) / (unsigned int)bVar1;
            if ((short)uVar3 == 0) {
                uVar3 = 0;
            } else {
                uVar3 = (uVar6 & 0xffff) % uVar3;
            }
            FUN_0040c4c0((unsigned int)DAT_00523a7d + (unsigned int)iVar8 + uVar3, '\x01', '\x15', '\0', sVar11);
        }
        if ((int)(unsigned int)bRam00523a97 <= param_4) {
            return 0;
        }
        FUN_0040c4c0((unsigned int)iVar8 + uVar12 + (unsigned int)param_4, '\x01', '\x17', '\x01', sVar11);
        pcVar10 = &DAT_0073b7e0[(unsigned int)param_4 + uVar12 + auStack_18[0]];
        *pcVar10 = (unsigned char)(*pcVar10 + 1);
    }
    return 1;
}

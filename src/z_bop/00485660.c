/* SPDX-License-Identifier: MIT */
/* z_bop 00485660 */
extern ushort DAT_00a07cec;
extern int DAT_009a1fe0;
extern unsigned char *DAT_009a1fe8;
extern unsigned char DAT_009e9670[];
extern unsigned char DAT_009e967c[];
extern unsigned char DAT_009d8c88[];
extern unsigned char DAT_009fd458[];
extern uint DAT_009a8c00;
extern int DAT_009d1308;
extern int DAT_00a07ce8;

uint __cdecl FUN_00484f80(uint param_1);
void __cdecl FUN_00485000(int param_1, uint param_2);

uint __cdecl FUN_00485660(int param_1, uint param_2)
{
    char cVar1;
    unsigned char *iVar2;
    unsigned char *pEnt;
    unsigned char *pEnt2;
    uint uVar3;
    ushort uVar4;
    uint uVar5;
    int bVar6;
    uint local_14;
    uint local_c;
    uint tmp32;
    ushort tmp16;

    local_14 = 0xffffffff;
    local_c = 0xffff;
    uVar4 = DAT_00a07cec;
    if ((short)DAT_00a07cec < DAT_009a1fe0) {
        do {
            uVar5 = (uint)(int)(short)uVar4;
            iVar2 = DAT_009a1fe8 + uVar5 * 0x23u;
            if (iVar2[3] != 0) {
                __builtin_memcpy(&tmp32, DAT_009e9670 + (uint)param_1 * 0x5bdu, sizeof(tmp32));
                if ((tmp32 != uVar5) && (param_2 != uVar5)) {
                    __builtin_memcpy(&tmp16, DAT_009e967c + (uint)param_1 * 0x5bdu, sizeof(tmp16));
                    uVar3 = (uint)tmp16;
                    __builtin_memcpy(&pEnt, iVar2 + 0x1f, sizeof(pEnt));
                    if ((uint)pEnt[0x89] != uVar3) {
                        if ((uint)DAT_009d8c88[(uint)DAT_009a1fe8[(uint)uVar4 * 0x23u + 0x16u] * 0x566u] == uVar3) {
                            bVar6 = 0;
                        } else {
                            bVar6 = DAT_009fd458[(uint)DAT_009d8c88[(uint)DAT_009a1fe8[(uint)uVar4 * 0x23u + 0x16u] * 0x566u] + uVar3 * 0x1e5u] == 0;
                        }
                        if (bVar6) {
                            cVar1 = (char)iVar2[2];
                            if ((cVar1 != '\r') && (cVar1 != '\x04') && (cVar1 != '\x03') && (cVar1 != '\x05')) {
                                uVar3 = FUN_00484f80((uint)uVar4);
                                if ((char)uVar3 == '\0') {
                                    __builtin_memcpy(&pEnt2, DAT_009a1fe8 + 0x1fu + uVar5 * 0x23u, sizeof(pEnt2));
                                    __builtin_memcpy(&iVar2, pEnt2 + 0xa9, sizeof(iVar2));
                                    __builtin_memcpy(&tmp16, iVar2 + 0x1e5, sizeof(tmp16));
                                    if (tmp16 != 0) {
                                        cVar1 = (char)iVar2[7];
                                        if ((cVar1 == '\0') || (cVar1 == '\x06')) {
                                            FUN_00485000(param_1, uVar5);
                                            if (DAT_009a8c00 < local_14) {
                                                local_14 = DAT_009a8c00;
                                                local_c = uVar5;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            uVar4 = (ushort)(uVar4 + 1);
        } while ((short)uVar4 < DAT_009a1fe0);
    }
    uVar4 = (ushort)DAT_00a07ce8;
    if ((int)(short)uVar4 < (int)((uint)DAT_009d1308 + (uint)DAT_00a07ce8)) {
        do {
            uVar5 = (uint)(int)(short)uVar4;
            iVar2 = DAT_009a1fe8 + uVar5 * 0x23u;
            if ((iVar2[3] != 0) && (iVar2[2] == 8) && (param_2 != uVar5)) {
                __builtin_memcpy(&tmp16, iVar2 + 0x17, sizeof(tmp16));
                if (tmp16 != 0) {
                    __builtin_memcpy(&tmp16, DAT_009e967c + (uint)param_1 * 0x5bdu, sizeof(tmp16));
                    uVar3 = (uint)tmp16;
                    if (uVar3 == (uint)DAT_009d8c88[(uint)DAT_009a1fe8[(uint)uVar4 * 0x23u + 0x16u] * 0x566u]) {
                        bVar6 = 0;
                    } else {
                        bVar6 = DAT_009fd458[(uint)DAT_009d8c88[(uint)DAT_009a1fe8[(uint)uVar4 * 0x23u + 0x16u] * 0x566u] + uVar3 * 0x1e5u] == 0;
                    }
                    if (bVar6) {
                        FUN_00485000(param_1, uVar5);
                        if (DAT_009a8c00 < local_14) {
                            local_14 = DAT_009a8c00;
                            local_c = uVar5;
                        }
                    }
                }
            }
            uVar4 = (ushort)(uVar4 + 1);
        } while ((int)(short)uVar4 < (int)((uint)DAT_009d1308 + (uint)DAT_00a07ce8));
    }
    return local_c;
}

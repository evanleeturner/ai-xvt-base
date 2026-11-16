/* SPDX-License-Identifier: MIT */
/* z_bop 0043f140 */
extern unsigned char DAT_009e9a77[];
extern int DAT_009e9670;
extern int DAT_009ecc34;
extern int DAT_009a1fe8;
extern ushort DAT_00a08374;
extern ushort DAT_00a08c74;

void __cdecl FUN_00440760(uint param_1, uint param_2);

void __stdcall FUN_0043f140(void)
{
    char cVar1;
    int iVar2;
    int bVar3;
    uint uVar4;
    ushort uVar5;
    uint uVar6;
    ushort uVar7;
    int iVar8;
    int iVar9;
    ushort uVar10;
    ushort uVar11;

    if (DAT_009e9a77[(uint)DAT_009ecc34 * 0x5bd] != '\0') {
        if (DAT_009e9a77[(uint)DAT_009ecc34 * 0x5bd] == '\x13') {
            uVar7 = 1;
            uVar6 = 0;
            do {
                if (uVar7 < 9) {
                    if (((uVar7 != 8) && (uVar7 != 2)) && (uVar7 != 4)) goto LAB_0043f2fc;
                }
                else {
                    if ((uVar7 == 0x10) || (uVar7 == 0x1000)) {
                        __builtin_memcpy(&iVar2, (char *)&DAT_009e9670 + (uint)DAT_009ecc34 * 0x5bd, 4);
                        if ((iVar2 == -1) ||
                            ((((cVar1 = *(char *)((uint)iVar2 * 0x23 + 3 + (uint)DAT_009a1fe8), cVar1 != '\x01' &&
                               (cVar1 != '\x02')) && (cVar1 != '\x03')) &&
                              ((cVar1 != '\x0e' && (cVar1 != '\x04')))))) {
                            bVar3 = 0;
                        }
                        else {
                            bVar3 = 1;
                        }
                        if (bVar3) goto LAB_0043f371;
                    }
LAB_0043f2fc:
                    __builtin_memcpy(&iVar2, (char *)&DAT_009e9670 + (uint)DAT_009ecc34 * 0x5bd, 4);
                    __builtin_memcpy(&iVar8, (char *)(uint)((uint)iVar2 * 0x23 + 0x1f + (uint)DAT_009a1fe8), 4);
                    __builtin_memcpy(&iVar2, (char *)(uint)((uint)iVar8 + 0xa9), 4);
                    __builtin_memcpy(&uVar10, (char *)(uint)((uint)iVar2 + 0x1df), 2);
                    if ((uVar10 & uVar7) != 0) {
                        __builtin_memcpy(&uVar11, (char *)(uint)((uint)iVar2 + 0x1e1), 2);
                        FUN_00440760(uVar6 + DAT_00a08374 + 0x45,
                                     (uint)(-(ushort)((uVar11 & uVar7) == 0) & 0xd));
                    }
                }
LAB_0043f371:
                uVar7 = (ushort)(uVar7 * 2);
                uVar5 = (ushort)((short)uVar6 + 1);
                uVar6 = (uint)uVar5;
            } while (uVar5 < 0xd);
            DAT_00a08c74 = 0xffff;
        }
        return;
    }
    uVar7 = 1;
    uVar6 = 0;
    do {
        __builtin_memcpy(&iVar2, (char *)&DAT_009e9670 + (uint)DAT_009ecc34 * 0x5bd, 4);
        if ((iVar2 == -1) ||
            ((((cVar1 = *(char *)((uint)DAT_009a1fe8 + 3 + (uint)iVar2 * 0x23), cVar1 != '\x01') && (cVar1 != '\x02'))
              && (cVar1 != '\x03')) && ((cVar1 != '\x0e' && (cVar1 != '\x04'))))) {
            bVar3 = 0;
        }
        else {
            bVar3 = 1;
        }
        __builtin_memcpy(&iVar8, (char *)(uint)((uint)DAT_009a1fe8 + 0x1f + (uint)iVar2 * 0x23), 4);
        __builtin_memcpy(&iVar9, (char *)(uint)((uint)iVar8 + 0xa9), 4);
        __builtin_memcpy(&uVar11, (char *)(uint)((uint)iVar9 + 0x1e1), 2);
        if (bVar3) {
            if ((uVar11 & uVar7) == 0) {
                uVar4 = 0;
                goto LAB_0043f217;
            }
        }
        else {
            uVar4 = (uint)(-(ushort)((uVar11 & uVar7) == 0) & 0xd);
LAB_0043f217:
            __builtin_memcpy(&iVar8, (char *)(uint)((uint)DAT_009a1fe8 + 0x1f + (uint)iVar2 * 0x23), 4);
            __builtin_memcpy(&iVar9, (char *)(uint)((uint)iVar8 + 0xa9), 4);
            __builtin_memcpy(&uVar10, (char *)(uint)((uint)iVar9 + 0x1df), 2);
            if ((uVar10 & uVar7) != 0) {
                FUN_00440760(uVar6 + 0x45, uVar4);
            }
        }
        uVar7 = (ushort)(uVar7 * 2);
        uVar5 = (ushort)((short)uVar6 + 1);
        uVar6 = (uint)uVar5;
        if (0xc < uVar5) {
            DAT_00a08c74 = 0xffff;
            return;
        }
    } while (1);
}

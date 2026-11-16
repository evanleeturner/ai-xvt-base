/* SPDX-License-Identifier: MIT */
/* z_bop 004823e0 */
extern uint DAT_009d1308;
extern uint DAT_00a07ce8;
extern int DAT_009e9640;
extern int DAT_00a00854;
extern int DAT_009a1fe8;
extern int DAT_00a08104;
extern byte DAT_009e9670[];
extern byte DAT_009e967c[];
extern byte DAT_00521f2e[];
extern byte DAT_009d8c88[];
extern byte DAT_009fd458[];

uint __cdecl FUN_00484f80(ushort param_1);

uint __cdecl FUN_004823e0(ushort param_1, short param_2, int param_3, undefined4 param_4, uint param_5)
{
    int iVar1;
    short sVar2;
    byte bVar3;
    char cVar4;
    int iVar5;
    int iVar6;
    uint pcVar7;
    ushort uVar8;
    uint uVar9;
    int bVar10;
    short local_2;
    int local_i32;
    int local_i32b;
    ushort local_u16;
    byte *local_ptr;

    local_2 = (short)(DAT_009d1308 + DAT_00a07ce8);
    uVar9 = (uint)param_1;
LAB_00482401:
    do {
        do {
            sVar2 = (short)(local_2 - 1);
            if (local_2 == 0) {
                return 0xffff;
            }
            uVar8 = (ushort)((short)uVar9 + param_2);
            uVar9 = (uint)uVar8;
            if (uVar8 < 0x8000) {
                if (DAT_009d1308 + DAT_00a07ce8 == (uint)uVar8) {
                    uVar9 = 0;
                }
            } else {
                uVar9 = DAT_009d1308 - 1 + (DAT_00a07ce8 & 0xffff);
            }
            iVar6 = (int)((uint)param_3 * 0x5bd);
            pcVar7 = uVar9 & 0xffff;
            local_2 = sVar2;
            __builtin_memcpy(&local_i32, DAT_009e9670 + iVar6, 4);
        } while (((uint)local_i32 == pcVar7) ||
                 (((((param_5 & 4) != 0) && (DAT_009e9640 <= (int)pcVar7)) && ((int)pcVar7 < DAT_00a00854)) ||
                  (((param_5 & 2) != 0) && (DAT_00a00854 <= (int)pcVar7))));
        iVar5 = (int)(pcVar7 * 0x23);
        iVar1 = (int)((uint)DAT_009a1fe8 + (uint)iVar5);
        bVar3 = *(byte *)(DAT_009a1fe8 + 3 + iVar5);
    } while ((bVar3 == 0) ||
             ((DAT_00521f2e[(uint)bVar3 * 0x1a] & 1) == 0 ||
              (((param_5 & 1) != 0) && (*(char *)(iVar1 + 2) == '\b'))));

    __builtin_memcpy(&local_i32, (byte *)iVar1 + 0x1f, 4);
    if (local_i32 == 0) {
        bVar3 = DAT_009d8c88[(uint)*(byte *)(iVar1 + 0x16) * 0x566];
    } else {
        bVar3 = *(byte *)(local_i32 + 0x89);
    }
    pcVar7 = (uint)bVar3;
    __builtin_memcpy(&local_u16, DAT_009e967c + iVar6, 2);
    switch (param_4) {
    case 1:
        if ((uint)local_u16 != pcVar7) goto LAB_00482401;
        break;
    case 2:
        if ((uint)local_u16 == pcVar7) {
            bVar10 = 0;
        } else {
            if (DAT_009fd458[pcVar7 + (uint)local_u16 * 0x1e5] == 0) {
                bVar10 = 1;
            } else {
                bVar10 = 0;
            }
        }
        if (bVar10) {
            goto LAB_00482401;
        }
        break;
    case 3:
        if ((uint)local_u16 == pcVar7) {
            bVar10 = 0;
        } else {
            if (DAT_009fd458[pcVar7 + (uint)local_u16 * 0x1e5] == 0) {
                bVar10 = 1;
            } else {
                bVar10 = 0;
            }
        }
        if (!bVar10) {
            goto LAB_00482401;
        }
        break;
    case 4:
        __builtin_memcpy(&local_i32b, (byte *)iVar1 + 0x1b, 4);
        if (local_i32b == -1) goto LAB_00482401;
    }
    if (local_i32 == 0) {
        return (uint)(ushort)uVar9;
    }
    if ((*(char *)(iVar1 + 2) != '\r') &&
        ((char)FUN_00484f80((ushort)uVar9) == '\0')) {
        __builtin_memcpy(&local_ptr, (byte *)(DAT_009a1fe8 + 0x1f + iVar5), 4);
        if (*local_ptr != 0) {
            return (uint)(ushort)uVar9;
        }
        __builtin_memcpy(&DAT_00a08104, local_ptr + 0xa9, 4);
        cVar4 = *(char *)(DAT_00a08104 + 7);
        if ((cVar4 != '\x03') && (cVar4 != '\x04')) {
            return (uint)(ushort)uVar9;
        }
    }
    goto LAB_00482401;
}

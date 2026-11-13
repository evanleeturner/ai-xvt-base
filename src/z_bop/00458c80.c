/* SPDX-License-Identifier: MIT */
/* z_bop 00458c80 */
extern uint DAT_0099f970;
extern uint DAT_0099f974;
extern uint DAT_0099f978;
extern ushort DAT_0099f97c;
extern ushort DAT_0099f97e;
extern ushort DAT_0099f980;
extern ushort DAT_009a8d40;
extern uint DAT_009a1fe8;
extern uint DAT_009a20cc;

extern uint __stdcall FUN_004597f0(void);

uint __cdecl FUN_00458c80(ushort param_1, short param_2, uchar param_3)
{
    uint uVar4;
    short sVar3;
    int iVar2;

    uVar4 = FUN_004597f0();
    sVar3 = (short)uVar4;
    if (sVar3 != -1) {
        iVar2 = (uVar4 & 0xffff) * 0x23;
        *(uint *)(DAT_009a1fe8 + 4 + iVar2) = DAT_0099f970;
        *(uint *)(DAT_009a1fe8 + 8 + iVar2) = DAT_0099f974;
        *(uint *)(DAT_009a1fe8 + 0xc + iVar2) = DAT_0099f978;
        *(ushort *)(DAT_009a1fe8 + 0x10 + iVar2) = DAT_0099f97c;
        *(ushort *)(DAT_009a1fe8 + 0x12 + iVar2) = DAT_0099f97e;
        *(ushort *)(DAT_009a1fe8 + 0x14 + iVar2) = DAT_0099f980;
        *(uint *)(DAT_009a1fe8 + 4 + iVar2) = *(uint *)(DAT_009a1fe8 + 4 + iVar2) << 8;
        *(uint *)(DAT_009a1fe8 + 8 + iVar2) = *(uint *)(DAT_009a1fe8 + 8 + iVar2) << 8;
        *(uint *)(DAT_009a1fe8 + 0xc + iVar2) = *(uint *)(DAT_009a1fe8 + 0xc + iVar2) << 8;
        *(ushort *)(DAT_009a1fe8 + 0x10 + iVar2) = *(ushort *)(DAT_009a1fe8 + 0x10 + iVar2) << 8;
        *(ushort *)(DAT_009a1fe8 + 0x12 + iVar2) = *(ushort *)(DAT_009a1fe8 + 0x12 + iVar2) << 8;
        *(ushort *)(DAT_009a1fe8 + 0x14 + iVar2) = *(ushort *)(DAT_009a1fe8 + 0x14 + iVar2) << 8;
        *(ushort *)(DAT_009a1fe8 + iVar2) = DAT_009a8d40;
        DAT_009a8d40 = DAT_009a8d40 + 1;
        *(uchar *)(DAT_009a1fe8 + 0x16 + iVar2) = (uchar)param_1;
        *(uchar *)(DAT_009a1fe8 + 2 + iVar2) = (uchar)param_2;
        *(uchar *)(DAT_009a1fe8 + 3 + iVar2) = param_3;
        ushort tmp_0x17 = 0x3ff;
        __builtin_memcpy((void *)(DAT_009a1fe8 + 0x17 + iVar2), &tmp_0x17, sizeof(ushort));
        *(uchar *)(DAT_009a1fe8 + 0x19 + iVar2) = 0;
        if (param_2 == 8) {
            *(uchar *)(DAT_009a1fe8 + 0x1a + iVar2) =
                ((uchar)*(ushort *)((uint)&DAT_009a20cc + (uint)param_1 * 0x126) & 7) * 0x1d;
        }
        else {
            *(uchar *)(DAT_009a1fe8 + 0x1a + iVar2) = 0;
        }
        uVar4 = (uint)param_1 * 0x93;
        *(ushort *)((uint)&DAT_009a20cc + (uint)param_1 * 0x126) =
            *(ushort *)((uint)&DAT_009a20cc + (uint)param_1 * 0x126) + 1;
    }
    return (uVar4 & 0xFFFF0000) | (uint)(ushort)sVar3;
}

/* SPDX-License-Identifier: MIT */
/* z_bop 004aaba0 */
extern int DAT_00527f48[2];
extern unsigned int DAT_00527f50;
extern unsigned int DAT_00527f54;
extern unsigned int DAT_00527f58;
extern unsigned int DAT_00527f5c;
extern unsigned int DAT_00527f60;
extern unsigned int DAT_00527f64;
extern int DAT_00527f68;
extern int DAT_00527f6c;
extern int DAT_00527f70;
extern unsigned int DAT_00527f74;
extern unsigned int DAT_00622ca8[2];
extern int DAT_00622cb4;
extern int DAT_00622cb8;
extern int DAT_00622ca0;

unsigned int __cdecl FUN_004d6440(int param_1);
void __stdcall RaiseException(DWORD, DWORD, DWORD, const DWORD *);

void __cdecl FUN_004aaba0(int param_1, unsigned int *param_2, unsigned int *param_3, unsigned int *param_4, unsigned int *param_5)
{
    unsigned int uVar1;
    unsigned int UVar2;
    unsigned int MVar3;
    unsigned int uVar4;
    int iVar5;
    unsigned int *puVar6;
    unsigned int local_1c8[13];
    unsigned int local_194[101];

    uVar1 = (unsigned int)(param_1 != 0);
    if (DAT_00527f48[uVar1] == 0) {
        DAT_00527f48[uVar1] = 1;
        puVar6 = local_194;
        for (iVar5 = 0x65; iVar5 != 0; iVar5 = iVar5 - 1) {
            *puVar6 = 0;
            puVar6 = puVar6 + 1;
        }
        UVar2 = FUN_004d6440(0);
        MVar3 = joyGetDevCapsA(UVar2, (void *)local_194, 0x194);
        if (MVar3 == 0) {
            uVar4 = FUN_004d6440(0);
            DAT_00622ca8[uVar1] = uVar4;
            DAT_00527f74 = (local_194[24] & 0x10) >> 4;
            DAT_00527f50 = local_194[10] - local_194[9];
            DAT_00527f54 = local_194[12] - local_194[11];
            DAT_00527f58 = local_194[14] - local_194[13];
            DAT_00527f5c = (unsigned int)((int)DAT_00527f50 >> 1) - local_194[10];
            DAT_00527f60 = (unsigned int)((int)DAT_00527f54 >> 1) - local_194[12];
            DAT_00527f64 = (unsigned int)((int)DAT_00527f58 >> 1) - local_194[14];
            DAT_00527f68 = (int)DAT_00527f50 / 0x14;
            DAT_00527f6c = (int)DAT_00527f54 / 0x14;
            DAT_00527f70 = (int)DAT_00527f58 / 0x14;
            DAT_00622cb4 = (int)(local_194[9] + (unsigned int)((int)DAT_00527f50 >> 1));
            DAT_00622cb8 = (int)(local_194[11] + (unsigned int)((int)DAT_00527f54 >> 1));
            DAT_00622ca0 = (int)(local_194[13] + (unsigned int)((int)DAT_00527f58 >> 1));
        } else {
            UVar2 = FUN_004d6440(1);
            MVar3 = joyGetDevCapsA(UVar2, (void *)local_194, 0x194);
            if (MVar3 == 0) {
                uVar4 = FUN_004d6440(1);
                DAT_00622ca8[uVar1] = uVar4;
                DAT_00527f74 = (local_194[24] & 0x10) >> 4;
                DAT_00527f50 = local_194[10] - local_194[9];
                DAT_00527f54 = local_194[12] - local_194[11];
                DAT_00527f58 = local_194[14] - local_194[13];
                DAT_00527f5c = (unsigned int)((int)DAT_00527f50 >> 1) - local_194[10];
                DAT_00527f60 = (unsigned int)((int)DAT_00527f54 >> 1) - local_194[12];
                DAT_00527f64 = (unsigned int)((int)DAT_00527f58 >> 1) - local_194[14];
                DAT_00527f68 = (int)DAT_00527f50 / 0x14;
                DAT_00527f6c = (int)DAT_00527f54 / 0x14;
                DAT_00527f70 = (int)DAT_00527f58 / 0x14;
                DAT_00622cb4 = (int)(local_194[9] + (unsigned int)((int)DAT_00527f50 >> 1));
                DAT_00622cb8 = (int)(local_194[11] + (unsigned int)((int)DAT_00527f54 >> 1));
                DAT_00622ca0 = (int)(local_194[13] + (unsigned int)((int)DAT_00527f58 >> 1));
            } else {
                DAT_00527f74 = 0;
                DAT_00527f50 = 1;
                DAT_00527f54 = 1;
                DAT_00527f58 = 1;
                DAT_00527f5c = 0;
                DAT_00527f60 = 0;
                DAT_00527f64 = 0;
                DAT_00527f68 = 0;
                DAT_00527f6c = 0;
                DAT_00527f70 = 0;
            }
        }
    }
    *param_5 = 0;
    puVar6 = local_1c8;
    for (iVar5 = 0xd; iVar5 != 0; iVar5 = iVar5 - 1) {
        *puVar6 = 0;
        puVar6 = puVar6 + 1;
    }
    local_1c8[0] = 0x34;
    local_1c8[1] = 0x4c7;
    MVar3 = joyGetPosEx(DAT_00622ca8[uVar1], (void *)local_1c8);
    if (MVar3 == 0) {
        iVar5 = (int)(local_1c8[2] - (unsigned int)DAT_00622cb4);
        if (iVar5 < 0) {
            iVar5 = (int)(0u - (unsigned int)iVar5);
        }
        if (DAT_00527f68 < iVar5) {
            if (DAT_00527f50 == 0) {
                RaiseException(0xC0000094UL, 0, 0, NULL);
            }
            *param_2 = ((local_1c8[2] + DAT_00527f5c) * 0xff) / DAT_00527f50;
        } else {
            *param_2 = 0;
        }
        iVar5 = (int)(local_1c8[3] - (unsigned int)DAT_00622cb8);
        if (iVar5 < 0) {
            iVar5 = (int)(0u - (unsigned int)iVar5);
        }
        if (DAT_00527f6c < iVar5) {
            if (DAT_00527f54 == 0) {
                RaiseException(0xC0000094UL, 0, 0, NULL);
            }
            *param_3 = ((local_1c8[3] + DAT_00527f60) * 0xff) / DAT_00527f54;
        } else {
            *param_3 = 0;
        }
        iVar5 = (int)(local_1c8[4] - (unsigned int)DAT_00622ca0);
        if (iVar5 < 0) {
            iVar5 = (int)(0u - (unsigned int)iVar5);
        }
        if ((DAT_00527f70 < iVar5) && (0 < (int)DAT_00527f58)) {
            *param_4 = ((local_1c8[4] + DAT_00527f64) * 0xff) / DAT_00527f58;
        } else {
            *param_4 = 0;
        }
        *param_5 = local_1c8[8] & 0xffff;
        if ((DAT_00527f74 != 0) && (local_1c8[10] != 0xffff)) {
            *param_5 = (0x10000u << ((local_1c8[10] / 9000u) & 0x1f)) |
                       (local_1c8[8] & 0xffff);
            return;
        }
    } else {
        *param_2 = 32000;
        *param_3 = 32000;
        *param_4 = 32000;
    }
    return;
}

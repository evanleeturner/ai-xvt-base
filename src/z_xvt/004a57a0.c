/* SPDX-License-Identifier: MIT */
/* z_xvt 004a57a0 */
void __stdcall RaiseException(DWORD, DWORD, DWORD, const DWORD *);
MMRESULT __stdcall joyGetDevCapsA(unsigned int, LPJOYCAPSA, unsigned int);
MMRESULT __stdcall joyGetPosEx(unsigned int, LPJOYINFOEX);

extern int DAT_0052aec8[2];
extern unsigned int DAT_0052aed0;
extern unsigned int DAT_0052aed4;
extern unsigned int DAT_0052aed8;
extern unsigned int DAT_0052aedc;
extern unsigned int DAT_0052aee0;
extern unsigned int DAT_0052aee4;
extern int DAT_0052aee8;
extern int DAT_0052aeec;
extern int DAT_0052aef0;
extern unsigned int DAT_0052aef4;
extern int DAT_00645b90;
extern int DAT_00645b98;
extern int DAT_00645b9c;

void __cdecl FUN_004a57a0(int param_1, unsigned int *param_2, unsigned int *param_3,
                          unsigned int *param_4, unsigned int *param_5)
{
    unsigned int uJoyID;
    MMRESULT MVar1;
    int iVar2;
    unsigned int *ptVar3;
    unsigned int *pjVar4;
    unsigned int local_1c8[13];   /* JOYINFOEX, 0x34 bytes */
    unsigned int local_194[101];  /* JOYCAPSA, 0x194 bytes */

    uJoyID = (unsigned int)(param_1 != 0);
    if (DAT_0052aec8[uJoyID] == 0) {
        DAT_0052aec8[uJoyID] = 1;
        ptVar3 = local_194;
        for (iVar2 = 0x65; iVar2 != 0; iVar2 = iVar2 - 1) {
            *ptVar3 = 0;
            ptVar3 = ptVar3 + 1;
        }
        MVar1 = joyGetDevCapsA(0, (LPJOYCAPSA)local_194, 0x194);
        if (MVar1 == 0) {
            DAT_0052aef4 = (local_194[24] & 0x10) >> 4;
            DAT_0052aed0 = local_194[10] - local_194[9];
            DAT_0052aed4 = local_194[12] - local_194[11];
            DAT_0052aed8 = local_194[14] - local_194[13];
            DAT_0052aedc = (unsigned int)((int)DAT_0052aed0 >> 1) - local_194[10];
            DAT_0052aee0 = (unsigned int)((int)DAT_0052aed4 >> 1) - local_194[12];
            DAT_0052aee4 = (unsigned int)((int)DAT_0052aed8 >> 1) - local_194[14];
            DAT_0052aee8 = (int)DAT_0052aed0 / 0x14;
            DAT_0052aeec = (int)DAT_0052aed4 / 0x14;
            DAT_0052aef0 = (int)DAT_0052aed8 / 0x14;
            DAT_00645b98 = (int)((unsigned int)((int)DAT_0052aed0 >> 1) + local_194[9]);
            DAT_00645b90 = (int)((unsigned int)((int)DAT_0052aed8 >> 1) + local_194[13]);
            DAT_00645b9c = (int)((unsigned int)((int)DAT_0052aed4 >> 1) + local_194[11]);
        } else {
            MVar1 = joyGetDevCapsA(1, (LPJOYCAPSA)local_194, 0x194);
            if (MVar1 == 0) {
                DAT_0052aef4 = (local_194[24] & 0x10) >> 4;
                DAT_0052aed0 = local_194[10] - local_194[9];
                DAT_0052aed4 = local_194[12] - local_194[11];
                DAT_0052aed8 = local_194[14] - local_194[13];
                DAT_0052aedc = (unsigned int)((int)DAT_0052aed0 >> 1) - local_194[10];
                DAT_0052aee0 = (unsigned int)((int)DAT_0052aed4 >> 1) - local_194[12];
                DAT_0052aee4 = (unsigned int)((int)DAT_0052aed8 >> 1) - local_194[14];
                DAT_0052aee8 = (int)DAT_0052aed0 / 0x14;
                DAT_0052aeec = (int)DAT_0052aed4 / 0x14;
                DAT_0052aef0 = (int)DAT_0052aed8 / 0x14;
                DAT_00645b98 = (int)((unsigned int)((int)DAT_0052aed0 >> 1) + local_194[9]);
                DAT_00645b90 = (int)((unsigned int)((int)DAT_0052aed8 >> 1) + local_194[13]);
                DAT_00645b9c = (int)((unsigned int)((int)DAT_0052aed4 >> 1) + local_194[11]);
            } else {
                DAT_0052aef4 = 0;
                DAT_0052aed0 = 1;
                DAT_0052aed4 = 1;
                DAT_0052aed8 = 1;
                DAT_0052aedc = 0;
                DAT_0052aee0 = 0;
                DAT_0052aee4 = 0;
                DAT_0052aee8 = 0;
                DAT_0052aeec = 0;
                DAT_0052aef0 = 0;
            }
        }
    }
    *param_5 = 0;
    pjVar4 = local_1c8;
    for (iVar2 = 0xd; iVar2 != 0; iVar2 = iVar2 - 1) {
        *pjVar4 = 0;
        pjVar4 = pjVar4 + 1;
    }
    local_1c8[0] = 0x34;
    local_1c8[1] = 0x4c7;
    MVar1 = joyGetPosEx(uJoyID, (LPJOYINFOEX)local_1c8);
    if (MVar1 == 0) {
        iVar2 = (int)(local_1c8[2] - (unsigned int)DAT_00645b98);
        if (iVar2 < 0) {
            iVar2 = (int)(0u - (unsigned int)iVar2);
        }
        if (DAT_0052aee8 < iVar2) {
            if (DAT_0052aed0 == 0) {
                RaiseException(0xC0000094UL, 0, 0, NULL);
            }
            *param_2 = ((DAT_0052aedc + local_1c8[2]) * 0xffu) / DAT_0052aed0;
        } else {
            *param_2 = 0;
        }
        iVar2 = (int)(local_1c8[3] - (unsigned int)DAT_00645b9c);
        if (iVar2 < 0) {
            iVar2 = (int)(0u - (unsigned int)iVar2);
        }
        if (DAT_0052aeec < iVar2) {
            if (DAT_0052aed4 == 0) {
                RaiseException(0xC0000094UL, 0, 0, NULL);
            }
            *param_3 = ((DAT_0052aee0 + local_1c8[3]) * 0xffu) / DAT_0052aed4;
        } else {
            *param_3 = 0;
        }
        iVar2 = (int)(local_1c8[4] - (unsigned int)DAT_00645b90);
        if (iVar2 < 0) {
            iVar2 = (int)(0u - (unsigned int)iVar2);
        }
        if ((DAT_0052aef0 < iVar2) && (0 < (int)DAT_0052aed8)) {
            *param_4 = ((DAT_0052aee4 + local_1c8[4]) * 0xffu) / DAT_0052aed8;
        } else {
            *param_4 = 0;
        }
        *param_5 = local_1c8[8] & 0xffff;
        if ((DAT_0052aef4 != 0) && (local_1c8[10] != 0xffff)) {
            *param_5 = (0x10000u << ((unsigned char)(local_1c8[10] / 9000u) & 0x1f)) |
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

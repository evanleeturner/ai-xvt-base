/* SPDX-License-Identifier: MIT */
/* z_xvt 00418cd0 */
extern unsigned short DAT_0071ed02;
extern unsigned short DAT_0071ed04;
extern unsigned short DAT_0071ed0c;
extern unsigned int _DAT_006db794;
extern unsigned int _DAT_0071b70c;
extern unsigned int DAT_007325d0;
extern unsigned short _DAT_00732e4e;
extern unsigned int DAT_006db580;
extern unsigned int DAT_007327f0;
extern unsigned short DAT_00524570[];
extern unsigned short DAT_006da9f0;
extern unsigned short DAT_00732e40;

void __cdecl FUN_00419000(unsigned int param_1, unsigned int param_2, unsigned short *param_3, unsigned short *param_4);

void __cdecl FUN_00418cd0(unsigned int param_1, unsigned int param_2, unsigned int param_3)
{
    short sVar1;
    int bVar2;
    unsigned int local_4;

    bVar2 = (int)param_1 < 0;
    if (bVar2) {
        param_1 = 0u - param_1;
    }
    DAT_0071ed02 = (unsigned short)bVar2;
    bVar2 = (int)param_2 < 0;
    if (bVar2) {
        param_2 = 0u - param_2;
    }
    DAT_0071ed04 = (unsigned short)bVar2;
    bVar2 = (int)param_3 < 0;
    if (bVar2) {
        param_3 = 0u - param_3;
    }
    DAT_0071ed0c = (unsigned short)bVar2;
    _DAT_006db794 = param_1;
    _DAT_0071b70c = param_2;
    DAT_007325d0 = param_3;
    FUN_00419000(param_1, param_2, (unsigned short *)((int)&local_4 + 2), (unsigned short *)&local_4);
    _DAT_00732e4e = *(unsigned short *)((int)&local_4 + 2);
    DAT_006db580 = (((DAT_007327f0 & 0xffffu) * (unsigned int)DAT_00524570[local_4 & 0xffffu] + 0x8000u) >> 0x10)
                 + (DAT_007327f0 >> 0x10) * (unsigned int)DAT_00524570[local_4 & 0xffffu]
                 + DAT_007327f0;
    sVar1 = (short)*(unsigned short *)((int)&local_4 + 2);
    if (DAT_0071ed04 != 0) {
        sVar1 = (short)(0u - (unsigned int)(int)sVar1);
    }
    if (DAT_0071ed02 != 0) {
        sVar1 = (short)(0xffff8000u - (unsigned int)(int)sVar1);
    }
    DAT_006da9f0 = (unsigned short)(0x4000u - (unsigned int)(int)sVar1);
    FUN_00419000(DAT_006db580, DAT_007325d0, (unsigned short *)((int)&local_4 + 2), (unsigned short *)&local_4);
    _DAT_00732e4e = *(unsigned short *)((int)&local_4 + 2);
    DAT_006db580 = DAT_007327f0
                 + (((DAT_007327f0 & 0xffffu) * (unsigned int)DAT_00524570[local_4 & 0xffffu] + 0x8000u) >> 0x10)
                 + (DAT_007327f0 >> 0x10) * (unsigned int)DAT_00524570[local_4 & 0xffffu];
    if (DAT_0071ed0c != 0) {
        *(unsigned short *)((int)&local_4 + 2) = (unsigned short)(0u - (unsigned int)*(unsigned short *)((int)&local_4 + 2));
    }
    DAT_00732e40 = (unsigned short)(0x4000u - (unsigned int)*(unsigned short *)((int)&local_4 + 2));
    return;
}

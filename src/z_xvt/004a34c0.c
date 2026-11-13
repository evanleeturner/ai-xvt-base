/* SPDX-License-Identifier: MIT */
/* z_xvt 004a34c0 */
extern unsigned int DAT_006d4988;
extern unsigned short DAT_00732fe2;
extern unsigned short _DAT_00732950;
extern unsigned int _DAT_00732e70;
extern unsigned short _DAT_00732952;
extern unsigned int _DAT_00732e74;
extern unsigned short _DAT_00732954;
extern unsigned short DAT_00732956;
extern unsigned int DAT_0071ecf4;
extern unsigned int DAT_0052b070;
extern unsigned int DAT_0070acf0;

extern unsigned int __cdecl FUN_004ac6f0(int param_1, unsigned int * param_2);
extern void __cdecl FUN_004a1d70(unsigned int param_1, unsigned int param_2);

void __cdecl FUN_004a34c0(unsigned int param_1, unsigned int * param_2, unsigned short param_3)
{
    unsigned int uVar1;
    unsigned int uVar2;
    int iVar3;
    unsigned int local_ptr;

    iVar3 = (param_1 & 0xffff) * 0x23 + DAT_006d4988;
    uVar2 = *(unsigned char *)(iVar3 + 0x16);
    __builtin_memcpy(&local_ptr, (void *)(iVar3 + 0x1f), 4);
    DAT_00732fe2 = (unsigned short)*(unsigned char *)(local_ptr + 0x86);
    _DAT_00732950 = 0x8000;
    _DAT_00732e70 = (unsigned int)&DAT_0052b070 + *(unsigned char *)(param_2 + 1) * 0xef;
    _DAT_00732952 = 0x8001;
    _DAT_00732e74 = (unsigned int)&DAT_0070acf0 + uVar2 * 0x566;
    uVar1 = FUN_004ac6f0(uVar2, param_2);
    _DAT_00732954 = (unsigned short)uVar1;
    if (_DAT_00732954 != 0) {
        DAT_00732956 = param_3;
        FUN_004a1d70(0x84, DAT_0071ecf4);
        return;
    }
    _DAT_00732954 = param_3;
    FUN_004a1d70(0x85, DAT_0071ecf4);
    return;
}

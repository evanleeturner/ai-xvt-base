/* SPDX-License-Identifier: MIT */
/* z_xvt 004a41d0 */
extern int DAT_0071ecf4;
extern int DAT_006d4988;
extern unsigned short DAT_00732fe2;
extern unsigned short _DAT_00732950;
extern int _DAT_00732e70;
extern unsigned short _DAT_00732952;
extern int _DAT_00732e74;
extern unsigned short _DAT_00732954;

extern unsigned int __cdecl FUN_004ac6f0(unsigned int param_1, unsigned int param_2);
extern void __cdecl FUN_004a1d70(int param_1, int param_2);

void __cdecl FUN_004a41d0(int param_1)
{
    unsigned int uVar1;
    int iVar2;
    int v1f;
    int va7;
    
    iVar2 = *(int *)(0x71b730 + DAT_0071ecf4 * 0x5bd) * 0x23 + DAT_006d4988;
    __builtin_memcpy(&v1f, (void *)(iVar2 + 0x1f), 4);
    DAT_00732fe2 = (unsigned short)*(unsigned char *)(v1f + 0x86);
    _DAT_00732950 = 0x8000;
    __builtin_memcpy(&va7, (void *)(v1f + 0xa7), 4);
    _DAT_00732e70 = 0x52b070 + (unsigned int)*(unsigned char *)(va7 + 4) * 0xef;
    _DAT_00732952 = 0x8001;
    _DAT_00732e74 = 0x70acf0 + (unsigned int)*(unsigned char *)(iVar2 + 0x16) * 0x566;
    uVar1 = FUN_004ac6f0((unsigned int)*(unsigned char *)(iVar2 + 0x16), va7);
    _DAT_00732954 = (unsigned short)uVar1;
    FUN_004a1d70(param_1, DAT_0071ecf4);
    return;
}

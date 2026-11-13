/* SPDX-License-Identifier: MIT */
/* z_bop 00451e70 */
extern unsigned int DAT_009a1fe8;
extern unsigned short DAT_00a08292;
extern unsigned int DAT_009d8c87;
extern unsigned int DAT_0051c560;
extern unsigned int DAT_009d8c30;
extern short _DAT_00a07be4;
extern short DAT_00a07be6;
extern unsigned int DAT_009ecc34;

void FUN_00451bf0(short, unsigned int);
unsigned int FUN_0043c740(unsigned int, unsigned int *);
void FUN_00450650(short, unsigned int);

void FUN_00451e70(unsigned int param_1, unsigned int *param_2, unsigned short param_3)
{
    unsigned int uVar1;
    unsigned int uVar2;
    
    uVar2 = (unsigned int)*(unsigned char *)((param_1 & 0xffff) * 0x23 + 0x16 + DAT_009a1fe8);
    DAT_00a08292 = (unsigned short)*(unsigned char *)((unsigned char *)&DAT_009d8c87 + uVar2 * 0x566);
    FUN_00451bf0(0, (unsigned int)&DAT_0051c560 + (unsigned int)*(unsigned char *)(param_2 + 1) * 0x101);
    FUN_00451bf0(1, (unsigned int)&DAT_009d8c30 + uVar2 * 0x566);
    uVar1 = FUN_0043c740(uVar2, param_2);
    _DAT_00a07be4 = (short)uVar1;
    if (_DAT_00a07be4 != 0) {
        DAT_00a07be6 = param_3;
        FUN_00450650(0x9d, DAT_009ecc34);
        return;
    }
    _DAT_00a07be4 = param_3;
    FUN_00450650(0x9e, DAT_009ecc34);
    return;
}

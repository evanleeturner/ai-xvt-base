/* SPDX-License-Identifier: MIT */
/* z_bop 0045d1c0 */
extern unsigned short DAT_009a73c0;
extern unsigned short FUN_0045d2b0(unsigned short, unsigned short, unsigned short, unsigned short, unsigned short);

void __cdecl FUN_0045d1c0(unsigned int param_1)
{
    unsigned int iVar1;
    unsigned int iVar2;
    unsigned short uVar3;
    
    iVar2 = (param_1 & 0xffff) * 0x52;
    iVar1 = iVar2 + DAT_009a73c0 * 0x566;
    uVar3 = FUN_0045d2b0((unsigned short)(*(unsigned char *)(0x009d8cde + iVar1)),
                         (unsigned short)(*(unsigned char *)(0x009d8cdf + iVar1)),
                         (unsigned short)(*(unsigned char *)(0x009d8ce2 + iVar1)),
                         (unsigned short)(*(unsigned char *)(0x009d8ce0 + iVar1)),
                         (unsigned short)(*(unsigned char *)(0x009d8ce1 + iVar1)));
    if (uVar3 == 0xffff) {
        FUN_0045d2b0((unsigned short)(*(unsigned char *)(0x009d8cd8 + iVar1)),
                     (unsigned short)(*(unsigned char *)(0x009d8cda + iVar1)),
                     (unsigned short)(*(unsigned char *)(0x009d8cdc + iVar1)),
                     (unsigned short)(*(unsigned char *)(0x009d8cd9 + iVar1)),
                     (unsigned short)(*(unsigned char *)(0x009d8cdb + iVar1)));
    }
    return;
}

/* SPDX-License-Identifier: MIT */
/* z_bop 004a44d0 */
extern unsigned int DAT_009a73b0;
extern int DAT_009a1fe8;
extern int DAT_009a73b6;
extern short DAT_009a8070;
extern int DAT_00a08104;
extern short DAT_00a080f0;

void __cdecl FUN_0046a8d0(unsigned int param_1, unsigned int param_2, unsigned int param_3);
unsigned int __cdecl FUN_00404620(int param_1);
void __cdecl FUN_004a4cb0(unsigned int param_1);

void __cdecl FUN_004a44d0(short param_1, int param_2)
{
    unsigned int uVar1;
    int iVar2;
    unsigned int val_1f;
    unsigned int val_23;
    unsigned int val_27;
    unsigned short val_0d;

    iVar2 = (unsigned int)DAT_009a73b0 * 0x23 + DAT_009a1fe8;
    
    __builtin_memcpy(&val_1f, (void *)(DAT_009a73b6 + 0x1f), 4);
    __builtin_memcpy(&val_23, (void *)(DAT_009a73b6 + 0x23), 4);
    __builtin_memcpy(&val_27, (void *)(DAT_009a73b6 + 0x27), 4);
    
    FUN_0046a8d0(val_1f - *(unsigned int *)(iVar2 + 4),
                 val_23 - *(unsigned int *)(iVar2 + 8),
                 val_27 - *(unsigned int *)(iVar2 + 0xc));
                 
    *(short *)(DAT_009a73b6 + 0x32) = DAT_009a8070 + param_1;
    uVar1 = FUN_00404620(DAT_00a08104);
    FUN_004a4cb0(((unsigned short)(uVar1 >> 1) & 0x7fff) + 0x4000);
    
    if (param_2 != 0) {
        *(unsigned short *)(DAT_009a73b6 + 0x2e) = DAT_00a080f0;
        *(unsigned short *)(DAT_00a08104 + 0x9e) = 0xffff;
        *(unsigned char *)(DAT_00a08104 + 0x9d) = 0;
        
        __builtin_memcpy(&val_0d, (void *)(DAT_00a08104 + 0xd), 2);
        if (val_0d < *(unsigned short *)(DAT_009a73b6 + 0x2e)) {
            *(unsigned char *)(DAT_00a08104 + 0x9c) = 2;
        }
        else {
            *(unsigned char *)(DAT_00a08104 + 0x9c) = 1;
        }
        *(unsigned char *)(DAT_00a08104 + 0x96) = 0;
        *(unsigned char *)(DAT_00a08104 + 0x97) = 0;
    }
    return;
}

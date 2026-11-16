/* SPDX-License-Identifier: MIT */
/* z_xvt 00441830 */
extern int DAT_0071bb44;
extern int DAT_0071bb4a;
extern int DAT_0071bb4c;
extern int DAT_0071bb2f;
extern int DAT_0071bbc4;
extern int DAT_0071bb2b;
extern int DAT_0071bc3c;
extern int DAT_0071bb2d;
extern int DAT_0071bb40;
extern int DAT_0071bb27;
extern int DAT_0071b730;
extern int DAT_0071bb3c;
extern int DAT_0071bb35;
extern int DAT_0071bb3e;
extern int DAT_0071bb3a;
extern int DAT_0071bb33;

void FUN_0040cbf0(unsigned int param_1, int param_2);

void __cdecl FUN_00441830(int param_1)
{
    int iVar2 = param_1 * 0x5bd;
    short val_0071bb44;
    __builtin_memcpy(&val_0071bb44, (void *)((char *)&DAT_0071bb44 + iVar2), 2);
    if (val_0071bb44 != 0) {
        unsigned int uVar5;
        short val_0071bb4a;
        __builtin_memcpy(&val_0071bb4a, (void *)((char *)&DAT_0071bb4a + iVar2), 2);
        if (val_0071bb4a == 0) {
            uVar5 = 0x12;
        } else {
            uVar5 = 0x14;
        }
        FUN_0040cbf0(uVar5, param_1);
        
        unsigned short uVar3 = 0;
        unsigned short uVar4;
        do {
            uVar4 = uVar3 + 1;
            unsigned short val_0071bb2f;
            __builtin_memcpy(&val_0071bb2f, (void *)((char *)&DAT_0071bb2f + iVar2), 2);
            __builtin_memcpy((void *)((char *)&DAT_0071bb4c + (unsigned int)uVar3 * 2 + iVar2), &val_0071bb2f, 2);
            
            int iVar1 = iVar2 + (unsigned int)uVar3 * 2;
            unsigned short val_0071bb2b;
            __builtin_memcpy(&val_0071bb2b, (void *)((char *)&DAT_0071bb2b + iVar2), 2);
            __builtin_memcpy((void *)((char *)&DAT_0071bbc4 + iVar1), &val_0071bb2b, 2);
            
            unsigned short val_0071bb2d;
            __builtin_memcpy(&val_0071bb2d, (void *)((char *)&DAT_0071bb2d + iVar2), 2);
            __builtin_memcpy((void *)((char *)&DAT_0071bc3c + iVar1), &val_0071bb2d, 2);
            
            uVar3 = uVar4;
        } while (uVar4 < 0x3c);
        return;
    }
    
    unsigned short val_zero = 0;
    __builtin_memcpy((void *)((char *)&DAT_0071bb40 + iVar2), &val_zero, 2);
    
    unsigned short val_0071bb27;
    __builtin_memcpy(&val_0071bb27, (void *)((char *)&DAT_0071bb27 + iVar2), 2);
    unsigned int val_0071b730;
    __builtin_memcpy(&val_0071b730, (void *)((char *)&DAT_0071b730 + iVar2), 4);
    
    if ((unsigned int)val_0071bb27 == val_0071b730) {
        unsigned short val_0071bb3c;
        __builtin_memcpy(&val_0071bb3c, (void *)((char *)&DAT_0071bb3c + iVar2), 2);
        __builtin_memcpy((void *)((char *)&DAT_0071bb33 + iVar2), &val_0071bb3c, 2);
        
        unsigned short val_0071bb3e;
        __builtin_memcpy(&val_0071bb3e, (void *)((char *)&DAT_0071bb3e + iVar2), 2);
        __builtin_memcpy((void *)((char *)&DAT_0071bb35 + iVar2), &val_0071bb3e, 2);
        
        unsigned char val_0071bb3a;
        __builtin_memcpy(&val_0071bb3a, (void *)((char *)&DAT_0071bb3a + iVar2), 1);
        FUN_0040cbf0((unsigned int)val_0071bb3a, param_1);
        return;
    }
    
    unsigned short val_zero2 = 0;
    __builtin_memcpy((void *)((char *)&DAT_0071bb33 + iVar2), &val_zero2, 2);
    __builtin_memcpy((void *)((char *)&DAT_0071bb35 + iVar2), &val_zero2, 2);
    
    FUN_0040cbf0(0x12, param_1);
    
    unsigned short val_one = 1;
    __builtin_memcpy((void *)((char *)&DAT_0071bb44 + iVar2), &val_one, 2);
    return;
}

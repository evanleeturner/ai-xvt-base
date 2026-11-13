/* SPDX-License-Identifier: MIT */
/* z_xvt 004441e0 */
extern unsigned char DAT_0071b740;
extern unsigned char DAT_0071b743;
extern unsigned char DAT_0071b74a;
extern unsigned char DAT_0071b741;
extern unsigned char DAT_0071b730;
extern unsigned char DAT_0071b776;
extern unsigned char DAT_0071bb40;
extern unsigned char DAT_0071bb44;
extern unsigned char DAT_0071bb46;
extern unsigned char DAT_0071bb27;
extern unsigned char DAT_0071bb29;
extern unsigned char DAT_0071bb23;
extern unsigned char DAT_0071bb1b;
extern unsigned char DAT_0071bb1f;
extern unsigned char DAT_0071bcdd;
extern unsigned char DAT_0071e528;
extern unsigned char DAT_006d4988;
extern unsigned char DAT_0052686c;
extern unsigned char DAT_007092c0;

void __cdecl FUN_0040cbf0(unsigned int param_1, int param_2);
void __cdecl FUN_0040ae30(void);
void __cdecl FUN_0040acd0(void);
void __cdecl FUN_0040b190(void);

void __cdecl FUN_004441e0(int param_1)
{
    int iVar1;
    int iVar2;
    unsigned char *pcVar3;
    unsigned short local_ushort;
    unsigned int local_uint;
    unsigned char local_byte;
    
    pcVar3 = &DAT_0071b740;
    iVar1 = param_1 * 0x5bd;
    iVar2 = 0;
    (&DAT_0071b740)[iVar1] = 2;
    do {
        if (*pcVar3 == 1) {
            iVar2 = iVar2 + 1;
        }
        pcVar3 = pcVar3 + 0x5bd;
    } while (pcVar3 < &DAT_0071e528);
    
    if (iVar2 != 0) {
        (&DAT_0071b743)[iVar1] = 0xff;
        FUN_0040cbf0(0x15, param_1);
        
        local_ushort = 1;
        __builtin_memcpy(&DAT_0071bb40 + iVar1, &local_ushort, 2);
        local_ushort = 1;
        __builtin_memcpy(&DAT_0071bb44 + iVar1, &local_ushort, 2);
        local_uint = 0x40000;
        __builtin_memcpy(&DAT_0071bb46 + iVar1, &local_uint, 4);
        local_ushort = 0xffff;
        __builtin_memcpy(&DAT_0071bb27 + iVar1, &local_ushort, 2);
        local_ushort = 0xffff;
        __builtin_memcpy(&DAT_0071bb29 + iVar1, &local_ushort, 2);
        local_uint = 0x40000;
        __builtin_memcpy(&DAT_0071bb23 + iVar1, &local_uint, 4);
        
        __builtin_memcpy(&local_ushort, &DAT_0071b74a + iVar1, 2);
        if (local_ushort != 0xffff) {
            __builtin_memcpy(&local_uint, &DAT_006d4988 + (unsigned int)local_ushort * 0x23 + 4, 4);
            __builtin_memcpy(&DAT_0071bb1b + iVar1, &local_uint, 4);
            
            __builtin_memcpy(&local_uint, &DAT_006d4988 + (unsigned int)local_ushort * 0x23 + 8, 4);
            __builtin_memcpy(&DAT_0071bb1f + iVar1, &local_uint, 4);
            
            __builtin_memcpy(&local_byte, &DAT_006d4988 + (unsigned int)local_ushort * 0x23 + 3, 1);
            __builtin_memcpy(&local_ushort, &DAT_0052686c + (unsigned int)local_byte * 0x16, 2);
            local_uint = (unsigned int)local_ushort << 4;
            __builtin_memcpy(&DAT_0071bb46 + iVar1, &local_uint, 4);
        }
        
        (&DAT_0071b741)[iVar1] = 0;
        local_uint = 0xffffffff;
        __builtin_memcpy(&DAT_0071b730 + iVar1, &local_uint, 4);
        local_uint = 0;
        __builtin_memcpy(&DAT_0071bcdd + iVar1, &local_uint, 4);
        (&DAT_0071b776)[iVar1] = 0;
        
        FUN_0040ae30();
        FUN_0040acd0();
        FUN_0040b190();
        return;
    }
    DAT_007092c0 = 1;
    return;
}

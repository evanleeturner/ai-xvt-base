/* SPDX-License-Identifier: MIT */
/* z_bop 0043e050 */
extern int DAT_009e9670;
extern int DAT_009ecc34;
extern int DAT_009a1fe8;
extern int DAT_00a08374;

void __cdecl FUN_004a95c0(unsigned int);
unsigned int __cdecl FUN_00425b70(unsigned int, unsigned short);
void __cdecl FUN_004a9640(char);
void __cdecl FUN_00440840(int, unsigned short, int);

void __stdcall FUN_0043e050(void)
{
    int iVar1;
    unsigned int uVar2;
    char cVar3;
    int ptr1;
    int ptr2;
    int ptr3;
    unsigned short val_short;

    __builtin_memcpy(&ptr1, (void *)((int)&DAT_009e9670 + DAT_009ecc34 * 0x5bd), sizeof(int));
    __builtin_memcpy(&ptr2, (void *)(ptr1 * 0x23 + 0x1f + DAT_009a1fe8), sizeof(int));
    __builtin_memcpy(&ptr3, (void *)(ptr2 + 0xa9), sizeof(int));

    if ((*(unsigned char *)(ptr3 + 0x1e1) & 0x40) != 0) {
        FUN_004a95c0(0x40);
        iVar1 = ptr1 * 0x23;
        __builtin_memcpy(&val_short, (void *)(ptr2 + 0x79), sizeof(unsigned short));
        uVar2 = FUN_00425b70((iVar1 & 0xFFFF0000) | val_short, 0x71c7);
        if (DAT_00a08374 == 0) {
            cVar3 = 2;
        }
        else {
            cVar3 = 0;
        }
        FUN_004a9640(cVar3);
        FUN_00440840(DAT_00a08374 + 0x28, (unsigned short)uVar2, 1);
    }
    return;
}

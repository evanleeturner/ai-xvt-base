/* SPDX-License-Identifier: MIT */
/* z_xvt 004026c0 */
extern int DAT_00522038[];
extern void (*DAT_006d9984)(unsigned int);
void __stdcall RaiseException(unsigned int, unsigned int, unsigned int, const unsigned int *);

void __cdecl FUN_004026c0(int param_1, unsigned int param_2, unsigned int param_3)
{
    int bVar1 = 0;
    unsigned int uVar2;
    unsigned short uVar4;
    unsigned int uVar3;
    int divisor;

    for (; param_2 != 0; param_2 -= 1) {
        divisor = DAT_00522038[param_2];
        if (divisor == 0 || (param_1 == (int)0x80000000 && divisor == -1)) {
            RaiseException(0xC0000094UL, 0, 0, NULL);
        }
        uVar2 = (unsigned int)(param_1 / divisor);
        param_1 -= (int)((uVar2 & 0xFFFF) * divisor);
        uVar4 = (unsigned short)(uVar2 >> 16);

        if (bVar1 || param_3 < param_2 || (unsigned short)uVar2 != 0) {
            bVar1 = 1;
            if (9 < (unsigned short)uVar2) {
                uVar2 = ((unsigned int)uVar4 << 16) | 9;
            }
            uVar3 = (((unsigned int)(short)(uVar2 >> 16)) << 16) | (unsigned short)((short)uVar2 + 0x30);
        }
        else {
            uVar3 = ((unsigned int)uVar4 << 16) | 0x20;
            bVar1 = 0;
        }
        (*DAT_006d9984)(uVar3);
    }
}

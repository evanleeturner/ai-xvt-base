/* SPDX-License-Identifier: MIT */
/* z_xvt 00413f60 */
void __stdcall RaiseException(DWORD, DWORD, DWORD, const DWORD *);

unsigned int __cdecl FUN_00413f60(unsigned int param_1, unsigned int param_2)
{
    unsigned int uVar1;
    unsigned int uVar2;
    unsigned int uVar3;
    unsigned long long uVar4;

    uVar3 = param_1 * 0x100u;
    uVar2 = (param_1 >> 0x18) + (unsigned int)(0xffffff7fu < uVar3);
    if (uVar2 < param_2) {
        uVar4 = ((unsigned long long)uVar2 << 32) | (unsigned long long)(uVar3 + 0x80u);
        if (param_2 == 0u || (uVar4 >> 32) >= (unsigned long long)param_2) {
            RaiseException(0xC0000094UL, 0, 0, NULL);
            uVar1 = 0u;
        }
        else {
            uVar1 = (unsigned int)(uVar4 / (unsigned long long)param_2);
        }
    }
    else {
        uVar1 = 0x7fffff00u;
    }
    return uVar1;
}

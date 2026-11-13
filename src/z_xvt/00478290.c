/* SPDX-License-Identifier: MIT */
/* z_xvt 00478290 */
int __cdecl FUN_00478290(unsigned int param_1, unsigned int param_2, unsigned int param_3)
{
    int iVar1;
    unsigned int uVar2;
    int bVar3;

    bVar3 = (int)param_1 < 0;
    if ((int)param_1 < 0) {
        param_1 = 0u - param_1;
    }
    if ((int)param_2 < 0) {
        param_2 = 0u - param_2;
        bVar3 = !bVar3;
    }
    if ((int)param_3 < 0) {
        param_3 = 0u - param_3;
        bVar3 = !bVar3;
    }
    if (bVar3) {
        uVar2 = (unsigned int)(((unsigned long long)param_1 * (unsigned long long)param_2) >> 0x20);
        if (uVar2 < param_3) {
            /* The uVar2 < param_3 check guarantees the quotient fits in 32 bits
               and param_3 != 0, so the 64/32 divide cannot fault. */
            iVar1 = (int)((((unsigned long long)uVar2 << 0x20) |
                           (((unsigned long long)param_1 * (unsigned long long)param_2) & 0xffffffffULL)) /
                          (unsigned long long)param_3);
        } else {
            iVar1 = 0x7fffffff;
        }
        return (int)(0u - (unsigned int)iVar1);
    }
    uVar2 = (unsigned int)(((unsigned long long)param_1 * (unsigned long long)param_2) >> 0x20);
    if (uVar2 < param_3) {
        iVar1 = (int)((((unsigned long long)uVar2 << 0x20) |
                       (((unsigned long long)param_1 * (unsigned long long)param_2) & 0xffffffffULL)) /
                      (unsigned long long)param_3);
    } else {
        iVar1 = 0x7fffffff;
    }
    return iVar1;
}

/* SPDX-License-Identifier: MIT */
/* z_xvt 004d52a0 */
void __stdcall RaiseException(DWORD, DWORD, DWORD, const DWORD *);

extern int DAT_007806ee;

int __stdcall FUN_004d52a0(void)
{
    int divisor = DAT_007806ee;

    /* 1000 / divisor is a single IDIV of EDX:EAX (=1000) by a 32-bit divisor.
       It faults on divisor 0. The quotient always fits in 32 bits for a
       dividend of 1000, so only the zero check is reachable. */
    if (divisor == 0) {
        RaiseException(0xC0000094UL, 0, 0, NULL);
    }
    return 1000 / divisor;
}

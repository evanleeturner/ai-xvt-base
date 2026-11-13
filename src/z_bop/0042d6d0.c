/* SPDX-License-Identifier: MIT */
/* z_bop 0042d6d0 */
extern int DAT_00a604cc;
extern char DAT_00a1051c;
extern int DAT_00a6052c;

unsigned char __cdecl FUN_0042d600(void *);

unsigned char __stdcall FUN_0042d6d0(void)
{
    unsigned char bVar3 = 1;
    int *piVar2 = &DAT_00a604cc;
    
    do {
        if (*piVar2 != -1) {
            unsigned char bVar1 = FUN_0042d600(&DAT_00a1051c + *piVar2 * 0x145);
            bVar3 = bVar3 & bVar1;
        }
        piVar2 += 3;
    } while (piVar2 < &DAT_00a6052c);
    
    return bVar3;
}

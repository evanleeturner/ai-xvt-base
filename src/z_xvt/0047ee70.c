/* SPDX-License-Identifier: MIT */
/* z_xvt 0047ee70 */
extern void *DAT_00732948;
extern char DAT_0070ab20[];

int __cdecl _fclose(void *);
int __cdecl FID_conflict__remove(char *);

int __cdecl FUN_0047ee70(short param_1)
{
    int eax;
    short cx = 0;

    if (((*(int *)((char *)DAT_00732948 + 0xc)) & 0x20) == 0) {
        eax = (int)DAT_00732948;
        cx = 1;
    } else {
        eax = _fclose(DAT_00732948);
        if (eax != -1) {
            eax = (int)DAT_00732948;
            cx = 1;
        }
    }

    unsigned short high16 = (unsigned int)eax >> 16;

    if (param_1 != 0 && cx != 0) {
        eax = FID_conflict__remove((char *)&DAT_0070ab20);
        high16 = (unsigned int)eax >> 16;
    }

    return (high16 << 16) | (unsigned short)cx;
}

/* SPDX-License-Identifier: MIT */
/* z_xvt 004df3c0 */
extern int DAT_0064fca8;
extern unsigned char DAT_0064fcac;
extern void FUN_004df420(void);
int __cdecl FUN_004df760(int, int, int);
void __cdecl FUN_004db6c0(void *, void *);

void __stdcall FUN_004df3c0(void)
{
    int local_buf[4];
    int iVar1;

    local_buf[0] = 0x78;
    local_buf[2] = 0x208;
    local_buf[1] = 0xbe;
    local_buf[3] = 0x122;
    iVar1 = FUN_004df760(0, 0, 0xff);
    DAT_0064fca8 = 0;
    DAT_0064fcac = (unsigned char)iVar1;
    FUN_004db6c0(FUN_004df420, local_buf);
    return;
}

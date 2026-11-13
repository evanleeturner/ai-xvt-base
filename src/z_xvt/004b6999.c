/* SPDX-License-Identifier: MIT */
/* z_xvt 004b6999 */
extern void *DAT_0064ee44;
extern void *DAT_0064eecc;
extern void *DAT_0064ee50;
extern char *FUN_004b51f0(int param_1, int param_2, int param_3);
extern void FUN_0045bbe0(void);
extern void *_memset(void *dst, int c, unsigned int count);

int __cdecl FUN_004b6999(unsigned char param_1)
{
    unsigned int local_6c[20];
    int local_8;

    if ((param_1 & 1) != 0) {
        local_8 = ((int (*)(void *, int, void *, int))(*(void ***)DAT_0064ee44)[0x30 / 4])(DAT_0064ee44, 1, &DAT_0064ee50, 1);
        if (local_8 != 0) {
            FUN_004b51f0(local_8, 0x52f210, 0x79);
            FUN_0045bbe0();
            return 0;
        }
    }
    if ((param_1 & 2) != 0) {
        _memset(local_6c, 0, 100);
        local_6c[0] = 100;
        local_8 = ((int (*)(void *, int, int, int, unsigned int, void *))(*(void ***)DAT_0064eecc)[0x14 / 4])(DAT_0064eecc, 0, 0, 0, 0x3000000, local_6c);
        if (local_8 != 0) {
            FUN_004b51f0(local_8, 0x52f210, 0x79);
            FUN_0045bbe0();
            return 0;
        }
    }
    return 1;
}

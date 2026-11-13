/* SPDX-License-Identifier: MIT */
/* z_bop 004a45e0 */
extern void *DAT_00a08104;
extern void *DAT_009a73b6;

void __stdcall FUN_004a45e0(void)
{
  *(uchar *)(DAT_00a08104 + 0x97) = 0;
  *(uchar *)(DAT_00a08104 + 0x96) = 0;
  *(ushort *)(DAT_009a73b6 + 0x2e) = 0x4000;
  *(ushort *)(DAT_00a08104 + 0x9e) = 0xffff;
  *(uchar *)(DAT_00a08104 + 0x9d) = 0;
  ushort val_0xd;
  __builtin_memcpy(&val_0xd, DAT_00a08104 + 0xd, 2);
  if (val_0xd < 0x4000) {
    *(uchar *)(DAT_00a08104 + 0x9c) = 2;
  }
  else if (val_0xd < 0x4001) {
    *(uchar *)(DAT_00a08104 + 0x9c) = 3;
  }
  else {
    *(uchar *)(DAT_00a08104 + 0x9c) = 1;
  }
  *(uchar *)(DAT_00a08104 + 0xa4) = 1;
  ushort val_0xa5 = 0xffff;
  __builtin_memcpy(DAT_00a08104 + 0xa5, &val_0xa5, 2);
  *(ushort *)(DAT_009a73b6 + 0x30) = 0;
  *(uchar *)(DAT_00a08104 + 0xab) = 0;
  return;
}

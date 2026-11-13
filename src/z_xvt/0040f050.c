/* SPDX-License-Identifier: MIT */
/* z_xvt 0040f050 */
extern int DAT_006d4988;
extern int DAT_006d4980;
extern int _DAT_00732a3c;
extern int unaff_ESI;

void __cdecl FUN_0040ed40(byte *param_1, ushort param_2, uint param_3);

void __cdecl FUN_0040f050(ushort param_1)
{
  int iVar1;
  int iVar3;
  int ptr_0x1f;
  int val_0x1b;
  char val_0x03;

  __builtin_memcpy(&ptr_0x1f, (byte *)DAT_006d4988 + 0x1f + (uint)param_1 * 0x23, 4);
  if (ptr_0x1f != 0) {
    *(undefined4 *)(ptr_0x1f + 0x73) = 0;
    *(undefined1 *)(ptr_0x1f + 0x12) = 0;
    return;
  }
  if (_DAT_00732a3c < DAT_006d4980) {
    for (iVar3 = _DAT_00732a3c; iVar3 < DAT_006d4980; iVar3++) {
      iVar1 = DAT_006d4988 + iVar3 * 0x23;
      __builtin_memcpy(&val_0x03, (byte *)iVar1 + 3, 1);
      __builtin_memcpy(&val_0x1b, (byte *)iVar1 + 0x1b, 4);
      __builtin_memcpy(&ptr_0x1f, (byte *)iVar1 + 0x1f, 4);
      if (val_0x03 != '\0' && val_0x1b != -1 && ptr_0x1f != 0) {
        FUN_0040ed40((byte *)(ptr_0x1f + 0x12), (ushort)iVar3,
                     (((uint)unaff_ESI >> 0x10) << 0x10) | param_1);
      }
    }
  }
  return;
}

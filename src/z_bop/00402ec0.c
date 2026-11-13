/* SPDX-License-Identifier: MIT */
/* z_bop 00402ec0 */
extern int DAT_009a1fe8;
extern int DAT_009a73b0;
extern int DAT_009a1ff8;
extern int DAT_00524100;
extern int DAT_009a73d0;

extern void FUN_00403540(uint param_1, uint param_2);
extern uint FUN_00425b70(uint param_1, uint param_2);
extern int FUN_00403360(uint param_1, uint param_2);

uint __cdecl FUN_00402ec0(uint param_1, uint param_2, int param_3)
{
  char cVar1;
  uint bVar2;
  int iVar3;
  int iVar4;
  uint uVar5;
  uint uVar6;
  int unaligned_int;
  uint unaligned_uint;

  if (param_2 == 0xffff) {
    iVar3 = 0;
    goto LAB_00402ffc;
  }
  iVar3 = DAT_009a1fe8 + param_2 * 0x23;
  __builtin_memcpy(&unaligned_int, (char*)iVar3 + 0x1f, 4);
  if (unaligned_int == 0) {
    bVar2 = (*(char *)(iVar3 + 3) == '\0');
  }
  else {
    if (*(char *)(iVar3 + 3) == '\0') {
      iVar3 = 0;
      goto LAB_00402ffc;
    }
    __builtin_memcpy(&unaligned_int, (char*)iVar3 + 0x1f, 4);
    iVar3 = *(int *)(unaligned_int + 0xa9);
    if (iVar3 == 0) {
      iVar3 = 1;
      goto LAB_00402ffc;
    }
    if ((*(char *)(iVar3 + 0x5e) == '\x15') && (*(char *)(iVar3 + 0x5f) != '\0')) {
      iVar3 = 0;
      goto LAB_00402ffc;
    }
    cVar1 = *(char *)(iVar3 + 7);
    if ((cVar1 == '\x01') || (cVar1 == '\x03' || cVar1 == '\x04')) {
      iVar3 = 0;
      goto LAB_00402ffc;
    }
    if ((*(char *)(iVar3 + 0x23d) == '\x03') && (*(char *)(iVar3 + 0x241) != '\0')) {
      FUN_00403540(param_2, (uint)DAT_009a73b0);
      cVar1 = *(char *)((uint)DAT_009a73b0 * 0x23 + 2 + DAT_009a1fe8);
      if ((cVar1 == '\0') || (cVar1 == '\x01' || cVar1 == '\x02')) {
        if (0x8000 < DAT_009a1ff8) {
          iVar3 = 0;
          goto LAB_00402ffc;
        }
      }
      else if (0x10000 < DAT_009a1ff8) {
        iVar3 = 0;
        goto LAB_00402ffc;
      }
    }
    iVar4 = DAT_009a1fe8 + param_2 * 0x23;
    __builtin_memcpy(&unaligned_int, (char*)iVar4 + 0x1b, 4);
    if (unaligned_int != -1) {
      iVar3 = 1;
      goto LAB_00402ffc;
    }
    __builtin_memcpy(&unaligned_int, (char*)iVar4 + 0x1f, 4);
    int ptr_a9 = *(int *)(unaligned_int + 0xa9);
    __builtin_memcpy(&unaligned_uint, (char*)ptr_a9 + 0xcb, 4);
    uint val_c3;
    __builtin_memcpy(&val_c3, (char*)iVar3 + 0xc3, 4);
    bVar2 = unaligned_uint < val_c3;
  }
  iVar3 = 1 - bVar2;
LAB_00402ffc:
  if (iVar3 != 0) {
    uVar5 = FUN_00425b70(0x500, *(ushort *)((char*)&DAT_00524100 + (uint)DAT_009a73d0 * 2));
    uVar5 = (uVar5 & 0xffff) + 0xa00;
    if (param_3 != 0) {
      uVar6 = FUN_00425b70(uVar5, 0x5555);
      uVar5 = uVar5 + (uVar6 & 0xffff);
    }
    bVar2 = FUN_00403360(param_2, uVar5 << 8);
    if (bVar2 == 1) {
      return 1;
    }
  }
  return 0;
}

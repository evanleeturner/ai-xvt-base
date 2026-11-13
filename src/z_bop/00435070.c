/* SPDX-License-Identifier: MIT */
/* z_bop 00435070 */
extern int DAT_009a1fe8;
extern int DAT_009a1fe0;
extern int DAT_009e9640;
extern int DAT_009d9192;
extern short DAT_009e96f7;
extern short DAT_009e96fb;
extern short DAT_009e96ff;
extern int DAT_009e96dd;
extern ushort DAT_0051a3de;
extern uint DAT_009d6990;
extern byte DAT_009d8c88;

void __cdecl FUN_00435070(ushort param_1)
{
  byte bVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  ushort uVar5;
  uint uVar6;
  int iVar7;
  
  iVar3 = (uint)param_1 * 0x23;
  uVar6 = (uint)*(ushort *)(*(int *)(DAT_009a1fe8 + iVar3 + 0x1f) + 0x85);
  if ((int)uVar6 < DAT_009a1fe0) {
    iVar4 = uVar6 * 0x23;
    iVar7 = DAT_009a1fe8 + iVar4;
    if (*(char *)(iVar7 + 3) != 0) {
      iVar2 = *(int *)(*(int *)(iVar7 + 0x1f) + 0xa9);
      uVar5 = (short)DAT_009e9640 + 0x80;
      iVar7 = *(int *)((int)&DAT_009d9192 + (uint)*(byte *)(iVar7 + 0x16) * 0x566);
      switch(*(byte *)(DAT_009a1fe8 + iVar3 + 3)) {
      case 0x89:
      case 0x8a:
      case 0x8b:
      case 0x8c:
        *(short *)(iVar2 + 0x24e) = *(short *)(iVar2 + 0x24e) + 1;
        if ((param_1 < uVar5) && (iVar7 != -1)) {
          *(short *)(&DAT_009e96f7 + iVar7 * 0x5bd) = *(short *)(&DAT_009e96f7 + iVar7 * 0x5bd) + 1;
          return;
        }
        break;
      case 0x8d:
      case 0x8e:
        *(short *)(iVar2 + 0x252) = *(short *)(iVar2 + 0x252) + 1;
        if ((param_1 < uVar5) && (iVar7 != -1)) {
          *(short *)(&DAT_009e96fb + iVar7 * 0x5bd) = *(short *)(&DAT_009e96fb + iVar7 * 0x5bd) + 1;
          return;
        }
        break;
      case 0x8f:
      case 0x90:
      case 0x94:
      case 0x95:
      case 0x96:
      case 0x97:
      case 0x98:
      case 0x99:
      case 0x9a:
        *(char *)(iVar2 + 0x255) = *(char *)(iVar2 + 0x255) + 1;
        iVar2 = DAT_009a1fe8;
        bVar1 = *(byte *)(DAT_009a1fe8 + 0x16 + iVar4);
        if ((param_1 < uVar5) && (iVar7 != -1)) {
          *(short *)(&DAT_009e96ff + iVar7 * 0x5bd) = *(short *)(&DAT_009e96ff + iVar7 * 0x5bd) + 1;
          *(uint *)((int)&DAT_009e96dd + iVar7 * 0x5bd) =
               *(int *)((int)&DAT_009e96dd + iVar7 * 0x5bd) +
               (uint)*(ushort *)(&DAT_0051a3de + (uint)*(byte *)(iVar2 + 3 + iVar3) * 2);
        }
        (&DAT_009d6990)[(byte)(&DAT_009d8c88)[(uint)bVar1 * 0x566]] =
             (&DAT_009d6990)[(byte)(&DAT_009d8c88)[(uint)bVar1 * 0x566]] +
             (uint)*(ushort *)(&DAT_0051a3de + (uint)*(byte *)(DAT_009a1fe8 + 3 + iVar3) * 2);
        break;
      }
    }
  }
  return;
}

/* SPDX-License-Identifier: MIT */
/* z_bop 00407910 */
extern int DAT_009a8e18;
extern int DAT_009a1fe8;
extern char DAT_009e96b6;
extern char DAT_009e96b9;
extern char DAT_009e96b7;
extern char DAT_009e9c1d;
extern char DAT_009e9670;
extern int DAT_009e9640;
extern int DAT_009ecc34;

void __cdecl FUN_00450650(uint, int);
void __cdecl FUN_0042fb70(int, uint, int, int, uint, uint);

void __cdecl FUN_00407910(ushort param_1)
{
  int iVar1;
  int iVar2;
  int iVar3;
  
  iVar1 = *(int *)((uint)*(ushort *)(DAT_009a8e18 + 4 + (uint)param_1 * 10) * 0x23 + 0x1b + DAT_009a1fe8);
  if (iVar1 != -1) {
    iVar2 = iVar1 * 0x5bd;
    if ((&DAT_009e96b6)[iVar2] == '\0') {
      (&DAT_009e96b6)[iVar2] = 1;
      *(undefined2 *)((int)&DAT_009e96b9 + iVar2) = 0xffff;
      iVar3 = DAT_009ecc34;
      *(ushort *)((int)&DAT_009e96b7 + iVar2) = (short)DAT_009e9640 + param_1;
      *(undefined4 *)((int)&DAT_009e9c1d + iVar2) = 0x588;
      if (iVar1 == iVar3) {
        FUN_00450650(0x74, iVar1);
        FUN_0042fb70(DAT_009ecc34, 0xffffffff, 0xc, -1, *(uint *)((int)&DAT_009e9670 + iVar2), 0xffff);
      }
    }
  }
  return;
}

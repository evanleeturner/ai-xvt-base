/* SPDX-License-Identifier: MIT */
/* z_xvt 004a00e0 */
extern char DAT_006d4a60;
extern char DAT_006d4a61;
extern char DAT_006d4a6a;
extern char DAT_006d4a6c;
extern char DAT_006d4a70;
extern char DAT_006d4a74;
extern char DAT_006d4a78;
extern char DAT_006d4a7c;
extern char DAT_006d4a80;
extern char DAT_006d4a88;
extern char DAT_006d4aa2;
extern char DAT_006d4aa3;
extern char DAT_006d4aa5;
extern char DAT_006d4aa7;
extern char DAT_006d4aa9;
extern char DAT_006d4aab;
extern char DAT_006d4aad;
extern char DAT_006d4ab1;

void __cdecl FUN_004a00e0(int param_1)
{
  int iVar1;
  short sVar2;
  char cVar3;
  
  iVar1 = param_1 * 0x126;
  sVar2 = (&DAT_006d4a6a)[param_1 * 0x93] - *(short *)(&DAT_006d4a6c + iVar1);
  cVar3 = (&DAT_006d4aa2)[param_1 * 0x126] - (&DAT_006d4aa3)[param_1 * 0x126];
  *(short *)(&DAT_006d4a6c + iVar1) = *(short *)(&DAT_006d4a6c + iVar1) + sVar2;
  *(short *)(&DAT_006d4a70 + iVar1) = *(short *)(&DAT_006d4a70 + iVar1) + sVar2;
  (&DAT_006d4aa5)[iVar1] = (&DAT_006d4aa5)[iVar1] + cVar3;
  *(short *)(&DAT_006d4a7c + iVar1) = *(short *)(&DAT_006d4a7c + iVar1) + sVar2;
  (&DAT_006d4aab)[iVar1] = (&DAT_006d4aab)[iVar1] + cVar3;
  *(short *)(&DAT_006d4a88 + iVar1) = *(short *)(&DAT_006d4a88 + iVar1) + sVar2;
  (&DAT_006d4ab1)[iVar1] = (&DAT_006d4ab1)[iVar1] + cVar3;
  *(short *)(&DAT_006d4a78 + iVar1) = *(short *)(&DAT_006d4a78 + iVar1) + sVar2;
  (&DAT_006d4aa9)[iVar1] = (&DAT_006d4aa9)[iVar1] + cVar3;
  *(short *)(&DAT_006d4a74 + iVar1) = *(short *)(&DAT_006d4a74 + iVar1) + sVar2;
  (&DAT_006d4aa7)[iVar1] = (&DAT_006d4aa7)[iVar1] + cVar3;
  *(short *)(&DAT_006d4a80 + iVar1) = *(short *)(&DAT_006d4a80 + iVar1) + sVar2;
  (&DAT_006d4aad)[iVar1] = (&DAT_006d4aad)[iVar1] + cVar3;
  (&DAT_006d4a60)[iVar1] = 1;
  (&DAT_006d4a61)[iVar1] = 0;
  return;
}

/* SPDX-License-Identifier: MIT */
/* z_bop 00402cb0 */
extern ushort DAT_009a73b0;
extern int _DAT_009a73b2;
extern byte *DAT_009a73b6;
extern ushort DAT_009a73ba;
extern int DAT_009a73bc;
extern ushort DAT_009a73c0;
extern ushort DAT_009a73c2;
extern int DAT_009a73c4;
extern int DAT_009a73c8;
extern int DAT_009a73cc;
extern ushort DAT_009a73d0;
extern ushort DAT_009a73d2;
extern int DAT_009a73d4;
extern int DAT_009a73d8;
extern char DAT_009a73d9;
extern int DAT_009a1fe8;
extern int DAT_00a07cdc;
extern int DAT_00a07cd8;
extern int DAT_00a07cd4;
extern byte DAT_00a07cf0[];
extern int DAT_0051a1d0;

extern void FUN_00458e20(uint param_1, uint param_2);
extern uint FUN_00404620(int param_1);
extern int FUN_00402e00(ushort param_1);
extern int FUN_0046b610(int *param_1);

void __cdecl FUN_00402cb0(ushort param_1)
{
  uint uVar1;
  int iVar2;
  int iVar3;
  int tmp_ptr;
  int tmp_val;

  DAT_009a73b0 = param_1;
  iVar3 = (uint)param_1 * 0x23 + DAT_009a1fe8;
  
  __builtin_memcpy(&tmp_ptr, (void *)(iVar3 + 0x1f), sizeof(int));
  __builtin_memcpy(&tmp_val, (void *)(tmp_ptr + 0xa9), sizeof(int));
  iVar2 = tmp_val;
  
  DAT_009a73b6 = (byte *)(iVar2 + 0x2a);
  DAT_009a73ba = (ushort)*(byte *)(iVar2 + 5);
  
  if (DAT_009a73ba == 0xff) {
    DAT_009a73bc = tmp_val;
  }
  else {
    int addr = (uint)DAT_009a73ba * 0x23 + 0x1f + DAT_009a1fe8;
    int p1;
    int p2;
    __builtin_memcpy(&p1, (void *)addr, sizeof(int));
    __builtin_memcpy(&p2, (void *)(p1 + 0xa9), sizeof(int));
    DAT_009a73bc = p2;
  }
  
  DAT_009a73c0 = (ushort)*(byte *)(iVar3 + 0x16);
  DAT_009a73c2 = (ushort)*DAT_009a73b6;
  _DAT_009a73b2 = iVar2;
  FUN_00458e20((uint)param_1, (uint)DAT_009a73c0);
  DAT_009a73c4 = DAT_00a07cdc;
  DAT_009a73c8 = DAT_00a07cd8;
  DAT_009a73cc = DAT_00a07cd4;
  uVar1 = FUN_00404620(_DAT_009a73b2);
  iVar3 = FUN_00402e00((ushort)uVar1);
  DAT_009a73d0 = (ushort)iVar3;
  
  byte idx = *(byte *)(iVar2 + 0x34);
  DAT_009a73d4 = DAT_00a07cf0[idx] + 2;
  DAT_009a73d8 = 0;
  DAT_009a73d2 = (ushort)DAT_00a07cf0[idx + 1];
  
  iVar2 = FUN_0046b610(&DAT_0051a1d0);
  DAT_009a73d9 = (char)iVar2;
  return;
}

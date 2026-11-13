/* SPDX-License-Identifier: MIT */
/* z_xvt 004296b0 */
extern ushort DAT_006d9d50;
extern int _DAT_006d9d52;
extern ushort DAT_006d9d5a;
extern uint DAT_006d9d5c;
extern ushort DAT_006d9d60;
extern ushort DAT_006d9d62;
extern byte *DAT_006d9d56;
extern int DAT_006d9d64;
extern int DAT_006d9d68;
extern int DAT_006d9d6c;
extern ushort DAT_006d9d70;
extern int DAT_006d9d74;
extern byte DAT_006d9d78;
extern ushort DAT_006d9d72;
extern char DAT_006d9d79;
extern int DAT_006d4988;
extern int DAT_00732a2c;
extern int DAT_00732a28;
extern int DAT_00732a24;
extern int DAT_00732a40;
extern char DAT_00524948;

extern int __cdecl FUN_0042b150(int param_1);
extern void __cdecl FUN_00421c70(uint param_1, int param_2);
extern int __cdecl FUN_00481380(char * param_1);

void __cdecl FUN_004296b0(ushort param_1)
{
  short *psVar1;
  ushort *puVar2;
  ushort uVar3;
  byte *pbVar4;
  int iVar5;
  int iVar6;
  int tmp_addr1;
  int tmp_val1;
  int tmp_addr2;
  uint tmp_val2;
  short *tmp_ptr1;
  ushort tmp_val3;
  int tmp_ptr2;
  ushort *tmp_ptr3;
  uint tmp_zero;
  
  DAT_006d9d50 = param_1;
  iVar5 = (uint)param_1 * 0x23;
  __builtin_memcpy(&tmp_addr1, (void *)(DAT_006d4988 + 0x1f + iVar5), 4);
  __builtin_memcpy(&tmp_val1, (void *)((int)tmp_addr1 + 0xa7), 4);
  _DAT_006d9d52 = tmp_val1;
  DAT_006d9d5a = (ushort)*(byte *)(_DAT_006d9d52 + 5);
  pbVar4 = (byte *)FUN_0042b150(_DAT_006d9d52);
  if (DAT_006d9d5a == 0xff) {
    __builtin_memcpy(&tmp_addr2, (void *)(DAT_006d4988 + 0x1f + iVar5), 4);
  }
  else {
    __builtin_memcpy(&tmp_addr2, (void *)((uint)DAT_006d9d5a * 0x23 + 0x1f + DAT_006d4988), 4);
  }
  iVar6 = tmp_addr2;
  __builtin_memcpy(&tmp_val2, (void *)(iVar6 + 0xa7), 4);
  DAT_006d9d5c = tmp_val2;
  DAT_006d9d60 = (ushort)*(byte *)(DAT_006d4988 + 0x16 + iVar5);
  DAT_006d9d62 = (ushort)*pbVar4;
  DAT_006d9d56 = pbVar4;
  FUN_00421c70((uint)DAT_006d9d50,(uint)DAT_006d9d60);
  iVar5 = _DAT_006d9d52;
  DAT_006d9d64 = DAT_00732a2c;
  DAT_006d9d68 = DAT_00732a28;
  DAT_006d9d6c = DAT_00732a24;
  __builtin_memcpy(&tmp_ptr1, (void *)(_DAT_006d9d52 + 0x432), 4);
  psVar1 = tmp_ptr1;
  if (psVar1 == (short *)0x0) {
    __builtin_memcpy(&tmp_val3, (void *)(_DAT_006d9d52 + 9), 2);
    uVar3 = tmp_val3;
  }
  else {
    if (*(short *)(_DAT_006d9d52 + 900) == *psVar1) {
      __builtin_memcpy(&tmp_ptr2, (void *)((int)psVar1 + 0x1f), 4);
      __builtin_memcpy(&tmp_ptr3, (void *)((int)tmp_ptr2 + 0xab), 4);
      if ((tmp_ptr2 != 0) && (puVar2 = tmp_ptr3, puVar2 != (ushort *)0x0)) {
        uVar3 = *puVar2;
        goto LAB_004297ee;
      }
    }
    else {
      tmp_zero = 0;
      __builtin_memcpy((void *)(_DAT_006d9d52 + 0x432), &tmp_zero, 4);
    }
    __builtin_memcpy(&tmp_val3, (void *)(iVar5 + 9), 2);
    uVar3 = tmp_val3;
  }
LAB_004297ee:
  if (uVar3 < 0x8000) {
    DAT_006d9d70 = 0;
  }
  else {
    DAT_006d9d70 = 2 - (ushort)(uVar3 < 0xc000);
  }
  DAT_006d9d74 = (&DAT_00732a40)[pbVar4[10]] + 2;
  DAT_006d9d78 = 0;
  DAT_006d9d72 = (ushort)*(byte *)((&DAT_00732a40)[pbVar4[10]] + 1);
  iVar5 = FUN_00481380(&DAT_00524948);
  DAT_006d9d79 = (char)iVar5;
  return;
}

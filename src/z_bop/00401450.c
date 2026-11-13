/* SPDX-License-Identifier: MIT */
/* z_bop 00401450 */
extern int __cdecl FUN_004243d0(int param_1, uint param_2, uint param_3);
extern int __cdecl FUN_004ac720(uint param_1);
extern void __stdcall FUN_004ac740(void);
extern void __cdecl FUN_0040bbf0(ushort param_1, int param_2, int param_3, uint param_4, int param_5);
extern void __cdecl FUN_00421850(ushort param_1);
extern void __cdecl FUN_00421930(int param_1);
extern void __cdecl FUN_00421560(ushort param_1, ushort param_2, ushort param_3, int param_4);

extern int DAT_0052361c;
extern ushort DAT_009ed664;
extern uint DAT_009a1fe8;
extern int DAT_009fd39c;
extern int DAT_009e9a5b;
extern uint DAT_009ecc34;
extern int DAT_009fd3a0;
extern int DAT_009e9a5f;
extern int DAT_009fd398;
extern int DAT_009e9a63;
extern uint DAT_009a8e34;
extern uint DAT_00521f1c;
extern ushort DAT_00521f24;
extern int DAT_00527e90;
extern ushort unaff_retaddr;

void __cdecl FUN_00401450(ushort *param_1)
{
  ushort uVar1;
  int iVar2;
  uint uVar3;
  int iVar4;
  int tmp_9e9a5b;
  int tmp_9e9a5f;
  int tmp_9e9a63;
  uint tmp_param_5;
  uint tmp_521f1c;
  ushort tmp_521f24;

  uVar1 = param_1[1];
  DAT_0052361c = 1;
  uVar3 = (uint)((uVar1 & 0x7fff) >> 7);
  DAT_009ed664 = *param_1;
  iVar4 = (uint)DAT_009ed664 * 0x23 + DAT_009a1fe8;

  __builtin_memcpy(&tmp_9e9a5b, (void*)((char*)&DAT_009e9a5b + DAT_009ecc34 * 0x5bd), sizeof(int));
  DAT_009fd39c = *(int *)(iVar4 + 4) - tmp_9e9a5b;

  __builtin_memcpy(&tmp_9e9a5f, (void*)((char*)&DAT_009e9a5f + DAT_009ecc34 * 0x5bd), sizeof(int));
  iVar2 = tmp_9e9a5f;
  DAT_009fd3a0 = *(int *)(iVar4 + 8) - iVar2;

  __builtin_memcpy(&tmp_9e9a63, (void*)((char*)&DAT_009e9a63 + DAT_009ecc34 * 0x5bd), sizeof(int));
  DAT_009fd398 = *(int *)(iVar4 + 0xc) - tmp_9e9a63;

  __builtin_memcpy(&tmp_param_5, (void*)((char*)param_1 + 10), sizeof(uint));
  DAT_009a8e34 = tmp_param_5;

  __builtin_memcpy(&tmp_521f1c, (void*)((char*)&DAT_00521f1c + uVar3 * 0x1a), sizeof(uint));
  iVar4 = FUN_004243d0((int)tmp_param_5, tmp_521f1c, (uint)param_1[2]);

  __builtin_memcpy(&tmp_521f24, (void*)((char*)&DAT_00521f24 + uVar3 * 0xd), sizeof(ushort));
  iVar2 = FUN_004ac720((iVar2 & 0xFFFF0000) | (uint)tmp_521f24);

  FUN_004ac740();

  iVar2 = *(int *)(*(int *)(iVar2 + 0x10) + ((uVar1 & 0x7fff) & 0x7f) * 4 + iVar2) + iVar2;

  if (DAT_00527e90 != 0) {
    FUN_0040bbf0(param_1[7], (int)(short)param_1[3], (int)(short)param_1[4],
                ((ushort)unaff_retaddr << 16) | (ushort)iVar4, iVar2);
    return;
  }
  FUN_00421850(param_1[7]);
  FUN_00421930(iVar2);
  FUN_00421560(param_1[3], param_1[4], (ushort)iVar4, iVar2);
  return;
}

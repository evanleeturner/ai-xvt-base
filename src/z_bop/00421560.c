/* SPDX-License-Identifier: MIT */
/* z_bop 00421560 */
extern int DAT_0052361c;
extern short DAT_009a8d4a;
extern short DAT_009a8d4e;
extern int DAT_009fe7d4;
extern unsigned short DAT_009ec610;
extern short DAT_009a8d42;
extern short DAT_009a7b48;
extern short DAT_009a8d48;
extern short DAT_009a7b40;

void __cdecl FUN_004219d0(unsigned short param_1, int param_2, unsigned short *param_3);
void __cdecl FUN_00421ae0(int param_1, unsigned short *param_2);
void __cdecl FUN_00422170(unsigned char *param_1, int param_2);
void __cdecl FUN_00421700(int *param_1);

void __cdecl FUN_00421560(short param_1, short param_2, unsigned short param_3, int param_4)
{
  short sVar1;
  short sVar2;
  int *puVar4;
  int local_20;

  puVar4 = (int *)(*(int *)(param_4 + 8) + param_4);
  if (DAT_0052361c == 1) {
    sVar1 = (short)*puVar4;
  }
  else {
    sVar1 = -(short)puVar4[2];
  }
  sVar2 = -(short)puVar4[1];
  DAT_009a8d4a = sVar1;
  DAT_009a8d4e = sVar2;
  FUN_004219d0(param_3, DAT_009fe7d4, (unsigned short *)&DAT_009ec610);
  FUN_00421ae0(DAT_009fe7d4, (unsigned short *)&DAT_009ec610);
  DAT_009a8d42 = DAT_009a7b48 + param_1;
  local_20 = (int)DAT_009a8d42;
  DAT_009a8d48 = DAT_009a7b40 + param_2;
  FUN_00422170((unsigned char *)(puVar4 + 4), *(int *)(param_4 + 0x20));
  DAT_009a8d4a = (short)*(int *)(param_4 + 0x10) + sVar1;
  DAT_009a8d4e = sVar2;
  FUN_00421ae0(DAT_009fe7d4, (unsigned short *)&DAT_009ec610);
  DAT_009a8d4a = (short)*(int *)(param_4 + 0x10) + sVar1;
  DAT_009a8d4e = sVar2 - *(short *)(param_4 + 0x14);
  FUN_00421ae0(DAT_009fe7d4, (unsigned short *)&DAT_009ec610);
  DAT_009a8d4e = sVar2 - *(short *)(param_4 + 0x14);
  DAT_009a8d4a = sVar1;
  FUN_00421ae0(DAT_009fe7d4, (unsigned short *)&DAT_009ec610);
  FUN_00421700(&local_20);
  return;
}

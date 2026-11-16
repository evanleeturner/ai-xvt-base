/* SPDX-License-Identifier: MIT */
/* z_xvt 004cff10 */
extern int DAT_00759b08;
extern int DAT_00759b0c;
extern int DAT_00759b14;
extern int DAT_00759b18;
extern int DAT_00759b20;
extern int DAT_00759b24;
extern int DAT_00759b2c;

int __cdecl FUN_004cfe80(int param_1);

int __stdcall FUN_004cff10(void)
{
  DWORD DVar1;
  int iVar2;
  unsigned char local_4[4];
  
  iVar2 = DAT_00759b18;
  if (DAT_00759b08 == 0) {
    return 0;
  }
  if (DAT_00759b2c == 0) {
    if ((DAT_00759b0c != 0) && (DAT_00759b14 == 0)) {
      DVar1 = GetTickCount();
      DAT_00759b20 = (iVar2 - DVar1) - 2000;
      iVar2 = FUN_004cfe80(DAT_00759b0c);
      DAT_00759b24 = iVar2 - DAT_00759b20;
      mciSendCommandA(DAT_00759b08, 0x808, 0, (DWORD)local_4);
      DAT_00759b2c = 1;
      return 1;
    }
  }
  else if (DAT_00759b2c == 2) {
    DAT_00759b2c = 1;
  }
  return 1;
}

/* SPDX-License-Identifier: MIT */
/* z_bop 004a5210 */
extern DWORD DAT_00622c4c;
extern DWORD DAT_00622c48;
extern DWORD _DAT_00622bc0;
extern DWORD DAT_00622bc8;
extern DWORD DAT_00622bd0;

MMRESULT __stdcall FUN_004a5210(void)
{
  DWORD dwVolume;
  UINT uNumDevs;
  MMRESULT mmrResult;
  UINT uDeviceID;
  unsigned char auStack_34[28];
  unsigned char auxCaps[0x30];
  DWORD *puVar4;
  int iVar3;

  if (DAT_00622c4c != 0) {
    if (DAT_00622c48 != 0) {
      mciSendCommandA(DAT_00622c4c, 0x808, 0, (DWORD)auStack_34);
      DAT_00622c48 = 0;
      _DAT_00622bc0 = 0;
    }
    uDeviceID = 0;
    mciSendCommandA(DAT_00622c4c, 0x804, 0, 0);
    DAT_00622c4c = 0;
    puVar4 = &DAT_00622bd0;
    for (iVar3 = 0x1e; iVar3 != 0; iVar3 = iVar3 + -1) {
      *puVar4 = 0;
      puVar4 = puVar4 + 1;
    }
    DAT_00622c48 = 0;
    _DAT_00622bc0 = 0;
    uNumDevs = auxGetNumDevs();
    mmrResult = DAT_00622bc8;
    if ((DAT_00622bc8 != 0xffffffff) && (dwVolume = DAT_00622bc8 * 0x10001, 0 < (int)uNumDevs)) {
      do {
        puVar4 = (DWORD *)auxCaps;
        for (iVar3 = 0xc; iVar3 != 0; iVar3 = iVar3 + -1) {
          *puVar4 = 0;
          puVar4 = puVar4 + 1;
        }
        mmrResult = auxGetDevCapsA(uDeviceID, (LPAUXCAPSA)auxCaps, 0x30);
        if ((*(unsigned short *)(auxCaps + 0x18) == 1) && (mmrResult = *(unsigned int *)(auxCaps + 0x14), (*(unsigned int *)(auxCaps + 0x14) & 1) != 0)) {
          mmrResult = auxSetVolume(uDeviceID, dwVolume);
        }
        uDeviceID = uDeviceID + 1;
      } while ((int)uDeviceID < (int)uNumDevs);
    }
    DAT_00622bc8 = 0xffffffff;
    return mmrResult;
  }
  return 0;
}

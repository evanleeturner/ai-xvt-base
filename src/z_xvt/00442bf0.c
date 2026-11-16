/* SPDX-License-Identifier: MIT */
/* z_xvt 00442bf0 */
/* Prototypes (return types and parameter types are not fully known from the evidence). */
extern void FUN_0040a1a0(unsigned int, unsigned int, int);
extern int  FUN_004490f0(unsigned int);
extern int  FUN_00449790(unsigned int, unsigned int);
extern void FUN_004a3830(unsigned int, int, int, int);
extern void FUN_004a1d70(int, int);

extern int DAT_006d4988;
extern int DAT_006d4980;
extern int DAT_0071ecf4;
extern unsigned char DAT_0071b730[];
extern unsigned char DAT_0071b743[];
extern unsigned char DAT_0071b74a[];
extern unsigned char DAT_0071b756[];
extern unsigned char DAT_0071b759[];
extern unsigned char DAT_0071bb27[];
extern unsigned char DAT_0071bb37[];
extern unsigned char DAT_0071bb4a[];
extern unsigned short _DAT_00732950;
extern unsigned short _DAT_00732952;

void FUN_00442bf0(unsigned short param_1, int param_2)
{
  unsigned int uVar1;
  int bVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  unsigned int uVar7;
  unsigned int uVar6;
  unsigned int uTmp;
  unsigned int uP;
  unsigned short sTmp;
  unsigned short sCur;
  unsigned char bTmp;

  uVar6 = param_1;
  if (param_1 != 0xffff) {
    uVar7 = (unsigned int)param_1;
    iVar3 = (int)(uVar7 * 0x23u + (unsigned int)DAT_006d4988);
    if ((*(unsigned char *)(iVar3 + 3) != 0) && (*(unsigned char *)(iVar3 + 2) != '\r')) {
      iVar3 = (int)((unsigned int)param_2 * 0x5bdu);
      __builtin_memcpy(&uVar1, DAT_0071b730 + iVar3, 4);
      if (uVar1 != uVar7) {
        bVar2 = 1;
        if (uVar1 != 0xffffffffu) {
          __builtin_memcpy(&uTmp, (unsigned char *)(uVar1 * 0x23u + 0x1fu + (unsigned int)DAT_006d4988), 4);
          __builtin_memcpy(&uP, (unsigned char *)(uTmp + 0xa7u), 4);
          bTmp = *(unsigned char *)(uP + 0x1dfu);
          if ((bTmp & 4) == 0) {
            bVar2 = 0;
          }
        }
        if (bVar2) {
          __builtin_memcpy(&sTmp, DAT_0071b74a + iVar3, 2);
          if ((param_1 != 0xffff) && (sTmp != param_1)) {
            FUN_0040a1a0(0x31, 0xffffffffu, param_2);
            __builtin_memcpy(DAT_0071b74a + iVar3, &param_1, 2);
            sTmp = 0;
            __builtin_memcpy(DAT_0071b759 + iVar3, &sTmp, 2);
            if ((int)uVar7 < DAT_006d4980) {
              uVar7 = 0;
              __builtin_memcpy(&sCur, DAT_0071b74a + iVar3, 2);
              iVar4 = FUN_004490f0((unsigned int)*(unsigned char *)((unsigned int)sCur * 0x23u + 3u + (unsigned int)DAT_006d4988));
              if (0 < iVar4) {
                do {
                  __builtin_memcpy(&sCur, DAT_0071b74a + iVar3, 2);
                  iVar5 = FUN_00449790((unsigned int)*(unsigned char *)((unsigned int)sCur * 0x23u + 3u + (unsigned int)DAT_006d4988), uVar7);
                  if ((iVar5 == 1) || (iVar5 == 3)) {
                    sTmp = (unsigned short)uVar7;
                    __builtin_memcpy(DAT_0071b759 + iVar3, &sTmp, 2);
                    break;
                  }
                  uVar7 = (unsigned int)(unsigned short)((short)uVar7 + 1);
                } while ((int)uVar7 < iVar4);
              }
            }
            __builtin_memcpy(&sTmp, DAT_0071bb4a + iVar3, 2);
            if (sTmp != 0) {
              __builtin_memcpy(&sTmp, DAT_0071b74a + iVar3, 2);
              __builtin_memcpy(DAT_0071bb27 + iVar3, &sTmp, 2);
            }
            DAT_0071b756[iVar3] = 0;
            __builtin_memcpy(&uTmp, DAT_0071b730 + iVar3, 4);
            if ((int)uTmp != -1) {
              __builtin_memcpy(&uTmp, (unsigned char *)(uTmp * 0x23u + 0x1fu + (unsigned int)DAT_006d4988), 4);
              __builtin_memcpy(&uP, (unsigned char *)(uTmp + 0xa7u), 4);
              sTmp = 0;
              __builtin_memcpy((unsigned char *)(uP + 0x231u), &sTmp, 2);
            }
            if (DAT_0071b743[iVar3] != '\0') {
              FUN_004a3830(uVar6, param_2, 1, 0);
              return;
            }
            __builtin_memcpy(&uTmp, DAT_0071b730 + iVar3, 4);
            __builtin_memcpy(&uTmp, (unsigned char *)(uTmp * 0x23u + 0x1fu + (unsigned int)DAT_006d4988), 4);
            __builtin_memcpy(&uP, (unsigned char *)(uTmp + 0xa7u), 4);
            if (((*(unsigned char *)(uP + 0x1dbu) & 1) != 0) &&
                ((((DAT_0071bb37[iVar3] == '\x13') || (DAT_0071bb37[iVar3] == '\0')) ||
                  (DAT_0071bb37[(int)((unsigned int)DAT_0071ecf4 * 0x5bdu)] == '\x14')))) {
              FUN_004a3830(uVar6, param_2, 1, 0);
              return;
            }
          }
        }
        else {
          _DAT_00732950 = 0x5f;
          _DAT_00732952 = 0x57;
          FUN_004a1d70(0x56, param_2);
        }
      }
    }
  }
  return;
}

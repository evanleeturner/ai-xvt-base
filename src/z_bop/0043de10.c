/* SPDX-License-Identifier: MIT */
/* z_bop 0043de10 */
void __cdecl FUN_00440760(int param_1, unsigned int param_2);

extern unsigned short DAT_00a08374;
extern unsigned int DAT_009e9670;
extern unsigned int DAT_009ecc34;
extern unsigned int DAT_009a1fe8;
extern unsigned int DAT_00a0a246;
extern unsigned char DAT_0052158b;
extern unsigned char DAT_00521588;
extern unsigned short DAT_005233ec;
extern int DAT_00521590;
extern int DAT_00521592;
extern int DAT_005215b8;
extern int DAT_005215ba;
extern short DAT_00a08ca0;
extern short DAT_00a08ca2;
extern void (__cdecl *DAT_009d130c)(unsigned int, short, short, unsigned short, unsigned int, unsigned short);
extern unsigned int DAT_009ed240;
extern unsigned short DAT_00a08f0c;
extern unsigned short DAT_00a08f08;
extern unsigned short DAT_00a08f0a;

void __stdcall FUN_0043de10(void)
{
  int iVar1;
  unsigned short uVar2;
  short sVar3;
  unsigned short uVar4;
  unsigned short uVar5;
  unsigned int uVar6;
  int iVar7;
  unsigned int uVar9;
  unsigned int uVar8;
  short sVar10;
  unsigned short uVar11;
  unsigned short uVar12;
  short sVar13;
  short local_e;
  
  uVar12 = DAT_00a08374 + 0x33;
  iVar1 = *(int *)(*(int *)(*(int *)((int)&DAT_009e9670 + DAT_009ecc34 * 0x5bd) * 0x23 + 0x1f +
                           DAT_009a1fe8) + 0xa9);
  if ((*(unsigned char *)(iVar1 + 0x1e1) & 0x10) != 0) {
    unsigned short tmp_23f;
    __builtin_memcpy(&tmp_23f, (void *)(iVar1 + 0x23f), 2);
    uVar11 = tmp_23f;
    if ((short)uVar11 < 0) {
      uVar11 = 0;
    }
    unsigned short tmp_1e5;
    __builtin_memcpy(&tmp_1e5, (void *)(iVar1 + 0x1e5), 2);
    uVar5 = tmp_1e5 & 0x100;
    if (uVar5 == 0) {
      uVar11 = 0;
    }
    uVar2 = (unsigned short)(*(char *)(iVar1 + 0x241) != '\0');
    if (uVar5 == 0) {
      uVar2 = 0;
    }
    FUN_00440760(DAT_00a08374 + 0x74, (unsigned int)uVar2);
    uVar6 = (unsigned int)(short)uVar11;
    if (DAT_00a0a246 != uVar6) {
      local_e = 0;
      DAT_00a0a246 = uVar11;
      do {
        iVar7 = (int)local_e;
        iVar1 = iVar7 * 5 + 5;
        uVar9 = (unsigned int)((unsigned int)(iVar1 * 5) >> 8);
        if (iVar1 * 200 < (int)uVar6) {
          uVar8 = (uVar9 << 8) | (unsigned int)DAT_0052158b;
        }
        else if ((short)uVar11 < 0) {
          uVar8 = (uVar9 << 8) | (unsigned int)DAT_00521588;
        }
        else {
          uVar5 = uVar11;
          if (1000 < (short)uVar11) {
            uVar5 = 1000;
          }
          uVar8 = ((uVar9 << 8) | (unsigned int)(&DAT_00521588)[(short)uVar5 / 0x14d]) & 0xffff00ff;
        }
        sVar13 = *(short *)(&DAT_00a08ca0 + (unsigned int)uVar12 * 0xc);
        sVar10 = *(short *)(&DAT_00a08ca2 + (unsigned int)uVar12 * 0xc);
        if (DAT_005233ec == 0x101) {
          sVar3 = (8 - local_e) * 3;
          sVar13 = sVar13 + sVar3;
          sVar10 = sVar10 + sVar3;
        }
        else if (DAT_005233ec == 0x112) {
          sVar13 = sVar13 + *(short *)(&DAT_00521590 + iVar7 * 4);
          sVar10 = sVar10 + *(short *)(&DAT_00521592 + iVar7 * 4);
        }
        else {
          sVar13 = sVar13 + *(short *)(&DAT_005215b8 + iVar7 * 4);
          sVar10 = sVar10 + *(short *)(&DAT_005215ba + iVar7 * 4);
        }
        if ((unsigned int)DAT_00521588 == (uVar8 & 0xff)) {
          uVar4 = 0;
        }
        else {
          uVar4 = *(unsigned short *)(&DAT_00a08f0c + (unsigned int)DAT_00a08374 * 0xc);
        }
        uVar11 = uVar11 - 1000;
        (*DAT_009d130c)(*(unsigned int *)
                         (&DAT_009ed240 +
                         ((unsigned int)*(unsigned short *)(&DAT_00a08f08 + (unsigned int)DAT_00a08374 * 0xc) + iVar7) * 4),
                        sVar13, sVar10, *(unsigned short *)(&DAT_00a08f0a + (unsigned int)DAT_00a08374 * 0xc),
                        uVar8 & 0xffff00ff, uVar4);
        local_e = local_e + 1;
      } while (local_e < 9);
    }
  }
  return;
}

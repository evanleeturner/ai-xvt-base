/* SPDX-License-Identifier: MIT */
/* z_bop 00439030 */
typedef unsigned char byte;
typedef unsigned short ushort;
typedef unsigned int uint;

extern uint DAT_009ecc34;
extern char DAT_009e9a77;
extern char DAT_009e9670;
extern int DAT_009a1fe8;
extern ushort DAT_00a08374;
extern char DAT_00a08ca4;
extern char DAT_00a08ca6;
extern char DAT_00a08ca0;
extern char DAT_00a08ca2;
extern char DAT_00a08ca8;
extern byte DAT_009a20ae;
extern char DAT_00a09f58;
extern char DAT_00a09250;
extern char DAT_00a09252;
extern char DAT_00a0924c;
extern char DAT_00a0924e;
extern char DAT_00a09254;
extern byte *DAT_00a0a134;
extern byte *DAT_00a0a130;
extern char DAT_00a09244;
extern char DAT_00a09246;
extern char DAT_00a09240;
extern char DAT_00a09242;
extern char DAT_00a09248;
extern byte *DAT_00a0a13c;
extern byte *DAT_00a0a138;
extern char DAT_005215e0;
extern char DAT_005215dc;
extern char DAT_00521540;
extern char DAT_00a08e8e;
extern char DAT_00a08e8c;
extern void (__cdecl *DAT_009a6fe4)(int);
extern char DAT_00a092a4;
extern char DAT_00a092a0;
extern char DAT_00a092a2;
extern char DAT_00a092a8;
extern char DAT_00a092aa;
extern char DAT_00a092a6;
extern char DAT_00a0a18c;
extern int DAT_005233ec;
extern char DAT_00a09288;
extern char DAT_00a0928a;
extern char DAT_00a09290;
extern char DAT_00a09292;
extern void (__cdecl *DAT_009d1300)(void);
extern char DAT_009a1ed0;
extern int DAT_009a7bac;
extern int DAT_009a8c14;
extern byte DAT_009ec464;
extern short DAT_00a08ec8;
extern short DAT_00a08eca;

void __cdecl FUN_004a9640(char);
void __cdecl FUN_004a9590(uint);
void __cdecl FUN_004a95c0(uint);
void __cdecl FUN_004a9500(short, short);
void __cdecl FUN_004a9520(short, short, short, short);
void __cdecl FUN_004a9960(byte *);
unsigned __int64 __cdecl FUN_004278c0(void *);
void __cdecl FUN_0043c290(ushort, byte);
void __cdecl FUN_004a9a50(void *);

void __cdecl FUN_00439030(uint param_1)
{
  ushort uVar1;
  short sVar2;
  short sVar3;
  short sVar4;
  short sVar5;
  int iVar6;
  byte *pbVar7;
  ushort uVar8;
  ushort uVar9;
  unsigned __int64 uVar10;
  char cVar11;
  byte bVar12;
  short uVar13;
  uint uVar14;
  int t0;
  int t1;
  int t2;

  if (DAT_009ecc34 == (param_1 & 0xffff)) {
    cVar11 = (&DAT_009e9a77)[(param_1 & 0xffff) * 0x5bd];
    if ((cVar11 == '\0') || (cVar11 == '\x13')) {
      __builtin_memcpy(&t0, &DAT_009e9670 + DAT_009ecc34 * 0x5bd, 4);
      t1 = (int)((uint)t0 * 0x23u + 0x1fu + (uint)DAT_009a1fe8);
      __builtin_memcpy(&t2, (char *)t1, 4);
      __builtin_memcpy(&iVar6, (char *)t2 + 0xa9, 4);
      __builtin_memcpy(&uVar9, (char *)iVar6 + 0x1e3, 2);
      __builtin_memcpy(&uVar1, (char *)iVar6 + 0x1e1, 2);
      FUN_004a9640('\0');
      uVar14 = 0x7a;
      FUN_004a9590(0x2f);
      do {
        iVar6 = (int)(((uint)DAT_00a08374 + uVar14) * 0xc);
        if (*(short *)(&DAT_00a08ca4 + iVar6) != 0) {
          switch (uVar14) {
          case 0x7a:
            uVar8 = uVar1 & 0x200;
            break;
          case 0x7b:
            if ((uVar9 & 1) == 0) goto LAB_004391e5;
            uVar8 = uVar1 & 0x800;
            break;
          case 0x7c:
            uVar8 = uVar1 & 0x400;
            break;
          case 0x7d:
            if ((uVar9 & 0x100) == 0) goto LAB_004391e5;
            uVar8 = uVar1 & 0x1000;
            break;
          default:
            goto switchD_00439101_default;
          }
          if (uVar8 != 0) {
switchD_00439101_default:
            FUN_004a95c0((uint)*(ushort *)(&DAT_00a08ca6 + iVar6));
            FUN_004a9500(*(short *)(&DAT_00a08ca0 + ((uint)DAT_00a08374 + uVar14) * 0xc),
                         *(short *)(&DAT_00a08ca2 + ((uint)DAT_00a08374 + uVar14) * 0xc));
            iVar6 = (int)(((uint)DAT_00a08374 + uVar14) * 0xc);
            FUN_004a9520(*(short *)(&DAT_00a08ca0 + iVar6), *(short *)(&DAT_00a08ca2 + iVar6),
                         (short)(*(short *)(&DAT_00a08ca8 + iVar6) + *(short *)(&DAT_00a08ca0 + iVar6)),
                         (short)((ushort)DAT_009a20ae + *(short *)(&DAT_00a08ca2 + iVar6)));
            FUN_004a9960(*(byte **)(&DAT_00a09f58 + uVar14 * 4));
          }
        }
LAB_004391e5:
        uVar8 = (ushort)((short)uVar14 + 1);
        uVar14 = (uint)uVar8;
      } while (uVar8 < 0x7e);
      if ((uVar1 & 0x40) != 0) {
        if (*(short *)(&DAT_00a09250 + (uint)DAT_00a08374 * 0xc) != 0) {
          if (DAT_00a08374 != 0) {
            FUN_004a9590(0x4a);
          }
          FUN_004a95c0((uint)*(ushort *)(&DAT_00a09252 + (uint)DAT_00a08374 * 0xc));
          FUN_004a9500(*(short *)(&DAT_00a0924c + (uint)DAT_00a08374 * 0xc),
                       *(short *)(&DAT_00a0924e + (uint)DAT_00a08374 * 0xc));
          iVar6 = (int)((uint)DAT_00a08374 * 0xc);
          FUN_004a9520(*(short *)(&DAT_00a0924c + iVar6), *(short *)(&DAT_00a0924e + iVar6),
                       (short)(*(short *)(&DAT_00a09254 + iVar6) + *(short *)(&DAT_00a0924c + iVar6)),
                       (short)((ushort)DAT_009a20ae + *(short *)(&DAT_00a0924e + iVar6)));
          pbVar7 = DAT_00a0a134;
          if (*(ushort *)(&DAT_00a09250 + (uint)DAT_00a08374 * 0xc) < 5) {
            pbVar7 = DAT_00a0a130;
          }
          FUN_004a9960(pbVar7);
        }
        if (*(short *)(&DAT_00a09244 + (uint)DAT_00a08374 * 0xc) != 0) {
          if (DAT_00a08374 != 0) {
            FUN_004a9590(0x4a);
          }
          FUN_004a95c0((uint)*(ushort *)(&DAT_00a09246 + (uint)DAT_00a08374 * 0xc));
          FUN_004a9500(*(short *)(&DAT_00a09240 + (uint)DAT_00a08374 * 0xc),
                       *(short *)(&DAT_00a09242 + (uint)DAT_00a08374 * 0xc));
          iVar6 = (int)((uint)DAT_00a08374 * 0xc);
          FUN_004a9520(*(short *)(&DAT_00a09240 + iVar6), *(short *)(&DAT_00a09242 + iVar6),
                       (short)(*(short *)(&DAT_00a09248 + iVar6) + *(short *)(&DAT_00a09240 + iVar6)),
                       (short)((ushort)DAT_009a20ae + *(short *)(&DAT_00a09242 + iVar6)));
          pbVar7 = DAT_00a0a13c;
          if (*(ushort *)(&DAT_00a09244 + (uint)DAT_00a08374 * 0xc) < 5) {
            pbVar7 = DAT_00a0a138;
          }
          FUN_004a9960(pbVar7);
        }
        if (DAT_00a08374 == 0) {
          cVar11 = '\x02';
        }
        else {
          cVar11 = '\0';
        }
        FUN_004a9640(cVar11);
        uVar10 = FUN_004278c0(&DAT_005215e0);
        sVar4 = (short)uVar10;
        FUN_004a9590(0x4a);
        FUN_004a95c0(0x2c);
        sVar2 = *(short *)(&DAT_00a08e8e + (uint)DAT_00a08374 * 0xc);
        sVar3 = *(short *)(&DAT_00a08e8c + (uint)DAT_00a08374 * 0xc);
        sVar5 = (short)((ushort)DAT_009a20ae + sVar2);
        uVar10 = FUN_004278c0(&DAT_005215dc);
        FUN_004a9520((short)(sVar3 + sVar4), sVar2, (short)((short)uVar10 + sVar3 + sVar4), sVar5);
        FUN_004a9500((short)(*(short *)(&DAT_00a08e8c + (uint)DAT_00a08374 * 0xc) + sVar4),
                     *(short *)(&DAT_00a08e8e + (uint)DAT_00a08374 * 0xc));
        (*DAT_009a6fe4)(0x25);
      }
      if ((uVar1 & 0x20) != 0) {
        uVar14 = 0;
        do {
          iVar6 = (int)(((uint)DAT_00a08374 + uVar14) * 0xc);
          if (*(short *)(&DAT_00a092a4 + ((uint)DAT_00a08374 + uVar14) * 0xc) != 0) {
            FUN_004a9520(*(short *)(&DAT_00a092a0 + iVar6), *(short *)(&DAT_00a092a2 + iVar6),
                         (short)(*(short *)(&DAT_00a092a8 + iVar6) + *(short *)(&DAT_00a092a0 + iVar6)),
                         (short)((ushort)DAT_009a20ae + *(short *)(&DAT_00a092a2 + iVar6)));
            FUN_004a9500(*(short *)(&DAT_00a092a0 + ((uint)DAT_00a08374 + uVar14) * 0xc),
                         *(short *)(&DAT_00a092a2 + ((uint)DAT_00a08374 + uVar14) * 0xc));
            FUN_004a9590((uint)*(ushort *)(&DAT_00a092aa + ((uint)DAT_00a08374 + uVar14) * 0xc));
            FUN_004a95c0((uint)*(ushort *)(&DAT_00a092a6 + ((uint)DAT_00a08374 + uVar14) * 0xc));
            FUN_004a9960(*(byte **)(&DAT_00a0a18c + uVar14 * 4));
          }
          uVar9 = (ushort)((short)uVar14 + 1);
          uVar14 = (uint)uVar9;
        } while (uVar9 < 2);
      }
      FUN_004a95c0(0x2c);
      FUN_004a9640('\0');
      if (DAT_005233ec == 0x13) {
        bVar12 = 7;
      }
      else {
        bVar12 = 3;
      }
      __builtin_memcpy(&t0, &DAT_009e9670 + DAT_009ecc34 * 0x5bd, 4);
      FUN_0043c290((ushort)t0, bVar12);
      FUN_004a9500(*(short *)(&DAT_00a09288 + (uint)DAT_00a08374 * 0xc),
                   *(short *)(&DAT_00a0928a + (uint)DAT_00a08374 * 0xc));
      iVar6 = (int)((uint)DAT_00a08374 * 0xc);
      FUN_004a9520(*(short *)(&DAT_00a09288 + iVar6), *(short *)(&DAT_00a0928a + iVar6),
                   (short)(*(short *)(&DAT_00a09290 + iVar6) + *(short *)(&DAT_00a09288 + iVar6)),
                   (short)(*(short *)(&DAT_00a09292 + iVar6) + *(short *)(&DAT_00a0928a + iVar6)));
      (*DAT_009d1300)();
      FUN_004a9a50(&DAT_009a1ed0);
      if ((&DAT_009e9a77)[(param_1 & 0xffff) * 0x5bd] == '\0') {
        FUN_004a9640('\x02');
        FUN_004a9520(0, 0, (short)DAT_009a7bac, (short)DAT_009a8c14);
        FUN_004a95c0(0x40);
        if (DAT_005233ec == 0x13) {
          uVar14 = 0x4d;
        }
        else {
          uVar14 = 0x4e;
        }
        FUN_004a9590(uVar14);
        DAT_009ec464 = 0;
        uVar13 = DAT_00a08eca;
        uVar10 = FUN_004278c0(&DAT_00521540);
        FUN_004a9500((short)((short)uVar10 + DAT_00a08ec8), uVar13);
        (*DAT_009a6fe4)(0x3a);
      }
    }
  }
  return;
}

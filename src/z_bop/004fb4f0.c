/* SPDX-License-Identifier: MIT */
/* z_bop 004fb4f0 */
void __stdcall FUN_004d4dd0(void);
void __cdecl FUN_004d4c00(int);
void __stdcall FUN_004de110(void);
void __stdcall FUN_004d4bf0(void);
void __stdcall FUN_004d4f60(void);
void __cdecl FUN_004dada0(int);
void __cdecl FUN_004df700(const char *);
void __cdecl FUN_004b4a60(const char *, const char *);
void __cdecl FUN_004b4b20(const char *, const char *, int, int);
void __stdcall FUN_004d3330(void);
void __stdcall FUN_004d2e50(void);
void __cdecl FUN_004d34a0(int);
void __cdecl FUN_004d3020(int, int, int);

extern const char *s_sfx_sfx_lst_0052b12c;
extern const char *s_frontres_credits_bmp_0052d180;
extern const char *s_background_0052bf74;
extern const char *s_frontres_leclogo_bmp_0052d160;
extern const char *s_leclogo_0052d178;
extern const char *s_frontres_totallyg_bmp_0052d13c;
extern const char *s_totallylogo_0052d154;
extern const char *s_frontres_comp01_bmp_0052d120;
extern const char *s_comp01_0052d134;
extern const char *s_frontres_test_bmp_0052d104;
extern const char *s_testers_0052d118;
extern const char *s_frontres_jbrs_bmp_0052d0e8;
extern const char *s_lakota_0052d0fc;
extern const char *s_frontres_arts_bmp_0052d0cc;
extern const char *s_artists_0052d0e0;

unsigned int __stdcall FUN_004fb4f0(void)
{
  FUN_004d4dd0();
  FUN_004d4c00(0);
  FUN_004de110();
  FUN_004d4bf0();
  FUN_004d4f60();
  FUN_004dada0(0xf);
  FUN_004df700(s_sfx_sfx_lst_0052b12c);
  FUN_004b4a60(s_frontres_credits_bmp_0052d180, s_background_0052bf74);
  FUN_004b4b20(s_frontres_leclogo_bmp_0052d160, s_leclogo_0052d178, 0, 0);
  FUN_004b4b20(s_frontres_totallyg_bmp_0052d13c, s_totallylogo_0052d154, 0, 0);
  FUN_004b4a60(s_frontres_comp01_bmp_0052d120, s_comp01_0052d134);
  FUN_004b4a60(s_frontres_test_bmp_0052d104, s_testers_0052d118);
  FUN_004b4a60(s_frontres_jbrs_bmp_0052d0e8, s_lakota_0052d0fc);
  FUN_004b4a60(s_frontres_arts_bmp_0052d0cc, s_artists_0052d0e0);
  FUN_004d3330();
  FUN_004d2e50();
  FUN_004d34a0(0x8000);
  FUN_004d3020(4, 0, 0);
  return 0;
}

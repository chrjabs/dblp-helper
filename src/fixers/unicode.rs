use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    /// Unicode to LaTeX map taken from
    /// https://github.com/phfaist/pylatexenc/blob/main/pylatexenc/latexencode/_uni2latexmap.py, where
    /// it was originally taken from latexcodec
    static ref UNICODE_2_LATEX: HashMap<char, &'static str> = HashMap::from_iter([
        ('"', "''"),
        ('#', r"\#"),
        ('$', r"\$"),
        ('%', r"\%"),
        ('&', r"\&"),
        ('<', r"\ensuremath{<}"),
        ('>', r"\ensuremath{>}"),
        ('\\', r"\textbackslash"),
        ('^', r"\textasciicircum"),
        ('_', r"\_"),
        ('{', r"\{"),
        ('}', r"\}"),
        ('~', r"\textasciitilde"),
        ('\u{00A0}', r"~"), // NO-BREAK SPACE
        ('¡', r"\textexclamdown"),
        ('¢', r"\textcent"),
        ('£', r"\textsterling"),
        ('¤', r"\textcurrency"),
        ('¥', r"\textyen"),
        ('¦', r"\textbrokenbar"),
        ('§', r"\textsection"),
        ('¨', r"\textasciidieresis"),
        ('©', r"\textcopyright"),
        ('ª', r"\textordfeminine"),
        ('«', r"\guillemotleft"),
        ('¬', r"\textlnot"),
        ('\u{00AD}', r"\-"), // SOFT HYPHEN
        ('®', r"\textregistered"),
        ('¯', r"\textasciimacron"),
        ('°', r"\textdegree"),
        ('±', r"\ensuremath{\pm}"),
        ('²', r"\texttwosuperior"),
        ('³', r"\textthreesuperior"),
        ('´', r"\textasciiacute"),
        ('µ', r"\textmu"),
        ('¶', r"\textparagraph"),
        ('·', r"\textperiodcentered"),
        ('¹', r"\textonesuperior"),
        ('º', r"\textordmasculine"),
        ('»', r"\guillemotright"),
        ('¼', r"\textonequarter"),
        ('½', r"\textonehalf"),
        ('¾', r"\textthreequarters"),
        ('¿', r"\textquestiondown"),
        ('À', r"\`A"),
        ('Á', r"\'A"),
        ('Â', r"\^A"),
        ('Ã', r"\~A"),
        ('Ä', r#"\"A"#),
        ('Å', r"\r{A}"),
        ('Æ', r"\AE"),
        ('Ç', r"\c{C}"),
        ('È', r"\`E"),
        ('É', r"\'E"),
        ('Ê', r"\^E"),
        ('Ë', r#"\"E"#),
        ('Ì', r"\`I"),
        ('Í', r"\'I"),
        ('Î', r"\^I"),
        ('Ï', r#"\"I"#),
        ('Ð', r"\DH"),
        ('Ñ', r"\~N"),
        ('Ò', r"\`O"),
        ('Ó', r"\'O"),
        ('Ô', r"\^O"),
        ('Õ', r"\~O"),
        ('Ö', r#"\"O"#),
        ('×', r"\texttimes"),
        ('Ø', r"\O"),
        ('Ù', r"\`U"),
        ('Ú', r"\'U"),
        ('Û', r"\^U"),
        ('Ü', r#"\"U"#),
        ('Ý', r"\'Y"),
        ('Þ', r"\TH"),
        ('ß', r"\ss"),
        ('à', r"\`a"),
        ('á', r"\'a"),
        ('â', r"\^a"),
        ('ã', r"\~a"),
        ('ä', r#"\"a"#),
        ('å', r"\r{a}"),
        ('æ', r"\ae"),
        ('ç', r"\c{c}"),
        ('è', r"\`e"),
        ('é', r"\'e"),
        ('ê', r"\^e"),
        ('ë', r#"\"e"#),
        ('ì', r"\`i"),
        ('í', r"\'i"),
        ('î', r"\^i"),
        ('ï', r#"\"i"#),
        ('ð', r"\dh"),
        ('ñ', r"\~n"),
        ('ò', r"\`o"),
        ('ó', r"\'o"),
        ('ô', r"\^o"),
        ('õ', r"\~o"),
        ('ö', r#"\"o"#),
        ('÷', r"\textdiv"),
        ('ø', r"\o"),
        ('ù', r"\`u"),
        ('ú', r"\'u"),
        ('û', r"\^u"),
        ('ü', r#"\"u"#),
        ('ý', r"\'y"),
        ('þ', r"\th"),
        ('ÿ', r#"\"y"#),
        ('\u{0100}', r"\={A}"),
        ('\u{0101}', r"\={a}"),
        ('\u{0102}', r"\u{A}"),
        ('\u{0103}', r"\u{a}"),
        ('\u{0104}', r"\k{A}"),
        ('\u{0105}', r"\k{a}"),
        ('\u{0106}', "\\'C"),
        ('\u{0107}', "\\'c"),
        ('\u{0108}', r"\^{C}"),
        ('\u{0109}', r"\^{c}"),
        ('\u{010A}', r"\.{C}"),
        ('\u{010B}', r"\.{c}"),
        ('\u{010C}', r"\v{C}"),
        ('\u{010D}', r"\v{c}"),
        ('\u{010E}', r"\v{D}"),
        ('\u{010F}', r"\v{d}"),
        ('\u{0110}', r"\DJ"),
        ('\u{0111}', r"\dj"),
        ('\u{0112}', r"\={E}"),
        ('\u{0113}', r"\={e}"),
        ('\u{0114}', r"\u{E}"),
        ('\u{0115}', r"\u{e}"),
        ('\u{0116}', r"\.{E}"),
        ('\u{0117}', r"\.{e}"),
        ('\u{0118}', r"\k{E}"),
        ('\u{0119}', r"\k{e}"),
        ('\u{011A}', r"\v{E}"),
        ('\u{011B}', r"\v{e}"),
        ('\u{011C}', r"\^{G}"),
        ('\u{011D}', r"\^{g}"),
        ('\u{011E}', r"\u{G}"),
        ('\u{011F}', r"\u{g}"),
        ('\u{0120}', r"\.{G}"),
        ('\u{0121}', r"\.{g}"),
        ('\u{0122}', r"\c{G}"),
        ('\u{0123}', r"\c{g}"),
        ('\u{0124}', r"\^{H}"),
        ('\u{0125}', r"\^{h}"),
        ('\u{0126}', r"\={H}"),
        ('\u{0127}', r"\={h}"),
        ('\u{0128}', r"\~{I}"),
        ('\u{0129}', r"\~{i}"),
        ('\u{012A}', r"\={I}"),
        ('\u{012B}', r"\={i}"),
        ('\u{012C}', r"\u{I}"),
        ('\u{012D}', r"\u{i}"),
        ('\u{012E}', r"\k{I}"),
        ('\u{012F}', r"\k{i}"),
        ('\u{0130}', r"\.I"),
        ('\u{0131}', r"\i"),
        ('\u{0132}', r"\IJ"),
        ('\u{0133}', r"\ij"),
        ('\u{0134}', r"\^{J}"),
        ('\u{0135}', r"\^{j}"),
        ('\u{0136}', r"\c{K}"),
        ('\u{0137}', r"\c{k}"),
        ('\u{0138}', r"\textsc{k}"),
        ('\u{0139}', "\\'L"),
        ('\u{013A}', "\\'l"),
        ('\u{013B}', r"\c{L}"),
        ('\u{013C}', r"\c{l}"),
        ('\u{013D}', r"\v{L}"),
        ('\u{013E}', r"\v{l}"),
        ('\u{013F}', r"\.{L}"),
        ('\u{0140}', r"\.{l}"),
        ('\u{0141}', r"\L"),
        ('\u{0142}', r"\l"),
        ('\u{0143}', "\\'N"),
        ('\u{0144}', "\\'n"),
        ('\u{0145}', r"\c{N}"),
        ('\u{0146}', r"\c{n}"),
        ('\u{0147}', r"\v{N}"),
        ('\u{0148}', r"\v{n}"),
        ('\u{0149}', r"\nument{149}"),
        ('\u{014A}', r"\NG"),
        ('\u{014B}', r"\ng"),
        ('\u{014C}', r"\={O}"),
        ('\u{014D}', r"\={o}"),
        ('\u{014E}', r"\u{O}"),
        ('\u{014F}', r"\u{o}"),
        ('\u{0150}', r"\H{O}"),
        ('\u{0151}', r"\H{o}"),
        ('\u{0152}', r"\OE"),
        ('\u{0153}', r"\oe"),
        ('\u{0154}', "\\'R"),
        ('\u{0155}', "\\'r"),
        ('\u{0156}', r"\c{R}"),
        ('\u{0157}', r"\c{r}"),
        ('\u{0158}', r"\v{R}"),
        ('\u{0159}', r"\v{r}"),
        ('\u{015A}', "\\'S"),
        ('\u{015B}', "\\'s"),
        ('\u{015C}', r"\^{S}"),
        ('\u{015D}', r"\^{s}"),
        ('\u{015E}', r"\c{S}"),
        ('\u{015F}', r"\c{s}"),
        ('\u{0160}', r"\v{S}"),
        ('\u{0161}', r"\v{s}"),
        ('\u{0162}', r"\c{T}"),
        ('\u{0163}', r"\c{t}"),
        ('\u{0164}', r"\v{T}"),
        ('\u{0165}', r"\v{t}"),
        ('\u{0166}', r"\={T}"),
        ('\u{0167}', r"\={t}"),
        ('\u{0168}', r"\~{U}"),
        ('\u{0169}', r"\~{u}"),
        ('\u{016A}', r"\={U}"),
        ('\u{016B}', r"\={u}"),
        ('\u{016C}', r"\u{U}"),
        ('\u{016D}', r"\u{u}"),
        ('\u{016E}', r"\r{U}"),
        ('\u{016F}', r"\r{u}"),
        ('\u{0170}', "\\'{U}"),
        ('\u{0171}', "\\'{u}"),
        ('\u{0172}', r"\k{U}"),
        ('\u{0173}', r"\k{u}"),
        ('\u{0174}', r"\^{W}"),
        ('\u{0175}', r"\^{w}"),
        ('\u{0176}', r"\^{Y}"),
        ('\u{0177}', r"\^{y}"),
        ('\u{0178}', r#"\"Y"#),
        ('\u{0179}', "\\'Z"),
        ('\u{017A}', "\\'z"),
        ('\u{017B}', r"\.Z"),
        ('\u{017C}', r"\.z"),
        ('\u{017D}', r"\v{Z}"),
        ('\u{017E}', r"\v{z}"),
        ('\u{0192}', r"\textflorin"),
        ('ƕ', r"\texthvlig"),
        ('ƞ', r"\textnrleg"),
        ('ǧ', r"\v{g}"),
        ('ǵ', r"\'{g}"),
        ('\u{0228}', r"\c{E}"),
        ('\u{0229}', r"\c{e}"),
        ('\u{0259}', r"\textschwa"),
        ('ɛ', r"\varepsilon"),
        ('ɸ', r"\textphi"),
        ('\u{0294}', r"\textglotstop"),
        ('ʞ', r"\textturnk"),
        ('\u{02B7}', r"\textsuperscript{w}"),
        ('\u{02C6}', r"\textasciicircum"),
        ('\u{02C7}', r"\textasciicaron"),
        ('\u{02D8}', r"\textasciibreve"),
        ('˙', r"\textperiodcentered"),
        ('˚', r"\r{}"),
        ('˛', r"\k{}"),
        ('\u{02DC}', r"\textasciitilde"),
        ('\u{02DD}', r"\textacutedbl"),
        ('\u{02BC}', "'"), // MODIFIER LETTER APOSTROPHE
        ('\u{0307}', r"\ensuremath{\dot{}}"),
        ('\u{0308}', r"\ensuremath{\ddot{}}"),
        ('Ά', r"\'{}A"),
        ('Έ', r"\'{}E"),
        ('Ή', r"\'{}H"),
        ('Ί', r"\'{}I"),
        ('Ό', r"\'{}O"),
        ('Ύ', r"\'{}Y"),
        ('Ώ', r"\'{}\ensuremath{\Omega}"),
        ('ΐ', r"\acute{\ddot{\iota}}"),
        ('\u{0391}', r"A"),
        ('\u{0392}', r"B"),
        ('\u{0393}', r"\ensuremath{\Gamma}"),
        ('\u{0394}', r"\ensuremath{\Delta}"),
        ('\u{0395}', r"E"),
        ('\u{0396}', r"Z"),
        ('\u{0397}', r"H"),
        ('\u{0398}', r"\ensuremath{\Theta}"),
        ('\u{0399}', r"I"),
        ('\u{039A}', r"K"),
        ('\u{039B}', r"\ensuremath{\Lambda}"),
        ('\u{039C}', r"M"),
        ('\u{039D}', r"N"),
        ('\u{039E}', r"\ensuremath{\Xi}"),
        ('\u{039F}', r"O"),
        ('\u{03A0}', r"\ensuremath{\Pi}"),
        ('\u{03A1}', r"P"),
        ('\u{03A3}', r"\ensuremath{\Sigma}"),
        ('\u{03A4}', r"T"),
        ('\u{03A5}', r"\ensuremath{\Upsilon}"),
        ('\u{03A6}', r"\ensuremath{\Phi}"),
        ('\u{03A7}', r"X"),
        ('\u{03A8}', r"\ensuremath{\Psi}"),
        ('\u{03A9}', r"\ensuremath{\Omega}"),
        ('Ϊ', r"\ensuremath{\ddot{I}}"),
        ('Ϋ', r"\ensuremath{\ddot{Y}}"),
        ('ά', r"\ensuremath{\acute\alpha}"),
        ('έ', r"\ensuremath{\acute\epsilon}"),
        ('ή', r"\ensuremath{\acute\eta}"),
        ('ί', r"\ensuremath{\acute\iota}"),
        ('ΰ', r"\ensuremath{\acute{\ddot{\upsilon}}}"),
        ('ϊ', r"\ensuremath{\ddot\iota}"),
        ('ϋ', r"\ensuremath{\ddot{\upsilon}}"),
        ('ό', r"\'{o}"),
        ('ύ', r"\ensuremath{\acute\upsilon}"),
        ('ώ', r"\ensuremath{\acute\omega}"),
        ('\u{03B1}', r"\ensuremath{\alpha}"),
        ('\u{03B2}', r"\ensuremath{\beta}"),
        ('\u{03B3}', r"\ensuremath{\gamma}"),
        ('\u{03B4}', r"\ensuremath{\delta}"),
        ('\u{03B5}', r"\ensuremath{\varepsilon}"),
        ('\u{03B6}', r"\ensuremath{\zeta}"),
        ('\u{03B7}', r"\ensuremath{\eta}"),
        ('\u{03B8}', r"\ensuremath{\theta}"),
        ('\u{03B9}', r"\ensuremath{\iota}"),
        ('\u{03BA}', r"\ensuremath{\kappa}"),
        ('\u{03BB}', r"\ensuremath{\lambda}"),
        ('\u{03BC}', r"\ensuremath{\mu}"),
        ('\u{03BD}', r"\ensuremath{\nu}"),
        ('\u{03BE}', r"\ensuremath{\xi}"),
        ('\u{03BF}', r"o"),
        ('\u{03C0}', r"\ensuremath{\pi}"),
        ('\u{03C1}', r"\ensuremath{\rho}"),
        ('\u{03C2}', r"\ensuremath{\varsigma}"),
        ('\u{03C3}', r"\ensuremath{\sigma}"),
        ('\u{03C4}', r"\ensuremath{\tau}"),
        ('\u{03C5}', r"\ensuremath{\upsilon}"),
        ('\u{03C6}', r"\ensuremath{\varphi}"),
        ('\u{03C7}', r"\ensuremath{\chi}"),
        ('\u{03C8}', r"\ensuremath{\psi}"),
        ('\u{03C9}', r"\ensuremath{\omega}"),
        ('ϑ', r"\ensuremath{\vartheta}"),
        ('ϒ', r"\Upsilon"),
        ('ϕ', r"\ensuremath{\phi}"),
        ('ϖ', r"\ensuremath{\varpi}"),
        ('ϰ', r"\ensuremath{\varkappa}"),
        ('ϱ', r"\ensuremath{\varrho}"),
        ('ϵ', r"\ensuremath{\epsilon}"),
        ('϶', r"\ensuremath{\backepsilon}"),
        ('\u{0400}', r"\`\CYRE"),                              // 0x0400
        ('\u{0401}', r"\CYRYO"),
        ('\u{0402}', r"\CYRDJE"),
        ('\u{0403}', r"\`\CYRG"),
        ('\u{0404}', r"\CYRIE"),
        ('\u{0405}', r"\CYRDZE"),
        ('\u{0406}', r"\CYRII"),
        ('\u{0407}', r"\CYRYI"),
        ('\u{0408}', r"\CYRJE"),
        ('\u{0409}', r"\CYRLJE"),
        ('\u{040A}', r"\CYRNJE"),
        ('\u{040B}', r"\CYRTSHE"),
        ('\u{040C}', r"\`\CYRK"),
        ('\u{040D}', r"\`\CYRI"),
        ('\u{040E}', r"\CYRUSHRT"),
        ('\u{040F}', r"\CYRDZHE"),
        ('\u{0410}', r"\CYRA"),
        ('\u{0411}', r"\CYRB"),
        ('\u{0412}', r"\CYRV"),
        ('\u{0413}', r"\CYRG"),
        ('\u{0414}', r"\CYRD"),
        ('\u{0415}', r"\CYRE"),
        ('\u{0416}', r"\CYRZH"),
        ('\u{0417}', r"\CYRZ"),
        ('\u{0418}', r"\CYRI"),
        ('\u{0419}', r"\CYRISHRT"),
        ('\u{041A}', r"\CYRK"),
        ('\u{041B}', r"\CYRL"),
        ('\u{041C}', r"\CYRM"),
        ('\u{041D}', r"\CYRN"),
        ('\u{041E}', r"\CYRO"),
        ('\u{041F}', r"\CYRP"),
        ('\u{0420}', r"\CYRR"),
        ('\u{0421}', r"\CYRS"),
        ('\u{0422}', r"\CYRT"),
        ('\u{0423}', r"\CYRU"),
        ('\u{0424}', r"\CYRF"),
        ('\u{0425}', r"\CYRH"),
        ('\u{0426}', r"\CYRC"),
        ('\u{0427}', r"\CYRCH"),
        ('\u{0428}', r"\CYRSH"),
        ('\u{0429}', r"\CYRSHCH"),
        ('\u{042A}', r"\CYRHRDSN"),
        ('\u{042B}', r"\CYRERY"),
        ('\u{042C}', r"\CYRSFTSN"),
        ('\u{042D}', r"\CYREREV"),
        ('\u{042E}', r"\CYRYU"),
        ('\u{042F}', r"\CYRYA"),
        ('\u{0430}', r"\cyra"),
        ('\u{0431}', r"\cyrb"),
        ('\u{0432}', r"\cyrv"),
        ('\u{0433}', r"\cyrg"),
        ('\u{0434}', r"\cyrd"),
        ('\u{0435}', r"\cyre"),
        ('\u{0436}', r"\cyrzh"),
        ('\u{0437}', r"\cyrz"),
        ('\u{0438}', r"\cyri"),
        ('\u{0439}', r"\cyrishrt"),
        ('\u{043A}', r"\cyrk"),
        ('\u{043B}', r"\cyrl"),
        ('\u{043C}', r"\cyrm"),
        ('\u{043D}', r"\cyrn"),
        ('\u{043E}', r"\cyro"),
        ('\u{043F}', r"\cyrp"),
        ('\u{0440}', r"\cyrr"),
        ('\u{0441}', r"\cyrs"),
        ('\u{0442}', r"\cyrt"),
        ('\u{0443}', r"\cyru"),
        ('\u{0444}', r"\cyrf"),
        ('\u{0445}', r"\cyrh"),
        ('\u{0446}', r"\cyrc"),
        ('\u{0447}', r"\cyrch"),
        ('\u{0448}', r"\cyrsh"),
        ('\u{0449}', r"\cyrshch"),
        ('\u{044A}', r"\cyrhrdsn"),
        ('\u{044B}', r"\cyrery"),
        ('\u{044C}', r"\cyrsftsn"),
        ('\u{044D}', r"\cyrerev"),
        ('\u{044E}', r"\cyryu"),
        ('\u{044F}', r"\cyrya"),
        ('\u{0450}', r"\`\cyre"),
        ('\u{0451}', r"\cyryo"),
        ('\u{0452}', r"\cyrdje"),
        ('\u{0453}', r"\`\cyrg"),
        ('\u{0454}', r"\cyrie"),
        ('\u{0455}', r"\cyrdze"),
        ('\u{0456}', r"\cyrii"),
        ('\u{0457}', r"\cyryi"),
        ('\u{0458}', r"\cyrje"),
        ('\u{0459}', r"\cyrlje"),
        ('\u{045A}', r"\cyrnje"),
        ('\u{045B}', r"\cyrtshe"),
        ('\u{045C}', r"\`\cyrk"),
        ('\u{045D}', r"\`\cyri"),
        ('\u{045E}', r"\cyrushrt"),
        ('\u{045F}', r"\cyrdzhe"),
        ('\u{0460}', r"\cyrchar\CYROMEGA"),                     // CYRILLIC CAPITAL LETTER OMEGA [Ѡ]
        ('\u{0461}', r"\cyrchar\cyromega"),                     // CYRILLIC SMALL LETTER OMEGA [ѡ]
        ('\u{0462}', r"\CYRYAT"),
        ('\u{0463}', r"\cyryat"),
        ('\u{0464}', r"\cyrchar\CYRIOTE"),                      // CYRILLIC CAPITAL LETTER IOTIFIED E [Ѥ]
        ('\u{0465}', r"\cyrchar\cyriote"),                      // CYRILLIC SMALL LETTER IOTIFIED E [ѥ]
        ('\u{0466}', r"\cyrchar\CYRLYUS"),                      // CYRILLIC CAPITAL LETTER LITTLE YUS [Ѧ]
        ('\u{0467}', r"\cyrchar\cyrlyus"),                      // CYRILLIC SMALL LETTER LITTLE YUS [ѧ]
        ('\u{0468}', r"\cyrchar\CYRIOTLYUS"),                   // CYRILLIC CAPITAL LETTER IOTIFIED LITTLE YUS [Ѩ]
        ('\u{0469}', r"\cyrchar\cyriotlyus"),                   // CYRILLIC SMALL LETTER IOTIFIED LITTLE YUS [ѩ]
        ('\u{046A}', r"\CYRBYUS"),
        ('\u{046B}', r"\cyrbyus"),
        ('\u{046C}', r"\cyrchar\CYRIOTBYUS"),                   // CYRILLIC CAPITAL LETTER IOTIFIED BIG YUS [Ѭ]
        ('\u{046D}', r"\cyrchar\cyriotbyus"),                   // CYRILLIC SMALL LETTER IOTIFIED BIG YUS [ѭ]
        ('\u{046E}', r"\cyrchar\CYRKSI"),                       // CYRILLIC CAPITAL LETTER KSI [Ѯ]
        ('\u{046F}', r"\cyrchar\cyrksi"),                       // CYRILLIC SMALL LETTER KSI [ѯ]
        ('\u{0470}', r"\cyrchar\CYRPSI"),                       // CYRILLIC CAPITAL LETTER PSI [Ѱ]
        ('\u{0471}', r"\cyrchar\cyrpsi"),                       // CYRILLIC SMALL LETTER PSI [ѱ]
        ('\u{0472}', r"\CYRFITA"),
        ('\u{0473}', r"\cyrfita"),
        ('\u{0474}', r"\CYRIZH"),
        ('\u{0475}', r"\cyrizh"),
        ('\u{0476}', r"\C\CYRIZH"),
        ('\u{0477}', r"\C\cyrizh"),
        ('\u{0478}', r"\cyrchar\CYRUK"),                        // CYRILLIC CAPITAL LETTER UK [Ѹ]
        ('\u{0479}', r"\cyrchar\cyruk"),                        // CYRILLIC SMALL LETTER UK [ѹ]
        ('\u{047A}', r"\cyrchar\CYROMEGARND"),                  // CYRILLIC CAPITAL LETTER ROUND OMEGA [Ѻ]
        ('\u{047B}', r"\cyrchar\cyromegarnd"),                  // CYRILLIC SMALL LETTER ROUND OMEGA [ѻ]
        ('\u{047C}', r"\cyrchar\CYROMEGATITLO"),                // CYRILLIC CAPITAL LETTER OMEGA WITH TITLO [Ѽ]
        ('\u{047D}', r"\cyrchar\cyromegatitlo"),                // CYRILLIC SMALL LETTER OMEGA WITH TITLO [ѽ]
        ('\u{047E}', r"\cyrchar\CYROT"),                        // CYRILLIC CAPITAL LETTER OT [Ѿ]
        ('\u{047F}', r"\cyrchar\cyrot"),                        // CYRILLIC SMALL LETTER OT [ѿ]
        ('\u{0480}', r"\cyrchar\CYRKOPPA"),                     // CYRILLIC CAPITAL LETTER KOPPA [Ҁ]
        ('\u{0481}', r"\cyrchar\cyrkoppa"),                     // CYRILLIC SMALL LETTER KOPPA [ҁ]
        ('\u{0482}', r"\cyrchar\cyrthousands"),                 // CYRILLIC THOUSANDS SIGN [҂]
        ('\u{0488}', r"\cyrchar\cyrhundredthousands"),          // COMBINING CYRILLIC HUNDRED THOUSANDS SIGN [҈]
        ('\u{0489}', r"\cyrchar\cyrmillions"),                  // COMBINING CYRILLIC MILLIONS SIGN [҉]
        ('\u{048C}', r"\CYRSEMISFTSN"),
        ('\u{048D}', r"\cyrsemisftsn"),
        ('\u{048E}', r"\CYRRTICK"),
        ('\u{048F}', r"\cyrrtick"),
        ('\u{0490}', r"\CYRGUP"),
        ('\u{0491}', r"\cyrgup"),
        ('\u{0492}', r"\CYRGHCRS"),
        ('\u{0493}', r"\cyrghcrs"),
        ('\u{0494}', r"\CYRGHK"),
        ('\u{0495}', r"\cyrghk"),
        ('\u{0496}', r"\CYRZHDSC"),
        ('\u{0497}', r"\cyrzhdsc"),
        ('\u{0498}', r"\CYRZDSC"),
        ('\u{0499}', r"\cyrzdsc"),
        ('\u{049A}', r"\CYRKDSC"),
        ('\u{049B}', r"\cyrkdsc"),
        ('\u{049C}', r"\CYRKVCRS"),
        ('\u{049D}', r"\cyrkvcrs"),
        ('\u{049E}', r"\CYRKHCRS"),
        ('\u{049F}', r"\cyrkhcrs"),
        ('\u{04A0}', r"\CYRKBEAK"),
        ('\u{04A1}', r"\cyrkbeak"),
        ('\u{04A2}', r"\CYRNDSC"),
        ('\u{04A3}', r"\cyrndsc"),
        ('\u{04A4}', r"\CYRNG"),
        ('\u{04A5}', r"\cyrng"),
        ('\u{04A6}', r"\CYRPHK"),
        ('\u{04A7}', r"\cyrphk"),
        ('\u{04A8}', r"\CYRABHHA"),
        ('\u{04A9}', r"\cyrabhha"),
        ('\u{04AA}', r"\CYRSDSC"),
        ('\u{04AB}', r"\cyrsdsc"),
        ('\u{04AC}', r"\CYRTDSC"),
        ('\u{04AD}', r"\cyrtdsc"),
        ('\u{04AE}', r"\CYRY"),
        ('\u{04AF}', r"\cyry"),
        ('\u{04B0}', r"\CYRYHCRS"),
        ('\u{04B1}', r"\cyryhcrs"),
        ('\u{04B2}', r"\CYRHDSC"),
        ('\u{04B3}', r"\cyrhdsc"),
        ('\u{04B4}', r"\CYRTETSE"),
        ('\u{04B5}', r"\cyrtetse"),
        ('\u{04B6}', r"\CYRCHRDSC"),
        ('\u{04B7}', r"\cyrchrdsc"),
        ('\u{04B8}', r"\CYRCHVCRS"),
        ('\u{04B9}', r"\cyrchvcrs"),
        ('\u{04BA}', r"\CYRSHHA"),
        ('\u{04BB}', r"\cyrshha"),
        ('\u{04BC}', r"\CYRABHCH"),
        ('\u{04BD}', r"\cyrabhch"),
        ('\u{04BE}', r"\CYRABHCHDSC"),
        ('\u{04BF}', r"\cyrabhchdsc"),
        ('\u{04C0}', r"\CYRpalochka"),
        ('\u{04C1}', r"\U\CYRZH"),
        ('\u{04C2}', r"\U\cyrzh"),
        ('\u{04C3}', r"\CYRKHK"),
        ('\u{04C4}', r"\cyrkhk"),
        ('\u{04C5}', r"\CYRLDSC"),
        ('\u{04C6}', r"\cyrldsc"),
        ('\u{04C7}', r"\CYRNHK"),
        ('\u{04C8}', r"\cyrnhk"),
        ('\u{04CB}', r"\CYRCHLDSC"),
        ('\u{04CC}', r"\cyrchldsc"),
        ('\u{04CD}', r"\CYRMDSC"),
        ('\u{04CE}', r"\cyrmdsc"),
        ('\u{04D0}', r"\U\CYRA"),
        ('\u{04D1}', r"\U\cyra"),
        ('\u{04D2}', r#"\"\CYRA"#),
        ('\u{04D3}', r#"\"\cyra"#),
        ('\u{04D4}', r"\CYRAE"),
        ('\u{04D5}', r"\cyrae"),
        ('\u{04D6}', r"\U\CYRE"),
        ('\u{04D7}', r"\U\cyre"),
        ('\u{04D8}', r"\CYRSCHWA"),
        ('\u{04D9}', r"\cyrschwa"),
        ('\u{04DA}', r#"\"\CYRSCHWA"#),
        ('\u{04DB}', r#"\"\cyrschwa"#),
        ('\u{04DC}', r#"\"\CYRZH"#),
        ('\u{04DD}', r#"\"\cyrzh"#),
        ('\u{04DE}', r#"\"\CYRZ"#),
        ('\u{04DF}', r#"\"\cyrz"#),
        ('\u{04E0}', r"\CYRABHDZE"),
        ('\u{04E1}', r"\cyrabhdze"),
        ('\u{04E2}', r"\=\CYRI"),
        ('\u{04E3}', r"\=\cyri"),
        ('\u{04E4}', r#"\"\CYRI"#),
        ('\u{04E5}', r#"\"\cyri"#),
        ('\u{04E6}', r#"\"\CYRO"#),
        ('\u{04E7}', r#"\"\cyro"#),
        ('\u{04E8}', r"\CYROTLD"),
        ('\u{04E9}', r"\cyrotld"),
        ('\u{04EC}', r#"\"\CYREREV"#),
        ('\u{04ED}', r#"\"\cyrerev"#),
        ('\u{04EE}', r"\=\CYRU"),
        ('\u{04EF}', r"\=\cyru"),
        ('\u{04F0}', r#"\"\CYRU"#),
        ('\u{04F1}', r#"\"\cyru"#),
        ('\u{04F2}', r"\H\CYRU"),
        ('\u{04F3}', r"\H\cyru"),
        ('\u{04F4}', r#"\"\CYRCH"#),
        ('\u{04F5}', r#"\"\cyrch"#),
        ('\u{04F6}', r"\CYRGDSC"),
        ('\u{04F7}', r"\cyrgdsc"),
        ('\u{04F8}', r#"\"\CYRERY"#),
        ('\u{04F9}', r#"\"\cyrery"#),
        ('\u{04FA}', r"\CYRGDSCHCRS"),
        ('\u{04FB}', r"\cyrgdschcrs"),
        ('\u{04FC}', r"\CYRHHK"),
        ('\u{04FD}', r"\cyrhhk"),
        ('\u{04FE}', r"\CYRHHCRS"),
        ('\u{04FF}', r"\cyrhhcrs"),
        ('\u{0E3F}', r"\textbaht"),
        ('\u{2000}', r"\enskip"),                              // EN QUAD (= EN SPACE U+2002)
        ('\u{2001}', r"\quad"),                                // EM QUAD (= EM SPACE U+2003)
        ('\u{2002}', r"\enskip"),                              // EN SPACE
        ('\u{2003}', r"\quad"),                                // EM SPACE
        ('\u{2004}', r"\hspace{0.33em}"),                      // THREE-PER-EM SPACE
        ('\u{2005}', r"\hspace{0.25em}"),                      // FOUR-PER-EM SPACE
        ('\u{2006}', r"\hspace{0.167em}"),                     // SIX-PER-EM SPACE
        ('\u{2007}', r"~"),                                    // FIGURE SPACE
        ('\u{2008}', r"\;"),                                   // PUNCTUATION SPACE
        ('\u{2009}', r"\,"),                                   // thin space
        ('\u{200A}', r"\hspace{1pt}"),                         // supposed to be thinnest typographical space available
        ('\u{200C}', r"\textcompwordmark"),                    // ZERO WIDTH NON-JOINER
        ('\u{2010}', r"-"),                                    // HYPHEN
        ('\u{2011}', r"\nobreakdash-"),                        // NON-BREAKING HYPHEN, https://tex.stackexchange.com/a/330437/32188
        ('\u{2012}', r"-"),                                    // FIGURE DASH
        ('\u{2013}', r"\textendash"),                          // 0x2013
        ('\u{2014}', r"\textemdash"),
        ('\u{2015}', r"\textemdash"),                          // HORIZONTAL BAR
        ('\u{2016}', r"\ensuremath{\Vert}"),
        ('\u{2018}', r"\textquoteleft"),
        ('\u{2019}', r"\textquoteright"),
        ('\u{201A}', r"\quotesinglbase"),                      // 0x201A
        ('\u{201C}', r"\textquotedblleft"),
        ('\u{201D}', r"\textquotedblright"),
        ('\u{201E}', r"\quotedblbase"),
        ('\u{2020}', r"\textdagger"),
        ('\u{2021}', r"\textdaggerdbl"),
        ('\u{2022}', r"\textbullet"),
        ('\u{2024}', r"."),                                     // ONE DOT LEADER [․]
        ('\u{2025}', r".."),                                    // TWO DOT LEADER [‥]
        ('\u{2026}', r"\textellipsis"),
        ('\u{2030}', r"\textperthousand"),
        ('\u{2031}', r"\textpertenthousand"),
        ('′', "'"),
        ('″', "''"),
        ('‴', "'''"),
        ('‵', r"\ensuremath{\backprime}"),
        ('\u{2039}', r"\guilsinglleft"),
        ('\u{203A}', r"\guilsinglright"),
        ('\u{203B}', r"\textreferencemark"),
        ('\u{203D}', r"\textinterrobang"),
        ('\u{2044}', r"\textfractionsolidus"),
        ('\u{204E}', r"\textasteriskcentered"),
        ('\u{2052}', r"\textdiscount"),
        ('⁗', "''''"),
        ('\u{205F}', r"\hspace{0.22em}"),                             // MEDIUM MATHEMATICAL SPACE [ ]
        ('\u{2060}', r"\nolinebreak"),                          // WORD JOINER [⁠]
        ('\u{2061}', r""),                                     // FUNCTION APPLICATION
        ('\u{20A1}', r"\textcolonmonetary"),                   // 0x20A1
        ('\u{20A4}', r"\textlira"),
        ('\u{20A6}', r"\textnaira"),
        ('\u{20A9}', r"\textwon"),
        ('\u{20AB}', r"\textdong"),
        ('\u{20AC}', r"\texteuro"),
        ('\u{20B1}', r"\textpeso"),
        ('\u{2102}', r"\ensuremath{\mathbb{C}}"),              // DOUBLE-STRUCK CAPITAL C
        ('\u{2103}', r"\textcelsius"),                         // DEGREE CELSIUS
        ('\u{2109}', r"\ensuremath{^\circ}F"),                 // DEGREE FARENHEIT
        ('\u{210A}', r"\ensuremath{g}"),                       // SCRIPT SMALL G
        ('\u{210B}', r"\ensuremath{\mathscr{H}}"),             // SCRIPT CAPITAL H
        ('\u{210C}', r"\ensuremath{\mathfrak{H}}"),            // BLACK-LETTER CAPITAL H
        ('\u{210D}', r"\ensuremath{\mathbb{H}}"),              // DOUBLE-STRUCK CAPITAL H
        ('\u{210E}', r"\ensuremath{h}"),                       // PLANCK CONSTANT
        ('\u{210F}', r"\ensuremath{\hbar}"),                   // h bar, PLANCK CONSTANT OVER TWO PI
        ('\u{2110}', r"\ensuremath{\mathscr{I}}"),             // SCRIPT CAPITAL I
        ('\u{2111}', r"\ensuremath{\mathfrak{I}}"),            // BLACK-LETTER CAPITAL I
        ('\u{2112}', r"\ensuremath{\mathscr{L}}"),             // SCRIPT CAPITAL L
        ('\u{2113}', r"\ensuremath{\ell}"),                    // SCRIPT SMALL L
        ('\u{2115}', r"\ensuremath{\mathbb{N}}"),              // DOUBLE-STRUCK CAPITAL N
        ('\u{2116}', r"\textnumero"),                          // NUMERO SIGN
        ('\u{2117}', r"\textcircledP"),                        // SOUND RECORDING COPYRIGHT
        ('\u{2118}', r"\ensuremath{\wp}"),                     // SCRIPT CAPITAL P [℘]
        ('\u{211E}', r"\textrecipe"),                          // PRESCRIPTION TAKE
        ('\u{2119}', r"\ensuremath{\mathbb{P}}"),              // DOUBLE-STRUCK CAPITAL P
        ('\u{211A}', r"\ensuremath{\mathbb{Q}}"),              // DOUBLE-STRUCK CAPITAL Q
        ('\u{211B}', r"\ensuremath{\mathscr{R}}"),             // SCRIPT CAPITAL R
        ('\u{211C}', r"\ensuremath{\mathfrak{R}}"),            // BLACK-LETTER CAPITAL R
        ('\u{211D}', r"\ensuremath{\mathbb{R}}"),              // DOUBLE-STRUCK CAPITAL R
        ('\u{2120}', r"\textservicemark"),                     // SERVICE MARK
        ('\u{2122}', r"\texttrademark"),                       // TRADE MARK SIGN
        ('\u{2124}', r"\ensuremath{\mathbb{Z}}"),              // DOUBLE-STRUCK CAPITAL Z
        ('\u{2126}', r"\textohm"),                             // OHM SIGN
        ('\u{2127}', r"\textmho"),                             // OHM SIGN
        ('\u{2128}', r"\ensuremath{\mathfrak{Z}}"),            // BLACK-LETTER CAPITAL Z
        ('\u{212A}', r"K"),                                    // KELVIN SIGN
        ('\u{212B}', r"\r{A}"),                                // ANGSTROM SIGN
        ('\u{212C}', r"\ensuremath{\mathscr{B}}"),             // SCRIPT CAPITAL B
        ('\u{212D}', r"\ensuremath{\mathfrak{C}}"),            // BLACK-LETTER CAPITAL C
        ('\u{212E}', r"\textestimated"),                       // ESTIMATED SYMBOL
        ('\u{212F}', r"\ensuremath{e}"),                       // SCRIPT SMALL E
        ('\u{2130}', r"\ensuremath{\mathscr{E}}"),             // SCRIPT CAPITAL E
        ('\u{2131}', r"\ensuremath{\mathscr{F}}"),             // SCRIPT CAPITAL F
        ('\u{2133}', r"\ensuremath{\mathscr{M}}"),             // SCRIPT CAPITAL M
        ('\u{2134}', r"\ensuremath{o}"),                       // SCRIPT SMALL O
        ('\u{2135}', r"\ensuremath{\aleph}"),                  // ALEF SYMBOL
        ('\u{2136}', r"\ensuremath{\beth}"),                    // BET SYMBOL [ℶ]
        ('\u{2137}', r"\ensuremath{\gimel}"),                   // GIMEL SYMBOL [ℷ]
        ('\u{2138}', r"\ensuremath{\daleth}"),                  // DALET SYMBOL [ℸ]
        //
        ('\u{2153}', r"\textfrac{1}{3}"),                       // VULGAR FRACTION ONE THIRD [⅓]
        ('\u{2154}', r"\textfrac{2}{3}"),                       // VULGAR FRACTION TWO THIRDS [⅔]
        ('\u{2155}', r"\textfrac{1}{5}"),                       // VULGAR FRACTION ONE FIFTH [⅕]
        ('\u{2156}', r"\textfrac{2}{5}"),                       // VULGAR FRACTION TWO FIFTHS [⅖]
        ('\u{2157}', r"\textfrac{3}{5}"),                       // VULGAR FRACTION THREE FIFTHS [⅗]
        ('\u{2158}', r"\textfrac{4}{5}"),                       // VULGAR FRACTION FOUR FIFTHS [⅘]
        ('\u{2159}', r"\textfrac{1}{6}"),                       // VULGAR FRACTION ONE SIXTH [⅙]
        ('\u{215A}', r"\textfrac{5}{6}"),                       // VULGAR FRACTION FIVE SIXTHS [⅚]
        ('\u{215B}', r"\textfrac{1}{8}"),                       // VULGAR FRACTION ONE EIGHTH [⅛]
        ('\u{215C}', r"\textfrac{3}{8}"),                       // VULGAR FRACTION THREE EIGHTHS [⅜]
        ('\u{215D}', r"\textfrac{5}{8}"),                       // VULGAR FRACTION FIVE EIGHTHS [⅝]
        ('\u{215E}', r"\textfrac{7}{8}"),                       // VULGAR FRACTION SEVEN EIGHTHS [⅞]
        //
        ('\u{2190}', r"\textleftarrow"),                       // 0x2190
        ('\u{2191}', r"\textuparrow"),
        ('\u{2192}', r"\textrightarrow"),
        ('\u{2193}', r"\textdownarrow"),                       // 0x2193
        ('\u{2194}', r"\ensuremath{\leftrightarrow}"),          // LEFT RIGHT ARROW [↔]
        ('\u{2195}', r"\ensuremath{\updownarrow}"),                         // UP DOWN ARROW [↕]
        ('\u{2196}', r"\ensuremath{\nwarrow}"),                             // NORTH WEST ARROW [↖]
        ('\u{2197}', r"\ensuremath{\nearrow}"),                             // NORTH EAST ARROW [↗]
        ('\u{2198}', r"\ensuremath{\searrow}"),                             // SOUTH EAST ARROW [↘]
        ('\u{2199}', r"\ensuremath{\swarrow}"),                             // SOUTH WEST ARROW [↙]
        ('\u{219A}', r"\ensuremath{\nleftarrow}"),                          // LEFTWARDS ARROW WITH STROKE [↚]
        ('\u{219B}', r"\ensuremath{\nrightarrow}"),                         // RIGHTWARDS ARROW WITH STROKE [↛]
        ('\u{219C}', r"\ensuremath{\arrowwaveleft}"),                       // LEFTWARDS WAVE ARROW [↜]
        ('\u{219D}', r"\ensuremath{\arrowwaveright}"),                      // RIGHTWARDS WAVE ARROW [↝]
        ('\u{219E}', r"\ensuremath{\twoheadleftarrow}"),                    // LEFTWARDS TWO HEADED ARROW [↞]
        ('\u{21A0}', r"\ensuremath{\twoheadrightarrow}"),                   // RIGHTWARDS TWO HEADED ARROW [↠]
        ('\u{21A2}', r"\ensuremath{\leftarrowtail}"),                       // LEFTWARDS ARROW WITH TAIL [↢]
        ('\u{21A3}', r"\ensuremath{\rightarrowtail}"),                      // RIGHTWARDS ARROW WITH TAIL [↣]
        ('\u{21A6}', r"\ensuremath{\mapsto}"),                              // RIGHTWARDS ARROW FROM BAR [↦]
        ('\u{21A9}', r"\ensuremath{\hookleftarrow}"),                       // LEFTWARDS ARROW WITH HOOK [↩]
        ('\u{21AA}', r"\ensuremath{\hookrightarrow}"),                      // RIGHTWARDS ARROW WITH HOOK [↪]
        ('\u{21AB}', r"\ensuremath{\looparrowleft}"),                       // LEFTWARDS ARROW WITH LOOP [↫]
        ('\u{21AC}', r"\ensuremath{\looparrowright}"),                      // RIGHTWARDS ARROW WITH LOOP [↬]
        ('\u{21AD}', r"\ensuremath{\leftrightsquigarrow}"),                 // LEFT RIGHT WAVE ARROW [↭]
        ('\u{21AE}', r"\ensuremath{\nleftrightarrow}"),                     // LEFT RIGHT ARROW WITH STROKE [↮]
        ('\u{21B0}', r"\ensuremath{\Lsh}"),                                 // UPWARDS ARROW WITH TIP LEFTWARDS [↰]
        ('\u{21B1}', r"\ensuremath{\Rsh}"),                                 // UPWARDS ARROW WITH TIP RIGHTWARDS [↱]
        ('\u{21B6}', r"\ensuremath{\curvearrowleft}"),                      // ANTICLOCKWISE TOP SEMICIRCLE ARROW [↶]
        ('\u{21B7}', r"\ensuremath{\curvearrowright}"),                     // CLOCKWISE TOP SEMICIRCLE ARROW [↷]
        ('\u{21BA}', r"\ensuremath{\circlearrowleft}"),                     // ANTICLOCKWISE OPEN CIRCLE ARROW [↺]
        ('\u{21BB}', r"\ensuremath{\circlearrowright}"),                    // CLOCKWISE OPEN CIRCLE ARROW [↻]
        ('\u{21BC}', r"\ensuremath{\leftharpoonup}"),                       // LEFTWARDS HARPOON WITH BARB UPWARDS [↼]
        ('\u{21BD}', r"\ensuremath{\leftharpoondown}"),                     // LEFTWARDS HARPOON WITH BARB DOWNWARDS [↽]
        ('\u{21BE}', r"\ensuremath{\upharpoonright}"),                      // UPWARDS HARPOON WITH BARB RIGHTWARDS [↾]
        ('\u{21BF}', r"\ensuremath{\upharpoonleft}"),                       // UPWARDS HARPOON WITH BARB LEFTWARDS [↿]
        ('\u{21C0}', r"\ensuremath{\rightharpoonup}"),                      // RIGHTWARDS HARPOON WITH BARB UPWARDS [⇀]
        ('\u{21C1}', r"\ensuremath{\rightharpoondown}"),                    // RIGHTWARDS HARPOON WITH BARB DOWNWARDS [⇁]
        ('\u{21C2}', r"\ensuremath{\downharpoonright}"),                    // DOWNWARDS HARPOON WITH BARB RIGHTWARDS [⇂]
        ('\u{21C3}', r"\ensuremath{\downharpoonleft}"),                     // DOWNWARDS HARPOON WITH BARB LEFTWARDS [⇃]
        ('\u{21C4}', r"\ensuremath{\rightleftarrows}"),                     // RIGHTWARDS ARROW OVER LEFTWARDS ARROW [⇄]
        ('\u{21C5}', r"\ensuremath{\dblarrowupdown}"),                      // UPWARDS ARROW LEFTWARDS OF DOWNWARDS ARROW [⇅]
        ('\u{21C6}', r"\ensuremath{\leftrightarrows}"),                     // LEFTWARDS ARROW OVER RIGHTWARDS ARROW [⇆]
        ('\u{21C7}', r"\ensuremath{\leftleftarrows}"),                      // LEFTWARDS PAIRED ARROWS [⇇]
        ('\u{21C8}', r"\ensuremath{\upuparrows}"),                          // UPWARDS PAIRED ARROWS [⇈]
        ('\u{21C9}', r"\ensuremath{\rightrightarrows}"),                    // RIGHTWARDS PAIRED ARROWS [⇉]
        ('\u{21CA}', r"\ensuremath{\downdownarrows}"),                      // DOWNWARDS PAIRED ARROWS [⇊]
        ('\u{21CB}', r"\ensuremath{\leftrightharpoons}"),                   // LEFTWARDS HARPOON OVER RIGHTWARDS HARPOON [⇋]
        ('\u{21CC}', r"\ensuremath{\rightleftharpoons}"),                   // RIGHTWARDS HARPOON OVER LEFTWARDS HARPOON [⇌]
        ('\u{21CD}', r"\ensuremath{\nLeftarrow}"),                          // LEFTWARDS DOUBLE ARROW WITH STROKE [⇍]
        ('\u{21CE}', r"\ensuremath{\nLeftrightarrow}"),                     // LEFT RIGHT DOUBLE ARROW WITH STROKE [⇎]
        ('\u{21CF}', r"\ensuremath{\nRightarrow}"),                         // RIGHTWARDS DOUBLE ARROW WITH STROKE [⇏]
        ('\u{21D0}', r"\ensuremath{\Leftarrow}"),                           // LEFTWARDS DOUBLE ARROW [⇐]
        ('\u{21D1}', r"\ensuremath{\Uparrow}"),                             // UPWARDS DOUBLE ARROW [⇑]
        ('\u{21D2}', r"\ensuremath{\Rightarrow}"),                          // RIGHTWARDS DOUBLE ARROW [⇒]
        ('\u{21D3}', r"\ensuremath{\Downarrow}"),                           // DOWNWARDS DOUBLE ARROW [⇓]
        ('\u{21D4}', r"\ensuremath{\Leftrightarrow}"),                      // LEFT RIGHT DOUBLE ARROW [⇔]
        ('\u{21D5}', r"\ensuremath{\Updownarrow}"),                         // UP DOWN DOUBLE ARROW [⇕]
        ('\u{21DA}', r"\ensuremath{\Lleftarrow}"),                          // LEFTWARDS TRIPLE ARROW [⇚]
        ('\u{21DB}', r"\ensuremath{\Rrightarrow}"),                         // RIGHTWARDS TRIPLE ARROW [⇛]
        ('\u{21DD}', r"\ensuremath{\rightsquigarrow}"),                     // RIGHTWARDS SQUIGGLE ARROW [⇝]
        ('\u{21F5}', r"\ensuremath{\DownArrowUpArrow}"),                    // DOWNWARDS ARROW LEFTWARDS OF UPWARDS ARROW [⇵]
        //
        //
        // // Math operators and symbols (U+22XX)
        ('\u{2200}', r"\ensuremath{\forall}"),
        ('\u{2201}', r"\ensuremath{\complement}"),
        ('\u{2202}', r"\ensuremath{\partial}"),
        ('\u{2203}', r"\ensuremath{\exists}"),
        ('\u{2204}', r"\ensuremath{\nexists}"),
        ('\u{2205}', r"\ensuremath{\varnothing}"),
        ('\u{2206}', r"\ensuremath{\Delta}"),
        ('\u{2207}', r"\ensuremath{\nabla}"),
        ('\u{2208}', r"\ensuremath{\in}"),
        ('\u{2209}', r"\ensuremath{\notin}"),
        ('\u{220A}', r"\ensuremath{\in}"),                     // alternative
        ('\u{220B}', r"\ensuremath{\ni}"),
        ('\u{220C}', r"\ensuremath{\not\ni}"),
        ('\u{220D}', r"\ensuremath{\ni}"),                     // alternative
        ('\u{220E}', r"\ensuremath{\blacksquare}"),
        ('\u{220F}', r"\ensuremath{\prod}"),
        ('\u{2210}', r"\ensuremath{\coprod}"),
        ('\u{2211}', r"\ensuremath{\sum}"),
        ('\u{2212}', r"\ensuremath{-}"),
        ('\u{2213}', r"\ensuremath{\mp}"),
        ('\u{2214}', r"\ensuremath{\dotplus}"),                             // DOT PLUS [∔]
        ('\u{2215}', r"\ensuremath{/}"),
        ('\u{2216}', r"\ensuremath{\smallsetminus}"),
        ('\u{2217}', r"\ensuremath{*}"),
        ('\u{2218}', r"\ensuremath{\circ}"),
        ('\u{2219}', r"\ensuremath{\bullet}"),
        ('\u{221A}', r"\ensuremath{\sqrt{}}"),
        ('\u{221B}', r"\ensuremath{\sqrt[3]{}}"),
        ('\u{221C}', r"\ensuremath{\sqrt[4]{}}"),
        ('\u{221D}', r"\ensuremath{\propto}"),
        ('\u{221E}', r"\ensuremath{\infty}"),
        ('\u{221F}', r"\ensuremath{\rightangle}"),                          // RIGHT ANGLE [∟]
        ('\u{2220}', r"\ensuremath{\angle}"),                               // ANGLE [∠]
        ('\u{2221}', r"\ensuremath{\measuredangle}"),                       // MEASURED ANGLE [∡]
        ('\u{2222}', r"\ensuremath{\sphericalangle}"),                      // SPHERICAL ANGLE [∢]
        ('\u{2223}', r"\ensuremath{\mid}"),
        ('\u{2224}', r"\ensuremath{\nmid}"),
        ('\u{2225}', r"\ensuremath{\parallel}"),
        ('\u{2226}', r"\ensuremath{\nparallel}"),
        ('\u{2227}', r"\ensuremath{\wedge}"),
        ('\u{2228}', r"\ensuremath{\vee}"),
        ('\u{2229}', r"\ensuremath{\cap}"),
        ('\u{222A}', r"\ensuremath{\cup}"),
        ('\u{222B}', r"\ensuremath{\int}"),
        ('\u{222C}', r"\ensuremath{\iint}"),
        ('\u{222D}', r"\ensuremath{\iiint}"),
        ('\u{222E}', r"\ensuremath{\oint}"),
        ('\u{222F}', r"\ensuremath{\surfintegral}"),                        // SURFACE INTEGRAL [∯]
        ('\u{2230}', r"\ensuremath{\volintegral}"),                         // VOLUME INTEGRAL [∰]
        ('\u{2231}', r"\ensuremath{\clwintegral}"),                         // CLOCKWISE INTEGRAL [∱]
        // #0x2232: CLOCKWISE CONTOUR INTEGRAL
        // #0x2233: ANTICLOCKWISE CONTOUR INTEGRAL
        ('\u{2234}', r"\ensuremath{\therefore}"),
        ('\u{2235}', r"\ensuremath{\because}"),
        ('\u{2236}', r"\ensuremath{:}"),
        ('\u{2237}', r"\ensuremath{::}"),
        // #0x2238: DOT MINUS
        // #...
        ('\u{223A}', r"\ensuremath{\mathbin{{:}\!\!{-}\!\!{:}}}"), // GEOMETRIC PROPORTION [∺]
        ('\u{223B}', r"\ensuremath{\homothetic}"),              // HOMOTHETIC [∻]
        ('\u{223C}', r"\ensuremath{\sim}"),
        ('\u{223D}', r"\ensuremath{\backsim}"),
        ('\u{223E}', r"\ensuremath{\lazysinv}"),                            // INVERTED LAZY S [∾]
        // #
        ('\u{2240}', r"\ensuremath{\wr}"),                                  // WREATH PRODUCT [≀]
        ('\u{2241}', r"\ensuremath{\not\sim}"),                             // NOT TILDE [≁]
        ('\u{2243}', r"\ensuremath{\simeq}"),                               // ASYMPTOTICALLY EQUAL TO [≃]
        ('\u{2244}', r"\ensuremath{\not\simeq}"),                           // NOT ASYMPTOTICALLY EQUAL TO [≄]
        ('\u{2245}', r"\ensuremath{\cong}"),                                // APPROXIMATELY EQUAL TO [≅]
        ('\u{2246}', r"\ensuremath{\approxnotequal}"),          // APPROXIMATELY BUT NOT ACTUALLY EQUAL TO [≆]
        ('\u{2247}', r"\ensuremath{\not\cong}"),                // NEITHER APPROXIMATELY NOR ACTUALLY EQUAL TO [≇]
        ('\u{2248}', r"\ensuremath{\approx}"),
        ('\u{2249}', r"\ensuremath{\not\approx}"),                          // NOT ALMOST EQUAL TO [≉]
        ('\u{224A}', r"\ensuremath{\approxeq}"),                            // ALMOST EQUAL OR EQUAL TO [≊]
        ('\u{224B}', r"\ensuremath{\tildetrpl}"),                           // TRIPLE TILDE [≋]
        ('\u{224C}', r"\ensuremath{\allequal}"),                            // ALL EQUAL TO [≌]
        ('\u{224D}', r"\ensuremath{\asymp}"),                               // EQUIVALENT TO [≍]
        ('\u{224E}', r"\ensuremath{\Bumpeq}"),                              // GEOMETRICALLY EQUIVALENT TO [≎]
        ('\u{224F}', r"\ensuremath{\bumpeq}"),                              // DIFFERENCE BETWEEN [≏]
        ('\u{2250}', r"\ensuremath{\doteq}"),                               // APPROACHES THE LIMIT [≐]
        ('\u{2251}', r"\ensuremath{\doteqdot}"),                            // GEOMETRICALLY EQUAL TO [≑]
        ('\u{2252}', r"\ensuremath{\fallingdotseq}"),                       // APPROXIMATELY EQUAL TO OR THE IMAGE OF [≒]
        ('\u{2253}', r"\ensuremath{\risingdotseq}"),                        // IMAGE OF OR APPROXIMATELY EQUAL TO [≓]
        ('\u{2254}', r"\ensuremath{:=}"),                                    // COLON EQUALS [≔]
        ('\u{2255}', r"\ensuremath{=:}"),                                    // EQUALS COLON [≕]
        ('\u{2256}', r"\ensuremath{\eqcirc}"),                              // RING IN EQUAL TO [≖]
        ('\u{2257}', r"\ensuremath{\circeq}"),                              // RING EQUAL TO [≗]
        ('\u{2259}', r"\ensuremath{\estimates}"),                           // ESTIMATES [≙]
        ('\u{225B}', r"\ensuremath{\starequal}"),                           // STAR EQUALS [≛]
        ('\u{225C}', r"\ensuremath{\triangleq}"),                           // DELTA EQUAL TO [≜]
        // #
        ('\u{2260}', r"\ensuremath{\neq}"),
        ('\u{2261}', r"\ensuremath{\equiv}"),
        ('\u{2262}', r"\ensuremath{\not\equiv}"),
        // #0x2263: STRICTLY EQUIVALENT TO
        ('\u{2264}', r"\ensuremath{\leq}"),
        ('\u{2265}', r"\ensuremath{\geq}"),
        ('\u{2266}', r"\ensuremath{\leqq}"),
        ('\u{2267}', r"\ensuremath{\geqq}"),
        ('\u{2268}', r"\ensuremath{\lneqq}"),
        ('\u{2269}', r"\ensuremath{\gneqq}"),
        ('\u{226A}', r"\ensuremath{\ll}"),
        ('\u{226B}', r"\ensuremath{\gg}"),
        ('\u{226C}', r"\ensuremath{\between}"),                             // BETWEEN [≬]
        ('\u{226D}', r"\ensuremath{\not\kern-0.3em\times}"),                // NOT EQUIVALENT TO [≭]
        ('\u{226E}', r"\ensuremath{\nless}"),
        ('\u{226F}', r"\ensuremath{\ngtr}"),
        ('\u{2270}', r"\ensuremath{\nleq}"),
        ('\u{2271}', r"\ensuremath{\ngeq}"),
        ('\u{2272}', r"\ensuremath{\lesssim}"),
        ('\u{2273}', r"\ensuremath{\gtrsim}"),
        ('\u{2274}', r"\ensuremath{\not\lesssim}"),
        ('\u{2275}', r"\ensuremath{\not\gtrsim}"),
        ('\u{2276}', r"\ensuremath{\lessgtr}"),
        ('\u{2277}', r"\ensuremath{\gtrless}"),
        ('\u{2278}', r"\ensuremath{\notlessgreater}"),                      // NEITHER LESS-THAN NOR GREATER-THAN [≸]
        ('\u{2279}', r"\ensuremath{\notgreaterless}"),                      // NEITHER GREATER-THAN NOR LESS-THAN [≹]
        ('\u{227A}', r"\ensuremath{\prec}"),
        ('\u{227B}', r"\ensuremath{\succ}"),
        ('\u{227C}', r"\ensuremath{\preceq}"),
        ('\u{227D}', r"\ensuremath{\succeq}"),
        ('\u{227E}', r"\ensuremath{\precsim}"),
        ('\u{227F}', r"\ensuremath{\succsim}"),
        ('\u{2280}', r"\ensuremath{\nprec}"),
        ('\u{2281}', r"\ensuremath{\nsucc}"),
        ('\u{2282}', r"\ensuremath{\subset}"),
        ('\u{2283}', r"\ensuremath{\supset}"),
        ('\u{2284}', r"\ensuremath{\not\subset}"),
        ('\u{2285}', r"\ensuremath{\not\supset}"),
        ('\u{2286}', r"\ensuremath{\subseteq}"),
        ('\u{2287}', r"\ensuremath{\supseteq}"),
        ('\u{2288}', r"\ensuremath{\nsubseteq}"),
        ('\u{2289}', r"\ensuremath{\nsupseteq}"),
        ('\u{228A}', r"\ensuremath{\subsetneq}"),
        ('\u{228B}', r"\ensuremath{\supsetneq}"),
        ('\u{228E}', r"\ensuremath{\uplus}"),                               // MULTISET UNION [⊎]
        ('\u{228F}', r"\ensuremath{\sqsubset}"),                            // SQUARE IMAGE OF [⊏]
        ('\u{2290}', r"\ensuremath{\sqsupset}"),                            // SQUARE ORIGINAL OF [⊐]
        ('\u{2291}', r"\ensuremath{\sqsubseteq}"),                          // SQUARE IMAGE OF OR EQUAL TO [⊑]
        ('\u{2292}', r"\ensuremath{\sqsupseteq}"),                          // SQUARE ORIGINAL OF OR EQUAL TO [⊒]
        ('\u{2293}', r"\ensuremath{\sqcap}"),
        ('\u{2294}', r"\ensuremath{\sqcup}"),
        ('\u{2295}', r"\ensuremath{\oplus}"),
        ('\u{2296}', r"\ensuremath{\ominus}"),
        ('\u{2297}', r"\ensuremath{\otimes}"),
        ('\u{2298}', r"\ensuremath{\oslash}"),
        ('\u{2299}', r"\ensuremath{\odot}"),
        ('\u{229A}', r"\ensuremath{\circledcirc}"),                         // CIRCLED RING OPERATOR [⊚]
        ('\u{229B}', r"\ensuremath{\circledast}"),                          // CIRCLED ASTERISK OPERATOR [⊛]
        ('\u{229D}', r"\ensuremath{\circleddash}"),                         // CIRCLED DASH [⊝]
        ('\u{229E}', r"\ensuremath{\boxplus}"),                             // SQUARED PLUS [⊞]
        ('\u{229F}', r"\ensuremath{\boxminus}"),                            // SQUARED MINUS [⊟]
        ('\u{22A0}', r"\ensuremath{\boxtimes}"),                            // SQUARED TIMES [⊠]
        ('\u{22A1}', r"\ensuremath{\boxdot}"),                              // SQUARED DOT OPERATOR [⊡]
        ('\u{22A2}', r"\ensuremath{\vdash}"),                               // RIGHT TACK [⊢]
        ('\u{22A3}', r"\ensuremath{\dashv}"),                               // LEFT TACK [⊣]
        ('\u{22A4}', r"\ensuremath{\top}"),                                 // DOWN TACK [⊤]
        ('\u{22A5}', r"\ensuremath{\perp}"),                                // UP TACK [⊥]
        ('\u{22A7}', r"\ensuremath{\truestate}"),                           // MODELS [⊧]
        ('\u{22A8}', r"\ensuremath{\forcesextra}"),                         // TRUE [⊨]
        ('\u{22A9}', r"\ensuremath{\Vdash}"),                               // FORCES [⊩]
        ('\u{22AA}', r"\ensuremath{\Vvdash}"),                              // TRIPLE VERTICAL BAR RIGHT TURNSTILE [⊪]
        ('\u{22AB}', r"\ensuremath{\VDash}"),                   // DOUBLE VERTICAL BAR DOUBLE RIGHT TURNSTILE [⊫]
        ('\u{22AC}', r"\ensuremath{\nvdash}"),                              // DOES NOT PROVE [⊬]
        ('\u{22AD}', r"\ensuremath{\nvDash}"),                              // NOT TRUE [⊭]
        ('\u{22AE}', r"\ensuremath{\nVdash}"),                              // DOES NOT FORCE [⊮]
        ('\u{22AF}', r"\ensuremath{\nVDash}"),                  // NEGATED DOUBLE VERTICAL BAR DOUBLE RIGHT TURNSTILE [⊯]
        ('\u{22B2}', r"\ensuremath{\vartriangleleft}"),                     // NORMAL SUBGROUP OF [⊲]
        ('\u{22B3}', r"\ensuremath{\vartriangleright}"),                    // CONTAINS AS NORMAL SUBGROUP [⊳]
        ('\u{22B4}', r"\ensuremath{\trianglelefteq}"),                      // NORMAL SUBGROUP OF OR EQUAL TO [⊴]
        ('\u{22B5}', r"\ensuremath{\trianglerighteq}"),                     // CONTAINS AS NORMAL SUBGROUP OR EQUAL TO [⊵]
        ('\u{22B6}', r"\ensuremath{\original}"),                            // ORIGINAL OF [⊶]
        ('\u{22B7}', r"\ensuremath{\image}"),                               // IMAGE OF [⊷]
        ('\u{22B8}', r"\ensuremath{\multimap}"),                            // MULTIMAP [⊸]
        ('\u{22B9}', r"\ensuremath{\hermitconjmatrix}"),                    // HERMITIAN CONJUGATE MATRIX [⊹]
        ('\u{22BA}', r"\ensuremath{\intercal}"),                            // INTERCALATE [⊺]
        ('\u{22BB}', r"\ensuremath{\veebar}"),                              // XOR [⊻]
        ('\u{22BE}', r"\ensuremath{\rightanglearc}"),                       // RIGHT ANGLE WITH ARC [⊾]
        ('\u{22C0}', r"\ensuremath{\bigwedge}"),
        ('\u{22C1}', r"\ensuremath{\bigvee}"),
        ('\u{22C2}', r"\ensuremath{\bigcap}"),
        ('\u{22C3}', r"\ensuremath{\bigcup}"),
        ('\u{22C4}', r"\ensuremath{\diamond}"),
        ('\u{22C5}', r"\ensuremath{\cdot}"),
        ('\u{22C6}', r"\ensuremath{\star}"),
        ('\u{22C7}', r"\ensuremath{\divideontimes}"),
        ('\u{22C8}', r"\ensuremath{\bowtie}"),
        ('\u{22C9}', r"\ensuremath{\ltimes}"),
        ('\u{22CA}', r"\ensuremath{\rtimes}"),
        ('\u{22CB}', r"\ensuremath{\leftthreetimes}"),
        ('\u{22CC}', r"\ensuremath{\rightthreetimes}"),
        ('\u{22CD}', r"\ensuremath{\backsimeq}"),                           // REVERSED TILDE EQUALS [⋍]
        ('\u{22CE}', r"\ensuremath{\curlyvee}"),                            // CURLY LOGICAL OR [⋎]
        ('\u{22CF}', r"\ensuremath{\curlywedge}"),                          // CURLY LOGICAL AND [⋏]
        ('\u{22D0}', r"\ensuremath{\Subset}"),                              // DOUBLE SUBSET [⋐]
        ('\u{22D1}', r"\ensuremath{\Supset}"),                              // DOUBLE SUPERSET [⋑]
        ('\u{22D2}', r"\ensuremath{\Cap}"),                                 // DOUBLE INTERSECTION [⋒]
        ('\u{22D3}', r"\ensuremath{\Cup}"),                                 // DOUBLE UNION [⋓]
        ('\u{22D4}', r"\ensuremath{\pitchfork}"),                           // PITCHFORK [⋔]
        ('\u{22D6}', r"\ensuremath{\lessdot}"),                             // LESS-THAN WITH DOT [⋖]
        ('\u{22D7}', r"\ensuremath{\gtrdot}"),                              // GREATER-THAN WITH DOT [⋗]
        ('\u{22D8}', r"\ensuremath{\verymuchless}"),                        // VERY MUCH LESS-THAN [⋘]
        ('\u{22D9}', r"\ensuremath{\verymuchgreater}"),                     // VERY MUCH GREATER-THAN [⋙]
        ('\u{22DA}', r"\ensuremath{\lesseqgtr}"),                           // LESS-THAN EQUAL TO OR GREATER-THAN [⋚]
        ('\u{22DB}', r"\ensuremath{\gtreqless}"),                           // GREATER-THAN EQUAL TO OR LESS-THAN [⋛]
        ('\u{22DE}', r"\ensuremath{\curlyeqprec}"),                         // EQUAL TO OR PRECEDES [⋞]
        ('\u{22DF}', r"\ensuremath{\curlyeqsucc}"),                         // EQUAL TO OR SUCCEEDS [⋟]
        ('\u{22E2}', r"\ensuremath{\not\sqsubseteq}"),                      // NOT SQUARE IMAGE OF OR EQUAL TO [⋢]
        ('\u{22E3}', r"\ensuremath{\not\sqsupseteq}"),                      // NOT SQUARE ORIGINAL OF OR EQUAL TO [⋣]
        ('\u{22E6}', r"\ensuremath{\lnsim}"),                               // LESS-THAN BUT NOT EQUIVALENT TO [⋦]
        ('\u{22E7}', r"\ensuremath{\gnsim}"),                               // GREATER-THAN BUT NOT EQUIVALENT TO [⋧]
        ('\u{22E8}', r"\ensuremath{\precedesnotsimilar}"),                  // PRECEDES BUT NOT EQUIVALENT TO [⋨]
        ('\u{22E9}', r"\ensuremath{\succnsim}"),                            // SUCCEEDS BUT NOT EQUIVALENT TO [⋩]
        ('\u{22EA}', r"\ensuremath{\ntriangleleft}"),                       // NOT NORMAL SUBGROUP OF [⋪]
        ('\u{22EB}', r"\ensuremath{\ntriangleright}"),                      // DOES NOT CONTAIN AS NORMAL SUBGROUP [⋫]
        ('\u{22EC}', r"\ensuremath{\ntrianglelefteq}"),                     // NOT NORMAL SUBGROUP OF OR EQUAL TO [⋬]
        ('\u{22ED}', r"\ensuremath{\ntrianglerighteq}"),        // DOES NOT CONTAIN AS NORMAL SUBGROUP OR EQUAL [⋭]
        ('\u{22EE}', r"\ensuremath{\vdots}"),
        ('\u{22EF}', r"\ensuremath{\cdots}"),
        ('\u{22F0}', r"\ensuremath{\udots}"),
        ('\u{22F1}', r"\ensuremath{\ddots}"),
        // // ...
        ('\u{2305}', r"\ensuremath{\barwedge}"),                            // PROJECTIVE [⌅]
        ('\u{2306}', r"\ensuremath{\varperspcorrespond}"),                  // PERSPECTIVE [⌆]
        ('\u{2308}', r"\ensuremath{\lceil}"),                               // LEFT CEILING [⌈]
        ('\u{2309}', r"\ensuremath{\rceil}"),                               // RIGHT CEILING [⌉]
        ('\u{230A}', r"\ensuremath{\lfloor}"),                              // LEFT FLOOR [⌊]
        ('\u{230B}', r"\ensuremath{\rfloor}"),                              // RIGHT FLOOR [⌋]
        ('\u{2315}', r"\ensuremath{\recorder}"),                            // TELEPHONE RECORDER [⌕]
        ('\u{2316}', r#"\ensuremath{\mathchar"2208}"#),                        // POSITION INDICATOR [⌖]
        ('\u{231C}', r"\ensuremath{\ulcorner}"),                            // TOP LEFT CORNER [⌜]
        ('\u{231D}', r"\ensuremath{\urcorner}"),                            // TOP RIGHT CORNER [⌝]
        ('\u{231E}', r"\ensuremath{\llcorner}"),                            // BOTTOM LEFT CORNER [⌞]
        ('\u{231F}', r"\ensuremath{\lrcorner}"),                            // BOTTOM RIGHT CORNER [⌟]
        ('\u{2322}', r"\ensuremath{\frown}"),                               // FROWN [⌢]
        ('\u{2323}', r"\ensuremath{\smile}"),                               // SMILE [⌣]
        //
        ('\u{23B0}', r"\ensuremath{\lmoustache}"),              // UPPER LEFT OR LOWER RIGHT CURLY BRACKET SECTION [⎰]
        ('\u{23B1}', r"\ensuremath{\rmoustache}"),              // UPPER RIGHT OR LOWER LEFT CURLY BRACKET SECTION [⎱]
        //
        ('\u{2329}', r"\textlangle"),                          // 0x2329
        ('\u{232A}', r"\textrangle"),
        ('\u{2422}', r"\textblank"),
        ('\u{2423}', r"\textvisiblespace"),
        ('\u{25A0}', r"\ensuremath{\blacksquare}"),             // BLACK SQUARE [■]
        ('\u{25A1}', r"\ensuremath{\square}"),                  // WHITE SQUARE [□]
        ('\u{25AA}', r"{\small\ensuremath{\blacksquare}}"),     // BLACK SMALL SQUARE [▪]
        ('\u{25AD}', r"\fbox{~~}"),                             // WHITE RECTANGLE [▭]
        ('\u{25B3}', r"\ensuremath{\bigtriangleup}"),                       // WHITE UP-POINTING TRIANGLE [△]
        ('\u{25B4}', r"\ensuremath{\blacktriangle}"),                       // BLACK UP-POINTING SMALL TRIANGLE [▴]
        ('\u{25B5}', r"\ensuremath{\vartriangle}"),                         // WHITE UP-POINTING SMALL TRIANGLE [▵]
        ('\u{25B8}', r"\ensuremath{\blacktriangleright}"),                  // BLACK RIGHT-POINTING SMALL TRIANGLE [▸]
        ('\u{25B9}', r"\ensuremath{\triangleright}"),                       // WHITE RIGHT-POINTING SMALL TRIANGLE [▹]
        ('\u{25BD}', r"\ensuremath{\bigtriangledown}"),                     // WHITE DOWN-POINTING TRIANGLE [▽]
        ('\u{25BE}', r"\ensuremath{\blacktriangledown}"),                   // BLACK DOWN-POINTING SMALL TRIANGLE [▾]
        ('\u{25BF}', r"\ensuremath{\triangledown}"),                        // WHITE DOWN-POINTING SMALL TRIANGLE [▿]
        ('\u{25C2}', r"\ensuremath{\blacktriangleleft}"),                   // BLACK LEFT-POINTING SMALL TRIANGLE [◂]
        ('\u{25C3}', r"\ensuremath{\triangleleft}"),                        // WHITE LEFT-POINTING SMALL TRIANGLE [◃]
        ('\u{25CA}', r"\ensuremath{\lozenge}"),                             // LOZENGE [◊]
        ('\u{25CB}', r"\ensuremath{\bigcirc}"),                             // WHITE CIRCLE [○]
        //
        ('\u{25E6}', r"\textopenbullet"),
        ('\u{25EF}', r"\textbigcircle"),
        ('\u{2662}', r"\ensuremath{\diamond}"),                             // WHITE DIAMOND SUIT [♢]
        ('\u{266A}', r"\textmusicalnote"),                     // 0x266A
        ('\u{2669}', r"\quarternote"),                          // QUARTER NOTE [♩]
        ('\u{266D}', r"\flat"),                                 // MUSIC FLAT SIGN [♭]
        ('\u{266E}', r"\natural"),                              // MUSIC NATURAL SIGN [♮]
        ('\u{266F}', r"\sharp"),                                // MUSIC SHARP SIGN [♯]
        //
        //
        ('\u{27E8}', r"\ensuremath{\langle}"),                 // MATHEMATICAL LEFT ANGLE BRACKET
        ('\u{27E9}', r"\ensuremath{\rangle}"),                 // MATHEMATICAL RIGHT ANGLE BRACKET
        //
        ('\u{27F5}', r"\ensuremath{\longleftarrow}"),                       // LONG LEFTWARDS ARROW [⟵]
        ('\u{27F6}', r"\ensuremath{\longrightarrow}"),                      // LONG RIGHTWARDS ARROW [⟶]
        ('\u{27F7}', r"\ensuremath{\longleftrightarrow}"),                  // LONG LEFT RIGHT ARROW [⟷]
        ('\u{27F8}', r"\ensuremath{\Longleftarrow}"),                       // LONG LEFTWARDS DOUBLE ARROW [⟸]
        ('\u{27F9}', r"\ensuremath{\Longrightarrow}"),                      // LONG RIGHTWARDS DOUBLE ARROW [⟹]
        ('\u{27FA}', r"\ensuremath{\Longleftrightarrow}"),                  // LONG LEFT RIGHT DOUBLE ARROW [⟺]
        ('\u{27FC}', r"\ensuremath{\longmapsto}"),                          // LONG RIGHTWARDS ARROW FROM BAR [⟼]
        ('\u{27FF}', r"\ensuremath{\sim\joinrel\leadsto}"),                 // LONG RIGHTWARDS SQUIGGLE ARROW [⟿]
        //
        ('\u{2993}', r"\ensuremath{<\kern-0.58em(}"),                        // LEFT ARC LESS-THAN BRACKET [⦓]
        ('\u{29EB}', r"\ensuremath{\blacklozenge}"),                        // BLACK LOZENGE [⧫]
        // // Supplemental Mathematical Operators U+2AXX
        ('\u{2A0F}', r"\ensuremath{\clockoint}"),                           // INTEGRAL AVERAGE WITH SLASH [⨏]
        ('\u{2A16}', r"\ensuremath{\sqrint}"),                              // QUATERNION INTEGRAL OPERATOR [⨖]
        ('\u{2A3F}', r"\ensuremath{\amalg}"),                               // AMALGAMATION OR COPRODUCT [⨿]
        ('\u{2A6E}', r"\ensuremath{\stackrel{*}{=}}"),                       // EQUALS WITH ASTERISK [⩮]
        ('\u{2A75}', r"=="),                               // TWO CONSECUTIVE EQUALS SIGNS [⩵]
        ('\u{2A7D}', r"\ensuremath{\leqslant}"),
        ('\u{2A7E}', r"\ensuremath{\geqslant}"),
        ('\u{2A85}', r"\ensuremath{\lessapprox}"),                          // LESS-THAN OR APPROXIMATE [⪅]
        ('\u{2A86}', r"\ensuremath{\gtrapprox}"),                           // GREATER-THAN OR APPROXIMATE [⪆]
        ('\u{2A87}', r"\ensuremath{\lneq}"),                                // LESS-THAN AND SINGLE-LINE NOT EQUAL TO [⪇]
        ('\u{2A88}', r"\ensuremath{\gneq}"),                                // GREATER-THAN AND SINGLE-LINE NOT EQUAL TO [⪈]
        ('\u{2A89}', r"\ensuremath{\lnapprox}"),                            // LESS-THAN AND NOT APPROXIMATE [⪉]
        ('\u{2A8A}', r"\ensuremath{\gnapprox}"),                            // GREATER-THAN AND NOT APPROXIMATE [⪊]
        ('\u{2A8B}', r"\ensuremath{\lesseqqgtr}"),              // LESS-THAN ABOVE DOUBLE-LINE EQUAL ABOVE GREATER-THAN [⪋]
        ('\u{2A8C}', r"\ensuremath{\gtreqqless}"),              // GREATER-THAN ABOVE DOUBLE-LINE EQUAL ABOVE LESS-THAN [⪌]
        ('\u{2A95}', r"\ensuremath{\eqslantless}"),                         // SLANTED EQUAL TO OR LESS-THAN [⪕]
        ('\u{2A96}', r"\ensuremath{\eqslantgtr}"),                          // SLANTED EQUAL TO OR GREATER-THAN [⪖]
        ('\u{2AAF}', r"\ensuremath{\preceq}"),                              // PRECEDES ABOVE SINGLE-LINE EQUALS SIGN [⪯]
        ('\u{2AB0}', r"\ensuremath{\succeq}"),                              // SUCCEEDS ABOVE SINGLE-LINE EQUALS SIGN [⪰]
        ('\u{2AB5}', r"\ensuremath{\precneqq}"),                            // PRECEDES ABOVE NOT EQUAL TO [⪵]
        ('\u{2AB6}', r"\ensuremath{\succneqq}"),                            // SUCCEEDS ABOVE NOT EQUAL TO [⪶]
        ('\u{2AB7}', r"\ensuremath{\precapprox}"),                          // PRECEDES ABOVE ALMOST EQUAL TO [⪷]
        ('\u{2AB8}', r"\ensuremath{\succapprox}"),                          // SUCCEEDS ABOVE ALMOST EQUAL TO [⪸]
        ('\u{2AB9}', r"\ensuremath{\precnapprox}"),                         // PRECEDES ABOVE NOT ALMOST EQUAL TO [⪹]
        ('\u{2ABA}', r"\ensuremath{\succnapprox}"),                         // SUCCEEDS ABOVE NOT ALMOST EQUAL TO [⪺]
        ('\u{2AC5}', r"\ensuremath{\subseteqq}"),                           // SUBSET OF ABOVE EQUALS SIGN [⫅]
        ('\u{2AC6}', r"\ensuremath{\supseteqq}"),                           // SUPERSET OF ABOVE EQUALS SIGN [⫆]
        ('\u{2ACB}', r"\ensuremath{\subsetneqq}"),                          // SUBSET OF ABOVE NOT EQUAL TO [⫋]
        ('\u{2ACC}', r"\ensuremath{\supsetneqq}"),                          // SUPERSET OF ABOVE NOT EQUAL TO [⫌]
        ('\u{2AFD}', r"\ensuremath{{{/}\!\!{/}}}"),                          // DOUBLE SOLIDUS OPERATOR [⫽]
        ('\u{3008}', r"\ensuremath{\langle}"),
        ('\u{3009}', r"\ensuremath{\rangle}"),
        ('\u{FB00}', r"ff"),                                   // LATIN SMALL LIGATURE FF
        ('\u{FB01}', r"fi"),                                   // LATIN SMALL LIGATURE FI
        ('\u{FB02}', r"fl"),                                   // LATIN SMALL LIGATURE FL
        ('\u{FB03}', r"ffi"),                                  // LATIN SMALL LIGATURE FFI
        ('\u{FB04}', r"ffl"),                                  // LATIN SMALL LIGATURE FFL
        ('\u{1D400}', r"\ensuremath{\mathbf{A}}"),              // MATHEMATICAL BOLD CAPITAL A
        ('\u{1D401}', r"\ensuremath{\mathbf{B}}"),              // MATHEMATICAL BOLD CAPITAL B
        ('\u{1D402}', r"\ensuremath{\mathbf{C}}"),              // MATHEMATICAL BOLD CAPITAL C
        ('\u{1D403}', r"\ensuremath{\mathbf{D}}"),              // MATHEMATICAL BOLD CAPITAL D
        ('\u{1D404}', r"\ensuremath{\mathbf{E}}"),              // MATHEMATICAL BOLD CAPITAL E
        ('\u{1D405}', r"\ensuremath{\mathbf{F}}"),              // MATHEMATICAL BOLD CAPITAL F
        ('\u{1D406}', r"\ensuremath{\mathbf{G}}"),              // MATHEMATICAL BOLD CAPITAL G
        ('\u{1D407}', r"\ensuremath{\mathbf{H}}"),              // MATHEMATICAL BOLD CAPITAL H
        ('\u{1D408}', r"\ensuremath{\mathbf{I}}"),              // MATHEMATICAL BOLD CAPITAL I
        ('\u{1D409}', r"\ensuremath{\mathbf{J}}"),              // MATHEMATICAL BOLD CAPITAL J
        ('\u{1D40A}', r"\ensuremath{\mathbf{K}}"),              // MATHEMATICAL BOLD CAPITAL K
        ('\u{1D40B}', r"\ensuremath{\mathbf{L}}"),              // MATHEMATICAL BOLD CAPITAL L
        ('\u{1D40C}', r"\ensuremath{\mathbf{M}}"),              // MATHEMATICAL BOLD CAPITAL M
        ('\u{1D40D}', r"\ensuremath{\mathbf{N}}"),              // MATHEMATICAL BOLD CAPITAL N
        ('\u{1D40E}', r"\ensuremath{\mathbf{O}}"),              // MATHEMATICAL BOLD CAPITAL O
        ('\u{1D40F}', r"\ensuremath{\mathbf{P}}"),              // MATHEMATICAL BOLD CAPITAL P
        ('\u{1D410}', r"\ensuremath{\mathbf{Q}}"),              // MATHEMATICAL BOLD CAPITAL Q
        ('\u{1D411}', r"\ensuremath{\mathbf{R}}"),              // MATHEMATICAL BOLD CAPITAL R
        ('\u{1D412}', r"\ensuremath{\mathbf{S}}"),              // MATHEMATICAL BOLD CAPITAL S
        ('\u{1D413}', r"\ensuremath{\mathbf{T}}"),              // MATHEMATICAL BOLD CAPITAL T
        ('\u{1D414}', r"\ensuremath{\mathbf{U}}"),              // MATHEMATICAL BOLD CAPITAL U
        ('\u{1D415}', r"\ensuremath{\mathbf{V}}"),              // MATHEMATICAL BOLD CAPITAL V
        ('\u{1D416}', r"\ensuremath{\mathbf{W}}"),              // MATHEMATICAL BOLD CAPITAL W
        ('\u{1D417}', r"\ensuremath{\mathbf{X}}"),              // MATHEMATICAL BOLD CAPITAL X
        ('\u{1D418}', r"\ensuremath{\mathbf{Y}}"),              // MATHEMATICAL BOLD CAPITAL Y
        ('\u{1D419}', r"\ensuremath{\mathbf{Z}}"),              // MATHEMATICAL BOLD CAPITAL Z
        ('\u{1D41A}', r"\ensuremath{\mathbf{a}}"),              // MATHEMATICAL BOLD SMALL a
        ('\u{1D41B}', r"\ensuremath{\mathbf{b}}"),              // MATHEMATICAL BOLD SMALL b
        ('\u{1D41C}', r"\ensuremath{\mathbf{c}}"),              // MATHEMATICAL BOLD SMALL c
        ('\u{1D41D}', r"\ensuremath{\mathbf{d}}"),              // MATHEMATICAL BOLD SMALL d
        ('\u{1D41E}', r"\ensuremath{\mathbf{e}}"),              // MATHEMATICAL BOLD SMALL e
        ('\u{1D41F}', r"\ensuremath{\mathbf{f}}"),              // MATHEMATICAL BOLD SMALL f
        ('\u{1D420}', r"\ensuremath{\mathbf{g}}"),              // MATHEMATICAL BOLD SMALL g
        ('\u{1D421}', r"\ensuremath{\mathbf{h}}"),              // MATHEMATICAL BOLD SMALL h
        ('\u{1D422}', r"\ensuremath{\mathbf{i}}"),              // MATHEMATICAL BOLD SMALL i
        ('\u{1D423}', r"\ensuremath{\mathbf{j}}"),              // MATHEMATICAL BOLD SMALL j
        ('\u{1D424}', r"\ensuremath{\mathbf{k}}"),              // MATHEMATICAL BOLD SMALL k
        ('\u{1D425}', r"\ensuremath{\mathbf{l}}"),              // MATHEMATICAL BOLD SMALL l
        ('\u{1D426}', r"\ensuremath{\mathbf{m}}"),              // MATHEMATICAL BOLD SMALL m
        ('\u{1D427}', r"\ensuremath{\mathbf{n}}"),              // MATHEMATICAL BOLD SMALL n
        ('\u{1D428}', r"\ensuremath{\mathbf{o}}"),              // MATHEMATICAL BOLD SMALL o
        ('\u{1D429}', r"\ensuremath{\mathbf{p}}"),              // MATHEMATICAL BOLD SMALL p
        ('\u{1D42A}', r"\ensuremath{\mathbf{q}}"),              // MATHEMATICAL BOLD SMALL q
        ('\u{1D42B}', r"\ensuremath{\mathbf{r}}"),              // MATHEMATICAL BOLD SMALL r
        ('\u{1D42C}', r"\ensuremath{\mathbf{s}}"),              // MATHEMATICAL BOLD SMALL s
        ('\u{1D42D}', r"\ensuremath{\mathbf{t}}"),              // MATHEMATICAL BOLD SMALL t
        ('\u{1D42E}', r"\ensuremath{\mathbf{u}}"),              // MATHEMATICAL BOLD SMALL u
        ('\u{1D42F}', r"\ensuremath{\mathbf{v}}"),              // MATHEMATICAL BOLD SMALL v
        ('\u{1D430}', r"\ensuremath{\mathbf{w}}"),              // MATHEMATICAL BOLD SMALL w
        ('\u{1D431}', r"\ensuremath{\mathbf{x}}"),              // MATHEMATICAL BOLD SMALL x
        ('\u{1D432}', r"\ensuremath{\mathbf{y}}"),              // MATHEMATICAL BOLD SMALL y
        ('\u{1D433}', r"\ensuremath{\mathbf{z}}"),              // MATHEMATICAL BOLD SMALL z
        //
        ('\u{1D434}', r"\ensuremath{\mathit{A}}"),              // MATHEMATICAL ITALIC CAPITAL A
        ('\u{1D435}', r"\ensuremath{\mathit{B}}"),              // MATHEMATICAL ITALIC CAPITAL B
        ('\u{1D436}', r"\ensuremath{\mathit{C}}"),              // MATHEMATICAL ITALIC CAPITAL C
        ('\u{1D437}', r"\ensuremath{\mathit{D}}"),              // MATHEMATICAL ITALIC CAPITAL D
        ('\u{1D438}', r"\ensuremath{\mathit{E}}"),              // MATHEMATICAL ITALIC CAPITAL E
        ('\u{1D439}', r"\ensuremath{\mathit{F}}"),              // MATHEMATICAL ITALIC CAPITAL F
        ('\u{1D43A}', r"\ensuremath{\mathit{G}}"),              // MATHEMATICAL ITALIC CAPITAL G
        ('\u{1D43B}', r"\ensuremath{\mathit{H}}"),              // MATHEMATICAL ITALIC CAPITAL H
        ('\u{1D43C}', r"\ensuremath{\mathit{I}}"),              // MATHEMATICAL ITALIC CAPITAL I
        ('\u{1D43D}', r"\ensuremath{\mathit{J}}"),              // MATHEMATICAL ITALIC CAPITAL J
        ('\u{1D43E}', r"\ensuremath{\mathit{K}}"),              // MATHEMATICAL ITALIC CAPITAL K
        ('\u{1D43F}', r"\ensuremath{\mathit{L}}"),              // MATHEMATICAL ITALIC CAPITAL L
        ('\u{1D440}', r"\ensuremath{\mathit{M}}"),              // MATHEMATICAL ITALIC CAPITAL M
        ('\u{1D441}', r"\ensuremath{\mathit{N}}"),              // MATHEMATICAL ITALIC CAPITAL N
        ('\u{1D442}', r"\ensuremath{\mathit{O}}"),              // MATHEMATICAL ITALIC CAPITAL O
        ('\u{1D443}', r"\ensuremath{\mathit{P}}"),              // MATHEMATICAL ITALIC CAPITAL P
        ('\u{1D444}', r"\ensuremath{\mathit{Q}}"),              // MATHEMATICAL ITALIC CAPITAL Q
        ('\u{1D445}', r"\ensuremath{\mathit{R}}"),              // MATHEMATICAL ITALIC CAPITAL R
        ('\u{1D446}', r"\ensuremath{\mathit{S}}"),              // MATHEMATICAL ITALIC CAPITAL S
        ('\u{1D447}', r"\ensuremath{\mathit{T}}"),              // MATHEMATICAL ITALIC CAPITAL T
        ('\u{1D448}', r"\ensuremath{\mathit{U}}"),              // MATHEMATICAL ITALIC CAPITAL U
        ('\u{1D449}', r"\ensuremath{\mathit{V}}"),              // MATHEMATICAL ITALIC CAPITAL V
        ('\u{1D44A}', r"\ensuremath{\mathit{W}}"),              // MATHEMATICAL ITALIC CAPITAL W
        ('\u{1D44B}', r"\ensuremath{\mathit{X}}"),              // MATHEMATICAL ITALIC CAPITAL X
        ('\u{1D44C}', r"\ensuremath{\mathit{Y}}"),              // MATHEMATICAL ITALIC CAPITAL Y
        ('\u{1D44D}', r"\ensuremath{\mathit{Z}}"),              // MATHEMATICAL ITALIC CAPITAL Z
        //
        ('\u{1D44E}', r"\ensuremath{\mathit{a}}"),              // MATHEMATICAL ITALIC SMALL a
        ('\u{1D44F}', r"\ensuremath{\mathit{b}}"),              // MATHEMATICAL ITALIC SMALL b
        ('\u{1D450}', r"\ensuremath{\mathit{c}}"),              // MATHEMATICAL ITALIC SMALL c
        ('\u{1D451}', r"\ensuremath{\mathit{d}}"),              // MATHEMATICAL ITALIC SMALL d
        ('\u{1D452}', r"\ensuremath{\mathit{e}}"),              // MATHEMATICAL ITALIC SMALL e
        ('\u{1D453}', r"\ensuremath{\mathit{f}}"),              // MATHEMATICAL ITALIC SMALL f
        ('\u{1D454}', r"\ensuremath{\mathit{g}}"),              // MATHEMATICAL ITALIC SMALL g
        ('\u{1D455}', r"\ensuremath{\mathit{h}}"),              // MATHEMATICAL ITALIC SMALL h
        ('\u{1D456}', r"\ensuremath{\mathit{i}}"),              // MATHEMATICAL ITALIC SMALL i
        ('\u{1D457}', r"\ensuremath{\mathit{j}}"),              // MATHEMATICAL ITALIC SMALL j
        ('\u{1D458}', r"\ensuremath{\mathit{k}}"),              // MATHEMATICAL ITALIC SMALL k
        ('\u{1D459}', r"\ensuremath{\mathit{l}}"),              // MATHEMATICAL ITALIC SMALL l
        ('\u{1D45A}', r"\ensuremath{\mathit{m}}"),              // MATHEMATICAL ITALIC SMALL m
        ('\u{1D45B}', r"\ensuremath{\mathit{n}}"),              // MATHEMATICAL ITALIC SMALL n
        ('\u{1D45C}', r"\ensuremath{\mathit{o}}"),              // MATHEMATICAL ITALIC SMALL o
        ('\u{1D45D}', r"\ensuremath{\mathit{p}}"),              // MATHEMATICAL ITALIC SMALL p
        ('\u{1D45E}', r"\ensuremath{\mathit{q}}"),              // MATHEMATICAL ITALIC SMALL q
        ('\u{1D45F}', r"\ensuremath{\mathit{r}}"),              // MATHEMATICAL ITALIC SMALL r
        ('\u{1D460}', r"\ensuremath{\mathit{s}}"),              // MATHEMATICAL ITALIC SMALL s
        ('\u{1D461}', r"\ensuremath{\mathit{t}}"),              // MATHEMATICAL ITALIC SMALL t
        ('\u{1D462}', r"\ensuremath{\mathit{u}}"),              // MATHEMATICAL ITALIC SMALL u
        ('\u{1D463}', r"\ensuremath{\mathit{v}}"),              // MATHEMATICAL ITALIC SMALL v
        ('\u{1D464}', r"\ensuremath{\mathit{w}}"),              // MATHEMATICAL ITALIC SMALL w
        ('\u{1D465}', r"\ensuremath{\mathit{x}}"),              // MATHEMATICAL ITALIC SMALL x
        ('\u{1D466}', r"\ensuremath{\mathit{y}}"),              // MATHEMATICAL ITALIC SMALL y
        ('\u{1D467}', r"\ensuremath{\mathit{z}}"),              // MATHEMATICAL ITALIC SMALL z
        //
        ('\u{1D468}', r"\ensuremath{\boldsymbol{\mathit{A}}}"), // MATHEMATICAL BOLD ITALIC CAPITAL A
        ('\u{1D469}', r"\ensuremath{\boldsymbol{\mathit{B}}}"), // MATHEMATICAL BOLD ITALIC CAPITAL B
        ('\u{1D46A}', r"\ensuremath{\boldsymbol{\mathit{C}}}"), // MATHEMATICAL BOLD ITALIC CAPITAL C
        ('\u{1D46B}', r"\ensuremath{\boldsymbol{\mathit{D}}}"), // MATHEMATICAL BOLD ITALIC CAPITAL D
        ('\u{1D46C}', r"\ensuremath{\boldsymbol{\mathit{E}}}"), // MATHEMATICAL BOLD ITALIC CAPITAL E
        ('\u{1D46D}', r"\ensuremath{\boldsymbol{\mathit{F}}}"), // MATHEMATICAL BOLD ITALIC CAPITAL F
        ('\u{1D46E}', r"\ensuremath{\boldsymbol{\mathit{G}}}"), // MATHEMATICAL BOLD ITALIC CAPITAL G
        ('\u{1D46F}', r"\ensuremath{\boldsymbol{\mathit{H}}}"), // MATHEMATICAL BOLD ITALIC CAPITAL H
        ('\u{1D470}', r"\ensuremath{\boldsymbol{\mathit{I}}}"), // MATHEMATICAL BOLD ITALIC CAPITAL I
        ('\u{1D471}', r"\ensuremath{\boldsymbol{\mathit{J}}}"), // MATHEMATICAL BOLD ITALIC CAPITAL J
        ('\u{1D472}', r"\ensuremath{\boldsymbol{\mathit{K}}}"), // MATHEMATICAL BOLD ITALIC CAPITAL K
        ('\u{1D473}', r"\ensuremath{\boldsymbol{\mathit{L}}}"), // MATHEMATICAL BOLD ITALIC CAPITAL L
        ('\u{1D474}', r"\ensuremath{\boldsymbol{\mathit{M}}}"), // MATHEMATICAL BOLD ITALIC CAPITAL M
        ('\u{1D475}', r"\ensuremath{\boldsymbol{\mathit{N}}}"), // MATHEMATICAL BOLD ITALIC CAPITAL N
        ('\u{1D476}', r"\ensuremath{\boldsymbol{\mathit{O}}}"), // MATHEMATICAL BOLD ITALIC CAPITAL O
        ('\u{1D477}', r"\ensuremath{\boldsymbol{\mathit{P}}}"), // MATHEMATICAL BOLD ITALIC CAPITAL P
        ('\u{1D478}', r"\ensuremath{\boldsymbol{\mathit{Q}}}"), // MATHEMATICAL BOLD ITALIC CAPITAL Q
        ('\u{1D479}', r"\ensuremath{\boldsymbol{\mathit{R}}}"), // MATHEMATICAL BOLD ITALIC CAPITAL R
        ('\u{1D47A}', r"\ensuremath{\boldsymbol{\mathit{S}}}"), // MATHEMATICAL BOLD ITALIC CAPITAL S
        ('\u{1D47B}', r"\ensuremath{\boldsymbol{\mathit{T}}}"), // MATHEMATICAL BOLD ITALIC CAPITAL T
        ('\u{1D47C}', r"\ensuremath{\boldsymbol{\mathit{U}}}"), // MATHEMATICAL BOLD ITALIC CAPITAL U
        ('\u{1D47D}', r"\ensuremath{\boldsymbol{\mathit{V}}}"), // MATHEMATICAL BOLD ITALIC CAPITAL V
        ('\u{1D47E}', r"\ensuremath{\boldsymbol{\mathit{W}}}"), // MATHEMATICAL BOLD ITALIC CAPITAL W
        ('\u{1D47F}', r"\ensuremath{\boldsymbol{\mathit{X}}}"), // MATHEMATICAL BOLD ITALIC CAPITAL X
        ('\u{1D480}', r"\ensuremath{\boldsymbol{\mathit{Y}}}"), // MATHEMATICAL BOLD ITALIC CAPITAL Y
        ('\u{1D481}', r"\ensuremath{\boldsymbol{\mathit{Z}}}"), // MATHEMATICAL BOLD ITALIC CAPITAL Z
        //
        ('\u{1D482}', r"\ensuremath{\boldsymbol{\mathit{a}}}"), // MATHEMATICAL BOLD ITALIC SMALL a
        ('\u{1D483}', r"\ensuremath{\boldsymbol{\mathit{b}}}"), // MATHEMATICAL BOLD ITALIC SMALL b
        ('\u{1D484}', r"\ensuremath{\boldsymbol{\mathit{c}}}"), // MATHEMATICAL BOLD ITALIC SMALL c
        ('\u{1D485}', r"\ensuremath{\boldsymbol{\mathit{d}}}"), // MATHEMATICAL BOLD ITALIC SMALL d
        ('\u{1D486}', r"\ensuremath{\boldsymbol{\mathit{e}}}"), // MATHEMATICAL BOLD ITALIC SMALL e
        ('\u{1D487}', r"\ensuremath{\boldsymbol{\mathit{f}}}"), // MATHEMATICAL BOLD ITALIC SMALL f
        ('\u{1D488}', r"\ensuremath{\boldsymbol{\mathit{g}}}"), // MATHEMATICAL BOLD ITALIC SMALL g
        ('\u{1D489}', r"\ensuremath{\boldsymbol{\mathit{h}}}"), // MATHEMATICAL BOLD ITALIC SMALL h
        ('\u{1D48A}', r"\ensuremath{\boldsymbol{\mathit{i}}}"), // MATHEMATICAL BOLD ITALIC SMALL i
        ('\u{1D48B}', r"\ensuremath{\boldsymbol{\mathit{j}}}"), // MATHEMATICAL BOLD ITALIC SMALL j
        ('\u{1D48C}', r"\ensuremath{\boldsymbol{\mathit{k}}}"), // MATHEMATICAL BOLD ITALIC SMALL k
        ('\u{1D48D}', r"\ensuremath{\boldsymbol{\mathit{l}}}"), // MATHEMATICAL BOLD ITALIC SMALL l
        ('\u{1D48E}', r"\ensuremath{\boldsymbol{\mathit{m}}}"), // MATHEMATICAL BOLD ITALIC SMALL m
        ('\u{1D48F}', r"\ensuremath{\boldsymbol{\mathit{n}}}"), // MATHEMATICAL BOLD ITALIC SMALL n
        ('\u{1D490}', r"\ensuremath{\boldsymbol{\mathit{o}}}"), // MATHEMATICAL BOLD ITALIC SMALL o
        ('\u{1D491}', r"\ensuremath{\boldsymbol{\mathit{p}}}"), // MATHEMATICAL BOLD ITALIC SMALL p
        ('\u{1D492}', r"\ensuremath{\boldsymbol{\mathit{q}}}"), // MATHEMATICAL BOLD ITALIC SMALL q
        ('\u{1D493}', r"\ensuremath{\boldsymbol{\mathit{r}}}"), // MATHEMATICAL BOLD ITALIC SMALL r
        ('\u{1D494}', r"\ensuremath{\boldsymbol{\mathit{s}}}"), // MATHEMATICAL BOLD ITALIC SMALL s
        ('\u{1D495}', r"\ensuremath{\boldsymbol{\mathit{t}}}"), // MATHEMATICAL BOLD ITALIC SMALL t
        ('\u{1D496}', r"\ensuremath{\boldsymbol{\mathit{u}}}"), // MATHEMATICAL BOLD ITALIC SMALL u
        ('\u{1D497}', r"\ensuremath{\boldsymbol{\mathit{v}}}"), // MATHEMATICAL BOLD ITALIC SMALL v
        ('\u{1D498}', r"\ensuremath{\boldsymbol{\mathit{w}}}"), // MATHEMATICAL BOLD ITALIC SMALL w
        ('\u{1D499}', r"\ensuremath{\boldsymbol{\mathit{x}}}"), // MATHEMATICAL BOLD ITALIC SMALL x
        ('\u{1D49A}', r"\ensuremath{\boldsymbol{\mathit{y}}}"), // MATHEMATICAL BOLD ITALIC SMALL y
        ('\u{1D49B}', r"\ensuremath{\boldsymbol{\mathit{z}}}"), // MATHEMATICAL BOLD ITALIC SMALL z
        //
        ('\u{1D49C}', r"\ensuremath{\mathscr{A}}"),             // MATHEMATICAL SCRIPT CAPITAL A
        ('\u{1D49D}', r"\ensuremath{\mathscr{B}}"),             // MATHEMATICAL SCRIPT CAPITAL B
        ('\u{1D49E}', r"\ensuremath{\mathscr{C}}"),             // MATHEMATICAL SCRIPT CAPITAL C
        ('\u{1D49F}', r"\ensuremath{\mathscr{D}}"),             // MATHEMATICAL SCRIPT CAPITAL D
        ('\u{1D4A0}', r"\ensuremath{\mathscr{E}}"),             // MATHEMATICAL SCRIPT CAPITAL E
        ('\u{1D4A1}', r"\ensuremath{\mathscr{F}}"),             // MATHEMATICAL SCRIPT CAPITAL F
        ('\u{1D4A2}', r"\ensuremath{\mathscr{G}}"),             // MATHEMATICAL SCRIPT CAPITAL G
        ('\u{1D4A3}', r"\ensuremath{\mathscr{H}}"),             // MATHEMATICAL SCRIPT CAPITAL H
        ('\u{1D4A4}', r"\ensuremath{\mathscr{I}}"),             // MATHEMATICAL SCRIPT CAPITAL I
        ('\u{1D4A5}', r"\ensuremath{\mathscr{J}}"),             // MATHEMATICAL SCRIPT CAPITAL J
        ('\u{1D4A6}', r"\ensuremath{\mathscr{K}}"),             // MATHEMATICAL SCRIPT CAPITAL K
        ('\u{1D4A7}', r"\ensuremath{\mathscr{L}}"),             // MATHEMATICAL SCRIPT CAPITAL L
        ('\u{1D4A8}', r"\ensuremath{\mathscr{M}}"),             // MATHEMATICAL SCRIPT CAPITAL M
        ('\u{1D4A9}', r"\ensuremath{\mathscr{N}}"),             // MATHEMATICAL SCRIPT CAPITAL N
        ('\u{1D4AA}', r"\ensuremath{\mathscr{O}}"),             // MATHEMATICAL SCRIPT CAPITAL O
        ('\u{1D4AB}', r"\ensuremath{\mathscr{P}}"),             // MATHEMATICAL SCRIPT CAPITAL P
        ('\u{1D4AC}', r"\ensuremath{\mathscr{Q}}"),             // MATHEMATICAL SCRIPT CAPITAL Q
        ('\u{1D4AD}', r"\ensuremath{\mathscr{R}}"),             // MATHEMATICAL SCRIPT CAPITAL R
        ('\u{1D4AE}', r"\ensuremath{\mathscr{S}}"),             // MATHEMATICAL SCRIPT CAPITAL S
        ('\u{1D4AF}', r"\ensuremath{\mathscr{T}}"),             // MATHEMATICAL SCRIPT CAPITAL T
        ('\u{1D4B0}', r"\ensuremath{\mathscr{U}}"),             // MATHEMATICAL SCRIPT CAPITAL U
        ('\u{1D4B1}', r"\ensuremath{\mathscr{V}}"),             // MATHEMATICAL SCRIPT CAPITAL V
        ('\u{1D4B2}', r"\ensuremath{\mathscr{W}}"),             // MATHEMATICAL SCRIPT CAPITAL W
        ('\u{1D4B3}', r"\ensuremath{\mathscr{X}}"),             // MATHEMATICAL SCRIPT CAPITAL X
        ('\u{1D4B4}', r"\ensuremath{\mathscr{Y}}"),             // MATHEMATICAL SCRIPT CAPITAL Y
        ('\u{1D4B5}', r"\ensuremath{\mathscr{Z}}"),             // MATHEMATICAL SCRIPT CAPITAL Z
        ('\u{1D4B6}', r"\ensuremath{\mathscr{a}}"),                          // MATHEMATICAL SCRIPT SMALL A [𝒶]
        ('\u{1D4B7}', r"\ensuremath{\mathscr{b}}"),                          // MATHEMATICAL SCRIPT SMALL B [𝒷]
        ('\u{1D4B8}', r"\ensuremath{\mathscr{c}}"),                          // MATHEMATICAL SCRIPT SMALL C [𝒸]
        ('\u{1D4B9}', r"\ensuremath{\mathscr{d}}"),                          // MATHEMATICAL SCRIPT SMALL D [𝒹]
        ('\u{1D4BB}', r"\ensuremath{\mathscr{f}}"),                          // MATHEMATICAL SCRIPT SMALL F [𝒻]
        ('\u{1D4BD}', r"\ensuremath{\mathscr{h}}"),                          // MATHEMATICAL SCRIPT SMALL H [𝒽]
        ('\u{1D4BE}', r"\ensuremath{\mathscr{i}}"),                          // MATHEMATICAL SCRIPT SMALL I [𝒾]
        ('\u{1D4BF}', r"\ensuremath{\mathscr{j}}"),                          // MATHEMATICAL SCRIPT SMALL J [𝒿]
        ('\u{1D4C0}', r"\ensuremath{\mathscr{k}}"),                          // MATHEMATICAL SCRIPT SMALL K [𝓀]
        ('\u{1D4C1}', r"\ensuremath{\mathscr{l}}"),                          // MATHEMATICAL SCRIPT SMALL L [𝓁]
        ('\u{1D4C2}', r"\ensuremath{\mathscr{m}}"),                          // MATHEMATICAL SCRIPT SMALL M [𝓂]
        ('\u{1D4C3}', r"\ensuremath{\mathscr{n}}"),                          // MATHEMATICAL SCRIPT SMALL N [𝓃]
        ('\u{1D4C5}', r"\ensuremath{\mathscr{p}}"),                          // MATHEMATICAL SCRIPT SMALL P [𝓅]
        ('\u{1D4C6}', r"\ensuremath{\mathscr{q}}"),                          // MATHEMATICAL SCRIPT SMALL Q [𝓆]
        ('\u{1D4C7}', r"\ensuremath{\mathscr{r}}"),                          // MATHEMATICAL SCRIPT SMALL R [𝓇]
        ('\u{1D4C8}', r"\ensuremath{\mathscr{s}}"),                          // MATHEMATICAL SCRIPT SMALL S [𝓈]
        ('\u{1D4C9}', r"\ensuremath{\mathscr{t}}"),                          // MATHEMATICAL SCRIPT SMALL T [𝓉]
        ('\u{1D4CA}', r"\ensuremath{\mathscr{u}}"),                          // MATHEMATICAL SCRIPT SMALL U [𝓊]
        ('\u{1D4CB}', r"\ensuremath{\mathscr{v}}"),                          // MATHEMATICAL SCRIPT SMALL V [𝓋]
        ('\u{1D4CC}', r"\ensuremath{\mathscr{w}}"),                          // MATHEMATICAL SCRIPT SMALL W [𝓌]
        ('\u{1D4CD}', r"\ensuremath{\mathscr{x}}"),                          // MATHEMATICAL SCRIPT SMALL X [𝓍]
        ('\u{1D4CE}', r"\ensuremath{\mathscr{y}}"),                          // MATHEMATICAL SCRIPT SMALL Y [𝓎]
        ('\u{1D4CF}', r"\ensuremath{\mathscr{z}}"),                          // MATHEMATICAL SCRIPT SMALL Z [𝓏]
        //
        ('\u{1D504}', r"\ensuremath{\mathfrak{A}}"),            // MATHEMATICAL FRAKTUR CAPITAL A
        ('\u{1D505}', r"\ensuremath{\mathfrak{B}}"),            // MATHEMATICAL FRAKTUR CAPITAL B
        ('\u{1D506}', r"\ensuremath{\mathfrak{C}}"),            // MATHEMATICAL FRAKTUR CAPITAL C
        ('\u{1D507}', r"\ensuremath{\mathfrak{D}}"),            // MATHEMATICAL FRAKTUR CAPITAL D
        ('\u{1D508}', r"\ensuremath{\mathfrak{E}}"),            // MATHEMATICAL FRAKTUR CAPITAL E
        ('\u{1D509}', r"\ensuremath{\mathfrak{F}}"),            // MATHEMATICAL FRAKTUR CAPITAL F
        ('\u{1D50A}', r"\ensuremath{\mathfrak{G}}"),            // MATHEMATICAL FRAKTUR CAPITAL G
        ('\u{1D50B}', r"\ensuremath{\mathfrak{H}}"),            // MATHEMATICAL FRAKTUR CAPITAL H
        ('\u{1D50C}', r"\ensuremath{\mathfrak{I}}"),            // MATHEMATICAL FRAKTUR CAPITAL I
        ('\u{1D50D}', r"\ensuremath{\mathfrak{J}}"),            // MATHEMATICAL FRAKTUR CAPITAL J
        ('\u{1D50E}', r"\ensuremath{\mathfrak{K}}"),            // MATHEMATICAL FRAKTUR CAPITAL K
        ('\u{1D50F}', r"\ensuremath{\mathfrak{L}}"),            // MATHEMATICAL FRAKTUR CAPITAL L
        ('\u{1D510}', r"\ensuremath{\mathfrak{M}}"),            // MATHEMATICAL FRAKTUR CAPITAL M
        ('\u{1D511}', r"\ensuremath{\mathfrak{N}}"),            // MATHEMATICAL FRAKTUR CAPITAL N
        ('\u{1D512}', r"\ensuremath{\mathfrak{O}}"),            // MATHEMATICAL FRAKTUR CAPITAL O
        ('\u{1D513}', r"\ensuremath{\mathfrak{P}}"),            // MATHEMATICAL FRAKTUR CAPITAL P
        ('\u{1D514}', r"\ensuremath{\mathfrak{Q}}"),            // MATHEMATICAL FRAKTUR CAPITAL Q
        ('\u{1D515}', r"\ensuremath{\mathfrak{R}}"),            // MATHEMATICAL FRAKTUR CAPITAL R
        ('\u{1D516}', r"\ensuremath{\mathfrak{S}}"),            // MATHEMATICAL FRAKTUR CAPITAL S
        ('\u{1D517}', r"\ensuremath{\mathfrak{T}}"),            // MATHEMATICAL FRAKTUR CAPITAL T
        ('\u{1D518}', r"\ensuremath{\mathfrak{U}}"),            // MATHEMATICAL FRAKTUR CAPITAL U
        ('\u{1D519}', r"\ensuremath{\mathfrak{V}}"),            // MATHEMATICAL FRAKTUR CAPITAL V
        ('\u{1D51A}', r"\ensuremath{\mathfrak{W}}"),            // MATHEMATICAL FRAKTUR CAPITAL W
        ('\u{1D51B}', r"\ensuremath{\mathfrak{X}}"),            // MATHEMATICAL FRAKTUR CAPITAL X
        ('\u{1D51C}', r"\ensuremath{\mathfrak{Y}}"),            // MATHEMATICAL FRAKTUR CAPITAL Y
        ('\u{1D51D}', r"\ensuremath{\mathfrak{Z}}"),            // MATHEMATICAL FRAKTUR CAPITAL Z
        //
        ('\u{1D51E}', r"\ensuremath{\mathfrak{a}}"),            // MATHEMATICAL FRAKTUR SMALL a
        ('\u{1D51F}', r"\ensuremath{\mathfrak{b}}"),            // MATHEMATICAL FRAKTUR SMALL b
        ('\u{1D520}', r"\ensuremath{\mathfrak{c}}"),            // MATHEMATICAL FRAKTUR SMALL c
        ('\u{1D521}', r"\ensuremath{\mathfrak{d}}"),            // MATHEMATICAL FRAKTUR SMALL d
        ('\u{1D522}', r"\ensuremath{\mathfrak{e}}"),            // MATHEMATICAL FRAKTUR SMALL e
        ('\u{1D523}', r"\ensuremath{\mathfrak{f}}"),            // MATHEMATICAL FRAKTUR SMALL f
        ('\u{1D524}', r"\ensuremath{\mathfrak{g}}"),            // MATHEMATICAL FRAKTUR SMALL g
        ('\u{1D525}', r"\ensuremath{\mathfrak{h}}"),            // MATHEMATICAL FRAKTUR SMALL h
        ('\u{1D526}', r"\ensuremath{\mathfrak{i}}"),            // MATHEMATICAL FRAKTUR SMALL i
        ('\u{1D527}', r"\ensuremath{\mathfrak{j}}"),            // MATHEMATICAL FRAKTUR SMALL j
        ('\u{1D528}', r"\ensuremath{\mathfrak{k}}"),            // MATHEMATICAL FRAKTUR SMALL k
        ('\u{1D529}', r"\ensuremath{\mathfrak{l}}"),            // MATHEMATICAL FRAKTUR SMALL l
        ('\u{1D52A}', r"\ensuremath{\mathfrak{m}}"),            // MATHEMATICAL FRAKTUR SMALL m
        ('\u{1D52B}', r"\ensuremath{\mathfrak{n}}"),            // MATHEMATICAL FRAKTUR SMALL n
        ('\u{1D52C}', r"\ensuremath{\mathfrak{o}}"),            // MATHEMATICAL FRAKTUR SMALL o
        ('\u{1D52D}', r"\ensuremath{\mathfrak{p}}"),            // MATHEMATICAL FRAKTUR SMALL p
        ('\u{1D52E}', r"\ensuremath{\mathfrak{q}}"),            // MATHEMATICAL FRAKTUR SMALL q
        ('\u{1D52F}', r"\ensuremath{\mathfrak{r}}"),            // MATHEMATICAL FRAKTUR SMALL r
        ('\u{1D530}', r"\ensuremath{\mathfrak{s}}"),            // MATHEMATICAL FRAKTUR SMALL s
        ('\u{1D531}', r"\ensuremath{\mathfrak{t}}"),            // MATHEMATICAL FRAKTUR SMALL t
        ('\u{1D532}', r"\ensuremath{\mathfrak{u}}"),            // MATHEMATICAL FRAKTUR SMALL u
        ('\u{1D533}', r"\ensuremath{\mathfrak{v}}"),            // MATHEMATICAL FRAKTUR SMALL v
        ('\u{1D534}', r"\ensuremath{\mathfrak{w}}"),            // MATHEMATICAL FRAKTUR SMALL w
        ('\u{1D535}', r"\ensuremath{\mathfrak{x}}"),            // MATHEMATICAL FRAKTUR SMALL x
        ('\u{1D536}', r"\ensuremath{\mathfrak{y}}"),            // MATHEMATICAL FRAKTUR SMALL y
        ('\u{1D537}', r"\ensuremath{\mathfrak{z}}"),            // MATHEMATICAL FRAKTUR SMALL z
        //
        ('\u{1D538}', r"\ensuremath{\mathbb{A}}"),              // MATHEMATICAL DOUBLE-STRUCK CAPITAL A
        ('\u{1D539}', r"\ensuremath{\mathbb{B}}"),              // MATHEMATICAL DOUBLE-STRUCK CAPITAL B
        ('\u{1D53A}', r"\ensuremath{\mathbb{C}}"),              // MATHEMATICAL DOUBLE-STRUCK CAPITAL C
        ('\u{1D53B}', r"\ensuremath{\mathbb{D}}"),              // MATHEMATICAL DOUBLE-STRUCK CAPITAL D
        ('\u{1D53C}', r"\ensuremath{\mathbb{E}}"),              // MATHEMATICAL DOUBLE-STRUCK CAPITAL E
        ('\u{1D53D}', r"\ensuremath{\mathbb{F}}"),              // MATHEMATICAL DOUBLE-STRUCK CAPITAL F
        ('\u{1D53E}', r"\ensuremath{\mathbb{G}}"),              // MATHEMATICAL DOUBLE-STRUCK CAPITAL G
        ('\u{1D53F}', r"\ensuremath{\mathbb{H}}"),              // MATHEMATICAL DOUBLE-STRUCK CAPITAL H
        ('\u{1D540}', r"\ensuremath{\mathbb{I}}"),              // MATHEMATICAL DOUBLE-STRUCK CAPITAL I
        ('\u{1D541}', r"\ensuremath{\mathbb{J}}"),              // MATHEMATICAL DOUBLE-STRUCK CAPITAL J
        ('\u{1D542}', r"\ensuremath{\mathbb{K}}"),              // MATHEMATICAL DOUBLE-STRUCK CAPITAL K
        ('\u{1D543}', r"\ensuremath{\mathbb{L}}"),              // MATHEMATICAL DOUBLE-STRUCK CAPITAL L
        ('\u{1D544}', r"\ensuremath{\mathbb{M}}"),              // MATHEMATICAL DOUBLE-STRUCK CAPITAL M
        ('\u{1D545}', r"\ensuremath{\mathbb{N}}"),              // MATHEMATICAL DOUBLE-STRUCK CAPITAL N
        ('\u{1D546}', r"\ensuremath{\mathbb{O}}"),              // MATHEMATICAL DOUBLE-STRUCK CAPITAL O
        ('\u{1D547}', r"\ensuremath{\mathbb{P}}"),              // MATHEMATICAL DOUBLE-STRUCK CAPITAL P
        ('\u{1D548}', r"\ensuremath{\mathbb{Q}}"),              // MATHEMATICAL DOUBLE-STRUCK CAPITAL Q
        ('\u{1D549}', r"\ensuremath{\mathbb{R}}"),              // MATHEMATICAL DOUBLE-STRUCK CAPITAL R
        ('\u{1D54A}', r"\ensuremath{\mathbb{S}}"),              // MATHEMATICAL DOUBLE-STRUCK CAPITAL S
        ('\u{1D54B}', r"\ensuremath{\mathbb{T}}"),              // MATHEMATICAL DOUBLE-STRUCK CAPITAL T
        ('\u{1D54C}', r"\ensuremath{\mathbb{U}}"),              // MATHEMATICAL DOUBLE-STRUCK CAPITAL U
        ('\u{1D54D}', r"\ensuremath{\mathbb{V}}"),              // MATHEMATICAL DOUBLE-STRUCK CAPITAL V
        ('\u{1D54E}', r"\ensuremath{\mathbb{W}}"),              // MATHEMATICAL DOUBLE-STRUCK CAPITAL W
        ('\u{1D54F}', r"\ensuremath{\mathbb{X}}"),              // MATHEMATICAL DOUBLE-STRUCK CAPITAL X
        ('\u{1D550}', r"\ensuremath{\mathbb{Y}}"),              // MATHEMATICAL DOUBLE-STRUCK CAPITAL Y
        ('\u{1D551}', r"\ensuremath{\mathbb{Z}}"),              // MATHEMATICAL DOUBLE-STRUCK CAPITAL Z
        ('\u{1D552}', r"\ensuremath{\mathbb{a}}"),                           // MATHEMATICAL DOUBLE-STRUCK SMALL A [𝕒]
        ('\u{1D553}', r"\ensuremath{\mathbb{b}}"),                           // MATHEMATICAL DOUBLE-STRUCK SMALL B [𝕓]
        ('\u{1D554}', r"\ensuremath{\mathbb{c}}"),                           // MATHEMATICAL DOUBLE-STRUCK SMALL C [𝕔]
        ('\u{1D555}', r"\ensuremath{\mathbb{d}}"),                           // MATHEMATICAL DOUBLE-STRUCK SMALL D [𝕕]
        ('\u{1D556}', r"\ensuremath{\mathbb{e}}"),                           // MATHEMATICAL DOUBLE-STRUCK SMALL E [𝕖]
        ('\u{1D557}', r"\ensuremath{\mathbb{f}}"),                           // MATHEMATICAL DOUBLE-STRUCK SMALL F [𝕗]
        ('\u{1D558}', r"\ensuremath{\mathbb{g}}"),                           // MATHEMATICAL DOUBLE-STRUCK SMALL G [𝕘]
        ('\u{1D559}', r"\ensuremath{\mathbb{h}}"),                           // MATHEMATICAL DOUBLE-STRUCK SMALL H [𝕙]
        ('\u{1D55A}', r"\ensuremath{\mathbb{i}}"),                           // MATHEMATICAL DOUBLE-STRUCK SMALL I [𝕚]
        ('\u{1D55B}', r"\ensuremath{\mathbb{j}}"),                           // MATHEMATICAL DOUBLE-STRUCK SMALL J [𝕛]
        ('\u{1D55C}', r"\ensuremath{\mathbb{k}}"),                           // MATHEMATICAL DOUBLE-STRUCK SMALL K [𝕜]
        ('\u{1D55D}', r"\ensuremath{\mathbb{l}}"),                           // MATHEMATICAL DOUBLE-STRUCK SMALL L [𝕝]
        ('\u{1D55E}', r"\ensuremath{\mathbb{m}}"),                           // MATHEMATICAL DOUBLE-STRUCK SMALL M [𝕞]
        ('\u{1D55F}', r"\ensuremath{\mathbb{n}}"),                           // MATHEMATICAL DOUBLE-STRUCK SMALL N [𝕟]
        ('\u{1D560}', r"\ensuremath{\mathbb{o}}"),                           // MATHEMATICAL DOUBLE-STRUCK SMALL O [𝕠]
        ('\u{1D561}', r"\ensuremath{\mathbb{p}}"),                           // MATHEMATICAL DOUBLE-STRUCK SMALL P [𝕡]
        ('\u{1D562}', r"\ensuremath{\mathbb{q}}"),                           // MATHEMATICAL DOUBLE-STRUCK SMALL Q [𝕢]
        ('\u{1D563}', r"\ensuremath{\mathbb{r}}"),                           // MATHEMATICAL DOUBLE-STRUCK SMALL R [𝕣]
        ('\u{1D564}', r"\ensuremath{\mathbb{s}}"),                           // MATHEMATICAL DOUBLE-STRUCK SMALL S [𝕤]
        ('\u{1D565}', r"\ensuremath{\mathbb{t}}"),                           // MATHEMATICAL DOUBLE-STRUCK SMALL T [𝕥]
        ('\u{1D566}', r"\ensuremath{\mathbb{u}}"),                           // MATHEMATICAL DOUBLE-STRUCK SMALL U [𝕦]
        ('\u{1D567}', r"\ensuremath{\mathbb{v}}"),                           // MATHEMATICAL DOUBLE-STRUCK SMALL V [𝕧]
        ('\u{1D568}', r"\ensuremath{\mathbb{w}}"),                           // MATHEMATICAL DOUBLE-STRUCK SMALL W [𝕨]
        ('\u{1D569}', r"\ensuremath{\mathbb{x}}"),                           // MATHEMATICAL DOUBLE-STRUCK SMALL X [𝕩]
        ('\u{1D56A}', r"\ensuremath{\mathbb{y}}"),                           // MATHEMATICAL DOUBLE-STRUCK SMALL Y [𝕪]
        ('\u{1D56B}', r"\ensuremath{\mathbb{z}}"),                           // MATHEMATICAL DOUBLE-STRUCK SMALL Z [𝕫]
        //
        ('\u{1D5A0}', r"\ensuremath{\mathsf{A}}"),                           // MATHEMATICAL SANS-SERIF CAPITAL A [𝖠]
        ('\u{1D5A1}', r"\ensuremath{\mathsf{B}}"),                           // MATHEMATICAL SANS-SERIF CAPITAL B [𝖡]
        ('\u{1D5A2}', r"\ensuremath{\mathsf{C}}"),                           // MATHEMATICAL SANS-SERIF CAPITAL C [𝖢]
        ('\u{1D5A3}', r"\ensuremath{\mathsf{D}}"),                           // MATHEMATICAL SANS-SERIF CAPITAL D [𝖣]
        ('\u{1D5A4}', r"\ensuremath{\mathsf{E}}"),                           // MATHEMATICAL SANS-SERIF CAPITAL E [𝖤]
        ('\u{1D5A5}', r"\ensuremath{\mathsf{F}}"),                           // MATHEMATICAL SANS-SERIF CAPITAL F [𝖥]
        ('\u{1D5A6}', r"\ensuremath{\mathsf{G}}"),                           // MATHEMATICAL SANS-SERIF CAPITAL G [𝖦]
        ('\u{1D5A7}', r"\ensuremath{\mathsf{H}}"),                           // MATHEMATICAL SANS-SERIF CAPITAL H [𝖧]
        ('\u{1D5A8}', r"\ensuremath{\mathsf{I}}"),                           // MATHEMATICAL SANS-SERIF CAPITAL I [𝖨]
        ('\u{1D5A9}', r"\ensuremath{\mathsf{J}}"),                           // MATHEMATICAL SANS-SERIF CAPITAL J [𝖩]
        ('\u{1D5AA}', r"\ensuremath{\mathsf{K}}"),                           // MATHEMATICAL SANS-SERIF CAPITAL K [𝖪]
        ('\u{1D5AB}', r"\ensuremath{\mathsf{L}}"),                           // MATHEMATICAL SANS-SERIF CAPITAL L [𝖫]
        ('\u{1D5AC}', r"\ensuremath{\mathsf{M}}"),                           // MATHEMATICAL SANS-SERIF CAPITAL M [𝖬]
        ('\u{1D5AD}', r"\ensuremath{\mathsf{N}}"),                           // MATHEMATICAL SANS-SERIF CAPITAL N [𝖭]
        ('\u{1D5AE}', r"\ensuremath{\mathsf{O}}"),                           // MATHEMATICAL SANS-SERIF CAPITAL O [𝖮]
        ('\u{1D5AF}', r"\ensuremath{\mathsf{P}}"),                           // MATHEMATICAL SANS-SERIF CAPITAL P [𝖯]
        ('\u{1D5B0}', r"\ensuremath{\mathsf{Q}}"),                           // MATHEMATICAL SANS-SERIF CAPITAL Q [𝖰]
        ('\u{1D5B1}', r"\ensuremath{\mathsf{R}}"),                           // MATHEMATICAL SANS-SERIF CAPITAL R [𝖱]
        ('\u{1D5B2}', r"\ensuremath{\mathsf{S}}"),                           // MATHEMATICAL SANS-SERIF CAPITAL S [𝖲]
        ('\u{1D5B3}', r"\ensuremath{\mathsf{T}}"),                           // MATHEMATICAL SANS-SERIF CAPITAL T [𝖳]
        ('\u{1D5B4}', r"\ensuremath{\mathsf{U}}"),                           // MATHEMATICAL SANS-SERIF CAPITAL U [𝖴]
        ('\u{1D5B5}', r"\ensuremath{\mathsf{V}}"),                           // MATHEMATICAL SANS-SERIF CAPITAL V [𝖵]
        ('\u{1D5B6}', r"\ensuremath{\mathsf{W}}"),                           // MATHEMATICAL SANS-SERIF CAPITAL W [𝖶]
        ('\u{1D5B7}', r"\ensuremath{\mathsf{X}}"),                           // MATHEMATICAL SANS-SERIF CAPITAL X [𝖷]
        ('\u{1D5B8}', r"\ensuremath{\mathsf{Y}}"),                           // MATHEMATICAL SANS-SERIF CAPITAL Y [𝖸]
        ('\u{1D5B9}', r"\ensuremath{\mathsf{Z}}"),                           // MATHEMATICAL SANS-SERIF CAPITAL Z [𝖹]
        ('\u{1D5BA}', r"\ensuremath{\mathsf{a}}"),                           // MATHEMATICAL SANS-SERIF SMALL A [𝖺]
        ('\u{1D5BB}', r"\ensuremath{\mathsf{b}}"),                           // MATHEMATICAL SANS-SERIF SMALL B [𝖻]
        ('\u{1D5BC}', r"\ensuremath{\mathsf{c}}"),                           // MATHEMATICAL SANS-SERIF SMALL C [𝖼]
        ('\u{1D5BD}', r"\ensuremath{\mathsf{d}}"),                           // MATHEMATICAL SANS-SERIF SMALL D [𝖽]
        ('\u{1D5BE}', r"\ensuremath{\mathsf{e}}"),                           // MATHEMATICAL SANS-SERIF SMALL E [𝖾]
        ('\u{1D5BF}', r"\ensuremath{\mathsf{f}}"),                           // MATHEMATICAL SANS-SERIF SMALL F [𝖿]
        ('\u{1D5C0}', r"\ensuremath{\mathsf{g}}"),                           // MATHEMATICAL SANS-SERIF SMALL G [𝗀]
        ('\u{1D5C1}', r"\ensuremath{\mathsf{h}}"),                           // MATHEMATICAL SANS-SERIF SMALL H [𝗁]
        ('\u{1D5C2}', r"\ensuremath{\mathsf{i}}"),                           // MATHEMATICAL SANS-SERIF SMALL I [𝗂]
        ('\u{1D5C3}', r"\ensuremath{\mathsf{j}}"),                           // MATHEMATICAL SANS-SERIF SMALL J [𝗃]
        ('\u{1D5C4}', r"\ensuremath{\mathsf{k}}"),                           // MATHEMATICAL SANS-SERIF SMALL K [𝗄]
        ('\u{1D5C5}', r"\ensuremath{\mathsf{l}}"),                           // MATHEMATICAL SANS-SERIF SMALL L [𝗅]
        ('\u{1D5C6}', r"\ensuremath{\mathsf{m}}"),                           // MATHEMATICAL SANS-SERIF SMALL M [𝗆]
        ('\u{1D5C7}', r"\ensuremath{\mathsf{n}}"),                           // MATHEMATICAL SANS-SERIF SMALL N [𝗇]
        ('\u{1D5C8}', r"\ensuremath{\mathsf{o}}"),                           // MATHEMATICAL SANS-SERIF SMALL O [𝗈]
        ('\u{1D5C9}', r"\ensuremath{\mathsf{p}}"),                           // MATHEMATICAL SANS-SERIF SMALL P [𝗉]
        ('\u{1D5CA}', r"\ensuremath{\mathsf{q}}"),                           // MATHEMATICAL SANS-SERIF SMALL Q [𝗊]
        ('\u{1D5CB}', r"\ensuremath{\mathsf{r}}"),                           // MATHEMATICAL SANS-SERIF SMALL R [𝗋]
        ('\u{1D5CC}', r"\ensuremath{\mathsf{s}}"),                           // MATHEMATICAL SANS-SERIF SMALL S [𝗌]
        ('\u{1D5CD}', r"\ensuremath{\mathsf{t}}"),                           // MATHEMATICAL SANS-SERIF SMALL T [𝗍]
        ('\u{1D5CE}', r"\ensuremath{\mathsf{u}}"),                           // MATHEMATICAL SANS-SERIF SMALL U [𝗎]
        ('\u{1D5CF}', r"\ensuremath{\mathsf{v}}"),                           // MATHEMATICAL SANS-SERIF SMALL V [𝗏]
        ('\u{1D5D0}', r"\ensuremath{\mathsf{w}}"),                           // MATHEMATICAL SANS-SERIF SMALL W [𝗐]
        ('\u{1D5D1}', r"\ensuremath{\mathsf{x}}"),                           // MATHEMATICAL SANS-SERIF SMALL X [𝗑]
        ('\u{1D5D2}', r"\ensuremath{\mathsf{y}}"),                           // MATHEMATICAL SANS-SERIF SMALL Y [𝗒]
        ('\u{1D5D3}', r"\ensuremath{\mathsf{z}}"),                           // MATHEMATICAL SANS-SERIF SMALL Z [𝗓]
        //
        ('\u{1D670}', r"\ensuremath{\mathtt{A}}"),                           // MATHEMATICAL MONOSPACE CAPITAL A [𝙰]
        ('\u{1D671}', r"\ensuremath{\mathtt{B}}"),                           // MATHEMATICAL MONOSPACE CAPITAL B [𝙱]
        ('\u{1D672}', r"\ensuremath{\mathtt{C}}"),                           // MATHEMATICAL MONOSPACE CAPITAL C [𝙲]
        ('\u{1D673}', r"\ensuremath{\mathtt{D}}"),                           // MATHEMATICAL MONOSPACE CAPITAL D [𝙳]
        ('\u{1D674}', r"\ensuremath{\mathtt{E}}"),                           // MATHEMATICAL MONOSPACE CAPITAL E [𝙴]
        ('\u{1D675}', r"\ensuremath{\mathtt{F}}"),                           // MATHEMATICAL MONOSPACE CAPITAL F [𝙵]
        ('\u{1D676}', r"\ensuremath{\mathtt{G}}"),                           // MATHEMATICAL MONOSPACE CAPITAL G [𝙶]
        ('\u{1D677}', r"\ensuremath{\mathtt{H}}"),                           // MATHEMATICAL MONOSPACE CAPITAL H [𝙷]
        ('\u{1D678}', r"\ensuremath{\mathtt{I}}"),                           // MATHEMATICAL MONOSPACE CAPITAL I [𝙸]
        ('\u{1D679}', r"\ensuremath{\mathtt{J}}"),                           // MATHEMATICAL MONOSPACE CAPITAL J [𝙹]
        ('\u{1D67A}', r"\ensuremath{\mathtt{K}}"),                           // MATHEMATICAL MONOSPACE CAPITAL K [𝙺]
        ('\u{1D67B}', r"\ensuremath{\mathtt{L}}"),                           // MATHEMATICAL MONOSPACE CAPITAL L [𝙻]
        ('\u{1D67C}', r"\ensuremath{\mathtt{M}}"),                           // MATHEMATICAL MONOSPACE CAPITAL M [𝙼]
        ('\u{1D67D}', r"\ensuremath{\mathtt{N}}"),                           // MATHEMATICAL MONOSPACE CAPITAL N [𝙽]
        ('\u{1D67E}', r"\ensuremath{\mathtt{O}}"),                           // MATHEMATICAL MONOSPACE CAPITAL O [𝙾]
        ('\u{1D67F}', r"\ensuremath{\mathtt{P}}"),                           // MATHEMATICAL MONOSPACE CAPITAL P [𝙿]
        ('\u{1D680}', r"\ensuremath{\mathtt{Q}}"),                           // MATHEMATICAL MONOSPACE CAPITAL Q [𝚀]
        ('\u{1D681}', r"\ensuremath{\mathtt{R}}"),                           // MATHEMATICAL MONOSPACE CAPITAL R [𝚁]
        ('\u{1D682}', r"\ensuremath{\mathtt{S}}"),                           // MATHEMATICAL MONOSPACE CAPITAL S [𝚂]
        ('\u{1D683}', r"\ensuremath{\mathtt{T}}"),                           // MATHEMATICAL MONOSPACE CAPITAL T [𝚃]
        ('\u{1D684}', r"\ensuremath{\mathtt{U}}"),                           // MATHEMATICAL MONOSPACE CAPITAL U [𝚄]
        ('\u{1D685}', r"\ensuremath{\mathtt{V}}"),                           // MATHEMATICAL MONOSPACE CAPITAL V [𝚅]
        ('\u{1D686}', r"\ensuremath{\mathtt{W}}"),                           // MATHEMATICAL MONOSPACE CAPITAL W [𝚆]
        ('\u{1D687}', r"\ensuremath{\mathtt{X}}"),                           // MATHEMATICAL MONOSPACE CAPITAL X [𝚇]
        ('\u{1D688}', r"\ensuremath{\mathtt{Y}}"),                           // MATHEMATICAL MONOSPACE CAPITAL Y [𝚈]
        ('\u{1D689}', r"\ensuremath{\mathtt{Z}}"),                           // MATHEMATICAL MONOSPACE CAPITAL Z [𝚉]
        ('\u{1D68A}', r"\ensuremath{\mathtt{a}}"),                           // MATHEMATICAL MONOSPACE SMALL A [𝚊]
        ('\u{1D68B}', r"\ensuremath{\mathtt{b}}"),                           // MATHEMATICAL MONOSPACE SMALL B [𝚋]
        ('\u{1D68C}', r"\ensuremath{\mathtt{c}}"),                           // MATHEMATICAL MONOSPACE SMALL C [𝚌]
        ('\u{1D68D}', r"\ensuremath{\mathtt{d}}"),                           // MATHEMATICAL MONOSPACE SMALL D [𝚍]
        ('\u{1D68E}', r"\ensuremath{\mathtt{e}}"),                           // MATHEMATICAL MONOSPACE SMALL E [𝚎]
        ('\u{1D68F}', r"\ensuremath{\mathtt{f}}"),                           // MATHEMATICAL MONOSPACE SMALL F [𝚏]
        ('\u{1D690}', r"\ensuremath{\mathtt{g}}"),                           // MATHEMATICAL MONOSPACE SMALL G [𝚐]
        ('\u{1D691}', r"\ensuremath{\mathtt{h}}"),                           // MATHEMATICAL MONOSPACE SMALL H [𝚑]
        ('\u{1D692}', r"\ensuremath{\mathtt{i}}"),                           // MATHEMATICAL MONOSPACE SMALL I [𝚒]
        ('\u{1D693}', r"\ensuremath{\mathtt{j}}"),                           // MATHEMATICAL MONOSPACE SMALL J [𝚓]
        ('\u{1D694}', r"\ensuremath{\mathtt{k}}"),                           // MATHEMATICAL MONOSPACE SMALL K [𝚔]
        ('\u{1D695}', r"\ensuremath{\mathtt{l}}"),                           // MATHEMATICAL MONOSPACE SMALL L [𝚕]
        ('\u{1D696}', r"\ensuremath{\mathtt{m}}"),                           // MATHEMATICAL MONOSPACE SMALL M [𝚖]
        ('\u{1D697}', r"\ensuremath{\mathtt{n}}"),                           // MATHEMATICAL MONOSPACE SMALL N [𝚗]
        ('\u{1D698}', r"\ensuremath{\mathtt{o}}"),                           // MATHEMATICAL MONOSPACE SMALL O [𝚘]
        ('\u{1D699}', r"\ensuremath{\mathtt{p}}"),                           // MATHEMATICAL MONOSPACE SMALL P [𝚙]
        ('\u{1D69A}', r"\ensuremath{\mathtt{q}}"),                           // MATHEMATICAL MONOSPACE SMALL Q [𝚚]
        ('\u{1D69B}', r"\ensuremath{\mathtt{r}}"),                           // MATHEMATICAL MONOSPACE SMALL R [𝚛]
        ('\u{1D69C}', r"\ensuremath{\mathtt{s}}"),                           // MATHEMATICAL MONOSPACE SMALL S [𝚜]
        ('\u{1D69D}', r"\ensuremath{\mathtt{t}}"),                           // MATHEMATICAL MONOSPACE SMALL T [𝚝]
        ('\u{1D69E}', r"\ensuremath{\mathtt{u}}"),                           // MATHEMATICAL MONOSPACE SMALL U [𝚞]
        ('\u{1D69F}', r"\ensuremath{\mathtt{v}}"),                           // MATHEMATICAL MONOSPACE SMALL V [𝚟]
        ('\u{1D6A0}', r"\ensuremath{\mathtt{w}}"),                           // MATHEMATICAL MONOSPACE SMALL W [𝚠]
        ('\u{1D6A1}', r"\ensuremath{\mathtt{x}}"),                           // MATHEMATICAL MONOSPACE SMALL X [𝚡]
        ('\u{1D6A2}', r"\ensuremath{\mathtt{y}}"),                           // MATHEMATICAL MONOSPACE SMALL Y [𝚢]
        ('\u{1D6A3}', r"\ensuremath{\mathtt{z}}"),                           // MATHEMATICAL MONOSPACE SMALL Z [𝚣]
        //
        ('\u{1D7CE}', r"\ensuremath{\mathbf{0}}"),                           // MATHEMATICAL BOLD DIGIT ZERO [𝟎]
        ('\u{1D7CF}', r"\ensuremath{\mathbf{1}}"),                           // MATHEMATICAL BOLD DIGIT ONE [𝟏]
        ('\u{1D7D0}', r"\ensuremath{\mathbf{2}}"),                           // MATHEMATICAL BOLD DIGIT TWO [𝟐]
        ('\u{1D7D1}', r"\ensuremath{\mathbf{3}}"),                           // MATHEMATICAL BOLD DIGIT THREE [𝟑]
        ('\u{1D7D2}', r"\ensuremath{\mathbf{4}}"),                           // MATHEMATICAL BOLD DIGIT FOUR [𝟒]
        ('\u{1D7D3}', r"\ensuremath{\mathbf{5}}"),                           // MATHEMATICAL BOLD DIGIT FIVE [𝟓]
        ('\u{1D7D4}', r"\ensuremath{\mathbf{6}}"),                           // MATHEMATICAL BOLD DIGIT SIX [𝟔]
        ('\u{1D7D5}', r"\ensuremath{\mathbf{7}}"),                           // MATHEMATICAL BOLD DIGIT SEVEN [𝟕]
        ('\u{1D7D6}', r"\ensuremath{\mathbf{8}}"),                           // MATHEMATICAL BOLD DIGIT EIGHT [𝟖]
        ('\u{1D7D7}', r"\ensuremath{\mathbf{9}}"),                           // MATHEMATICAL BOLD DIGIT NINE [𝟗]
        ('\u{1D7D8}', r"\ensuremath{\mathbb{0}}"),                           // MATHEMATICAL DOUBLE-STRUCK DIGIT ZERO [𝟘]
        ('\u{1D7D9}', r"\ensuremath{\mathbb{1}}"),                           // MATHEMATICAL DOUBLE-STRUCK DIGIT ONE [𝟙]
        ('\u{1D7DA}', r"\ensuremath{\mathbb{2}}"),                           // MATHEMATICAL DOUBLE-STRUCK DIGIT TWO [𝟚]
        ('\u{1D7DB}', r"\ensuremath{\mathbb{3}}"),                           // MATHEMATICAL DOUBLE-STRUCK DIGIT THREE [𝟛]
        ('\u{1D7DC}', r"\ensuremath{\mathbb{4}}"),                           // MATHEMATICAL DOUBLE-STRUCK DIGIT FOUR [𝟜]
        ('\u{1D7DD}', r"\ensuremath{\mathbb{5}}"),                           // MATHEMATICAL DOUBLE-STRUCK DIGIT FIVE [𝟝]
        ('\u{1D7DE}', r"\ensuremath{\mathbb{6}}"),                           // MATHEMATICAL DOUBLE-STRUCK DIGIT SIX [𝟞]
        ('\u{1D7DF}', r"\ensuremath{\mathbb{7}}"),                           // MATHEMATICAL DOUBLE-STRUCK DIGIT SEVEN [𝟟]
        ('\u{1D7E0}', r"\ensuremath{\mathbb{8}}"),                           // MATHEMATICAL DOUBLE-STRUCK DIGIT EIGHT [𝟠]
        ('\u{1D7E1}', r"\ensuremath{\mathbb{9}}"),                           // MATHEMATICAL DOUBLE-STRUCK DIGIT NINE [𝟡]
        ('\u{1D7E2}', r"\ensuremath{\mathsf{0}}"),                           // MATHEMATICAL SANS-SERIF DIGIT ZERO [𝟢]
        ('\u{1D7E3}', r"\ensuremath{\mathsf{1}}"),                           // MATHEMATICAL SANS-SERIF DIGIT ONE [𝟣]
        ('\u{1D7E4}', r"\ensuremath{\mathsf{2}}"),                           // MATHEMATICAL SANS-SERIF DIGIT TWO [𝟤]
        ('\u{1D7E5}', r"\ensuremath{\mathsf{3}}"),                           // MATHEMATICAL SANS-SERIF DIGIT THREE [𝟥]
        ('\u{1D7E6}', r"\ensuremath{\mathsf{4}}"),                           // MATHEMATICAL SANS-SERIF DIGIT FOUR [𝟦]
        ('\u{1D7E7}', r"\ensuremath{\mathsf{5}}"),                           // MATHEMATICAL SANS-SERIF DIGIT FIVE [𝟧]
        ('\u{1D7E8}', r"\ensuremath{\mathsf{6}}"),                           // MATHEMATICAL SANS-SERIF DIGIT SIX [𝟨]
        ('\u{1D7E9}', r"\ensuremath{\mathsf{7}}"),                           // MATHEMATICAL SANS-SERIF DIGIT SEVEN [𝟩]
        ('\u{1D7EA}', r"\ensuremath{\mathsf{8}}"),                           // MATHEMATICAL SANS-SERIF DIGIT EIGHT [𝟪]
        ('\u{1D7EB}', r"\ensuremath{\mathsf{9}}"),                           // MATHEMATICAL SANS-SERIF DIGIT NINE [𝟫]
        //
        ('\u{1D7F6}', r"\ensuremath{\mathtt{0}}"),                           // MATHEMATICAL MONOSPACE DIGIT ZERO [𝟶]
        ('\u{1D7F7}', r"\ensuremath{\mathtt{1}}"),                           // MATHEMATICAL MONOSPACE DIGIT ONE [𝟷]
        ('\u{1D7F8}', r"\ensuremath{\mathtt{2}}"),                           // MATHEMATICAL MONOSPACE DIGIT TWO [𝟸]
        ('\u{1D7F9}', r"\ensuremath{\mathtt{3}}"),                           // MATHEMATICAL MONOSPACE DIGIT THREE [𝟹]
        ('\u{1D7FA}', r"\ensuremath{\mathtt{4}}"),                           // MATHEMATICAL MONOSPACE DIGIT FOUR [𝟺]
        ('\u{1D7FB}', r"\ensuremath{\mathtt{5}}"),                           // MATHEMATICAL MONOSPACE DIGIT FIVE [𝟻]
        ('\u{1D7FC}', r"\ensuremath{\mathtt{6}}"),                           // MATHEMATICAL MONOSPACE DIGIT SIX [𝟼]
        ('\u{1D7FD}', r"\ensuremath{\mathtt{7}}"),                           // MATHEMATICAL MONOSPACE DIGIT SEVEN [𝟽]
        ('\u{1D7FE}', r"\ensuremath{\mathtt{8}}"),                           // MATHEMATICAL MONOSPACE DIGIT EIGHT [𝟾]
        ('\u{1D7FF}', r"\ensuremath{\mathtt{9}}"),                           // MATHEMATICAL MONOSPACE DIGIT NINE [𝟿]
    ]);
}

pub fn replace(input: &mut String) {
    let mut out: Option<String> = None;
    for (idx, char) in input.chars().enumerate() {
        if char.is_ascii() {
            if let Some(out) = out.as_mut() {
                out.push(char);
            }
            continue;
        }
        if out.is_none() {
            out = Some(String::from(&input[..idx]));
        }
        let out = out.as_mut().unwrap();
        let Some(replaced) = UNICODE_2_LATEX.get(&char) else {
            panic!("unknown unicode character");
        };
        out.push('{');
        out.push_str(replaced);
        out.push('}');
    }
    if let Some(out) = out {
        let _ = std::mem::replace(input, out);
    }
}
